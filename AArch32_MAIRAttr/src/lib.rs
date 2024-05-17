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
use u__id::*;
use common::*;
pub fn AArch32_MAIRAttr<T: Tracer>(
    state: &mut State,
    tracer: &T,
    index: i128,
    mair: ProductType5c790c8ef59cc8b2,
) -> u8 {
    #[derive(Default)]
    struct FunctionState {
        bit_index: i128,
        gs_27881: bool,
        index: i128,
        mair: ProductType5c790c8ef59cc8b2,
    }
    let fn_state = FunctionState {
        index,
        mair,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_0_0: const #8s : i
        let s_0_0: i128 = 8;
        // D s_0_1: read-var index:i
        let s_0_1: i128 = fn_state.index;
        // D s_0_2: cmp-lt s_0_1 s_0_0
        let s_0_2: bool = ((s_0_1) < (s_0_0));
        // N s_0_3: assert s_0_2
        let s_0_3: () = assert!(s_0_2);
        // C s_0_4: const #8s : i
        let s_0_4: i128 = 8;
        // D s_0_5: read-var index:i
        let s_0_5: i128 = fn_state.index;
        // D s_0_6: mul s_0_4 s_0_5
        let s_0_6: i128 = ((s_0_4) * (s_0_5));
        // D s_0_7: write-var bit_index <= s_0_6
        fn_state.bit_index = s_0_6;
        // D s_0_8: read-var bit_index:i
        let s_0_8: i128 = fn_state.bit_index;
        // D s_0_9: call __id(s_0_8)
        let s_0_9: i128 = u__id(state, tracer, s_0_8);
        // C s_0_10: const #0s : i
        let s_0_10: i128 = 0;
        // D s_0_11: cmp-le s_0_10 s_0_9
        let s_0_11: bool = ((s_0_10) <= (s_0_9));
        // N s_0_12: branch s_0_11 b3 b1
        if s_0_11 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_1_0: read-var index:i
        let s_1_0: i128 = fn_state.index;
        // D s_1_1: call __id(s_1_0)
        let s_1_1: i128 = u__id(state, tracer, s_1_0);
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: cmp-lt s_1_1 s_1_2
        let s_1_3: bool = ((s_1_1) < (s_1_2));
        // D s_1_4: not s_1_3
        let s_1_4: bool = !s_1_3;
        // D s_1_5: write-var gs#27881 <= s_1_4
        fn_state.gs_27881 = s_1_4;
        // N s_1_6: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_2_0: read-var gs#27881:u8
        let s_2_0: bool = fn_state.gs_27881;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // D s_2_2: read-var mair.0:struct
        let s_2_2: u64 = fn_state.mair._0;
        // C s_2_3: const #8s : i
        let s_2_3: i128 = 8;
        // D s_2_4: cast zx s_2_2 -> bv
        let s_2_4: Bits = Bits::new(s_2_2 as u128, 64u16);
        // D s_2_5: read-var bit_index:i
        let s_2_5: i128 = fn_state.bit_index;
        // D s_2_6: bit-extract s_2_4 s_2_5 s_2_3
        let s_2_6: Bits = (Bits::new(
            ((s_2_4) >> (s_2_5)).value(),
            u16::try_from(s_2_3).unwrap(),
        ));
        // D s_2_7: cast reint s_2_6 -> u8
        let s_2_7: u8 = (s_2_6.value() as u8);
        // N s_2_8: return s_2_7
        return s_2_7;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_3_0: const #1u : u8
        let s_3_0: bool = true;
        // D s_3_1: write-var gs#27881 <= s_3_0
        fn_state.gs_27881 = s_3_0;
        // N s_3_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
