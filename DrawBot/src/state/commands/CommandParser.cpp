#include "../CommandParser.h"

ParseResult CommandParser::read_char(const char c) {
    switch (m_state) {
        case ParseState::Id:
            if (c == '\r' || c == '\n') return ParseResult::Continue;

            if (c == 'E') {
                m_state = ParseState::EndN;
                return ParseResult::Continue;
            }

            if (c == 'S' || c == 'M' || c == 'L') {
                m_command.type = c;
                m_current_field_value = std::nullopt;
                m_state = ParseState::Field1;
                return ParseResult::Continue;
            }

            return ParseResult::BadId;

        case ParseState::Field1:
        case ParseState::Field2:
            if (c >= '0' && c <= '9') {
                m_current_field_value = m_current_field_value.value_or(0) * 10 + (c - '0');
                return m_current_field_value > UINT16_MAX ? ParseResult::NumberTooBig : ParseResult::Continue;
            }

            if (!m_current_field_value.has_value()) return ParseResult::EmptyField;

            if (c == ',' && m_state == ParseState::Field1) {
                m_command.x = m_current_field_value.value();
                m_current_field_value = std::nullopt;
                m_state = ParseState::Field2;
                return ParseResult::Continue;
            }

            if (c == ';' && m_state == ParseState::Field2) {
                m_command.y = m_current_field_value.value();
                m_state = ParseState::Id;
                return ParseResult::CommandReady;
            }

            return ParseResult::BadChar;

        case ParseState::EndN:
            if (c != 'N') return ParseResult::BadChar;
            m_state = ParseState::EndD;
            return ParseResult::Continue;
        case ParseState::EndD:
            if (c != 'D') return ParseResult::BadChar;
            m_state = ParseState::Done;
            return ParseResult::Done;
        case ParseState::Done:
            return ParseResult::BadChar;
    }

    return ParseResult::BadChar;
}
