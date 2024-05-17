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
use AArch64_AllocationTagAccessIsEnabled::*;
use common::*;
pub fn AArch64_AddressWithAllocationTag<T: Tracer>(
    state: &mut State,
    tracer: &T,
    address: u64,
    allocation_tag: u8,
) -> u64 {
    #[derive(Default)]
    struct FunctionState {
        tag: u8,
        result: u64,
        address: u64,
        allocation_tag: u8,
    }
    let fn_state = FunctionState {
        address,
        allocation_tag,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_0_0: read-var address:u64
        let s_0_0: u64 = fn_state.address;
        // D s_0_1: write-var result <= s_0_0
        fn_state.result = s_0_0;
        // C s_0_2: const #16975u : u32
        let s_0_2: u32 = 16975;
        // D s_0_3: read-reg s_0_2:u8
        let s_0_3: u8 = {
            let value = state.read_register::<u8>(s_0_2 as isize);
            tracer.read_register(s_0_2 as isize, value);
            value
        };
        // D s_0_4: call AArch64_AllocationTagAccessIsEnabled(s_0_3)
        let s_0_4: bool = AArch64_AllocationTagAccessIsEnabled(state, tracer, s_0_3);
        // N s_0_5: branch s_0_4 b3 b1
        if s_0_4 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_1_0: const #0u : u8
        let s_1_0: u8 = 0;
        // D s_1_1: write-var tag <= s_1_0
        fn_state.tag = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_2_0: const #56s : i
        let s_2_0: i128 = 56;
        // D s_2_1: read-var result:u64
        let s_2_1: u64 = fn_state.result;
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 64u16);
        // D s_2_3: read-var tag:u8
        let s_2_3: u8 = fn_state.tag;
        // D s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 4u16);
        // C s_2_5: const #3s : i
        let s_2_5: i128 = 3;
        // C s_2_6: const #1u : u64
        let s_2_6: u64 = 1;
        // C s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 64u16);
        // C s_2_8: lsl s_2_7 s_2_5
        let s_2_8: Bits = s_2_7 << s_2_5;
        // C s_2_9: sub s_2_8 s_2_7
        let s_2_9: Bits = ((s_2_8) - (s_2_7));
        // D s_2_10: and s_2_4 s_2_9
        let s_2_10: Bits = ((s_2_4) & (s_2_9));
        // D s_2_11: lsl s_2_10 s_2_0
        let s_2_11: Bits = s_2_10 << s_2_0;
        // C s_2_12: lsl s_2_9 s_2_0
        let s_2_12: Bits = s_2_9 << s_2_0;
        // C s_2_13: cmpl s_2_12
        let s_2_13: Bits = !s_2_12;
        // D s_2_14: and s_2_2 s_2_13
        let s_2_14: Bits = ((s_2_2) & (s_2_13));
        // D s_2_15: or s_2_14 s_2_11
        let s_2_15: Bits = ((s_2_14) | (s_2_11));
        // D s_2_16: cast reint s_2_15 -> u64
        let s_2_16: u64 = (s_2_15.value() as u64);
        // D s_2_17: write-var result <= s_2_16
        fn_state.result = s_2_16;
        // D s_2_18: read-var result:u64
        let s_2_18: u64 = fn_state.result;
        // N s_2_19: return s_2_18
        return s_2_18;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_3_0: read-var allocation_tag:u8
        let s_3_0: u8 = fn_state.allocation_tag;
        // D s_3_1: write-var tag <= s_3_0
        fn_state.tag = s_3_0;
        // N s_3_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
