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
use Mem_set::*;
use common::*;
pub fn AddGCSRecord<T: Tracer>(state: &mut State, tracer: &T, vaddress: u64) -> () {
    #[derive(Default)]
    struct FunctionState {
        ptr: u64,
        vaddress: u64,
    }
    let fn_state = FunctionState {
        vaddress,
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
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call GetCurrentGCSPointer(s_0_4)
        let s_0_5: u64 = GetCurrentGCSPointer(state, tracer, s_0_4);
        // D s_0_6: write-var ptr <= s_0_5
        fn_state.ptr = s_0_5;
        // C s_0_7: const #8s : i
        let s_0_7: i128 = 8;
        // D s_0_8: read-var ptr:u64
        let s_0_8: u64 = fn_state.ptr;
        // D s_0_9: cast zx s_0_8 -> bv
        let s_0_9: Bits = Bits::new(s_0_8 as u128, 64u16);
        // C s_0_10: cast cvt s_0_7 -> bv
        let s_0_10: Bits = Bits::new(s_0_7 as u128, 128);
        // D s_0_11: sub s_0_9 s_0_10
        let s_0_11: Bits = ((s_0_9) - (s_0_10));
        // D s_0_12: cast reint s_0_11 -> u64
        let s_0_12: u64 = (s_0_11.value() as u64);
        // C s_0_13: const #8s : i
        let s_0_13: i128 = 8;
        // D s_0_14: read-var vaddress:u64
        let s_0_14: u64 = fn_state.vaddress;
        // D s_0_15: cast zx s_0_14 -> bv
        let s_0_15: Bits = Bits::new(s_0_14 as u128, 64u16);
        // D s_0_16: call Mem_set(s_0_12, s_0_13, s_0_3, s_0_15)
        let s_0_16: () = Mem_set(state, tracer, s_0_12, s_0_13, s_0_3, s_0_15);
        // N s_0_17: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #8s : i
        let s_1_0: i128 = 8;
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
        // D s_1_6: call SetCurrentGCSPointer(s_1_5)
        let s_1_6: () = SetCurrentGCSPointer(state, tracer, s_1_5);
        // N s_1_7: return
        return;
    }
}
