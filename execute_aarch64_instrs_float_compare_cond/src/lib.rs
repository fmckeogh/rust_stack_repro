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
use V_read::*;
use SPESampleAddOpOther::*;
use ConditionHolds::*;
use FPCR_read::*;
use common::*;
pub fn execute_aarch64_instrs_float_compare_cond<T: Tracer>(
    state: &mut State,
    tracer: &T,
    condition: u8,
    datasize: i64,
    flags__arg: u8,
    m: i64,
    n: i64,
    signal_all_nans: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        datasizeshadow_1253: i64,
        taken: bool,
        operand1: Bits,
        flags: u8,
        operand2: Bits,
        condition: u8,
        datasize: i64,
        flags__arg: u8,
        m: i64,
        n: i64,
        signal_all_nans: bool,
    }
    let fn_state = FunctionState {
        condition,
        datasize,
        flags__arg,
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
        // D s_0_3: write-var datasizeshadow#1253 <= s_0_2
        fn_state.datasizeshadow_1253 = s_0_2;
        // D s_0_4: read-var flags__arg:u8
        let s_0_4: u8 = fn_state.flags__arg;
        // D s_0_5: write-var flags <= s_0_4
        fn_state.flags = s_0_4;
        // C s_0_6: const #() : ()
        let s_0_6: () = ();
        // S s_0_7: call CheckFPEnabled64(s_0_6)
        let s_0_7: () = CheckFPEnabled64(state, tracer, s_0_6);
        // N s_0_8: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var datasizeshadow#1253:i64
        let s_1_0: i64 = fn_state.datasizeshadow_1253;
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
        // D s_1_7: read-var datasizeshadow#1253:i64
        let s_1_7: i64 = fn_state.datasizeshadow_1253;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: cast reint s_1_8 -> i64
        let s_1_9: i64 = (s_1_8 as i64);
        // D s_1_10: read-var m:i64
        let s_1_10: i64 = fn_state.m;
        // D s_1_11: cast zx s_1_10 -> i
        let s_1_11: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_12: call V_read(s_1_11, s_1_9)
        let s_1_12: Bits = V_read(state, tracer, s_1_11, s_1_9);
        // D s_1_13: write-var operand2 <= s_1_12
        fn_state.operand2 = s_1_12;
        // C s_1_14: const #0u : u8
        let s_1_14: bool = false;
        // D s_1_15: write-var taken <= s_1_14
        fn_state.taken = s_1_14;
        // D s_1_16: read-var condition:u8
        let s_1_16: u8 = fn_state.condition;
        // D s_1_17: call ConditionHolds(s_1_16)
        let s_1_17: bool = ConditionHolds(state, tracer, s_1_16);
        // N s_1_18: branch s_1_17 b6 b2
        if s_1_17 {
            return block_6(state, tracer, fn_state);
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
        // C s_3_0: const #3s : i
        let s_3_0: i128 = 3;
        // D s_3_1: read-var flags:u8
        let s_3_1: u8 = fn_state.flags;
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 4u16);
        // C s_3_3: const #1s : i64
        let s_3_3: i64 = 1;
        // C s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // C s_3_5: const #0s : i
        let s_3_5: i128 = 0;
        // C s_3_6: add s_3_5 s_3_4
        let s_3_6: i128 = (s_3_5 + s_3_4);
        // D s_3_7: bit-extract s_3_2 s_3_0 s_3_6
        let s_3_7: Bits = (Bits::new(
            ((s_3_2) >> (s_3_0)).value(),
            u16::try_from(s_3_6).unwrap(),
        ));
        // D s_3_8: cast reint s_3_7 -> u8
        let s_3_8: bool = ((s_3_7.value()) != 0);
        // C s_3_9: const #16984u : u32
        let s_3_9: u32 = 16984;
        // N s_3_10: write-reg s_3_9 <= s_3_8
        let s_3_10: () = {
            state.write_register::<bool>(s_3_9 as isize, s_3_8);
            tracer.write_register(s_3_9 as isize, s_3_8);
        };
        // C s_3_11: const #2s : i
        let s_3_11: i128 = 2;
        // D s_3_12: read-var flags:u8
        let s_3_12: u8 = fn_state.flags;
        // D s_3_13: cast zx s_3_12 -> bv
        let s_3_13: Bits = Bits::new(s_3_12 as u128, 4u16);
        // C s_3_14: const #1s : i64
        let s_3_14: i64 = 1;
        // C s_3_15: cast zx s_3_14 -> i
        let s_3_15: i128 = (i128::try_from(s_3_14).unwrap());
        // C s_3_16: const #0s : i
        let s_3_16: i128 = 0;
        // C s_3_17: add s_3_16 s_3_15
        let s_3_17: i128 = (s_3_16 + s_3_15);
        // D s_3_18: bit-extract s_3_13 s_3_11 s_3_17
        let s_3_18: Bits = (Bits::new(
            ((s_3_13) >> (s_3_11)).value(),
            u16::try_from(s_3_17).unwrap(),
        ));
        // D s_3_19: cast reint s_3_18 -> u8
        let s_3_19: bool = ((s_3_18.value()) != 0);
        // C s_3_20: const #16997u : u32
        let s_3_20: u32 = 16997;
        // N s_3_21: write-reg s_3_20 <= s_3_19
        let s_3_21: () = {
            state.write_register::<bool>(s_3_20 as isize, s_3_19);
            tracer.write_register(s_3_20 as isize, s_3_19);
        };
        // C s_3_22: const #1s : i
        let s_3_22: i128 = 1;
        // D s_3_23: read-var flags:u8
        let s_3_23: u8 = fn_state.flags;
        // D s_3_24: cast zx s_3_23 -> bv
        let s_3_24: Bits = Bits::new(s_3_23 as u128, 4u16);
        // C s_3_25: const #1s : i64
        let s_3_25: i64 = 1;
        // C s_3_26: cast zx s_3_25 -> i
        let s_3_26: i128 = (i128::try_from(s_3_25).unwrap());
        // C s_3_27: const #0s : i
        let s_3_27: i128 = 0;
        // C s_3_28: add s_3_27 s_3_26
        let s_3_28: i128 = (s_3_27 + s_3_26);
        // D s_3_29: bit-extract s_3_24 s_3_22 s_3_28
        let s_3_29: Bits = (Bits::new(
            ((s_3_24) >> (s_3_22)).value(),
            u16::try_from(s_3_28).unwrap(),
        ));
        // D s_3_30: cast reint s_3_29 -> u8
        let s_3_30: bool = ((s_3_29.value()) != 0);
        // C s_3_31: const #16971u : u32
        let s_3_31: u32 = 16971;
        // N s_3_32: write-reg s_3_31 <= s_3_30
        let s_3_32: () = {
            state.write_register::<bool>(s_3_31 as isize, s_3_30);
            tracer.write_register(s_3_31 as isize, s_3_30);
        };
        // C s_3_33: const #0s : i
        let s_3_33: i128 = 0;
        // D s_3_34: read-var flags:u8
        let s_3_34: u8 = fn_state.flags;
        // D s_3_35: cast zx s_3_34 -> bv
        let s_3_35: Bits = Bits::new(s_3_34 as u128, 4u16);
        // C s_3_36: const #1s : i64
        let s_3_36: i64 = 1;
        // C s_3_37: cast zx s_3_36 -> i
        let s_3_37: i128 = (i128::try_from(s_3_36).unwrap());
        // C s_3_38: const #0s : i
        let s_3_38: i128 = 0;
        // C s_3_39: add s_3_38 s_3_37
        let s_3_39: i128 = (s_3_38 + s_3_37);
        // D s_3_40: bit-extract s_3_35 s_3_33 s_3_39
        let s_3_40: Bits = (Bits::new(
            ((s_3_35) >> (s_3_33)).value(),
            u16::try_from(s_3_39).unwrap(),
        ));
        // D s_3_41: cast reint s_3_40 -> u8
        let s_3_41: bool = ((s_3_40.value()) != 0);
        // C s_3_42: const #16996u : u32
        let s_3_42: u32 = 16996;
        // N s_3_43: write-reg s_3_42 <= s_3_41
        let s_3_43: () = {
            state.write_register::<bool>(s_3_42 as isize, s_3_41);
            tracer.write_register(s_3_42 as isize, s_3_41);
        };
        // C s_3_44: const #22416u : u32
        let s_3_44: u32 = 22416;
        // D s_3_45: read-reg s_3_44:u8
        let s_3_45: bool = {
            let value = state.read_register::<bool>(s_3_44 as isize);
            tracer.read_register(s_3_44 as isize, value);
            value
        };
        // N s_3_46: branch s_3_45 b5 b4
        if s_3_45 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #1u : u8
        let s_5_0: bool = true;
        // D s_5_1: read-var taken:u8
        let s_5_1: bool = fn_state.taken;
        // D s_5_2: call SPESampleAddOpOther(s_5_0, s_5_1)
        let s_5_2: () = SPESampleAddOpOther(state, tracer, s_5_0, s_5_1);
        // N s_5_3: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call FPCR_read(s_6_0)
        let s_6_1: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_6_0);
        // D s_6_2: read-var operand1:bv
        let s_6_2: Bits = fn_state.operand1;
        // D s_6_3: read-var operand2:bv
        let s_6_3: Bits = fn_state.operand2;
        // D s_6_4: read-var signal_all_nans:u8
        let s_6_4: bool = fn_state.signal_all_nans;
        // D s_6_5: call FPCompare(s_6_2, s_6_3, s_6_4, s_6_1)
        let s_6_5: u8 = FPCompare(state, tracer, s_6_2, s_6_3, s_6_4, s_6_1);
        // D s_6_6: write-var flags <= s_6_5
        fn_state.flags = s_6_5;
        // N s_6_7: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #1u : u8
        let s_7_0: bool = true;
        // D s_7_1: write-var taken <= s_7_0
        fn_state.taken = s_7_0;
        // N s_7_2: jump b3
        return block_3(state, tracer, fn_state);
    }
}
