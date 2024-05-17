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
pub fn execute_aarch32_instrs_VADDHN_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d__arg: i64,
    elements: i128,
    esize: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        esizeshadow_7362: i64,
        d: i128,
        gs_306268: i64,
        d__arg: i64,
        elements: i128,
        esize: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        d__arg,
        elements,
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
        // D s_0_0: read-var esize:i64
        let s_0_0: i64 = fn_state.esize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var esizeshadow#7362 <= s_0_2
        fn_state.esizeshadow_7362 = s_0_2;
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
        // D s_1_5: write-var gs#306268 <= s_1_4
        fn_state.gs_306268 = s_1_4;
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
        // D s_2_1: read-var gs#306268:i64
        let s_2_1: i64 = fn_state.gs_306268;
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
        // C s_3_0: const #1s : i64
        let s_3_0: i64 = 1;
        // D s_3_1: read-var n:i64
        let s_3_1: i64 = fn_state.n;
        // D s_3_2: lsr s_3_1 s_3_0
        let s_3_2: i64 = s_3_1 >> s_3_0;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: call Qin_read(s_3_3)
        let s_3_4: u128 = Qin_read(state, tracer, s_3_3);
        // C s_3_5: const #2s : i
        let s_3_5: i128 = 2;
        // D s_3_6: read-var esizeshadow#7362:i64
        let s_3_6: i64 = fn_state.esizeshadow_7362;
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_8: mul s_3_5 s_3_7
        let s_3_8: i128 = ((s_3_5) * (s_3_7));
        // D s_3_9: cast reint s_3_8 -> i64
        let s_3_9: i64 = (s_3_8 as i64);
        // D s_3_10: cast zx s_3_9 -> i
        let s_3_10: i128 = (i128::try_from(s_3_9).unwrap());
        // D s_3_11: cast reint s_3_10 -> i64
        let s_3_11: i64 = (s_3_10 as i64);
        // D s_3_12: cast zx s_3_4 -> bv
        let s_3_12: Bits = Bits::new(s_3_4 as u128, 128u16);
        // D s_3_13: read-var e:i64
        let s_3_13: i64 = fn_state.e;
        // D s_3_14: cast zx s_3_13 -> i
        let s_3_14: i128 = (i128::try_from(s_3_13).unwrap());
        // D s_3_15: cast zx s_3_11 -> i
        let s_3_15: i128 = (i128::try_from(s_3_11).unwrap());
        // D s_3_16: call Elem_read(s_3_12, s_3_14, s_3_15)
        let s_3_16: Bits = Elem_read(state, tracer, s_3_12, s_3_14, s_3_15);
        // C s_3_17: const #1s : i64
        let s_3_17: i64 = 1;
        // D s_3_18: read-var m:i64
        let s_3_18: i64 = fn_state.m;
        // D s_3_19: lsr s_3_18 s_3_17
        let s_3_19: i64 = s_3_18 >> s_3_17;
        // D s_3_20: cast zx s_3_19 -> i
        let s_3_20: i128 = (i128::try_from(s_3_19).unwrap());
        // D s_3_21: call Qin_read(s_3_20)
        let s_3_21: u128 = Qin_read(state, tracer, s_3_20);
        // C s_3_22: const #2s : i
        let s_3_22: i128 = 2;
        // D s_3_23: read-var esizeshadow#7362:i64
        let s_3_23: i64 = fn_state.esizeshadow_7362;
        // D s_3_24: cast zx s_3_23 -> i
        let s_3_24: i128 = (i128::try_from(s_3_23).unwrap());
        // D s_3_25: mul s_3_22 s_3_24
        let s_3_25: i128 = ((s_3_22) * (s_3_24));
        // D s_3_26: cast reint s_3_25 -> i64
        let s_3_26: i64 = (s_3_25 as i64);
        // D s_3_27: cast zx s_3_26 -> i
        let s_3_27: i128 = (i128::try_from(s_3_26).unwrap());
        // D s_3_28: cast reint s_3_27 -> i64
        let s_3_28: i64 = (s_3_27 as i64);
        // D s_3_29: cast zx s_3_21 -> bv
        let s_3_29: Bits = Bits::new(s_3_21 as u128, 128u16);
        // D s_3_30: read-var e:i64
        let s_3_30: i64 = fn_state.e;
        // D s_3_31: cast zx s_3_30 -> i
        let s_3_31: i128 = (i128::try_from(s_3_30).unwrap());
        // D s_3_32: cast zx s_3_28 -> i
        let s_3_32: i128 = (i128::try_from(s_3_28).unwrap());
        // D s_3_33: call Elem_read(s_3_29, s_3_31, s_3_32)
        let s_3_33: Bits = Elem_read(state, tracer, s_3_29, s_3_31, s_3_32);
        // D s_3_34: add s_3_16 s_3_33
        let s_3_34: Bits = (s_3_16 + s_3_33);
        // D s_3_35: read-var d:i
        let s_3_35: i128 = fn_state.d;
        // D s_3_36: call D_read(s_3_35)
        let s_3_36: u64 = D_read(state, tracer, s_3_35);
        // D s_3_37: read-var esizeshadow#7362:i64
        let s_3_37: i64 = fn_state.esizeshadow_7362;
        // D s_3_38: cast zx s_3_37 -> i
        let s_3_38: i128 = (i128::try_from(s_3_37).unwrap());
        // D s_3_39: cast reint s_3_38 -> i64
        let s_3_39: i64 = (s_3_38 as i64);
        // C s_3_40: const #2s : i
        let s_3_40: i128 = 2;
        // D s_3_41: read-var esizeshadow#7362:i64
        let s_3_41: i64 = fn_state.esizeshadow_7362;
        // D s_3_42: cast zx s_3_41 -> i
        let s_3_42: i128 = (i128::try_from(s_3_41).unwrap());
        // D s_3_43: mul s_3_40 s_3_42
        let s_3_43: i128 = ((s_3_40) * (s_3_42));
        // D s_3_44: cast reint s_3_43 -> i64
        let s_3_44: i64 = (s_3_43 as i64);
        // C s_3_45: const #1s : i
        let s_3_45: i128 = 1;
        // D s_3_46: cast zx s_3_44 -> i
        let s_3_46: i128 = (i128::try_from(s_3_44).unwrap());
        // D s_3_47: sub s_3_46 s_3_45
        let s_3_47: i128 = ((s_3_46) - (s_3_45));
        // D s_3_48: cast reint s_3_47 -> i64
        let s_3_48: i64 = (s_3_47 as i64);
        // D s_3_49: cast zx s_3_48 -> i
        let s_3_49: i128 = (i128::try_from(s_3_48).unwrap());
        // D s_3_50: read-var esizeshadow#7362:i64
        let s_3_50: i64 = fn_state.esizeshadow_7362;
        // D s_3_51: cast zx s_3_50 -> i
        let s_3_51: i128 = (i128::try_from(s_3_50).unwrap());
        // C s_3_52: const #1s : i64
        let s_3_52: i64 = 1;
        // C s_3_53: cast zx s_3_52 -> i
        let s_3_53: i128 = (i128::try_from(s_3_52).unwrap());
        // D s_3_54: sub s_3_49 s_3_51
        let s_3_54: i128 = ((s_3_49) - (s_3_51));
        // D s_3_55: add s_3_54 s_3_53
        let s_3_55: i128 = (s_3_54 + s_3_53);
        // D s_3_56: bit-extract s_3_34 s_3_51 s_3_55
        let s_3_56: Bits = (Bits::new(
            ((s_3_34) >> (s_3_51)).value(),
            u16::try_from(s_3_55).unwrap(),
        ));
        // D s_3_57: cast zx s_3_36 -> bv
        let s_3_57: Bits = Bits::new(s_3_36 as u128, 64u16);
        // D s_3_58: read-var e:i64
        let s_3_58: i64 = fn_state.e;
        // D s_3_59: cast zx s_3_58 -> i
        let s_3_59: i128 = (i128::try_from(s_3_58).unwrap());
        // D s_3_60: cast zx s_3_39 -> i
        let s_3_60: i128 = (i128::try_from(s_3_39).unwrap());
        // D s_3_61: call Elem_set(s_3_57, s_3_59, s_3_60, s_3_56)
        let s_3_61: Bits = Elem_set(state, tracer, s_3_57, s_3_59, s_3_60, s_3_56);
        // D s_3_62: cast reint s_3_61 -> u64
        let s_3_62: u64 = (s_3_61.value() as u64);
        // D s_3_63: read-var d:i
        let s_3_63: i128 = fn_state.d;
        // D s_3_64: call D_set(s_3_63, s_3_62)
        let s_3_64: () = D_set(state, tracer, s_3_63, s_3_62);
        // D s_3_65: read-var e:i64
        let s_3_65: i64 = fn_state.e;
        // C s_3_66: const #1s : i64
        let s_3_66: i64 = 1;
        // D s_3_67: add s_3_65 s_3_66
        let s_3_67: i64 = (s_3_65 + s_3_66);
        // D s_3_68: write-var e <= s_3_67
        fn_state.e = s_3_67;
        // N s_3_69: jump b2
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
