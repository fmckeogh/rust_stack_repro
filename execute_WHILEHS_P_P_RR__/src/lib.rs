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
use asl_Int::*;
use X_read::*;
use Elem_set::*;
use CheckSVEEnabled::*;
use Ones::*;
use P_set::*;
use common::*;
pub fn execute_WHILEHS_P_P_RR__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    m: i64,
    n: i64,
    op: u32,
    rsize: i64,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        last: bool,
        e: i64,
        pbit: bool,
        rsizeshadow_2604: i64,
        psize: i64,
        result: Bits,
        operand1: Bits,
        cond: bool,
        gs_187707: bool,
        PL: i64,
        mask: Bits,
        operand2: Bits,
        VL: i64,
        d: i64,
        esize: i64,
        m: i64,
        n: i64,
        op: u32,
        rsize: i64,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        m,
        n,
        op,
        rsize,
        is_unsigned,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var rsize:i64
        let s_0_0: i64 = fn_state.rsize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var rsizeshadow#2604 <= s_0_2
        fn_state.rsizeshadow_2604 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckSVEEnabled(s_0_4)
        let s_0_5: () = CheckSVEEnabled(state, tracer, s_0_4);
        // N s_0_6: jump b1
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
        // D s_1_6: cast zx s_1_0 -> i
        let s_1_6: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_7: read-var esize:i64
        let s_1_7: i64 = fn_state.esize;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: div s_1_6 s_1_8
        let s_1_9: i128 = ((s_1_6) / (s_1_8));
        // D s_1_10: cast reint s_1_9 -> i64
        let s_1_10: i64 = (s_1_9 as i64);
        // D s_1_11: read-var PL:i64
        let s_1_11: i64 = fn_state.PL;
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_13: call Ones(s_1_12)
        let s_1_13: Bits = Ones(state, tracer, s_1_12);
        // D s_1_14: write-var mask <= s_1_13
        fn_state.mask = s_1_13;
        // D s_1_15: read-var rsizeshadow#2604:i64
        let s_1_15: i64 = fn_state.rsizeshadow_2604;
        // D s_1_16: cast zx s_1_15 -> i
        let s_1_16: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_17: cast reint s_1_16 -> i64
        let s_1_17: i64 = (s_1_16 as i64);
        // D s_1_18: read-var n:i64
        let s_1_18: i64 = fn_state.n;
        // D s_1_19: cast zx s_1_18 -> i
        let s_1_19: i128 = (i128::try_from(s_1_18).unwrap());
        // D s_1_20: call X_read(s_1_19, s_1_17)
        let s_1_20: Bits = X_read(state, tracer, s_1_19, s_1_17);
        // D s_1_21: write-var operand1 <= s_1_20
        fn_state.operand1 = s_1_20;
        // D s_1_22: read-var rsizeshadow#2604:i64
        let s_1_22: i64 = fn_state.rsizeshadow_2604;
        // D s_1_23: cast zx s_1_22 -> i
        let s_1_23: i128 = (i128::try_from(s_1_22).unwrap());
        // D s_1_24: cast reint s_1_23 -> i64
        let s_1_24: i64 = (s_1_23 as i64);
        // D s_1_25: read-var m:i64
        let s_1_25: i64 = fn_state.m;
        // D s_1_26: cast zx s_1_25 -> i
        let s_1_26: i128 = (i128::try_from(s_1_25).unwrap());
        // D s_1_27: call X_read(s_1_26, s_1_24)
        let s_1_27: Bits = X_read(state, tracer, s_1_26, s_1_24);
        // D s_1_28: write-var operand2 <= s_1_27
        fn_state.operand2 = s_1_27;
        // C s_1_29: const #1u : u8
        let s_1_29: bool = true;
        // D s_1_30: write-var last <= s_1_29
        fn_state.last = s_1_29;
        // C s_1_31: const #8s : i
        let s_1_31: i128 = 8;
        // D s_1_32: read-var esize:i64
        let s_1_32: i64 = fn_state.esize;
        // D s_1_33: cast zx s_1_32 -> i
        let s_1_33: i128 = (i128::try_from(s_1_32).unwrap());
        // D s_1_34: div s_1_33 s_1_31
        let s_1_34: i128 = ((s_1_33) / (s_1_31));
        // D s_1_35: cast reint s_1_34 -> i64
        let s_1_35: i64 = (s_1_34 as i64);
        // D s_1_36: write-var psize <= s_1_35
        fn_state.psize = s_1_35;
        // C s_1_37: const #1s : i
        let s_1_37: i128 = 1;
        // D s_1_38: cast zx s_1_10 -> i
        let s_1_38: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_39: sub s_1_38 s_1_37
        let s_1_39: i128 = ((s_1_38) - (s_1_37));
        // D s_1_40: cast reint s_1_39 -> i64
        let s_1_40: i64 = (s_1_39 as i64);
        // D s_1_41: write-var e <= s_1_40
        fn_state.e = s_1_40;
        // N s_1_42: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // C s_2_1: const #0s : i64
        let s_2_1: i64 = 0;
        // D s_2_2: cmp-lt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) < (s_2_1));
        // N s_2_3: branch s_2_2 b15 b3
        if s_2_2 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #3u : u32
        let s_3_0: u32 = 3;
        // D s_3_1: read-var op:u32
        let s_3_1: u32 = fn_state.op;
        // D s_3_2: cmp-eq s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) == (s_3_1));
        // D s_3_3: not s_3_2
        let s_3_3: bool = !s_3_2;
        // N s_3_4: branch s_3_3 b12 b4
        if s_3_3 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var operand1:bv
        let s_4_0: Bits = fn_state.operand1;
        // D s_4_1: read-var is_unsigned:u8
        let s_4_1: bool = fn_state.is_unsigned;
        // D s_4_2: call asl_Int(s_4_0, s_4_1)
        let s_4_2: i128 = asl_Int(state, tracer, s_4_0, s_4_1);
        // D s_4_3: read-var operand2:bv
        let s_4_3: Bits = fn_state.operand2;
        // D s_4_4: read-var is_unsigned:u8
        let s_4_4: bool = fn_state.is_unsigned;
        // D s_4_5: call asl_Int(s_4_3, s_4_4)
        let s_4_5: i128 = asl_Int(state, tracer, s_4_3, s_4_4);
        // D s_4_6: cmp-gt s_4_2 s_4_5
        let s_4_6: bool = ((s_4_2) > (s_4_5));
        // D s_4_7: write-var cond <= s_4_6
        fn_state.cond = s_4_6;
        // N s_4_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var last:u8
        let s_5_0: bool = fn_state.last;
        // N s_5_1: branch s_5_0 b11 b6
        if s_5_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#187707 <= s_6_0
        fn_state.gs_187707 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#187707:u8
        let s_7_0: bool = fn_state.gs_187707;
        // D s_7_1: write-var last <= s_7_0
        fn_state.last = s_7_0;
        // D s_7_2: read-var last:u8
        let s_7_2: bool = fn_state.last;
        // N s_7_3: branch s_7_2 b10 b8
        if s_7_2 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var pbit <= s_8_0
        fn_state.pbit = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var psize:i64
        let s_9_0: i64 = fn_state.psize;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: cast reint s_9_1 -> i64
        let s_9_2: i64 = (s_9_1 as i64);
        // D s_9_3: read-var pbit:u8
        let s_9_3: bool = fn_state.pbit;
        // D s_9_4: cast zx s_9_3 -> bv
        let s_9_4: Bits = Bits::new(s_9_3 as u128, 1u16);
        // D s_9_5: read-var psize:i64
        let s_9_5: i64 = fn_state.psize;
        // D s_9_6: cast zx s_9_5 -> i
        let s_9_6: i128 = (i128::try_from(s_9_5).unwrap());
        // D s_9_7: bits-cast zx s_9_4 -> bv length s_9_6
        let s_9_7: Bits = s_9_4.zero_extend(s_9_6);
        // D s_9_8: read-var e:i64
        let s_9_8: i64 = fn_state.e;
        // D s_9_9: cast zx s_9_8 -> i
        let s_9_9: i128 = (i128::try_from(s_9_8).unwrap());
        // D s_9_10: cast zx s_9_2 -> i
        let s_9_10: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_11: read-var result:bv
        let s_9_11: Bits = fn_state.result;
        // D s_9_12: call Elem_set(s_9_11, s_9_9, s_9_10, s_9_7)
        let s_9_12: Bits = Elem_set(state, tracer, s_9_11, s_9_9, s_9_10, s_9_7);
        // D s_9_13: write-var result <= s_9_12
        fn_state.result = s_9_12;
        // C s_9_14: const #1s : i
        let s_9_14: i128 = 1;
        // D s_9_15: read-var operand1:bv
        let s_9_15: Bits = fn_state.operand1;
        // C s_9_16: cast cvt s_9_14 -> bv
        let s_9_16: Bits = Bits::new(s_9_14 as u128, 128);
        // D s_9_17: sub s_9_15 s_9_16
        let s_9_17: Bits = ((s_9_15) - (s_9_16));
        // D s_9_18: write-var operand1 <= s_9_17
        fn_state.operand1 = s_9_17;
        // D s_9_19: read-var e:i64
        let s_9_19: i64 = fn_state.e;
        // C s_9_20: const #1s : i64
        let s_9_20: i64 = 1;
        // D s_9_21: sub s_9_19 s_9_20
        let s_9_21: i64 = ((s_9_19) - (s_9_20));
        // D s_9_22: write-var e <= s_9_21
        fn_state.e = s_9_21;
        // N s_9_23: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #1u : u8
        let s_10_0: bool = true;
        // D s_10_1: write-var pbit <= s_10_0
        fn_state.pbit = s_10_0;
        // N s_10_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var cond:u8
        let s_11_0: bool = fn_state.cond;
        // D s_11_1: write-var gs#187707 <= s_11_0
        fn_state.gs_187707 = s_11_0;
        // N s_11_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #2u : u32
        let s_12_0: u32 = 2;
        // D s_12_1: read-var op:u32
        let s_12_1: u32 = fn_state.op;
        // D s_12_2: cmp-eq s_12_0 s_12_1
        let s_12_2: bool = ((s_12_0) == (s_12_1));
        // D s_12_3: not s_12_2
        let s_12_3: bool = !s_12_2;
        // N s_12_4: branch s_12_3 b14 b13
        if s_12_3 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var operand1:bv
        let s_13_0: Bits = fn_state.operand1;
        // D s_13_1: read-var is_unsigned:u8
        let s_13_1: bool = fn_state.is_unsigned;
        // D s_13_2: call asl_Int(s_13_0, s_13_1)
        let s_13_2: i128 = asl_Int(state, tracer, s_13_0, s_13_1);
        // D s_13_3: read-var operand2:bv
        let s_13_3: Bits = fn_state.operand2;
        // D s_13_4: read-var is_unsigned:u8
        let s_13_4: bool = fn_state.is_unsigned;
        // D s_13_5: call asl_Int(s_13_3, s_13_4)
        let s_13_5: i128 = asl_Int(state, tracer, s_13_3, s_13_4);
        // D s_13_6: cmp-ge s_13_2 s_13_5
        let s_13_6: bool = ((s_13_2) >= (s_13_5));
        // D s_13_7: write-var cond <= s_13_6
        fn_state.cond = s_13_6;
        // N s_13_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_14_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var esize:i64
        let s_15_0: i64 = fn_state.esize;
        // D s_15_1: cast zx s_15_0 -> i
        let s_15_1: i128 = (i128::try_from(s_15_0).unwrap());
        // D s_15_2: read-var mask:bv
        let s_15_2: Bits = fn_state.mask;
        // D s_15_3: read-var result:bv
        let s_15_3: Bits = fn_state.result;
        // D s_15_4: call PredTest(s_15_2, s_15_3, s_15_1)
        let s_15_4: u8 = PredTest(state, tracer, s_15_2, s_15_3, s_15_1);
        // C s_15_5: const #3s : i
        let s_15_5: i128 = 3;
        // D s_15_6: cast zx s_15_4 -> bv
        let s_15_6: Bits = Bits::new(s_15_4 as u128, 4u16);
        // C s_15_7: const #1s : i64
        let s_15_7: i64 = 1;
        // C s_15_8: cast zx s_15_7 -> i
        let s_15_8: i128 = (i128::try_from(s_15_7).unwrap());
        // C s_15_9: const #0s : i
        let s_15_9: i128 = 0;
        // C s_15_10: add s_15_9 s_15_8
        let s_15_10: i128 = (s_15_9 + s_15_8);
        // D s_15_11: bit-extract s_15_6 s_15_5 s_15_10
        let s_15_11: Bits = (Bits::new(
            ((s_15_6) >> (s_15_5)).value(),
            u16::try_from(s_15_10).unwrap(),
        ));
        // D s_15_12: cast reint s_15_11 -> u8
        let s_15_12: bool = ((s_15_11.value()) != 0);
        // C s_15_13: const #16984u : u32
        let s_15_13: u32 = 16984;
        // N s_15_14: write-reg s_15_13 <= s_15_12
        let s_15_14: () = {
            state.write_register::<bool>(s_15_13 as isize, s_15_12);
            tracer.write_register(s_15_13 as isize, s_15_12);
        };
        // C s_15_15: const #2s : i
        let s_15_15: i128 = 2;
        // D s_15_16: cast zx s_15_4 -> bv
        let s_15_16: Bits = Bits::new(s_15_4 as u128, 4u16);
        // C s_15_17: const #1s : i64
        let s_15_17: i64 = 1;
        // C s_15_18: cast zx s_15_17 -> i
        let s_15_18: i128 = (i128::try_from(s_15_17).unwrap());
        // C s_15_19: const #0s : i
        let s_15_19: i128 = 0;
        // C s_15_20: add s_15_19 s_15_18
        let s_15_20: i128 = (s_15_19 + s_15_18);
        // D s_15_21: bit-extract s_15_16 s_15_15 s_15_20
        let s_15_21: Bits = (Bits::new(
            ((s_15_16) >> (s_15_15)).value(),
            u16::try_from(s_15_20).unwrap(),
        ));
        // D s_15_22: cast reint s_15_21 -> u8
        let s_15_22: bool = ((s_15_21.value()) != 0);
        // C s_15_23: const #16997u : u32
        let s_15_23: u32 = 16997;
        // N s_15_24: write-reg s_15_23 <= s_15_22
        let s_15_24: () = {
            state.write_register::<bool>(s_15_23 as isize, s_15_22);
            tracer.write_register(s_15_23 as isize, s_15_22);
        };
        // C s_15_25: const #1s : i
        let s_15_25: i128 = 1;
        // D s_15_26: cast zx s_15_4 -> bv
        let s_15_26: Bits = Bits::new(s_15_4 as u128, 4u16);
        // C s_15_27: const #1s : i64
        let s_15_27: i64 = 1;
        // C s_15_28: cast zx s_15_27 -> i
        let s_15_28: i128 = (i128::try_from(s_15_27).unwrap());
        // C s_15_29: const #0s : i
        let s_15_29: i128 = 0;
        // C s_15_30: add s_15_29 s_15_28
        let s_15_30: i128 = (s_15_29 + s_15_28);
        // D s_15_31: bit-extract s_15_26 s_15_25 s_15_30
        let s_15_31: Bits = (Bits::new(
            ((s_15_26) >> (s_15_25)).value(),
            u16::try_from(s_15_30).unwrap(),
        ));
        // D s_15_32: cast reint s_15_31 -> u8
        let s_15_32: bool = ((s_15_31.value()) != 0);
        // C s_15_33: const #16971u : u32
        let s_15_33: u32 = 16971;
        // N s_15_34: write-reg s_15_33 <= s_15_32
        let s_15_34: () = {
            state.write_register::<bool>(s_15_33 as isize, s_15_32);
            tracer.write_register(s_15_33 as isize, s_15_32);
        };
        // C s_15_35: const #0s : i
        let s_15_35: i128 = 0;
        // D s_15_36: cast zx s_15_4 -> bv
        let s_15_36: Bits = Bits::new(s_15_4 as u128, 4u16);
        // C s_15_37: const #1s : i64
        let s_15_37: i64 = 1;
        // C s_15_38: cast zx s_15_37 -> i
        let s_15_38: i128 = (i128::try_from(s_15_37).unwrap());
        // C s_15_39: const #0s : i
        let s_15_39: i128 = 0;
        // C s_15_40: add s_15_39 s_15_38
        let s_15_40: i128 = (s_15_39 + s_15_38);
        // D s_15_41: bit-extract s_15_36 s_15_35 s_15_40
        let s_15_41: Bits = (Bits::new(
            ((s_15_36) >> (s_15_35)).value(),
            u16::try_from(s_15_40).unwrap(),
        ));
        // D s_15_42: cast reint s_15_41 -> u8
        let s_15_42: bool = ((s_15_41.value()) != 0);
        // C s_15_43: const #16996u : u32
        let s_15_43: u32 = 16996;
        // N s_15_44: write-reg s_15_43 <= s_15_42
        let s_15_44: () = {
            state.write_register::<bool>(s_15_43 as isize, s_15_42);
            tracer.write_register(s_15_43 as isize, s_15_42);
        };
        // D s_15_45: read-var PL:i64
        let s_15_45: i64 = fn_state.PL;
        // D s_15_46: cast zx s_15_45 -> i
        let s_15_46: i128 = (i128::try_from(s_15_45).unwrap());
        // D s_15_47: cast reint s_15_46 -> i64
        let s_15_47: i64 = (s_15_46 as i64);
        // D s_15_48: read-var d:i64
        let s_15_48: i64 = fn_state.d;
        // D s_15_49: cast zx s_15_48 -> i
        let s_15_49: i128 = (i128::try_from(s_15_48).unwrap());
        // D s_15_50: cast zx s_15_47 -> i
        let s_15_50: i128 = (i128::try_from(s_15_47).unwrap());
        // D s_15_51: read-var result:bv
        let s_15_51: Bits = fn_state.result;
        // D s_15_52: call P_set(s_15_49, s_15_50, s_15_51)
        let s_15_52: () = P_set(state, tracer, s_15_49, s_15_50, s_15_51);
        // N s_15_53: return
        return;
    }
}
