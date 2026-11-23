#pragma once

#include <WiFiClient.h>

#include "../CommandParser.h"
#include "../MoveCommand.h"

class CommandPage {
public:
    CommandPage() : m_commands{} {
    }

    void parse_from_response(WiFiClient &response_stream, int response_size);

    [[nodiscard]] bool empty() const { return m_command_count == 0; }
    [[nodiscard]] bool parse_failed() const { return m_parse_failed; }

    [[nodiscard]] int get_image_width() const { return m_image_width; }
    [[nodiscard]] int get_image_height() const { return m_image_height; }

    const MoveCommand *next_command();

private:
    void handle_parser_result(const CommandParser &parser, ParseResult parse_result);

    static constexpr size_t MAX_COMMANDS_PER_PAGE = 256;
    static constexpr uint32_t STALL_TIMEOUT_MS = 2000;

    MoveCommand m_commands[MAX_COMMANDS_PER_PAGE];
    size_t m_command_count = 0;
    size_t m_cursor = 0;

    uint16_t m_image_width = 1;
    uint16_t m_image_height = 1;
    bool m_parse_failed = false;
};
