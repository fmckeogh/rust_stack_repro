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
use D_set::*;
use Elem_set::*;
use Elem_read::*;
use CheckAdvSIMDEnabled::*;
use D_read::*;
use common::*;
pub fn execute_aarch32_instrs_VREV16_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    containers: i128,
    d: i64,
    elements_per_container: i128,
    esize: i64,
    m: i64,
    regs: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        rev_element: i128,
        e: i64,
        gs_317720: i64,
        gs_317735: i64,
        esizeshadow_7752: i64,
        gs_317727: i64,
        element: i128,
        result: u64,
        c: i64,
        containers: i128,
        d: i64,
        elements_per_container: i128,
        esize: i64,
        m: i64,
        regs: i64,
    }
    let fn_state = FunctionState {
        containers,
        d,
        elements_per_container,
        esize,
        m,
        regs,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var esize:i64
        let s_0_0: i64 = fn_state.esize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var esizeshadow#7752 <= s_0_2
        fn_state.esizeshadow_7752 = s_0_2;
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
        // C s_1_0: const #0s : i64
        let s_1_0: i64 = 0;
        // C s_1_1: const #1s : i
        let s_1_1: i128 = 1;
        // D s_1_2: read-var regs:i64
        let s_1_2: i64 = fn_state.regs;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: sub s_1_3 s_1_1
        let s_1_4: i128 = ((s_1_3) - (s_1_1));
        // D s_1_5: cast reint s_1_4 -> i64
        let s_1_5: i64 = (s_1_4 as i64);
        // D s_1_6: write-var gs#317720 <= s_1_5
        fn_state.gs_317720 = s_1_5;
        // D s_1_7: write-var r <= s_1_0
        fn_state.r = s_1_0;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var r:i64
        let s_2_0: i64 = fn_state.r;
        // D s_2_1: read-var gs#317720:i64
        let s_2_1: i64 = fn_state.gs_317720;
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
        // D s_3_1: write-var element <= s_3_0
        fn_state.element = s_3_0;
        // C s_3_2: const #0s : i64
        let s_3_2: i64 = 0;
        // C s_3_3: const #1s : i
        let s_3_3: i128 = 1;
        // D s_3_4: read-var containers:i
        let s_3_4: i128 = fn_state.containers;
        // D s_3_5: sub s_3_4 s_3_3
        let s_3_5: i128 = ((s_3_4) - (s_3_3));
        // D s_3_6: cast reint s_3_5 -> i64
        let s_3_6: i64 = (s_3_5 as i64);
        // D s_3_7: write-var gs#317727 <= s_3_6
        fn_state.gs_317727 = s_3_6;
        // D s_3_8: write-var c <= s_3_2
        fn_state.c = s_3_2;
        // N s_3_9: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var c:i64
        let s_4_0: i64 = fn_state.c;
        // D s_4_1: read-var gs#317727:i64
        let s_4_1: i64 = fn_state.gs_317727;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b9 b5
        if s_4_2 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var element:i
        let s_5_0: i128 = fn_state.element;
        // D s_5_1: read-var elements_per_container:i
        let s_5_1: i128 = fn_state.elements_per_container;
        // D s_5_2: add s_5_0 s_5_1
        let s_5_2: i128 = (s_5_0 + s_5_1);
        // C s_5_3: const #1s : i
        let s_5_3: i128 = 1;
        // D s_5_4: sub s_5_2 s_5_3
        let s_5_4: i128 = ((s_5_2) - (s_5_3));
        // D s_5_5: write-var rev_element <= s_5_4
        fn_state.rev_element = s_5_4;
        // C s_5_6: const #0s : i64
        let s_5_6: i64 = 0;
        // C s_5_7: const #1s : i
        let s_5_7: i128 = 1;
        // D s_5_8: read-var elements_per_container:i
        let s_5_8: i128 = fn_state.elements_per_container;
        // D s_5_9: sub s_5_8 s_5_7
        let s_5_9: i128 = ((s_5_8) - (s_5_7));
        // D s_5_10: cast reint s_5_9 -> i64
        let s_5_10: i64 = (s_5_9 as i64);
        // D s_5_11: write-var gs#317735 <= s_5_10
        fn_state.gs_317735 = s_5_10;
        // D s_5_12: write-var e <= s_5_6
        fn_state.e = s_5_6;
        // N s_5_13: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var e:i64
        let s_6_0: i64 = fn_state.e;
        // D s_6_1: read-var gs#317735:i64
        let s_6_1: i64 = fn_state.gs_317735;
        // D s_6_2: cmp-gt s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) > (s_6_1));
        // N s_6_3: branch s_6_2 b8 b7
        if s_6_2 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var rev_element:i
        let s_7_0: i128 = fn_state.rev_element;
        // D s_7_1: read-var element:i
        let s_7_1: i128 = fn_state.element;
        // D s_7_2: read-var esizeshadow#7752:i64
        let s_7_2: i64 = fn_state.esizeshadow_7752;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: cast reint s_7_3 -> i64
        let s_7_4: i64 = (s_7_3 as i64);
        // D s_7_5: read-var m:i64
        let s_7_5: i64 = fn_state.m;
        // D s_7_6: cast zx s_7_5 -> i
        let s_7_6: i128 = (i128::try_from(s_7_5).unwrap());
        // D s_7_7: read-var r:i64
        let s_7_7: i64 = fn_state.r;
        // D s_7_8: cast zx s_7_7 -> i
        let s_7_8: i128 = (i128::try_from(s_7_7).unwrap());
        // D s_7_9: add s_7_6 s_7_8
        let s_7_9: i128 = (s_7_6 + s_7_8);
        // D s_7_10: cast reint s_7_9 -> i64
        let s_7_10: i64 = (s_7_9 as i64);
        // D s_7_11: cast zx s_7_10 -> i
        let s_7_11: i128 = (i128::try_from(s_7_10).unwrap());
        // D s_7_12: call D_read(s_7_11)
        let s_7_12: u64 = D_read(state, tracer, s_7_11);
        // D s_7_13: read-var esizeshadow#7752:i64
        let s_7_13: i64 = fn_state.esizeshadow_7752;
        // D s_7_14: cast zx s_7_13 -> i
        let s_7_14: i128 = (i128::try_from(s_7_13).unwrap());
        // D s_7_15: cast reint s_7_14 -> i64
        let s_7_15: i64 = (s_7_14 as i64);
        // D s_7_16: cast zx s_7_12 -> bv
        let s_7_16: Bits = Bits::new(s_7_12 as u128, 64u16);
        // D s_7_17: cast zx s_7_15 -> i
        let s_7_17: i128 = (i128::try_from(s_7_15).unwrap());
        // D s_7_18: call Elem_read(s_7_16, s_7_1, s_7_17)
        let s_7_18: Bits = Elem_read(state, tracer, s_7_16, s_7_1, s_7_17);
        // D s_7_19: read-var result:u64
        let s_7_19: u64 = fn_state.result;
        // D s_7_20: cast zx s_7_19 -> bv
        let s_7_20: Bits = Bits::new(s_7_19 as u128, 64u16);
        // D s_7_21: cast zx s_7_4 -> i
        let s_7_21: i128 = (i128::try_from(s_7_4).unwrap());
        // D s_7_22: call Elem_set(s_7_20, s_7_0, s_7_21, s_7_18)
        let s_7_22: Bits = Elem_set(state, tracer, s_7_20, s_7_0, s_7_21, s_7_18);
        // D s_7_23: cast reint s_7_22 -> u64
        let s_7_23: u64 = (s_7_22.value() as u64);
        // D s_7_24: write-var result <= s_7_23
        fn_state.result = s_7_23;
        // C s_7_25: const #1s : i
        let s_7_25: i128 = 1;
        // D s_7_26: read-var element:i
        let s_7_26: i128 = fn_state.element;
        // D s_7_27: add s_7_26 s_7_25
        let s_7_27: i128 = (s_7_26 + s_7_25);
        // D s_7_28: write-var element <= s_7_27
        fn_state.element = s_7_27;
        // C s_7_29: const #1s : i
        let s_7_29: i128 = 1;
        // D s_7_30: read-var rev_element:i
        let s_7_30: i128 = fn_state.rev_element;
        // D s_7_31: sub s_7_30 s_7_29
        let s_7_31: i128 = ((s_7_30) - (s_7_29));
        // D s_7_32: write-var rev_element <= s_7_31
        fn_state.rev_element = s_7_31;
        // D s_7_33: read-var e:i64
        let s_7_33: i64 = fn_state.e;
        // C s_7_34: const #1s : i64
        let s_7_34: i64 = 1;
        // D s_7_35: add s_7_33 s_7_34
        let s_7_35: i64 = (s_7_33 + s_7_34);
        // D s_7_36: write-var e <= s_7_35
        fn_state.e = s_7_35;
        // N s_7_37: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var c:i64
        let s_8_0: i64 = fn_state.c;
        // C s_8_1: const #1s : i64
        let s_8_1: i64 = 1;
        // D s_8_2: add s_8_0 s_8_1
        let s_8_2: i64 = (s_8_0 + s_8_1);
        // D s_8_3: write-var c <= s_8_2
        fn_state.c = s_8_2;
        // N s_8_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var d:i64
        let s_9_0: i64 = fn_state.d;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: read-var r:i64
        let s_9_2: i64 = fn_state.r;
        // D s_9_3: cast zx s_9_2 -> i
        let s_9_3: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_4: add s_9_1 s_9_3
        let s_9_4: i128 = (s_9_1 + s_9_3);
        // D s_9_5: cast reint s_9_4 -> i64
        let s_9_5: i64 = (s_9_4 as i64);
        // D s_9_6: cast zx s_9_5 -> i
        let s_9_6: i128 = (i128::try_from(s_9_5).unwrap());
        // D s_9_7: read-var result:u64
        let s_9_7: u64 = fn_state.result;
        // D s_9_8: call D_set(s_9_6, s_9_7)
        let s_9_8: () = D_set(state, tracer, s_9_6, s_9_7);
        // D s_9_9: read-var r:i64
        let s_9_9: i64 = fn_state.r;
        // C s_9_10: const #1s : i64
        let s_9_10: i64 = 1;
        // D s_9_11: add s_9_9 s_9_10
        let s_9_11: i64 = (s_9_9 + s_9_10);
        // D s_9_12: write-var r <= s_9_11
        fn_state.r = s_9_11;
        // N s_9_13: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: return
        return;
    }
}
