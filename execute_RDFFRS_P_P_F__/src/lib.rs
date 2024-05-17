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
use PredTest::*;
use P_set::*;
use CheckNonStreamingSVEEnabled::*;
use P_read::*;
use FFR_read::*;
use common::*;
pub fn execute_RDFFRS_P_P_F__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    g: i64,
    setflags: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        result: Bits,
        PL: i64,
        mask: Bits,
        VL: i64,
        d: i64,
        g: i64,
        setflags: bool,
    }
    let fn_state = FunctionState {
        VL,
        d,
        g,
        setflags,
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
        // S s_0_1: call CheckNonStreamingSVEEnabled(s_0_0)
        let s_0_1: () = CheckNonStreamingSVEEnabled(state, tracer, s_0_0);
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
        // D s_1_5: write-var PL <= s_1_4
        fn_state.PL = s_1_4;
        // D s_1_6: read-var PL:i64
        let s_1_6: i64 = fn_state.PL;
        // D s_1_7: cast zx s_1_6 -> i
        let s_1_7: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_8: cast reint s_1_7 -> i64
        let s_1_8: i64 = (s_1_7 as i64);
        // D s_1_9: read-var g:i64
        let s_1_9: i64 = fn_state.g;
        // D s_1_10: cast zx s_1_9 -> i
        let s_1_10: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_11: cast zx s_1_8 -> i
        let s_1_11: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_12: call P_read(s_1_10, s_1_11)
        let s_1_12: Bits = P_read(state, tracer, s_1_10, s_1_11);
        // D s_1_13: write-var mask <= s_1_12
        fn_state.mask = s_1_12;
        // D s_1_14: read-var PL:i64
        let s_1_14: i64 = fn_state.PL;
        // D s_1_15: cast zx s_1_14 -> i
        let s_1_15: i128 = (i128::try_from(s_1_14).unwrap());
        // D s_1_16: cast reint s_1_15 -> i64
        let s_1_16: i64 = (s_1_15 as i64);
        // D s_1_17: cast zx s_1_16 -> i
        let s_1_17: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_18: call FFR_read(s_1_17)
        let s_1_18: Bits = FFR_read(state, tracer, s_1_17);
        // D s_1_19: read-var mask:bv
        let s_1_19: Bits = fn_state.mask;
        // D s_1_20: and s_1_18 s_1_19
        let s_1_20: Bits = ((s_1_18) & (s_1_19));
        // D s_1_21: write-var result <= s_1_20
        fn_state.result = s_1_20;
        // D s_1_22: read-var setflags:u8
        let s_1_22: bool = fn_state.setflags;
        // N s_1_23: branch s_1_22 b4 b2
        if s_1_22 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_2_0: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var PL:i64
        let s_3_0: i64 = fn_state.PL;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: cast reint s_3_1 -> i64
        let s_3_2: i64 = (s_3_1 as i64);
        // D s_3_3: read-var d:i64
        let s_3_3: i64 = fn_state.d;
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: cast zx s_3_2 -> i
        let s_3_5: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_6: read-var result:bv
        let s_3_6: Bits = fn_state.result;
        // D s_3_7: call P_set(s_3_4, s_3_5, s_3_6)
        let s_3_7: () = P_set(state, tracer, s_3_4, s_3_5, s_3_6);
        // N s_3_8: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #8s : i
        let s_4_0: i128 = 8;
        // D s_4_1: read-var mask:bv
        let s_4_1: Bits = fn_state.mask;
        // D s_4_2: read-var result:bv
        let s_4_2: Bits = fn_state.result;
        // D s_4_3: call PredTest(s_4_1, s_4_2, s_4_0)
        let s_4_3: u8 = PredTest(state, tracer, s_4_1, s_4_2, s_4_0);
        // C s_4_4: const #3s : i
        let s_4_4: i128 = 3;
        // D s_4_5: cast zx s_4_3 -> bv
        let s_4_5: Bits = Bits::new(s_4_3 as u128, 4u16);
        // C s_4_6: const #1s : i64
        let s_4_6: i64 = 1;
        // C s_4_7: cast zx s_4_6 -> i
        let s_4_7: i128 = (i128::try_from(s_4_6).unwrap());
        // C s_4_8: const #0s : i
        let s_4_8: i128 = 0;
        // C s_4_9: add s_4_8 s_4_7
        let s_4_9: i128 = (s_4_8 + s_4_7);
        // D s_4_10: bit-extract s_4_5 s_4_4 s_4_9
        let s_4_10: Bits = (Bits::new(
            ((s_4_5) >> (s_4_4)).value(),
            u16::try_from(s_4_9).unwrap(),
        ));
        // D s_4_11: cast reint s_4_10 -> u8
        let s_4_11: bool = ((s_4_10.value()) != 0);
        // C s_4_12: const #16984u : u32
        let s_4_12: u32 = 16984;
        // N s_4_13: write-reg s_4_12 <= s_4_11
        let s_4_13: () = {
            state.write_register::<bool>(s_4_12 as isize, s_4_11);
            tracer.write_register(s_4_12 as isize, s_4_11);
        };
        // C s_4_14: const #2s : i
        let s_4_14: i128 = 2;
        // D s_4_15: cast zx s_4_3 -> bv
        let s_4_15: Bits = Bits::new(s_4_3 as u128, 4u16);
        // C s_4_16: const #1s : i64
        let s_4_16: i64 = 1;
        // C s_4_17: cast zx s_4_16 -> i
        let s_4_17: i128 = (i128::try_from(s_4_16).unwrap());
        // C s_4_18: const #0s : i
        let s_4_18: i128 = 0;
        // C s_4_19: add s_4_18 s_4_17
        let s_4_19: i128 = (s_4_18 + s_4_17);
        // D s_4_20: bit-extract s_4_15 s_4_14 s_4_19
        let s_4_20: Bits = (Bits::new(
            ((s_4_15) >> (s_4_14)).value(),
            u16::try_from(s_4_19).unwrap(),
        ));
        // D s_4_21: cast reint s_4_20 -> u8
        let s_4_21: bool = ((s_4_20.value()) != 0);
        // C s_4_22: const #16997u : u32
        let s_4_22: u32 = 16997;
        // N s_4_23: write-reg s_4_22 <= s_4_21
        let s_4_23: () = {
            state.write_register::<bool>(s_4_22 as isize, s_4_21);
            tracer.write_register(s_4_22 as isize, s_4_21);
        };
        // C s_4_24: const #1s : i
        let s_4_24: i128 = 1;
        // D s_4_25: cast zx s_4_3 -> bv
        let s_4_25: Bits = Bits::new(s_4_3 as u128, 4u16);
        // C s_4_26: const #1s : i64
        let s_4_26: i64 = 1;
        // C s_4_27: cast zx s_4_26 -> i
        let s_4_27: i128 = (i128::try_from(s_4_26).unwrap());
        // C s_4_28: const #0s : i
        let s_4_28: i128 = 0;
        // C s_4_29: add s_4_28 s_4_27
        let s_4_29: i128 = (s_4_28 + s_4_27);
        // D s_4_30: bit-extract s_4_25 s_4_24 s_4_29
        let s_4_30: Bits = (Bits::new(
            ((s_4_25) >> (s_4_24)).value(),
            u16::try_from(s_4_29).unwrap(),
        ));
        // D s_4_31: cast reint s_4_30 -> u8
        let s_4_31: bool = ((s_4_30.value()) != 0);
        // C s_4_32: const #16971u : u32
        let s_4_32: u32 = 16971;
        // N s_4_33: write-reg s_4_32 <= s_4_31
        let s_4_33: () = {
            state.write_register::<bool>(s_4_32 as isize, s_4_31);
            tracer.write_register(s_4_32 as isize, s_4_31);
        };
        // C s_4_34: const #0s : i
        let s_4_34: i128 = 0;
        // D s_4_35: cast zx s_4_3 -> bv
        let s_4_35: Bits = Bits::new(s_4_3 as u128, 4u16);
        // C s_4_36: const #1s : i64
        let s_4_36: i64 = 1;
        // C s_4_37: cast zx s_4_36 -> i
        let s_4_37: i128 = (i128::try_from(s_4_36).unwrap());
        // C s_4_38: const #0s : i
        let s_4_38: i128 = 0;
        // C s_4_39: add s_4_38 s_4_37
        let s_4_39: i128 = (s_4_38 + s_4_37);
        // D s_4_40: bit-extract s_4_35 s_4_34 s_4_39
        let s_4_40: Bits = (Bits::new(
            ((s_4_35) >> (s_4_34)).value(),
            u16::try_from(s_4_39).unwrap(),
        ));
        // D s_4_41: cast reint s_4_40 -> u8
        let s_4_41: bool = ((s_4_40.value()) != 0);
        // C s_4_42: const #16996u : u32
        let s_4_42: u32 = 16996;
        // N s_4_43: write-reg s_4_42 <= s_4_41
        let s_4_43: () = {
            state.write_register::<bool>(s_4_42 as isize, s_4_41);
            tracer.write_register(s_4_42 as isize, s_4_41);
        };
        // N s_4_44: jump b3
        return block_3(state, tracer, fn_state);
    }
}
