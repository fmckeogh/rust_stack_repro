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
use Ones::*;
use PredTest::*;
use Zeros::*;
use CheckSVEEnabled::*;
use P_set::*;
use P_read::*;
use LastActive::*;
use common::*;
pub fn execute_BRKNS_P_P_PP__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    dm: i64,
    g: i64,
    n: i64,
    setflags: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        result: Bits,
        PL: i64,
        operand2: Bits,
        VL: i64,
        dm: i64,
        g: i64,
        n: i64,
        setflags: bool,
    }
    let fn_state = FunctionState {
        VL,
        dm,
        g,
        n,
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
        // D s_1_13: read-var PL:i64
        let s_1_13: i64 = fn_state.PL;
        // D s_1_14: cast zx s_1_13 -> i
        let s_1_14: i128 = (i128::try_from(s_1_13).unwrap());
        // D s_1_15: cast reint s_1_14 -> i64
        let s_1_15: i64 = (s_1_14 as i64);
        // D s_1_16: read-var n:i64
        let s_1_16: i64 = fn_state.n;
        // D s_1_17: cast zx s_1_16 -> i
        let s_1_17: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_18: cast zx s_1_15 -> i
        let s_1_18: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_19: call P_read(s_1_17, s_1_18)
        let s_1_19: Bits = P_read(state, tracer, s_1_17, s_1_18);
        // D s_1_20: read-var PL:i64
        let s_1_20: i64 = fn_state.PL;
        // D s_1_21: cast zx s_1_20 -> i
        let s_1_21: i128 = (i128::try_from(s_1_20).unwrap());
        // D s_1_22: cast reint s_1_21 -> i64
        let s_1_22: i64 = (s_1_21 as i64);
        // D s_1_23: read-var dm:i64
        let s_1_23: i64 = fn_state.dm;
        // D s_1_24: cast zx s_1_23 -> i
        let s_1_24: i128 = (i128::try_from(s_1_23).unwrap());
        // D s_1_25: cast zx s_1_22 -> i
        let s_1_25: i128 = (i128::try_from(s_1_22).unwrap());
        // D s_1_26: call P_read(s_1_24, s_1_25)
        let s_1_26: Bits = P_read(state, tracer, s_1_24, s_1_25);
        // D s_1_27: write-var operand2 <= s_1_26
        fn_state.operand2 = s_1_26;
        // C s_1_28: const #8s : i
        let s_1_28: i128 = 8;
        // D s_1_29: call LastActive(s_1_12, s_1_19, s_1_28)
        let s_1_29: bool = LastActive(state, tracer, s_1_12, s_1_19, s_1_28);
        // D s_1_30: cast zx s_1_29 -> bv
        let s_1_30: Bits = Bits::new(s_1_29 as u128, 1u16);
        // C s_1_31: const #1u : u8
        let s_1_31: bool = true;
        // C s_1_32: cast zx s_1_31 -> bv
        let s_1_32: Bits = Bits::new(s_1_31 as u128, 1u16);
        // D s_1_33: cmp-eq s_1_30 s_1_32
        let s_1_33: bool = ((s_1_30) == (s_1_32));
        // N s_1_34: branch s_1_33 b7 b2
        if s_1_33 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var PL:i64
        let s_2_0: i64 = fn_state.PL;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: call Zeros(s_2_1)
        let s_2_2: Bits = Zeros(state, tracer, s_2_1);
        // D s_2_3: write-var result <= s_2_2
        fn_state.result = s_2_2;
        // N s_2_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var setflags:u8
        let s_3_0: bool = fn_state.setflags;
        // N s_3_1: branch s_3_0 b6 b4
        if s_3_0 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var PL:i64
        let s_5_0: i64 = fn_state.PL;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: cast reint s_5_1 -> i64
        let s_5_2: i64 = (s_5_1 as i64);
        // D s_5_3: read-var dm:i64
        let s_5_3: i64 = fn_state.dm;
        // D s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // D s_5_5: cast zx s_5_2 -> i
        let s_5_5: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_6: read-var result:bv
        let s_5_6: Bits = fn_state.result;
        // D s_5_7: call P_set(s_5_4, s_5_5, s_5_6)
        let s_5_7: () = P_set(state, tracer, s_5_4, s_5_5, s_5_6);
        // N s_5_8: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var PL:i64
        let s_6_0: i64 = fn_state.PL;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: call Ones(s_6_1)
        let s_6_2: Bits = Ones(state, tracer, s_6_1);
        // C s_6_3: const #8s : i
        let s_6_3: i128 = 8;
        // D s_6_4: read-var result:bv
        let s_6_4: Bits = fn_state.result;
        // D s_6_5: call PredTest(s_6_2, s_6_4, s_6_3)
        let s_6_5: u8 = PredTest(state, tracer, s_6_2, s_6_4, s_6_3);
        // C s_6_6: const #3s : i
        let s_6_6: i128 = 3;
        // D s_6_7: cast zx s_6_5 -> bv
        let s_6_7: Bits = Bits::new(s_6_5 as u128, 4u16);
        // C s_6_8: const #1s : i64
        let s_6_8: i64 = 1;
        // C s_6_9: cast zx s_6_8 -> i
        let s_6_9: i128 = (i128::try_from(s_6_8).unwrap());
        // C s_6_10: const #0s : i
        let s_6_10: i128 = 0;
        // C s_6_11: add s_6_10 s_6_9
        let s_6_11: i128 = (s_6_10 + s_6_9);
        // D s_6_12: bit-extract s_6_7 s_6_6 s_6_11
        let s_6_12: Bits = (Bits::new(
            ((s_6_7) >> (s_6_6)).value(),
            u16::try_from(s_6_11).unwrap(),
        ));
        // D s_6_13: cast reint s_6_12 -> u8
        let s_6_13: bool = ((s_6_12.value()) != 0);
        // C s_6_14: const #16984u : u32
        let s_6_14: u32 = 16984;
        // N s_6_15: write-reg s_6_14 <= s_6_13
        let s_6_15: () = {
            state.write_register::<bool>(s_6_14 as isize, s_6_13);
            tracer.write_register(s_6_14 as isize, s_6_13);
        };
        // C s_6_16: const #2s : i
        let s_6_16: i128 = 2;
        // D s_6_17: cast zx s_6_5 -> bv
        let s_6_17: Bits = Bits::new(s_6_5 as u128, 4u16);
        // C s_6_18: const #1s : i64
        let s_6_18: i64 = 1;
        // C s_6_19: cast zx s_6_18 -> i
        let s_6_19: i128 = (i128::try_from(s_6_18).unwrap());
        // C s_6_20: const #0s : i
        let s_6_20: i128 = 0;
        // C s_6_21: add s_6_20 s_6_19
        let s_6_21: i128 = (s_6_20 + s_6_19);
        // D s_6_22: bit-extract s_6_17 s_6_16 s_6_21
        let s_6_22: Bits = (Bits::new(
            ((s_6_17) >> (s_6_16)).value(),
            u16::try_from(s_6_21).unwrap(),
        ));
        // D s_6_23: cast reint s_6_22 -> u8
        let s_6_23: bool = ((s_6_22.value()) != 0);
        // C s_6_24: const #16997u : u32
        let s_6_24: u32 = 16997;
        // N s_6_25: write-reg s_6_24 <= s_6_23
        let s_6_25: () = {
            state.write_register::<bool>(s_6_24 as isize, s_6_23);
            tracer.write_register(s_6_24 as isize, s_6_23);
        };
        // C s_6_26: const #1s : i
        let s_6_26: i128 = 1;
        // D s_6_27: cast zx s_6_5 -> bv
        let s_6_27: Bits = Bits::new(s_6_5 as u128, 4u16);
        // C s_6_28: const #1s : i64
        let s_6_28: i64 = 1;
        // C s_6_29: cast zx s_6_28 -> i
        let s_6_29: i128 = (i128::try_from(s_6_28).unwrap());
        // C s_6_30: const #0s : i
        let s_6_30: i128 = 0;
        // C s_6_31: add s_6_30 s_6_29
        let s_6_31: i128 = (s_6_30 + s_6_29);
        // D s_6_32: bit-extract s_6_27 s_6_26 s_6_31
        let s_6_32: Bits = (Bits::new(
            ((s_6_27) >> (s_6_26)).value(),
            u16::try_from(s_6_31).unwrap(),
        ));
        // D s_6_33: cast reint s_6_32 -> u8
        let s_6_33: bool = ((s_6_32.value()) != 0);
        // C s_6_34: const #16971u : u32
        let s_6_34: u32 = 16971;
        // N s_6_35: write-reg s_6_34 <= s_6_33
        let s_6_35: () = {
            state.write_register::<bool>(s_6_34 as isize, s_6_33);
            tracer.write_register(s_6_34 as isize, s_6_33);
        };
        // C s_6_36: const #0s : i
        let s_6_36: i128 = 0;
        // D s_6_37: cast zx s_6_5 -> bv
        let s_6_37: Bits = Bits::new(s_6_5 as u128, 4u16);
        // C s_6_38: const #1s : i64
        let s_6_38: i64 = 1;
        // C s_6_39: cast zx s_6_38 -> i
        let s_6_39: i128 = (i128::try_from(s_6_38).unwrap());
        // C s_6_40: const #0s : i
        let s_6_40: i128 = 0;
        // C s_6_41: add s_6_40 s_6_39
        let s_6_41: i128 = (s_6_40 + s_6_39);
        // D s_6_42: bit-extract s_6_37 s_6_36 s_6_41
        let s_6_42: Bits = (Bits::new(
            ((s_6_37) >> (s_6_36)).value(),
            u16::try_from(s_6_41).unwrap(),
        ));
        // D s_6_43: cast reint s_6_42 -> u8
        let s_6_43: bool = ((s_6_42.value()) != 0);
        // C s_6_44: const #16996u : u32
        let s_6_44: u32 = 16996;
        // N s_6_45: write-reg s_6_44 <= s_6_43
        let s_6_45: () = {
            state.write_register::<bool>(s_6_44 as isize, s_6_43);
            tracer.write_register(s_6_44 as isize, s_6_43);
        };
        // N s_6_46: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var operand2:bv
        let s_7_0: Bits = fn_state.operand2;
        // D s_7_1: write-var result <= s_7_0
        fn_state.result = s_7_0;
        // N s_7_2: jump b3
        return block_3(state, tracer, fn_state);
    }
}
