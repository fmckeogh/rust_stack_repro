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
use X_set::*;
use GCSPCREnabled::*;
use PC_read::*;
use AddGCSRecord::*;
use BranchTo::*;
use HaveGCS::*;
use common::*;
pub fn execute_aarch64_instrs_branch_unconditional_immediate<T: Tracer>(
    state: &mut State,
    tracer: &T,
    branch_type: u32,
    offset: u64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_143929: bool,
        branch_type: u32,
        offset: u64,
    }
    let fn_state = FunctionState {
        branch_type,
        offset,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var branch_type:u32
        let s_0_0: u32 = fn_state.branch_type;
        // C s_0_1: const #0u : u32
        let s_0_1: u32 = 0;
        // D s_0_2: cmp-eq s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) == (s_0_1));
        // N s_0_3: branch s_0_2 b3 b1
        if s_0_2 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // C s_2_1: const #() : ()
        let s_2_1: () = ();
        // S s_2_2: call PC_read(s_2_1)
        let s_2_2: u64 = PC_read(state, tracer, s_2_1);
        // S s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 64u16);
        // D s_2_4: read-var offset:u64
        let s_2_4: u64 = fn_state.offset;
        // D s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 64u16);
        // D s_2_6: add s_2_3 s_2_5
        let s_2_6: Bits = (s_2_3 + s_2_5);
        // D s_2_7: cast reint s_2_6 -> u64
        let s_2_7: u64 = (s_2_6.value() as u64);
        // D s_2_8: cast zx s_2_7 -> bv
        let s_2_8: Bits = Bits::new(s_2_7 as u128, 64u16);
        // D s_2_9: read-var branch_type:u32
        let s_2_9: u32 = fn_state.branch_type;
        // D s_2_10: call BranchTo(s_2_8, s_2_9, s_2_0)
        let s_2_10: () = BranchTo(state, tracer, s_2_8, s_2_9, s_2_0);
        // N s_2_11: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call HaveGCS(s_3_0)
        let s_3_1: bool = HaveGCS(state, tracer, s_3_0);
        // N s_3_2: branch s_3_1 b9 b4
        if s_3_1 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#143929 <= s_4_0
        fn_state.gs_143929 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#143929:u8
        let s_5_0: bool = fn_state.gs_143929;
        // N s_5_1: branch s_5_0 b8 b6
        if s_5_0 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #64s : i64
        let s_7_0: i64 = 64;
        // C s_7_1: const #() : ()
        let s_7_1: () = ();
        // S s_7_2: call PC_read(s_7_1)
        let s_7_2: u64 = PC_read(state, tracer, s_7_1);
        // C s_7_3: const #4s : i
        let s_7_3: i128 = 4;
        // S s_7_4: cast zx s_7_2 -> bv
        let s_7_4: Bits = Bits::new(s_7_2 as u128, 64u16);
        // C s_7_5: cast cvt s_7_3 -> bv
        let s_7_5: Bits = Bits::new(s_7_3 as u128, 128);
        // S s_7_6: add s_7_4 s_7_5
        let s_7_6: Bits = (s_7_4 + s_7_5);
        // S s_7_7: cast reint s_7_6 -> u64
        let s_7_7: u64 = (s_7_6.value() as u64);
        // C s_7_8: const #30s : i
        let s_7_8: i128 = 30;
        // S s_7_9: cast zx s_7_7 -> bv
        let s_7_9: Bits = Bits::new(s_7_7 as u128, 64u16);
        // S s_7_10: call X_set(s_7_8, s_7_0, s_7_9)
        let s_7_10: () = X_set(state, tracer, s_7_8, s_7_0, s_7_9);
        // N s_7_11: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call PC_read(s_8_0)
        let s_8_1: u64 = PC_read(state, tracer, s_8_0);
        // C s_8_2: const #4s : i
        let s_8_2: i128 = 4;
        // S s_8_3: cast zx s_8_1 -> bv
        let s_8_3: Bits = Bits::new(s_8_1 as u128, 64u16);
        // C s_8_4: cast cvt s_8_2 -> bv
        let s_8_4: Bits = Bits::new(s_8_2 as u128, 128);
        // S s_8_5: add s_8_3 s_8_4
        let s_8_5: Bits = (s_8_3 + s_8_4);
        // S s_8_6: cast reint s_8_5 -> u64
        let s_8_6: u64 = (s_8_5.value() as u64);
        // S s_8_7: call AddGCSRecord(s_8_6)
        let s_8_7: () = AddGCSRecord(state, tracer, s_8_6);
        // N s_8_8: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #16975u : u32
        let s_9_0: u32 = 16975;
        // D s_9_1: read-reg s_9_0:u8
        let s_9_1: u8 = {
            let value = state.read_register::<u8>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // D s_9_2: call GCSPCREnabled(s_9_1)
        let s_9_2: bool = GCSPCREnabled(state, tracer, s_9_1);
        // D s_9_3: write-var gs#143929 <= s_9_2
        fn_state.gs_143929 = s_9_2;
        // N s_9_4: jump b5
        return block_5(state, tracer, fn_state);
    }
}
