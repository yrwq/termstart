export type ParsedCommand = {
  name: string;
  args: string[];
  flags: string[];
  raw: string;
};

export type ParseError = {
  error: string;
};

function tokenize(input: string): { tokens: string[]; error?: string } {
  const tokens: string[] = [];
  let current = '';
  let inQuotes = false;
  let quoteChar: '"' | '\'' | null = null;

  for (let i = 0; i < input.length; i += 1) {
    const char = input[i];

    if ((char === '"' || char === '\'') && !inQuotes) {
      inQuotes = true;
      quoteChar = char as '"' | '\'';
      continue;
    }

    if (inQuotes && char === quoteChar) {
      inQuotes = false;
      quoteChar = null;
      continue;
    }

    if (!inQuotes && /\s/.test(char)) {
      if (current.length > 0) {
        tokens.push(current);
        current = '';
      }
      continue;
    }

    current += char;
  }

  if (inQuotes) {
    return { tokens: [], error: 'Unterminated quoted string' };
  }

  if (current.length > 0) {
    tokens.push(current);
  }

  return { tokens };
}

export function parseCommand(input: string): ParsedCommand | ParseError {
  const trimmed = input.trim();
  if (trimmed.length === 0) {
    return { error: 'Empty command' };
  }

  const { tokens, error } = tokenize(trimmed);
  if (error) return { error };
  if (tokens.length === 0) return { error: 'Empty command' };

  const [name, ...rest] = tokens;
  const args: string[] = [];
  const flags: string[] = [];

  for (const token of rest) {
    if (token.startsWith('-') && token !== '-') {
      flags.push(token);
    } else {
      args.push(token);
    }
  }

  return {
    name,
    args,
    flags,
    raw: input,
  };
}

export function hasFlag(parsed: ParsedCommand, ...flagNames: string[]): boolean {
  return flagNames.some((flag) => parsed.flags.includes(flag));
}
