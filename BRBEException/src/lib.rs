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
use u_get_MDCR_EL3_Type_E3BREW::*;
use BranchEncCycleCount::*;
use AArch64_BranchAddr::*;
use UpdateBranchRecordBuffer::*;
use PMUEvent::*;
use Unreachable::*;
use u_get_BRBCR_EL2_Type_EXCEPTION::*;
use u_get_BRBCR_EL1_Type_EXCEPTION::*;
use u_get_BRBFCR_EL1_Type_LASTFAILED::*;
use Zeros::*;
use u_get_MDCR_EL3_Type_E3BREC::*;
use HaveBRBEv1p1::*;
use BranchRecordAllowed::*;
use HaveTME::*;
use common::*;
pub fn BRBEException<T: Tracer>(
    state: &mut State,
    tracer: &T,
    erec: ProductTypeb7f99f96751e17c4,
    preferred_exception_return: u64,
    target_address_in: u64,
    target_el: u8,
    trappedsyscallinst: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_6166: bool,
        iss: u32,
        tv: bool,
        target_address: u64,
        branch_type: u8,
        source_valid: bool,
        source_address: u64,
        gs_6229: bool,
        ga_4083: ProductType1a93b8c16f53fb84,
        el: u8,
        sv: bool,
        branch_valid: bool,
        except: u32,
        target_valid: bool,
        gs_6231: bool,
        lastfailed: bool,
        cc: u16,
        transactional: bool,
        gs_6175: bool,
        ccu: bool,
        erec: ProductTypeb7f99f96751e17c4,
        preferred_exception_return: u64,
        target_address_in: u64,
        target_el: u8,
        trappedsyscallinst: bool,
    }
    let fn_state = FunctionState {
        erec,
        preferred_exception_return,
        target_address_in,
        target_el,
        trappedsyscallinst,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var target_address_in:u64
        let s_0_0: u64 = fn_state.target_address_in;
        // D s_0_1: write-var target_address <= s_0_0
        fn_state.target_address = s_0_0;
        // D s_0_2: read-var erec.1:struct
        let s_0_2: u32 = fn_state.erec._1;
        // D s_0_3: write-var except <= s_0_2
        fn_state.except = s_0_2;
        // D s_0_4: read-var erec.6:struct
        let s_0_4: u32 = fn_state.erec._6;
        // D s_0_5: write-var iss <= s_0_4
        fn_state.iss = s_0_4;
        // D s_0_6: read-var target_el:u8
        let s_0_6: u8 = fn_state.target_el;
        // D s_0_7: cast zx s_0_6 -> bv
        let s_0_7: Bits = Bits::new(s_0_6 as u128, 2u16);
        // C s_0_8: const #424u : u32
        let s_0_8: u32 = 424;
        // D s_0_9: read-reg s_0_8:u8
        let s_0_9: u8 = {
            let value = state.read_register::<u8>(s_0_8 as isize);
            tracer.read_register(s_0_8 as isize, value);
            value
        };
        // D s_0_10: cast zx s_0_9 -> bv
        let s_0_10: Bits = Bits::new(s_0_9 as u128, 2u16);
        // D s_0_11: cmp-eq s_0_7 s_0_10
        let s_0_11: bool = ((s_0_7) == (s_0_10));
        // D s_0_12: not s_0_11
        let s_0_12: bool = !s_0_11;
        // N s_0_13: branch s_0_12 b128 b1
        if s_0_12 {
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
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call HaveBRBEv1p1(s_1_0)
        let s_1_1: bool = HaveBRBEv1p1(state, tracer, s_1_0);
        // S s_1_2: not s_1_1
        let s_1_2: bool = !s_1_1;
        // N s_1_3: branch s_1_2 b127 b2
        if s_1_2 {
            return block_127(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #22712u : u32
        let s_2_0: u32 = 22712;
        // D s_2_1: read-reg s_2_0:struct
        let s_2_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // D s_2_2: call _get_MDCR_EL3_Type_E3BREC(s_2_1)
        let s_2_2: bool = u_get_MDCR_EL3_Type_E3BREC(state, tracer, s_2_1);
        // C s_2_3: const #22712u : u32
        let s_2_3: u32 = 22712;
        // D s_2_4: read-reg s_2_3:struct
        let s_2_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_2_3 as isize);
            tracer.read_register(s_2_3 as isize, value);
            value
        };
        // D s_2_5: call _get_MDCR_EL3_Type_E3BREW(s_2_4)
        let s_2_5: bool = u_get_MDCR_EL3_Type_E3BREW(state, tracer, s_2_4);
        // D s_2_6: cast zx s_2_2 -> bv
        let s_2_6: Bits = Bits::new(s_2_2 as u128, 1u16);
        // D s_2_7: cast zx s_2_5 -> bv
        let s_2_7: Bits = Bits::new(s_2_5 as u128, 1u16);
        // D s_2_8: cmp-eq s_2_6 s_2_7
        let s_2_8: bool = ((s_2_6) == (s_2_7));
        // D s_2_9: write-var gs#6166 <= s_2_8
        fn_state.gs_6166 = s_2_8;
        // N s_2_10: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#6166:u8
        let s_3_0: bool = fn_state.gs_6166;
        // N s_3_1: branch s_3_0 b126 b4
        if s_3_0 {
            return block_126(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #16975u : u32
        let s_5_0: u32 = 16975;
        // D s_5_1: read-reg s_5_0:u8
        let s_5_1: u8 = {
            let value = state.read_register::<u8>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // D s_5_2: call BranchRecordAllowed(s_5_1)
        let s_5_2: bool = BranchRecordAllowed(state, tracer, s_5_1);
        // D s_5_3: write-var source_valid <= s_5_2
        fn_state.source_valid = s_5_2;
        // D s_5_4: read-var target_el:u8
        let s_5_4: u8 = fn_state.target_el;
        // D s_5_5: call BranchRecordAllowed(s_5_4)
        let s_5_5: bool = BranchRecordAllowed(state, tracer, s_5_4);
        // D s_5_6: write-var target_valid <= s_5_5
        fn_state.target_valid = s_5_5;
        // D s_5_7: read-var source_valid:u8
        let s_5_7: bool = fn_state.source_valid;
        // N s_5_8: branch s_5_7 b125 b6
        if s_5_7 {
            return block_125(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var target_valid:u8
        let s_6_0: bool = fn_state.target_valid;
        // D s_6_1: write-var gs#6175 <= s_6_0
        fn_state.gs_6175 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#6175:u8
        let s_7_0: bool = fn_state.gs_6175;
        // D s_7_1: write-var branch_valid <= s_7_0
        fn_state.branch_valid = s_7_0;
        // D s_7_2: read-var branch_valid:u8
        let s_7_2: bool = fn_state.branch_valid;
        // N s_7_3: branch s_7_2 b10 b8
        if s_7_2 {
            return block_10(state, tracer, fn_state);
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
        // D s_9_0: read-var branch_valid:u8
        let s_9_0: bool = fn_state.branch_valid;
        // C s_9_1: const #14032u : u32
        let s_9_1: u32 = 14032;
        // N s_9_2: write-reg s_9_1 <= s_9_0
        let s_9_2: () = {
            state.write_register::<bool>(s_9_1 as isize, s_9_0);
            tracer.write_register(s_9_1 as isize, s_9_0);
        };
        // N s_9_3: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0u : u32
        let s_10_0: u32 = 0;
        // D s_10_1: read-var except:u32
        let s_10_1: u32 = fn_state.except;
        // D s_10_2: cmp-eq s_10_0 s_10_1
        let s_10_2: bool = ((s_10_0) == (s_10_1));
        // D s_10_3: not s_10_2
        let s_10_3: bool = !s_10_2;
        // N s_10_4: branch s_10_3 b40 b11
        if s_10_3 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #35u : u8
        let s_11_0: u8 = 35;
        // D s_11_1: write-var branch_type <= s_11_0
        fn_state.branch_type = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #() : ()
        let s_12_0: () = ();
        // S s_12_1: call BranchEncCycleCount(s_12_0)
        let s_12_1: ProductType1a93b8c16f53fb84 = BranchEncCycleCount(
            state,
            tracer,
            s_12_0,
        );
        // D s_12_2: write-var ga#4083 <= s_12_1
        fn_state.ga_4083 = s_12_1;
        // D s_12_3: read-var ga#4083.0:struct
        let s_12_3: bool = fn_state.ga_4083._0;
        // D s_12_4: read-var ga#4083.1:struct
        let s_12_4: u16 = fn_state.ga_4083._1;
        // D s_12_5: write-var ccu <= s_12_3
        fn_state.ccu = s_12_3;
        // D s_12_6: write-var cc <= s_12_4
        fn_state.cc = s_12_4;
        // C s_12_7: const #() : ()
        let s_12_7: () = ();
        // S s_12_8: call HaveTME(s_12_7)
        let s_12_8: bool = HaveTME(state, tracer, s_12_7);
        // N s_12_9: branch s_12_8 b39 b13
        if s_12_8 {
            return block_39(state, tracer, fn_state);
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
        // D s_13_1: write-var lastfailed <= s_13_0
        fn_state.lastfailed = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var source_valid:u8
        let s_14_0: bool = fn_state.source_valid;
        // N s_14_1: branch s_14_0 b38 b15
        if s_14_0 {
            return block_38(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#6229 <= s_15_0
        fn_state.gs_6229 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#6229:u8
        let s_16_0: bool = fn_state.gs_6229;
        // N s_16_1: branch s_16_0 b37 b17
        if s_16_0 {
            return block_37(state, tracer, fn_state);
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
        // D s_17_1: write-var gs#6231 <= s_17_0
        fn_state.gs_6231 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#6231:u8
        let s_18_0: bool = fn_state.gs_6231;
        // N s_18_1: branch s_18_0 b36 b19
        if s_18_0 {
            return block_36(state, tracer, fn_state);
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
        // D s_19_1: write-var transactional <= s_19_0
        fn_state.transactional = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var target_valid:u8
        let s_20_0: bool = fn_state.target_valid;
        // N s_20_1: branch s_20_0 b35 b21
        if s_20_0 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #0u : u8
        let s_21_0: u8 = 0;
        // D s_21_1: write-var el <= s_21_0
        fn_state.el = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var source_valid:u8
        let s_22_0: bool = fn_state.source_valid;
        // N s_22_1: branch s_22_0 b34 b23
        if s_22_0 {
            return block_34(state, tracer, fn_state);
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
        let s_23_0: bool = false;
        // D s_23_1: write-var sv <= s_23_0
        fn_state.sv = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var target_valid:u8
        let s_24_0: bool = fn_state.target_valid;
        // N s_24_1: branch s_24_0 b33 b25
        if s_24_0 {
            return block_33(state, tracer, fn_state);
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
        // D s_25_1: write-var tv <= s_25_0
        fn_state.tv = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var source_valid:u8
        let s_26_0: bool = fn_state.source_valid;
        // N s_26_1: branch s_26_0 b32 b27
        if s_26_0 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #64s : i
        let s_27_0: i128 = 64;
        // S s_27_1: call Zeros(s_27_0)
        let s_27_1: Bits = Zeros(state, tracer, s_27_0);
        // S s_27_2: cast reint s_27_1 -> u64
        let s_27_2: u64 = (s_27_1.value() as u64);
        // D s_27_3: write-var source_address <= s_27_2
        fn_state.source_address = s_27_2;
        // N s_27_4: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var target_valid:u8
        let s_28_0: bool = fn_state.target_valid;
        // D s_28_1: not s_28_0
        let s_28_1: bool = !s_28_0;
        // N s_28_2: branch s_28_1 b31 b29
        if s_28_1 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var target_address:u64
        let s_29_0: u64 = fn_state.target_address;
        // D s_29_1: read-var target_el:u8
        let s_29_1: u8 = fn_state.target_el;
        // D s_29_2: call AArch64_BranchAddr(s_29_0, s_29_1)
        let s_29_2: u64 = AArch64_BranchAddr(state, tracer, s_29_0, s_29_1);
        // D s_29_3: write-var target_address <= s_29_2
        fn_state.target_address = s_29_2;
        // N s_29_4: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var sv:u8
        let s_30_0: bool = fn_state.sv;
        // D s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 1u16);
        // D s_30_2: read-var tv:u8
        let s_30_2: bool = fn_state.tv;
        // D s_30_3: cast zx s_30_2 -> bv
        let s_30_3: Bits = Bits::new(s_30_2 as u128, 1u16);
        // D s_30_4: cast reint s_30_1 -> u128
        let s_30_4: u128 = (s_30_1.value() as u128);
        // D s_30_5: size-of s_30_1
        let s_30_5: u16 = s_30_1.length();
        // D s_30_6: cast reint s_30_3 -> u128
        let s_30_6: u128 = (s_30_3.value() as u128);
        // D s_30_7: size-of s_30_3
        let s_30_7: u16 = s_30_3.length();
        // D s_30_8: lsl s_30_4 s_30_7
        let s_30_8: u128 = s_30_4 << s_30_7;
        // D s_30_9: or s_30_8 s_30_6
        let s_30_9: u128 = ((s_30_8) | (s_30_6));
        // D s_30_10: add s_30_5 s_30_7
        let s_30_10: u16 = (s_30_5 + s_30_7);
        // D s_30_11: create-bits s_30_9 s_30_10
        let s_30_11: Bits = Bits::new(s_30_9, s_30_10);
        // D s_30_12: cast reint s_30_11 -> u8
        let s_30_12: u8 = (s_30_11.value() as u8);
        // D s_30_13: read-var ccu:u8
        let s_30_13: bool = fn_state.ccu;
        // D s_30_14: read-var cc:u14
        let s_30_14: u16 = fn_state.cc;
        // D s_30_15: read-var lastfailed:u8
        let s_30_15: bool = fn_state.lastfailed;
        // D s_30_16: read-var transactional:u8
        let s_30_16: bool = fn_state.transactional;
        // D s_30_17: read-var branch_type:u8
        let s_30_17: u8 = fn_state.branch_type;
        // D s_30_18: read-var el:u8
        let s_30_18: u8 = fn_state.el;
        // C s_30_19: const #0u : u8
        let s_30_19: bool = false;
        // D s_30_20: read-var source_address:u64
        let s_30_20: u64 = fn_state.source_address;
        // D s_30_21: read-var target_address:u64
        let s_30_21: u64 = fn_state.target_address;
        // D s_30_22: call UpdateBranchRecordBuffer(s_30_13, s_30_14, s_30_15, s_30_16, s_30_17, s_30_18, s_30_19, s_30_12, s_30_20, s_30_21)
        let s_30_22: () = UpdateBranchRecordBuffer(
            state,
            tracer,
            s_30_13,
            s_30_14,
            s_30_15,
            s_30_16,
            s_30_17,
            s_30_18,
            s_30_19,
            s_30_12,
            s_30_20,
            s_30_21,
        );
        // C s_30_23: const #16536u : u32
        let s_30_23: u32 = 16536;
        // D s_30_24: read-reg s_30_23:struct
        let s_30_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_30_23 as isize);
            tracer.read_register(s_30_23 as isize, value);
            value
        };
        // C s_30_25: const #16536u : u32
        let s_30_25: u32 = 16536;
        // N s_30_26: write-reg s_30_25 <= s_30_24
        let s_30_26: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_30_25 as isize, s_30_24);
            tracer.write_register(s_30_25 as isize, s_30_24);
        };
        // C s_30_27: const #216u : u32
        let s_30_27: u32 = 216;
        // D s_30_28: read-reg s_30_27:u16
        let s_30_28: u16 = {
            let value = state.read_register::<u16>(s_30_27 as isize);
            tracer.read_register(s_30_27 as isize, value);
            value
        };
        // D s_30_29: call PMUEvent(s_30_28)
        let s_30_29: () = PMUEvent(state, tracer, s_30_28);
        // N s_30_30: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #64s : i
        let s_31_0: i128 = 64;
        // S s_31_1: call Zeros(s_31_0)
        let s_31_1: Bits = Zeros(state, tracer, s_31_0);
        // S s_31_2: cast reint s_31_1 -> u64
        let s_31_2: u64 = (s_31_1.value() as u64);
        // D s_31_3: write-var target_address <= s_31_2
        fn_state.target_address = s_31_2;
        // N s_31_4: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var preferred_exception_return:u64
        let s_32_0: u64 = fn_state.preferred_exception_return;
        // D s_32_1: write-var source_address <= s_32_0
        fn_state.source_address = s_32_0;
        // N s_32_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #1u : u8
        let s_33_0: bool = true;
        // D s_33_1: write-var tv <= s_33_0
        fn_state.tv = s_33_0;
        // N s_33_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #1u : u8
        let s_34_0: bool = true;
        // D s_34_1: write-var sv <= s_34_0
        fn_state.sv = s_34_0;
        // N s_34_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var target_el:u8
        let s_35_0: u8 = fn_state.target_el;
        // D s_35_1: write-var el <= s_35_0
        fn_state.el = s_35_0;
        // N s_35_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #1u : u8
        let s_36_0: bool = true;
        // D s_36_1: write-var transactional <= s_36_0
        fn_state.transactional = s_36_0;
        // N s_36_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #100180u : u32
        let s_37_0: u32 = 100180;
        // D s_37_1: read-reg s_37_0:i
        let s_37_1: i128 = {
            let value = state.read_register::<i128>(s_37_0 as isize);
            tracer.read_register(s_37_0 as isize, value);
            value
        };
        // C s_37_2: const #0s : i
        let s_37_2: i128 = 0;
        // D s_37_3: cmp-gt s_37_1 s_37_2
        let s_37_3: bool = ((s_37_1) > (s_37_2));
        // D s_37_4: write-var gs#6231 <= s_37_3
        fn_state.gs_6231 = s_37_3;
        // N s_37_5: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #() : ()
        let s_38_0: () = ();
        // S s_38_1: call HaveTME(s_38_0)
        let s_38_1: bool = HaveTME(state, tracer, s_38_0);
        // D s_38_2: write-var gs#6229 <= s_38_1
        fn_state.gs_6229 = s_38_1;
        // N s_38_3: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #16536u : u32
        let s_39_0: u32 = 16536;
        // D s_39_1: read-reg s_39_0:struct
        let s_39_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_39_0 as isize);
            tracer.read_register(s_39_0 as isize, value);
            value
        };
        // D s_39_2: call _get_BRBFCR_EL1_Type_LASTFAILED(s_39_1)
        let s_39_2: bool = u_get_BRBFCR_EL1_Type_LASTFAILED(state, tracer, s_39_1);
        // D s_39_3: write-var lastfailed <= s_39_2
        fn_state.lastfailed = s_39_2;
        // N s_39_4: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #1u : u32
        let s_40_0: u32 = 1;
        // D s_40_1: read-var except:u32
        let s_40_1: u32 = fn_state.except;
        // D s_40_2: cmp-eq s_40_0 s_40_1
        let s_40_2: bool = ((s_40_0) == (s_40_1));
        // D s_40_3: not s_40_2
        let s_40_3: bool = !s_40_2;
        // N s_40_4: branch s_40_3 b42 b41
        if s_40_3 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #35u : u8
        let s_41_0: u8 = 35;
        // D s_41_1: write-var branch_type <= s_41_0
        fn_state.branch_type = s_41_0;
        // N s_41_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #2u : u32
        let s_42_0: u32 = 2;
        // D s_42_1: read-var except:u32
        let s_42_1: u32 = fn_state.except;
        // D s_42_2: cmp-eq s_42_0 s_42_1
        let s_42_2: bool = ((s_42_0) == (s_42_1));
        // D s_42_3: not s_42_2
        let s_42_3: bool = !s_42_2;
        // N s_42_4: branch s_42_3 b44 b43
        if s_42_3 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #35u : u8
        let s_43_0: u8 = 35;
        // D s_43_1: write-var branch_type <= s_43_0
        fn_state.branch_type = s_43_0;
        // N s_43_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #3u : u32
        let s_44_0: u32 = 3;
        // D s_44_1: read-var except:u32
        let s_44_1: u32 = fn_state.except;
        // D s_44_2: cmp-eq s_44_0 s_44_1
        let s_44_2: bool = ((s_44_0) == (s_44_1));
        // D s_44_3: not s_44_2
        let s_44_3: bool = !s_44_2;
        // N s_44_4: branch s_44_3 b46 b45
        if s_44_3 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #35u : u8
        let s_45_0: u8 = 35;
        // D s_45_1: write-var branch_type <= s_45_0
        fn_state.branch_type = s_45_0;
        // N s_45_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #4u : u32
        let s_46_0: u32 = 4;
        // D s_46_1: read-var except:u32
        let s_46_1: u32 = fn_state.except;
        // D s_46_2: cmp-eq s_46_0 s_46_1
        let s_46_2: bool = ((s_46_0) == (s_46_1));
        // D s_46_3: not s_46_2
        let s_46_3: bool = !s_46_2;
        // N s_46_4: branch s_46_3 b48 b47
        if s_46_3 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #35u : u8
        let s_47_0: u8 = 35;
        // D s_47_1: write-var branch_type <= s_47_0
        fn_state.branch_type = s_47_0;
        // N s_47_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #5u : u32
        let s_48_0: u32 = 5;
        // D s_48_1: read-var except:u32
        let s_48_1: u32 = fn_state.except;
        // D s_48_2: cmp-eq s_48_0 s_48_1
        let s_48_2: bool = ((s_48_0) == (s_48_1));
        // D s_48_3: not s_48_2
        let s_48_3: bool = !s_48_2;
        // N s_48_4: branch s_48_3 b50 b49
        if s_48_3 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #35u : u8
        let s_49_0: u8 = 35;
        // D s_49_1: write-var branch_type <= s_49_0
        fn_state.branch_type = s_49_0;
        // N s_49_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #7u : u32
        let s_50_0: u32 = 7;
        // D s_50_1: read-var except:u32
        let s_50_1: u32 = fn_state.except;
        // D s_50_2: cmp-eq s_50_0 s_50_1
        let s_50_2: bool = ((s_50_0) == (s_50_1));
        // D s_50_3: not s_50_2
        let s_50_3: bool = !s_50_2;
        // N s_50_4: branch s_50_3 b52 b51
        if s_50_3 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #35u : u8
        let s_51_0: u8 = 35;
        // D s_51_1: write-var branch_type <= s_51_0
        fn_state.branch_type = s_51_0;
        // N s_51_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #8u : u32
        let s_52_0: u32 = 8;
        // D s_52_1: read-var except:u32
        let s_52_1: u32 = fn_state.except;
        // D s_52_2: cmp-eq s_52_0 s_52_1
        let s_52_2: bool = ((s_52_0) == (s_52_1));
        // D s_52_3: not s_52_2
        let s_52_3: bool = !s_52_2;
        // N s_52_4: branch s_52_3 b54 b53
        if s_52_3 {
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
        // C s_53_0: const #35u : u8
        let s_53_0: u8 = 35;
        // D s_53_1: write-var branch_type <= s_53_0
        fn_state.branch_type = s_53_0;
        // N s_53_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #10u : u32
        let s_54_0: u32 = 10;
        // D s_54_1: read-var except:u32
        let s_54_1: u32 = fn_state.except;
        // D s_54_2: cmp-eq s_54_0 s_54_1
        let s_54_2: bool = ((s_54_0) == (s_54_1));
        // D s_54_3: not s_54_2
        let s_54_3: bool = !s_54_2;
        // N s_54_4: branch s_54_3 b56 b55
        if s_54_3 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #35u : u8
        let s_55_0: u8 = 35;
        // D s_55_1: write-var branch_type <= s_55_0
        fn_state.branch_type = s_55_0;
        // N s_55_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #34u : u32
        let s_56_0: u32 = 34;
        // D s_56_1: read-var except:u32
        let s_56_1: u32 = fn_state.except;
        // D s_56_2: cmp-eq s_56_0 s_56_1
        let s_56_2: bool = ((s_56_0) == (s_56_1));
        // D s_56_3: not s_56_2
        let s_56_3: bool = !s_56_2;
        // N s_56_4: branch s_56_3 b58 b57
        if s_56_3 {
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
        // C s_57_0: const #35u : u8
        let s_57_0: u8 = 35;
        // D s_57_1: write-var branch_type <= s_57_0
        fn_state.branch_type = s_57_0;
        // N s_57_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #6u : u32
        let s_58_0: u32 = 6;
        // D s_58_1: read-var except:u32
        let s_58_1: u32 = fn_state.except;
        // D s_58_2: cmp-eq s_58_0 s_58_1
        let s_58_2: bool = ((s_58_0) == (s_58_1));
        // D s_58_3: not s_58_2
        let s_58_3: bool = !s_58_2;
        // N s_58_4: branch s_58_3 b60 b59
        if s_58_3 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #35u : u8
        let s_59_0: u8 = 35;
        // D s_59_1: write-var branch_type <= s_59_0
        fn_state.branch_type = s_59_0;
        // N s_59_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #36u : u32
        let s_60_0: u32 = 36;
        // D s_60_1: read-var except:u32
        let s_60_1: u32 = fn_state.except;
        // D s_60_2: cmp-eq s_60_0 s_60_1
        let s_60_2: bool = ((s_60_0) == (s_60_1));
        // D s_60_3: not s_60_2
        let s_60_3: bool = !s_60_2;
        // N s_60_4: branch s_60_3 b62 b61
        if s_60_3 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #43u : u8
        let s_61_0: u8 = 43;
        // D s_61_1: write-var branch_type <= s_61_0
        fn_state.branch_type = s_61_0;
        // N s_61_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #11u : u32
        let s_62_0: u32 = 11;
        // D s_62_1: read-var except:u32
        let s_62_1: u32 = fn_state.except;
        // D s_62_2: cmp-eq s_62_0 s_62_1
        let s_62_2: bool = ((s_62_0) == (s_62_1));
        // D s_62_3: not s_62_2
        let s_62_3: bool = !s_62_2;
        // N s_62_4: branch s_62_3 b64 b63
        if s_62_3 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #35u : u8
        let s_63_0: u8 = 35;
        // D s_63_1: write-var branch_type <= s_63_0
        fn_state.branch_type = s_63_0;
        // N s_63_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #12u : u32
        let s_64_0: u32 = 12;
        // D s_64_1: read-var except:u32
        let s_64_1: u32 = fn_state.except;
        // D s_64_2: cmp-eq s_64_0 s_64_1
        let s_64_2: bool = ((s_64_0) == (s_64_1));
        // D s_64_3: not s_64_2
        let s_64_3: bool = !s_64_2;
        // N s_64_4: branch s_64_3 b68 b65
        if s_64_3 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var trappedsyscallinst:u8
        let s_65_0: bool = fn_state.trappedsyscallinst;
        // D s_65_1: not s_65_0
        let s_65_1: bool = !s_65_0;
        // N s_65_2: branch s_65_1 b67 b66
        if s_65_1 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_66(state, tracer, fn_state);
        };
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #35u : u8
        let s_66_0: u8 = 35;
        // D s_66_1: write-var branch_type <= s_66_0
        fn_state.branch_type = s_66_0;
        // N s_66_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #34u : u8
        let s_67_0: u8 = 34;
        // D s_67_1: write-var branch_type <= s_67_0
        fn_state.branch_type = s_67_0;
        // N s_67_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #13u : u32
        let s_68_0: u32 = 13;
        // D s_68_1: read-var except:u32
        let s_68_1: u32 = fn_state.except;
        // D s_68_2: cmp-eq s_68_0 s_68_1
        let s_68_2: bool = ((s_68_0) == (s_68_1));
        // D s_68_3: not s_68_2
        let s_68_3: bool = !s_68_2;
        // N s_68_4: branch s_68_3 b70 b69
        if s_68_3 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #34u : u8
        let s_69_0: u8 = 34;
        // D s_69_1: write-var branch_type <= s_69_0
        fn_state.branch_type = s_69_0;
        // N s_69_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #14u : u32
        let s_70_0: u32 = 14;
        // D s_70_1: read-var except:u32
        let s_70_1: u32 = fn_state.except;
        // D s_70_2: cmp-eq s_70_0 s_70_1
        let s_70_2: bool = ((s_70_0) == (s_70_1));
        // D s_70_3: not s_70_2
        let s_70_3: bool = !s_70_2;
        // N s_70_4: branch s_70_3 b74 b71
        if s_70_3 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var trappedsyscallinst:u8
        let s_71_0: bool = fn_state.trappedsyscallinst;
        // D s_71_1: not s_71_0
        let s_71_1: bool = !s_71_0;
        // N s_71_2: branch s_71_1 b73 b72
        if s_71_1 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_72(state, tracer, fn_state);
        };
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #35u : u8
        let s_72_0: u8 = 35;
        // D s_72_1: write-var branch_type <= s_72_0
        fn_state.branch_type = s_72_0;
        // N s_72_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #34u : u8
        let s_73_0: u8 = 34;
        // D s_73_1: write-var branch_type <= s_73_0
        fn_state.branch_type = s_73_0;
        // N s_73_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #15u : u32
        let s_74_0: u32 = 15;
        // D s_74_1: read-var except:u32
        let s_74_1: u32 = fn_state.except;
        // D s_74_2: cmp-eq s_74_0 s_74_1
        let s_74_2: bool = ((s_74_0) == (s_74_1));
        // D s_74_3: not s_74_2
        let s_74_3: bool = !s_74_2;
        // N s_74_4: branch s_74_3 b76 b75
        if s_74_3 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_75(state, tracer, fn_state);
        };
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #35u : u8
        let s_75_0: u8 = 35;
        // D s_75_1: write-var branch_type <= s_75_0
        fn_state.branch_type = s_75_0;
        // N s_75_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #39u : u32
        let s_76_0: u32 = 39;
        // D s_76_1: read-var except:u32
        let s_76_1: u32 = fn_state.except;
        // D s_76_2: cmp-eq s_76_0 s_76_1
        let s_76_2: bool = ((s_76_0) == (s_76_1));
        // D s_76_3: not s_76_2
        let s_76_3: bool = !s_76_2;
        // N s_76_4: branch s_76_3 b78 b77
        if s_76_3 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_77(state, tracer, fn_state);
        };
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #35u : u8
        let s_77_0: u8 = 35;
        // D s_77_1: write-var branch_type <= s_77_0
        fn_state.branch_type = s_77_0;
        // N s_77_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #32u : u32
        let s_78_0: u32 = 32;
        // D s_78_1: read-var except:u32
        let s_78_1: u32 = fn_state.except;
        // D s_78_2: cmp-eq s_78_0 s_78_1
        let s_78_2: bool = ((s_78_0) == (s_78_1));
        // D s_78_3: not s_78_2
        let s_78_3: bool = !s_78_2;
        // N s_78_4: branch s_78_3 b80 b79
        if s_78_3 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #35u : u8
        let s_79_0: u8 = 35;
        // D s_79_1: write-var branch_type <= s_79_0
        fn_state.branch_type = s_79_0;
        // N s_79_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #33u : u32
        let s_80_0: u32 = 33;
        // D s_80_1: read-var except:u32
        let s_80_1: u32 = fn_state.except;
        // D s_80_2: cmp-eq s_80_0 s_80_1
        let s_80_2: bool = ((s_80_0) == (s_80_1));
        // D s_80_3: not s_80_2
        let s_80_3: bool = !s_80_2;
        // N s_80_4: branch s_80_3 b82 b81
        if s_80_3 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_81(state, tracer, fn_state);
        };
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #35u : u8
        let s_81_0: u8 = 35;
        // D s_81_1: write-var branch_type <= s_81_0
        fn_state.branch_type = s_81_0;
        // N s_81_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #16u : u32
        let s_82_0: u32 = 16;
        // D s_82_1: read-var except:u32
        let s_82_1: u32 = fn_state.except;
        // D s_82_2: cmp-eq s_82_0 s_82_1
        let s_82_2: bool = ((s_82_0) == (s_82_1));
        // D s_82_3: not s_82_2
        let s_82_3: bool = !s_82_2;
        // N s_82_4: branch s_82_3 b84 b83
        if s_82_3 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_83(state, tracer, fn_state);
        };
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #35u : u8
        let s_83_0: u8 = 35;
        // D s_83_1: write-var branch_type <= s_83_0
        fn_state.branch_type = s_83_0;
        // N s_83_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #21u : u32
        let s_84_0: u32 = 21;
        // D s_84_1: read-var except:u32
        let s_84_1: u32 = fn_state.except;
        // D s_84_2: cmp-eq s_84_0 s_84_1
        let s_84_2: bool = ((s_84_0) == (s_84_1));
        // D s_84_3: not s_84_2
        let s_84_3: bool = !s_84_2;
        // N s_84_4: branch s_84_3 b86 b85
        if s_84_3 {
            return block_86(state, tracer, fn_state);
        } else {
            return block_85(state, tracer, fn_state);
        };
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #44u : u8
        let s_85_0: u8 = 44;
        // D s_85_1: write-var branch_type <= s_85_0
        fn_state.branch_type = s_85_0;
        // N s_85_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #17u : u32
        let s_86_0: u32 = 17;
        // D s_86_1: read-var except:u32
        let s_86_1: u32 = fn_state.except;
        // D s_86_2: cmp-eq s_86_0 s_86_1
        let s_86_2: bool = ((s_86_0) == (s_86_1));
        // D s_86_3: not s_86_2
        let s_86_3: bool = !s_86_2;
        // N s_86_4: branch s_86_3 b88 b87
        if s_86_3 {
            return block_88(state, tracer, fn_state);
        } else {
            return block_87(state, tracer, fn_state);
        };
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #43u : u8
        let s_87_0: u8 = 43;
        // D s_87_1: write-var branch_type <= s_87_0
        fn_state.branch_type = s_87_0;
        // N s_87_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #18u : u32
        let s_88_0: u32 = 18;
        // D s_88_1: read-var except:u32
        let s_88_1: u32 = fn_state.except;
        // D s_88_2: cmp-eq s_88_0 s_88_1
        let s_88_2: bool = ((s_88_0) == (s_88_1));
        // D s_88_3: not s_88_2
        let s_88_3: bool = !s_88_2;
        // N s_88_4: branch s_88_3 b90 b89
        if s_88_3 {
            return block_90(state, tracer, fn_state);
        } else {
            return block_89(state, tracer, fn_state);
        };
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #42u : u8
        let s_89_0: u8 = 42;
        // D s_89_1: write-var branch_type <= s_89_0
        fn_state.branch_type = s_89_0;
        // N s_89_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #19u : u32
        let s_90_0: u32 = 19;
        // D s_90_1: read-var except:u32
        let s_90_1: u32 = fn_state.except;
        // D s_90_2: cmp-eq s_90_0 s_90_1
        let s_90_2: bool = ((s_90_0) == (s_90_1));
        // D s_90_3: not s_90_2
        let s_90_3: bool = !s_90_2;
        // N s_90_4: branch s_90_3 b92 b91
        if s_90_3 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_91(state, tracer, fn_state);
        };
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #44u : u8
        let s_91_0: u8 = 44;
        // D s_91_1: write-var branch_type <= s_91_0
        fn_state.branch_type = s_91_0;
        // N s_91_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #20u : u32
        let s_92_0: u32 = 20;
        // D s_92_1: read-var except:u32
        let s_92_1: u32 = fn_state.except;
        // D s_92_2: cmp-eq s_92_0 s_92_1
        let s_92_2: bool = ((s_92_0) == (s_92_1));
        // D s_92_3: not s_92_2
        let s_92_3: bool = !s_92_2;
        // N s_92_4: branch s_92_3 b94 b93
        if s_92_3 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_93(state, tracer, fn_state);
        };
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #44u : u8
        let s_93_0: u8 = 44;
        // D s_93_1: write-var branch_type <= s_93_0
        fn_state.branch_type = s_93_0;
        // N s_93_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #22u : u32
        let s_94_0: u32 = 22;
        // D s_94_1: read-var except:u32
        let s_94_1: u32 = fn_state.except;
        // D s_94_2: cmp-eq s_94_0 s_94_1
        let s_94_2: bool = ((s_94_0) == (s_94_1));
        // D s_94_3: not s_94_2
        let s_94_3: bool = !s_94_2;
        // N s_94_4: branch s_94_3 b96 b95
        if s_94_3 {
            return block_96(state, tracer, fn_state);
        } else {
            return block_95(state, tracer, fn_state);
        };
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #42u : u8
        let s_95_0: u8 = 42;
        // D s_95_1: write-var branch_type <= s_95_0
        fn_state.branch_type = s_95_0;
        // N s_95_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_96_0: const #23u : u32
        let s_96_0: u32 = 23;
        // D s_96_1: read-var except:u32
        let s_96_1: u32 = fn_state.except;
        // D s_96_2: cmp-eq s_96_0 s_96_1
        let s_96_2: bool = ((s_96_0) == (s_96_1));
        // D s_96_3: not s_96_2
        let s_96_3: bool = !s_96_2;
        // N s_96_4: branch s_96_3 b98 b97
        if s_96_3 {
            return block_98(state, tracer, fn_state);
        } else {
            return block_97(state, tracer, fn_state);
        };
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #35u : u8
        let s_97_0: u8 = 35;
        // D s_97_1: write-var branch_type <= s_97_0
        fn_state.branch_type = s_97_0;
        // N s_97_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #24u : u32
        let s_98_0: u32 = 24;
        // D s_98_1: read-var except:u32
        let s_98_1: u32 = fn_state.except;
        // D s_98_2: cmp-eq s_98_0 s_98_1
        let s_98_2: bool = ((s_98_0) == (s_98_1));
        // D s_98_3: not s_98_2
        let s_98_3: bool = !s_98_2;
        // N s_98_4: branch s_98_3 b100 b99
        if s_98_3 {
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
        // C s_99_0: const #36u : u8
        let s_99_0: u8 = 36;
        // D s_99_1: write-var branch_type <= s_99_0
        fn_state.branch_type = s_99_0;
        // N s_99_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #25u : u32
        let s_100_0: u32 = 25;
        // D s_100_1: read-var except:u32
        let s_100_1: u32 = fn_state.except;
        // D s_100_2: cmp-eq s_100_0 s_100_1
        let s_100_2: bool = ((s_100_0) == (s_100_1));
        // D s_100_3: not s_100_2
        let s_100_3: bool = !s_100_2;
        // N s_100_4: branch s_100_3 b102 b101
        if s_100_3 {
            return block_102(state, tracer, fn_state);
        } else {
            return block_101(state, tracer, fn_state);
        };
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_101_0: const #38u : u8
        let s_101_0: u8 = 38;
        // D s_101_1: write-var branch_type <= s_101_0
        fn_state.branch_type = s_101_0;
        // N s_101_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_102_0: const #26u : u32
        let s_102_0: u32 = 26;
        // D s_102_1: read-var except:u32
        let s_102_1: u32 = fn_state.except;
        // D s_102_2: cmp-eq s_102_0 s_102_1
        let s_102_2: bool = ((s_102_0) == (s_102_1));
        // D s_102_3: not s_102_2
        let s_102_3: bool = !s_102_2;
        // N s_102_4: branch s_102_3 b104 b103
        if s_102_3 {
            return block_104(state, tracer, fn_state);
        } else {
            return block_103(state, tracer, fn_state);
        };
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_103_0: const #38u : u8
        let s_103_0: u8 = 38;
        // D s_103_1: write-var branch_type <= s_103_0
        fn_state.branch_type = s_103_0;
        // N s_103_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_104_0: const #27u : u32
        let s_104_0: u32 = 27;
        // D s_104_1: read-var except:u32
        let s_104_1: u32 = fn_state.except;
        // D s_104_2: cmp-eq s_104_0 s_104_1
        let s_104_2: bool = ((s_104_0) == (s_104_1));
        // D s_104_3: not s_104_2
        let s_104_3: bool = !s_104_2;
        // N s_104_4: branch s_104_3 b106 b105
        if s_104_3 {
            return block_106(state, tracer, fn_state);
        } else {
            return block_105(state, tracer, fn_state);
        };
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #39u : u8
        let s_105_0: u8 = 39;
        // D s_105_1: write-var branch_type <= s_105_0
        fn_state.branch_type = s_105_0;
        // N s_105_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #28u : u32
        let s_106_0: u32 = 28;
        // D s_106_1: read-var except:u32
        let s_106_1: u32 = fn_state.except;
        // D s_106_2: cmp-eq s_106_0 s_106_1
        let s_106_2: bool = ((s_106_0) == (s_106_1));
        // D s_106_3: not s_106_2
        let s_106_3: bool = !s_106_2;
        // N s_106_4: branch s_106_3 b108 b107
        if s_106_3 {
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
        // C s_107_0: const #39u : u8
        let s_107_0: u8 = 39;
        // D s_107_1: write-var branch_type <= s_107_0
        fn_state.branch_type = s_107_0;
        // N s_107_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_108_0: const #29u : u32
        let s_108_0: u32 = 29;
        // D s_108_1: read-var except:u32
        let s_108_1: u32 = fn_state.except;
        // D s_108_2: cmp-eq s_108_0 s_108_1
        let s_108_2: bool = ((s_108_0) == (s_108_1));
        // D s_108_3: not s_108_2
        let s_108_3: bool = !s_108_2;
        // N s_108_4: branch s_108_3 b110 b109
        if s_108_3 {
            return block_110(state, tracer, fn_state);
        } else {
            return block_109(state, tracer, fn_state);
        };
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #38u : u8
        let s_109_0: u8 = 38;
        // D s_109_1: write-var branch_type <= s_109_0
        fn_state.branch_type = s_109_0;
        // N s_109_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_110_0: const #31u : u32
        let s_110_0: u32 = 31;
        // D s_110_1: read-var except:u32
        let s_110_1: u32 = fn_state.except;
        // D s_110_2: cmp-eq s_110_0 s_110_1
        let s_110_2: bool = ((s_110_0) == (s_110_1));
        // D s_110_3: not s_110_2
        let s_110_3: bool = !s_110_2;
        // N s_110_4: branch s_110_3 b112 b111
        if s_110_3 {
            return block_112(state, tracer, fn_state);
        } else {
            return block_111(state, tracer, fn_state);
        };
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_111_0: const #46u : u8
        let s_111_0: u8 = 46;
        // D s_111_1: write-var branch_type <= s_111_0
        fn_state.branch_type = s_111_0;
        // N s_111_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_112_0: const #40u : u32
        let s_112_0: u32 = 40;
        // D s_112_1: read-var except:u32
        let s_112_1: u32 = fn_state.except;
        // D s_112_2: cmp-eq s_112_0 s_112_1
        let s_112_2: bool = ((s_112_0) == (s_112_1));
        // D s_112_3: not s_112_2
        let s_112_3: bool = !s_112_2;
        // N s_112_4: branch s_112_3 b114 b113
        if s_112_3 {
            return block_114(state, tracer, fn_state);
        } else {
            return block_113(state, tracer, fn_state);
        };
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_113_0: const #47u : u8
        let s_113_0: u8 = 47;
        // D s_113_1: write-var branch_type <= s_113_0
        fn_state.branch_type = s_113_0;
        // N s_113_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #37u : u32
        let s_114_0: u32 = 37;
        // D s_114_1: read-var except:u32
        let s_114_1: u32 = fn_state.except;
        // D s_114_2: cmp-eq s_114_0 s_114_1
        let s_114_2: bool = ((s_114_0) == (s_114_1));
        // D s_114_3: not s_114_2
        let s_114_3: bool = !s_114_2;
        // N s_114_4: branch s_114_3 b116 b115
        if s_114_3 {
            return block_116(state, tracer, fn_state);
        } else {
            return block_115(state, tracer, fn_state);
        };
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_115_0: const #35u : u8
        let s_115_0: u8 = 35;
        // D s_115_1: write-var branch_type <= s_115_0
        fn_state.branch_type = s_115_0;
        // N s_115_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_116_0: const #38u : u32
        let s_116_0: u32 = 38;
        // D s_116_1: read-var except:u32
        let s_116_1: u32 = fn_state.except;
        // D s_116_2: cmp-eq s_116_0 s_116_1
        let s_116_2: bool = ((s_116_0) == (s_116_1));
        // D s_116_3: not s_116_2
        let s_116_3: bool = !s_116_2;
        // N s_116_4: branch s_116_3 b124 b117
        if s_116_3 {
            return block_124(state, tracer, fn_state);
        } else {
            return block_117(state, tracer, fn_state);
        };
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_117_0: const #20s : i
        let s_117_0: i128 = 20;
        // D s_117_1: read-var iss:u25
        let s_117_1: u32 = fn_state.iss;
        // D s_117_2: cast zx s_117_1 -> bv
        let s_117_2: Bits = Bits::new(s_117_1 as u128, 25u16);
        // C s_117_3: const #1s : i64
        let s_117_3: i64 = 1;
        // C s_117_4: cast zx s_117_3 -> i
        let s_117_4: i128 = (i128::try_from(s_117_3).unwrap());
        // C s_117_5: const #3s : i
        let s_117_5: i128 = 3;
        // C s_117_6: add s_117_5 s_117_4
        let s_117_6: i128 = (s_117_5 + s_117_4);
        // D s_117_7: bit-extract s_117_2 s_117_0 s_117_6
        let s_117_7: Bits = (Bits::new(
            ((s_117_2) >> (s_117_0)).value(),
            u16::try_from(s_117_6).unwrap(),
        ));
        // D s_117_8: cast reint s_117_7 -> u8
        let s_117_8: u8 = (s_117_7.value() as u8);
        // D s_117_9: cast zx s_117_8 -> bv
        let s_117_9: Bits = Bits::new(s_117_8 as u128, 4u16);
        // C s_117_10: const #0u : u8
        let s_117_10: u8 = 0;
        // C s_117_11: cast zx s_117_10 -> bv
        let s_117_11: Bits = Bits::new(s_117_10 as u128, 4u16);
        // D s_117_12: cmp-eq s_117_9 s_117_11
        let s_117_12: bool = ((s_117_9) == (s_117_11));
        // N s_117_13: branch s_117_12 b123 b118
        if s_117_12 {
            return block_123(state, tracer, fn_state);
        } else {
            return block_118(state, tracer, fn_state);
        };
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #20s : i
        let s_118_0: i128 = 20;
        // D s_118_1: read-var iss:u25
        let s_118_1: u32 = fn_state.iss;
        // D s_118_2: cast zx s_118_1 -> bv
        let s_118_2: Bits = Bits::new(s_118_1 as u128, 25u16);
        // C s_118_3: const #1s : i64
        let s_118_3: i64 = 1;
        // C s_118_4: cast zx s_118_3 -> i
        let s_118_4: i128 = (i128::try_from(s_118_3).unwrap());
        // C s_118_5: const #3s : i
        let s_118_5: i128 = 3;
        // C s_118_6: add s_118_5 s_118_4
        let s_118_6: i128 = (s_118_5 + s_118_4);
        // D s_118_7: bit-extract s_118_2 s_118_0 s_118_6
        let s_118_7: Bits = (Bits::new(
            ((s_118_2) >> (s_118_0)).value(),
            u16::try_from(s_118_6).unwrap(),
        ));
        // D s_118_8: cast reint s_118_7 -> u8
        let s_118_8: u8 = (s_118_7.value() as u8);
        // D s_118_9: cast zx s_118_8 -> bv
        let s_118_9: Bits = Bits::new(s_118_8 as u128, 4u16);
        // C s_118_10: const #1u : u8
        let s_118_10: u8 = 1;
        // C s_118_11: cast zx s_118_10 -> bv
        let s_118_11: Bits = Bits::new(s_118_10 as u128, 4u16);
        // D s_118_12: cmp-eq s_118_9 s_118_11
        let s_118_12: bool = ((s_118_9) == (s_118_11));
        // N s_118_13: branch s_118_12 b122 b119
        if s_118_12 {
            return block_122(state, tracer, fn_state);
        } else {
            return block_119(state, tracer, fn_state);
        };
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_119_0: const #20s : i
        let s_119_0: i128 = 20;
        // D s_119_1: read-var iss:u25
        let s_119_1: u32 = fn_state.iss;
        // D s_119_2: cast zx s_119_1 -> bv
        let s_119_2: Bits = Bits::new(s_119_1 as u128, 25u16);
        // C s_119_3: const #1s : i64
        let s_119_3: i64 = 1;
        // C s_119_4: cast zx s_119_3 -> i
        let s_119_4: i128 = (i128::try_from(s_119_3).unwrap());
        // C s_119_5: const #3s : i
        let s_119_5: i128 = 3;
        // C s_119_6: add s_119_5 s_119_4
        let s_119_6: i128 = (s_119_5 + s_119_4);
        // D s_119_7: bit-extract s_119_2 s_119_0 s_119_6
        let s_119_7: Bits = (Bits::new(
            ((s_119_2) >> (s_119_0)).value(),
            u16::try_from(s_119_6).unwrap(),
        ));
        // D s_119_8: cast reint s_119_7 -> u8
        let s_119_8: u8 = (s_119_7.value() as u8);
        // D s_119_9: cast zx s_119_8 -> bv
        let s_119_9: Bits = Bits::new(s_119_8 as u128, 4u16);
        // C s_119_10: const #2u : u8
        let s_119_10: u8 = 2;
        // C s_119_11: cast zx s_119_10 -> bv
        let s_119_11: Bits = Bits::new(s_119_10 as u128, 4u16);
        // D s_119_12: cmp-eq s_119_9 s_119_11
        let s_119_12: bool = ((s_119_9) == (s_119_11));
        // N s_119_13: branch s_119_12 b121 b120
        if s_119_12 {
            return block_121(state, tracer, fn_state);
        } else {
            return block_120(state, tracer, fn_state);
        };
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_120_0: const #() : ()
        let s_120_0: () = ();
        // S s_120_1: call Unreachable(s_120_0)
        let s_120_1: () = Unreachable(state, tracer, s_120_0);
        // N s_120_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_121_0: const #35u : u8
        let s_121_0: u8 = 35;
        // D s_121_1: write-var branch_type <= s_121_0
        fn_state.branch_type = s_121_0;
        // N s_121_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_122_0: const #43u : u8
        let s_122_0: u8 = 43;
        // D s_122_1: write-var branch_type <= s_122_0
        fn_state.branch_type = s_122_0;
        // N s_122_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_123_0: const #44u : u8
        let s_123_0: u8 = 44;
        // D s_123_1: write-var branch_type <= s_123_0
        fn_state.branch_type = s_123_0;
        // N s_123_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_124_0: const #() : ()
        let s_124_0: () = ();
        // S s_124_1: call Unreachable(s_124_0)
        let s_124_1: () = Unreachable(state, tracer, s_124_0);
        // N s_124_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_125_0: const #1u : u8
        let s_125_0: bool = true;
        // D s_125_1: write-var gs#6175 <= s_125_0
        fn_state.gs_6175 = s_125_0;
        // N s_125_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_126_0: return
        return;
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_127_0: const #1u : u8
        let s_127_0: bool = true;
        // D s_127_1: write-var gs#6166 <= s_127_0
        fn_state.gs_6166 = s_127_0;
        // N s_127_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_128_0: read-var target_el:u8
        let s_128_0: u8 = fn_state.target_el;
        // D s_128_1: cast zx s_128_0 -> bv
        let s_128_1: Bits = Bits::new(s_128_0 as u128, 2u16);
        // C s_128_2: const #432u : u32
        let s_128_2: u32 = 432;
        // D s_128_3: read-reg s_128_2:u8
        let s_128_3: u8 = {
            let value = state.read_register::<u8>(s_128_2 as isize);
            tracer.read_register(s_128_2 as isize, value);
            value
        };
        // D s_128_4: cast zx s_128_3 -> bv
        let s_128_4: Bits = Bits::new(s_128_3 as u128, 2u16);
        // D s_128_5: cmp-eq s_128_1 s_128_4
        let s_128_5: bool = ((s_128_1) == (s_128_4));
        // D s_128_6: not s_128_5
        let s_128_6: bool = !s_128_5;
        // N s_128_7: branch s_128_6 b132 b129
        if s_128_6 {
            return block_132(state, tracer, fn_state);
        } else {
            return block_129(state, tracer, fn_state);
        };
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_129_0: const #18272u : u32
        let s_129_0: u32 = 18272;
        // D s_129_1: read-reg s_129_0:struct
        let s_129_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_129_0 as isize);
            tracer.read_register(s_129_0 as isize, value);
            value
        };
        // D s_129_2: call _get_BRBCR_EL2_Type_EXCEPTION(s_129_1)
        let s_129_2: bool = u_get_BRBCR_EL2_Type_EXCEPTION(state, tracer, s_129_1);
        // D s_129_3: cast zx s_129_2 -> bv
        let s_129_3: Bits = Bits::new(s_129_2 as u128, 1u16);
        // C s_129_4: const #0u : u8
        let s_129_4: bool = false;
        // C s_129_5: cast zx s_129_4 -> bv
        let s_129_5: Bits = Bits::new(s_129_4 as u128, 1u16);
        // D s_129_6: cmp-eq s_129_3 s_129_5
        let s_129_6: bool = ((s_129_3) == (s_129_5));
        // N s_129_7: branch s_129_6 b131 b130
        if s_129_6 {
            return block_131(state, tracer, fn_state);
        } else {
            return block_130(state, tracer, fn_state);
        };
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_130_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_131_0: return
        return;
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_132_0: read-var target_el:u8
        let s_132_0: u8 = fn_state.target_el;
        // D s_132_1: cast zx s_132_0 -> bv
        let s_132_1: Bits = Bits::new(s_132_0 as u128, 2u16);
        // C s_132_2: const #440u : u32
        let s_132_2: u32 = 440;
        // D s_132_3: read-reg s_132_2:u8
        let s_132_3: u8 = {
            let value = state.read_register::<u8>(s_132_2 as isize);
            tracer.read_register(s_132_2 as isize, value);
            value
        };
        // D s_132_4: cast zx s_132_3 -> bv
        let s_132_4: Bits = Bits::new(s_132_3 as u128, 2u16);
        // D s_132_5: cmp-eq s_132_1 s_132_4
        let s_132_5: bool = ((s_132_1) == (s_132_4));
        // D s_132_6: not s_132_5
        let s_132_6: bool = !s_132_5;
        // N s_132_7: branch s_132_6 b136 b133
        if s_132_6 {
            return block_136(state, tracer, fn_state);
        } else {
            return block_133(state, tracer, fn_state);
        };
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_133_0: const #90640u : u32
        let s_133_0: u32 = 90640;
        // D s_133_1: read-reg s_133_0:struct
        let s_133_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_133_0 as isize);
            tracer.read_register(s_133_0 as isize, value);
            value
        };
        // D s_133_2: call _get_BRBCR_EL1_Type_EXCEPTION(s_133_1)
        let s_133_2: bool = u_get_BRBCR_EL1_Type_EXCEPTION(state, tracer, s_133_1);
        // D s_133_3: cast zx s_133_2 -> bv
        let s_133_3: Bits = Bits::new(s_133_2 as u128, 1u16);
        // C s_133_4: const #0u : u8
        let s_133_4: bool = false;
        // C s_133_5: cast zx s_133_4 -> bv
        let s_133_5: Bits = Bits::new(s_133_4 as u128, 1u16);
        // D s_133_6: cmp-eq s_133_3 s_133_5
        let s_133_6: bool = ((s_133_3) == (s_133_5));
        // N s_133_7: branch s_133_6 b135 b134
        if s_133_6 {
            return block_135(state, tracer, fn_state);
        } else {
            return block_134(state, tracer, fn_state);
        };
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_134_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_135_0: return
        return;
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_136_0: jump b5
        return block_5(state, tracer, fn_state);
    }
}
