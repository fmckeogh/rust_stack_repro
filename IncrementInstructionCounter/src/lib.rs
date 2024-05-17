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
use integer_access::*;
use CountPMUEvents::*;
use integer_subrange::*;
use Mk_PMICNTR_EL0_Type::*;
use common::*;
pub fn IncrementInstructionCounter<T: Tracer>(
    state: &mut State,
    tracer: &T,
    increment_name: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        increment_name: i128,
    }
    let fn_state = FunctionState {
        increment_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #8u : u32
        let s_0_0: u32 = 8;
        // D s_0_1: read-reg s_0_0:i64
        let s_0_1: i64 = {
            let value = state.read_register::<i64>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: call CountPMUEvents(s_0_1)
        let s_0_2: bool = CountPMUEvents(state, tracer, s_0_1);
        // N s_0_3: branch s_0_2 b2 b1
        if s_0_2 {
            return block_2(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #12944u : u32
        let s_2_0: u32 = 12944;
        // D s_2_1: read-reg s_2_0:u64
        let s_2_1: u64 = {
            let value = state.read_register::<u64>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 64u16);
        // D s_2_3: cast zx s_2_2 -> i
        let s_2_3: i128 = (s_2_2.value() as i128);
        // D s_2_4: read-var increment_name:i
        let s_2_4: i128 = fn_state.increment_name;
        // D s_2_5: add s_2_3 s_2_4
        let s_2_5: i128 = (s_2_3 + s_2_4);
        // C s_2_6: const #63s : i
        let s_2_6: i128 = 63;
        // C s_2_7: const #0s : i
        let s_2_7: i128 = 0;
        // D s_2_8: call integer_subrange(s_2_5, s_2_6, s_2_7)
        let s_2_8: Bits = integer_subrange(state, tracer, s_2_5, s_2_6, s_2_7);
        // D s_2_9: cast reint s_2_8 -> u64
        let s_2_9: u64 = (s_2_8.value() as u64);
        // D s_2_10: call Mk_PMICNTR_EL0_Type(s_2_9)
        let s_2_10: ProductType5c790c8ef59cc8b2 = Mk_PMICNTR_EL0_Type(
            state,
            tracer,
            s_2_9,
        );
        // C s_2_11: const #12944u : u32
        let s_2_11: u32 = 12944;
        // N s_2_12: write-reg s_2_11 <= s_2_10
        let s_2_12: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_2_11 as isize, s_2_10);
            tracer.write_register(s_2_11 as isize, s_2_10);
        };
        // C s_2_13: const #64s : i
        let s_2_13: i128 = 64;
        // D s_2_14: call integer_access(s_2_3, s_2_13)
        let s_2_14: bool = integer_access(state, tracer, s_2_3, s_2_13);
        // C s_2_15: const #0s : i
        let s_2_15: i128 = 0;
        // C s_2_16: const #0u : u64
        let s_2_16: u64 = 0;
        // D s_2_17: cast zx s_2_14 -> u64
        let s_2_17: u64 = (s_2_14 as u64);
        // C s_2_18: const #1u : u64
        let s_2_18: u64 = 1;
        // D s_2_19: and s_2_17 s_2_18
        let s_2_19: u64 = ((s_2_17) & (s_2_18));
        // D s_2_20: cmp-eq s_2_19 s_2_18
        let s_2_20: bool = ((s_2_19) == (s_2_18));
        // D s_2_21: lsl s_2_17 s_2_15
        let s_2_21: u64 = s_2_17 << s_2_15;
        // D s_2_22: or s_2_16 s_2_21
        let s_2_22: u64 = ((s_2_16) | (s_2_21));
        // D s_2_23: cmpl s_2_21
        let s_2_23: u64 = !s_2_21;
        // D s_2_24: and s_2_16 s_2_23
        let s_2_24: u64 = ((s_2_16) & (s_2_23));
        // D s_2_25: select s_2_20 s_2_22 s_2_24
        let s_2_25: u64 = if s_2_20 { s_2_22 } else { s_2_24 };
        // D s_2_26: cast trunc s_2_25 -> u8
        let s_2_26: bool = ((s_2_25) != 0);
        // C s_2_27: const #64s : i
        let s_2_27: i128 = 64;
        // D s_2_28: call integer_access(s_2_5, s_2_27)
        let s_2_28: bool = integer_access(state, tracer, s_2_5, s_2_27);
        // C s_2_29: const #0s : i
        let s_2_29: i128 = 0;
        // C s_2_30: const #0u : u64
        let s_2_30: u64 = 0;
        // D s_2_31: cast zx s_2_28 -> u64
        let s_2_31: u64 = (s_2_28 as u64);
        // C s_2_32: const #1u : u64
        let s_2_32: u64 = 1;
        // D s_2_33: and s_2_31 s_2_32
        let s_2_33: u64 = ((s_2_31) & (s_2_32));
        // D s_2_34: cmp-eq s_2_33 s_2_32
        let s_2_34: bool = ((s_2_33) == (s_2_32));
        // D s_2_35: lsl s_2_31 s_2_29
        let s_2_35: u64 = s_2_31 << s_2_29;
        // D s_2_36: or s_2_30 s_2_35
        let s_2_36: u64 = ((s_2_30) | (s_2_35));
        // D s_2_37: cmpl s_2_35
        let s_2_37: u64 = !s_2_35;
        // D s_2_38: and s_2_30 s_2_37
        let s_2_38: u64 = ((s_2_30) & (s_2_37));
        // D s_2_39: select s_2_34 s_2_36 s_2_38
        let s_2_39: u64 = if s_2_34 { s_2_36 } else { s_2_38 };
        // D s_2_40: cast trunc s_2_39 -> u8
        let s_2_40: bool = ((s_2_39) != 0);
        // D s_2_41: cast zx s_2_26 -> bv
        let s_2_41: Bits = Bits::new(s_2_26 as u128, 1u16);
        // D s_2_42: cast zx s_2_40 -> bv
        let s_2_42: Bits = Bits::new(s_2_40 as u128, 1u16);
        // D s_2_43: cmp-ne s_2_41 s_2_42
        let s_2_43: bool = ((s_2_41) != (s_2_42));
        // N s_2_44: branch s_2_43 b4 b3
        if s_2_43 {
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
        // N s_3_0: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #104640u : u32
        let s_4_0: u32 = 104640;
        // D s_4_1: read-reg s_4_0:struct
        let s_4_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // C s_4_2: const #104640u : u32
        let s_4_2: u32 = 104640;
        // N s_4_3: write-reg s_4_2 <= s_4_1
        let s_4_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_4_2 as isize, s_4_1);
            tracer.write_register(s_4_2 as isize, s_4_1);
        };
        // C s_4_4: const #104888u : u32
        let s_4_4: u32 = 104888;
        // D s_4_5: read-reg s_4_4:struct
        let s_4_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_4_4 as isize);
            tracer.read_register(s_4_4 as isize, value);
            value
        };
        // C s_4_6: const #104888u : u32
        let s_4_6: u32 = 104888;
        // N s_4_7: write-reg s_4_6 <= s_4_5
        let s_4_7: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_4_6 as isize, s_4_5);
            tracer.write_register(s_4_6 as isize, s_4_5);
        };
        // N s_4_8: return
        return;
    }
}
