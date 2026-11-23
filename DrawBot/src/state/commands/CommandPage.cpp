#include "../CommandPage.h"

#include <climits>

#include "../CommandParser.h"
#include "../MoveCommand.h"

/// Clears the page and parses a new page straight from the stream.
void CommandPage::parse_from_response(WiFiClient &response_stream, const int response_size) {
    m_command_count = 0;
    m_cursor = 0;
    m_parse_failed = false;

    auto parser = CommandParser();
    uint32_t last_receive_time = millis();

    const bool missing_content_length_header = response_size == -1;
    int remaining = response_size;
    if (missing_content_length_header) {
        // If we didn't get a content length header, we can't validate based on that, but we can probably
        // still parse just fine.
        remaining = INT_MAX;
    }

    // WiFiClient::connected() stays true while unread bytes remain, even after the server has closed.
    while (!m_parse_failed && parser.state() != ParseState::Done && remaining > 0 && response_stream.connected()) {
        int available = response_stream.available();
        if (available <= 0) {
            if (millis() - last_receive_time > STALL_TIMEOUT_MS) {
                Serial.println("CPP failed: Response timed out.");
                m_parse_failed = true;
            } else {
                // Wait for the wifi stack to read more bytes.
                delay(1);
            }
            continue;
        }
        last_receive_time = millis();

        while (available > 0 && remaining > 0) {
            handle_parser_result(parser, parser.read_char(static_cast<char>(response_stream.read())));

            available--;
            remaining--;

            if (m_parse_failed || parser.state() == ParseState::Done) break;
        }
    }

    // Make sure we consumed the full body and got the END symbol marking the end of the page.
    const int available = response_stream.available();
    if (available > 0) {
        // CPP = command page parse
        Serial.println("CPP failed: Exited with available bytes remaining.");
        m_parse_failed = true;
    }
    if (remaining > 0 && !missing_content_length_header) {
        Serial.println("CPP failed: Exited before consuming length specified by content length header.");
        m_parse_failed = true;
    }
    if (parser.state() != ParseState::Done) {
        Serial.println("CPP failed: Didn't get end symbol.");
        m_parse_failed = true;
    }

    if (m_parse_failed) {
        Serial.printf(
            "CPP failure diagnostics: %i, %i, %i, %i.\n",
            response_size,
            remaining,
            available,
            parser.state() == ParseState::Done
        );
        // Clear the command page so it's more obvious if an invalid page is accidentally used.
        m_image_width = 1;
        m_image_height = 1;
        m_command_count = 0;
    }
}

void CommandPage::handle_parser_result(const CommandParser &parser, const ParseResult parse_result) {
    switch (parse_result) {
        case ParseResult::Continue:
        case ParseResult::Done:
            break;
        case ParseResult::CommandReady: {
            const MoveCommand command = parser.command();
            if (command.type == 'S') {
                m_image_width = command.x;
                m_image_height = command.y;
            } else {
                if (m_command_count >= MAX_COMMANDS_PER_PAGE) {
                    Serial.println("CPP failed: Too many commands in the page.");
                    m_parse_failed = true;
                    break;
                }
                m_commands[m_command_count++] = command;
            }
            break;
        }
        case ParseResult::BadId:
            Serial.println("CPP failed: Invalid command id.");
            m_parse_failed = true;
            break;
        case ParseResult::BadChar:
            Serial.println("CPP failed: Unexpected character.");
            m_parse_failed = true;
            break;
        case ParseResult::NumberTooBig:
            Serial.println("CPP failed: Received number too big for u16.");
            m_parse_failed = true;
            break;
        case ParseResult::EmptyField:
            Serial.println("CPP failed: Encountered empty field.");
            m_parse_failed = true;
            break;
    }
}

/// Returns the next unexecuted command, or nullptr if the page has no more commands.
const MoveCommand *CommandPage::next_command() {
    if (m_cursor >= m_command_count) {
        return nullptr;
    }

    return &m_commands[m_cursor++];
}
