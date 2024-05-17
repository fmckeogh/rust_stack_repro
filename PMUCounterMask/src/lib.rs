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
use AArch32_GetNumEventCountersAccessible::*;
use u__id::*;
use zext_ones::*;
use AArch64_GetNumEventCountersAccessible::*;
use UsingAArch32::*;
use Bit::*;
use HaveAArch64::*;
use HavePMUv3ICNTR::*;
use common::*;
pub fn PMUCounterMask<T: Tracer>(state: &mut State, tracer: &T, gs_2042: ()) -> u64 {
    #[derive(Default)]
    struct FunctionState {
        n: i128,
        gs_2050: bool,
        mask: u64,
        gs_2042: (),
    }
    let fn_state = FunctionState {
        gs_2042,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call UsingAArch32(s_0_0)
        let s_0_1: bool = UsingAArch32(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b9 b1
        if s_0_1 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call AArch64_GetNumEventCountersAccessible(s_1_0)
        let s_1_1: i128 = AArch64_GetNumEventCountersAccessible(state, tracer, s_1_0);
        // D s_1_2: write-var n <= s_1_1
        fn_state.n = s_1_1;
        // N s_1_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_2_0: read-var n:i
        let s_2_0: i128 = fn_state.n;
        // D s_2_1: call __id(s_2_0)
        let s_2_1: i128 = u__id(state, tracer, s_2_0);
        // C s_2_2: const #0s : i
        let s_2_2: i128 = 0;
        // D s_2_3: cmp-ge s_2_1 s_2_2
        let s_2_3: bool = ((s_2_1) >= (s_2_2));
        // N s_2_4: assert s_2_3
        let s_2_4: () = assert!(s_2_3);
        // D s_2_5: call __id(s_2_0)
        let s_2_5: i128 = u__id(state, tracer, s_2_0);
        // C s_2_6: const #64s : i
        let s_2_6: i128 = 64;
        // D s_2_7: cmp-ge s_2_6 s_2_5
        let s_2_7: bool = ((s_2_6) >= (s_2_5));
        // N s_2_8: assert s_2_7
        let s_2_8: () = assert!(s_2_7);
        // C s_2_9: const #64s : i
        let s_2_9: i128 = 64;
        // D s_2_10: call zext_ones(s_2_9, s_2_0)
        let s_2_10: Bits = zext_ones(state, tracer, s_2_9, s_2_0);
        // D s_2_11: cast reint s_2_10 -> u64
        let s_2_11: u64 = (s_2_10.value() as u64);
        // D s_2_12: write-var mask <= s_2_11
        fn_state.mask = s_2_11;
        // C s_2_13: const #1u : u8
        let s_2_13: bool = true;
        // S s_2_14: call Bit(s_2_13)
        let s_2_14: bool = Bit(state, tracer, s_2_13);
        // D s_2_15: read-var mask:u64
        let s_2_15: u64 = fn_state.mask;
        // D s_2_16: cast zx s_2_15 -> bv
        let s_2_16: Bits = Bits::new(s_2_15 as u128, 64u16);
        // C s_2_17: const #0u : u32
        let s_2_17: u32 = 0;
        // D s_2_18: read-reg s_2_17:i64
        let s_2_18: i64 = {
            let value = state.read_register::<i64>(s_2_17 as isize);
            tracer.read_register(s_2_17 as isize, value);
            value
        };
        // D s_2_19: cast zx s_2_18 -> i
        let s_2_19: i128 = (i128::try_from(s_2_18).unwrap());
        // C s_2_20: const #1u : u64
        let s_2_20: u64 = 1;
        // D s_2_21: bit-insert s_2_16 s_2_16 s_2_19 s_2_20
        let s_2_21: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_2_20 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_2_16.length(),
            );
            (s_2_16 & mask) | (s_2_16 << s_2_19)
        };
        // D s_2_22: cast reint s_2_21 -> u64
        let s_2_22: u64 = (s_2_21.value() as u64);
        // D s_2_23: write-var mask <= s_2_22
        fn_state.mask = s_2_22;
        // C s_2_24: const #() : ()
        let s_2_24: () = ();
        // S s_2_25: call HaveAArch64(s_2_24)
        let s_2_25: bool = HaveAArch64(state, tracer, s_2_24);
        // N s_2_26: branch s_2_25 b8 b3
        if s_2_25 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#2050 <= s_3_0
        fn_state.gs_2050 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_4_0: read-var gs#2050:u8
        let s_4_0: bool = fn_state.gs_2050;
        // N s_4_1: branch s_4_0 b7 b5
        if s_4_0 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_6_0: read-var mask:u64
        let s_6_0: u64 = fn_state.mask;
        // N s_6_1: return s_6_0
        return s_6_0;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_7_0: const #1u : u8
        let s_7_0: bool = true;
        // S s_7_1: call Bit(s_7_0)
        let s_7_1: bool = Bit(state, tracer, s_7_0);
        // D s_7_2: read-var mask:u64
        let s_7_2: u64 = fn_state.mask;
        // D s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 64u16);
        // C s_7_4: const #8u : u32
        let s_7_4: u32 = 8;
        // D s_7_5: read-reg s_7_4:i64
        let s_7_5: i64 = {
            let value = state.read_register::<i64>(s_7_4 as isize);
            tracer.read_register(s_7_4 as isize, value);
            value
        };
        // D s_7_6: cast zx s_7_5 -> i
        let s_7_6: i128 = (i128::try_from(s_7_5).unwrap());
        // C s_7_7: const #1u : u64
        let s_7_7: u64 = 1;
        // D s_7_8: bit-insert s_7_3 s_7_3 s_7_6 s_7_7
        let s_7_8: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_7_7 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_7_3.length(),
            );
            (s_7_3 & mask) | (s_7_3 << s_7_6)
        };
        // D s_7_9: cast reint s_7_8 -> u64
        let s_7_9: u64 = (s_7_8.value() as u64);
        // D s_7_10: write-var mask <= s_7_9
        fn_state.mask = s_7_9;
        // N s_7_11: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call HavePMUv3ICNTR(s_8_0)
        let s_8_1: bool = HavePMUv3ICNTR(state, tracer, s_8_0);
        // D s_8_2: write-var gs#2050 <= s_8_1
        fn_state.gs_2050 = s_8_1;
        // N s_8_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call AArch32_GetNumEventCountersAccessible(s_9_0)
        let s_9_1: i128 = AArch32_GetNumEventCountersAccessible(state, tracer, s_9_0);
        // D s_9_2: write-var n <= s_9_1
        fn_state.n = s_9_1;
        // N s_9_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
