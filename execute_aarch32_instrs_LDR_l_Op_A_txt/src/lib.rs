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
use LoadWritePC::*;
use MemU_read::*;
use R_set::*;
use PC_read__1::*;
use Align_bits::*;
use common::*;
pub fn execute_aarch32_instrs_LDR_l_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    add: bool,
    imm32: u32,
    t: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_880416: Bits,
        base: u32,
        data: u32,
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
        // N s_0_8: branch s_0_7 b11 b1
        if s_0_7 {
            return block_11(state, tracer, fn_state);
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
        // N s_2_4: branch s_2_3 b10 b3
        if s_2_3 {
            return block_10(state, tracer, fn_state);
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
        // C s_4_0: const #4s : i64
        let s_4_0: i64 = 4;
        // D s_4_1: read-var address:u32
        let s_4_1: u32 = fn_state.address;
        // D s_4_2: call MemU_read(s_4_1, s_4_0)
        let s_4_2: Bits = MemU_read(state, tracer, s_4_1, s_4_0);
        // D s_4_3: write-var gs#880416 <= s_4_2
        fn_state.gs_880416 = s_4_2;
        // N s_4_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#880416:bv
        let s_5_0: Bits = fn_state.gs_880416;
        // D s_5_1: cast reint s_5_0 -> u32
        let s_5_1: u32 = (s_5_0.value() as u32);
        // D s_5_2: write-var data <= s_5_1
        fn_state.data = s_5_1;
        // C s_5_3: const #15s : i
        let s_5_3: i128 = 15;
        // D s_5_4: read-var t:i64
        let s_5_4: i64 = fn_state.t;
        // D s_5_5: cast zx s_5_4 -> i
        let s_5_5: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_6: cmp-eq s_5_5 s_5_3
        let s_5_6: bool = ((s_5_5) == (s_5_3));
        // N s_5_7: branch s_5_6 b7 b6
        if s_5_6 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var t:i64
        let s_6_0: i64 = fn_state.t;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: read-var data:u32
        let s_6_2: u32 = fn_state.data;
        // D s_6_3: call R_set(s_6_1, s_6_2)
        let s_6_3: () = R_set(state, tracer, s_6_1, s_6_2);
        // N s_6_4: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0s : i
        let s_7_0: i128 = 0;
        // D s_7_1: read-var address:u32
        let s_7_1: u32 = fn_state.address;
        // D s_7_2: cast zx s_7_1 -> bv
        let s_7_2: Bits = Bits::new(s_7_1 as u128, 32u16);
        // C s_7_3: const #1s : i64
        let s_7_3: i64 = 1;
        // C s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // C s_7_5: const #1s : i
        let s_7_5: i128 = 1;
        // C s_7_6: add s_7_5 s_7_4
        let s_7_6: i128 = (s_7_5 + s_7_4);
        // D s_7_7: bit-extract s_7_2 s_7_0 s_7_6
        let s_7_7: Bits = (Bits::new(
            ((s_7_2) >> (s_7_0)).value(),
            u16::try_from(s_7_6).unwrap(),
        ));
        // D s_7_8: cast reint s_7_7 -> u8
        let s_7_8: u8 = (s_7_7.value() as u8);
        // D s_7_9: cast zx s_7_8 -> bv
        let s_7_9: Bits = Bits::new(s_7_8 as u128, 2u16);
        // C s_7_10: const #0u : u8
        let s_7_10: u8 = 0;
        // C s_7_11: cast zx s_7_10 -> bv
        let s_7_11: Bits = Bits::new(s_7_10 as u128, 2u16);
        // D s_7_12: cmp-eq s_7_9 s_7_11
        let s_7_12: bool = ((s_7_9) == (s_7_11));
        // N s_7_13: branch s_7_12 b9 b8
        if s_7_12 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: panic
        panic!("{:?}", ());
        // N s_8_1: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var data:u32
        let s_9_0: u32 = fn_state.data;
        // D s_9_1: call LoadWritePC(s_9_0)
        let s_9_1: () = LoadWritePC(state, tracer, s_9_0);
        // N s_9_2: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #4s : i
        let s_10_0: i128 = 4;
        // D s_10_1: read-var t:i64
        let s_10_1: i64 = fn_state.t;
        // D s_10_2: cast zx s_10_1 -> i
        let s_10_2: i128 = (i128::try_from(s_10_1).unwrap());
        // C s_10_3: const #0u : u8
        let s_10_3: bool = false;
        // C s_10_4: const #0u : u8
        let s_10_4: bool = false;
        // D s_10_5: call AArch32_SetLSInstructionSyndrome(s_10_0, s_10_3, s_10_2, s_10_4)
        let s_10_5: () = AArch32_SetLSInstructionSyndrome(
            state,
            tracer,
            s_10_0,
            s_10_3,
            s_10_2,
            s_10_4,
        );
        // N s_10_6: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var base:u32
        let s_11_0: u32 = fn_state.base;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 32u16);
        // D s_11_2: read-var imm32:u32
        let s_11_2: u32 = fn_state.imm32;
        // D s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 32u16);
        // D s_11_4: add s_11_1 s_11_3
        let s_11_4: Bits = (s_11_1 + s_11_3);
        // D s_11_5: cast reint s_11_4 -> u32
        let s_11_5: u32 = (s_11_4.value() as u32);
        // D s_11_6: write-var address <= s_11_5
        fn_state.address = s_11_5;
        // N s_11_7: jump b2
        return block_2(state, tracer, fn_state);
    }
}
