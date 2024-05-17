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
use DBGDTRTXint_LDC_da4baa792b999a1f::*;
use common::*;
pub fn AArch32_LDC<T: Tracer>(
    state: &mut State,
    tracer: &T,
    coproc: u8,
    CRd: u8,
    address: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_137037: bool,
        coproc: u8,
        CRd: u8,
        address: u32,
    }
    let fn_state = FunctionState {
        coproc,
        CRd,
        address,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var CRd:u8
        let s_0_0: u8 = fn_state.CRd;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 4u16);
        // C s_0_2: const #5u : u8
        let s_0_2: u8 = 5;
        // C s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 4u16);
        // D s_0_4: cmp-eq s_0_1 s_0_3
        let s_0_4: bool = ((s_0_1) == (s_0_3));
        // N s_0_5: branch s_0_4 b5 b1
        if s_0_4 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#137037 <= s_1_0
        fn_state.gs_137037 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#137037:u8
        let s_2_0: bool = fn_state.gs_137037;
        // N s_2_1: branch s_2_0 b4 b3
        if s_2_0 {
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
        // N s_3_0: panic
        panic!("{:?}", ());
        // N s_3_1: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var coproc:u8
        let s_4_0: u8 = fn_state.coproc;
        // D s_4_1: read-var CRd:u8
        let s_4_1: u8 = fn_state.CRd;
        // D s_4_2: read-var address:u32
        let s_4_2: u32 = fn_state.address;
        // D s_4_3: call DBGDTRTXint_LDC_da4baa792b999a1f(s_4_0, s_4_1, s_4_2)
        let s_4_3: () = DBGDTRTXint_LDC_da4baa792b999a1f(
            state,
            tracer,
            s_4_0,
            s_4_1,
            s_4_2,
        );
        // N s_4_4: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var coproc:u8
        let s_5_0: u8 = fn_state.coproc;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 4u16);
        // C s_5_2: const #14u : u8
        let s_5_2: u8 = 14;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 4u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // D s_5_5: write-var gs#137037 <= s_5_4
        fn_state.gs_137037 = s_5_4;
        // N s_5_6: jump b2
        return block_2(state, tracer, fn_state);
    }
}
