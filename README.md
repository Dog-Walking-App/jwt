# JWT Crate

This is a JWT (JSON Web Token) library for Rust, providing both a Rust crate and FFI (Foreign Function Interface) support.

## Features

- Full JWT token generation and verification
- FFI support for integration with other languages

## Usage

First, add the following to your `Cargo.toml`:

```toml
[dependencies]
jwt = "0.1.0"
```

Then, in your Rust files:

```rust
use jwt::JWT;

let jwt = JWT::new(<secret>);
```

## FFI Support

This crate also provides FFI support.

Example usage in Node.js:
```ts
import ffi from 'ffi-napi';
import ref from 'ref-napi';
import RefStructDI from 'ref-struct-di';

const Struct = RefStructDI(ref)

const FfiResult = Struct({
  'success': 'bool',
  'data': 'string',
  'error': 'string',
});

const {
  generate,
  get_claims,
  validate,
} = ffi.Library('./librustinjs.so', {
  'generate': ['string', ['string', 'string']],
  'get_claims': [FfiResult, ['string', 'string']],
  'validate': ['bool', ['string', 'string']],
});


interface BaseClaims {
  sub: string;
  exp: number;
}

class JWT {
  private secret: string;

  public static new(secret: string): JWT {
    return new JWT(secret);
  }
  
  private constructor(secret: string) {
    this.secret = secret;
  }

  public generate<T extends BaseClaims>(claims: T): string {
    return generate(this.secret, JSON.stringify(claims)) as string;
  }

  public getClaims<T extends BaseClaims>(token: string): T {
    const result = get_claims(this.secret, token);

    if (result.success === false) {
      throw new Error(result.error as string);
    }
    
    return JSON.parse(result.data as string);
  }

  public validate(token: string): boolean {
    if (validate(this.secret, token)) {
      return true;
    }

    throw new Error('Invalid token');
  }
}

const secret = 'your_secret';
const claims = { sub: '1', username: 'admin', exp: 1699971962 };

const jwt = JWT.new(secret);

try {
  const generatedToken = jwt.generate(claims);
  console.log('Generated token:', generatedToken);

  const claimsResult = jwt.getClaims(generatedToken);
  console.log('Claims:', claimsResult);

  const isValid = jwt.validate(generatedToken);
  console.log('Is valid:', isValid);
} catch (error) {
  console.error(error);
}
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT
