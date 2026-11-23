#pragma once
#include <cstdint>
#include <optional>

#include "../MoveCommand.h"

enum class ParseResult: uint8_t {
    Continue,
    CommandReady,
    Done,
    BadId,
    BadChar,
    NumberTooBig,
    EmptyField,
};

/// Describes what the parser is waiting to receive next.
enum class ParseState: uint8_t { Id, Field1, Field2, EndN, EndD, Done };

class CommandParser {
public:
    ParseResult read_char(char c);

    [[nodiscard]] ParseState state() const { return m_state; }

    /// Valid after read_char returns CommandReady.
    [[nodiscard]] const MoveCommand &command() const { return m_command; }

private:
    ParseState m_state = ParseState::Id;
    MoveCommand m_command{};
    std::optional<int> m_current_field_value = std::nullopt;
};
