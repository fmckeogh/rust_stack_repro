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
use AArch32_SysRegWriteM::*;
use ThisInstr::*;
use PC_read__1::*;
use Align_bits::*;
use common::*;
pub fn execute_aarch32_instrs_LDC_l_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    add: bool,
    cp: i64,
    imm32: u32,
    index: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        offset_addr: u32,
        address: u32,
        add: bool,
        cp: i64,
        imm32: u32,
        index: bool,
    }
    let fn_state = FunctionState {
        add,
        cp,
        imm32,
        index,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var add:u8
        let s_0_0: bool = fn_state.add;
        // N s_0_1: branch s_0_0 b6 b1
        if s_0_0 {
            return block_6(state, tracer, fn_state);
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
        // S s_1_1: call PC_read__1(s_1_0)
        let s_1_1: u32 = PC_read__1(state, tracer, s_1_0);
        // C s_1_2: const #4s : i
        let s_1_2: i128 = 4;
        // S s_1_3: cast zx s_1_1 -> bv
        let s_1_3: Bits = Bits::new(s_1_1 as u128, 32u16);
        // S s_1_4: call Align_bits(s_1_3, s_1_2)
        let s_1_4: Bits = Align_bits(state, tracer, s_1_3, s_1_2);
        // S s_1_5: cast reint s_1_4 -> u32
        let s_1_5: u32 = (s_1_4.value() as u32);
        // S s_1_6: cast zx s_1_5 -> bv
        let s_1_6: Bits = Bits::new(s_1_5 as u128, 32u16);
        // D s_1_7: read-var imm32:u32
        let s_1_7: u32 = fn_state.imm32;
        // D s_1_8: cast zx s_1_7 -> bv
        let s_1_8: Bits = Bits::new(s_1_7 as u128, 32u16);
        // D s_1_9: sub s_1_6 s_1_8
        let s_1_9: Bits = ((s_1_6) - (s_1_8));
        // D s_1_10: cast reint s_1_9 -> u32
        let s_1_10: u32 = (s_1_9.value() as u32);
        // D s_1_11: write-var offset_addr <= s_1_10
        fn_state.offset_addr = s_1_10;
        // N s_1_12: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var index:u8
        let s_2_0: bool = fn_state.index;
        // N s_2_1: branch s_2_0 b5 b3
        if s_2_0 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call PC_read__1(s_3_0)
        let s_3_1: u32 = PC_read__1(state, tracer, s_3_0);
        // C s_3_2: const #4s : i
        let s_3_2: i128 = 4;
        // S s_3_3: cast zx s_3_1 -> bv
        let s_3_3: Bits = Bits::new(s_3_1 as u128, 32u16);
        // S s_3_4: call Align_bits(s_3_3, s_3_2)
        let s_3_4: Bits = Align_bits(state, tracer, s_3_3, s_3_2);
        // S s_3_5: cast reint s_3_4 -> u32
        let s_3_5: u32 = (s_3_4.value() as u32);
        // D s_3_6: write-var address <= s_3_5
        fn_state.address = s_3_5;
        // N s_3_7: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #() : ()
        let s_4_0: () = ();
        // S s_4_1: call ThisInstr(s_4_0)
        let s_4_1: u32 = ThisInstr(state, tracer, s_4_0);
        // D s_4_2: read-var cp:i64
        let s_4_2: i64 = fn_state.cp;
        // D s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_4: read-var address:u32
        let s_4_4: u32 = fn_state.address;
        // D s_4_5: call AArch32_SysRegWriteM(s_4_3, s_4_1, s_4_4)
        let s_4_5: () = AArch32_SysRegWriteM(state, tracer, s_4_3, s_4_1, s_4_4);
        // N s_4_6: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var offset_addr:u32
        let s_5_0: u32 = fn_state.offset_addr;
        // D s_5_1: write-var address <= s_5_0
        fn_state.address = s_5_0;
        // N s_5_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call PC_read__1(s_6_0)
        let s_6_1: u32 = PC_read__1(state, tracer, s_6_0);
        // C s_6_2: const #4s : i
        let s_6_2: i128 = 4;
        // S s_6_3: cast zx s_6_1 -> bv
        let s_6_3: Bits = Bits::new(s_6_1 as u128, 32u16);
        // S s_6_4: call Align_bits(s_6_3, s_6_2)
        let s_6_4: Bits = Align_bits(state, tracer, s_6_3, s_6_2);
        // S s_6_5: cast reint s_6_4 -> u32
        let s_6_5: u32 = (s_6_4.value() as u32);
        // S s_6_6: cast zx s_6_5 -> bv
        let s_6_6: Bits = Bits::new(s_6_5 as u128, 32u16);
        // D s_6_7: read-var imm32:u32
        let s_6_7: u32 = fn_state.imm32;
        // D s_6_8: cast zx s_6_7 -> bv
        let s_6_8: Bits = Bits::new(s_6_7 as u128, 32u16);
        // D s_6_9: add s_6_6 s_6_8
        let s_6_9: Bits = (s_6_6 + s_6_8);
        // D s_6_10: cast reint s_6_9 -> u32
        let s_6_10: u32 = (s_6_9.value() as u32);
        // D s_6_11: write-var offset_addr <= s_6_10
        fn_state.offset_addr = s_6_10;
        // N s_6_12: jump b2
        return block_2(state, tracer, fn_state);
    }
}
