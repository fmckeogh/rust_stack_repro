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
use X_set::*;
use X_read::*;
use SP_read::*;
use AddWithCarry::*;
use common::*;
pub fn execute_aarch64_instrs_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags<
    T: Tracer,
>(state: &mut State, tracer: &T, d: i64, m: i64, n: i64, setflags: bool) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_718634: ProductTyped54bc449dd09e5bd,
        nzcv: u8,
        result: u64,
        operand1: u64,
        operand2: u64,
        d: i64,
        m: i64,
        n: i64,
        setflags: bool,
    }
    let fn_state = FunctionState {
        d,
        m,
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
        // C s_0_0: const #31s : i
        let s_0_0: i128 = 31;
        // D s_0_1: read-var n:i64
        let s_0_1: i64 = fn_state.n;
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (i128::try_from(s_0_1).unwrap());
        // D s_0_3: cmp-eq s_0_2 s_0_0
        let s_0_3: bool = ((s_0_2) == (s_0_0));
        // N s_0_4: branch s_0_3 b9 b1
        if s_0_3 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #64s : i64
        let s_1_0: i64 = 64;
        // D s_1_1: read-var n:i64
        let s_1_1: i64 = fn_state.n;
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: call X_read(s_1_2, s_1_0)
        let s_1_3: Bits = X_read(state, tracer, s_1_2, s_1_0);
        // D s_1_4: cast reint s_1_3 -> u64
        let s_1_4: u64 = (s_1_3.value() as u64);
        // D s_1_5: write-var operand1 <= s_1_4
        fn_state.operand1 = s_1_4;
        // N s_1_6: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #31s : i
        let s_2_0: i128 = 31;
        // D s_2_1: read-var m:i64
        let s_2_1: i64 = fn_state.m;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: cmp-eq s_2_2 s_2_0
        let s_2_3: bool = ((s_2_2) == (s_2_0));
        // N s_2_4: branch s_2_3 b8 b3
        if s_2_3 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #64s : i64
        let s_3_0: i64 = 64;
        // D s_3_1: read-var m:i64
        let s_3_1: i64 = fn_state.m;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: call X_read(s_3_2, s_3_0)
        let s_3_3: Bits = X_read(state, tracer, s_3_2, s_3_0);
        // D s_3_4: cast reint s_3_3 -> u64
        let s_3_4: u64 = (s_3_3.value() as u64);
        // D s_3_5: write-var operand2 <= s_3_4
        fn_state.operand2 = s_3_4;
        // N s_3_6: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0s : i
        let s_4_0: i128 = 0;
        // D s_4_1: read-var operand1:u64
        let s_4_1: u64 = fn_state.operand1;
        // D s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 64u16);
        // C s_4_3: const #1s : i64
        let s_4_3: i64 = 1;
        // C s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // C s_4_5: const #55s : i
        let s_4_5: i128 = 55;
        // C s_4_6: add s_4_5 s_4_4
        let s_4_6: i128 = (s_4_5 + s_4_4);
        // D s_4_7: bit-extract s_4_2 s_4_0 s_4_6
        let s_4_7: Bits = (Bits::new(
            ((s_4_2) >> (s_4_0)).value(),
            u16::try_from(s_4_6).unwrap(),
        ));
        // D s_4_8: cast reint s_4_7 -> u56
        let s_4_8: u64 = (s_4_7.value() as u64);
        // C s_4_9: const #64s : i
        let s_4_9: i128 = 64;
        // D s_4_10: cast zx s_4_8 -> bv
        let s_4_10: Bits = Bits::new(s_4_8 as u128, 56u16);
        // D s_4_11: bits-cast sx s_4_10 -> bv length s_4_9
        let s_4_11: Bits = s_4_10.sign_extend(s_4_9);
        // D s_4_12: cast reint s_4_11 -> u64
        let s_4_12: u64 = (s_4_11.value() as u64);
        // C s_4_13: const #0s : i
        let s_4_13: i128 = 0;
        // D s_4_14: read-var operand2:u64
        let s_4_14: u64 = fn_state.operand2;
        // D s_4_15: cast zx s_4_14 -> bv
        let s_4_15: Bits = Bits::new(s_4_14 as u128, 64u16);
        // C s_4_16: const #1s : i64
        let s_4_16: i64 = 1;
        // C s_4_17: cast zx s_4_16 -> i
        let s_4_17: i128 = (i128::try_from(s_4_16).unwrap());
        // C s_4_18: const #55s : i
        let s_4_18: i128 = 55;
        // C s_4_19: add s_4_18 s_4_17
        let s_4_19: i128 = (s_4_18 + s_4_17);
        // D s_4_20: bit-extract s_4_15 s_4_13 s_4_19
        let s_4_20: Bits = (Bits::new(
            ((s_4_15) >> (s_4_13)).value(),
            u16::try_from(s_4_19).unwrap(),
        ));
        // D s_4_21: cast reint s_4_20 -> u56
        let s_4_21: u64 = (s_4_20.value() as u64);
        // C s_4_22: const #64s : i
        let s_4_22: i128 = 64;
        // D s_4_23: cast zx s_4_21 -> bv
        let s_4_23: Bits = Bits::new(s_4_21 as u128, 56u16);
        // D s_4_24: bits-cast sx s_4_23 -> bv length s_4_22
        let s_4_24: Bits = s_4_23.sign_extend(s_4_22);
        // D s_4_25: cast reint s_4_24 -> u64
        let s_4_25: u64 = (s_4_24.value() as u64);
        // D s_4_26: write-var operand2 <= s_4_25
        fn_state.operand2 = s_4_25;
        // D s_4_27: read-var operand2:u64
        let s_4_27: u64 = fn_state.operand2;
        // D s_4_28: cast zx s_4_27 -> bv
        let s_4_28: Bits = Bits::new(s_4_27 as u128, 64u16);
        // D s_4_29: not s_4_28
        let s_4_29: Bits = !s_4_28;
        // D s_4_30: cast reint s_4_29 -> u64
        let s_4_30: u64 = (s_4_29.value() as u64);
        // D s_4_31: cast zx s_4_12 -> bv
        let s_4_31: Bits = Bits::new(s_4_12 as u128, 64u16);
        // D s_4_32: cast zx s_4_30 -> bv
        let s_4_32: Bits = Bits::new(s_4_30 as u128, 64u16);
        // C s_4_33: const #1u : u8
        let s_4_33: bool = true;
        // D s_4_34: call AddWithCarry(s_4_31, s_4_32, s_4_33)
        let s_4_34: ProductTyped54bc449dd09e5bd = AddWithCarry(
            state,
            tracer,
            s_4_31,
            s_4_32,
            s_4_33,
        );
        // D s_4_35: write-var gs#718634 <= s_4_34
        fn_state.gs_718634 = s_4_34;
        // D s_4_36: read-var gs#718634.0:struct
        let s_4_36: Bits = fn_state.gs_718634._0;
        // D s_4_37: cast reint s_4_36 -> u64
        let s_4_37: u64 = (s_4_36.value() as u64);
        // D s_4_38: read-var gs#718634.1:struct
        let s_4_38: u8 = fn_state.gs_718634._1;
        // D s_4_39: write-var result <= s_4_37
        fn_state.result = s_4_37;
        // D s_4_40: write-var nzcv <= s_4_38
        fn_state.nzcv = s_4_38;
        // D s_4_41: read-var setflags:u8
        let s_4_41: bool = fn_state.setflags;
        // N s_4_42: branch s_4_41 b7 b5
        if s_4_41 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #64s : i64
        let s_6_0: i64 = 64;
        // D s_6_1: read-var d:i64
        let s_6_1: i64 = fn_state.d;
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // D s_6_3: read-var result:u64
        let s_6_3: u64 = fn_state.result;
        // D s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 64u16);
        // D s_6_5: call X_set(s_6_2, s_6_0, s_6_4)
        let s_6_5: () = X_set(state, tracer, s_6_2, s_6_0, s_6_4);
        // N s_6_6: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #3s : i
        let s_7_0: i128 = 3;
        // D s_7_1: read-var nzcv:u8
        let s_7_1: u8 = fn_state.nzcv;
        // D s_7_2: cast zx s_7_1 -> bv
        let s_7_2: Bits = Bits::new(s_7_1 as u128, 4u16);
        // C s_7_3: const #1s : i64
        let s_7_3: i64 = 1;
        // C s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // C s_7_5: const #0s : i
        let s_7_5: i128 = 0;
        // C s_7_6: add s_7_5 s_7_4
        let s_7_6: i128 = (s_7_5 + s_7_4);
        // D s_7_7: bit-extract s_7_2 s_7_0 s_7_6
        let s_7_7: Bits = (Bits::new(
            ((s_7_2) >> (s_7_0)).value(),
            u16::try_from(s_7_6).unwrap(),
        ));
        // D s_7_8: cast reint s_7_7 -> u8
        let s_7_8: bool = ((s_7_7.value()) != 0);
        // C s_7_9: const #16984u : u32
        let s_7_9: u32 = 16984;
        // N s_7_10: write-reg s_7_9 <= s_7_8
        let s_7_10: () = {
            state.write_register::<bool>(s_7_9 as isize, s_7_8);
            tracer.write_register(s_7_9 as isize, s_7_8);
        };
        // C s_7_11: const #2s : i
        let s_7_11: i128 = 2;
        // D s_7_12: read-var nzcv:u8
        let s_7_12: u8 = fn_state.nzcv;
        // D s_7_13: cast zx s_7_12 -> bv
        let s_7_13: Bits = Bits::new(s_7_12 as u128, 4u16);
        // C s_7_14: const #1s : i64
        let s_7_14: i64 = 1;
        // C s_7_15: cast zx s_7_14 -> i
        let s_7_15: i128 = (i128::try_from(s_7_14).unwrap());
        // C s_7_16: const #0s : i
        let s_7_16: i128 = 0;
        // C s_7_17: add s_7_16 s_7_15
        let s_7_17: i128 = (s_7_16 + s_7_15);
        // D s_7_18: bit-extract s_7_13 s_7_11 s_7_17
        let s_7_18: Bits = (Bits::new(
            ((s_7_13) >> (s_7_11)).value(),
            u16::try_from(s_7_17).unwrap(),
        ));
        // D s_7_19: cast reint s_7_18 -> u8
        let s_7_19: bool = ((s_7_18.value()) != 0);
        // C s_7_20: const #16997u : u32
        let s_7_20: u32 = 16997;
        // N s_7_21: write-reg s_7_20 <= s_7_19
        let s_7_21: () = {
            state.write_register::<bool>(s_7_20 as isize, s_7_19);
            tracer.write_register(s_7_20 as isize, s_7_19);
        };
        // C s_7_22: const #1s : i
        let s_7_22: i128 = 1;
        // D s_7_23: read-var nzcv:u8
        let s_7_23: u8 = fn_state.nzcv;
        // D s_7_24: cast zx s_7_23 -> bv
        let s_7_24: Bits = Bits::new(s_7_23 as u128, 4u16);
        // C s_7_25: const #1s : i64
        let s_7_25: i64 = 1;
        // C s_7_26: cast zx s_7_25 -> i
        let s_7_26: i128 = (i128::try_from(s_7_25).unwrap());
        // C s_7_27: const #0s : i
        let s_7_27: i128 = 0;
        // C s_7_28: add s_7_27 s_7_26
        let s_7_28: i128 = (s_7_27 + s_7_26);
        // D s_7_29: bit-extract s_7_24 s_7_22 s_7_28
        let s_7_29: Bits = (Bits::new(
            ((s_7_24) >> (s_7_22)).value(),
            u16::try_from(s_7_28).unwrap(),
        ));
        // D s_7_30: cast reint s_7_29 -> u8
        let s_7_30: bool = ((s_7_29.value()) != 0);
        // C s_7_31: const #16971u : u32
        let s_7_31: u32 = 16971;
        // N s_7_32: write-reg s_7_31 <= s_7_30
        let s_7_32: () = {
            state.write_register::<bool>(s_7_31 as isize, s_7_30);
            tracer.write_register(s_7_31 as isize, s_7_30);
        };
        // C s_7_33: const #0s : i
        let s_7_33: i128 = 0;
        // D s_7_34: read-var nzcv:u8
        let s_7_34: u8 = fn_state.nzcv;
        // D s_7_35: cast zx s_7_34 -> bv
        let s_7_35: Bits = Bits::new(s_7_34 as u128, 4u16);
        // C s_7_36: const #1s : i64
        let s_7_36: i64 = 1;
        // C s_7_37: cast zx s_7_36 -> i
        let s_7_37: i128 = (i128::try_from(s_7_36).unwrap());
        // C s_7_38: const #0s : i
        let s_7_38: i128 = 0;
        // C s_7_39: add s_7_38 s_7_37
        let s_7_39: i128 = (s_7_38 + s_7_37);
        // D s_7_40: bit-extract s_7_35 s_7_33 s_7_39
        let s_7_40: Bits = (Bits::new(
            ((s_7_35) >> (s_7_33)).value(),
            u16::try_from(s_7_39).unwrap(),
        ));
        // D s_7_41: cast reint s_7_40 -> u8
        let s_7_41: bool = ((s_7_40.value()) != 0);
        // C s_7_42: const #16996u : u32
        let s_7_42: u32 = 16996;
        // N s_7_43: write-reg s_7_42 <= s_7_41
        let s_7_43: () = {
            state.write_register::<bool>(s_7_42 as isize, s_7_41);
            tracer.write_register(s_7_42 as isize, s_7_41);
        };
        // N s_7_44: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call SP_read(s_8_0)
        let s_8_1: u64 = SP_read(state, tracer, s_8_0);
        // D s_8_2: write-var operand2 <= s_8_1
        fn_state.operand2 = s_8_1;
        // N s_8_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call SP_read(s_9_0)
        let s_9_1: u64 = SP_read(state, tracer, s_9_0);
        // D s_9_2: write-var operand1 <= s_9_1
        fn_state.operand1 = s_9_1;
        // N s_9_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
