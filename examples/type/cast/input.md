Rust provides no implicit type conversion (coercion) between primitive types.
But, explicit type conversion (casting) can be performed using the `as` keyword.

Rules for converting between integral types follow C conventions, with added
provisions for conversion to signed types.

Let val be the value being assigned, and let uT be the unsigned type with the
same number of bits as the target type. Then
1. If the value can be represented in the target type, the value is unchanged.
2. Otherwise, if the two's-complement of the value can be represented in the
   target type, the two's-complement of the value is the result.
3. Otherwise, the target type has fewer bits than the source type, and the
   resulting value is as if ( val % std::uT::MAX + 1 ) was performed with
   exact arithmetic and reinterpreted according to the previous rules.

{cast.play}
