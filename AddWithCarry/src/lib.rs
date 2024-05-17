#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
extern crate alloc;
use integer_subrange::*;
use IsZero::*;
use common::*;
pub fn AddWithCarry<T: Tracer>(
    state: &mut State,
    tracer: &T,
    x: Bits,
    y: Bits,
    carry_in: bool,
) -> ProductTyped54bc449dd09e5bd {
    #[derive(Default)]
    struct FunctionState {
        n: bool,
        v: bool,
        signed_sum: i128,
        c: bool,
        z: bool,
        unsigned_sum: i128,
        x: Bits,
        y: Bits,
        carry_in: bool,
    }
    let fn_state = FunctionState {
        x,
        y,
        carry_in,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped54bc449dd09e5bd {
        // D s_0_0: read-var x:bv
        let s_0_0: Bits = fn_state.x;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (s_0_0.value() as i128);
        // D s_0_2: read-var y:bv
        let s_0_2: Bits = fn_state.y;
        // D s_0_3: cast zx s_0_2 -> i
        let s_0_3: i128 = (s_0_2.value() as i128);
        // D s_0_4: add s_0_1 s_0_3
        let s_0_4: i128 = (s_0_1 + s_0_3);
        // D s_0_5: read-var carry_in:u8
        let s_0_5: bool = fn_state.carry_in;
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 1u16);
        // D s_0_7: cast zx s_0_6 -> i
        let s_0_7: i128 = (s_0_6.value() as i128);
        // D s_0_8: cast reint s_0_7 -> i64
        let s_0_8: i64 = (s_0_7 as i64);
        // D s_0_9: cast zx s_0_8 -> i
        let s_0_9: i128 = (i128::try_from(s_0_8).unwrap());
        // D s_0_10: add s_0_4 s_0_9
        let s_0_10: i128 = (s_0_4 + s_0_9);
        // D s_0_11: write-var unsigned_sum <= s_0_10
        fn_state.unsigned_sum = s_0_10;
        // D s_0_12: read-var x:bv
        let s_0_12: Bits = fn_state.x;
        // D s_0_13: cast sx s_0_12 -> i
        let s_0_13: i128 = {
            let sign_bit = s_0_12.length() - 1;
            let mut result = s_0_12.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_0_14: read-var y:bv
        let s_0_14: Bits = fn_state.y;
        // D s_0_15: cast sx s_0_14 -> i
        let s_0_15: i128 = {
            let sign_bit = s_0_14.length() - 1;
            let mut result = s_0_14.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_0_16: add s_0_13 s_0_15
        let s_0_16: i128 = (s_0_13 + s_0_15);
        // D s_0_17: read-var carry_in:u8
        let s_0_17: bool = fn_state.carry_in;
        // D s_0_18: cast zx s_0_17 -> bv
        let s_0_18: Bits = Bits::new(s_0_17 as u128, 1u16);
        // D s_0_19: cast zx s_0_18 -> i
        let s_0_19: i128 = (s_0_18.value() as i128);
        // D s_0_20: cast reint s_0_19 -> i64
        let s_0_20: i64 = (s_0_19 as i64);
        // D s_0_21: cast zx s_0_20 -> i
        let s_0_21: i128 = (i128::try_from(s_0_20).unwrap());
        // D s_0_22: add s_0_16 s_0_21
        let s_0_22: i128 = (s_0_16 + s_0_21);
        // D s_0_23: write-var signed_sum <= s_0_22
        fn_state.signed_sum = s_0_22;
        // D s_0_24: read-var y:bv
        let s_0_24: Bits = fn_state.y;
        // D s_0_25: size-of s_0_24
        let s_0_25: u16 = s_0_24.length();
        // D s_0_26: cast zx s_0_25 -> i
        let s_0_26: i128 = (i128::try_from(s_0_25).unwrap());
        // C s_0_27: const #1s : i
        let s_0_27: i128 = 1;
        // D s_0_28: sub s_0_26 s_0_27
        let s_0_28: i128 = ((s_0_26) - (s_0_27));
        // C s_0_29: const #0s : i
        let s_0_29: i128 = 0;
        // D s_0_30: read-var unsigned_sum:i
        let s_0_30: i128 = fn_state.unsigned_sum;
        // D s_0_31: call integer_subrange(s_0_30, s_0_28, s_0_29)
        let s_0_31: Bits = integer_subrange(state, tracer, s_0_30, s_0_28, s_0_29);
        // D s_0_32: read-var y:bv
        let s_0_32: Bits = fn_state.y;
        // D s_0_33: size-of s_0_32
        let s_0_33: u16 = s_0_32.length();
        // D s_0_34: cast zx s_0_33 -> i
        let s_0_34: i128 = (i128::try_from(s_0_33).unwrap());
        // C s_0_35: const #1s : i
        let s_0_35: i128 = 1;
        // D s_0_36: sub s_0_34 s_0_35
        let s_0_36: i128 = ((s_0_34) - (s_0_35));
        // C s_0_37: const #1u : u64
        let s_0_37: u64 = 1;
        // D s_0_38: bit-extract s_0_31 s_0_36 s_0_37
        let s_0_38: Bits = (Bits::new(
            ((s_0_31) >> (s_0_36)).value(),
            u16::try_from(s_0_37).unwrap(),
        ));
        // D s_0_39: cast reint s_0_38 -> u8
        let s_0_39: bool = ((s_0_38.value()) != 0);
        // C s_0_40: const #0s : i
        let s_0_40: i128 = 0;
        // C s_0_41: const #0u : u64
        let s_0_41: u64 = 0;
        // D s_0_42: cast zx s_0_39 -> u64
        let s_0_42: u64 = (s_0_39 as u64);
        // C s_0_43: const #1u : u64
        let s_0_43: u64 = 1;
        // D s_0_44: and s_0_42 s_0_43
        let s_0_44: u64 = ((s_0_42) & (s_0_43));
        // D s_0_45: cmp-eq s_0_44 s_0_43
        let s_0_45: bool = ((s_0_44) == (s_0_43));
        // D s_0_46: lsl s_0_42 s_0_40
        let s_0_46: u64 = s_0_42 << s_0_40;
        // D s_0_47: or s_0_41 s_0_46
        let s_0_47: u64 = ((s_0_41) | (s_0_46));
        // D s_0_48: cmpl s_0_46
        let s_0_48: u64 = !s_0_46;
        // D s_0_49: and s_0_41 s_0_48
        let s_0_49: u64 = ((s_0_41) & (s_0_48));
        // D s_0_50: select s_0_45 s_0_47 s_0_49
        let s_0_50: u64 = if s_0_45 { s_0_47 } else { s_0_49 };
        // D s_0_51: cast trunc s_0_50 -> u8
        let s_0_51: bool = ((s_0_50) != 0);
        // D s_0_52: write-var n <= s_0_51
        fn_state.n = s_0_51;
        // D s_0_53: read-var y:bv
        let s_0_53: Bits = fn_state.y;
        // D s_0_54: size-of s_0_53
        let s_0_54: u16 = s_0_53.length();
        // D s_0_55: cast zx s_0_54 -> i
        let s_0_55: i128 = (i128::try_from(s_0_54).unwrap());
        // C s_0_56: const #1s : i
        let s_0_56: i128 = 1;
        // D s_0_57: sub s_0_55 s_0_56
        let s_0_57: i128 = ((s_0_55) - (s_0_56));
        // C s_0_58: const #0s : i
        let s_0_58: i128 = 0;
        // D s_0_59: read-var unsigned_sum:i
        let s_0_59: i128 = fn_state.unsigned_sum;
        // D s_0_60: call integer_subrange(s_0_59, s_0_57, s_0_58)
        let s_0_60: Bits = integer_subrange(state, tracer, s_0_59, s_0_57, s_0_58);
        // D s_0_61: call IsZero(s_0_60)
        let s_0_61: bool = IsZero(state, tracer, s_0_60);
        // N s_0_62: branch s_0_61 b9 b1
        if s_0_61 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped54bc449dd09e5bd {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var z <= s_1_0
        fn_state.z = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped54bc449dd09e5bd {
        // D s_2_0: read-var y:bv
        let s_2_0: Bits = fn_state.y;
        // D s_2_1: size-of s_2_0
        let s_2_1: u16 = s_2_0.length();
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // C s_2_3: const #1s : i
        let s_2_3: i128 = 1;
        // D s_2_4: sub s_2_2 s_2_3
        let s_2_4: i128 = ((s_2_2) - (s_2_3));
        // C s_2_5: const #0s : i
        let s_2_5: i128 = 0;
        // D s_2_6: read-var unsigned_sum:i
        let s_2_6: i128 = fn_state.unsigned_sum;
        // D s_2_7: call integer_subrange(s_2_6, s_2_4, s_2_5)
        let s_2_7: Bits = integer_subrange(state, tracer, s_2_6, s_2_4, s_2_5);
        // D s_2_8: cast zx s_2_7 -> i
        let s_2_8: i128 = (s_2_7.value() as i128);
        // D s_2_9: read-var unsigned_sum:i
        let s_2_9: i128 = fn_state.unsigned_sum;
        // D s_2_10: cmp-eq s_2_8 s_2_9
        let s_2_10: bool = ((s_2_8) == (s_2_9));
        // N s_2_11: branch s_2_10 b8 b3
        if s_2_10 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped54bc449dd09e5bd {
        // C s_3_0: const #1u : u8
        let s_3_0: bool = true;
        // D s_3_1: write-var c <= s_3_0
        fn_state.c = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped54bc449dd09e5bd {
        // D s_4_0: read-var y:bv
        let s_4_0: Bits = fn_state.y;
        // D s_4_1: size-of s_4_0
        let s_4_1: u16 = s_4_0.length();
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // C s_4_3: const #1s : i
        let s_4_3: i128 = 1;
        // D s_4_4: sub s_4_2 s_4_3
        let s_4_4: i128 = ((s_4_2) - (s_4_3));
        // C s_4_5: const #0s : i
        let s_4_5: i128 = 0;
        // D s_4_6: read-var unsigned_sum:i
        let s_4_6: i128 = fn_state.unsigned_sum;
        // D s_4_7: call integer_subrange(s_4_6, s_4_4, s_4_5)
        let s_4_7: Bits = integer_subrange(state, tracer, s_4_6, s_4_4, s_4_5);
        // D s_4_8: cast sx s_4_7 -> i
        let s_4_8: i128 = {
            let sign_bit = s_4_7.length() - 1;
            let mut result = s_4_7.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_4_9: read-var signed_sum:i
        let s_4_9: i128 = fn_state.signed_sum;
        // D s_4_10: cmp-eq s_4_8 s_4_9
        let s_4_10: bool = ((s_4_8) == (s_4_9));
        // N s_4_11: branch s_4_10 b7 b5
        if s_4_10 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped54bc449dd09e5bd {
        // C s_5_0: const #1u : u8
        let s_5_0: bool = true;
        // D s_5_1: write-var v <= s_5_0
        fn_state.v = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped54bc449dd09e5bd {
        // D s_6_0: read-var y:bv
        let s_6_0: Bits = fn_state.y;
        // D s_6_1: size-of s_6_0
        let s_6_1: u16 = s_6_0.length();
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // C s_6_3: const #1s : i
        let s_6_3: i128 = 1;
        // D s_6_4: sub s_6_2 s_6_3
        let s_6_4: i128 = ((s_6_2) - (s_6_3));
        // C s_6_5: const #0s : i
        let s_6_5: i128 = 0;
        // D s_6_6: read-var unsigned_sum:i
        let s_6_6: i128 = fn_state.unsigned_sum;
        // D s_6_7: call integer_subrange(s_6_6, s_6_4, s_6_5)
        let s_6_7: Bits = integer_subrange(state, tracer, s_6_6, s_6_4, s_6_5);
        // D s_6_8: read-var n:u8
        let s_6_8: bool = fn_state.n;
        // D s_6_9: cast zx s_6_8 -> bv
        let s_6_9: Bits = Bits::new(s_6_8 as u128, 1u16);
        // D s_6_10: read-var z:u8
        let s_6_10: bool = fn_state.z;
        // D s_6_11: cast zx s_6_10 -> bv
        let s_6_11: Bits = Bits::new(s_6_10 as u128, 1u16);
        // D s_6_12: cast reint s_6_9 -> u128
        let s_6_12: u128 = (s_6_9.value() as u128);
        // D s_6_13: size-of s_6_9
        let s_6_13: u16 = s_6_9.length();
        // D s_6_14: cast reint s_6_11 -> u128
        let s_6_14: u128 = (s_6_11.value() as u128);
        // D s_6_15: size-of s_6_11
        let s_6_15: u16 = s_6_11.length();
        // D s_6_16: lsl s_6_12 s_6_15
        let s_6_16: u128 = s_6_12 << s_6_15;
        // D s_6_17: or s_6_16 s_6_14
        let s_6_17: u128 = ((s_6_16) | (s_6_14));
        // D s_6_18: add s_6_13 s_6_15
        let s_6_18: u16 = (s_6_13 + s_6_15);
        // D s_6_19: create-bits s_6_17 s_6_18
        let s_6_19: Bits = Bits::new(s_6_17, s_6_18);
        // D s_6_20: cast reint s_6_19 -> u8
        let s_6_20: u8 = (s_6_19.value() as u8);
        // D s_6_21: cast zx s_6_20 -> bv
        let s_6_21: Bits = Bits::new(s_6_20 as u128, 2u16);
        // D s_6_22: read-var c:u8
        let s_6_22: bool = fn_state.c;
        // D s_6_23: cast zx s_6_22 -> bv
        let s_6_23: Bits = Bits::new(s_6_22 as u128, 1u16);
        // D s_6_24: cast reint s_6_21 -> u128
        let s_6_24: u128 = (s_6_21.value() as u128);
        // D s_6_25: size-of s_6_21
        let s_6_25: u16 = s_6_21.length();
        // D s_6_26: cast reint s_6_23 -> u128
        let s_6_26: u128 = (s_6_23.value() as u128);
        // D s_6_27: size-of s_6_23
        let s_6_27: u16 = s_6_23.length();
        // D s_6_28: lsl s_6_24 s_6_27
        let s_6_28: u128 = s_6_24 << s_6_27;
        // D s_6_29: or s_6_28 s_6_26
        let s_6_29: u128 = ((s_6_28) | (s_6_26));
        // D s_6_30: add s_6_25 s_6_27
        let s_6_30: u16 = (s_6_25 + s_6_27);
        // D s_6_31: create-bits s_6_29 s_6_30
        let s_6_31: Bits = Bits::new(s_6_29, s_6_30);
        // D s_6_32: cast reint s_6_31 -> u8
        let s_6_32: u8 = (s_6_31.value() as u8);
        // D s_6_33: cast zx s_6_32 -> bv
        let s_6_33: Bits = Bits::new(s_6_32 as u128, 3u16);
        // D s_6_34: read-var v:u8
        let s_6_34: bool = fn_state.v;
        // D s_6_35: cast zx s_6_34 -> bv
        let s_6_35: Bits = Bits::new(s_6_34 as u128, 1u16);
        // D s_6_36: cast reint s_6_33 -> u128
        let s_6_36: u128 = (s_6_33.value() as u128);
        // D s_6_37: size-of s_6_33
        let s_6_37: u16 = s_6_33.length();
        // D s_6_38: cast reint s_6_35 -> u128
        let s_6_38: u128 = (s_6_35.value() as u128);
        // D s_6_39: size-of s_6_35
        let s_6_39: u16 = s_6_35.length();
        // D s_6_40: lsl s_6_36 s_6_39
        let s_6_40: u128 = s_6_36 << s_6_39;
        // D s_6_41: or s_6_40 s_6_38
        let s_6_41: u128 = ((s_6_40) | (s_6_38));
        // D s_6_42: add s_6_37 s_6_39
        let s_6_42: u16 = (s_6_37 + s_6_39);
        // D s_6_43: create-bits s_6_41 s_6_42
        let s_6_43: Bits = Bits::new(s_6_41, s_6_42);
        // D s_6_44: cast reint s_6_43 -> u8
        let s_6_44: u8 = (s_6_43.value() as u8);
        // D s_6_45: create-product struct = ["s_6_7", "s_6_44"]
        let s_6_45: ProductTyped54bc449dd09e5bd = ProductTyped54bc449dd09e5bd {
            _0: s_6_7,
            _1: s_6_44,
        };
        // N s_6_46: return s_6_45
        return s_6_45;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped54bc449dd09e5bd {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var v <= s_7_0
        fn_state.v = s_7_0;
        // N s_7_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped54bc449dd09e5bd {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var c <= s_8_0
        fn_state.c = s_8_0;
        // N s_8_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped54bc449dd09e5bd {
        // C s_9_0: const #1u : u8
        let s_9_0: bool = true;
        // D s_9_1: write-var z <= s_9_0
        fn_state.z = s_9_0;
        // N s_9_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
