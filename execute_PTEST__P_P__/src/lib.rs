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
use CheckSVEEnabled::*;
use PredTest::*;
use P_read::*;
use common::*;
pub fn execute_PTEST__P_P__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    esize: i64,
    g: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        VL: i64,
        esize: i64,
        g: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        VL,
        esize,
        g,
        n,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call CheckSVEEnabled(s_0_0)
        let s_0_1: () = CheckSVEEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VL:i64
        let s_1_0: i64 = fn_state.VL;
        // C s_1_1: const #8s : i
        let s_1_1: i128 = 8;
        // D s_1_2: cast zx s_1_0 -> i
        let s_1_2: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_3: div s_1_2 s_1_1
        let s_1_3: i128 = ((s_1_2) / (s_1_1));
        // D s_1_4: cast reint s_1_3 -> i64
        let s_1_4: i64 = (s_1_3 as i64);
        // D s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var g:i64
        let s_1_7: i64 = fn_state.g;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: cast zx s_1_6 -> i
        let s_1_9: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_10: call P_read(s_1_8, s_1_9)
        let s_1_10: Bits = P_read(state, tracer, s_1_8, s_1_9);
        // D s_1_11: cast zx s_1_4 -> i
        let s_1_11: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_12: cast reint s_1_11 -> i64
        let s_1_12: i64 = (s_1_11 as i64);
        // D s_1_13: read-var n:i64
        let s_1_13: i64 = fn_state.n;
        // D s_1_14: cast zx s_1_13 -> i
        let s_1_14: i128 = (i128::try_from(s_1_13).unwrap());
        // D s_1_15: cast zx s_1_12 -> i
        let s_1_15: i128 = (i128::try_from(s_1_12).unwrap());
        // D s_1_16: call P_read(s_1_14, s_1_15)
        let s_1_16: Bits = P_read(state, tracer, s_1_14, s_1_15);
        // D s_1_17: read-var esize:i64
        let s_1_17: i64 = fn_state.esize;
        // D s_1_18: cast zx s_1_17 -> i
        let s_1_18: i128 = (i128::try_from(s_1_17).unwrap());
        // D s_1_19: call PredTest(s_1_10, s_1_16, s_1_18)
        let s_1_19: u8 = PredTest(state, tracer, s_1_10, s_1_16, s_1_18);
        // C s_1_20: const #3s : i
        let s_1_20: i128 = 3;
        // D s_1_21: cast zx s_1_19 -> bv
        let s_1_21: Bits = Bits::new(s_1_19 as u128, 4u16);
        // C s_1_22: const #1s : i64
        let s_1_22: i64 = 1;
        // C s_1_23: cast zx s_1_22 -> i
        let s_1_23: i128 = (i128::try_from(s_1_22).unwrap());
        // C s_1_24: const #0s : i
        let s_1_24: i128 = 0;
        // C s_1_25: add s_1_24 s_1_23
        let s_1_25: i128 = (s_1_24 + s_1_23);
        // D s_1_26: bit-extract s_1_21 s_1_20 s_1_25
        let s_1_26: Bits = (Bits::new(
            ((s_1_21) >> (s_1_20)).value(),
            u16::try_from(s_1_25).unwrap(),
        ));
        // D s_1_27: cast reint s_1_26 -> u8
        let s_1_27: bool = ((s_1_26.value()) != 0);
        // C s_1_28: const #16984u : u32
        let s_1_28: u32 = 16984;
        // N s_1_29: write-reg s_1_28 <= s_1_27
        let s_1_29: () = {
            state.write_register::<bool>(s_1_28 as isize, s_1_27);
            tracer.write_register(s_1_28 as isize, s_1_27);
        };
        // C s_1_30: const #2s : i
        let s_1_30: i128 = 2;
        // D s_1_31: cast zx s_1_19 -> bv
        let s_1_31: Bits = Bits::new(s_1_19 as u128, 4u16);
        // C s_1_32: const #1s : i64
        let s_1_32: i64 = 1;
        // C s_1_33: cast zx s_1_32 -> i
        let s_1_33: i128 = (i128::try_from(s_1_32).unwrap());
        // C s_1_34: const #0s : i
        let s_1_34: i128 = 0;
        // C s_1_35: add s_1_34 s_1_33
        let s_1_35: i128 = (s_1_34 + s_1_33);
        // D s_1_36: bit-extract s_1_31 s_1_30 s_1_35
        let s_1_36: Bits = (Bits::new(
            ((s_1_31) >> (s_1_30)).value(),
            u16::try_from(s_1_35).unwrap(),
        ));
        // D s_1_37: cast reint s_1_36 -> u8
        let s_1_37: bool = ((s_1_36.value()) != 0);
        // C s_1_38: const #16997u : u32
        let s_1_38: u32 = 16997;
        // N s_1_39: write-reg s_1_38 <= s_1_37
        let s_1_39: () = {
            state.write_register::<bool>(s_1_38 as isize, s_1_37);
            tracer.write_register(s_1_38 as isize, s_1_37);
        };
        // C s_1_40: const #1s : i
        let s_1_40: i128 = 1;
        // D s_1_41: cast zx s_1_19 -> bv
        let s_1_41: Bits = Bits::new(s_1_19 as u128, 4u16);
        // C s_1_42: const #1s : i64
        let s_1_42: i64 = 1;
        // C s_1_43: cast zx s_1_42 -> i
        let s_1_43: i128 = (i128::try_from(s_1_42).unwrap());
        // C s_1_44: const #0s : i
        let s_1_44: i128 = 0;
        // C s_1_45: add s_1_44 s_1_43
        let s_1_45: i128 = (s_1_44 + s_1_43);
        // D s_1_46: bit-extract s_1_41 s_1_40 s_1_45
        let s_1_46: Bits = (Bits::new(
            ((s_1_41) >> (s_1_40)).value(),
            u16::try_from(s_1_45).unwrap(),
        ));
        // D s_1_47: cast reint s_1_46 -> u8
        let s_1_47: bool = ((s_1_46.value()) != 0);
        // C s_1_48: const #16971u : u32
        let s_1_48: u32 = 16971;
        // N s_1_49: write-reg s_1_48 <= s_1_47
        let s_1_49: () = {
            state.write_register::<bool>(s_1_48 as isize, s_1_47);
            tracer.write_register(s_1_48 as isize, s_1_47);
        };
        // C s_1_50: const #0s : i
        let s_1_50: i128 = 0;
        // D s_1_51: cast zx s_1_19 -> bv
        let s_1_51: Bits = Bits::new(s_1_19 as u128, 4u16);
        // C s_1_52: const #1s : i64
        let s_1_52: i64 = 1;
        // C s_1_53: cast zx s_1_52 -> i
        let s_1_53: i128 = (i128::try_from(s_1_52).unwrap());
        // C s_1_54: const #0s : i
        let s_1_54: i128 = 0;
        // C s_1_55: add s_1_54 s_1_53
        let s_1_55: i128 = (s_1_54 + s_1_53);
        // D s_1_56: bit-extract s_1_51 s_1_50 s_1_55
        let s_1_56: Bits = (Bits::new(
            ((s_1_51) >> (s_1_50)).value(),
            u16::try_from(s_1_55).unwrap(),
        ));
        // D s_1_57: cast reint s_1_56 -> u8
        let s_1_57: bool = ((s_1_56.value()) != 0);
        // C s_1_58: const #16996u : u32
        let s_1_58: u32 = 16996;
        // N s_1_59: write-reg s_1_58 <= s_1_57
        let s_1_59: () = {
            state.write_register::<bool>(s_1_58 as isize, s_1_57);
            tracer.write_register(s_1_58 as isize, s_1_57);
        };
        // N s_1_60: return
        return;
    }
}
