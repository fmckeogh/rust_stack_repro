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
use R_set::*;
use R_read::*;
use common::*;
pub fn execute_aarch32_instrs_REVSH_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    m: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        result: u32,
        d: i64,
        m: i64,
    }
    let fn_state = FunctionState {
        d,
        m,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var m:i64
        let s_0_0: i64 = fn_state.m;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: call R_read(s_0_1)
        let s_0_2: u32 = R_read(state, tracer, s_0_1);
        // C s_0_3: const #0s : i
        let s_0_3: i128 = 0;
        // D s_0_4: cast zx s_0_2 -> bv
        let s_0_4: Bits = Bits::new(s_0_2 as u128, 32u16);
        // C s_0_5: const #1s : i64
        let s_0_5: i64 = 1;
        // C s_0_6: cast zx s_0_5 -> i
        let s_0_6: i128 = (i128::try_from(s_0_5).unwrap());
        // C s_0_7: const #7s : i
        let s_0_7: i128 = 7;
        // C s_0_8: add s_0_7 s_0_6
        let s_0_8: i128 = (s_0_7 + s_0_6);
        // D s_0_9: bit-extract s_0_4 s_0_3 s_0_8
        let s_0_9: Bits = (Bits::new(
            ((s_0_4) >> (s_0_3)).value(),
            u16::try_from(s_0_8).unwrap(),
        ));
        // D s_0_10: cast reint s_0_9 -> u8
        let s_0_10: u8 = (s_0_9.value() as u8);
        // C s_0_11: const #24s : i
        let s_0_11: i128 = 24;
        // D s_0_12: cast zx s_0_10 -> bv
        let s_0_12: Bits = Bits::new(s_0_10 as u128, 8u16);
        // D s_0_13: bits-cast sx s_0_12 -> bv length s_0_11
        let s_0_13: Bits = s_0_12.sign_extend(s_0_11);
        // D s_0_14: cast reint s_0_13 -> u24
        let s_0_14: u32 = (s_0_13.value() as u32);
        // C s_0_15: const #8s : i
        let s_0_15: i128 = 8;
        // D s_0_16: read-var result:u32
        let s_0_16: u32 = fn_state.result;
        // D s_0_17: cast zx s_0_16 -> bv
        let s_0_17: Bits = Bits::new(s_0_16 as u128, 32u16);
        // D s_0_18: cast zx s_0_14 -> bv
        let s_0_18: Bits = Bits::new(s_0_14 as u128, 24u16);
        // C s_0_19: const #23s : i
        let s_0_19: i128 = 23;
        // C s_0_20: const #1u : u64
        let s_0_20: u64 = 1;
        // C s_0_21: cast zx s_0_20 -> bv
        let s_0_21: Bits = Bits::new(s_0_20 as u128, 64u16);
        // C s_0_22: lsl s_0_21 s_0_19
        let s_0_22: Bits = s_0_21 << s_0_19;
        // C s_0_23: sub s_0_22 s_0_21
        let s_0_23: Bits = ((s_0_22) - (s_0_21));
        // D s_0_24: and s_0_18 s_0_23
        let s_0_24: Bits = ((s_0_18) & (s_0_23));
        // D s_0_25: lsl s_0_24 s_0_15
        let s_0_25: Bits = s_0_24 << s_0_15;
        // C s_0_26: lsl s_0_23 s_0_15
        let s_0_26: Bits = s_0_23 << s_0_15;
        // C s_0_27: cmpl s_0_26
        let s_0_27: Bits = !s_0_26;
        // D s_0_28: and s_0_17 s_0_27
        let s_0_28: Bits = ((s_0_17) & (s_0_27));
        // D s_0_29: or s_0_28 s_0_25
        let s_0_29: Bits = ((s_0_28) | (s_0_25));
        // D s_0_30: cast reint s_0_29 -> u32
        let s_0_30: u32 = (s_0_29.value() as u32);
        // D s_0_31: write-var result <= s_0_30
        fn_state.result = s_0_30;
        // D s_0_32: read-var m:i64
        let s_0_32: i64 = fn_state.m;
        // D s_0_33: cast zx s_0_32 -> i
        let s_0_33: i128 = (i128::try_from(s_0_32).unwrap());
        // D s_0_34: call R_read(s_0_33)
        let s_0_34: u32 = R_read(state, tracer, s_0_33);
        // C s_0_35: const #8s : i
        let s_0_35: i128 = 8;
        // D s_0_36: cast zx s_0_34 -> bv
        let s_0_36: Bits = Bits::new(s_0_34 as u128, 32u16);
        // C s_0_37: const #1s : i64
        let s_0_37: i64 = 1;
        // C s_0_38: cast zx s_0_37 -> i
        let s_0_38: i128 = (i128::try_from(s_0_37).unwrap());
        // C s_0_39: const #7s : i
        let s_0_39: i128 = 7;
        // C s_0_40: add s_0_39 s_0_38
        let s_0_40: i128 = (s_0_39 + s_0_38);
        // D s_0_41: bit-extract s_0_36 s_0_35 s_0_40
        let s_0_41: Bits = (Bits::new(
            ((s_0_36) >> (s_0_35)).value(),
            u16::try_from(s_0_40).unwrap(),
        ));
        // D s_0_42: cast reint s_0_41 -> u8
        let s_0_42: u8 = (s_0_41.value() as u8);
        // C s_0_43: const #0s : i
        let s_0_43: i128 = 0;
        // D s_0_44: cast zx s_0_30 -> bv
        let s_0_44: Bits = Bits::new(s_0_30 as u128, 32u16);
        // D s_0_45: cast zx s_0_42 -> bv
        let s_0_45: Bits = Bits::new(s_0_42 as u128, 8u16);
        // C s_0_46: const #7s : i
        let s_0_46: i128 = 7;
        // C s_0_47: const #1u : u64
        let s_0_47: u64 = 1;
        // C s_0_48: cast zx s_0_47 -> bv
        let s_0_48: Bits = Bits::new(s_0_47 as u128, 64u16);
        // C s_0_49: lsl s_0_48 s_0_46
        let s_0_49: Bits = s_0_48 << s_0_46;
        // C s_0_50: sub s_0_49 s_0_48
        let s_0_50: Bits = ((s_0_49) - (s_0_48));
        // D s_0_51: and s_0_45 s_0_50
        let s_0_51: Bits = ((s_0_45) & (s_0_50));
        // D s_0_52: lsl s_0_51 s_0_43
        let s_0_52: Bits = s_0_51 << s_0_43;
        // C s_0_53: lsl s_0_50 s_0_43
        let s_0_53: Bits = s_0_50 << s_0_43;
        // C s_0_54: cmpl s_0_53
        let s_0_54: Bits = !s_0_53;
        // D s_0_55: and s_0_44 s_0_54
        let s_0_55: Bits = ((s_0_44) & (s_0_54));
        // D s_0_56: or s_0_55 s_0_52
        let s_0_56: Bits = ((s_0_55) | (s_0_52));
        // D s_0_57: cast reint s_0_56 -> u32
        let s_0_57: u32 = (s_0_56.value() as u32);
        // D s_0_58: write-var result <= s_0_57
        fn_state.result = s_0_57;
        // D s_0_59: read-var d:i64
        let s_0_59: i64 = fn_state.d;
        // D s_0_60: cast zx s_0_59 -> i
        let s_0_60: i128 = (i128::try_from(s_0_59).unwrap());
        // D s_0_61: call R_set(s_0_60, s_0_57)
        let s_0_61: () = R_set(state, tracer, s_0_60, s_0_57);
        // N s_0_62: return
        return;
    }
}
