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
use AArch64_PhysicalAddressSize::*;
use is_zero_subrange::*;
use common::*;
pub fn AArch64_OAOutOfRange<T: Tracer>(
    state: &mut State,
    tracer: &T,
    address: u64,
    d128: bool,
    ps: u8,
    tgx: u32,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        oasize: i128,
        return_value: bool,
        address: u64,
        d128: bool,
        ps: u8,
        tgx: u32,
    }
    let fn_state = FunctionState {
        address,
        d128,
        ps,
        tgx,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var d128:u8
        let s_0_0: bool = fn_state.d128;
        // D s_0_1: read-var ps:u8
        let s_0_1: u8 = fn_state.ps;
        // D s_0_2: read-var tgx:u32
        let s_0_2: u32 = fn_state.tgx;
        // D s_0_3: call AArch64_PhysicalAddressSize(s_0_0, s_0_1, s_0_2)
        let s_0_3: i128 = AArch64_PhysicalAddressSize(
            state,
            tracer,
            s_0_0,
            s_0_1,
            s_0_2,
        );
        // D s_0_4: write-var oasize <= s_0_3
        fn_state.oasize = s_0_3;
        // C s_0_5: const #56s : i
        let s_0_5: i128 = 56;
        // D s_0_6: read-var oasize:i
        let s_0_6: i128 = fn_state.oasize;
        // D s_0_7: cmp-lt s_0_6 s_0_5
        let s_0_7: bool = ((s_0_6) < (s_0_5));
        // N s_0_8: branch s_0_7 b3 b1
        if s_0_7 {
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
        // D s_1_1: write-var return_value <= s_1_0
        fn_state.return_value = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var return_value:u8
        let s_2_0: bool = fn_state.return_value;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_3_0: const #55s : i
        let s_3_0: i128 = 55;
        // D s_3_1: read-var address:u56
        let s_3_1: u64 = fn_state.address;
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 56u16);
        // D s_3_3: read-var oasize:i
        let s_3_3: i128 = fn_state.oasize;
        // D s_3_4: call is_zero_subrange(s_3_2, s_3_0, s_3_3)
        let s_3_4: bool = is_zero_subrange(state, tracer, s_3_2, s_3_0, s_3_3);
        // D s_3_5: not s_3_4
        let s_3_5: bool = !s_3_4;
        // D s_3_6: write-var return_value <= s_3_5
        fn_state.return_value = s_3_5;
        // N s_3_7: jump b2
        return block_2(state, tracer, fn_state);
    }
}
