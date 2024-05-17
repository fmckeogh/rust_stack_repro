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
use SPEBranch::*;
use AArch64_BranchAddr::*;
use BRBEBranch::*;
use Hint_Branch::*;
use HaveBRBExt::*;
use UsingAArch32::*;
use HaveStatisticalProfiling::*;
use common::*;
pub fn BranchTo<T: Tracer>(
    state: &mut State,
    tracer: &T,
    target: Bits,
    branch_type: u32,
    branch_conditional: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        target_vaddress: u64,
        gs_3703: bool,
        gs_3702: bool,
        gs_3700: bool,
        gs_3701: bool,
        gs_3696: bool,
        gs_3704: bool,
        target: Bits,
        branch_type: u32,
        branch_conditional: bool,
    }
    let fn_state = FunctionState {
        target,
        branch_type,
        branch_conditional,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var branch_type:u32
        let s_0_0: u32 = fn_state.branch_type;
        // D s_0_1: call Hint_Branch(s_0_0)
        let s_0_1: () = Hint_Branch(state, tracer, s_0_0);
        // D s_0_2: read-var target:bv
        let s_0_2: Bits = fn_state.target;
        // D s_0_3: size-of s_0_2
        let s_0_3: u16 = s_0_2.length();
        // D s_0_4: cast zx s_0_3 -> i
        let s_0_4: i128 = (i128::try_from(s_0_3).unwrap());
        // C s_0_5: const #32s : i
        let s_0_5: i128 = 32;
        // D s_0_6: cmp-eq s_0_4 s_0_5
        let s_0_6: bool = ((s_0_4) == (s_0_5));
        // N s_0_7: branch s_0_6 b27 b1
        if s_0_6 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var target:bv
        let s_1_0: Bits = fn_state.target;
        // D s_1_1: size-of s_1_0
        let s_1_1: u16 = s_1_0.length();
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // C s_1_3: const #64s : i
        let s_1_3: i128 = 64;
        // D s_1_4: cmp-eq s_1_2 s_1_3
        let s_1_4: bool = ((s_1_2) == (s_1_3));
        // N s_1_5: branch s_1_4 b26 b2
        if s_1_4 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#3696 <= s_2_0
        fn_state.gs_3696 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#3696:u8
        let s_3_0: bool = fn_state.gs_3696;
        // N s_3_1: assert s_3_0
        let s_3_1: () = assert!(s_3_0);
        // C s_3_2: const #0s : i
        let s_3_2: i128 = 0;
        // D s_3_3: read-var target:bv
        let s_3_3: Bits = fn_state.target;
        // C s_3_4: const #1s : i64
        let s_3_4: i64 = 1;
        // C s_3_5: cast zx s_3_4 -> i
        let s_3_5: i128 = (i128::try_from(s_3_4).unwrap());
        // C s_3_6: const #63s : i
        let s_3_6: i128 = 63;
        // C s_3_7: add s_3_6 s_3_5
        let s_3_7: i128 = (s_3_6 + s_3_5);
        // D s_3_8: bit-extract s_3_3 s_3_2 s_3_7
        let s_3_8: Bits = (Bits::new(
            ((s_3_3) >> (s_3_2)).value(),
            u16::try_from(s_3_7).unwrap(),
        ));
        // D s_3_9: cast reint s_3_8 -> u64
        let s_3_9: u64 = (s_3_8.value() as u64);
        // C s_3_10: const #16975u : u32
        let s_3_10: u32 = 16975;
        // D s_3_11: read-reg s_3_10:u8
        let s_3_11: u8 = {
            let value = state.read_register::<u8>(s_3_10 as isize);
            tracer.read_register(s_3_10 as isize, value);
            value
        };
        // D s_3_12: call AArch64_BranchAddr(s_3_9, s_3_11)
        let s_3_12: u64 = AArch64_BranchAddr(state, tracer, s_3_9, s_3_11);
        // D s_3_13: write-var target_vaddress <= s_3_12
        fn_state.target_vaddress = s_3_12;
        // C s_3_14: const #() : ()
        let s_3_14: () = ();
        // S s_3_15: call HaveBRBExt(s_3_14)
        let s_3_15: bool = HaveBRBExt(state, tracer, s_3_14);
        // N s_3_16: branch s_3_15 b13 b4
        if s_3_15 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#3704 <= s_4_0
        fn_state.gs_3704 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#3704:u8
        let s_5_0: bool = fn_state.gs_3704;
        // N s_5_1: branch s_5_0 b12 b6
        if s_5_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call HaveStatisticalProfiling(s_7_0)
        let s_7_1: bool = HaveStatisticalProfiling(state, tracer, s_7_0);
        // N s_7_2: branch s_7_1 b11 b8
        if s_7_1 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var target_vaddress:u64
        let s_9_0: u64 = fn_state.target_vaddress;
        // C s_9_1: const #12744u : u32
        let s_9_1: u32 = 12744;
        // N s_9_2: write-reg s_9_1 <= s_9_0
        let s_9_2: () = {
            state.write_register::<u64>(s_9_1 as isize, s_9_0);
            tracer.write_register(s_9_1 as isize, s_9_0);
        };
        // N s_9_3: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #1u : u8
        let s_10_0: bool = true;
        // C s_10_1: const #14944u : u32
        let s_10_1: u32 = 14944;
        // N s_10_2: write-reg s_10_1 <= s_10_0
        let s_10_2: () = {
            state.write_register::<bool>(s_10_1 as isize, s_10_0);
            tracer.write_register(s_10_1 as isize, s_10_0);
        };
        // N s_10_3: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var target:bv
        let s_11_0: Bits = fn_state.target;
        // D s_11_1: read-var branch_type:u32
        let s_11_1: u32 = fn_state.branch_type;
        // D s_11_2: read-var branch_conditional:u8
        let s_11_2: bool = fn_state.branch_conditional;
        // C s_11_3: const #1u : u8
        let s_11_3: bool = true;
        // D s_11_4: call SPEBranch(s_11_0, s_11_1, s_11_2, s_11_3)
        let s_11_4: () = SPEBranch(state, tracer, s_11_0, s_11_1, s_11_2, s_11_3);
        // N s_11_5: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var branch_type:u32
        let s_12_0: u32 = fn_state.branch_type;
        // D s_12_1: read-var branch_conditional:u8
        let s_12_1: bool = fn_state.branch_conditional;
        // D s_12_2: read-var target_vaddress:u64
        let s_12_2: u64 = fn_state.target_vaddress;
        // D s_12_3: call BRBEBranch(s_12_0, s_12_1, s_12_2)
        let s_12_3: () = BRBEBranch(state, tracer, s_12_0, s_12_1, s_12_2);
        // N s_12_4: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var branch_type:u32
        let s_13_0: u32 = fn_state.branch_type;
        // C s_13_1: const #5u : u32
        let s_13_1: u32 = 5;
        // D s_13_2: cmp-eq s_13_0 s_13_1
        let s_13_2: bool = ((s_13_0) == (s_13_1));
        // N s_13_3: branch s_13_2 b25 b14
        if s_13_2 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var branch_type:u32
        let s_14_0: u32 = fn_state.branch_type;
        // C s_14_1: const #6u : u32
        let s_14_1: u32 = 6;
        // D s_14_2: cmp-eq s_14_0 s_14_1
        let s_14_2: bool = ((s_14_0) == (s_14_1));
        // N s_14_3: branch s_14_2 b24 b15
        if s_14_2 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var branch_type:u32
        let s_15_0: u32 = fn_state.branch_type;
        // C s_15_1: const #0u : u32
        let s_15_1: u32 = 0;
        // D s_15_2: cmp-eq s_15_0 s_15_1
        let s_15_2: bool = ((s_15_0) == (s_15_1));
        // N s_15_3: branch s_15_2 b23 b16
        if s_15_2 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var branch_type:u32
        let s_16_0: u32 = fn_state.branch_type;
        // C s_16_1: const #1u : u32
        let s_16_1: u32 = 1;
        // D s_16_2: cmp-eq s_16_0 s_16_1
        let s_16_2: bool = ((s_16_0) == (s_16_1));
        // N s_16_3: branch s_16_2 b22 b17
        if s_16_2 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var branch_type:u32
        let s_17_0: u32 = fn_state.branch_type;
        // C s_17_1: const #4u : u32
        let s_17_1: u32 = 4;
        // D s_17_2: cmp-eq s_17_0 s_17_1
        let s_17_2: bool = ((s_17_0) == (s_17_1));
        // D s_17_3: write-var gs#3700 <= s_17_2
        fn_state.gs_3700 = s_17_2;
        // N s_17_4: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#3700:u8
        let s_18_0: bool = fn_state.gs_3700;
        // D s_18_1: write-var gs#3701 <= s_18_0
        fn_state.gs_3701 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#3701:u8
        let s_19_0: bool = fn_state.gs_3701;
        // D s_19_1: write-var gs#3702 <= s_19_0
        fn_state.gs_3702 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#3702:u8
        let s_20_0: bool = fn_state.gs_3702;
        // D s_20_1: write-var gs#3703 <= s_20_0
        fn_state.gs_3703 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#3703:u8
        let s_21_0: bool = fn_state.gs_3703;
        // D s_21_1: write-var gs#3704 <= s_21_0
        fn_state.gs_3704 = s_21_0;
        // N s_21_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // D s_22_1: write-var gs#3700 <= s_22_0
        fn_state.gs_3700 = s_22_0;
        // N s_22_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #1u : u8
        let s_23_0: bool = true;
        // D s_23_1: write-var gs#3701 <= s_23_0
        fn_state.gs_3701 = s_23_0;
        // N s_23_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #1u : u8
        let s_24_0: bool = true;
        // D s_24_1: write-var gs#3702 <= s_24_0
        fn_state.gs_3702 = s_24_0;
        // N s_24_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #1u : u8
        let s_25_0: bool = true;
        // D s_25_1: write-var gs#3703 <= s_25_0
        fn_state.gs_3703 = s_25_0;
        // N s_25_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #() : ()
        let s_26_0: () = ();
        // S s_26_1: call UsingAArch32(s_26_0)
        let s_26_1: bool = UsingAArch32(state, tracer, s_26_0);
        // S s_26_2: not s_26_1
        let s_26_2: bool = !s_26_1;
        // D s_26_3: write-var gs#3696 <= s_26_2
        fn_state.gs_3696 = s_26_2;
        // N s_26_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #() : ()
        let s_27_0: () = ();
        // S s_27_1: call UsingAArch32(s_27_0)
        let s_27_1: bool = UsingAArch32(state, tracer, s_27_0);
        // N s_27_2: assert s_27_1
        let s_27_2: () = assert!(s_27_1);
        // C s_27_3: const #64s : i
        let s_27_3: i128 = 64;
        // D s_27_4: read-var target:bv
        let s_27_4: Bits = fn_state.target;
        // D s_27_5: bits-cast zx s_27_4 -> bv length s_27_3
        let s_27_5: Bits = s_27_4.zero_extend(s_27_3);
        // D s_27_6: cast reint s_27_5 -> u64
        let s_27_6: u64 = (s_27_5.value() as u64);
        // C s_27_7: const #12744u : u32
        let s_27_7: u32 = 12744;
        // N s_27_8: write-reg s_27_7 <= s_27_6
        let s_27_8: () = {
            state.write_register::<u64>(s_27_7 as isize, s_27_6);
            tracer.write_register(s_27_7 as isize, s_27_6);
        };
        // N s_27_9: jump b10
        return block_10(state, tracer, fn_state);
    }
}
