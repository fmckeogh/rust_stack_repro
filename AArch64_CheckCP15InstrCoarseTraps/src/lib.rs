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
use u_get_SCTLR_EL2_Type_TIDCP::*;
use IsInHost::*;
use u__id::*;
use u_get_SCTLR_EL1_Type_TIDCP::*;
use HaveFeatTIDCP1::*;
use ELUsingAArch32::*;
use u_get_HCR_EL2_Type_TIDCP::*;
use u__IMPDEF_boolean::*;
use AArch64_AArch32SystemAccessTrap::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_TGE::*;
use common::*;
pub fn AArch64_CheckCP15InstrCoarseTraps<T: Tracer>(
    state: &mut State,
    tracer: &T,
    CRn: i128,
    nreg: i128,
    CRm: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_25157: bool,
        gs_25141: bool,
        gs_25161: bool,
        gs_25167: bool,
        gs_25138: bool,
        gs_25190: bool,
        gs_25139: bool,
        gs_25174: bool,
        gs_25162: bool,
        trapped_encoding: bool,
        gs_25181: bool,
        gs_25156: bool,
        gs_25198: bool,
        gs_25175: bool,
        gs_25173: bool,
        gs_25129: bool,
        gs_25191: bool,
        gs_25130: bool,
        gs_25153: bool,
        gs_25172: bool,
        majorshadow_483: i128,
        gs_25154: bool,
        gs_25140: bool,
        gs_25163: bool,
        gs_25184: bool,
        gs_25155: bool,
        gs_25159: bool,
        gs_25128: bool,
        ga_19635: i128,
        gs_25193: bool,
        gs_25185: bool,
        gs_25187: bool,
        gs_25160: bool,
        gs_25170: bool,
        gs_25127: bool,
        gs_25126: bool,
        gs_25158: bool,
        gs_25168: bool,
        gs_25131: bool,
        gs_25137: bool,
        gs_25166: bool,
        gs_25165: bool,
        gs_25164: bool,
        gs_25189: bool,
        gs_25169: bool,
        gs_25125: bool,
        CRn: i128,
        nreg: i128,
        CRm: i128,
    }
    let fn_state = FunctionState {
        CRn,
        nreg,
        CRm,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #9s : i
        let s_0_0: i128 = 9;
        // D s_0_1: read-var CRn:i
        let s_0_1: i128 = fn_state.CRn;
        // D s_0_2: cmp-eq s_0_1 s_0_0
        let s_0_2: bool = ((s_0_1) == (s_0_0));
        // N s_0_3: branch s_0_2 b128 b1
        if s_0_2 {
            return block_128(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#25131 <= s_1_0
        fn_state.gs_25131 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#25131:u8
        let s_2_0: bool = fn_state.gs_25131;
        // N s_2_1: branch s_2_0 b127 b3
        if s_2_0 {
            return block_127(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #10s : i
        let s_3_0: i128 = 10;
        // D s_3_1: read-var CRn:i
        let s_3_1: i128 = fn_state.CRn;
        // D s_3_2: cmp-eq s_3_1 s_3_0
        let s_3_2: bool = ((s_3_1) == (s_3_0));
        // N s_3_3: branch s_3_2 b117 b4
        if s_3_2 {
            return block_117(state, tracer, fn_state);
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
        // D s_4_1: write-var gs#25140 <= s_4_0
        fn_state.gs_25140 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#25140:u8
        let s_5_0: bool = fn_state.gs_25140;
        // D s_5_1: write-var gs#25141 <= s_5_0
        fn_state.gs_25141 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#25141:u8
        let s_6_0: bool = fn_state.gs_25141;
        // N s_6_1: branch s_6_0 b116 b7
        if s_6_0 {
            return block_116(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #11s : i
        let s_7_0: i128 = 11;
        // D s_7_1: read-var CRn:i
        let s_7_1: i128 = fn_state.CRn;
        // D s_7_2: cmp-eq s_7_1 s_7_0
        let s_7_2: bool = ((s_7_1) == (s_7_0));
        // N s_7_3: branch s_7_2 b88 b8
        if s_7_2 {
            return block_88(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#25162 <= s_8_0
        fn_state.gs_25162 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#25162:u8
        let s_9_0: bool = fn_state.gs_25162;
        // D s_9_1: write-var gs#25163 <= s_9_0
        fn_state.gs_25163 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#25163:u8
        let s_10_0: bool = fn_state.gs_25163;
        // D s_10_1: write-var trapped_encoding <= s_10_0
        fn_state.trapped_encoding = s_10_0;
        // C s_10_2: const #() : ()
        let s_10_2: () = ();
        // S s_10_3: call HaveFeatTIDCP1(s_10_2)
        let s_10_3: bool = HaveFeatTIDCP1(state, tracer, s_10_2);
        // N s_10_4: branch s_10_3 b87 b11
        if s_10_3 {
            return block_87(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#25164 <= s_11_0
        fn_state.gs_25164 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#25164:u8
        let s_12_0: bool = fn_state.gs_25164;
        // N s_12_1: branch s_12_0 b86 b13
        if s_12_0 {
            return block_86(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#25165 <= s_13_0
        fn_state.gs_25165 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#25165:u8
        let s_14_0: bool = fn_state.gs_25165;
        // N s_14_1: branch s_14_0 b85 b15
        if s_14_0 {
            return block_85(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#25166 <= s_15_0
        fn_state.gs_25166 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#25166:u8
        let s_16_0: bool = fn_state.gs_25166;
        // N s_16_1: branch s_16_0 b84 b17
        if s_16_0 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#25167 <= s_17_0
        fn_state.gs_25167 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#25167:u8
        let s_18_0: bool = fn_state.gs_25167;
        // N s_18_1: branch s_18_0 b83 b19
        if s_18_0 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#25168 <= s_19_0
        fn_state.gs_25168 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#25168:u8
        let s_20_0: bool = fn_state.gs_25168;
        // N s_20_1: branch s_20_0 b77 b21
        if s_20_0 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_21_0: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #16975u : u32
        let s_22_0: u32 = 16975;
        // D s_22_1: read-reg s_22_0:u8
        let s_22_1: u8 = {
            let value = state.read_register::<u8>(s_22_0 as isize);
            tracer.read_register(s_22_0 as isize, value);
            value
        };
        // D s_22_2: cast zx s_22_1 -> bv
        let s_22_2: Bits = Bits::new(s_22_1 as u128, 2u16);
        // C s_22_3: const #448u : u32
        let s_22_3: u32 = 448;
        // D s_22_4: read-reg s_22_3:u8
        let s_22_4: u8 = {
            let value = state.read_register::<u8>(s_22_3 as isize);
            tracer.read_register(s_22_3 as isize, value);
            value
        };
        // D s_22_5: cast zx s_22_4 -> bv
        let s_22_5: Bits = Bits::new(s_22_4 as u128, 2u16);
        // D s_22_6: cmp-eq s_22_2 s_22_5
        let s_22_6: bool = ((s_22_2) == (s_22_5));
        // N s_22_7: branch s_22_6 b76 b23
        if s_22_6 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #16975u : u32
        let s_23_0: u32 = 16975;
        // D s_23_1: read-reg s_23_0:u8
        let s_23_1: u8 = {
            let value = state.read_register::<u8>(s_23_0 as isize);
            tracer.read_register(s_23_0 as isize, value);
            value
        };
        // D s_23_2: cast zx s_23_1 -> bv
        let s_23_2: Bits = Bits::new(s_23_1 as u128, 2u16);
        // C s_23_3: const #440u : u32
        let s_23_3: u32 = 440;
        // D s_23_4: read-reg s_23_3:u8
        let s_23_4: u8 = {
            let value = state.read_register::<u8>(s_23_3 as isize);
            tracer.read_register(s_23_3 as isize, value);
            value
        };
        // D s_23_5: cast zx s_23_4 -> bv
        let s_23_5: Bits = Bits::new(s_23_4 as u128, 2u16);
        // D s_23_6: cmp-eq s_23_2 s_23_5
        let s_23_6: bool = ((s_23_2) == (s_23_5));
        // D s_23_7: write-var gs#25169 <= s_23_6
        fn_state.gs_25169 = s_23_6;
        // N s_23_8: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#25169:u8
        let s_24_0: bool = fn_state.gs_25169;
        // N s_24_1: branch s_24_0 b75 b25
        if s_24_0 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // D s_25_1: write-var gs#25170 <= s_25_0
        fn_state.gs_25170 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#25170:u8
        let s_26_0: bool = fn_state.gs_25170;
        // N s_26_1: branch s_26_0 b28 b27
        if s_26_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_27_0: return
        return;
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #() : ()
        let s_28_0: () = ();
        // S s_28_1: call HaveFeatTIDCP1(s_28_0)
        let s_28_1: bool = HaveFeatTIDCP1(state, tracer, s_28_0);
        // N s_28_2: branch s_28_1 b74 b29
        if s_28_1 {
            return block_74(state, tracer, fn_state);
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
        // D s_29_1: write-var gs#25172 <= s_29_0
        fn_state.gs_25172 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#25172:u8
        let s_30_0: bool = fn_state.gs_25172;
        // N s_30_1: branch s_30_0 b73 b31
        if s_30_0 {
            return block_73(state, tracer, fn_state);
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
        // D s_31_1: write-var gs#25173 <= s_31_0
        fn_state.gs_25173 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#25173:u8
        let s_32_0: bool = fn_state.gs_25173;
        // N s_32_1: branch s_32_0 b72 b33
        if s_32_0 {
            return block_72(state, tracer, fn_state);
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
        // D s_33_1: write-var gs#25174 <= s_33_0
        fn_state.gs_25174 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#25174:u8
        let s_34_0: bool = fn_state.gs_25174;
        // N s_34_1: branch s_34_0 b71 b35
        if s_34_0 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #0u : u8
        let s_35_0: bool = false;
        // D s_35_1: write-var gs#25175 <= s_35_0
        fn_state.gs_25175 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#25175:u8
        let s_36_0: bool = fn_state.gs_25175;
        // N s_36_1: branch s_36_0 b70 b37
        if s_36_0 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_37_0: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #1s : i
        let s_38_0: i128 = 1;
        // D s_38_1: read-var nreg:i
        let s_38_1: i128 = fn_state.nreg;
        // D s_38_2: cmp-eq s_38_1 s_38_0
        let s_38_2: bool = ((s_38_1) == (s_38_0));
        // N s_38_3: branch s_38_2 b69 b39
        if s_38_2 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var CRm:i
        let s_39_0: i128 = fn_state.CRm;
        // D s_39_1: write-var ga#19635 <= s_39_0
        fn_state.ga_19635 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var ga#19635:i
        let s_40_0: i128 = fn_state.ga_19635;
        // D s_40_1: write-var majorshadow#483 <= s_40_0
        fn_state.majorshadow_483 = s_40_0;
        // D s_40_2: read-var majorshadow#483:i
        let s_40_2: i128 = fn_state.majorshadow_483;
        // D s_40_3: call __id(s_40_2)
        let s_40_3: i128 = u__id(state, tracer, s_40_2);
        // C s_40_4: const #0s : i
        let s_40_4: i128 = 0;
        // D s_40_5: cmp-le s_40_4 s_40_3
        let s_40_5: bool = ((s_40_4) <= (s_40_3));
        // N s_40_6: branch s_40_5 b68 b41
        if s_40_5 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #0u : u8
        let s_41_0: bool = false;
        // D s_41_1: write-var gs#25181 <= s_41_0
        fn_state.gs_25181 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#25181:u8
        let s_42_0: bool = fn_state.gs_25181;
        // N s_42_1: assert s_42_0
        let s_42_1: () = assert!(s_42_0);
        // C s_42_2: const #() : ()
        let s_42_2: () = ();
        // S s_42_3: call IsInHost(s_42_2)
        let s_42_3: bool = IsInHost(state, tracer, s_42_2);
        // S s_42_4: not s_42_3
        let s_42_4: bool = !s_42_3;
        // N s_42_5: branch s_42_4 b64 b43
        if s_42_4 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #0u : u8
        let s_43_0: bool = false;
        // D s_43_1: write-var gs#25185 <= s_43_0
        fn_state.gs_25185 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#25185:u8
        let s_44_0: bool = fn_state.gs_25185;
        // N s_44_1: branch s_44_0 b63 b45
        if s_44_0 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #0u : u8
        let s_45_0: bool = false;
        // D s_45_1: write-var gs#25187 <= s_45_0
        fn_state.gs_25187 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#25187:u8
        let s_46_0: bool = fn_state.gs_25187;
        // N s_46_1: branch s_46_0 b62 b47
        if s_46_0 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #102552u : u32
        let s_47_0: u32 = 102552;
        // D s_47_1: read-reg s_47_0:struct
        let s_47_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_47_0 as isize);
            tracer.read_register(s_47_0 as isize, value);
            value
        };
        // D s_47_2: call _get_HCR_EL2_Type_TIDCP(s_47_1)
        let s_47_2: bool = u_get_HCR_EL2_Type_TIDCP(state, tracer, s_47_1);
        // D s_47_3: cast zx s_47_2 -> bv
        let s_47_3: Bits = Bits::new(s_47_2 as u128, 1u16);
        // C s_47_4: const #1u : u8
        let s_47_4: bool = true;
        // C s_47_5: cast zx s_47_4 -> bv
        let s_47_5: Bits = Bits::new(s_47_4 as u128, 1u16);
        // D s_47_6: cmp-eq s_47_3 s_47_5
        let s_47_6: bool = ((s_47_3) == (s_47_5));
        // N s_47_7: branch s_47_6 b61 b48
        if s_47_6 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #0u : u8
        let s_48_0: bool = false;
        // D s_48_1: write-var gs#25189 <= s_48_0
        fn_state.gs_25189 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#25189:u8
        let s_49_0: bool = fn_state.gs_25189;
        // N s_49_1: branch s_49_0 b60 b50
        if s_49_0 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #0u : u8
        let s_50_0: bool = false;
        // D s_50_1: write-var gs#25190 <= s_50_0
        fn_state.gs_25190 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#25190:u8
        let s_51_0: bool = fn_state.gs_25190;
        // D s_51_1: write-var gs#25191 <= s_51_0
        fn_state.gs_25191 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#25191:u8
        let s_52_0: bool = fn_state.gs_25191;
        // N s_52_1: branch s_52_0 b54 b53
        if s_52_0 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_53_0: return
        return;
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #16975u : u32
        let s_54_0: u32 = 16975;
        // D s_54_1: read-reg s_54_0:u8
        let s_54_1: u8 = {
            let value = state.read_register::<u8>(s_54_0 as isize);
            tracer.read_register(s_54_0 as isize, value);
            value
        };
        // D s_54_2: cast zx s_54_1 -> bv
        let s_54_2: Bits = Bits::new(s_54_1 as u128, 2u16);
        // C s_54_3: const #448u : u32
        let s_54_3: u32 = 448;
        // D s_54_4: read-reg s_54_3:u8
        let s_54_4: u8 = {
            let value = state.read_register::<u8>(s_54_3 as isize);
            tracer.read_register(s_54_3 as isize, value);
            value
        };
        // D s_54_5: cast zx s_54_4 -> bv
        let s_54_5: Bits = Bits::new(s_54_4 as u128, 2u16);
        // D s_54_6: cmp-eq s_54_2 s_54_5
        let s_54_6: bool = ((s_54_2) == (s_54_5));
        // N s_54_7: branch s_54_6 b59 b55
        if s_54_6 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #0u : u8
        let s_55_0: bool = false;
        // D s_55_1: write-var gs#25193 <= s_55_0
        fn_state.gs_25193 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var gs#25193:u8
        let s_56_0: bool = fn_state.gs_25193;
        // N s_56_1: branch s_56_0 b58 b57
        if s_56_0 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #3u : u8
        let s_57_0: u8 = 3;
        // C s_57_1: cast zx s_57_0 -> bv
        let s_57_1: Bits = Bits::new(s_57_0 as u128, 4u16);
        // C s_57_2: cast zx s_57_1 -> i
        let s_57_2: i128 = (s_57_1.value() as i128);
        // C s_57_3: cast reint s_57_2 -> i64
        let s_57_3: i64 = (s_57_2 as i64);
        // C s_57_4: cast zx s_57_3 -> i
        let s_57_4: i128 = (i128::try_from(s_57_3).unwrap());
        // C s_57_5: const #432u : u32
        let s_57_5: u32 = 432;
        // D s_57_6: read-reg s_57_5:u8
        let s_57_6: u8 = {
            let value = state.read_register::<u8>(s_57_5 as isize);
            tracer.read_register(s_57_5 as isize, value);
            value
        };
        // D s_57_7: call AArch64_AArch32SystemAccessTrap(s_57_6, s_57_4)
        let s_57_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_57_6, s_57_4);
        // N s_57_8: return
        return;
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_58_0: panic
        panic!("{:?}", ());
        // N s_58_1: return
        return;
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #"UNDEF unallocated CP15 access at EL0" : str
        let s_59_0: &'static str = "UNDEF unallocated CP15 access at EL0";
        // S s_59_1: call __IMPDEF_boolean(s_59_0)
        let s_59_1: bool = u__IMPDEF_boolean(state, tracer, s_59_0);
        // D s_59_2: write-var gs#25193 <= s_59_1
        fn_state.gs_25193 = s_59_1;
        // N s_59_3: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var trapped_encoding:u8
        let s_60_0: bool = fn_state.trapped_encoding;
        // D s_60_1: write-var gs#25190 <= s_60_0
        fn_state.gs_25190 = s_60_0;
        // N s_60_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #1s : i
        let s_61_0: i128 = 1;
        // D s_61_1: read-var nreg:i
        let s_61_1: i128 = fn_state.nreg;
        // D s_61_2: cmp-eq s_61_1 s_61_0
        let s_61_2: bool = ((s_61_1) == (s_61_0));
        // D s_61_3: write-var gs#25189 <= s_61_2
        fn_state.gs_25189 = s_61_2;
        // N s_61_4: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #1u : u8
        let s_62_0: bool = true;
        // D s_62_1: write-var gs#25191 <= s_62_0
        fn_state.gs_25191 = s_62_0;
        // N s_62_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #104936u : u32
        let s_63_0: u32 = 104936;
        // D s_63_1: read-reg s_63_0:u64
        let s_63_1: u64 = {
            let value = state.read_register::<u64>(s_63_0 as isize);
            tracer.read_register(s_63_0 as isize, value);
            value
        };
        // D s_63_2: cast zx s_63_1 -> bv
        let s_63_2: Bits = Bits::new(s_63_1 as u128, 64u16);
        // D s_63_3: read-var majorshadow#483:i
        let s_63_3: i128 = fn_state.majorshadow_483;
        // C s_63_4: const #1u : u64
        let s_63_4: u64 = 1;
        // D s_63_5: bit-extract s_63_2 s_63_3 s_63_4
        let s_63_5: Bits = (Bits::new(
            ((s_63_2) >> (s_63_3)).value(),
            u16::try_from(s_63_4).unwrap(),
        ));
        // D s_63_6: cast reint s_63_5 -> u8
        let s_63_6: bool = ((s_63_5.value()) != 0);
        // C s_63_7: const #0s : i
        let s_63_7: i128 = 0;
        // C s_63_8: const #0u : u64
        let s_63_8: u64 = 0;
        // D s_63_9: cast zx s_63_6 -> u64
        let s_63_9: u64 = (s_63_6 as u64);
        // C s_63_10: const #1u : u64
        let s_63_10: u64 = 1;
        // D s_63_11: and s_63_9 s_63_10
        let s_63_11: u64 = ((s_63_9) & (s_63_10));
        // D s_63_12: cmp-eq s_63_11 s_63_10
        let s_63_12: bool = ((s_63_11) == (s_63_10));
        // D s_63_13: lsl s_63_9 s_63_7
        let s_63_13: u64 = s_63_9 << s_63_7;
        // D s_63_14: or s_63_8 s_63_13
        let s_63_14: u64 = ((s_63_8) | (s_63_13));
        // D s_63_15: cmpl s_63_13
        let s_63_15: u64 = !s_63_13;
        // D s_63_16: and s_63_8 s_63_15
        let s_63_16: u64 = ((s_63_8) & (s_63_15));
        // D s_63_17: select s_63_12 s_63_14 s_63_16
        let s_63_17: u64 = if s_63_12 { s_63_14 } else { s_63_16 };
        // D s_63_18: cast trunc s_63_17 -> u8
        let s_63_18: bool = ((s_63_17) != 0);
        // D s_63_19: cast zx s_63_18 -> bv
        let s_63_19: Bits = Bits::new(s_63_18 as u128, 1u16);
        // C s_63_20: const #1u : u8
        let s_63_20: bool = true;
        // C s_63_21: cast zx s_63_20 -> bv
        let s_63_21: Bits = Bits::new(s_63_20 as u128, 1u16);
        // D s_63_22: cmp-eq s_63_19 s_63_21
        let s_63_22: bool = ((s_63_19) == (s_63_21));
        // D s_63_23: write-var gs#25187 <= s_63_22
        fn_state.gs_25187 = s_63_22;
        // N s_63_24: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #4s : i
        let s_64_0: i128 = 4;
        // D s_64_1: read-var majorshadow#483:i
        let s_64_1: i128 = fn_state.majorshadow_483;
        // D s_64_2: cmp-eq s_64_1 s_64_0
        let s_64_2: bool = ((s_64_1) == (s_64_0));
        // N s_64_3: branch s_64_2 b67 b65
        if s_64_2 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #14s : i
        let s_65_0: i128 = 14;
        // D s_65_1: read-var majorshadow#483:i
        let s_65_1: i128 = fn_state.majorshadow_483;
        // D s_65_2: cmp-eq s_65_1 s_65_0
        let s_65_2: bool = ((s_65_1) == (s_65_0));
        // D s_65_3: write-var gs#25184 <= s_65_2
        fn_state.gs_25184 = s_65_2;
        // N s_65_4: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var gs#25184:u8
        let s_66_0: bool = fn_state.gs_25184;
        // D s_66_1: not s_66_0
        let s_66_1: bool = !s_66_0;
        // D s_66_2: write-var gs#25185 <= s_66_1
        fn_state.gs_25185 = s_66_1;
        // N s_66_3: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #1u : u8
        let s_67_0: bool = true;
        // D s_67_1: write-var gs#25184 <= s_67_0
        fn_state.gs_25184 = s_67_0;
        // N s_67_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var majorshadow#483:i
        let s_68_0: i128 = fn_state.majorshadow_483;
        // D s_68_1: call __id(s_68_0)
        let s_68_1: i128 = u__id(state, tracer, s_68_0);
        // C s_68_2: const #64s : i
        let s_68_2: i128 = 64;
        // D s_68_3: cmp-lt s_68_1 s_68_2
        let s_68_3: bool = ((s_68_1) < (s_68_2));
        // D s_68_4: write-var gs#25181 <= s_68_3
        fn_state.gs_25181 = s_68_3;
        // N s_68_5: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var CRn:i
        let s_69_0: i128 = fn_state.CRn;
        // D s_69_1: write-var ga#19635 <= s_69_0
        fn_state.ga_19635 = s_69_0;
        // N s_69_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #3u : u8
        let s_70_0: u8 = 3;
        // C s_70_1: cast zx s_70_0 -> bv
        let s_70_1: Bits = Bits::new(s_70_0 as u128, 4u16);
        // C s_70_2: cast zx s_70_1 -> i
        let s_70_2: i128 = (s_70_1.value() as i128);
        // C s_70_3: cast reint s_70_2 -> i64
        let s_70_3: i64 = (s_70_2 as i64);
        // C s_70_4: cast zx s_70_3 -> i
        let s_70_4: i128 = (i128::try_from(s_70_3).unwrap());
        // C s_70_5: const #432u : u32
        let s_70_5: u32 = 432;
        // D s_70_6: read-reg s_70_5:u8
        let s_70_6: u8 = {
            let value = state.read_register::<u8>(s_70_5 as isize);
            tracer.read_register(s_70_5 as isize, value);
            value
        };
        // D s_70_7: call AArch64_AArch32SystemAccessTrap(s_70_6, s_70_4)
        let s_70_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_70_6, s_70_4);
        // N s_70_8: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var trapped_encoding:u8
        let s_71_0: bool = fn_state.trapped_encoding;
        // D s_71_1: write-var gs#25175 <= s_71_0
        fn_state.gs_25175 = s_71_0;
        // N s_71_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #20784u : u32
        let s_72_0: u32 = 20784;
        // D s_72_1: read-reg s_72_0:struct
        let s_72_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_0 as isize);
            tracer.read_register(s_72_0 as isize, value);
            value
        };
        // D s_72_2: call _get_SCTLR_EL2_Type_TIDCP(s_72_1)
        let s_72_2: bool = u_get_SCTLR_EL2_Type_TIDCP(state, tracer, s_72_1);
        // D s_72_3: cast zx s_72_2 -> bv
        let s_72_3: Bits = Bits::new(s_72_2 as u128, 1u16);
        // C s_72_4: const #1u : u8
        let s_72_4: bool = true;
        // C s_72_5: cast zx s_72_4 -> bv
        let s_72_5: Bits = Bits::new(s_72_4 as u128, 1u16);
        // D s_72_6: cmp-eq s_72_3 s_72_5
        let s_72_6: bool = ((s_72_3) == (s_72_5));
        // D s_72_7: write-var gs#25174 <= s_72_6
        fn_state.gs_25174 = s_72_6;
        // N s_72_8: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #() : ()
        let s_73_0: () = ();
        // S s_73_1: call IsInHost(s_73_0)
        let s_73_1: bool = IsInHost(state, tracer, s_73_0);
        // D s_73_2: write-var gs#25173 <= s_73_1
        fn_state.gs_25173 = s_73_1;
        // N s_73_3: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #16975u : u32
        let s_74_0: u32 = 16975;
        // D s_74_1: read-reg s_74_0:u8
        let s_74_1: u8 = {
            let value = state.read_register::<u8>(s_74_0 as isize);
            tracer.read_register(s_74_0 as isize, value);
            value
        };
        // D s_74_2: cast zx s_74_1 -> bv
        let s_74_2: Bits = Bits::new(s_74_1 as u128, 2u16);
        // C s_74_3: const #448u : u32
        let s_74_3: u32 = 448;
        // D s_74_4: read-reg s_74_3:u8
        let s_74_4: u8 = {
            let value = state.read_register::<u8>(s_74_3 as isize);
            tracer.read_register(s_74_3 as isize, value);
            value
        };
        // D s_74_5: cast zx s_74_4 -> bv
        let s_74_5: Bits = Bits::new(s_74_4 as u128, 2u16);
        // D s_74_6: cmp-eq s_74_2 s_74_5
        let s_74_6: bool = ((s_74_2) == (s_74_5));
        // D s_74_7: write-var gs#25172 <= s_74_6
        fn_state.gs_25172 = s_74_6;
        // N s_74_8: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #() : ()
        let s_75_0: () = ();
        // S s_75_1: call EL2Enabled(s_75_0)
        let s_75_1: bool = EL2Enabled(state, tracer, s_75_0);
        // D s_75_2: write-var gs#25170 <= s_75_1
        fn_state.gs_25170 = s_75_1;
        // N s_75_3: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #1u : u8
        let s_76_0: bool = true;
        // D s_76_1: write-var gs#25169 <= s_76_0
        fn_state.gs_25169 = s_76_0;
        // N s_76_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #() : ()
        let s_77_0: () = ();
        // S s_77_1: call EL2Enabled(s_77_0)
        let s_77_1: bool = EL2Enabled(state, tracer, s_77_0);
        // N s_77_2: branch s_77_1 b82 b78
        if s_77_1 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_78(state, tracer, fn_state);
        };
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #0u : u8
        let s_78_0: bool = false;
        // D s_78_1: write-var gs#25198 <= s_78_0
        fn_state.gs_25198 = s_78_0;
        // N s_78_2: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var gs#25198:u8
        let s_79_0: bool = fn_state.gs_25198;
        // N s_79_1: branch s_79_0 b81 b80
        if s_79_0 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_80(state, tracer, fn_state);
        };
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #3u : u8
        let s_80_0: u8 = 3;
        // C s_80_1: cast zx s_80_0 -> bv
        let s_80_1: Bits = Bits::new(s_80_0 as u128, 4u16);
        // C s_80_2: cast zx s_80_1 -> i
        let s_80_2: i128 = (s_80_1.value() as i128);
        // C s_80_3: cast reint s_80_2 -> i64
        let s_80_3: i64 = (s_80_2 as i64);
        // C s_80_4: cast zx s_80_3 -> i
        let s_80_4: i128 = (i128::try_from(s_80_3).unwrap());
        // C s_80_5: const #440u : u32
        let s_80_5: u32 = 440;
        // D s_80_6: read-reg s_80_5:u8
        let s_80_6: u8 = {
            let value = state.read_register::<u8>(s_80_5 as isize);
            tracer.read_register(s_80_5 as isize, value);
            value
        };
        // D s_80_7: call AArch64_AArch32SystemAccessTrap(s_80_6, s_80_4)
        let s_80_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_80_6, s_80_4);
        // N s_80_8: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #3u : u8
        let s_81_0: u8 = 3;
        // C s_81_1: cast zx s_81_0 -> bv
        let s_81_1: Bits = Bits::new(s_81_0 as u128, 4u16);
        // C s_81_2: cast zx s_81_1 -> i
        let s_81_2: i128 = (s_81_1.value() as i128);
        // C s_81_3: cast reint s_81_2 -> i64
        let s_81_3: i64 = (s_81_2 as i64);
        // C s_81_4: cast zx s_81_3 -> i
        let s_81_4: i128 = (i128::try_from(s_81_3).unwrap());
        // C s_81_5: const #432u : u32
        let s_81_5: u32 = 432;
        // D s_81_6: read-reg s_81_5:u8
        let s_81_6: u8 = {
            let value = state.read_register::<u8>(s_81_5 as isize);
            tracer.read_register(s_81_5 as isize, value);
            value
        };
        // D s_81_7: call AArch64_AArch32SystemAccessTrap(s_81_6, s_81_4)
        let s_81_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_81_6, s_81_4);
        // N s_81_8: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #102552u : u32
        let s_82_0: u32 = 102552;
        // D s_82_1: read-reg s_82_0:struct
        let s_82_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_0 as isize);
            tracer.read_register(s_82_0 as isize, value);
            value
        };
        // D s_82_2: call _get_HCR_EL2_Type_TGE(s_82_1)
        let s_82_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_82_1);
        // D s_82_3: cast zx s_82_2 -> bv
        let s_82_3: Bits = Bits::new(s_82_2 as u128, 1u16);
        // C s_82_4: const #1u : u8
        let s_82_4: bool = true;
        // C s_82_5: cast zx s_82_4 -> bv
        let s_82_5: Bits = Bits::new(s_82_4 as u128, 1u16);
        // D s_82_6: cmp-eq s_82_3 s_82_5
        let s_82_6: bool = ((s_82_3) == (s_82_5));
        // D s_82_7: write-var gs#25198 <= s_82_6
        fn_state.gs_25198 = s_82_6;
        // N s_82_8: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_83_0: read-var trapped_encoding:u8
        let s_83_0: bool = fn_state.trapped_encoding;
        // D s_83_1: write-var gs#25168 <= s_83_0
        fn_state.gs_25168 = s_83_0;
        // N s_83_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #90272u : u32
        let s_84_0: u32 = 90272;
        // D s_84_1: read-reg s_84_0:struct
        let s_84_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_84_0 as isize);
            tracer.read_register(s_84_0 as isize, value);
            value
        };
        // D s_84_2: call _get_SCTLR_EL1_Type_TIDCP(s_84_1)
        let s_84_2: bool = u_get_SCTLR_EL1_Type_TIDCP(state, tracer, s_84_1);
        // D s_84_3: cast zx s_84_2 -> bv
        let s_84_3: Bits = Bits::new(s_84_2 as u128, 1u16);
        // C s_84_4: const #1u : u8
        let s_84_4: bool = true;
        // C s_84_5: cast zx s_84_4 -> bv
        let s_84_5: Bits = Bits::new(s_84_4 as u128, 1u16);
        // D s_84_6: cmp-eq s_84_3 s_84_5
        let s_84_6: bool = ((s_84_3) == (s_84_5));
        // D s_84_7: write-var gs#25167 <= s_84_6
        fn_state.gs_25167 = s_84_6;
        // N s_84_8: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #440u : u32
        let s_85_0: u32 = 440;
        // D s_85_1: read-reg s_85_0:u8
        let s_85_1: u8 = {
            let value = state.read_register::<u8>(s_85_0 as isize);
            tracer.read_register(s_85_0 as isize, value);
            value
        };
        // D s_85_2: call ELUsingAArch32(s_85_1)
        let s_85_2: bool = ELUsingAArch32(state, tracer, s_85_1);
        // D s_85_3: not s_85_2
        let s_85_3: bool = !s_85_2;
        // D s_85_4: write-var gs#25166 <= s_85_3
        fn_state.gs_25166 = s_85_3;
        // N s_85_5: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #() : ()
        let s_86_0: () = ();
        // S s_86_1: call IsInHost(s_86_0)
        let s_86_1: bool = IsInHost(state, tracer, s_86_0);
        // S s_86_2: not s_86_1
        let s_86_2: bool = !s_86_1;
        // D s_86_3: write-var gs#25165 <= s_86_2
        fn_state.gs_25165 = s_86_2;
        // N s_86_4: jump b14
        return block_14(state, tracer, fn_state);
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
        // D s_87_6: cmp-eq s_87_2 s_87_5
        let s_87_6: bool = ((s_87_2) == (s_87_5));
        // D s_87_7: write-var gs#25164 <= s_87_6
        fn_state.gs_25164 = s_87_6;
        // N s_87_8: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #0s : i
        let s_88_0: i128 = 0;
        // D s_88_1: read-var CRm:i
        let s_88_1: i128 = fn_state.CRm;
        // D s_88_2: cmp-eq s_88_1 s_88_0
        let s_88_2: bool = ((s_88_1) == (s_88_0));
        // N s_88_3: branch s_88_2 b115 b89
        if s_88_2 {
            return block_115(state, tracer, fn_state);
        } else {
            return block_89(state, tracer, fn_state);
        };
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #1s : i
        let s_89_0: i128 = 1;
        // D s_89_1: read-var CRm:i
        let s_89_1: i128 = fn_state.CRm;
        // D s_89_2: cmp-eq s_89_1 s_89_0
        let s_89_2: bool = ((s_89_1) == (s_89_0));
        // N s_89_3: branch s_89_2 b114 b90
        if s_89_2 {
            return block_114(state, tracer, fn_state);
        } else {
            return block_90(state, tracer, fn_state);
        };
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #2s : i
        let s_90_0: i128 = 2;
        // D s_90_1: read-var CRm:i
        let s_90_1: i128 = fn_state.CRm;
        // D s_90_2: cmp-eq s_90_1 s_90_0
        let s_90_2: bool = ((s_90_1) == (s_90_0));
        // N s_90_3: branch s_90_2 b113 b91
        if s_90_2 {
            return block_113(state, tracer, fn_state);
        } else {
            return block_91(state, tracer, fn_state);
        };
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #3s : i
        let s_91_0: i128 = 3;
        // D s_91_1: read-var CRm:i
        let s_91_1: i128 = fn_state.CRm;
        // D s_91_2: cmp-eq s_91_1 s_91_0
        let s_91_2: bool = ((s_91_1) == (s_91_0));
        // N s_91_3: branch s_91_2 b112 b92
        if s_91_2 {
            return block_112(state, tracer, fn_state);
        } else {
            return block_92(state, tracer, fn_state);
        };
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #4s : i
        let s_92_0: i128 = 4;
        // D s_92_1: read-var CRm:i
        let s_92_1: i128 = fn_state.CRm;
        // D s_92_2: cmp-eq s_92_1 s_92_0
        let s_92_2: bool = ((s_92_1) == (s_92_0));
        // N s_92_3: branch s_92_2 b111 b93
        if s_92_2 {
            return block_111(state, tracer, fn_state);
        } else {
            return block_93(state, tracer, fn_state);
        };
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #5s : i
        let s_93_0: i128 = 5;
        // D s_93_1: read-var CRm:i
        let s_93_1: i128 = fn_state.CRm;
        // D s_93_2: cmp-eq s_93_1 s_93_0
        let s_93_2: bool = ((s_93_1) == (s_93_0));
        // N s_93_3: branch s_93_2 b110 b94
        if s_93_2 {
            return block_110(state, tracer, fn_state);
        } else {
            return block_94(state, tracer, fn_state);
        };
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #6s : i
        let s_94_0: i128 = 6;
        // D s_94_1: read-var CRm:i
        let s_94_1: i128 = fn_state.CRm;
        // D s_94_2: cmp-eq s_94_1 s_94_0
        let s_94_2: bool = ((s_94_1) == (s_94_0));
        // N s_94_3: branch s_94_2 b109 b95
        if s_94_2 {
            return block_109(state, tracer, fn_state);
        } else {
            return block_95(state, tracer, fn_state);
        };
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #7s : i
        let s_95_0: i128 = 7;
        // D s_95_1: read-var CRm:i
        let s_95_1: i128 = fn_state.CRm;
        // D s_95_2: cmp-eq s_95_1 s_95_0
        let s_95_2: bool = ((s_95_1) == (s_95_0));
        // N s_95_3: branch s_95_2 b108 b96
        if s_95_2 {
            return block_108(state, tracer, fn_state);
        } else {
            return block_96(state, tracer, fn_state);
        };
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_96_0: const #8s : i
        let s_96_0: i128 = 8;
        // D s_96_1: read-var CRm:i
        let s_96_1: i128 = fn_state.CRm;
        // D s_96_2: cmp-eq s_96_1 s_96_0
        let s_96_2: bool = ((s_96_1) == (s_96_0));
        // N s_96_3: branch s_96_2 b107 b97
        if s_96_2 {
            return block_107(state, tracer, fn_state);
        } else {
            return block_97(state, tracer, fn_state);
        };
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #15s : i
        let s_97_0: i128 = 15;
        // D s_97_1: read-var CRm:i
        let s_97_1: i128 = fn_state.CRm;
        // D s_97_2: cmp-eq s_97_1 s_97_0
        let s_97_2: bool = ((s_97_1) == (s_97_0));
        // D s_97_3: write-var gs#25153 <= s_97_2
        fn_state.gs_25153 = s_97_2;
        // N s_97_4: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_98_0: read-var gs#25153:u8
        let s_98_0: bool = fn_state.gs_25153;
        // D s_98_1: write-var gs#25154 <= s_98_0
        fn_state.gs_25154 = s_98_0;
        // N s_98_2: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_99_0: read-var gs#25154:u8
        let s_99_0: bool = fn_state.gs_25154;
        // D s_99_1: write-var gs#25155 <= s_99_0
        fn_state.gs_25155 = s_99_0;
        // N s_99_2: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_100_0: read-var gs#25155:u8
        let s_100_0: bool = fn_state.gs_25155;
        // D s_100_1: write-var gs#25156 <= s_100_0
        fn_state.gs_25156 = s_100_0;
        // N s_100_2: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_101_0: read-var gs#25156:u8
        let s_101_0: bool = fn_state.gs_25156;
        // D s_101_1: write-var gs#25157 <= s_101_0
        fn_state.gs_25157 = s_101_0;
        // N s_101_2: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_102_0: read-var gs#25157:u8
        let s_102_0: bool = fn_state.gs_25157;
        // D s_102_1: write-var gs#25158 <= s_102_0
        fn_state.gs_25158 = s_102_0;
        // N s_102_2: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_103_0: read-var gs#25158:u8
        let s_103_0: bool = fn_state.gs_25158;
        // D s_103_1: write-var gs#25159 <= s_103_0
        fn_state.gs_25159 = s_103_0;
        // N s_103_2: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_104_0: read-var gs#25159:u8
        let s_104_0: bool = fn_state.gs_25159;
        // D s_104_1: write-var gs#25160 <= s_104_0
        fn_state.gs_25160 = s_104_0;
        // N s_104_2: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_105_0: read-var gs#25160:u8
        let s_105_0: bool = fn_state.gs_25160;
        // D s_105_1: write-var gs#25161 <= s_105_0
        fn_state.gs_25161 = s_105_0;
        // N s_105_2: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_106_0: read-var gs#25161:u8
        let s_106_0: bool = fn_state.gs_25161;
        // D s_106_1: write-var gs#25162 <= s_106_0
        fn_state.gs_25162 = s_106_0;
        // N s_106_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #1u : u8
        let s_107_0: bool = true;
        // D s_107_1: write-var gs#25153 <= s_107_0
        fn_state.gs_25153 = s_107_0;
        // N s_107_2: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_108_0: const #1u : u8
        let s_108_0: bool = true;
        // D s_108_1: write-var gs#25154 <= s_108_0
        fn_state.gs_25154 = s_108_0;
        // N s_108_2: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #1u : u8
        let s_109_0: bool = true;
        // D s_109_1: write-var gs#25155 <= s_109_0
        fn_state.gs_25155 = s_109_0;
        // N s_109_2: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_110_0: const #1u : u8
        let s_110_0: bool = true;
        // D s_110_1: write-var gs#25156 <= s_110_0
        fn_state.gs_25156 = s_110_0;
        // N s_110_2: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_111_0: const #1u : u8
        let s_111_0: bool = true;
        // D s_111_1: write-var gs#25157 <= s_111_0
        fn_state.gs_25157 = s_111_0;
        // N s_111_2: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_112_0: const #1u : u8
        let s_112_0: bool = true;
        // D s_112_1: write-var gs#25158 <= s_112_0
        fn_state.gs_25158 = s_112_0;
        // N s_112_2: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_113_0: const #1u : u8
        let s_113_0: bool = true;
        // D s_113_1: write-var gs#25159 <= s_113_0
        fn_state.gs_25159 = s_113_0;
        // N s_113_2: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #1u : u8
        let s_114_0: bool = true;
        // D s_114_1: write-var gs#25160 <= s_114_0
        fn_state.gs_25160 = s_114_0;
        // N s_114_2: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_115_0: const #1u : u8
        let s_115_0: bool = true;
        // D s_115_1: write-var gs#25161 <= s_115_0
        fn_state.gs_25161 = s_115_0;
        // N s_115_2: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_116_0: const #1u : u8
        let s_116_0: bool = true;
        // D s_116_1: write-var gs#25163 <= s_116_0
        fn_state.gs_25163 = s_116_0;
        // N s_116_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_117_0: const #0s : i
        let s_117_0: i128 = 0;
        // D s_117_1: read-var CRm:i
        let s_117_1: i128 = fn_state.CRm;
        // D s_117_2: cmp-eq s_117_1 s_117_0
        let s_117_2: bool = ((s_117_1) == (s_117_0));
        // N s_117_3: branch s_117_2 b126 b118
        if s_117_2 {
            return block_126(state, tracer, fn_state);
        } else {
            return block_118(state, tracer, fn_state);
        };
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #1s : i
        let s_118_0: i128 = 1;
        // D s_118_1: read-var CRm:i
        let s_118_1: i128 = fn_state.CRm;
        // D s_118_2: cmp-eq s_118_1 s_118_0
        let s_118_2: bool = ((s_118_1) == (s_118_0));
        // N s_118_3: branch s_118_2 b125 b119
        if s_118_2 {
            return block_125(state, tracer, fn_state);
        } else {
            return block_119(state, tracer, fn_state);
        };
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_119_0: const #4s : i
        let s_119_0: i128 = 4;
        // D s_119_1: read-var CRm:i
        let s_119_1: i128 = fn_state.CRm;
        // D s_119_2: cmp-eq s_119_1 s_119_0
        let s_119_2: bool = ((s_119_1) == (s_119_0));
        // N s_119_3: branch s_119_2 b124 b120
        if s_119_2 {
            return block_124(state, tracer, fn_state);
        } else {
            return block_120(state, tracer, fn_state);
        };
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_120_0: const #8s : i
        let s_120_0: i128 = 8;
        // D s_120_1: read-var CRm:i
        let s_120_1: i128 = fn_state.CRm;
        // D s_120_2: cmp-eq s_120_1 s_120_0
        let s_120_2: bool = ((s_120_1) == (s_120_0));
        // D s_120_3: write-var gs#25137 <= s_120_2
        fn_state.gs_25137 = s_120_2;
        // N s_120_4: jump b121
        return block_121(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_121_0: read-var gs#25137:u8
        let s_121_0: bool = fn_state.gs_25137;
        // D s_121_1: write-var gs#25138 <= s_121_0
        fn_state.gs_25138 = s_121_0;
        // N s_121_2: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_122_0: read-var gs#25138:u8
        let s_122_0: bool = fn_state.gs_25138;
        // D s_122_1: write-var gs#25139 <= s_122_0
        fn_state.gs_25139 = s_122_0;
        // N s_122_2: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_123_0: read-var gs#25139:u8
        let s_123_0: bool = fn_state.gs_25139;
        // D s_123_1: write-var gs#25140 <= s_123_0
        fn_state.gs_25140 = s_123_0;
        // N s_123_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_124_0: const #1u : u8
        let s_124_0: bool = true;
        // D s_124_1: write-var gs#25137 <= s_124_0
        fn_state.gs_25137 = s_124_0;
        // N s_124_2: jump b121
        return block_121(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_125_0: const #1u : u8
        let s_125_0: bool = true;
        // D s_125_1: write-var gs#25138 <= s_125_0
        fn_state.gs_25138 = s_125_0;
        // N s_125_2: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_126_0: const #1u : u8
        let s_126_0: bool = true;
        // D s_126_1: write-var gs#25139 <= s_126_0
        fn_state.gs_25139 = s_126_0;
        // N s_126_2: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_127_0: const #1u : u8
        let s_127_0: bool = true;
        // D s_127_1: write-var gs#25141 <= s_127_0
        fn_state.gs_25141 = s_127_0;
        // N s_127_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_128_0: const #0s : i
        let s_128_0: i128 = 0;
        // D s_128_1: read-var CRm:i
        let s_128_1: i128 = fn_state.CRm;
        // D s_128_2: cmp-eq s_128_1 s_128_0
        let s_128_2: bool = ((s_128_1) == (s_128_0));
        // N s_128_3: branch s_128_2 b146 b129
        if s_128_2 {
            return block_146(state, tracer, fn_state);
        } else {
            return block_129(state, tracer, fn_state);
        };
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_129_0: const #1s : i
        let s_129_0: i128 = 1;
        // D s_129_1: read-var CRm:i
        let s_129_1: i128 = fn_state.CRm;
        // D s_129_2: cmp-eq s_129_1 s_129_0
        let s_129_2: bool = ((s_129_1) == (s_129_0));
        // N s_129_3: branch s_129_2 b145 b130
        if s_129_2 {
            return block_145(state, tracer, fn_state);
        } else {
            return block_130(state, tracer, fn_state);
        };
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_130_0: const #2s : i
        let s_130_0: i128 = 2;
        // D s_130_1: read-var CRm:i
        let s_130_1: i128 = fn_state.CRm;
        // D s_130_2: cmp-eq s_130_1 s_130_0
        let s_130_2: bool = ((s_130_1) == (s_130_0));
        // N s_130_3: branch s_130_2 b144 b131
        if s_130_2 {
            return block_144(state, tracer, fn_state);
        } else {
            return block_131(state, tracer, fn_state);
        };
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_131_0: const #5s : i
        let s_131_0: i128 = 5;
        // D s_131_1: read-var CRm:i
        let s_131_1: i128 = fn_state.CRm;
        // D s_131_2: cmp-eq s_131_1 s_131_0
        let s_131_2: bool = ((s_131_1) == (s_131_0));
        // N s_131_3: branch s_131_2 b143 b132
        if s_131_2 {
            return block_143(state, tracer, fn_state);
        } else {
            return block_132(state, tracer, fn_state);
        };
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_132_0: const #6s : i
        let s_132_0: i128 = 6;
        // D s_132_1: read-var CRm:i
        let s_132_1: i128 = fn_state.CRm;
        // D s_132_2: cmp-eq s_132_1 s_132_0
        let s_132_2: bool = ((s_132_1) == (s_132_0));
        // N s_132_3: branch s_132_2 b142 b133
        if s_132_2 {
            return block_142(state, tracer, fn_state);
        } else {
            return block_133(state, tracer, fn_state);
        };
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_133_0: const #7s : i
        let s_133_0: i128 = 7;
        // D s_133_1: read-var CRm:i
        let s_133_1: i128 = fn_state.CRm;
        // D s_133_2: cmp-eq s_133_1 s_133_0
        let s_133_2: bool = ((s_133_1) == (s_133_0));
        // N s_133_3: branch s_133_2 b141 b134
        if s_133_2 {
            return block_141(state, tracer, fn_state);
        } else {
            return block_134(state, tracer, fn_state);
        };
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_134_0: const #8s : i
        let s_134_0: i128 = 8;
        // D s_134_1: read-var CRm:i
        let s_134_1: i128 = fn_state.CRm;
        // D s_134_2: cmp-eq s_134_1 s_134_0
        let s_134_2: bool = ((s_134_1) == (s_134_0));
        // D s_134_3: write-var gs#25125 <= s_134_2
        fn_state.gs_25125 = s_134_2;
        // N s_134_4: jump b135
        return block_135(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_135_0: read-var gs#25125:u8
        let s_135_0: bool = fn_state.gs_25125;
        // D s_135_1: write-var gs#25126 <= s_135_0
        fn_state.gs_25126 = s_135_0;
        // N s_135_2: jump b136
        return block_136(state, tracer, fn_state);
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_136_0: read-var gs#25126:u8
        let s_136_0: bool = fn_state.gs_25126;
        // D s_136_1: write-var gs#25127 <= s_136_0
        fn_state.gs_25127 = s_136_0;
        // N s_136_2: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_137_0: read-var gs#25127:u8
        let s_137_0: bool = fn_state.gs_25127;
        // D s_137_1: write-var gs#25128 <= s_137_0
        fn_state.gs_25128 = s_137_0;
        // N s_137_2: jump b138
        return block_138(state, tracer, fn_state);
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_138_0: read-var gs#25128:u8
        let s_138_0: bool = fn_state.gs_25128;
        // D s_138_1: write-var gs#25129 <= s_138_0
        fn_state.gs_25129 = s_138_0;
        // N s_138_2: jump b139
        return block_139(state, tracer, fn_state);
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_139_0: read-var gs#25129:u8
        let s_139_0: bool = fn_state.gs_25129;
        // D s_139_1: write-var gs#25130 <= s_139_0
        fn_state.gs_25130 = s_139_0;
        // N s_139_2: jump b140
        return block_140(state, tracer, fn_state);
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_140_0: read-var gs#25130:u8
        let s_140_0: bool = fn_state.gs_25130;
        // D s_140_1: write-var gs#25131 <= s_140_0
        fn_state.gs_25131 = s_140_0;
        // N s_140_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_141_0: const #1u : u8
        let s_141_0: bool = true;
        // D s_141_1: write-var gs#25125 <= s_141_0
        fn_state.gs_25125 = s_141_0;
        // N s_141_2: jump b135
        return block_135(state, tracer, fn_state);
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_142_0: const #1u : u8
        let s_142_0: bool = true;
        // D s_142_1: write-var gs#25126 <= s_142_0
        fn_state.gs_25126 = s_142_0;
        // N s_142_2: jump b136
        return block_136(state, tracer, fn_state);
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_143_0: const #1u : u8
        let s_143_0: bool = true;
        // D s_143_1: write-var gs#25127 <= s_143_0
        fn_state.gs_25127 = s_143_0;
        // N s_143_2: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_144_0: const #1u : u8
        let s_144_0: bool = true;
        // D s_144_1: write-var gs#25128 <= s_144_0
        fn_state.gs_25128 = s_144_0;
        // N s_144_2: jump b138
        return block_138(state, tracer, fn_state);
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_145_0: const #1u : u8
        let s_145_0: bool = true;
        // D s_145_1: write-var gs#25129 <= s_145_0
        fn_state.gs_25129 = s_145_0;
        // N s_145_2: jump b139
        return block_139(state, tracer, fn_state);
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_146_0: const #1u : u8
        let s_146_0: bool = true;
        // D s_146_1: write-var gs#25130 <= s_146_0
        fn_state.gs_25130 = s_146_0;
        // N s_146_2: jump b140
        return block_140(state, tracer, fn_state);
    }
}
