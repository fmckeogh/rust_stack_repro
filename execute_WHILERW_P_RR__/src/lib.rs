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
use X_read::*;
use Elem_set::*;
use CheckSVEEnabled::*;
use Ones::*;
use P_set::*;
use common::*;
pub fn execute_WHILERW_P_RR__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        PL: i64,
        mask: Bits,
        diff: i128,
        psize: i64,
        result: Bits,
        gs_216306: i64,
        gs_216310: bool,
        VL: i64,
        d: i64,
        esize: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        m,
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
        // C s_1_15: const #64s : i64
        let s_1_15: i64 = 64;
        // D s_1_16: read-var n:i64
        let s_1_16: i64 = fn_state.n;
        // D s_1_17: cast zx s_1_16 -> i
        let s_1_17: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_18: call X_read(s_1_17, s_1_15)
        let s_1_18: Bits = X_read(state, tracer, s_1_17, s_1_15);
        // D s_1_19: cast reint s_1_18 -> u64
        let s_1_19: u64 = (s_1_18.value() as u64);
        // C s_1_20: const #64s : i64
        let s_1_20: i64 = 64;
        // D s_1_21: read-var m:i64
        let s_1_21: i64 = fn_state.m;
        // D s_1_22: cast zx s_1_21 -> i
        let s_1_22: i128 = (i128::try_from(s_1_21).unwrap());
        // D s_1_23: call X_read(s_1_22, s_1_20)
        let s_1_23: Bits = X_read(state, tracer, s_1_22, s_1_20);
        // D s_1_24: cast reint s_1_23 -> u64
        let s_1_24: u64 = (s_1_23.value() as u64);
        // D s_1_25: cast zx s_1_19 -> bv
        let s_1_25: Bits = Bits::new(s_1_19 as u128, 64u16);
        // D s_1_26: cast zx s_1_25 -> i
        let s_1_26: i128 = (s_1_25.value() as i128);
        // D s_1_27: cast zx s_1_24 -> bv
        let s_1_27: Bits = Bits::new(s_1_24 as u128, 64u16);
        // D s_1_28: cast zx s_1_27 -> i
        let s_1_28: i128 = (s_1_27.value() as i128);
        // C s_1_29: const #8s : i
        let s_1_29: i128 = 8;
        // D s_1_30: read-var esize:i64
        let s_1_30: i64 = fn_state.esize;
        // D s_1_31: cast zx s_1_30 -> i
        let s_1_31: i128 = (i128::try_from(s_1_30).unwrap());
        // D s_1_32: div s_1_31 s_1_29
        let s_1_32: i128 = ((s_1_31) / (s_1_29));
        // D s_1_33: cast reint s_1_32 -> i64
        let s_1_33: i64 = (s_1_32 as i64);
        // D s_1_34: write-var psize <= s_1_33
        fn_state.psize = s_1_33;
        // D s_1_35: sub s_1_28 s_1_26
        let s_1_35: i128 = ((s_1_28) - (s_1_26));
        // D s_1_36: abs s_1_35
        let s_1_36: i128 = (s_1_35).abs();
        // C s_1_37: const #8s : i
        let s_1_37: i128 = 8;
        // D s_1_38: read-var esize:i64
        let s_1_38: i64 = fn_state.esize;
        // D s_1_39: cast zx s_1_38 -> i
        let s_1_39: i128 = (i128::try_from(s_1_38).unwrap());
        // D s_1_40: div s_1_39 s_1_37
        let s_1_40: i128 = ((s_1_39) / (s_1_37));
        // D s_1_41: cast reint s_1_40 -> i64
        let s_1_41: i64 = (s_1_40 as i64);
        // D s_1_42: cast zx s_1_41 -> i
        let s_1_42: i128 = (i128::try_from(s_1_41).unwrap());
        // D s_1_43: div s_1_36 s_1_42
        let s_1_43: i128 = ((s_1_36) / (s_1_42));
        // D s_1_44: write-var diff <= s_1_43
        fn_state.diff = s_1_43;
        // C s_1_45: const #0s : i64
        let s_1_45: i64 = 0;
        // C s_1_46: const #1s : i
        let s_1_46: i128 = 1;
        // D s_1_47: cast zx s_1_10 -> i
        let s_1_47: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_48: sub s_1_47 s_1_46
        let s_1_48: i128 = ((s_1_47) - (s_1_46));
        // D s_1_49: cast reint s_1_48 -> i64
        let s_1_49: i64 = (s_1_48 as i64);
        // D s_1_50: write-var gs#216306 <= s_1_49
        fn_state.gs_216306 = s_1_49;
        // D s_1_51: write-var e <= s_1_45
        fn_state.e = s_1_45;
        // N s_1_52: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#216306:i64
        let s_2_1: i64 = fn_state.gs_216306;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b10 b3
        if s_2_2 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0s : i
        let s_3_0: i128 = 0;
        // D s_3_1: read-var diff:i
        let s_3_1: i128 = fn_state.diff;
        // D s_3_2: cmp-eq s_3_1 s_3_0
        let s_3_2: bool = ((s_3_1) == (s_3_0));
        // N s_3_3: branch s_3_2 b9 b4
        if s_3_2 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: read-var diff:i
        let s_4_2: i128 = fn_state.diff;
        // D s_4_3: cmp-lt s_4_1 s_4_2
        let s_4_3: bool = ((s_4_1) < (s_4_2));
        // D s_4_4: write-var gs#216310 <= s_4_3
        fn_state.gs_216310 = s_4_3;
        // N s_4_5: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#216310:u8
        let s_5_0: bool = fn_state.gs_216310;
        // N s_5_1: branch s_5_0 b8 b6
        if s_5_0 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var psize:i64
        let s_6_0: i64 = fn_state.psize;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: cast reint s_6_1 -> i64
        let s_6_2: i64 = (s_6_1 as i64);
        // C s_6_3: const #0u : u8
        let s_6_3: bool = false;
        // C s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 1u16);
        // D s_6_5: read-var psize:i64
        let s_6_5: i64 = fn_state.psize;
        // D s_6_6: cast zx s_6_5 -> i
        let s_6_6: i128 = (i128::try_from(s_6_5).unwrap());
        // D s_6_7: bits-cast zx s_6_4 -> bv length s_6_6
        let s_6_7: Bits = s_6_4.zero_extend(s_6_6);
        // D s_6_8: read-var e:i64
        let s_6_8: i64 = fn_state.e;
        // D s_6_9: cast zx s_6_8 -> i
        let s_6_9: i128 = (i128::try_from(s_6_8).unwrap());
        // D s_6_10: cast zx s_6_2 -> i
        let s_6_10: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_11: read-var result:bv
        let s_6_11: Bits = fn_state.result;
        // D s_6_12: call Elem_set(s_6_11, s_6_9, s_6_10, s_6_7)
        let s_6_12: Bits = Elem_set(state, tracer, s_6_11, s_6_9, s_6_10, s_6_7);
        // D s_6_13: write-var result <= s_6_12
        fn_state.result = s_6_12;
        // N s_6_14: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var e:i64
        let s_7_0: i64 = fn_state.e;
        // C s_7_1: const #1s : i64
        let s_7_1: i64 = 1;
        // D s_7_2: add s_7_0 s_7_1
        let s_7_2: i64 = (s_7_0 + s_7_1);
        // D s_7_3: write-var e <= s_7_2
        fn_state.e = s_7_2;
        // N s_7_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var psize:i64
        let s_8_0: i64 = fn_state.psize;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: cast reint s_8_1 -> i64
        let s_8_2: i64 = (s_8_1 as i64);
        // C s_8_3: const #1u : u8
        let s_8_3: bool = true;
        // C s_8_4: cast zx s_8_3 -> bv
        let s_8_4: Bits = Bits::new(s_8_3 as u128, 1u16);
        // D s_8_5: read-var psize:i64
        let s_8_5: i64 = fn_state.psize;
        // D s_8_6: cast zx s_8_5 -> i
        let s_8_6: i128 = (i128::try_from(s_8_5).unwrap());
        // D s_8_7: bits-cast zx s_8_4 -> bv length s_8_6
        let s_8_7: Bits = s_8_4.zero_extend(s_8_6);
        // D s_8_8: read-var e:i64
        let s_8_8: i64 = fn_state.e;
        // D s_8_9: cast zx s_8_8 -> i
        let s_8_9: i128 = (i128::try_from(s_8_8).unwrap());
        // D s_8_10: cast zx s_8_2 -> i
        let s_8_10: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_11: read-var result:bv
        let s_8_11: Bits = fn_state.result;
        // D s_8_12: call Elem_set(s_8_11, s_8_9, s_8_10, s_8_7)
        let s_8_12: Bits = Elem_set(state, tracer, s_8_11, s_8_9, s_8_10, s_8_7);
        // D s_8_13: write-var result <= s_8_12
        fn_state.result = s_8_12;
        // N s_8_14: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #1u : u8
        let s_9_0: bool = true;
        // D s_9_1: write-var gs#216310 <= s_9_0
        fn_state.gs_216310 = s_9_0;
        // N s_9_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var esize:i64
        let s_10_0: i64 = fn_state.esize;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: read-var mask:bv
        let s_10_2: Bits = fn_state.mask;
        // D s_10_3: read-var result:bv
        let s_10_3: Bits = fn_state.result;
        // D s_10_4: call PredTest(s_10_2, s_10_3, s_10_1)
        let s_10_4: u8 = PredTest(state, tracer, s_10_2, s_10_3, s_10_1);
        // C s_10_5: const #3s : i
        let s_10_5: i128 = 3;
        // D s_10_6: cast zx s_10_4 -> bv
        let s_10_6: Bits = Bits::new(s_10_4 as u128, 4u16);
        // C s_10_7: const #1s : i64
        let s_10_7: i64 = 1;
        // C s_10_8: cast zx s_10_7 -> i
        let s_10_8: i128 = (i128::try_from(s_10_7).unwrap());
        // C s_10_9: const #0s : i
        let s_10_9: i128 = 0;
        // C s_10_10: add s_10_9 s_10_8
        let s_10_10: i128 = (s_10_9 + s_10_8);
        // D s_10_11: bit-extract s_10_6 s_10_5 s_10_10
        let s_10_11: Bits = (Bits::new(
            ((s_10_6) >> (s_10_5)).value(),
            u16::try_from(s_10_10).unwrap(),
        ));
        // D s_10_12: cast reint s_10_11 -> u8
        let s_10_12: bool = ((s_10_11.value()) != 0);
        // C s_10_13: const #16984u : u32
        let s_10_13: u32 = 16984;
        // N s_10_14: write-reg s_10_13 <= s_10_12
        let s_10_14: () = {
            state.write_register::<bool>(s_10_13 as isize, s_10_12);
            tracer.write_register(s_10_13 as isize, s_10_12);
        };
        // C s_10_15: const #2s : i
        let s_10_15: i128 = 2;
        // D s_10_16: cast zx s_10_4 -> bv
        let s_10_16: Bits = Bits::new(s_10_4 as u128, 4u16);
        // C s_10_17: const #1s : i64
        let s_10_17: i64 = 1;
        // C s_10_18: cast zx s_10_17 -> i
        let s_10_18: i128 = (i128::try_from(s_10_17).unwrap());
        // C s_10_19: const #0s : i
        let s_10_19: i128 = 0;
        // C s_10_20: add s_10_19 s_10_18
        let s_10_20: i128 = (s_10_19 + s_10_18);
        // D s_10_21: bit-extract s_10_16 s_10_15 s_10_20
        let s_10_21: Bits = (Bits::new(
            ((s_10_16) >> (s_10_15)).value(),
            u16::try_from(s_10_20).unwrap(),
        ));
        // D s_10_22: cast reint s_10_21 -> u8
        let s_10_22: bool = ((s_10_21.value()) != 0);
        // C s_10_23: const #16997u : u32
        let s_10_23: u32 = 16997;
        // N s_10_24: write-reg s_10_23 <= s_10_22
        let s_10_24: () = {
            state.write_register::<bool>(s_10_23 as isize, s_10_22);
            tracer.write_register(s_10_23 as isize, s_10_22);
        };
        // C s_10_25: const #1s : i
        let s_10_25: i128 = 1;
        // D s_10_26: cast zx s_10_4 -> bv
        let s_10_26: Bits = Bits::new(s_10_4 as u128, 4u16);
        // C s_10_27: const #1s : i64
        let s_10_27: i64 = 1;
        // C s_10_28: cast zx s_10_27 -> i
        let s_10_28: i128 = (i128::try_from(s_10_27).unwrap());
        // C s_10_29: const #0s : i
        let s_10_29: i128 = 0;
        // C s_10_30: add s_10_29 s_10_28
        let s_10_30: i128 = (s_10_29 + s_10_28);
        // D s_10_31: bit-extract s_10_26 s_10_25 s_10_30
        let s_10_31: Bits = (Bits::new(
            ((s_10_26) >> (s_10_25)).value(),
            u16::try_from(s_10_30).unwrap(),
        ));
        // D s_10_32: cast reint s_10_31 -> u8
        let s_10_32: bool = ((s_10_31.value()) != 0);
        // C s_10_33: const #16971u : u32
        let s_10_33: u32 = 16971;
        // N s_10_34: write-reg s_10_33 <= s_10_32
        let s_10_34: () = {
            state.write_register::<bool>(s_10_33 as isize, s_10_32);
            tracer.write_register(s_10_33 as isize, s_10_32);
        };
        // C s_10_35: const #0s : i
        let s_10_35: i128 = 0;
        // D s_10_36: cast zx s_10_4 -> bv
        let s_10_36: Bits = Bits::new(s_10_4 as u128, 4u16);
        // C s_10_37: const #1s : i64
        let s_10_37: i64 = 1;
        // C s_10_38: cast zx s_10_37 -> i
        let s_10_38: i128 = (i128::try_from(s_10_37).unwrap());
        // C s_10_39: const #0s : i
        let s_10_39: i128 = 0;
        // C s_10_40: add s_10_39 s_10_38
        let s_10_40: i128 = (s_10_39 + s_10_38);
        // D s_10_41: bit-extract s_10_36 s_10_35 s_10_40
        let s_10_41: Bits = (Bits::new(
            ((s_10_36) >> (s_10_35)).value(),
            u16::try_from(s_10_40).unwrap(),
        ));
        // D s_10_42: cast reint s_10_41 -> u8
        let s_10_42: bool = ((s_10_41.value()) != 0);
        // C s_10_43: const #16996u : u32
        let s_10_43: u32 = 16996;
        // N s_10_44: write-reg s_10_43 <= s_10_42
        let s_10_44: () = {
            state.write_register::<bool>(s_10_43 as isize, s_10_42);
            tracer.write_register(s_10_43 as isize, s_10_42);
        };
        // D s_10_45: read-var PL:i64
        let s_10_45: i64 = fn_state.PL;
        // D s_10_46: cast zx s_10_45 -> i
        let s_10_46: i128 = (i128::try_from(s_10_45).unwrap());
        // D s_10_47: cast reint s_10_46 -> i64
        let s_10_47: i64 = (s_10_46 as i64);
        // D s_10_48: read-var d:i64
        let s_10_48: i64 = fn_state.d;
        // D s_10_49: cast zx s_10_48 -> i
        let s_10_49: i128 = (i128::try_from(s_10_48).unwrap());
        // D s_10_50: cast zx s_10_47 -> i
        let s_10_50: i128 = (i128::try_from(s_10_47).unwrap());
        // D s_10_51: read-var result:bv
        let s_10_51: Bits = fn_state.result;
        // D s_10_52: call P_set(s_10_49, s_10_50, s_10_51)
        let s_10_52: () = P_set(state, tracer, s_10_49, s_10_50, s_10_51);
        // N s_10_53: return
        return;
    }
}
