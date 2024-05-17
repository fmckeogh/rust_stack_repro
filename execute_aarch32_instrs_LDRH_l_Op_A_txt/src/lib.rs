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
use neq_int::*;
use AArch32_SetLSInstructionSyndrome::*;
use MemU_read::*;
use R_set::*;
use PC_read__1::*;
use Align_bits::*;
use common::*;
pub fn execute_aarch32_instrs_LDRH_l_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    add: bool,
    imm32: u32,
    t: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_880134: Bits,
        base: u32,
        address: u32,
        add: bool,
        imm32: u32,
        t: i64,
    }
    let fn_state = FunctionState {
        add,
        imm32,
        t,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call PC_read__1(s_0_0)
        let s_0_1: u32 = PC_read__1(state, tracer, s_0_0);
        // C s_0_2: const #4s : i
        let s_0_2: i128 = 4;
        // S s_0_3: cast zx s_0_1 -> bv
        let s_0_3: Bits = Bits::new(s_0_1 as u128, 32u16);
        // S s_0_4: call Align_bits(s_0_3, s_0_2)
        let s_0_4: Bits = Align_bits(state, tracer, s_0_3, s_0_2);
        // S s_0_5: cast reint s_0_4 -> u32
        let s_0_5: u32 = (s_0_4.value() as u32);
        // D s_0_6: write-var base <= s_0_5
        fn_state.base = s_0_5;
        // D s_0_7: read-var add:u8
        let s_0_7: bool = fn_state.add;
        // N s_0_8: branch s_0_7 b7 b1
        if s_0_7 {
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
        // D s_1_0: read-var base:u32
        let s_1_0: u32 = fn_state.base;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 32u16);
        // D s_1_2: read-var imm32:u32
        let s_1_2: u32 = fn_state.imm32;
        // D s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 32u16);
        // D s_1_4: sub s_1_1 s_1_3
        let s_1_4: Bits = ((s_1_1) - (s_1_3));
        // D s_1_5: cast reint s_1_4 -> u32
        let s_1_5: u32 = (s_1_4.value() as u32);
        // D s_1_6: write-var address <= s_1_5
        fn_state.address = s_1_5;
        // N s_1_7: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #15s : i
        let s_2_0: i128 = 15;
        // D s_2_1: read-var t:i64
        let s_2_1: i64 = fn_state.t;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: call neq_int(s_2_2, s_2_0)
        let s_2_3: bool = neq_int(state, tracer, s_2_2, s_2_0);
        // N s_2_4: branch s_2_3 b6 b3
        if s_2_3 {
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
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #2s : i64
        let s_4_0: i64 = 2;
        // D s_4_1: read-var address:u32
        let s_4_1: u32 = fn_state.address;
        // D s_4_2: call MemU_read(s_4_1, s_4_0)
        let s_4_2: Bits = MemU_read(state, tracer, s_4_1, s_4_0);
        // D s_4_3: write-var gs#880134 <= s_4_2
        fn_state.gs_880134 = s_4_2;
        // N s_4_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#880134:bv
        let s_5_0: Bits = fn_state.gs_880134;
        // D s_5_1: cast reint s_5_0 -> u16
        let s_5_1: u16 = (s_5_0.value() as u16);
        // C s_5_2: const #32s : i
        let s_5_2: i128 = 32;
        // D s_5_3: cast zx s_5_1 -> bv
        let s_5_3: Bits = Bits::new(s_5_1 as u128, 16u16);
        // D s_5_4: bits-cast zx s_5_3 -> bv length s_5_2
        let s_5_4: Bits = s_5_3.zero_extend(s_5_2);
        // D s_5_5: cast reint s_5_4 -> u32
        let s_5_5: u32 = (s_5_4.value() as u32);
        // D s_5_6: read-var t:i64
        let s_5_6: i64 = fn_state.t;
        // D s_5_7: cast zx s_5_6 -> i
        let s_5_7: i128 = (i128::try_from(s_5_6).unwrap());
        // D s_5_8: call R_set(s_5_7, s_5_5)
        let s_5_8: () = R_set(state, tracer, s_5_7, s_5_5);
        // N s_5_9: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #2s : i
        let s_6_0: i128 = 2;
        // D s_6_1: read-var t:i64
        let s_6_1: i64 = fn_state.t;
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // C s_6_3: const #0u : u8
        let s_6_3: bool = false;
        // C s_6_4: const #0u : u8
        let s_6_4: bool = false;
        // D s_6_5: call AArch32_SetLSInstructionSyndrome(s_6_0, s_6_3, s_6_2, s_6_4)
        let s_6_5: () = AArch32_SetLSInstructionSyndrome(
            state,
            tracer,
            s_6_0,
            s_6_3,
            s_6_2,
            s_6_4,
        );
        // N s_6_6: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var base:u32
        let s_7_0: u32 = fn_state.base;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 32u16);
        // D s_7_2: read-var imm32:u32
        let s_7_2: u32 = fn_state.imm32;
        // D s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 32u16);
        // D s_7_4: add s_7_1 s_7_3
        let s_7_4: Bits = (s_7_1 + s_7_3);
        // D s_7_5: cast reint s_7_4 -> u32
        let s_7_5: u32 = (s_7_4.value() as u32);
        // D s_7_6: write-var address <= s_7_5
        fn_state.address = s_7_5;
        // N s_7_7: jump b2
        return block_2(state, tracer, fn_state);
    }
}
