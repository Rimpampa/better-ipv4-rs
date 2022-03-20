pub const IPV4: EnglishNumeral<super::Ipv4xxxx> =
    <EnglishNumeral<super::Ipv4xxxx> as super::Octet>::VALUES;

super::impls_for_octet!(EnglishNumeral);

macro_rules! english_numerals {
    ($($n:ident : $v:expr),* $(,)?) => {
        #[allow(unused)]
        pub struct EnglishNumeral<T> {
            $( pub $n : $crate::Value<Self, T, $v> ),*
        }

        impl<T> $crate::Octet for EnglishNumeral<T> {
            const VALUES: Self = Self {
                $( $n : $crate::Value {
                    _o: std::marker::PhantomData,
                    _t: std::marker::PhantomData
                } ),*
            };
        }
    };
}

english_numerals! {
    zero: 0,
    one: 1,
    two: 2,
    three: 3,
    four: 4,
    five: 5,
    six: 6,
    seven: 7,
    eight: 8,
    nine: 9,
    ten: 10,
    eleven: 11,
    twelve: 12,
    thirteen: 13,
    fourteen: 14,
    fifteen: 15,
    sixteen: 16,
    seventeen: 17,
    eighteen: 18,
    nineteen: 19,
    twenty: 20,
    twenty_one: 21,
    twenty_two: 22,
    twenty_three: 23,
    twenty_four: 24,
    twenty_five: 25,
    twenty_six: 26,
    twenty_seven: 27,
    twenty_eight: 28,
    twenty_nine: 29,
    thirty: 30,
    thirty_one: 31,
    thirty_two: 32,
    thirty_three: 33,
    thirty_four: 34,
    thirty_five: 35,
    thirty_six: 36,
    thirty_seven: 37,
    thirty_eight: 38,
    thirty_nine: 39,
    forty: 40,
    forty_one: 41,
    forty_two: 42,
    forty_three: 43,
    forty_four: 44,
    forty_five: 45,
    forty_six: 46,
    forty_seven: 47,
    forty_eight: 48,
    forty_nine: 49,
    fifty: 50,
    fifty_one: 51,
    fifty_two: 52,
    fifty_three: 53,
    fifty_four: 54,
    fifty_five: 55,
    fifty_six: 56,
    fifty_seven: 57,
    fifty_eight: 58,
    fifty_nine: 59,
    sixty: 60,
    sixty_one: 61,
    sixty_two: 62,
    sixty_three: 63,
    sixty_four: 64,
    sixty_five: 65,
    sixty_six: 66,
    sixty_seven: 67,
    sixty_eight: 68,
    sixty_nine: 69,
    seventy: 70,
    seventy_one: 71,
    seventy_two: 72,
    seventy_three: 73,
    seventy_four: 74,
    seventy_five: 75,
    seventy_six: 76,
    seventy_seven: 77,
    seventy_eight: 78,
    seventy_nine: 79,
    eighty: 80,
    eighty_one: 81,
    eighty_two: 82,
    eighty_three: 83,
    eighty_four: 84,
    eighty_five: 85,
    eighty_six: 86,
    eighty_seven: 87,
    eighty_eight: 88,
    eighty_nine: 89,
    ninety: 90,
    ninety_one: 91,
    ninety_two: 92,
    ninety_three: 93,
    ninety_four: 94,
    ninety_five: 95,
    ninety_six: 96,
    ninety_seven: 97,
    ninety_eight: 98,
    ninety_nine: 99,
    one_hundred: 100,
    one_hundred_one: 101,
    one_hundred_two: 102,
    one_hundred_three: 103,
    one_hundred_four: 104,
    one_hundred_five: 105,
    one_hundred_six: 106,
    one_hundred_seven: 107,
    one_hundred_eight: 108,
    one_hundred_nine: 109,
    one_hundred_ten: 110,
    one_hundred_eleven: 111,
    one_hundred_twelve: 112,
    one_hundred_thirteen: 113,
    one_hundred_fourteen: 114,
    one_hundred_fifteen: 115,
    one_hundred_sixteen: 116,
    one_hundred_seventeen: 117,
    one_hundred_eighteen: 118,
    one_hundred_nineteen: 119,
    one_hundred_twenty: 120,
    one_hundred_twenty_one: 121,
    one_hundred_twenty_two: 122,
    one_hundred_twenty_three: 123,
    one_hundred_twenty_four: 124,
    one_hundred_twenty_five: 125,
    one_hundred_twenty_six: 126,
    one_hundred_twenty_seven: 127,
    one_hundred_twenty_eight: 128,
    one_hundred_twenty_nine: 129,
    one_hundred_thirty: 130,
    one_hundred_thirty_one: 131,
    one_hundred_thirty_two: 132,
    one_hundred_thirty_three: 133,
    one_hundred_thirty_four: 134,
    one_hundred_thirty_five: 135,
    one_hundred_thirty_six: 136,
    one_hundred_thirty_seven: 137,
    one_hundred_thirty_eight: 138,
    one_hundred_thirty_nine: 139,
    one_hundred_forty: 140,
    one_hundred_forty_one: 141,
    one_hundred_forty_two: 142,
    one_hundred_forty_three: 143,
    one_hundred_forty_four: 144,
    one_hundred_forty_five: 145,
    one_hundred_forty_six: 146,
    one_hundred_forty_seven: 147,
    one_hundred_forty_eight: 148,
    one_hundred_forty_nine: 149,
    one_hundred_fifty: 150,
    one_hundred_fifty_one: 151,
    one_hundred_fifty_two: 152,
    one_hundred_fifty_three: 153,
    one_hundred_fifty_four: 154,
    one_hundred_fifty_five: 155,
    one_hundred_fifty_six: 156,
    one_hundred_fifty_seven: 157,
    one_hundred_fifty_eight: 158,
    one_hundred_fifty_nine: 159,
    one_hundred_sixty: 160,
    one_hundred_sixty_one: 161,
    one_hundred_sixty_two: 162,
    one_hundred_sixty_three: 163,
    one_hundred_sixty_four: 164,
    one_hundred_sixty_five: 165,
    one_hundred_sixty_six: 166,
    one_hundred_sixty_seven: 167,
    one_hundred_sixty_eight: 168,
    one_hundred_sixty_nine: 169,
    one_hundred_seventy: 170,
    one_hundred_seventy_one: 171,
    one_hundred_seventy_two: 172,
    one_hundred_seventy_three: 173,
    one_hundred_seventy_four: 174,
    one_hundred_seventy_five: 175,
    one_hundred_seventy_six: 176,
    one_hundred_seventy_seven: 177,
    one_hundred_seventy_eight: 178,
    one_hundred_seventy_nine: 179,
    one_hundred_eighty: 180,
    one_hundred_eighty_one: 181,
    one_hundred_eighty_two: 182,
    one_hundred_eighty_three: 183,
    one_hundred_eighty_four: 184,
    one_hundred_eighty_five: 185,
    one_hundred_eighty_six: 186,
    one_hundred_eighty_seven: 187,
    one_hundred_eighty_eight: 188,
    one_hundred_eighty_nine: 189,
    one_hundred_ninety: 190,
    one_hundred_ninety_one: 191,
    one_hundred_ninety_two: 192,
    one_hundred_ninety_three: 193,
    one_hundred_ninety_four: 194,
    one_hundred_ninety_five: 195,
    one_hundred_ninety_six: 196,
    one_hundred_ninety_seven: 197,
    one_hundred_ninety_eight: 198,
    one_hundred_ninety_nine: 199,
    two_hundred: 200,
    two_hundred_one: 201,
    two_hundred_two: 202,
    two_hundred_three: 203,
    two_hundred_four: 204,
    two_hundred_five: 205,
    two_hundred_six: 206,
    two_hundred_seven: 207,
    two_hundred_eight: 208,
    two_hundred_nine: 209,
    two_hundred_ten: 210,
    two_hundred_eleven: 211,
    two_hundred_twelve: 212,
    two_hundred_thirteen: 213,
    two_hundred_fourteen: 214,
    two_hundred_fifteen: 215,
    two_hundred_sixteen: 216,
    two_hundred_seventeen: 217,
    two_hundred_eighteen: 218,
    two_hundred_nineteen: 219,
    two_hundred_twenty: 220,
    two_hundred_twenty_one: 221,
    two_hundred_twenty_two: 222,
    two_hundred_twenty_three: 223,
    two_hundred_twenty_four: 224,
    two_hundred_twenty_five: 225,
    two_hundred_twenty_six: 226,
    two_hundred_twenty_seven: 227,
    two_hundred_twenty_eight: 228,
    two_hundred_twenty_nine: 229,
    two_hundred_thirty: 230,
    two_hundred_thirty_one: 231,
    two_hundred_thirty_two: 232,
    two_hundred_thirty_three: 233,
    two_hundred_thirty_four: 234,
    two_hundred_thirty_five: 235,
    two_hundred_thirty_six: 236,
    two_hundred_thirty_seven: 237,
    two_hundred_thirty_eight: 238,
    two_hundred_thirty_nine: 239,
    two_hundred_forty: 240,
    two_hundred_forty_one: 241,
    two_hundred_forty_two: 242,
    two_hundred_forty_three: 243,
    two_hundred_forty_four: 244,
    two_hundred_forty_five: 245,
    two_hundred_forty_six: 246,
    two_hundred_forty_seven: 247,
    two_hundred_forty_eight: 248,
    two_hundred_forty_nine: 249,
    two_hundred_fifty: 250,
    two_hundred_fifty_one: 251,
    two_hundred_fifty_two: 252,
    two_hundred_fifty_three: 253,
    two_hundred_fifty_four: 254,
    two_hundred_fifty_five: 255,
}