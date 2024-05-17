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
use EL2Enabled::*;
use u__UNKNOWN_bits::*;
use R_set::*;
use ELUsingAArch32::*;
use GetNreg::*;
use HaveFGTExt::*;
use u_get_HCR_EL2_Type_TID3::*;
use AArch32_CheckCP15InstrCoarseTraps::*;
use Zeros::*;
use AArch64_AArch32SystemAccessTrap::*;
use PLOfEL::*;
use u__IMPDEF_boolean::*;
use AArch32_SystemAccessTrap::*;
use common::*;
pub fn AArch32_UnallocatedSysRegAccess<T: Tracer>(
    state: &mut State,
    tracer: &T,
    coproc: u8,
    opc1: u8,
    CRn: u8,
    opc2: u8,
    CRm: u8,
    read: bool,
    t: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_142665: bool,
        gs_142644: bool,
        gs_142642: bool,
        gs_142658: bool,
        gs_142623: bool,
        gs_142646: bool,
        gs_142633: bool,
        gs_142656: bool,
        gs_142631: bool,
        gs_142641: bool,
        gs_142647: bool,
        gs_142655: bool,
        gs_142630: bool,
        gs_142612: bool,
        gs_142657: bool,
        gs_142625: bool,
        gs_142629: bool,
        gs_142610: bool,
        gs_142621: bool,
        gs_142607: bool,
        gs_142669: bool,
        gs_142648: bool,
        tid3_trap: bool,
        gs_142638: bool,
        gs_142643: bool,
        gs_142634: bool,
        gs_142606: bool,
        gs_142628: bool,
        gs_142622: bool,
        gs_142626: bool,
        gs_142614: bool,
        gs_142624: bool,
        gs_142608: bool,
        gs_142636: bool,
        gs_142645: bool,
        coproc: u8,
        opc1: u8,
        CRn: u8,
        opc2: u8,
        CRm: u8,
        read: bool,
        t: i128,
    }
    let fn_state = FunctionState {
        coproc,
        opc1,
        CRn,
        opc2,
        CRm,
        read,
        t,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #0u : u8
        let s_0_0: bool = false;
        // D s_0_1: write-var tid3_trap <= s_0_0
        fn_state.tid3_trap = s_0_0;
        // C s_0_2: const #16975u : u32
        let s_0_2: u32 = 16975;
        // D s_0_3: read-reg s_0_2:u8
        let s_0_3: u8 = {
            let value = state.read_register::<u8>(s_0_2 as isize);
            tracer.read_register(s_0_2 as isize, value);
            value
        };
        // D s_0_4: call PLOfEL(s_0_3)
        let s_0_4: u32 = PLOfEL(state, tracer, s_0_3);
        // C s_0_5: const #2u : u32
        let s_0_5: u32 = 2;
        // D s_0_6: cmp-eq s_0_4 s_0_5
        let s_0_6: bool = ((s_0_4) == (s_0_5));
        // N s_0_7: branch s_0_6 b134 b1
        if s_0_6 {
            return block_134(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#142606 <= s_1_0
        fn_state.gs_142606 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#142606:u8
        let s_2_0: bool = fn_state.gs_142606;
        // N s_2_1: branch s_2_0 b133 b3
        if s_2_0 {
            return block_133(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#142607 <= s_3_0
        fn_state.gs_142607 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#142607:u8
        let s_4_0: bool = fn_state.gs_142607;
        // N s_4_1: branch s_4_0 b132 b5
        if s_4_0 {
            return block_132(state, tracer, fn_state);
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
        // D s_5_1: write-var gs#142608 <= s_5_0
        fn_state.gs_142608 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#142608:u8
        let s_6_0: bool = fn_state.gs_142608;
        // N s_6_1: branch s_6_0 b128 b7
        if s_6_0 {
            return block_128(state, tracer, fn_state);
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
        // D s_8_0: read-var tid3_trap:u8
        let s_8_0: bool = fn_state.tid3_trap;
        // N s_8_1: branch s_8_0 b127 b9
        if s_8_0 {
            return block_127(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#142610 <= s_9_0
        fn_state.gs_142610 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#142610:u8
        let s_10_0: bool = fn_state.gs_142610;
        // N s_10_1: branch s_10_0 b126 b11
        if s_10_0 {
            return block_126(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#142612 <= s_11_0
        fn_state.gs_142612 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#142612:u8
        let s_12_0: bool = fn_state.gs_142612;
        // N s_12_1: branch s_12_0 b125 b13
        if s_12_0 {
            return block_125(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var gs#142614 <= s_13_0
        fn_state.gs_142614 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#142614:u8
        let s_14_0: bool = fn_state.gs_142614;
        // N s_14_1: branch s_14_0 b109 b15
        if s_14_0 {
            return block_109(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#142626 <= s_15_0
        fn_state.gs_142626 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#142626:u8
        let s_16_0: bool = fn_state.gs_142626;
        // N s_16_1: branch s_16_0 b106 b17
        if s_16_0 {
            return block_106(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var CRn:u8
        let s_18_0: u8 = fn_state.CRn;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 4u16);
        // D s_18_2: cast zx s_18_1 -> i
        let s_18_2: i128 = (s_18_1.value() as i128);
        // D s_18_3: cast reint s_18_2 -> i64
        let s_18_3: i64 = (s_18_2 as i64);
        // C s_18_4: const #() : ()
        let s_18_4: () = ();
        // S s_18_5: call GetNreg(s_18_4)
        let s_18_5: i128 = GetNreg(state, tracer, s_18_4);
        // D s_18_6: read-var CRm:u8
        let s_18_6: u8 = fn_state.CRm;
        // D s_18_7: cast zx s_18_6 -> bv
        let s_18_7: Bits = Bits::new(s_18_6 as u128, 4u16);
        // D s_18_8: cast zx s_18_7 -> i
        let s_18_8: i128 = (s_18_7.value() as i128);
        // D s_18_9: cast reint s_18_8 -> i64
        let s_18_9: i64 = (s_18_8 as i64);
        // D s_18_10: cast zx s_18_3 -> i
        let s_18_10: i128 = (i128::try_from(s_18_3).unwrap());
        // D s_18_11: cast zx s_18_9 -> i
        let s_18_11: i128 = (i128::try_from(s_18_9).unwrap());
        // D s_18_12: call AArch32_CheckCP15InstrCoarseTraps(s_18_10, s_18_5, s_18_11)
        let s_18_12: () = AArch32_CheckCP15InstrCoarseTraps(
            state,
            tracer,
            s_18_10,
            s_18_5,
            s_18_11,
        );
        // N s_18_13: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var coproc:u8
        let s_19_0: u8 = fn_state.coproc;
        // D s_19_1: cast zx s_19_0 -> bv
        let s_19_1: Bits = Bits::new(s_19_0 as u128, 4u16);
        // D s_19_2: cast zx s_19_1 -> i
        let s_19_2: i128 = (s_19_1.value() as i128);
        // D s_19_3: cast reint s_19_2 -> i64
        let s_19_3: i64 = (s_19_2 as i64);
        // C s_19_4: const #15s : i
        let s_19_4: i128 = 15;
        // D s_19_5: cast zx s_19_3 -> i
        let s_19_5: i128 = (i128::try_from(s_19_3).unwrap());
        // D s_19_6: cmp-eq s_19_5 s_19_4
        let s_19_6: bool = ((s_19_5) == (s_19_4));
        // N s_19_7: branch s_19_6 b105 b20
        if s_19_6 {
            return block_105(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // D s_20_1: write-var gs#142628 <= s_20_0
        fn_state.gs_142628 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#142628:u8
        let s_21_0: bool = fn_state.gs_142628;
        // N s_21_1: branch s_21_0 b104 b22
        if s_21_0 {
            return block_104(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #0u : u8
        let s_22_0: bool = false;
        // D s_22_1: write-var gs#142629 <= s_22_0
        fn_state.gs_142629 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#142629:u8
        let s_23_0: bool = fn_state.gs_142629;
        // N s_23_1: branch s_23_0 b103 b24
        if s_23_0 {
            return block_103(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #0u : u8
        let s_24_0: bool = false;
        // D s_24_1: write-var gs#142630 <= s_24_0
        fn_state.gs_142630 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#142630:u8
        let s_25_0: bool = fn_state.gs_142630;
        // N s_25_1: branch s_25_0 b102 b26
        if s_25_0 {
            return block_102(state, tracer, fn_state);
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
        // D s_26_1: write-var gs#142631 <= s_26_0
        fn_state.gs_142631 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#142631:u8
        let s_27_0: bool = fn_state.gs_142631;
        // N s_27_1: branch s_27_0 b96 b28
        if s_27_0 {
            return block_96(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var coproc:u8
        let s_28_0: u8 = fn_state.coproc;
        // D s_28_1: cast zx s_28_0 -> bv
        let s_28_1: Bits = Bits::new(s_28_0 as u128, 4u16);
        // D s_28_2: cast zx s_28_1 -> i
        let s_28_2: i128 = (s_28_1.value() as i128);
        // D s_28_3: cast reint s_28_2 -> i64
        let s_28_3: i64 = (s_28_2 as i64);
        // C s_28_4: const #15s : i
        let s_28_4: i128 = 15;
        // D s_28_5: cast zx s_28_3 -> i
        let s_28_5: i128 = (i128::try_from(s_28_3).unwrap());
        // D s_28_6: cmp-eq s_28_5 s_28_4
        let s_28_6: bool = ((s_28_5) == (s_28_4));
        // N s_28_7: branch s_28_6 b95 b29
        if s_28_6 {
            return block_95(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #0u : u8
        let s_29_0: bool = false;
        // D s_29_1: write-var gs#142633 <= s_29_0
        fn_state.gs_142633 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#142633:u8
        let s_30_0: bool = fn_state.gs_142633;
        // N s_30_1: branch s_30_0 b94 b31
        if s_30_0 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #0u : u8
        let s_31_0: bool = false;
        // D s_31_1: write-var gs#142634 <= s_31_0
        fn_state.gs_142634 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#142634:u8
        let s_32_0: bool = fn_state.gs_142634;
        // N s_32_1: branch s_32_0 b93 b33
        if s_32_0 {
            return block_93(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #0u : u8
        let s_33_0: bool = false;
        // D s_33_1: write-var gs#142636 <= s_33_0
        fn_state.gs_142636 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#142636:u8
        let s_34_0: bool = fn_state.gs_142636;
        // N s_34_1: branch s_34_0 b87 b35
        if s_34_0 {
            return block_87(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var read:u8
        let s_35_0: bool = fn_state.read;
        // D s_35_1: not s_35_0
        let s_35_1: bool = !s_35_0;
        // N s_35_2: branch s_35_1 b86 b36
        if s_35_1 {
            return block_86(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var coproc:u8
        let s_36_0: u8 = fn_state.coproc;
        // D s_36_1: cast zx s_36_0 -> bv
        let s_36_1: Bits = Bits::new(s_36_0 as u128, 4u16);
        // D s_36_2: cast zx s_36_1 -> i
        let s_36_2: i128 = (s_36_1.value() as i128);
        // D s_36_3: cast reint s_36_2 -> i64
        let s_36_3: i64 = (s_36_2 as i64);
        // C s_36_4: const #15s : i
        let s_36_4: i128 = 15;
        // D s_36_5: cast zx s_36_3 -> i
        let s_36_5: i128 = (i128::try_from(s_36_3).unwrap());
        // D s_36_6: cmp-eq s_36_5 s_36_4
        let s_36_6: bool = ((s_36_5) == (s_36_4));
        // N s_36_7: branch s_36_6 b85 b37
        if s_36_6 {
            return block_85(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #0u : u8
        let s_37_0: bool = false;
        // D s_37_1: write-var gs#142638 <= s_37_0
        fn_state.gs_142638 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#142638:u8
        let s_38_0: bool = fn_state.gs_142638;
        // N s_38_1: branch s_38_0 b40 b39
        if s_38_0 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_39_0: panic
        panic!("{:?}", ());
        // N s_39_1: return
        return;
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var opc1:u8
        let s_40_0: u8 = fn_state.opc1;
        // D s_40_1: cast zx s_40_0 -> bv
        let s_40_1: Bits = Bits::new(s_40_0 as u128, 3u16);
        // D s_40_2: cast zx s_40_1 -> i
        let s_40_2: i128 = (s_40_1.value() as i128);
        // D s_40_3: cast reint s_40_2 -> i64
        let s_40_3: i64 = (s_40_2 as i64);
        // C s_40_4: const #0s : i
        let s_40_4: i128 = 0;
        // D s_40_5: cast zx s_40_3 -> i
        let s_40_5: i128 = (i128::try_from(s_40_3).unwrap());
        // D s_40_6: cmp-gt s_40_5 s_40_4
        let s_40_6: bool = ((s_40_5) > (s_40_4));
        // N s_40_7: branch s_40_6 b84 b41
        if s_40_6 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var CRm:u8
        let s_41_0: u8 = fn_state.CRm;
        // D s_41_1: cast zx s_41_0 -> bv
        let s_41_1: Bits = Bits::new(s_41_0 as u128, 4u16);
        // C s_41_2: const #3u : u8
        let s_41_2: u8 = 3;
        // C s_41_3: cast zx s_41_2 -> bv
        let s_41_3: Bits = Bits::new(s_41_2 as u128, 4u16);
        // D s_41_4: cmp-eq s_41_1 s_41_3
        let s_41_4: bool = ((s_41_1) == (s_41_3));
        // N s_41_5: branch s_41_4 b77 b42
        if s_41_4 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #0u : u8
        let s_42_0: bool = false;
        // D s_42_1: write-var gs#142643 <= s_42_0
        fn_state.gs_142643 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#142643:u8
        let s_43_0: bool = fn_state.gs_142643;
        // N s_43_1: branch s_43_0 b76 b44
        if s_43_0 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var CRm:u8
        let s_44_0: u8 = fn_state.CRm;
        // D s_44_1: cast zx s_44_0 -> bv
        let s_44_1: Bits = Bits::new(s_44_0 as u128, 4u16);
        // C s_44_2: const #4u : u8
        let s_44_2: u8 = 4;
        // C s_44_3: cast zx s_44_2 -> bv
        let s_44_3: Bits = Bits::new(s_44_2 as u128, 4u16);
        // D s_44_4: cmp-eq s_44_1 s_44_3
        let s_44_4: bool = ((s_44_1) == (s_44_3));
        // N s_44_5: branch s_44_4 b75 b45
        if s_44_4 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var CRm:u8
        let s_45_0: u8 = fn_state.CRm;
        // D s_45_1: cast zx s_45_0 -> bv
        let s_45_1: Bits = Bits::new(s_45_0 as u128, 4u16);
        // C s_45_2: const #6u : u8
        let s_45_2: u8 = 6;
        // C s_45_3: cast zx s_45_2 -> bv
        let s_45_3: Bits = Bits::new(s_45_2 as u128, 4u16);
        // D s_45_4: cmp-eq s_45_1 s_45_3
        let s_45_4: bool = ((s_45_1) == (s_45_3));
        // N s_45_5: branch s_45_4 b74 b46
        if s_45_4 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var CRm:u8
        let s_46_0: u8 = fn_state.CRm;
        // D s_46_1: cast zx s_46_0 -> bv
        let s_46_1: Bits = Bits::new(s_46_0 as u128, 4u16);
        // C s_46_2: const #7u : u8
        let s_46_2: u8 = 7;
        // C s_46_3: cast zx s_46_2 -> bv
        let s_46_3: Bits = Bits::new(s_46_2 as u128, 4u16);
        // D s_46_4: cmp-eq s_46_1 s_46_3
        let s_46_4: bool = ((s_46_1) == (s_46_3));
        // D s_46_5: write-var gs#142644 <= s_46_4
        fn_state.gs_142644 = s_46_4;
        // N s_46_6: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#142644:u8
        let s_47_0: bool = fn_state.gs_142644;
        // D s_47_1: write-var gs#142645 <= s_47_0
        fn_state.gs_142645 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#142645:u8
        let s_48_0: bool = fn_state.gs_142645;
        // N s_48_1: branch s_48_0 b67 b49
        if s_48_0 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #0u : u8
        let s_49_0: bool = false;
        // D s_49_1: write-var gs#142648 <= s_49_0
        fn_state.gs_142648 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#142648:u8
        let s_50_0: bool = fn_state.gs_142648;
        // N s_50_1: branch s_50_0 b66 b51
        if s_50_0 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var CRm:u8
        let s_51_0: u8 = fn_state.CRm;
        // D s_51_1: cast zx s_51_0 -> bv
        let s_51_1: Bits = Bits::new(s_51_0 as u128, 4u16);
        // C s_51_2: const #5u : u8
        let s_51_2: u8 = 5;
        // C s_51_3: cast zx s_51_2 -> bv
        let s_51_3: Bits = Bits::new(s_51_2 as u128, 4u16);
        // D s_51_4: cmp-eq s_51_1 s_51_3
        let s_51_4: bool = ((s_51_1) == (s_51_3));
        // N s_51_5: branch s_51_4 b56 b52
        if s_51_4 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #0u : u8
        let s_52_0: bool = false;
        // D s_52_1: write-var gs#142658 <= s_52_0
        fn_state.gs_142658 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#142658:u8
        let s_53_0: bool = fn_state.gs_142658;
        // N s_53_1: branch s_53_0 b55 b54
        if s_53_0 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #32s : i
        let s_54_0: i128 = 32;
        // S s_54_1: call Zeros(s_54_0)
        let s_54_1: Bits = Zeros(state, tracer, s_54_0);
        // S s_54_2: cast reint s_54_1 -> u32
        let s_54_2: u32 = (s_54_1.value() as u32);
        // D s_54_3: read-var t:i
        let s_54_3: i128 = fn_state.t;
        // D s_54_4: call R_set(s_54_3, s_54_2)
        let s_54_4: () = R_set(state, tracer, s_54_3, s_54_2);
        // N s_54_5: return
        return;
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #32s : i64
        let s_55_0: i64 = 32;
        // C s_55_1: cast zx s_55_0 -> i
        let s_55_1: i128 = (i128::try_from(s_55_0).unwrap());
        // S s_55_2: call __UNKNOWN_bits(s_55_1)
        let s_55_2: Bits = u__UNKNOWN_bits(state, tracer, s_55_1);
        // S s_55_3: cast reint s_55_2 -> u32
        let s_55_3: u32 = (s_55_2.value() as u32);
        // D s_55_4: read-var t:i
        let s_55_4: i128 = fn_state.t;
        // D s_55_5: call R_set(s_55_4, s_55_3)
        let s_55_5: () = R_set(state, tracer, s_55_4, s_55_3);
        // N s_55_6: return
        return;
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var opc2:u8
        let s_56_0: u8 = fn_state.opc2;
        // D s_56_1: cast zx s_56_0 -> bv
        let s_56_1: Bits = Bits::new(s_56_0 as u128, 3u16);
        // C s_56_2: const #0u : u8
        let s_56_2: u8 = 0;
        // C s_56_3: cast zx s_56_2 -> bv
        let s_56_3: Bits = Bits::new(s_56_2 as u128, 3u16);
        // D s_56_4: cmp-eq s_56_1 s_56_3
        let s_56_4: bool = ((s_56_1) == (s_56_3));
        // N s_56_5: branch s_56_4 b65 b57
        if s_56_4 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var opc2:u8
        let s_57_0: u8 = fn_state.opc2;
        // D s_57_1: cast zx s_57_0 -> bv
        let s_57_1: Bits = Bits::new(s_57_0 as u128, 3u16);
        // C s_57_2: const #1u : u8
        let s_57_2: u8 = 1;
        // C s_57_3: cast zx s_57_2 -> bv
        let s_57_3: Bits = Bits::new(s_57_2 as u128, 3u16);
        // D s_57_4: cmp-eq s_57_1 s_57_3
        let s_57_4: bool = ((s_57_1) == (s_57_3));
        // N s_57_5: branch s_57_4 b64 b58
        if s_57_4 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var opc2:u8
        let s_58_0: u8 = fn_state.opc2;
        // D s_58_1: cast zx s_58_0 -> bv
        let s_58_1: Bits = Bits::new(s_58_0 as u128, 3u16);
        // C s_58_2: const #4u : u8
        let s_58_2: u8 = 4;
        // C s_58_3: cast zx s_58_2 -> bv
        let s_58_3: Bits = Bits::new(s_58_2 as u128, 3u16);
        // D s_58_4: cmp-eq s_58_1 s_58_3
        let s_58_4: bool = ((s_58_1) == (s_58_3));
        // N s_58_5: branch s_58_4 b63 b59
        if s_58_4 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var opc2:u8
        let s_59_0: u8 = fn_state.opc2;
        // D s_59_1: cast zx s_59_0 -> bv
        let s_59_1: Bits = Bits::new(s_59_0 as u128, 3u16);
        // C s_59_2: const #5u : u8
        let s_59_2: u8 = 5;
        // C s_59_3: cast zx s_59_2 -> bv
        let s_59_3: Bits = Bits::new(s_59_2 as u128, 3u16);
        // D s_59_4: cmp-eq s_59_1 s_59_3
        let s_59_4: bool = ((s_59_1) == (s_59_3));
        // D s_59_5: write-var gs#142655 <= s_59_4
        fn_state.gs_142655 = s_59_4;
        // N s_59_6: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#142655:u8
        let s_60_0: bool = fn_state.gs_142655;
        // D s_60_1: write-var gs#142656 <= s_60_0
        fn_state.gs_142656 = s_60_0;
        // N s_60_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var gs#142656:u8
        let s_61_0: bool = fn_state.gs_142656;
        // D s_61_1: write-var gs#142657 <= s_61_0
        fn_state.gs_142657 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#142657:u8
        let s_62_0: bool = fn_state.gs_142657;
        // D s_62_1: write-var gs#142658 <= s_62_0
        fn_state.gs_142658 = s_62_0;
        // N s_62_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #1u : u8
        let s_63_0: bool = true;
        // D s_63_1: write-var gs#142655 <= s_63_0
        fn_state.gs_142655 = s_63_0;
        // N s_63_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #1u : u8
        let s_64_0: bool = true;
        // D s_64_1: write-var gs#142656 <= s_64_0
        fn_state.gs_142656 = s_64_0;
        // N s_64_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #1u : u8
        let s_65_0: bool = true;
        // D s_65_1: write-var gs#142657 <= s_65_0
        fn_state.gs_142657 = s_65_0;
        // N s_65_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #32s : i64
        let s_66_0: i64 = 32;
        // C s_66_1: cast zx s_66_0 -> i
        let s_66_1: i128 = (i128::try_from(s_66_0).unwrap());
        // S s_66_2: call __UNKNOWN_bits(s_66_1)
        let s_66_2: Bits = u__UNKNOWN_bits(state, tracer, s_66_1);
        // S s_66_3: cast reint s_66_2 -> u32
        let s_66_3: u32 = (s_66_2.value() as u32);
        // D s_66_4: read-var t:i
        let s_66_4: i128 = fn_state.t;
        // D s_66_5: call R_set(s_66_4, s_66_3)
        let s_66_5: () = R_set(state, tracer, s_66_4, s_66_3);
        // N s_66_6: return
        return;
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var opc2:u8
        let s_67_0: u8 = fn_state.opc2;
        // D s_67_1: cast zx s_67_0 -> bv
        let s_67_1: Bits = Bits::new(s_67_0 as u128, 3u16);
        // C s_67_2: const #0u : u8
        let s_67_2: u8 = 0;
        // C s_67_3: cast zx s_67_2 -> bv
        let s_67_3: Bits = Bits::new(s_67_2 as u128, 3u16);
        // D s_67_4: cmp-eq s_67_1 s_67_3
        let s_67_4: bool = ((s_67_1) == (s_67_3));
        // N s_67_5: branch s_67_4 b73 b68
        if s_67_4 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_68(state, tracer, fn_state);
        };
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var opc2:u8
        let s_68_0: u8 = fn_state.opc2;
        // D s_68_1: cast zx s_68_0 -> bv
        let s_68_1: Bits = Bits::new(s_68_0 as u128, 3u16);
        // C s_68_2: const #1u : u8
        let s_68_2: u8 = 1;
        // C s_68_3: cast zx s_68_2 -> bv
        let s_68_3: Bits = Bits::new(s_68_2 as u128, 3u16);
        // D s_68_4: cmp-eq s_68_1 s_68_3
        let s_68_4: bool = ((s_68_1) == (s_68_3));
        // N s_68_5: branch s_68_4 b72 b69
        if s_68_4 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var opc2:u8
        let s_69_0: u8 = fn_state.opc2;
        // D s_69_1: cast zx s_69_0 -> bv
        let s_69_1: Bits = Bits::new(s_69_0 as u128, 3u16);
        // C s_69_2: const #7u : u8
        let s_69_2: u8 = 7;
        // C s_69_3: cast zx s_69_2 -> bv
        let s_69_3: Bits = Bits::new(s_69_2 as u128, 3u16);
        // D s_69_4: cmp-eq s_69_1 s_69_3
        let s_69_4: bool = ((s_69_1) == (s_69_3));
        // D s_69_5: write-var gs#142646 <= s_69_4
        fn_state.gs_142646 = s_69_4;
        // N s_69_6: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var gs#142646:u8
        let s_70_0: bool = fn_state.gs_142646;
        // D s_70_1: write-var gs#142647 <= s_70_0
        fn_state.gs_142647 = s_70_0;
        // N s_70_2: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var gs#142647:u8
        let s_71_0: bool = fn_state.gs_142647;
        // D s_71_1: write-var gs#142648 <= s_71_0
        fn_state.gs_142648 = s_71_0;
        // N s_71_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #1u : u8
        let s_72_0: bool = true;
        // D s_72_1: write-var gs#142646 <= s_72_0
        fn_state.gs_142646 = s_72_0;
        // N s_72_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #1u : u8
        let s_73_0: bool = true;
        // D s_73_1: write-var gs#142647 <= s_73_0
        fn_state.gs_142647 = s_73_0;
        // N s_73_2: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #1u : u8
        let s_74_0: bool = true;
        // D s_74_1: write-var gs#142644 <= s_74_0
        fn_state.gs_142644 = s_74_0;
        // N s_74_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #1u : u8
        let s_75_0: bool = true;
        // D s_75_1: write-var gs#142645 <= s_75_0
        fn_state.gs_142645 = s_75_0;
        // N s_75_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #32s : i64
        let s_76_0: i64 = 32;
        // C s_76_1: cast zx s_76_0 -> i
        let s_76_1: i128 = (i128::try_from(s_76_0).unwrap());
        // S s_76_2: call __UNKNOWN_bits(s_76_1)
        let s_76_2: Bits = u__UNKNOWN_bits(state, tracer, s_76_1);
        // S s_76_3: cast reint s_76_2 -> u32
        let s_76_3: u32 = (s_76_2.value() as u32);
        // D s_76_4: read-var t:i
        let s_76_4: i128 = fn_state.t;
        // D s_76_5: call R_set(s_76_4, s_76_3)
        let s_76_5: () = R_set(state, tracer, s_76_4, s_76_3);
        // N s_76_6: return
        return;
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var opc2:u8
        let s_77_0: u8 = fn_state.opc2;
        // D s_77_1: cast zx s_77_0 -> bv
        let s_77_1: Bits = Bits::new(s_77_0 as u128, 3u16);
        // C s_77_2: const #0u : u8
        let s_77_2: u8 = 0;
        // C s_77_3: cast zx s_77_2 -> bv
        let s_77_3: Bits = Bits::new(s_77_2 as u128, 3u16);
        // D s_77_4: cmp-eq s_77_1 s_77_3
        let s_77_4: bool = ((s_77_1) == (s_77_3));
        // N s_77_5: branch s_77_4 b83 b78
        if s_77_4 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_78(state, tracer, fn_state);
        };
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var opc2:u8
        let s_78_0: u8 = fn_state.opc2;
        // D s_78_1: cast zx s_78_0 -> bv
        let s_78_1: Bits = Bits::new(s_78_0 as u128, 3u16);
        // C s_78_2: const #1u : u8
        let s_78_2: u8 = 1;
        // C s_78_3: cast zx s_78_2 -> bv
        let s_78_3: Bits = Bits::new(s_78_2 as u128, 3u16);
        // D s_78_4: cmp-eq s_78_1 s_78_3
        let s_78_4: bool = ((s_78_1) == (s_78_3));
        // N s_78_5: branch s_78_4 b82 b79
        if s_78_4 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var opc2:u8
        let s_79_0: u8 = fn_state.opc2;
        // D s_79_1: cast zx s_79_0 -> bv
        let s_79_1: Bits = Bits::new(s_79_0 as u128, 3u16);
        // C s_79_2: const #2u : u8
        let s_79_2: u8 = 2;
        // C s_79_3: cast zx s_79_2 -> bv
        let s_79_3: Bits = Bits::new(s_79_2 as u128, 3u16);
        // D s_79_4: cmp-eq s_79_1 s_79_3
        let s_79_4: bool = ((s_79_1) == (s_79_3));
        // D s_79_5: write-var gs#142641 <= s_79_4
        fn_state.gs_142641 = s_79_4;
        // N s_79_6: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_80_0: read-var gs#142641:u8
        let s_80_0: bool = fn_state.gs_142641;
        // D s_80_1: write-var gs#142642 <= s_80_0
        fn_state.gs_142642 = s_80_0;
        // N s_80_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var gs#142642:u8
        let s_81_0: bool = fn_state.gs_142642;
        // D s_81_1: write-var gs#142643 <= s_81_0
        fn_state.gs_142643 = s_81_0;
        // N s_81_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #1u : u8
        let s_82_0: bool = true;
        // D s_82_1: write-var gs#142641 <= s_82_0
        fn_state.gs_142641 = s_82_0;
        // N s_82_2: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #1u : u8
        let s_83_0: bool = true;
        // D s_83_1: write-var gs#142642 <= s_83_0
        fn_state.gs_142642 = s_83_0;
        // N s_83_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_84_0: panic
        panic!("{:?}", ());
        // N s_84_1: return
        return;
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_85_0: read-var CRn:u8
        let s_85_0: u8 = fn_state.CRn;
        // D s_85_1: cast zx s_85_0 -> bv
        let s_85_1: Bits = Bits::new(s_85_0 as u128, 4u16);
        // C s_85_2: const #0u : u8
        let s_85_2: u8 = 0;
        // C s_85_3: cast zx s_85_2 -> bv
        let s_85_3: Bits = Bits::new(s_85_2 as u128, 4u16);
        // D s_85_4: cmp-eq s_85_1 s_85_3
        let s_85_4: bool = ((s_85_1) == (s_85_3));
        // D s_85_5: write-var gs#142638 <= s_85_4
        fn_state.gs_142638 = s_85_4;
        // N s_85_6: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_86_0: panic
        panic!("{:?}", ());
        // N s_86_1: return
        return;
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #16975u : u32
        let s_87_0: u32 = 16975;
        // D s_87_1: read-reg s_87_0:u8
        let s_87_1: u8 = {
            let value = state.read_register::<u8>(s_87_0 as isize);
            tracer.read_register(s_87_0 as isize, value);
            value
        };
        // D s_87_2: cast zx s_87_1 -> bv
        let s_87_2: Bits = Bits::new(s_87_1 as u128, 2u16);
        // C s_87_3: const #448u : u32
        let s_87_3: u32 = 448;
        // D s_87_4: read-reg s_87_3:u8
        let s_87_4: u8 = {
            let value = state.read_register::<u8>(s_87_3 as isize);
            tracer.read_register(s_87_3 as isize, value);
            value
        };
        // D s_87_5: cast zx s_87_4 -> bv
        let s_87_5: Bits = Bits::new(s_87_4 as u128, 2u16);
        // D s_87_6: cmp-ne s_87_2 s_87_5
        let s_87_6: bool = ((s_87_2) != (s_87_5));
        // N s_87_7: branch s_87_6 b92 b88
        if s_87_6 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_88(state, tracer, fn_state);
        };
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #0u : u8
        let s_88_0: bool = false;
        // D s_88_1: write-var gs#142665 <= s_88_0
        fn_state.gs_142665 = s_88_0;
        // N s_88_2: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_89_0: read-var gs#142665:u8
        let s_89_0: bool = fn_state.gs_142665;
        // N s_89_1: branch s_89_0 b91 b90
        if s_89_0 {
            return block_91(state, tracer, fn_state);
        } else {
            return block_90(state, tracer, fn_state);
        };
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_90_0: panic
        panic!("{:?}", ());
        // N s_90_1: return
        return;
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #32s : i64
        let s_91_0: i64 = 32;
        // C s_91_1: cast zx s_91_0 -> i
        let s_91_1: i128 = (i128::try_from(s_91_0).unwrap());
        // S s_91_2: call __UNKNOWN_bits(s_91_1)
        let s_91_2: Bits = u__UNKNOWN_bits(state, tracer, s_91_1);
        // S s_91_3: cast reint s_91_2 -> u32
        let s_91_3: u32 = (s_91_2.value() as u32);
        // D s_91_4: read-var t:i
        let s_91_4: i128 = fn_state.t;
        // D s_91_5: call R_set(s_91_4, s_91_3)
        let s_91_5: () = R_set(state, tracer, s_91_4, s_91_3);
        // N s_91_6: return
        return;
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var read:u8
        let s_92_0: bool = fn_state.read;
        // D s_92_1: write-var gs#142665 <= s_92_0
        fn_state.gs_142665 = s_92_0;
        // N s_92_2: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_93_0: read-var CRm:u8
        let s_93_0: u8 = fn_state.CRm;
        // D s_93_1: cast zx s_93_0 -> bv
        let s_93_1: Bits = Bits::new(s_93_0 as u128, 4u16);
        // D s_93_2: cast zx s_93_1 -> i
        let s_93_2: i128 = (s_93_1.value() as i128);
        // D s_93_3: cast reint s_93_2 -> i64
        let s_93_3: i64 = (s_93_2 as i64);
        // C s_93_4: const #3s : i
        let s_93_4: i128 = 3;
        // D s_93_5: cast zx s_93_3 -> i
        let s_93_5: i128 = (i128::try_from(s_93_3).unwrap());
        // D s_93_6: cmp-ge s_93_5 s_93_4
        let s_93_6: bool = ((s_93_5) >= (s_93_4));
        // D s_93_7: write-var gs#142636 <= s_93_6
        fn_state.gs_142636 = s_93_6;
        // N s_93_8: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_94_0: read-var CRn:u8
        let s_94_0: u8 = fn_state.CRn;
        // D s_94_1: cast zx s_94_0 -> bv
        let s_94_1: Bits = Bits::new(s_94_0 as u128, 4u16);
        // C s_94_2: const #0u : u8
        let s_94_2: u8 = 0;
        // C s_94_3: cast zx s_94_2 -> bv
        let s_94_3: Bits = Bits::new(s_94_2 as u128, 4u16);
        // D s_94_4: cmp-eq s_94_1 s_94_3
        let s_94_4: bool = ((s_94_1) == (s_94_3));
        // D s_94_5: write-var gs#142634 <= s_94_4
        fn_state.gs_142634 = s_94_4;
        // N s_94_6: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_95_0: read-var opc1:u8
        let s_95_0: u8 = fn_state.opc1;
        // D s_95_1: cast zx s_95_0 -> bv
        let s_95_1: Bits = Bits::new(s_95_0 as u128, 3u16);
        // C s_95_2: const #0u : u8
        let s_95_2: u8 = 0;
        // C s_95_3: cast zx s_95_2 -> bv
        let s_95_3: Bits = Bits::new(s_95_2 as u128, 3u16);
        // D s_95_4: cmp-eq s_95_1 s_95_3
        let s_95_4: bool = ((s_95_1) == (s_95_3));
        // D s_95_5: write-var gs#142633 <= s_95_4
        fn_state.gs_142633 = s_95_4;
        // N s_95_6: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_96_0: const #16975u : u32
        let s_96_0: u32 = 16975;
        // D s_96_1: read-reg s_96_0:u8
        let s_96_1: u8 = {
            let value = state.read_register::<u8>(s_96_0 as isize);
            tracer.read_register(s_96_0 as isize, value);
            value
        };
        // D s_96_2: cast zx s_96_1 -> bv
        let s_96_2: Bits = Bits::new(s_96_1 as u128, 2u16);
        // C s_96_3: const #448u : u32
        let s_96_3: u32 = 448;
        // D s_96_4: read-reg s_96_3:u8
        let s_96_4: u8 = {
            let value = state.read_register::<u8>(s_96_3 as isize);
            tracer.read_register(s_96_3 as isize, value);
            value
        };
        // D s_96_5: cast zx s_96_4 -> bv
        let s_96_5: Bits = Bits::new(s_96_4 as u128, 2u16);
        // D s_96_6: cmp-ne s_96_2 s_96_5
        let s_96_6: bool = ((s_96_2) != (s_96_5));
        // N s_96_7: branch s_96_6 b101 b97
        if s_96_6 {
            return block_101(state, tracer, fn_state);
        } else {
            return block_97(state, tracer, fn_state);
        };
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #0u : u8
        let s_97_0: bool = false;
        // D s_97_1: write-var gs#142669 <= s_97_0
        fn_state.gs_142669 = s_97_0;
        // N s_97_2: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_98_0: read-var gs#142669:u8
        let s_98_0: bool = fn_state.gs_142669;
        // N s_98_1: branch s_98_0 b100 b99
        if s_98_0 {
            return block_100(state, tracer, fn_state);
        } else {
            return block_99(state, tracer, fn_state);
        };
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_99_0: panic
        panic!("{:?}", ());
        // N s_99_1: return
        return;
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #32s : i64
        let s_100_0: i64 = 32;
        // C s_100_1: cast zx s_100_0 -> i
        let s_100_1: i128 = (i128::try_from(s_100_0).unwrap());
        // S s_100_2: call __UNKNOWN_bits(s_100_1)
        let s_100_2: Bits = u__UNKNOWN_bits(state, tracer, s_100_1);
        // S s_100_3: cast reint s_100_2 -> u32
        let s_100_3: u32 = (s_100_2.value() as u32);
        // D s_100_4: read-var t:i
        let s_100_4: i128 = fn_state.t;
        // D s_100_5: call R_set(s_100_4, s_100_3)
        let s_100_5: () = R_set(state, tracer, s_100_4, s_100_3);
        // N s_100_6: return
        return;
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_101_0: read-var read:u8
        let s_101_0: bool = fn_state.read;
        // D s_101_1: write-var gs#142669 <= s_101_0
        fn_state.gs_142669 = s_101_0;
        // N s_101_2: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_102_0: read-var opc2:u8
        let s_102_0: u8 = fn_state.opc2;
        // D s_102_1: cast zx s_102_0 -> bv
        let s_102_1: Bits = Bits::new(s_102_0 as u128, 3u16);
        // C s_102_2: const #7u : u8
        let s_102_2: u8 = 7;
        // C s_102_3: cast zx s_102_2 -> bv
        let s_102_3: Bits = Bits::new(s_102_2 as u128, 3u16);
        // D s_102_4: cmp-eq s_102_1 s_102_3
        let s_102_4: bool = ((s_102_1) == (s_102_3));
        // D s_102_5: write-var gs#142631 <= s_102_4
        fn_state.gs_142631 = s_102_4;
        // N s_102_6: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_103_0: read-var CRm:u8
        let s_103_0: u8 = fn_state.CRm;
        // D s_103_1: cast zx s_103_0 -> bv
        let s_103_1: Bits = Bits::new(s_103_0 as u128, 4u16);
        // C s_103_2: const #2u : u8
        let s_103_2: u8 = 2;
        // C s_103_3: cast zx s_103_2 -> bv
        let s_103_3: Bits = Bits::new(s_103_2 as u128, 4u16);
        // D s_103_4: cmp-eq s_103_1 s_103_3
        let s_103_4: bool = ((s_103_1) == (s_103_3));
        // D s_103_5: write-var gs#142630 <= s_103_4
        fn_state.gs_142630 = s_103_4;
        // N s_103_6: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_104_0: read-var CRn:u8
        let s_104_0: u8 = fn_state.CRn;
        // D s_104_1: cast zx s_104_0 -> bv
        let s_104_1: Bits = Bits::new(s_104_0 as u128, 4u16);
        // C s_104_2: const #0u : u8
        let s_104_2: u8 = 0;
        // C s_104_3: cast zx s_104_2 -> bv
        let s_104_3: Bits = Bits::new(s_104_2 as u128, 4u16);
        // D s_104_4: cmp-eq s_104_1 s_104_3
        let s_104_4: bool = ((s_104_1) == (s_104_3));
        // D s_104_5: write-var gs#142629 <= s_104_4
        fn_state.gs_142629 = s_104_4;
        // N s_104_6: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_105_0: read-var opc1:u8
        let s_105_0: u8 = fn_state.opc1;
        // D s_105_1: cast zx s_105_0 -> bv
        let s_105_1: Bits = Bits::new(s_105_0 as u128, 3u16);
        // C s_105_2: const #0u : u8
        let s_105_2: u8 = 0;
        // C s_105_3: cast zx s_105_2 -> bv
        let s_105_3: Bits = Bits::new(s_105_2 as u128, 3u16);
        // D s_105_4: cmp-eq s_105_1 s_105_3
        let s_105_4: bool = ((s_105_1) == (s_105_3));
        // D s_105_5: write-var gs#142628 <= s_105_4
        fn_state.gs_142628 = s_105_4;
        // N s_105_6: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #432u : u32
        let s_106_0: u32 = 432;
        // D s_106_1: read-reg s_106_0:u8
        let s_106_1: u8 = {
            let value = state.read_register::<u8>(s_106_0 as isize);
            tracer.read_register(s_106_0 as isize, value);
            value
        };
        // D s_106_2: call ELUsingAArch32(s_106_1)
        let s_106_2: bool = ELUsingAArch32(state, tracer, s_106_1);
        // N s_106_3: branch s_106_2 b108 b107
        if s_106_2 {
            return block_108(state, tracer, fn_state);
        } else {
            return block_107(state, tracer, fn_state);
        };
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #3u : u8
        let s_107_0: u8 = 3;
        // C s_107_1: cast zx s_107_0 -> bv
        let s_107_1: Bits = Bits::new(s_107_0 as u128, 4u16);
        // C s_107_2: cast zx s_107_1 -> i
        let s_107_2: i128 = (s_107_1.value() as i128);
        // C s_107_3: cast reint s_107_2 -> i64
        let s_107_3: i64 = (s_107_2 as i64);
        // C s_107_4: cast zx s_107_3 -> i
        let s_107_4: i128 = (i128::try_from(s_107_3).unwrap());
        // C s_107_5: const #432u : u32
        let s_107_5: u32 = 432;
        // D s_107_6: read-reg s_107_5:u8
        let s_107_6: u8 = {
            let value = state.read_register::<u8>(s_107_5 as isize);
            tracer.read_register(s_107_5 as isize, value);
            value
        };
        // D s_107_7: call AArch64_AArch32SystemAccessTrap(s_107_6, s_107_4)
        let s_107_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_107_6,
            s_107_4,
        );
        // N s_107_8: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_108_0: const #3u : u8
        let s_108_0: u8 = 3;
        // C s_108_1: cast zx s_108_0 -> bv
        let s_108_1: Bits = Bits::new(s_108_0 as u128, 4u16);
        // C s_108_2: cast zx s_108_1 -> i
        let s_108_2: i128 = (s_108_1.value() as i128);
        // C s_108_3: cast reint s_108_2 -> i64
        let s_108_3: i64 = (s_108_2 as i64);
        // C s_108_4: cast zx s_108_3 -> i
        let s_108_4: i128 = (i128::try_from(s_108_3).unwrap());
        // C s_108_5: const #400u : u32
        let s_108_5: u32 = 400;
        // D s_108_6: read-reg s_108_5:u8
        let s_108_6: u8 = {
            let value = state.read_register::<u8>(s_108_5 as isize);
            tracer.read_register(s_108_5 as isize, value);
            value
        };
        // D s_108_7: call AArch32_SystemAccessTrap(s_108_6, s_108_4)
        let s_108_7: () = AArch32_SystemAccessTrap(state, tracer, s_108_6, s_108_4);
        // N s_108_8: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_109_0: read-var CRm:u8
        let s_109_0: u8 = fn_state.CRm;
        // D s_109_1: cast zx s_109_0 -> bv
        let s_109_1: Bits = Bits::new(s_109_0 as u128, 4u16);
        // D s_109_2: cast zx s_109_1 -> i
        let s_109_2: i128 = (s_109_1.value() as i128);
        // D s_109_3: cast reint s_109_2 -> i64
        let s_109_3: i64 = (s_109_2 as i64);
        // C s_109_4: const #2s : i
        let s_109_4: i128 = 2;
        // D s_109_5: cast zx s_109_3 -> i
        let s_109_5: i128 = (i128::try_from(s_109_3).unwrap());
        // D s_109_6: cmp-eq s_109_5 s_109_4
        let s_109_6: bool = ((s_109_5) == (s_109_4));
        // N s_109_7: branch s_109_6 b124 b110
        if s_109_6 {
            return block_124(state, tracer, fn_state);
        } else {
            return block_110(state, tracer, fn_state);
        };
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_110_0: read-var CRm:u8
        let s_110_0: u8 = fn_state.CRm;
        // D s_110_1: cast zx s_110_0 -> bv
        let s_110_1: Bits = Bits::new(s_110_0 as u128, 4u16);
        // D s_110_2: cast zx s_110_1 -> i
        let s_110_2: i128 = (s_110_1.value() as i128);
        // D s_110_3: cast reint s_110_2 -> i64
        let s_110_3: i64 = (s_110_2 as i64);
        // C s_110_4: const #3s : i
        let s_110_4: i128 = 3;
        // D s_110_5: cast zx s_110_3 -> i
        let s_110_5: i128 = (i128::try_from(s_110_3).unwrap());
        // D s_110_6: cmp-eq s_110_5 s_110_4
        let s_110_6: bool = ((s_110_5) == (s_110_4));
        // N s_110_7: branch s_110_6 b123 b111
        if s_110_6 {
            return block_123(state, tracer, fn_state);
        } else {
            return block_111(state, tracer, fn_state);
        };
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_111_0: read-var CRm:u8
        let s_111_0: u8 = fn_state.CRm;
        // D s_111_1: cast zx s_111_0 -> bv
        let s_111_1: Bits = Bits::new(s_111_0 as u128, 4u16);
        // D s_111_2: cast zx s_111_1 -> i
        let s_111_2: i128 = (s_111_1.value() as i128);
        // D s_111_3: cast reint s_111_2 -> i64
        let s_111_3: i64 = (s_111_2 as i64);
        // C s_111_4: const #4s : i
        let s_111_4: i128 = 4;
        // D s_111_5: cast zx s_111_3 -> i
        let s_111_5: i128 = (i128::try_from(s_111_3).unwrap());
        // D s_111_6: cmp-eq s_111_5 s_111_4
        let s_111_6: bool = ((s_111_5) == (s_111_4));
        // N s_111_7: branch s_111_6 b122 b112
        if s_111_6 {
            return block_122(state, tracer, fn_state);
        } else {
            return block_112(state, tracer, fn_state);
        };
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_112_0: read-var CRm:u8
        let s_112_0: u8 = fn_state.CRm;
        // D s_112_1: cast zx s_112_0 -> bv
        let s_112_1: Bits = Bits::new(s_112_0 as u128, 4u16);
        // D s_112_2: cast zx s_112_1 -> i
        let s_112_2: i128 = (s_112_1.value() as i128);
        // D s_112_3: cast reint s_112_2 -> i64
        let s_112_3: i64 = (s_112_2 as i64);
        // C s_112_4: const #5s : i
        let s_112_4: i128 = 5;
        // D s_112_5: cast zx s_112_3 -> i
        let s_112_5: i128 = (i128::try_from(s_112_3).unwrap());
        // D s_112_6: cmp-eq s_112_5 s_112_4
        let s_112_6: bool = ((s_112_5) == (s_112_4));
        // N s_112_7: branch s_112_6 b121 b113
        if s_112_6 {
            return block_121(state, tracer, fn_state);
        } else {
            return block_113(state, tracer, fn_state);
        };
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_113_0: read-var CRm:u8
        let s_113_0: u8 = fn_state.CRm;
        // D s_113_1: cast zx s_113_0 -> bv
        let s_113_1: Bits = Bits::new(s_113_0 as u128, 4u16);
        // D s_113_2: cast zx s_113_1 -> i
        let s_113_2: i128 = (s_113_1.value() as i128);
        // D s_113_3: cast reint s_113_2 -> i64
        let s_113_3: i64 = (s_113_2 as i64);
        // C s_113_4: const #6s : i
        let s_113_4: i128 = 6;
        // D s_113_5: cast zx s_113_3 -> i
        let s_113_5: i128 = (i128::try_from(s_113_3).unwrap());
        // D s_113_6: cmp-eq s_113_5 s_113_4
        let s_113_6: bool = ((s_113_5) == (s_113_4));
        // N s_113_7: branch s_113_6 b120 b114
        if s_113_6 {
            return block_120(state, tracer, fn_state);
        } else {
            return block_114(state, tracer, fn_state);
        };
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_114_0: read-var CRm:u8
        let s_114_0: u8 = fn_state.CRm;
        // D s_114_1: cast zx s_114_0 -> bv
        let s_114_1: Bits = Bits::new(s_114_0 as u128, 4u16);
        // D s_114_2: cast zx s_114_1 -> i
        let s_114_2: i128 = (s_114_1.value() as i128);
        // D s_114_3: cast reint s_114_2 -> i64
        let s_114_3: i64 = (s_114_2 as i64);
        // C s_114_4: const #7s : i
        let s_114_4: i128 = 7;
        // D s_114_5: cast zx s_114_3 -> i
        let s_114_5: i128 = (i128::try_from(s_114_3).unwrap());
        // D s_114_6: cmp-eq s_114_5 s_114_4
        let s_114_6: bool = ((s_114_5) == (s_114_4));
        // D s_114_7: write-var gs#142621 <= s_114_6
        fn_state.gs_142621 = s_114_6;
        // N s_114_8: jump b115
        return block_115(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_115_0: read-var gs#142621:u8
        let s_115_0: bool = fn_state.gs_142621;
        // D s_115_1: write-var gs#142622 <= s_115_0
        fn_state.gs_142622 = s_115_0;
        // N s_115_2: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_116_0: read-var gs#142622:u8
        let s_116_0: bool = fn_state.gs_142622;
        // D s_116_1: write-var gs#142623 <= s_116_0
        fn_state.gs_142623 = s_116_0;
        // N s_116_2: jump b117
        return block_117(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_117_0: read-var gs#142623:u8
        let s_117_0: bool = fn_state.gs_142623;
        // D s_117_1: write-var gs#142624 <= s_117_0
        fn_state.gs_142624 = s_117_0;
        // N s_117_2: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_118_0: read-var gs#142624:u8
        let s_118_0: bool = fn_state.gs_142624;
        // D s_118_1: write-var gs#142625 <= s_118_0
        fn_state.gs_142625 = s_118_0;
        // N s_118_2: jump b119
        return block_119(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_119_0: read-var gs#142625:u8
        let s_119_0: bool = fn_state.gs_142625;
        // D s_119_1: write-var gs#142626 <= s_119_0
        fn_state.gs_142626 = s_119_0;
        // N s_119_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_120_0: const #1u : u8
        let s_120_0: bool = true;
        // D s_120_1: write-var gs#142621 <= s_120_0
        fn_state.gs_142621 = s_120_0;
        // N s_120_2: jump b115
        return block_115(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_121_0: const #1u : u8
        let s_121_0: bool = true;
        // D s_121_1: write-var gs#142622 <= s_121_0
        fn_state.gs_142622 = s_121_0;
        // N s_121_2: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_122_0: const #1u : u8
        let s_122_0: bool = true;
        // D s_122_1: write-var gs#142623 <= s_122_0
        fn_state.gs_142623 = s_122_0;
        // N s_122_2: jump b117
        return block_117(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_123_0: const #1u : u8
        let s_123_0: bool = true;
        // D s_123_1: write-var gs#142624 <= s_123_0
        fn_state.gs_142624 = s_123_0;
        // N s_123_2: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_124_0: const #1u : u8
        let s_124_0: bool = true;
        // D s_124_1: write-var gs#142625 <= s_124_0
        fn_state.gs_142625 = s_124_0;
        // N s_124_2: jump b119
        return block_119(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_125_0: read-var CRn:u8
        let s_125_0: u8 = fn_state.CRn;
        // D s_125_1: cast zx s_125_0 -> bv
        let s_125_1: Bits = Bits::new(s_125_0 as u128, 4u16);
        // D s_125_2: cast zx s_125_1 -> i
        let s_125_2: i128 = (s_125_1.value() as i128);
        // D s_125_3: cast reint s_125_2 -> i64
        let s_125_3: i64 = (s_125_2 as i64);
        // C s_125_4: const #0s : i
        let s_125_4: i128 = 0;
        // D s_125_5: cast zx s_125_3 -> i
        let s_125_5: i128 = (i128::try_from(s_125_3).unwrap());
        // D s_125_6: cmp-eq s_125_5 s_125_4
        let s_125_6: bool = ((s_125_5) == (s_125_4));
        // D s_125_7: write-var gs#142614 <= s_125_6
        fn_state.gs_142614 = s_125_6;
        // N s_125_8: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_126_0: read-var opc1:u8
        let s_126_0: u8 = fn_state.opc1;
        // D s_126_1: cast zx s_126_0 -> bv
        let s_126_1: Bits = Bits::new(s_126_0 as u128, 3u16);
        // D s_126_2: cast zx s_126_1 -> i
        let s_126_2: i128 = (s_126_1.value() as i128);
        // D s_126_3: cast reint s_126_2 -> i64
        let s_126_3: i64 = (s_126_2 as i64);
        // C s_126_4: const #0s : i
        let s_126_4: i128 = 0;
        // D s_126_5: cast zx s_126_3 -> i
        let s_126_5: i128 = (i128::try_from(s_126_3).unwrap());
        // D s_126_6: cmp-eq s_126_5 s_126_4
        let s_126_6: bool = ((s_126_5) == (s_126_4));
        // D s_126_7: write-var gs#142612 <= s_126_6
        fn_state.gs_142612 = s_126_6;
        // N s_126_8: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_127_0: read-var coproc:u8
        let s_127_0: u8 = fn_state.coproc;
        // D s_127_1: cast zx s_127_0 -> bv
        let s_127_1: Bits = Bits::new(s_127_0 as u128, 4u16);
        // D s_127_2: cast zx s_127_1 -> i
        let s_127_2: i128 = (s_127_1.value() as i128);
        // D s_127_3: cast reint s_127_2 -> i64
        let s_127_3: i64 = (s_127_2 as i64);
        // C s_127_4: const #15s : i
        let s_127_4: i128 = 15;
        // D s_127_5: cast zx s_127_3 -> i
        let s_127_5: i128 = (i128::try_from(s_127_3).unwrap());
        // D s_127_6: cmp-eq s_127_5 s_127_4
        let s_127_6: bool = ((s_127_5) == (s_127_4));
        // D s_127_7: write-var gs#142610 <= s_127_6
        fn_state.gs_142610 = s_127_6;
        // N s_127_8: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_128_0: const #() : ()
        let s_128_0: () = ();
        // S s_128_1: call HaveFGTExt(s_128_0)
        let s_128_1: bool = HaveFGTExt(state, tracer, s_128_0);
        // N s_128_2: branch s_128_1 b131 b129
        if s_128_1 {
            return block_131(state, tracer, fn_state);
        } else {
            return block_129(state, tracer, fn_state);
        };
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_129_0: const #"Unallocated encodings trapped by HCR_EL2.TID3" : str
        let s_129_0: &'static str = "Unallocated encodings trapped by HCR_EL2.TID3";
        // S s_129_1: call __IMPDEF_boolean(s_129_0)
        let s_129_1: bool = u__IMPDEF_boolean(state, tracer, s_129_0);
        // D s_129_2: write-var tid3_trap <= s_129_1
        fn_state.tid3_trap = s_129_1;
        // N s_129_3: jump b130
        return block_130(state, tracer, fn_state);
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_130_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_131_0: const #1u : u8
        let s_131_0: bool = true;
        // D s_131_1: write-var tid3_trap <= s_131_0
        fn_state.tid3_trap = s_131_0;
        // N s_131_2: jump b130
        return block_130(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_132_0: const #102552u : u32
        let s_132_0: u32 = 102552;
        // D s_132_1: read-reg s_132_0:struct
        let s_132_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_132_0 as isize);
            tracer.read_register(s_132_0 as isize, value);
            value
        };
        // D s_132_2: call _get_HCR_EL2_Type_TID3(s_132_1)
        let s_132_2: bool = u_get_HCR_EL2_Type_TID3(state, tracer, s_132_1);
        // D s_132_3: cast zx s_132_2 -> bv
        let s_132_3: Bits = Bits::new(s_132_2 as u128, 1u16);
        // C s_132_4: const #1u : u8
        let s_132_4: bool = true;
        // C s_132_5: cast zx s_132_4 -> bv
        let s_132_5: Bits = Bits::new(s_132_4 as u128, 1u16);
        // D s_132_6: cmp-eq s_132_3 s_132_5
        let s_132_6: bool = ((s_132_3) == (s_132_5));
        // D s_132_7: write-var gs#142608 <= s_132_6
        fn_state.gs_142608 = s_132_6;
        // N s_132_8: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_133_0: const #() : ()
        let s_133_0: () = ();
        // S s_133_1: call EL2Enabled(s_133_0)
        let s_133_1: bool = EL2Enabled(state, tracer, s_133_0);
        // D s_133_2: write-var gs#142607 <= s_133_1
        fn_state.gs_142607 = s_133_1;
        // N s_133_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_134_0: read-var read:u8
        let s_134_0: bool = fn_state.read;
        // D s_134_1: write-var gs#142606 <= s_134_0
        fn_state.gs_142606 = s_134_0;
        // N s_134_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
