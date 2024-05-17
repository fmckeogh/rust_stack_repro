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
use HaltingAllowed::*;
use execute_aarch32_instrs_HLT_Op_A_txt::*;
use u_get_EDSCR_Type_HDE::*;
use EDSCR_read::*;
use common::*;
pub fn decode_aarch32_instrs_HLT_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cond: u8,
    imm12: u16,
    imm4: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_324060: bool,
        cond: u8,
        imm12: u16,
        imm4: u8,
    }
    let fn_state = FunctionState {
        cond,
        imm12,
        imm4,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var cond:u8
        let s_0_0: u8 = fn_state.cond;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 4u16);
        // C s_0_2: const #15u : u8
        let s_0_2: u8 = 15;
        // C s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 4u16);
        // D s_0_4: cmp-ne s_0_1 s_0_3
        let s_0_4: bool = ((s_0_1) != (s_0_3));
        // N s_0_5: assert s_0_4
        let s_0_5: () = assert!(s_0_4);
        // C s_0_6: const #() : ()
        let s_0_6: () = ();
        // S s_0_7: call EDSCR_read(s_0_6)
        let s_0_7: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_0_6);
        // S s_0_8: call _get_EDSCR_Type_HDE(s_0_7)
        let s_0_8: bool = u_get_EDSCR_Type_HDE(state, tracer, s_0_7);
        // S s_0_9: cast zx s_0_8 -> bv
        let s_0_9: Bits = Bits::new(s_0_8 as u128, 1u16);
        // C s_0_10: const #0u : u8
        let s_0_10: bool = false;
        // C s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 1u16);
        // S s_0_12: cmp-eq s_0_9 s_0_11
        let s_0_12: bool = ((s_0_9) == (s_0_11));
        // N s_0_13: branch s_0_12 b7 b1
        if s_0_12 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call HaltingAllowed(s_1_0)
        let s_1_1: bool = HaltingAllowed(state, tracer, s_1_0);
        // S s_1_2: not s_1_1
        let s_1_2: bool = !s_1_1;
        // D s_1_3: write-var gs#324060 <= s_1_2
        fn_state.gs_324060 = s_1_2;
        // N s_1_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#324060:u8
        let s_2_0: bool = fn_state.gs_324060;
        // N s_2_1: branch s_2_0 b6 b3
        if s_2_0 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var cond:u8
        let s_3_0: u8 = fn_state.cond;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 4u16);
        // C s_3_2: const #14u : u8
        let s_3_2: u8 = 14;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 4u16);
        // D s_3_4: cmp-ne s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) != (s_3_3));
        // N s_3_5: branch s_3_4 b5 b4
        if s_3_4 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #() : ()
        let s_4_0: () = ();
        // S s_4_1: call execute_aarch32_instrs_HLT_Op_A_txt(s_4_0)
        let s_4_1: () = execute_aarch32_instrs_HLT_Op_A_txt(state, tracer, s_4_0);
        // N s_4_2: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: panic
        panic!("{:?}", ());
        // N s_5_1: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: panic
        panic!("{:?}", ());
        // N s_6_1: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #1u : u8
        let s_7_0: bool = true;
        // D s_7_1: write-var gs#324060 <= s_7_0
        fn_state.gs_324060 = s_7_0;
        // N s_7_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
