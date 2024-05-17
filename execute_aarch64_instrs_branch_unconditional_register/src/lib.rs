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
use GCSPCREnabled::*;
use AuthIA::*;
use PC_read::*;
use LoadCheckGCSRecord::*;
use SP_read::*;
use SetCurrentGCSPointer::*;
use X_read::*;
use AddGCSRecord::*;
use GetCurrentGCSPointer::*;
use BranchTo::*;
use HaveGCS::*;
use AuthIB::*;
use common::*;
pub fn execute_aarch64_instrs_branch_unconditional_register<T: Tracer>(
    state: &mut State,
    tracer: &T,
    branch_type: u32,
    m: i64,
    n: i64,
    pac: bool,
    source_is_sp: bool,
    use_key_a: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        target: u64,
        modifier: u64,
        inst_type: u32,
        gs_144654: bool,
        gs_144653: bool,
        gs_144665: bool,
        gs_144659: bool,
        branch_type: u32,
        m: i64,
        n: i64,
        pac: bool,
        source_is_sp: bool,
        use_key_a: bool,
    }
    let fn_state = FunctionState {
        branch_type,
        m,
        n,
        pac,
        source_is_sp,
        use_key_a,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #64s : i64
        let s_0_0: i64 = 64;
        // D s_0_1: read-var n:i64
        let s_0_1: i64 = fn_state.n;
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (i128::try_from(s_0_1).unwrap());
        // D s_0_3: call X_read(s_0_2, s_0_0)
        let s_0_3: Bits = X_read(state, tracer, s_0_2, s_0_0);
        // D s_0_4: cast reint s_0_3 -> u64
        let s_0_4: u64 = (s_0_3.value() as u64);
        // D s_0_5: write-var target <= s_0_4
        fn_state.target = s_0_4;
        // D s_0_6: read-var pac:u8
        let s_0_6: bool = fn_state.pac;
        // N s_0_7: branch s_0_6 b41 b1
        if s_0_6 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var branch_type:u32
        let s_2_0: u32 = fn_state.branch_type;
        // C s_2_1: const #4u : u32
        let s_2_1: u32 = 4;
        // D s_2_2: cmp-eq s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) == (s_2_1));
        // N s_2_3: branch s_2_2 b40 b3
        if s_2_2 {
            return block_40(state, tracer, fn_state);
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
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#144653 <= s_3_0
        fn_state.gs_144653 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#144653:u8
        let s_4_0: bool = fn_state.gs_144653;
        // N s_4_1: branch s_4_0 b39 b5
        if s_4_0 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#144654 <= s_5_0
        fn_state.gs_144654 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#144654:u8
        let s_6_0: bool = fn_state.gs_144654;
        // N s_6_1: branch s_6_0 b32 b7
        if s_6_0 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var branch_type:u32
        let s_8_0: u32 = fn_state.branch_type;
        // C s_8_1: const #1u : u32
        let s_8_1: u32 = 1;
        // D s_8_2: cmp-eq s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) == (s_8_1));
        // N s_8_3: branch s_8_2 b25 b9
        if s_8_2 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #6u : u32
        let s_10_0: u32 = 6;
        // D s_10_1: read-var branch_type:u32
        let s_10_1: u32 = fn_state.branch_type;
        // D s_10_2: cmp-eq s_10_0 s_10_1
        let s_10_2: bool = ((s_10_0) == (s_10_1));
        // D s_10_3: not s_10_2
        let s_10_3: bool = !s_10_2;
        // N s_10_4: branch s_10_3 b20 b11
        if s_10_3 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #102520u : u32
        let s_11_0: u32 = 102520;
        // D s_11_1: read-reg s_11_0:u8
        let s_11_1: bool = {
            let value = state.read_register::<bool>(s_11_0 as isize);
            tracer.read_register(s_11_0 as isize, value);
            value
        };
        // N s_11_2: branch s_11_1 b14 b12
        if s_11_1 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #1u : u8
        let s_12_0: u8 = 1;
        // C s_12_1: const #90912u : u32
        let s_12_1: u32 = 90912;
        // N s_12_2: write-reg s_12_1 <= s_12_0
        let s_12_2: () = {
            state.write_register::<u8>(s_12_1 as isize, s_12_0);
            tracer.write_register(s_12_1 as isize, s_12_0);
        };
        // N s_12_3: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: read-var target:u64
        let s_13_1: u64 = fn_state.target;
        // D s_13_2: cast zx s_13_1 -> bv
        let s_13_2: Bits = Bits::new(s_13_1 as u128, 64u16);
        // D s_13_3: read-var branch_type:u32
        let s_13_3: u32 = fn_state.branch_type;
        // D s_13_4: call BranchTo(s_13_2, s_13_3, s_13_0)
        let s_13_4: () = BranchTo(state, tracer, s_13_2, s_13_3, s_13_0);
        // N s_13_5: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #16s : i
        let s_14_0: i128 = 16;
        // D s_14_1: read-var n:i64
        let s_14_1: i64 = fn_state.n;
        // D s_14_2: cast zx s_14_1 -> i
        let s_14_2: i128 = (i128::try_from(s_14_1).unwrap());
        // D s_14_3: cmp-eq s_14_2 s_14_0
        let s_14_3: bool = ((s_14_2) == (s_14_0));
        // N s_14_4: branch s_14_3 b19 b15
        if s_14_3 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #17s : i
        let s_15_0: i128 = 17;
        // D s_15_1: read-var n:i64
        let s_15_1: i64 = fn_state.n;
        // D s_15_2: cast zx s_15_1 -> i
        let s_15_2: i128 = (i128::try_from(s_15_1).unwrap());
        // D s_15_3: cmp-eq s_15_2 s_15_0
        let s_15_3: bool = ((s_15_2) == (s_15_0));
        // D s_15_4: write-var gs#144659 <= s_15_3
        fn_state.gs_144659 = s_15_3;
        // N s_15_5: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#144659:u8
        let s_16_0: bool = fn_state.gs_144659;
        // N s_16_1: branch s_16_0 b18 b17
        if s_16_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #3u : u8
        let s_17_0: u8 = 3;
        // C s_17_1: const #90912u : u32
        let s_17_1: u32 = 90912;
        // N s_17_2: write-reg s_17_1 <= s_17_0
        let s_17_2: () = {
            state.write_register::<u8>(s_17_1 as isize, s_17_0);
            tracer.write_register(s_17_1 as isize, s_17_0);
        };
        // N s_17_3: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #1u : u8
        let s_18_0: u8 = 1;
        // C s_18_1: const #90912u : u32
        let s_18_1: u32 = 90912;
        // N s_18_2: write-reg s_18_1 <= s_18_0
        let s_18_2: () = {
            state.write_register::<u8>(s_18_1 as isize, s_18_0);
            tracer.write_register(s_18_1 as isize, s_18_0);
        };
        // N s_18_3: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #1u : u8
        let s_19_0: bool = true;
        // D s_19_1: write-var gs#144659 <= s_19_0
        fn_state.gs_144659 = s_19_0;
        // N s_19_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1u : u32
        let s_20_0: u32 = 1;
        // D s_20_1: read-var branch_type:u32
        let s_20_1: u32 = fn_state.branch_type;
        // D s_20_2: cmp-eq s_20_0 s_20_1
        let s_20_2: bool = ((s_20_0) == (s_20_1));
        // D s_20_3: not s_20_2
        let s_20_3: bool = !s_20_2;
        // N s_20_4: branch s_20_3 b22 b21
        if s_20_3 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #2u : u8
        let s_21_0: u8 = 2;
        // C s_21_1: const #90912u : u32
        let s_21_1: u32 = 90912;
        // N s_21_2: write-reg s_21_1 <= s_21_0
        let s_21_2: () = {
            state.write_register::<u8>(s_21_1 as isize, s_21_0);
            tracer.write_register(s_21_1 as isize, s_21_0);
        };
        // N s_21_3: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #4u : u32
        let s_22_0: u32 = 4;
        // D s_22_1: read-var branch_type:u32
        let s_22_1: u32 = fn_state.branch_type;
        // D s_22_2: cmp-eq s_22_0 s_22_1
        let s_22_2: bool = ((s_22_0) == (s_22_1));
        // D s_22_3: not s_22_2
        let s_22_3: bool = !s_22_2;
        // N s_22_4: branch s_22_3 b24 b23
        if s_22_3 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #0u : u8
        let s_23_0: u8 = 0;
        // C s_23_1: const #90912u : u32
        let s_23_1: u32 = 90912;
        // N s_23_2: write-reg s_23_1 <= s_23_0
        let s_23_2: () = {
            state.write_register::<u8>(s_23_1 as isize, s_23_0);
            tracer.write_register(s_23_1 as isize, s_23_0);
        };
        // N s_23_3: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_24_0: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #() : ()
        let s_25_0: () = ();
        // S s_25_1: call HaveGCS(s_25_0)
        let s_25_1: bool = HaveGCS(state, tracer, s_25_0);
        // N s_25_2: branch s_25_1 b31 b26
        if s_25_1 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #0u : u8
        let s_26_0: bool = false;
        // D s_26_1: write-var gs#144665 <= s_26_0
        fn_state.gs_144665 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#144665:u8
        let s_27_0: bool = fn_state.gs_144665;
        // N s_27_1: branch s_27_0 b30 b28
        if s_27_0 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_28_0: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #64s : i64
        let s_29_0: i64 = 64;
        // C s_29_1: const #() : ()
        let s_29_1: () = ();
        // S s_29_2: call PC_read(s_29_1)
        let s_29_2: u64 = PC_read(state, tracer, s_29_1);
        // C s_29_3: const #4s : i
        let s_29_3: i128 = 4;
        // S s_29_4: cast zx s_29_2 -> bv
        let s_29_4: Bits = Bits::new(s_29_2 as u128, 64u16);
        // C s_29_5: cast cvt s_29_3 -> bv
        let s_29_5: Bits = Bits::new(s_29_3 as u128, 128);
        // S s_29_6: add s_29_4 s_29_5
        let s_29_6: Bits = (s_29_4 + s_29_5);
        // S s_29_7: cast reint s_29_6 -> u64
        let s_29_7: u64 = (s_29_6.value() as u64);
        // C s_29_8: const #30s : i
        let s_29_8: i128 = 30;
        // S s_29_9: cast zx s_29_7 -> bv
        let s_29_9: Bits = Bits::new(s_29_7 as u128, 64u16);
        // S s_29_10: call X_set(s_29_8, s_29_0, s_29_9)
        let s_29_10: () = X_set(state, tracer, s_29_8, s_29_0, s_29_9);
        // N s_29_11: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #() : ()
        let s_30_0: () = ();
        // S s_30_1: call PC_read(s_30_0)
        let s_30_1: u64 = PC_read(state, tracer, s_30_0);
        // C s_30_2: const #4s : i
        let s_30_2: i128 = 4;
        // S s_30_3: cast zx s_30_1 -> bv
        let s_30_3: Bits = Bits::new(s_30_1 as u128, 64u16);
        // C s_30_4: cast cvt s_30_2 -> bv
        let s_30_4: Bits = Bits::new(s_30_2 as u128, 128);
        // S s_30_5: add s_30_3 s_30_4
        let s_30_5: Bits = (s_30_3 + s_30_4);
        // S s_30_6: cast reint s_30_5 -> u64
        let s_30_6: u64 = (s_30_5.value() as u64);
        // S s_30_7: call AddGCSRecord(s_30_6)
        let s_30_7: () = AddGCSRecord(state, tracer, s_30_6);
        // N s_30_8: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #16975u : u32
        let s_31_0: u32 = 16975;
        // D s_31_1: read-reg s_31_0:u8
        let s_31_1: u8 = {
            let value = state.read_register::<u8>(s_31_0 as isize);
            tracer.read_register(s_31_0 as isize, value);
            value
        };
        // D s_31_2: call GCSPCREnabled(s_31_1)
        let s_31_2: bool = GCSPCREnabled(state, tracer, s_31_1);
        // D s_31_3: write-var gs#144665 <= s_31_2
        fn_state.gs_144665 = s_31_2;
        // N s_31_4: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var pac:u8
        let s_32_0: bool = fn_state.pac;
        // D s_32_1: not s_32_0
        let s_32_1: bool = !s_32_0;
        // N s_32_2: branch s_32_1 b38 b33
        if s_32_1 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var use_key_a:u8
        let s_33_0: bool = fn_state.use_key_a;
        // N s_33_1: branch s_33_0 b37 b34
        if s_33_0 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #3u : u32
        let s_34_0: u32 = 3;
        // D s_34_1: write-var inst_type <= s_34_0
        fn_state.inst_type = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var target:u64
        let s_35_0: u64 = fn_state.target;
        // D s_35_1: read-var inst_type:u32
        let s_35_1: u32 = fn_state.inst_type;
        // D s_35_2: call LoadCheckGCSRecord(s_35_0, s_35_1)
        let s_35_2: u64 = LoadCheckGCSRecord(state, tracer, s_35_0, s_35_1);
        // D s_35_3: write-var target <= s_35_2
        fn_state.target = s_35_2;
        // N s_35_4: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #() : ()
        let s_36_0: () = ();
        // S s_36_1: call GetCurrentGCSPointer(s_36_0)
        let s_36_1: u64 = GetCurrentGCSPointer(state, tracer, s_36_0);
        // C s_36_2: const #8s : i
        let s_36_2: i128 = 8;
        // S s_36_3: cast zx s_36_1 -> bv
        let s_36_3: Bits = Bits::new(s_36_1 as u128, 64u16);
        // C s_36_4: cast cvt s_36_2 -> bv
        let s_36_4: Bits = Bits::new(s_36_2 as u128, 128);
        // S s_36_5: add s_36_3 s_36_4
        let s_36_5: Bits = (s_36_3 + s_36_4);
        // S s_36_6: cast reint s_36_5 -> u64
        let s_36_6: u64 = (s_36_5.value() as u64);
        // S s_36_7: call SetCurrentGCSPointer(s_36_6)
        let s_36_7: () = SetCurrentGCSPointer(state, tracer, s_36_6);
        // N s_36_8: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #2u : u32
        let s_37_0: u32 = 2;
        // D s_37_1: write-var inst_type <= s_37_0
        fn_state.inst_type = s_37_0;
        // N s_37_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #0u : u32
        let s_38_0: u32 = 0;
        // D s_38_1: write-var inst_type <= s_38_0
        fn_state.inst_type = s_38_0;
        // N s_38_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #16975u : u32
        let s_39_0: u32 = 16975;
        // D s_39_1: read-reg s_39_0:u8
        let s_39_1: u8 = {
            let value = state.read_register::<u8>(s_39_0 as isize);
            tracer.read_register(s_39_0 as isize, value);
            value
        };
        // D s_39_2: call GCSPCREnabled(s_39_1)
        let s_39_2: bool = GCSPCREnabled(state, tracer, s_39_1);
        // D s_39_3: write-var gs#144654 <= s_39_2
        fn_state.gs_144654 = s_39_2;
        // N s_39_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #() : ()
        let s_40_0: () = ();
        // S s_40_1: call HaveGCS(s_40_0)
        let s_40_1: bool = HaveGCS(state, tracer, s_40_0);
        // D s_40_2: write-var gs#144653 <= s_40_1
        fn_state.gs_144653 = s_40_1;
        // N s_40_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var source_is_sp:u8
        let s_41_0: bool = fn_state.source_is_sp;
        // N s_41_1: branch s_41_0 b48 b42
        if s_41_0 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #64s : i64
        let s_42_0: i64 = 64;
        // D s_42_1: read-var m:i64
        let s_42_1: i64 = fn_state.m;
        // D s_42_2: cast zx s_42_1 -> i
        let s_42_2: i128 = (i128::try_from(s_42_1).unwrap());
        // D s_42_3: call X_read(s_42_2, s_42_0)
        let s_42_3: Bits = X_read(state, tracer, s_42_2, s_42_0);
        // D s_42_4: cast reint s_42_3 -> u64
        let s_42_4: u64 = (s_42_3.value() as u64);
        // D s_42_5: write-var modifier <= s_42_4
        fn_state.modifier = s_42_4;
        // N s_42_6: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var use_key_a:u8
        let s_43_0: bool = fn_state.use_key_a;
        // N s_43_1: branch s_43_0 b46 b44
        if s_43_0 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var target:u64
        let s_44_0: u64 = fn_state.target;
        // D s_44_1: read-var modifier:u64
        let s_44_1: u64 = fn_state.modifier;
        // C s_44_2: const #1u : u8
        let s_44_2: bool = true;
        // D s_44_3: call AuthIB(s_44_0, s_44_1, s_44_2)
        let s_44_3: u64 = AuthIB(state, tracer, s_44_0, s_44_1, s_44_2);
        // D s_44_4: write-var target <= s_44_3
        fn_state.target = s_44_3;
        // N s_44_5: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_45_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var target:u64
        let s_46_0: u64 = fn_state.target;
        // D s_46_1: read-var modifier:u64
        let s_46_1: u64 = fn_state.modifier;
        // C s_46_2: const #1u : u8
        let s_46_2: bool = true;
        // D s_46_3: call AuthIA(s_46_0, s_46_1, s_46_2)
        let s_46_3: u64 = AuthIA(state, tracer, s_46_0, s_46_1, s_46_2);
        // D s_46_4: write-var target <= s_46_3
        fn_state.target = s_46_3;
        // N s_46_5: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_47_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #() : ()
        let s_48_0: () = ();
        // S s_48_1: call SP_read(s_48_0)
        let s_48_1: u64 = SP_read(state, tracer, s_48_0);
        // D s_48_2: write-var modifier <= s_48_1
        fn_state.modifier = s_48_1;
        // N s_48_3: jump b43
        return block_43(state, tracer, fn_state);
    }
}
