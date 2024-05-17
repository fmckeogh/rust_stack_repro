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
use StandardFPSCRValue::*;
use Elem_set::*;
use u__id::*;
use CheckAdvSIMDEnabled::*;
use FPMulAddH::*;
use FPNeg::*;
use D_set::*;
use Elem_read::*;
use S_read::*;
use D_read::*;
use common::*;
pub fn execute_aarch32_instrs_VFMAL_i_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Q: bool,
    d: i64,
    datasize: i64,
    esize: i64,
    index: i64,
    m: i64,
    n: i64,
    regs: i64,
    sub_op: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand2: Bits,
        r: i64,
        gs_326705: i64,
        e: i64,
        operand3: u64,
        element2: Bits,
        gs_916951: Bits,
        element1: Bits,
        gs_326694: bool,
        ga_366895: i64,
        result: u64,
        operand1: Bits,
        datasizeshadow_7992: i64,
        Q: bool,
        d: i64,
        datasize: i64,
        esize: i64,
        index: i64,
        m: i64,
        n: i64,
        regs: i64,
        sub_op: bool,
    }
    let fn_state = FunctionState {
        Q,
        d,
        datasize,
        esize,
        index,
        m,
        n,
        regs,
        sub_op,
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
        // D s_0_3: write-var datasizeshadow#7992 <= s_0_2
        fn_state.datasizeshadow_7992 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckAdvSIMDEnabled(s_0_4)
        let s_0_5: () = CheckAdvSIMDEnabled(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var Q:u8
        let s_1_0: bool = fn_state.Q;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 1u16);
        // C s_1_2: const #0u : u8
        let s_1_2: bool = false;
        // C s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 1u16);
        // D s_1_4: cmp-eq s_1_1 s_1_3
        let s_1_4: bool = ((s_1_1) == (s_1_3));
        // N s_1_5: branch s_1_4 b15 b2
        if s_1_4 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var n:i64
        let s_2_0: i64 = fn_state.n;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: call D_read(s_2_1)
        let s_2_2: u64 = D_read(state, tracer, s_2_1);
        // C s_2_3: const #1s : i
        let s_2_3: i128 = 1;
        // D s_2_4: read-var datasizeshadow#7992:i64
        let s_2_4: i64 = fn_state.datasizeshadow_7992;
        // D s_2_5: cast zx s_2_4 -> i
        let s_2_5: i128 = (i128::try_from(s_2_4).unwrap());
        // D s_2_6: sub s_2_5 s_2_3
        let s_2_6: i128 = ((s_2_5) - (s_2_3));
        // D s_2_7: cast reint s_2_6 -> i64
        let s_2_7: i64 = (s_2_6 as i64);
        // C s_2_8: const #0s : i
        let s_2_8: i128 = 0;
        // D s_2_9: cast zx s_2_2 -> bv
        let s_2_9: Bits = Bits::new(s_2_2 as u128, 64u16);
        // D s_2_10: cast zx s_2_7 -> i
        let s_2_10: i128 = (i128::try_from(s_2_7).unwrap());
        // C s_2_11: const #1s : i64
        let s_2_11: i64 = 1;
        // C s_2_12: cast zx s_2_11 -> i
        let s_2_12: i128 = (i128::try_from(s_2_11).unwrap());
        // D s_2_13: sub s_2_10 s_2_8
        let s_2_13: i128 = ((s_2_10) - (s_2_8));
        // D s_2_14: add s_2_13 s_2_12
        let s_2_14: i128 = (s_2_13 + s_2_12);
        // D s_2_15: bit-extract s_2_9 s_2_8 s_2_14
        let s_2_15: Bits = (Bits::new(
            ((s_2_9) >> (s_2_8)).value(),
            u16::try_from(s_2_14).unwrap(),
        ));
        // D s_2_16: write-var operand1 <= s_2_15
        fn_state.operand1 = s_2_15;
        // D s_2_17: read-var m:i64
        let s_2_17: i64 = fn_state.m;
        // D s_2_18: cast zx s_2_17 -> i
        let s_2_18: i128 = (i128::try_from(s_2_17).unwrap());
        // D s_2_19: call D_read(s_2_18)
        let s_2_19: u64 = D_read(state, tracer, s_2_18);
        // C s_2_20: const #1s : i
        let s_2_20: i128 = 1;
        // D s_2_21: read-var datasizeshadow#7992:i64
        let s_2_21: i64 = fn_state.datasizeshadow_7992;
        // D s_2_22: cast zx s_2_21 -> i
        let s_2_22: i128 = (i128::try_from(s_2_21).unwrap());
        // D s_2_23: sub s_2_22 s_2_20
        let s_2_23: i128 = ((s_2_22) - (s_2_20));
        // D s_2_24: cast reint s_2_23 -> i64
        let s_2_24: i64 = (s_2_23 as i64);
        // C s_2_25: const #0s : i
        let s_2_25: i128 = 0;
        // D s_2_26: cast zx s_2_19 -> bv
        let s_2_26: Bits = Bits::new(s_2_19 as u128, 64u16);
        // D s_2_27: cast zx s_2_24 -> i
        let s_2_27: i128 = (i128::try_from(s_2_24).unwrap());
        // C s_2_28: const #1s : i64
        let s_2_28: i64 = 1;
        // C s_2_29: cast zx s_2_28 -> i
        let s_2_29: i128 = (i128::try_from(s_2_28).unwrap());
        // D s_2_30: sub s_2_27 s_2_25
        let s_2_30: i128 = ((s_2_27) - (s_2_25));
        // D s_2_31: add s_2_30 s_2_29
        let s_2_31: i128 = (s_2_30 + s_2_29);
        // D s_2_32: bit-extract s_2_26 s_2_25 s_2_31
        let s_2_32: Bits = (Bits::new(
            ((s_2_26) >> (s_2_25)).value(),
            u16::try_from(s_2_31).unwrap(),
        ));
        // D s_2_33: write-var operand2 <= s_2_32
        fn_state.operand2 = s_2_32;
        // N s_2_34: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #2s : i
        let s_3_0: i128 = 2;
        // D s_3_1: read-var esize:i64
        let s_3_1: i64 = fn_state.esize;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: div s_3_2 s_3_0
        let s_3_3: i128 = ((s_3_2) / (s_3_0));
        // D s_3_4: cast reint s_3_3 -> i64
        let s_3_4: i64 = (s_3_3 as i64);
        // D s_3_5: cast zx s_3_4 -> i
        let s_3_5: i128 = (i128::try_from(s_3_4).unwrap());
        // D s_3_6: cast reint s_3_5 -> i64
        let s_3_6: i64 = (s_3_5 as i64);
        // D s_3_7: read-var index:i64
        let s_3_7: i64 = fn_state.index;
        // D s_3_8: cast zx s_3_7 -> i
        let s_3_8: i128 = (i128::try_from(s_3_7).unwrap());
        // D s_3_9: cast zx s_3_6 -> i
        let s_3_9: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_10: read-var operand2:bv
        let s_3_10: Bits = fn_state.operand2;
        // D s_3_11: call Elem_read(s_3_10, s_3_8, s_3_9)
        let s_3_11: Bits = Elem_read(state, tracer, s_3_10, s_3_8, s_3_9);
        // D s_3_12: write-var element2 <= s_3_11
        fn_state.element2 = s_3_11;
        // C s_3_13: const #0s : i64
        let s_3_13: i64 = 0;
        // C s_3_14: const #1s : i
        let s_3_14: i128 = 1;
        // D s_3_15: read-var regs:i64
        let s_3_15: i64 = fn_state.regs;
        // D s_3_16: cast zx s_3_15 -> i
        let s_3_16: i128 = (i128::try_from(s_3_15).unwrap());
        // D s_3_17: sub s_3_16 s_3_14
        let s_3_17: i128 = ((s_3_16) - (s_3_14));
        // D s_3_18: cast reint s_3_17 -> i64
        let s_3_18: i64 = (s_3_17 as i64);
        // D s_3_19: write-var gs#326705 <= s_3_18
        fn_state.gs_326705 = s_3_18;
        // D s_3_20: write-var r <= s_3_13
        fn_state.r = s_3_13;
        // N s_3_21: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var r:i64
        let s_4_0: i64 = fn_state.r;
        // D s_4_1: read-var gs#326705:i64
        let s_4_1: i64 = fn_state.gs_326705;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b14 b5
        if s_4_2 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var d:i64
        let s_5_0: i64 = fn_state.d;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: read-var r:i64
        let s_5_2: i64 = fn_state.r;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: add s_5_1 s_5_3
        let s_5_4: i128 = (s_5_1 + s_5_3);
        // D s_5_5: cast reint s_5_4 -> i64
        let s_5_5: i64 = (s_5_4 as i64);
        // D s_5_6: cast zx s_5_5 -> i
        let s_5_6: i128 = (i128::try_from(s_5_5).unwrap());
        // D s_5_7: call D_read(s_5_6)
        let s_5_7: u64 = D_read(state, tracer, s_5_6);
        // D s_5_8: write-var operand3 <= s_5_7
        fn_state.operand3 = s_5_7;
        // C s_5_9: const #0s : i64
        let s_5_9: i64 = 0;
        // D s_5_10: write-var e <= s_5_9
        fn_state.e = s_5_9;
        // N s_5_11: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var e:i64
        let s_6_0: i64 = fn_state.e;
        // C s_6_1: const #1s : i64
        let s_6_1: i64 = 1;
        // D s_6_2: cmp-gt s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) > (s_6_1));
        // N s_6_3: branch s_6_2 b13 b7
        if s_6_2 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #2s : i
        let s_7_0: i128 = 2;
        // D s_7_1: read-var r:i64
        let s_7_1: i64 = fn_state.r;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: mul s_7_0 s_7_2
        let s_7_3: i128 = ((s_7_0) * (s_7_2));
        // D s_7_4: cast reint s_7_3 -> i64
        let s_7_4: i64 = (s_7_3 as i64);
        // D s_7_5: cast zx s_7_4 -> i
        let s_7_5: i128 = (i128::try_from(s_7_4).unwrap());
        // D s_7_6: read-var e:i64
        let s_7_6: i64 = fn_state.e;
        // D s_7_7: cast zx s_7_6 -> i
        let s_7_7: i128 = (i128::try_from(s_7_6).unwrap());
        // D s_7_8: add s_7_5 s_7_7
        let s_7_8: i128 = (s_7_5 + s_7_7);
        // D s_7_9: cast reint s_7_8 -> i64
        let s_7_9: i64 = (s_7_8 as i64);
        // C s_7_10: const #2s : i
        let s_7_10: i128 = 2;
        // D s_7_11: read-var esize:i64
        let s_7_11: i64 = fn_state.esize;
        // D s_7_12: cast zx s_7_11 -> i
        let s_7_12: i128 = (i128::try_from(s_7_11).unwrap());
        // D s_7_13: div s_7_12 s_7_10
        let s_7_13: i128 = ((s_7_12) / (s_7_10));
        // D s_7_14: cast reint s_7_13 -> i64
        let s_7_14: i64 = (s_7_13 as i64);
        // D s_7_15: cast zx s_7_14 -> i
        let s_7_15: i128 = (i128::try_from(s_7_14).unwrap());
        // D s_7_16: cast reint s_7_15 -> i64
        let s_7_16: i64 = (s_7_15 as i64);
        // D s_7_17: cast zx s_7_9 -> i
        let s_7_17: i128 = (i128::try_from(s_7_9).unwrap());
        // D s_7_18: cast zx s_7_16 -> i
        let s_7_18: i128 = (i128::try_from(s_7_16).unwrap());
        // D s_7_19: read-var operand1:bv
        let s_7_19: Bits = fn_state.operand1;
        // D s_7_20: call Elem_read(s_7_19, s_7_17, s_7_18)
        let s_7_20: Bits = Elem_read(state, tracer, s_7_19, s_7_17, s_7_18);
        // D s_7_21: write-var element1 <= s_7_20
        fn_state.element1 = s_7_20;
        // D s_7_22: read-var sub_op:u8
        let s_7_22: bool = fn_state.sub_op;
        // N s_7_23: branch s_7_22 b11 b8
        if s_7_22 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var esize:i64
        let s_9_0: i64 = fn_state.esize;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: cast reint s_9_1 -> i64
        let s_9_2: i64 = (s_9_1 as i64);
        // D s_9_3: write-var ga#366895 <= s_9_2
        fn_state.ga_366895 = s_9_2;
        // D s_9_4: read-var esize:i64
        let s_9_4: i64 = fn_state.esize;
        // D s_9_5: cast zx s_9_4 -> i
        let s_9_5: i128 = (i128::try_from(s_9_4).unwrap());
        // D s_9_6: cast reint s_9_5 -> i64
        let s_9_6: i64 = (s_9_5 as i64);
        // D s_9_7: read-var operand3:u64
        let s_9_7: u64 = fn_state.operand3;
        // D s_9_8: cast zx s_9_7 -> bv
        let s_9_8: Bits = Bits::new(s_9_7 as u128, 64u16);
        // D s_9_9: read-var e:i64
        let s_9_9: i64 = fn_state.e;
        // D s_9_10: cast zx s_9_9 -> i
        let s_9_10: i128 = (i128::try_from(s_9_9).unwrap());
        // D s_9_11: cast zx s_9_6 -> i
        let s_9_11: i128 = (i128::try_from(s_9_6).unwrap());
        // D s_9_12: call Elem_read(s_9_8, s_9_10, s_9_11)
        let s_9_12: Bits = Elem_read(state, tracer, s_9_8, s_9_10, s_9_11);
        // D s_9_13: cast reint s_9_12 -> u32
        let s_9_13: u32 = (s_9_12.value() as u32);
        // C s_9_14: const #() : ()
        let s_9_14: () = ();
        // S s_9_15: call StandardFPSCRValue(s_9_14)
        let s_9_15: ProductType5c790c8ef59cc8b2 = StandardFPSCRValue(
            state,
            tracer,
            s_9_14,
        );
        // D s_9_16: cast zx s_9_13 -> bv
        let s_9_16: Bits = Bits::new(s_9_13 as u128, 32u16);
        // D s_9_17: read-var element1:bv
        let s_9_17: Bits = fn_state.element1;
        // D s_9_18: read-var element2:bv
        let s_9_18: Bits = fn_state.element2;
        // D s_9_19: call FPMulAddH(s_9_16, s_9_17, s_9_18, s_9_15)
        let s_9_19: Bits = FPMulAddH(state, tracer, s_9_16, s_9_17, s_9_18, s_9_15);
        // D s_9_20: write-var gs#916951 <= s_9_19
        fn_state.gs_916951 = s_9_19;
        // N s_9_21: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#916951:bv
        let s_10_0: Bits = fn_state.gs_916951;
        // D s_10_1: cast reint s_10_0 -> u32
        let s_10_1: u32 = (s_10_0.value() as u32);
        // D s_10_2: read-var result:u64
        let s_10_2: u64 = fn_state.result;
        // D s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 64u16);
        // D s_10_4: read-var e:i64
        let s_10_4: i64 = fn_state.e;
        // D s_10_5: cast zx s_10_4 -> i
        let s_10_5: i128 = (i128::try_from(s_10_4).unwrap());
        // D s_10_6: read-var ga#366895:i64
        let s_10_6: i64 = fn_state.ga_366895;
        // D s_10_7: cast zx s_10_6 -> i
        let s_10_7: i128 = (i128::try_from(s_10_6).unwrap());
        // D s_10_8: cast zx s_10_1 -> bv
        let s_10_8: Bits = Bits::new(s_10_1 as u128, 32u16);
        // D s_10_9: call Elem_set(s_10_3, s_10_5, s_10_7, s_10_8)
        let s_10_9: Bits = Elem_set(state, tracer, s_10_3, s_10_5, s_10_7, s_10_8);
        // D s_10_10: cast reint s_10_9 -> u64
        let s_10_10: u64 = (s_10_9.value() as u64);
        // D s_10_11: write-var result <= s_10_10
        fn_state.result = s_10_10;
        // D s_10_12: read-var e:i64
        let s_10_12: i64 = fn_state.e;
        // C s_10_13: const #1s : i64
        let s_10_13: i64 = 1;
        // D s_10_14: add s_10_12 s_10_13
        let s_10_14: i64 = (s_10_12 + s_10_13);
        // D s_10_15: write-var e <= s_10_14
        fn_state.e = s_10_14;
        // N s_10_16: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var element1:bv
        let s_11_0: Bits = fn_state.element1;
        // D s_11_1: call FPNeg(s_11_0)
        let s_11_1: Bits = FPNeg(state, tracer, s_11_0);
        // D s_11_2: write-var element1 <= s_11_1
        fn_state.element1 = s_11_1;
        // N s_11_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var d:i64
        let s_13_0: i64 = fn_state.d;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: read-var r:i64
        let s_13_2: i64 = fn_state.r;
        // D s_13_3: cast zx s_13_2 -> i
        let s_13_3: i128 = (i128::try_from(s_13_2).unwrap());
        // D s_13_4: add s_13_1 s_13_3
        let s_13_4: i128 = (s_13_1 + s_13_3);
        // D s_13_5: cast reint s_13_4 -> i64
        let s_13_5: i64 = (s_13_4 as i64);
        // D s_13_6: cast zx s_13_5 -> i
        let s_13_6: i128 = (i128::try_from(s_13_5).unwrap());
        // D s_13_7: read-var result:u64
        let s_13_7: u64 = fn_state.result;
        // D s_13_8: call D_set(s_13_6, s_13_7)
        let s_13_8: () = D_set(state, tracer, s_13_6, s_13_7);
        // D s_13_9: read-var r:i64
        let s_13_9: i64 = fn_state.r;
        // C s_13_10: const #1s : i64
        let s_13_10: i64 = 1;
        // D s_13_11: add s_13_9 s_13_10
        let s_13_11: i64 = (s_13_9 + s_13_10);
        // D s_13_12: write-var r <= s_13_11
        fn_state.r = s_13_11;
        // N s_13_13: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_14_0: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var datasizeshadow#7992:i64
        let s_15_0: i64 = fn_state.datasizeshadow_7992;
        // D s_15_1: cast zx s_15_0 -> i
        let s_15_1: i128 = (i128::try_from(s_15_0).unwrap());
        // D s_15_2: call __id(s_15_1)
        let s_15_2: i128 = u__id(state, tracer, s_15_1);
        // D s_15_3: cast reint s_15_2 -> i64
        let s_15_3: i64 = (s_15_2 as i64);
        // C s_15_4: const #1s : i
        let s_15_4: i128 = 1;
        // D s_15_5: cast zx s_15_3 -> i
        let s_15_5: i128 = (i128::try_from(s_15_3).unwrap());
        // D s_15_6: sub s_15_5 s_15_4
        let s_15_6: i128 = ((s_15_5) - (s_15_4));
        // D s_15_7: cast reint s_15_6 -> i64
        let s_15_7: i64 = (s_15_6 as i64);
        // C s_15_8: const #0s : i
        let s_15_8: i128 = 0;
        // D s_15_9: cast zx s_15_7 -> i
        let s_15_9: i128 = (i128::try_from(s_15_7).unwrap());
        // D s_15_10: cmp-le s_15_8 s_15_9
        let s_15_10: bool = ((s_15_8) <= (s_15_9));
        // N s_15_11: branch s_15_10 b18 b16
        if s_15_10 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var gs#326694 <= s_16_0
        fn_state.gs_326694 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#326694:u8
        let s_17_0: bool = fn_state.gs_326694;
        // N s_17_1: assert s_17_0
        let s_17_1: () = assert!(s_17_0);
        // D s_17_2: read-var n:i64
        let s_17_2: i64 = fn_state.n;
        // D s_17_3: cast zx s_17_2 -> i
        let s_17_3: i128 = (i128::try_from(s_17_2).unwrap());
        // D s_17_4: call S_read(s_17_3)
        let s_17_4: u32 = S_read(state, tracer, s_17_3);
        // C s_17_5: const #0s : i
        let s_17_5: i128 = 0;
        // C s_17_6: const #32s : i
        let s_17_6: i128 = 32;
        // D s_17_7: cast zx s_17_4 -> bv
        let s_17_7: Bits = Bits::new(s_17_4 as u128, 32u16);
        // D s_17_8: bit-extract s_17_7 s_17_5 s_17_6
        let s_17_8: Bits = (Bits::new(
            ((s_17_7) >> (s_17_5)).value(),
            u16::try_from(s_17_6).unwrap(),
        ));
        // D s_17_9: write-var operand1 <= s_17_8
        fn_state.operand1 = s_17_8;
        // D s_17_10: read-var m:i64
        let s_17_10: i64 = fn_state.m;
        // D s_17_11: cast zx s_17_10 -> i
        let s_17_11: i128 = (i128::try_from(s_17_10).unwrap());
        // D s_17_12: call S_read(s_17_11)
        let s_17_12: u32 = S_read(state, tracer, s_17_11);
        // C s_17_13: const #0s : i
        let s_17_13: i128 = 0;
        // C s_17_14: const #32s : i
        let s_17_14: i128 = 32;
        // D s_17_15: cast zx s_17_12 -> bv
        let s_17_15: Bits = Bits::new(s_17_12 as u128, 32u16);
        // D s_17_16: bit-extract s_17_15 s_17_13 s_17_14
        let s_17_16: Bits = (Bits::new(
            ((s_17_15) >> (s_17_13)).value(),
            u16::try_from(s_17_14).unwrap(),
        ));
        // D s_17_17: write-var operand2 <= s_17_16
        fn_state.operand2 = s_17_16;
        // N s_17_18: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var datasizeshadow#7992:i64
        let s_18_0: i64 = fn_state.datasizeshadow_7992;
        // D s_18_1: cast zx s_18_0 -> i
        let s_18_1: i128 = (i128::try_from(s_18_0).unwrap());
        // D s_18_2: call __id(s_18_1)
        let s_18_2: i128 = u__id(state, tracer, s_18_1);
        // D s_18_3: cast reint s_18_2 -> i64
        let s_18_3: i64 = (s_18_2 as i64);
        // C s_18_4: const #1s : i
        let s_18_4: i128 = 1;
        // D s_18_5: cast zx s_18_3 -> i
        let s_18_5: i128 = (i128::try_from(s_18_3).unwrap());
        // D s_18_6: sub s_18_5 s_18_4
        let s_18_6: i128 = ((s_18_5) - (s_18_4));
        // D s_18_7: cast reint s_18_6 -> i64
        let s_18_7: i64 = (s_18_6 as i64);
        // C s_18_8: const #32s : i
        let s_18_8: i128 = 32;
        // D s_18_9: cast zx s_18_7 -> i
        let s_18_9: i128 = (i128::try_from(s_18_7).unwrap());
        // D s_18_10: cmp-lt s_18_9 s_18_8
        let s_18_10: bool = ((s_18_9) < (s_18_8));
        // D s_18_11: write-var gs#326694 <= s_18_10
        fn_state.gs_326694 = s_18_10;
        // N s_18_12: jump b17
        return block_17(state, tracer, fn_state);
    }
}
