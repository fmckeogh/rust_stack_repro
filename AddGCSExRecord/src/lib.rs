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
use CreateAccDescGCS::*;
use SetCurrentGCSPointer::*;
use GetCurrentGCSPointer::*;
use Zeros::*;
use Mem_set::*;
use common::*;
pub fn AddGCSExRecord<T: Tracer>(
    state: &mut State,
    tracer: &T,
    elr: u64,
    spsr: u64,
    lr: u64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        accdesc: ProductType9878976b5bcce9c9,
        ptr: u64,
        elr: u64,
        spsr: u64,
        lr: u64,
    }
    let fn_state = FunctionState {
        elr,
        spsr,
        lr,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #16975u : u32
        let s_0_0: u32 = 16975;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // C s_0_2: const #1u : u32
        let s_0_2: u32 = 1;
        // D s_0_3: call CreateAccDescGCS(s_0_1, s_0_2)
        let s_0_3: ProductType9878976b5bcce9c9 = CreateAccDescGCS(
            state,
            tracer,
            s_0_1,
            s_0_2,
        );
        // D s_0_4: write-var accdesc <= s_0_3
        fn_state.accdesc = s_0_3;
        // C s_0_5: const #() : ()
        let s_0_5: () = ();
        // S s_0_6: call GetCurrentGCSPointer(s_0_5)
        let s_0_6: u64 = GetCurrentGCSPointer(state, tracer, s_0_5);
        // D s_0_7: write-var ptr <= s_0_6
        fn_state.ptr = s_0_6;
        // C s_0_8: const #8s : i
        let s_0_8: i128 = 8;
        // D s_0_9: read-var ptr:u64
        let s_0_9: u64 = fn_state.ptr;
        // D s_0_10: cast zx s_0_9 -> bv
        let s_0_10: Bits = Bits::new(s_0_9 as u128, 64u16);
        // C s_0_11: cast cvt s_0_8 -> bv
        let s_0_11: Bits = Bits::new(s_0_8 as u128, 128);
        // D s_0_12: sub s_0_10 s_0_11
        let s_0_12: Bits = ((s_0_10) - (s_0_11));
        // D s_0_13: cast reint s_0_12 -> u64
        let s_0_13: u64 = (s_0_12.value() as u64);
        // C s_0_14: const #8s : i
        let s_0_14: i128 = 8;
        // D s_0_15: read-var lr:u64
        let s_0_15: u64 = fn_state.lr;
        // D s_0_16: cast zx s_0_15 -> bv
        let s_0_16: Bits = Bits::new(s_0_15 as u128, 64u16);
        // D s_0_17: read-var accdesc:struct
        let s_0_17: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_0_18: call Mem_set(s_0_13, s_0_14, s_0_17, s_0_16)
        let s_0_18: () = Mem_set(state, tracer, s_0_13, s_0_14, s_0_17, s_0_16);
        // N s_0_19: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #16s : i
        let s_1_0: i128 = 16;
        // D s_1_1: read-var ptr:u64
        let s_1_1: u64 = fn_state.ptr;
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 64u16);
        // C s_1_3: cast cvt s_1_0 -> bv
        let s_1_3: Bits = Bits::new(s_1_0 as u128, 128);
        // D s_1_4: sub s_1_2 s_1_3
        let s_1_4: Bits = ((s_1_2) - (s_1_3));
        // D s_1_5: cast reint s_1_4 -> u64
        let s_1_5: u64 = (s_1_4.value() as u64);
        // C s_1_6: const #8s : i
        let s_1_6: i128 = 8;
        // D s_1_7: read-var spsr:u64
        let s_1_7: u64 = fn_state.spsr;
        // D s_1_8: cast zx s_1_7 -> bv
        let s_1_8: Bits = Bits::new(s_1_7 as u128, 64u16);
        // D s_1_9: read-var accdesc:struct
        let s_1_9: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_1_10: call Mem_set(s_1_5, s_1_6, s_1_9, s_1_8)
        let s_1_10: () = Mem_set(state, tracer, s_1_5, s_1_6, s_1_9, s_1_8);
        // N s_1_11: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #24s : i
        let s_2_0: i128 = 24;
        // D s_2_1: read-var ptr:u64
        let s_2_1: u64 = fn_state.ptr;
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 64u16);
        // C s_2_3: cast cvt s_2_0 -> bv
        let s_2_3: Bits = Bits::new(s_2_0 as u128, 128);
        // D s_2_4: sub s_2_2 s_2_3
        let s_2_4: Bits = ((s_2_2) - (s_2_3));
        // D s_2_5: cast reint s_2_4 -> u64
        let s_2_5: u64 = (s_2_4.value() as u64);
        // C s_2_6: const #8s : i
        let s_2_6: i128 = 8;
        // D s_2_7: read-var elr:u64
        let s_2_7: u64 = fn_state.elr;
        // D s_2_8: cast zx s_2_7 -> bv
        let s_2_8: Bits = Bits::new(s_2_7 as u128, 64u16);
        // D s_2_9: read-var accdesc:struct
        let s_2_9: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_2_10: call Mem_set(s_2_5, s_2_6, s_2_9, s_2_8)
        let s_2_10: () = Mem_set(state, tracer, s_2_5, s_2_6, s_2_9, s_2_8);
        // N s_2_11: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #32s : i
        let s_3_0: i128 = 32;
        // D s_3_1: read-var ptr:u64
        let s_3_1: u64 = fn_state.ptr;
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 64u16);
        // C s_3_3: cast cvt s_3_0 -> bv
        let s_3_3: Bits = Bits::new(s_3_0 as u128, 128);
        // D s_3_4: sub s_3_2 s_3_3
        let s_3_4: Bits = ((s_3_2) - (s_3_3));
        // D s_3_5: cast reint s_3_4 -> u64
        let s_3_5: u64 = (s_3_4.value() as u64);
        // C s_3_6: const #60s : i
        let s_3_6: i128 = 60;
        // S s_3_7: call Zeros(s_3_6)
        let s_3_7: Bits = Zeros(state, tracer, s_3_6);
        // S s_3_8: cast reint s_3_7 -> u60
        let s_3_8: u64 = (s_3_7.value() as u64);
        // S s_3_9: cast zx s_3_8 -> bv
        let s_3_9: Bits = Bits::new(s_3_8 as u128, 60u16);
        // C s_3_10: const #9u : u8
        let s_3_10: u8 = 9;
        // C s_3_11: cast zx s_3_10 -> bv
        let s_3_11: Bits = Bits::new(s_3_10 as u128, 4u16);
        // S s_3_12: cast reint s_3_9 -> u128
        let s_3_12: u128 = (s_3_9.value() as u128);
        // D s_3_13: size-of s_3_9
        let s_3_13: u16 = s_3_9.length();
        // C s_3_14: cast reint s_3_11 -> u128
        let s_3_14: u128 = (s_3_11.value() as u128);
        // D s_3_15: size-of s_3_11
        let s_3_15: u16 = s_3_11.length();
        // D s_3_16: lsl s_3_12 s_3_15
        let s_3_16: u128 = s_3_12 << s_3_15;
        // D s_3_17: or s_3_16 s_3_14
        let s_3_17: u128 = ((s_3_16) | (s_3_14));
        // D s_3_18: add s_3_13 s_3_15
        let s_3_18: u16 = (s_3_13 + s_3_15);
        // D s_3_19: create-bits s_3_17 s_3_18
        let s_3_19: Bits = Bits::new(s_3_17, s_3_18);
        // D s_3_20: cast reint s_3_19 -> u64
        let s_3_20: u64 = (s_3_19.value() as u64);
        // C s_3_21: const #8s : i
        let s_3_21: i128 = 8;
        // D s_3_22: cast zx s_3_20 -> bv
        let s_3_22: Bits = Bits::new(s_3_20 as u128, 64u16);
        // D s_3_23: read-var accdesc:struct
        let s_3_23: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_3_24: call Mem_set(s_3_5, s_3_21, s_3_23, s_3_22)
        let s_3_24: () = Mem_set(state, tracer, s_3_5, s_3_21, s_3_23, s_3_22);
        // N s_3_25: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #32s : i
        let s_4_0: i128 = 32;
        // D s_4_1: read-var ptr:u64
        let s_4_1: u64 = fn_state.ptr;
        // D s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 64u16);
        // C s_4_3: cast cvt s_4_0 -> bv
        let s_4_3: Bits = Bits::new(s_4_0 as u128, 128);
        // D s_4_4: sub s_4_2 s_4_3
        let s_4_4: Bits = ((s_4_2) - (s_4_3));
        // D s_4_5: cast reint s_4_4 -> u64
        let s_4_5: u64 = (s_4_4.value() as u64);
        // D s_4_6: call SetCurrentGCSPointer(s_4_5)
        let s_4_6: () = SetCurrentGCSPointer(state, tracer, s_4_5);
        // N s_4_7: return
        return;
    }
}
