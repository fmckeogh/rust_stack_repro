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
use AArch32_CurrentCond::*;
use Bit::*;
use CurrentInstrSet::*;
use BranchTo::*;
use common::*;
pub fn BranchWritePC<T: Tracer>(
    state: &mut State,
    tracer: &T,
    address_in: u32,
    branch_type: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_31417: bool,
        address: u32,
        address_in: u32,
        branch_type: u32,
    }
    let fn_state = FunctionState {
        address_in,
        branch_type,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var address_in:u32
        let s_0_0: u32 = fn_state.address_in;
        // D s_0_1: write-var address <= s_0_0
        fn_state.address = s_0_0;
        // C s_0_2: const #() : ()
        let s_0_2: () = ();
        // S s_0_3: call CurrentInstrSet(s_0_2)
        let s_0_3: u32 = CurrentInstrSet(state, tracer, s_0_2);
        // C s_0_4: const #1u : u32
        let s_0_4: u32 = 1;
        // S s_0_5: cmp-eq s_0_3 s_0_4
        let s_0_5: bool = ((s_0_3) == (s_0_4));
        // N s_0_6: branch s_0_5 b6 b1
        if s_0_5 {
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
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // S s_1_1: call Bit(s_1_0)
        let s_1_1: bool = Bit(state, tracer, s_1_0);
        // C s_1_2: const #0s : i
        let s_1_2: i128 = 0;
        // D s_1_3: read-var address:u32
        let s_1_3: u32 = fn_state.address;
        // D s_1_4: cast zx s_1_3 -> bv
        let s_1_4: Bits = Bits::new(s_1_3 as u128, 32u16);
        // C s_1_5: const #1u : u64
        let s_1_5: u64 = 1;
        // D s_1_6: bit-insert s_1_4 s_1_4 s_1_2 s_1_5
        let s_1_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_1_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_1_4.length(),
            );
            (s_1_4 & mask) | (s_1_4 << s_1_2)
        };
        // D s_1_7: cast reint s_1_6 -> u32
        let s_1_7: u32 = (s_1_6.value() as u32);
        // D s_1_8: write-var address <= s_1_7
        fn_state.address = s_1_7;
        // N s_1_9: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call AArch32_CurrentCond(s_2_0)
        let s_2_1: u8 = AArch32_CurrentCond(state, tracer, s_2_0);
        // C s_2_2: const #1s : i
        let s_2_2: i128 = 1;
        // S s_2_3: cast zx s_2_1 -> bv
        let s_2_3: Bits = Bits::new(s_2_1 as u128, 4u16);
        // C s_2_4: const #1s : i64
        let s_2_4: i64 = 1;
        // C s_2_5: cast zx s_2_4 -> i
        let s_2_5: i128 = (i128::try_from(s_2_4).unwrap());
        // C s_2_6: const #2s : i
        let s_2_6: i128 = 2;
        // C s_2_7: add s_2_6 s_2_5
        let s_2_7: i128 = (s_2_6 + s_2_5);
        // D s_2_8: bit-extract s_2_3 s_2_2 s_2_7
        let s_2_8: Bits = (Bits::new(
            ((s_2_3) >> (s_2_2)).value(),
            u16::try_from(s_2_7).unwrap(),
        ));
        // D s_2_9: cast reint s_2_8 -> u8
        let s_2_9: u8 = (s_2_8.value() as u8);
        // D s_2_10: cast zx s_2_9 -> bv
        let s_2_10: Bits = Bits::new(s_2_9 as u128, 3u16);
        // C s_2_11: const #7u : u8
        let s_2_11: u8 = 7;
        // C s_2_12: cast zx s_2_11 -> bv
        let s_2_12: Bits = Bits::new(s_2_11 as u128, 3u16);
        // D s_2_13: cmp-eq s_2_10 s_2_12
        let s_2_13: bool = ((s_2_10) == (s_2_12));
        // D s_2_14: not s_2_13
        let s_2_14: bool = !s_2_13;
        // N s_2_15: branch s_2_14 b5 b3
        if s_2_14 {
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
        // C s_3_0: const #1u : u8
        let s_3_0: bool = true;
        // D s_3_1: write-var gs#31417 <= s_3_0
        fn_state.gs_31417 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#31417:u8
        let s_4_0: bool = fn_state.gs_31417;
        // D s_4_1: not s_4_0
        let s_4_1: bool = !s_4_0;
        // D s_4_2: read-var address:u32
        let s_4_2: u32 = fn_state.address;
        // D s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 32u16);
        // D s_4_4: read-var branch_type:u32
        let s_4_4: u32 = fn_state.branch_type;
        // D s_4_5: call BranchTo(s_4_3, s_4_4, s_4_1)
        let s_4_5: () = BranchTo(state, tracer, s_4_3, s_4_4, s_4_1);
        // N s_4_6: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#31417 <= s_5_0
        fn_state.gs_31417 = s_5_0;
        // N s_5_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0s : i
        let s_6_0: i128 = 0;
        // D s_6_1: read-var address:u32
        let s_6_1: u32 = fn_state.address;
        // D s_6_2: cast zx s_6_1 -> bv
        let s_6_2: Bits = Bits::new(s_6_1 as u128, 32u16);
        // C s_6_3: const #0u : u8
        let s_6_3: u8 = 0;
        // C s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 2u16);
        // C s_6_5: const #1s : i
        let s_6_5: i128 = 1;
        // C s_6_6: const #1u : u64
        let s_6_6: u64 = 1;
        // C s_6_7: cast zx s_6_6 -> bv
        let s_6_7: Bits = Bits::new(s_6_6 as u128, 64u16);
        // C s_6_8: lsl s_6_7 s_6_5
        let s_6_8: Bits = s_6_7 << s_6_5;
        // C s_6_9: sub s_6_8 s_6_7
        let s_6_9: Bits = ((s_6_8) - (s_6_7));
        // C s_6_10: and s_6_4 s_6_9
        let s_6_10: Bits = ((s_6_4) & (s_6_9));
        // C s_6_11: lsl s_6_10 s_6_0
        let s_6_11: Bits = s_6_10 << s_6_0;
        // C s_6_12: lsl s_6_9 s_6_0
        let s_6_12: Bits = s_6_9 << s_6_0;
        // C s_6_13: cmpl s_6_12
        let s_6_13: Bits = !s_6_12;
        // D s_6_14: and s_6_2 s_6_13
        let s_6_14: Bits = ((s_6_2) & (s_6_13));
        // D s_6_15: or s_6_14 s_6_11
        let s_6_15: Bits = ((s_6_14) | (s_6_11));
        // D s_6_16: cast reint s_6_15 -> u32
        let s_6_16: u32 = (s_6_15.value() as u32);
        // D s_6_17: write-var address <= s_6_16
        fn_state.address = s_6_16;
        // N s_6_18: jump b2
        return block_2(state, tracer, fn_state);
    }
}
