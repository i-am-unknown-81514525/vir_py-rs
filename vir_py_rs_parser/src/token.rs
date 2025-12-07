use syn::{custom_keyword, custom_punctuation};

custom_keyword!(True);
custom_keyword!(False);
custom_keyword!(None);
custom_keyword!(elif);
custom_keyword!(def);
custom_keyword!(class);
custom_keyword!(pass);
custom_keyword!(is);
custom_keyword!(del);

custom_punctuation!(StarStar, **);
custom_punctuation!(Eq, ==);
custom_punctuation!(NotEq, !=);
custom_punctuation!(Lte, <=);
custom_punctuation!(Gte, >=);
custom_punctuation!(LeftShift, <<);
custom_punctuation!(RightShift, >>);


custom_punctuation!(PlusAssign, +=);
custom_punctuation!(MinusAssign, -=);
custom_punctuation!(StarAssign, *=);
custom_punctuation!(SlashAssign, /=);
custom_punctuation!(PercentAssign, %=);
custom_punctuation!(LeftShiftAssign, <<=);
custom_punctuation!(RightShiftAssign, >>=);
custom_punctuation!(BitshiftAndAssign, &=);
custom_punctuation!(BitshiftOrAssign, |=);
custom_punctuation!(BitshiftXorAssign, ^=);
