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
use common::*;
pub fn RotCell<T: Tracer>(
    state: &mut State,
    tracer: &T,
    incell_name: u8,
    amount: i64,
) -> u8 {
    #[derive(Default)]
    struct FunctionState {
        tmp: u8,
        incell_name: u8,
        amount: i64,
    }
    let fn_state = FunctionState {
        incell_name,
        amount,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_0_0: const #0s : i
        let s_0_0: i128 = 0;
        // D s_0_1: read-var incell_name:u8
        let s_0_1: u8 = fn_state.incell_name;
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 4u16);
        // C s_0_3: const #1s : i64
        let s_0_3: i64 = 1;
        // C s_0_4: cast zx s_0_3 -> i
        let s_0_4: i128 = (i128::try_from(s_0_3).unwrap());
        // C s_0_5: const #3s : i
        let s_0_5: i128 = 3;
        // C s_0_6: add s_0_5 s_0_4
        let s_0_6: i128 = (s_0_5 + s_0_4);
        // D s_0_7: bit-extract s_0_2 s_0_0 s_0_6
        let s_0_7: Bits = (Bits::new(
            ((s_0_2) >> (s_0_0)).value(),
            u16::try_from(s_0_6).unwrap(),
        ));
        // D s_0_8: cast reint s_0_7 -> u8
        let s_0_8: u8 = (s_0_7.value() as u8);
        // C s_0_9: const #0s : i
        let s_0_9: i128 = 0;
        // D s_0_10: read-var incell_name:u8
        let s_0_10: u8 = fn_state.incell_name;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 4u16);
        // C s_0_12: const #1s : i64
        let s_0_12: i64 = 1;
        // C s_0_13: cast zx s_0_12 -> i
        let s_0_13: i128 = (i128::try_from(s_0_12).unwrap());
        // C s_0_14: const #3s : i
        let s_0_14: i128 = 3;
        // C s_0_15: add s_0_14 s_0_13
        let s_0_15: i128 = (s_0_14 + s_0_13);
        // D s_0_16: bit-extract s_0_11 s_0_9 s_0_15
        let s_0_16: Bits = (Bits::new(
            ((s_0_11) >> (s_0_9)).value(),
            u16::try_from(s_0_15).unwrap(),
        ));
        // D s_0_17: cast reint s_0_16 -> u8
        let s_0_17: u8 = (s_0_16.value() as u8);
        // D s_0_18: cast zx s_0_8 -> bv
        let s_0_18: Bits = Bits::new(s_0_8 as u128, 4u16);
        // D s_0_19: cast zx s_0_17 -> bv
        let s_0_19: Bits = Bits::new(s_0_17 as u128, 4u16);
        // D s_0_20: cast reint s_0_18 -> u128
        let s_0_20: u128 = (s_0_18.value() as u128);
        // D s_0_21: size-of s_0_18
        let s_0_21: u16 = s_0_18.length();
        // D s_0_22: cast reint s_0_19 -> u128
        let s_0_22: u128 = (s_0_19.value() as u128);
        // D s_0_23: size-of s_0_19
        let s_0_23: u16 = s_0_19.length();
        // D s_0_24: lsl s_0_20 s_0_23
        let s_0_24: u128 = s_0_20 << s_0_23;
        // D s_0_25: or s_0_24 s_0_22
        let s_0_25: u128 = ((s_0_24) | (s_0_22));
        // D s_0_26: add s_0_21 s_0_23
        let s_0_26: u16 = (s_0_21 + s_0_23);
        // D s_0_27: create-bits s_0_25 s_0_26
        let s_0_27: Bits = Bits::new(s_0_25, s_0_26);
        // D s_0_28: cast reint s_0_27 -> u8
        let s_0_28: u8 = (s_0_27.value() as u8);
        // C s_0_29: const #0s : i
        let s_0_29: i128 = 0;
        // D s_0_30: read-var tmp:u8
        let s_0_30: u8 = fn_state.tmp;
        // D s_0_31: cast zx s_0_30 -> bv
        let s_0_31: Bits = Bits::new(s_0_30 as u128, 8u16);
        // D s_0_32: cast zx s_0_28 -> bv
        let s_0_32: Bits = Bits::new(s_0_28 as u128, 8u16);
        // C s_0_33: const #7s : i
        let s_0_33: i128 = 7;
        // C s_0_34: const #1u : u64
        let s_0_34: u64 = 1;
        // C s_0_35: cast zx s_0_34 -> bv
        let s_0_35: Bits = Bits::new(s_0_34 as u128, 64u16);
        // C s_0_36: lsl s_0_35 s_0_33
        let s_0_36: Bits = s_0_35 << s_0_33;
        // C s_0_37: sub s_0_36 s_0_35
        let s_0_37: Bits = ((s_0_36) - (s_0_35));
        // D s_0_38: and s_0_32 s_0_37
        let s_0_38: Bits = ((s_0_32) & (s_0_37));
        // D s_0_39: lsl s_0_38 s_0_29
        let s_0_39: Bits = s_0_38 << s_0_29;
        // C s_0_40: lsl s_0_37 s_0_29
        let s_0_40: Bits = s_0_37 << s_0_29;
        // C s_0_41: cmpl s_0_40
        let s_0_41: Bits = !s_0_40;
        // D s_0_42: and s_0_31 s_0_41
        let s_0_42: Bits = ((s_0_31) & (s_0_41));
        // D s_0_43: or s_0_42 s_0_39
        let s_0_43: Bits = ((s_0_42) | (s_0_39));
        // D s_0_44: cast reint s_0_43 -> u8
        let s_0_44: u8 = (s_0_43.value() as u8);
        // D s_0_45: write-var tmp <= s_0_44
        fn_state.tmp = s_0_44;
        // C s_0_46: const #4s : i
        let s_0_46: i128 = 4;
        // D s_0_47: read-var amount:i64
        let s_0_47: i64 = fn_state.amount;
        // D s_0_48: cast zx s_0_47 -> i
        let s_0_48: i128 = (i128::try_from(s_0_47).unwrap());
        // D s_0_49: sub s_0_46 s_0_48
        let s_0_49: i128 = ((s_0_46) - (s_0_48));
        // D s_0_50: cast reint s_0_49 -> i64
        let s_0_50: i64 = (s_0_49 as i64);
        // C s_0_51: const #4s : i
        let s_0_51: i128 = 4;
        // D s_0_52: cast zx s_0_44 -> bv
        let s_0_52: Bits = Bits::new(s_0_44 as u128, 8u16);
        // D s_0_53: cast zx s_0_50 -> i
        let s_0_53: i128 = (i128::try_from(s_0_50).unwrap());
        // D s_0_54: bit-extract s_0_52 s_0_53 s_0_51
        let s_0_54: Bits = (Bits::new(
            ((s_0_52) >> (s_0_53)).value(),
            u16::try_from(s_0_51).unwrap(),
        ));
        // D s_0_55: cast reint s_0_54 -> u8
        let s_0_55: u8 = (s_0_54.value() as u8);
        // N s_0_56: return s_0_55
        return s_0_55;
    }
}
