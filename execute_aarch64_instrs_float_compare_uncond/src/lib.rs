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
use FPCompare::*;
use CheckFPEnabled64::*;
use FPZero::*;
use V_read::*;
use FPCR_read::*;
use common::*;
pub fn execute_aarch64_instrs_float_compare_uncond<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cmp_with_zero: bool,
    datasize: i64,
    m: i64,
    n: i64,
    signal_all_nans: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand1: Bits,
        split_vec: u8,
        datasizeshadow_1285: i64,
        operand2: Bits,
        cmp_with_zero: bool,
        datasize: i64,
        m: i64,
        n: i64,
        signal_all_nans: bool,
    }
    let fn_state = FunctionState {
        cmp_with_zero,
        datasize,
        m,
        n,
        signal_all_nans,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var datasize:i64
        let s_0_0: i64 = fn_state.datasize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var datasizeshadow#1285 <= s_0_2
        fn_state.datasizeshadow_1285 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckFPEnabled64(s_0_4)
        let s_0_5: () = CheckFPEnabled64(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var datasizeshadow#1285:i64
        let s_1_0: i64 = fn_state.datasizeshadow_1285;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: cast reint s_1_1 -> i64
        let s_1_2: i64 = (s_1_1 as i64);
        // D s_1_3: read-var n:i64
        let s_1_3: i64 = fn_state.n;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: call V_read(s_1_4, s_1_2)
        let s_1_5: Bits = V_read(state, tracer, s_1_4, s_1_2);
        // D s_1_6: write-var operand1 <= s_1_5
        fn_state.operand1 = s_1_5;
        // D s_1_7: read-var cmp_with_zero:u8
        let s_1_7: bool = fn_state.cmp_with_zero;
        // N s_1_8: branch s_1_7 b5 b2
        if s_1_7 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var datasizeshadow#1285:i64
        let s_2_0: i64 = fn_state.datasizeshadow_1285;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: cast reint s_2_1 -> i64
        let s_2_2: i64 = (s_2_1 as i64);
        // D s_2_3: read-var m:i64
        let s_2_3: i64 = fn_state.m;
        // D s_2_4: cast zx s_2_3 -> i
        let s_2_4: i128 = (i128::try_from(s_2_3).unwrap());
        // D s_2_5: call V_read(s_2_4, s_2_2)
        let s_2_5: Bits = V_read(state, tracer, s_2_4, s_2_2);
        // D s_2_6: write-var operand2 <= s_2_5
        fn_state.operand2 = s_2_5;
        // N s_2_7: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call FPCR_read(s_3_0)
        let s_3_1: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_3_0);
        // D s_3_2: read-var operand1:bv
        let s_3_2: Bits = fn_state.operand1;
        // D s_3_3: read-var operand2:bv
        let s_3_3: Bits = fn_state.operand2;
        // D s_3_4: read-var signal_all_nans:u8
        let s_3_4: bool = fn_state.signal_all_nans;
        // D s_3_5: call FPCompare(s_3_2, s_3_3, s_3_4, s_3_1)
        let s_3_5: u8 = FPCompare(state, tracer, s_3_2, s_3_3, s_3_4, s_3_1);
        // D s_3_6: write-var split_vec <= s_3_5
        fn_state.split_vec = s_3_5;
        // N s_3_7: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #3s : i
        let s_4_0: i128 = 3;
        // D s_4_1: read-var split_vec:u8
        let s_4_1: u8 = fn_state.split_vec;
        // D s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 4u16);
        // C s_4_3: const #1s : i64
        let s_4_3: i64 = 1;
        // C s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // C s_4_5: const #0s : i
        let s_4_5: i128 = 0;
        // C s_4_6: add s_4_5 s_4_4
        let s_4_6: i128 = (s_4_5 + s_4_4);
        // D s_4_7: bit-extract s_4_2 s_4_0 s_4_6
        let s_4_7: Bits = (Bits::new(
            ((s_4_2) >> (s_4_0)).value(),
            u16::try_from(s_4_6).unwrap(),
        ));
        // D s_4_8: cast reint s_4_7 -> u8
        let s_4_8: bool = ((s_4_7.value()) != 0);
        // C s_4_9: const #16984u : u32
        let s_4_9: u32 = 16984;
        // N s_4_10: write-reg s_4_9 <= s_4_8
        let s_4_10: () = {
            state.write_register::<bool>(s_4_9 as isize, s_4_8);
            tracer.write_register(s_4_9 as isize, s_4_8);
        };
        // C s_4_11: const #2s : i
        let s_4_11: i128 = 2;
        // D s_4_12: read-var split_vec:u8
        let s_4_12: u8 = fn_state.split_vec;
        // D s_4_13: cast zx s_4_12 -> bv
        let s_4_13: Bits = Bits::new(s_4_12 as u128, 4u16);
        // C s_4_14: const #1s : i64
        let s_4_14: i64 = 1;
        // C s_4_15: cast zx s_4_14 -> i
        let s_4_15: i128 = (i128::try_from(s_4_14).unwrap());
        // C s_4_16: const #0s : i
        let s_4_16: i128 = 0;
        // C s_4_17: add s_4_16 s_4_15
        let s_4_17: i128 = (s_4_16 + s_4_15);
        // D s_4_18: bit-extract s_4_13 s_4_11 s_4_17
        let s_4_18: Bits = (Bits::new(
            ((s_4_13) >> (s_4_11)).value(),
            u16::try_from(s_4_17).unwrap(),
        ));
        // D s_4_19: cast reint s_4_18 -> u8
        let s_4_19: bool = ((s_4_18.value()) != 0);
        // C s_4_20: const #16997u : u32
        let s_4_20: u32 = 16997;
        // N s_4_21: write-reg s_4_20 <= s_4_19
        let s_4_21: () = {
            state.write_register::<bool>(s_4_20 as isize, s_4_19);
            tracer.write_register(s_4_20 as isize, s_4_19);
        };
        // C s_4_22: const #1s : i
        let s_4_22: i128 = 1;
        // D s_4_23: read-var split_vec:u8
        let s_4_23: u8 = fn_state.split_vec;
        // D s_4_24: cast zx s_4_23 -> bv
        let s_4_24: Bits = Bits::new(s_4_23 as u128, 4u16);
        // C s_4_25: const #1s : i64
        let s_4_25: i64 = 1;
        // C s_4_26: cast zx s_4_25 -> i
        let s_4_26: i128 = (i128::try_from(s_4_25).unwrap());
        // C s_4_27: const #0s : i
        let s_4_27: i128 = 0;
        // C s_4_28: add s_4_27 s_4_26
        let s_4_28: i128 = (s_4_27 + s_4_26);
        // D s_4_29: bit-extract s_4_24 s_4_22 s_4_28
        let s_4_29: Bits = (Bits::new(
            ((s_4_24) >> (s_4_22)).value(),
            u16::try_from(s_4_28).unwrap(),
        ));
        // D s_4_30: cast reint s_4_29 -> u8
        let s_4_30: bool = ((s_4_29.value()) != 0);
        // C s_4_31: const #16971u : u32
        let s_4_31: u32 = 16971;
        // N s_4_32: write-reg s_4_31 <= s_4_30
        let s_4_32: () = {
            state.write_register::<bool>(s_4_31 as isize, s_4_30);
            tracer.write_register(s_4_31 as isize, s_4_30);
        };
        // C s_4_33: const #0s : i
        let s_4_33: i128 = 0;
        // D s_4_34: read-var split_vec:u8
        let s_4_34: u8 = fn_state.split_vec;
        // D s_4_35: cast zx s_4_34 -> bv
        let s_4_35: Bits = Bits::new(s_4_34 as u128, 4u16);
        // C s_4_36: const #1s : i64
        let s_4_36: i64 = 1;
        // C s_4_37: cast zx s_4_36 -> i
        let s_4_37: i128 = (i128::try_from(s_4_36).unwrap());
        // C s_4_38: const #0s : i
        let s_4_38: i128 = 0;
        // C s_4_39: add s_4_38 s_4_37
        let s_4_39: i128 = (s_4_38 + s_4_37);
        // D s_4_40: bit-extract s_4_35 s_4_33 s_4_39
        let s_4_40: Bits = (Bits::new(
            ((s_4_35) >> (s_4_33)).value(),
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
        // N s_4_44: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var datasizeshadow#1285:i64
        let s_5_0: i64 = fn_state.datasizeshadow_1285;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: cast reint s_5_1 -> i64
        let s_5_2: i64 = (s_5_1 as i64);
        // C s_5_3: const #0u : u8
        let s_5_3: bool = false;
        // D s_5_4: call FPZero(s_5_3, s_5_2)
        let s_5_4: Bits = FPZero(state, tracer, s_5_3, s_5_2);
        // D s_5_5: write-var operand2 <= s_5_4
        fn_state.operand2 = s_5_4;
        // N s_5_6: jump b3
        return block_3(state, tracer, fn_state);
    }
}
