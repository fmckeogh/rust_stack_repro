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
use ContiguousSize::*;
use u__IMPDEF_boolean::*;
use TranslationSize::*;
use AArch64_IASize::*;
use common::*;
pub fn AArch64_ContiguousBitFaults<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d128: bool,
    txsz: u8,
    tgx: u32,
    level: i128,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_17540: bool,
        d128: bool,
        txsz: u8,
        tgx: u32,
        level: i128,
    }
    let fn_state = FunctionState {
        d128,
        txsz,
        tgx,
        level,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var txsz:u8
        let s_0_0: u8 = fn_state.txsz;
        // D s_0_1: call AArch64_IASize(s_0_0)
        let s_0_1: i128 = AArch64_IASize(state, tracer, s_0_0);
        // D s_0_2: read-var d128:u8
        let s_0_2: bool = fn_state.d128;
        // D s_0_3: read-var tgx:u32
        let s_0_3: u32 = fn_state.tgx;
        // D s_0_4: read-var level:i
        let s_0_4: i128 = fn_state.level;
        // D s_0_5: call TranslationSize(s_0_2, s_0_3, s_0_4)
        let s_0_5: i128 = TranslationSize(state, tracer, s_0_2, s_0_3, s_0_4);
        // D s_0_6: read-var d128:u8
        let s_0_6: bool = fn_state.d128;
        // D s_0_7: read-var tgx:u32
        let s_0_7: u32 = fn_state.tgx;
        // D s_0_8: read-var level:i
        let s_0_8: i128 = fn_state.level;
        // D s_0_9: call ContiguousSize(s_0_6, s_0_7, s_0_8)
        let s_0_9: i128 = ContiguousSize(state, tracer, s_0_6, s_0_7, s_0_8);
        // D s_0_10: add s_0_5 s_0_9
        let s_0_10: i128 = (s_0_5 + s_0_9);
        // D s_0_11: cmp-gt s_0_10 s_0_1
        let s_0_11: bool = ((s_0_10) > (s_0_1));
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
    ) -> bool {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#17540 <= s_1_0
        fn_state.gs_17540 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var gs#17540:u8
        let s_2_0: bool = fn_state.gs_17540;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_3_0: const #"Translation fault on misprogrammed contiguous bit" : str
        let s_3_0: &'static str = "Translation fault on misprogrammed contiguous bit";
        // S s_3_1: call __IMPDEF_boolean(s_3_0)
        let s_3_1: bool = u__IMPDEF_boolean(state, tracer, s_3_0);
        // D s_3_2: write-var gs#17540 <= s_3_1
        fn_state.gs_17540 = s_3_1;
        // N s_3_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
