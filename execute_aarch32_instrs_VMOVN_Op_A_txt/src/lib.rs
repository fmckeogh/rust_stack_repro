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
use Qin_read::*;
use common::*;
pub fn execute_aarch32_instrs_VMOVN_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d__arg: i64,
    elements: i128,
    esize: i64,
    m: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        esizeshadow_7550: i64,
        gs_313113: i64,
        d: i128,
        d__arg: i64,
        elements: i128,
        esize: i64,
        m: i64,
    }
    let fn_state = FunctionState {
        d__arg,
        elements,
        esize,
        m,
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
        // D s_0_3: write-var esizeshadow#7550 <= s_0_2
        fn_state.esizeshadow_7550 = s_0_2;
        // D s_0_4: read-var d__arg:i64
        let s_0_4: i64 = fn_state.d__arg;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: write-var d <= s_0_5
        fn_state.d = s_0_5;
        // C s_0_7: const #() : ()
        let s_0_7: () = ();
        // S s_0_8: call CheckAdvSIMDEnabled(s_0_7)
        let s_0_8: () = CheckAdvSIMDEnabled(state, tracer, s_0_7);
        // N s_0_9: jump b1
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
        // D s_1_2: read-var elements:i
        let s_1_2: i128 = fn_state.elements;
        // D s_1_3: sub s_1_2 s_1_1
        let s_1_3: i128 = ((s_1_2) - (s_1_1));
        // D s_1_4: cast reint s_1_3 -> i64
        let s_1_4: i64 = (s_1_3 as i64);
        // D s_1_5: write-var gs#313113 <= s_1_4
        fn_state.gs_313113 = s_1_4;
        // D s_1_6: write-var e <= s_1_0
        fn_state.e = s_1_0;
        // N s_1_7: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#313113:i64
        let s_2_1: i64 = fn_state.gs_313113;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b4 b3
        if s_2_2 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var d:i
        let s_3_0: i128 = fn_state.d;
        // D s_3_1: call D_read(s_3_0)
        let s_3_1: u64 = D_read(state, tracer, s_3_0);
        // D s_3_2: read-var esizeshadow#7550:i64
        let s_3_2: i64 = fn_state.esizeshadow_7550;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: cast reint s_3_3 -> i64
        let s_3_4: i64 = (s_3_3 as i64);
        // C s_3_5: const #1s : i64
        let s_3_5: i64 = 1;
        // D s_3_6: read-var m:i64
        let s_3_6: i64 = fn_state.m;
        // D s_3_7: lsr s_3_6 s_3_5
        let s_3_7: i64 = s_3_6 >> s_3_5;
        // D s_3_8: cast zx s_3_7 -> i
        let s_3_8: i128 = (i128::try_from(s_3_7).unwrap());
        // D s_3_9: call Qin_read(s_3_8)
        let s_3_9: u128 = Qin_read(state, tracer, s_3_8);
        // C s_3_10: const #2s : i
        let s_3_10: i128 = 2;
        // D s_3_11: read-var esizeshadow#7550:i64
        let s_3_11: i64 = fn_state.esizeshadow_7550;
        // D s_3_12: cast zx s_3_11 -> i
        let s_3_12: i128 = (i128::try_from(s_3_11).unwrap());
        // D s_3_13: mul s_3_10 s_3_12
        let s_3_13: i128 = ((s_3_10) * (s_3_12));
        // D s_3_14: cast reint s_3_13 -> i64
        let s_3_14: i64 = (s_3_13 as i64);
        // D s_3_15: cast zx s_3_14 -> i
        let s_3_15: i128 = (i128::try_from(s_3_14).unwrap());
        // D s_3_16: cast reint s_3_15 -> i64
        let s_3_16: i64 = (s_3_15 as i64);
        // D s_3_17: cast zx s_3_9 -> bv
        let s_3_17: Bits = Bits::new(s_3_9 as u128, 128u16);
        // D s_3_18: read-var e:i64
        let s_3_18: i64 = fn_state.e;
        // D s_3_19: cast zx s_3_18 -> i
        let s_3_19: i128 = (i128::try_from(s_3_18).unwrap());
        // D s_3_20: cast zx s_3_16 -> i
        let s_3_20: i128 = (i128::try_from(s_3_16).unwrap());
        // D s_3_21: call Elem_read(s_3_17, s_3_19, s_3_20)
        let s_3_21: Bits = Elem_read(state, tracer, s_3_17, s_3_19, s_3_20);
        // C s_3_22: const #1s : i
        let s_3_22: i128 = 1;
        // D s_3_23: read-var esizeshadow#7550:i64
        let s_3_23: i64 = fn_state.esizeshadow_7550;
        // D s_3_24: cast zx s_3_23 -> i
        let s_3_24: i128 = (i128::try_from(s_3_23).unwrap());
        // D s_3_25: sub s_3_24 s_3_22
        let s_3_25: i128 = ((s_3_24) - (s_3_22));
        // D s_3_26: cast reint s_3_25 -> i64
        let s_3_26: i64 = (s_3_25 as i64);
        // C s_3_27: const #0s : i
        let s_3_27: i128 = 0;
        // D s_3_28: cast zx s_3_26 -> i
        let s_3_28: i128 = (i128::try_from(s_3_26).unwrap());
        // C s_3_29: const #1s : i64
        let s_3_29: i64 = 1;
        // C s_3_30: cast zx s_3_29 -> i
        let s_3_30: i128 = (i128::try_from(s_3_29).unwrap());
        // D s_3_31: sub s_3_28 s_3_27
        let s_3_31: i128 = ((s_3_28) - (s_3_27));
        // D s_3_32: add s_3_31 s_3_30
        let s_3_32: i128 = (s_3_31 + s_3_30);
        // D s_3_33: bit-extract s_3_21 s_3_27 s_3_32
        let s_3_33: Bits = (Bits::new(
            ((s_3_21) >> (s_3_27)).value(),
            u16::try_from(s_3_32).unwrap(),
        ));
        // D s_3_34: cast zx s_3_1 -> bv
        let s_3_34: Bits = Bits::new(s_3_1 as u128, 64u16);
        // D s_3_35: read-var e:i64
        let s_3_35: i64 = fn_state.e;
        // D s_3_36: cast zx s_3_35 -> i
        let s_3_36: i128 = (i128::try_from(s_3_35).unwrap());
        // D s_3_37: cast zx s_3_4 -> i
        let s_3_37: i128 = (i128::try_from(s_3_4).unwrap());
        // D s_3_38: call Elem_set(s_3_34, s_3_36, s_3_37, s_3_33)
        let s_3_38: Bits = Elem_set(state, tracer, s_3_34, s_3_36, s_3_37, s_3_33);
        // D s_3_39: cast reint s_3_38 -> u64
        let s_3_39: u64 = (s_3_38.value() as u64);
        // D s_3_40: read-var d:i
        let s_3_40: i128 = fn_state.d;
        // D s_3_41: call D_set(s_3_40, s_3_39)
        let s_3_41: () = D_set(state, tracer, s_3_40, s_3_39);
        // D s_3_42: read-var e:i64
        let s_3_42: i64 = fn_state.e;
        // C s_3_43: const #1s : i64
        let s_3_43: i64 = 1;
        // D s_3_44: add s_3_42 s_3_43
        let s_3_44: i64 = (s_3_42 + s_3_43);
        // D s_3_45: write-var e <= s_3_44
        fn_state.e = s_3_44;
        // N s_3_46: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: return
        return;
    }
}
