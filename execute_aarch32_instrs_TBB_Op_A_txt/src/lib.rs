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
use R_read::*;
use MemU_read::*;
use PC_read__1::*;
use BranchWritePC::*;
use common::*;
pub fn execute_aarch32_instrs_TBB_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    is_tbh: bool,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_887261: Bits,
        halfwords: i64,
        gs_887250: Bits,
        is_tbh: bool,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        is_tbh,
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
        // D s_0_0: read-var is_tbh:u8
        let s_0_0: bool = fn_state.is_tbh;
        // N s_0_1: branch s_0_0 b4 b1
        if s_0_0 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var n:i64
        let s_1_0: i64 = fn_state.n;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: call R_read(s_1_1)
        let s_1_2: u32 = R_read(state, tracer, s_1_1);
        // D s_1_3: read-var m:i64
        let s_1_3: i64 = fn_state.m;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: call R_read(s_1_4)
        let s_1_5: u32 = R_read(state, tracer, s_1_4);
        // D s_1_6: cast zx s_1_2 -> bv
        let s_1_6: Bits = Bits::new(s_1_2 as u128, 32u16);
        // D s_1_7: cast zx s_1_5 -> bv
        let s_1_7: Bits = Bits::new(s_1_5 as u128, 32u16);
        // D s_1_8: add s_1_6 s_1_7
        let s_1_8: Bits = (s_1_6 + s_1_7);
        // D s_1_9: cast reint s_1_8 -> u32
        let s_1_9: u32 = (s_1_8.value() as u32);
        // C s_1_10: const #1s : i64
        let s_1_10: i64 = 1;
        // D s_1_11: call MemU_read(s_1_9, s_1_10)
        let s_1_11: Bits = MemU_read(state, tracer, s_1_9, s_1_10);
        // D s_1_12: write-var gs#887250 <= s_1_11
        fn_state.gs_887250 = s_1_11;
        // N s_1_13: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#887250:bv
        let s_2_0: Bits = fn_state.gs_887250;
        // D s_2_1: cast reint s_2_0 -> u8
        let s_2_1: u8 = (s_2_0.value() as u8);
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 8u16);
        // D s_2_3: cast zx s_2_2 -> i
        let s_2_3: i128 = (s_2_2.value() as i128);
        // D s_2_4: cast reint s_2_3 -> i64
        let s_2_4: i64 = (s_2_3 as i64);
        // D s_2_5: write-var halfwords <= s_2_4
        fn_state.halfwords = s_2_4;
        // N s_2_6: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var halfwords:i64
        let s_3_0: i64 = fn_state.halfwords;
        // C s_3_1: const #() : ()
        let s_3_1: () = ();
        // S s_3_2: call PC_read__1(s_3_1)
        let s_3_2: u32 = PC_read__1(state, tracer, s_3_1);
        // C s_3_3: const #2s : i
        let s_3_3: i128 = 2;
        // D s_3_4: cast zx s_3_0 -> i
        let s_3_4: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_5: mul s_3_3 s_3_4
        let s_3_5: i128 = ((s_3_3) * (s_3_4));
        // D s_3_6: cast reint s_3_5 -> i64
        let s_3_6: i64 = (s_3_5 as i64);
        // S s_3_7: cast zx s_3_2 -> bv
        let s_3_7: Bits = Bits::new(s_3_2 as u128, 32u16);
        // D s_3_8: cast zx s_3_6 -> i
        let s_3_8: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_9: cast cvt s_3_8 -> bv
        let s_3_9: Bits = Bits::new(s_3_8 as u128, 128);
        // D s_3_10: add s_3_7 s_3_9
        let s_3_10: Bits = (s_3_7 + s_3_9);
        // D s_3_11: cast reint s_3_10 -> u32
        let s_3_11: u32 = (s_3_10.value() as u32);
        // C s_3_12: const #6u : u32
        let s_3_12: u32 = 6;
        // D s_3_13: call BranchWritePC(s_3_11, s_3_12)
        let s_3_13: () = BranchWritePC(state, tracer, s_3_11, s_3_12);
        // N s_3_14: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var n:i64
        let s_4_0: i64 = fn_state.n;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: call R_read(s_4_1)
        let s_4_2: u32 = R_read(state, tracer, s_4_1);
        // D s_4_3: read-var m:i64
        let s_4_3: i64 = fn_state.m;
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_5: call R_read(s_4_4)
        let s_4_5: u32 = R_read(state, tracer, s_4_4);
        // C s_4_6: const #1s : i
        let s_4_6: i128 = 1;
        // D s_4_7: cast zx s_4_5 -> bv
        let s_4_7: Bits = Bits::new(s_4_5 as u128, 32u16);
        // D s_4_8: lsl s_4_7 s_4_6
        let s_4_8: Bits = s_4_7 << s_4_6;
        // D s_4_9: cast reint s_4_8 -> u32
        let s_4_9: u32 = (s_4_8.value() as u32);
        // D s_4_10: cast zx s_4_2 -> bv
        let s_4_10: Bits = Bits::new(s_4_2 as u128, 32u16);
        // D s_4_11: cast zx s_4_9 -> bv
        let s_4_11: Bits = Bits::new(s_4_9 as u128, 32u16);
        // D s_4_12: add s_4_10 s_4_11
        let s_4_12: Bits = (s_4_10 + s_4_11);
        // D s_4_13: cast reint s_4_12 -> u32
        let s_4_13: u32 = (s_4_12.value() as u32);
        // C s_4_14: const #2s : i64
        let s_4_14: i64 = 2;
        // D s_4_15: call MemU_read(s_4_13, s_4_14)
        let s_4_15: Bits = MemU_read(state, tracer, s_4_13, s_4_14);
        // D s_4_16: write-var gs#887261 <= s_4_15
        fn_state.gs_887261 = s_4_15;
        // N s_4_17: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#887261:bv
        let s_5_0: Bits = fn_state.gs_887261;
        // D s_5_1: cast reint s_5_0 -> u16
        let s_5_1: u16 = (s_5_0.value() as u16);
        // D s_5_2: cast zx s_5_1 -> bv
        let s_5_2: Bits = Bits::new(s_5_1 as u128, 16u16);
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (s_5_2.value() as i128);
        // D s_5_4: cast reint s_5_3 -> i64
        let s_5_4: i64 = (s_5_3 as i64);
        // D s_5_5: write-var halfwords <= s_5_4
        fn_state.halfwords = s_5_4;
        // N s_5_6: jump b3
        return block_3(state, tracer, fn_state);
    }
}
