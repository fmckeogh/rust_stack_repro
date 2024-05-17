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
use X_set::*;
use SP_read::*;
use SP_set::*;
use AddWithCarry::*;
use AArch64_AllocationTagAccessIsEnabled::*;
use u_get_GCR_EL1_Type_Exclude::*;
use X_read::*;
use AArch64_AddressWithAllocationTag::*;
use AArch64_ChooseNonExcludedTag::*;
use AArch64_AllocationTagFromAddress::*;
use common::*;
pub fn execute_aarch64_instrs_integer_tags_mcaddtag<T: Tracer>(
    state: &mut State,
    tracer: &T,
    ADD: bool,
    d: i64,
    n: i64,
    offset: u64,
    tag_offset: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        start_tag: u8,
        resultshadow_1030: u64,
        gs_682587: ProductTyped54bc449dd09e5bd,
        gs_682592: ProductTyped54bc449dd09e5bd,
        rtag: u8,
        result: u64,
        operand1: u64,
        exclude: u16,
        ADD: bool,
        d: i64,
        n: i64,
        offset: u64,
        tag_offset: u8,
    }
    let fn_state = FunctionState {
        ADD,
        d,
        n,
        offset,
        tag_offset,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #31s : i
        let s_0_0: i128 = 31;
        // D s_0_1: read-var n:i64
        let s_0_1: i64 = fn_state.n;
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (i128::try_from(s_0_1).unwrap());
        // D s_0_3: cmp-eq s_0_2 s_0_0
        let s_0_3: bool = ((s_0_2) == (s_0_0));
        // N s_0_4: branch s_0_3 b11 b1
        if s_0_3 {
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
        // C s_1_0: const #64s : i64
        let s_1_0: i64 = 64;
        // D s_1_1: read-var n:i64
        let s_1_1: i64 = fn_state.n;
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: call X_read(s_1_2, s_1_0)
        let s_1_3: Bits = X_read(state, tracer, s_1_2, s_1_0);
        // D s_1_4: cast reint s_1_3 -> u64
        let s_1_4: u64 = (s_1_3.value() as u64);
        // D s_1_5: write-var operand1 <= s_1_4
        fn_state.operand1 = s_1_4;
        // N s_1_6: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var operand1:u64
        let s_2_0: u64 = fn_state.operand1;
        // D s_2_1: call AArch64_AllocationTagFromAddress(s_2_0)
        let s_2_1: u8 = AArch64_AllocationTagFromAddress(state, tracer, s_2_0);
        // D s_2_2: write-var start_tag <= s_2_1
        fn_state.start_tag = s_2_1;
        // C s_2_3: const #11480u : u32
        let s_2_3: u32 = 11480;
        // D s_2_4: read-reg s_2_3:struct
        let s_2_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_2_3 as isize);
            tracer.read_register(s_2_3 as isize, value);
            value
        };
        // D s_2_5: call _get_GCR_EL1_Type_Exclude(s_2_4)
        let s_2_5: u16 = u_get_GCR_EL1_Type_Exclude(state, tracer, s_2_4);
        // D s_2_6: write-var exclude <= s_2_5
        fn_state.exclude = s_2_5;
        // C s_2_7: const #16975u : u32
        let s_2_7: u32 = 16975;
        // D s_2_8: read-reg s_2_7:u8
        let s_2_8: u8 = {
            let value = state.read_register::<u8>(s_2_7 as isize);
            tracer.read_register(s_2_7 as isize, value);
            value
        };
        // D s_2_9: call AArch64_AllocationTagAccessIsEnabled(s_2_8)
        let s_2_9: bool = AArch64_AllocationTagAccessIsEnabled(state, tracer, s_2_8);
        // N s_2_10: branch s_2_9 b10 b3
        if s_2_9 {
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
        // C s_3_0: const #0u : u8
        let s_3_0: u8 = 0;
        // D s_3_1: write-var rtag <= s_3_0
        fn_state.rtag = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var ADD:u8
        let s_4_0: bool = fn_state.ADD;
        // N s_4_1: branch s_4_0 b9 b5
        if s_4_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var offset:u64
        let s_5_0: u64 = fn_state.offset;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 64u16);
        // D s_5_2: not s_5_1
        let s_5_2: Bits = !s_5_1;
        // D s_5_3: cast reint s_5_2 -> u64
        let s_5_3: u64 = (s_5_2.value() as u64);
        // D s_5_4: read-var operand1:u64
        let s_5_4: u64 = fn_state.operand1;
        // D s_5_5: cast zx s_5_4 -> bv
        let s_5_5: Bits = Bits::new(s_5_4 as u128, 64u16);
        // D s_5_6: cast zx s_5_3 -> bv
        let s_5_6: Bits = Bits::new(s_5_3 as u128, 64u16);
        // C s_5_7: const #1u : u8
        let s_5_7: bool = true;
        // D s_5_8: call AddWithCarry(s_5_5, s_5_6, s_5_7)
        let s_5_8: ProductTyped54bc449dd09e5bd = AddWithCarry(
            state,
            tracer,
            s_5_5,
            s_5_6,
            s_5_7,
        );
        // D s_5_9: write-var gs#682587 <= s_5_8
        fn_state.gs_682587 = s_5_8;
        // D s_5_10: read-var gs#682587.0:struct
        let s_5_10: Bits = fn_state.gs_682587._0;
        // D s_5_11: cast reint s_5_10 -> u64
        let s_5_11: u64 = (s_5_10.value() as u64);
        // D s_5_12: write-var result <= s_5_11
        fn_state.result = s_5_11;
        // N s_5_13: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var result:u64
        let s_6_0: u64 = fn_state.result;
        // D s_6_1: read-var rtag:u8
        let s_6_1: u8 = fn_state.rtag;
        // D s_6_2: call AArch64_AddressWithAllocationTag(s_6_0, s_6_1)
        let s_6_2: u64 = AArch64_AddressWithAllocationTag(state, tracer, s_6_0, s_6_1);
        // D s_6_3: write-var resultshadow#1030 <= s_6_2
        fn_state.resultshadow_1030 = s_6_2;
        // C s_6_4: const #31s : i
        let s_6_4: i128 = 31;
        // D s_6_5: read-var d:i64
        let s_6_5: i64 = fn_state.d;
        // D s_6_6: cast zx s_6_5 -> i
        let s_6_6: i128 = (i128::try_from(s_6_5).unwrap());
        // D s_6_7: cmp-eq s_6_6 s_6_4
        let s_6_7: bool = ((s_6_6) == (s_6_4));
        // N s_6_8: branch s_6_7 b8 b7
        if s_6_7 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #64s : i64
        let s_7_0: i64 = 64;
        // D s_7_1: read-var d:i64
        let s_7_1: i64 = fn_state.d;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: read-var resultshadow#1030:u64
        let s_7_3: u64 = fn_state.resultshadow_1030;
        // D s_7_4: cast zx s_7_3 -> bv
        let s_7_4: Bits = Bits::new(s_7_3 as u128, 64u16);
        // D s_7_5: call X_set(s_7_2, s_7_0, s_7_4)
        let s_7_5: () = X_set(state, tracer, s_7_2, s_7_0, s_7_4);
        // N s_7_6: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var resultshadow#1030:u64
        let s_8_0: u64 = fn_state.resultshadow_1030;
        // D s_8_1: call SP_set(s_8_0)
        let s_8_1: () = SP_set(state, tracer, s_8_0);
        // N s_8_2: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var operand1:u64
        let s_9_0: u64 = fn_state.operand1;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 64u16);
        // D s_9_2: read-var offset:u64
        let s_9_2: u64 = fn_state.offset;
        // D s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 64u16);
        // C s_9_4: const #0u : u8
        let s_9_4: bool = false;
        // D s_9_5: call AddWithCarry(s_9_1, s_9_3, s_9_4)
        let s_9_5: ProductTyped54bc449dd09e5bd = AddWithCarry(
            state,
            tracer,
            s_9_1,
            s_9_3,
            s_9_4,
        );
        // D s_9_6: write-var gs#682592 <= s_9_5
        fn_state.gs_682592 = s_9_5;
        // D s_9_7: read-var gs#682592.0:struct
        let s_9_7: Bits = fn_state.gs_682592._0;
        // D s_9_8: cast reint s_9_7 -> u64
        let s_9_8: u64 = (s_9_7.value() as u64);
        // D s_9_9: write-var result <= s_9_8
        fn_state.result = s_9_8;
        // N s_9_10: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var start_tag:u8
        let s_10_0: u8 = fn_state.start_tag;
        // D s_10_1: read-var tag_offset:u8
        let s_10_1: u8 = fn_state.tag_offset;
        // D s_10_2: read-var exclude:u16
        let s_10_2: u16 = fn_state.exclude;
        // D s_10_3: call AArch64_ChooseNonExcludedTag(s_10_0, s_10_1, s_10_2)
        let s_10_3: u8 = AArch64_ChooseNonExcludedTag(
            state,
            tracer,
            s_10_0,
            s_10_1,
            s_10_2,
        );
        // D s_10_4: write-var rtag <= s_10_3
        fn_state.rtag = s_10_3;
        // N s_10_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call SP_read(s_11_0)
        let s_11_1: u64 = SP_read(state, tracer, s_11_0);
        // D s_11_2: write-var operand1 <= s_11_1
        fn_state.operand1 = s_11_1;
        // N s_11_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
