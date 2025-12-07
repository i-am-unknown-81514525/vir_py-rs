use syn::{custom_keyword, custom_punctuation};

// Custom keywords for identifiers that are NOT Rust keywords
custom_keyword!(True);
custom_keyword!(False);
custom_keyword!(None);
custom_keyword!(elif);
custom_keyword!(del);
custom_keyword!(pass);
custom_keyword!(is);

custom_punctuation!(StarStar, **);
custom_punctuation!(Eq, ==);
custom_punctuation!(NotEq, !=);
custom_punctuation!(Lte, <=);
custom_punctuation!(Gte, >=);
custom_punctuation!(LeftShift, <<);
custom_punctuation!(RightShift, >>);

// Assignment Operators
custom_punctuation!(PlusAssign, +=);
custom_punctuation!(MinusAssign, -=);
custom_punctuation!(StarAssign, *=);
custom_punctuation!(SlashAssign, /=);
custom_punctuation!(PercentAssign, %=);
custom_punctuation!(BitAndAssign, &=);
custom_punctuation!(BitOrAssign, |=);
custom_punctuation!(BitXorAssign, ^=);
custom_punctuation!(LeftShiftAssign, <<=);
custom_punctuation!(RightShiftAssign, >>=);
