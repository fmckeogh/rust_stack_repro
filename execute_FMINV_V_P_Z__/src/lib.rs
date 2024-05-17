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
use FPInfinity::*;
use u__id::*;
use CheckSVEEnabled::*;
use AnyActiveElement::*;
use P_read::*;
use V_set::*;
use ReducePredicated::*;
use Zeros::*;
use Z_read::*;
use common::*;
pub fn execute_FMINV_V_P_Z__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    g: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: Bits,
        ga_273537: i64,
        gs_179154: bool,
        gs_179152: bool,
        VLshadow_2216: i64,
        VLshadow_2217: i64,
        ga_273538: Bits,
        mask: Bits,
        esizeshadow_2215: i64,
        VL: i64,
        d: i64,
        esize: i64,
        g: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        g,
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
        // D s_0_3: write-var esizeshadow#2215 <= s_0_2
        fn_state.esizeshadow_2215 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#2216 <= s_0_6
        fn_state.VLshadow_2216 = s_0_6;
        // C s_0_8: const #() : ()
        let s_0_8: () = ();
        // S s_0_9: call CheckSVEEnabled(s_0_8)
        let s_0_9: () = CheckSVEEnabled(state, tracer, s_0_8);
        // N s_0_10: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#2216:i64
        let s_1_0: i64 = fn_state.VLshadow_2216;
        // D s_1_1: write-var VLshadow#2217 <= s_1_0
        fn_state.VLshadow_2217 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#2217:i64
        let s_1_3: i64 = fn_state.VLshadow_2217;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
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
        // D s_1_13: write-var mask <= s_1_12
        fn_state.mask = s_1_12;
        // D s_1_14: read-var esizeshadow#2215:i64
        let s_1_14: i64 = fn_state.esizeshadow_2215;
        // D s_1_15: cast zx s_1_14 -> i
        let s_1_15: i128 = (i128::try_from(s_1_14).unwrap());
        // D s_1_16: read-var mask:bv
        let s_1_16: Bits = fn_state.mask;
        // D s_1_17: call AnyActiveElement(s_1_16, s_1_15)
        let s_1_17: bool = AnyActiveElement(state, tracer, s_1_16, s_1_15);
        // N s_1_18: branch s_1_17 b11 b2
        if s_1_17 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var VLshadow#2217:i64
        let s_2_0: i64 = fn_state.VLshadow_2217;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: call Zeros(s_2_1)
        let s_2_2: Bits = Zeros(state, tracer, s_2_1);
        // D s_2_3: write-var operand <= s_2_2
        fn_state.operand = s_2_2;
        // N s_2_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var esizeshadow#2215:i64
        let s_3_0: i64 = fn_state.esizeshadow_2215;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: call __id(s_3_1)
        let s_3_2: i128 = u__id(state, tracer, s_3_1);
        // D s_3_3: cast reint s_3_2 -> i64
        let s_3_3: i64 = (s_3_2 as i64);
        // C s_3_4: const #16s : i
        let s_3_4: i128 = 16;
        // D s_3_5: cast zx s_3_3 -> i
        let s_3_5: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_6: cmp-eq s_3_5 s_3_4
        let s_3_6: bool = ((s_3_5) == (s_3_4));
        // N s_3_7: branch s_3_6 b10 b4
        if s_3_6 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var esizeshadow#2215:i64
        let s_4_0: i64 = fn_state.esizeshadow_2215;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: call __id(s_4_1)
        let s_4_2: i128 = u__id(state, tracer, s_4_1);
        // D s_4_3: cast reint s_4_2 -> i64
        let s_4_3: i64 = (s_4_2 as i64);
        // C s_4_4: const #32s : i
        let s_4_4: i128 = 32;
        // D s_4_5: cast zx s_4_3 -> i
        let s_4_5: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_6: cmp-eq s_4_5 s_4_4
        let s_4_6: bool = ((s_4_5) == (s_4_4));
        // D s_4_7: write-var gs#179152 <= s_4_6
        fn_state.gs_179152 = s_4_6;
        // N s_4_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#179152:u8
        let s_5_0: bool = fn_state.gs_179152;
        // N s_5_1: branch s_5_0 b9 b6
        if s_5_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var esizeshadow#2215:i64
        let s_6_0: i64 = fn_state.esizeshadow_2215;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: call __id(s_6_1)
        let s_6_2: i128 = u__id(state, tracer, s_6_1);
        // D s_6_3: cast reint s_6_2 -> i64
        let s_6_3: i64 = (s_6_2 as i64);
        // C s_6_4: const #64s : i
        let s_6_4: i128 = 64;
        // D s_6_5: cast zx s_6_3 -> i
        let s_6_5: i128 = (i128::try_from(s_6_3).unwrap());
        // D s_6_6: cmp-eq s_6_5 s_6_4
        let s_6_6: bool = ((s_6_5) == (s_6_4));
        // D s_6_7: write-var gs#179154 <= s_6_6
        fn_state.gs_179154 = s_6_6;
        // N s_6_8: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#179154:u8
        let s_7_0: bool = fn_state.gs_179154;
        // N s_7_1: assert s_7_0
        let s_7_1: () = assert!(s_7_0);
        // D s_7_2: read-var esizeshadow#2215:i64
        let s_7_2: i64 = fn_state.esizeshadow_2215;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: cast reint s_7_3 -> i64
        let s_7_4: i64 = (s_7_3 as i64);
        // C s_7_5: const #0u : u8
        let s_7_5: bool = false;
        // D s_7_6: call FPInfinity(s_7_5, s_7_4)
        let s_7_6: Bits = FPInfinity(state, tracer, s_7_5, s_7_4);
        // D s_7_7: read-var esizeshadow#2215:i64
        let s_7_7: i64 = fn_state.esizeshadow_2215;
        // D s_7_8: cast zx s_7_7 -> i
        let s_7_8: i128 = (i128::try_from(s_7_7).unwrap());
        // D s_7_9: cast reint s_7_8 -> i64
        let s_7_9: i64 = (s_7_8 as i64);
        // D s_7_10: write-var ga#273537 <= s_7_9
        fn_state.ga_273537 = s_7_9;
        // C s_7_11: const #2u : u32
        let s_7_11: u32 = 2;
        // D s_7_12: read-var operand:bv
        let s_7_12: Bits = fn_state.operand;
        // D s_7_13: read-var mask:bv
        let s_7_13: Bits = fn_state.mask;
        // D s_7_14: call ReducePredicated(s_7_11, s_7_12, s_7_13, s_7_6)
        let s_7_14: Bits = ReducePredicated(
            state,
            tracer,
            s_7_11,
            s_7_12,
            s_7_13,
            s_7_6,
        );
        // D s_7_15: write-var ga#273538 <= s_7_14
        fn_state.ga_273538 = s_7_14;
        // N s_7_16: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var d:i64
        let s_8_0: i64 = fn_state.d;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: read-var ga#273537:i64
        let s_8_2: i64 = fn_state.ga_273537;
        // D s_8_3: read-var ga#273538:bv
        let s_8_3: Bits = fn_state.ga_273538;
        // D s_8_4: call V_set(s_8_1, s_8_2, s_8_3)
        let s_8_4: () = V_set(state, tracer, s_8_1, s_8_2, s_8_3);
        // N s_8_5: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #1u : u8
        let s_9_0: bool = true;
        // D s_9_1: write-var gs#179154 <= s_9_0
        fn_state.gs_179154 = s_9_0;
        // N s_9_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #1u : u8
        let s_10_0: bool = true;
        // D s_10_1: write-var gs#179152 <= s_10_0
        fn_state.gs_179152 = s_10_0;
        // N s_10_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var VLshadow#2217:i64
        let s_11_0: i64 = fn_state.VLshadow_2217;
        // D s_11_1: cast zx s_11_0 -> i
        let s_11_1: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_2: cast reint s_11_1 -> i64
        let s_11_2: i64 = (s_11_1 as i64);
        // D s_11_3: read-var n:i64
        let s_11_3: i64 = fn_state.n;
        // D s_11_4: cast zx s_11_3 -> i
        let s_11_4: i128 = (i128::try_from(s_11_3).unwrap());
        // D s_11_5: cast zx s_11_2 -> i
        let s_11_5: i128 = (i128::try_from(s_11_2).unwrap());
        // D s_11_6: call Z_read(s_11_4, s_11_5)
        let s_11_6: Bits = Z_read(state, tracer, s_11_4, s_11_5);
        // D s_11_7: write-var operand <= s_11_6
        fn_state.operand = s_11_6;
        // N s_11_8: jump b3
        return block_3(state, tracer, fn_state);
    }
}
