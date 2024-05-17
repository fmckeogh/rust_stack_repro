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
use u_get_MDCR_EL2_Type_TDA::*;
use MemA_read::*;
use Mk_DBGDTRTXint_Type::*;
use DBGDSCRext_read::*;
use u_get_HDCR_Type_TDCC::*;
use Halted::*;
use HDCR_read::*;
use u_get_MDCR_EL2_Type_TDCC::*;
use u_get_HDCR_Type_TDE::*;
use HCR_read::*;
use u_get_MDCR_EL3_Type_TDCC::*;
use u_get_MDCR_EL2_Type_TDE::*;
use AArch64_AArch32SystemAccessTrap::*;
use u_get_HDCR_Type_TDA::*;
use u_get_HCR_Type_TGE::*;
use u_get_HCR_EL2_Type_TGE::*;
use u_get_MDCR_EL3_Type_TDA::*;
use u_get_DBGDSCRext_Type_UDCCdis::*;
use ELUsingAArch32::*;
use u_get_SDCR_Type_TDCC::*;
use u_get_MDSCR_EL1_Type_TDCC::*;
use DBGDTRTXint_write::*;
use EL2Enabled::*;
use AArch32_TakeMonitorTrapException::*;
use AArch32_TakeHypTrapException::*;
use common::*;
pub fn DBGDTRTXint_LDC_da4baa792b999a1f<T: Tracer>(
    state: &mut State,
    tracer: &T,
    coproc: u8,
    CRd: u8,
    address: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        u__DBGDSCRext_UDCCdis: bool,
        gs_137017: bool,
        gs_137004: bool,
        gs_670314: Bits,
        gs_137013: bool,
        gs_137005: bool,
        gs_136994: bool,
        gs_137003: bool,
        gs_137012: bool,
        gs_137033: bool,
        gs_136991: bool,
        u__SDCR_TDCC: bool,
        gs_137009: bool,
        gs_137007: bool,
        gs_137008: bool,
        gs_670354: Bits,
        gs_137026: bool,
        gs_136998: bool,
        gs_137034: bool,
        gs_137035: bool,
        gs_136993: bool,
        gs_136997: bool,
        gs_137014: bool,
        gs_670404: Bits,
        gs_670444: Bits,
        u__PSTATE_EL: u8,
        gs_137000: bool,
        gs_137002: bool,
        gs_136996: bool,
        gs_137011: bool,
        gs_137027: bool,
        gs_137021: bool,
        gs_137031: bool,
        gs_137018: bool,
        gs_137016: bool,
        u__MDCR_EL3_TDA: bool,
        gs_137032: bool,
        gs_137022: bool,
        gs_136990: bool,
        gs_137025: bool,
        gs_136999: bool,
        gs_137024: bool,
        u__MDCR_EL3_TDCC: bool,
        gs_137020: bool,
        u__PSTATE_M: u8,
        gs_137001: bool,
        gs_137015: bool,
        gs_136989: bool,
        u__MDSCR_EL1_TDCC: bool,
        gs_137028: bool,
        gs_137030: bool,
        gs_670324: Bits,
        u__MDCR_EL2_TDCC: bool,
        gs_137023: bool,
        u__HDCR_TDCC: bool,
        gs_136992: bool,
        gs_136987: bool,
        gs_137006: bool,
        gs_137019: bool,
        coproc: u8,
        CRd: u8,
        address: u32,
    }
    let fn_state = FunctionState {
        coproc,
        CRd,
        address,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #16975u : u32
        let s_0_0: u32 = 16975;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: write-var __PSTATE_EL <= s_0_1
        fn_state.u__PSTATE_EL = s_0_1;
        // C s_0_3: const #104648u : u32
        let s_0_3: u32 = 104648;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_MDSCR_EL1_Type_TDCC(s_0_4)
        let s_0_5: bool = u_get_MDSCR_EL1_Type_TDCC(state, tracer, s_0_4);
        // D s_0_6: write-var __MDSCR_EL1_TDCC <= s_0_5
        fn_state.u__MDSCR_EL1_TDCC = s_0_5;
        // C s_0_7: const #() : ()
        let s_0_7: () = ();
        // S s_0_8: call DBGDSCRext_read(s_0_7)
        let s_0_8: ProductType700c18a878c5601b = DBGDSCRext_read(state, tracer, s_0_7);
        // S s_0_9: call _get_DBGDSCRext_Type_UDCCdis(s_0_8)
        let s_0_9: bool = u_get_DBGDSCRext_Type_UDCCdis(state, tracer, s_0_8);
        // D s_0_10: write-var __DBGDSCRext_UDCCdis <= s_0_9
        fn_state.u__DBGDSCRext_UDCCdis = s_0_9;
        // C s_0_11: const #104880u : u32
        let s_0_11: u32 = 104880;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_MDCR_EL2_Type_TDCC(s_0_12)
        let s_0_13: bool = u_get_MDCR_EL2_Type_TDCC(state, tracer, s_0_12);
        // D s_0_14: write-var __MDCR_EL2_TDCC <= s_0_13
        fn_state.u__MDCR_EL2_TDCC = s_0_13;
        // C s_0_15: const #() : ()
        let s_0_15: () = ();
        // S s_0_16: call HDCR_read(s_0_15)
        let s_0_16: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_0_15);
        // S s_0_17: call _get_HDCR_Type_TDCC(s_0_16)
        let s_0_17: bool = u_get_HDCR_Type_TDCC(state, tracer, s_0_16);
        // D s_0_18: write-var __HDCR_TDCC <= s_0_17
        fn_state.u__HDCR_TDCC = s_0_17;
        // C s_0_19: const #22712u : u32
        let s_0_19: u32 = 22712;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_MDCR_EL3_Type_TDCC(s_0_20)
        let s_0_21: bool = u_get_MDCR_EL3_Type_TDCC(state, tracer, s_0_20);
        // D s_0_22: write-var __MDCR_EL3_TDCC <= s_0_21
        fn_state.u__MDCR_EL3_TDCC = s_0_21;
        // C s_0_23: const #15048u : u32
        let s_0_23: u32 = 15048;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_SDCR_Type_TDCC(s_0_24)
        let s_0_25: bool = u_get_SDCR_Type_TDCC(state, tracer, s_0_24);
        // D s_0_26: write-var __SDCR_TDCC <= s_0_25
        fn_state.u__SDCR_TDCC = s_0_25;
        // C s_0_27: const #22712u : u32
        let s_0_27: u32 = 22712;
        // D s_0_28: read-reg s_0_27:struct
        let s_0_28: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_27 as isize);
            tracer.read_register(s_0_27 as isize, value);
            value
        };
        // D s_0_29: call _get_MDCR_EL3_Type_TDA(s_0_28)
        let s_0_29: bool = u_get_MDCR_EL3_Type_TDA(state, tracer, s_0_28);
        // D s_0_30: write-var __MDCR_EL3_TDA <= s_0_29
        fn_state.u__MDCR_EL3_TDA = s_0_29;
        // C s_0_31: const #16983u : u32
        let s_0_31: u32 = 16983;
        // D s_0_32: read-reg s_0_31:u8
        let s_0_32: u8 = {
            let value = state.read_register::<u8>(s_0_31 as isize);
            tracer.read_register(s_0_31 as isize, value);
            value
        };
        // D s_0_33: write-var __PSTATE_M <= s_0_32
        fn_state.u__PSTATE_M = s_0_32;
        // C s_0_34: const #() : ()
        let s_0_34: () = ();
        // S s_0_35: call Halted(s_0_34)
        let s_0_35: bool = Halted(state, tracer, s_0_34);
        // N s_0_36: branch s_0_35 b195 b1
        if s_0_35 {
            return block_195(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var __PSTATE_EL:u8
        let s_1_0: u8 = fn_state.u__PSTATE_EL;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 2u16);
        // C s_1_2: const #448u : u32
        let s_1_2: u32 = 448;
        // D s_1_3: read-reg s_1_2:u8
        let s_1_3: u8 = {
            let value = state.read_register::<u8>(s_1_2 as isize);
            tracer.read_register(s_1_2 as isize, value);
            value
        };
        // D s_1_4: cast zx s_1_3 -> bv
        let s_1_4: Bits = Bits::new(s_1_3 as u128, 2u16);
        // D s_1_5: cmp-eq s_1_1 s_1_4
        let s_1_5: bool = ((s_1_1) == (s_1_4));
        // N s_1_6: branch s_1_5 b97 b2
        if s_1_5 {
            return block_97(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var __PSTATE_EL:u8
        let s_2_0: u8 = fn_state.u__PSTATE_EL;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 2u16);
        // C s_2_2: const #440u : u32
        let s_2_2: u32 = 440;
        // D s_2_3: read-reg s_2_2:u8
        let s_2_3: u8 = {
            let value = state.read_register::<u8>(s_2_2 as isize);
            tracer.read_register(s_2_2 as isize, value);
            value
        };
        // D s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 2u16);
        // D s_2_5: cmp-eq s_2_1 s_2_4
        let s_2_5: bool = ((s_2_1) == (s_2_4));
        // N s_2_6: branch s_2_5 b39 b3
        if s_2_5 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var __PSTATE_EL:u8
        let s_3_0: u8 = fn_state.u__PSTATE_EL;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 2u16);
        // C s_3_2: const #432u : u32
        let s_3_2: u32 = 432;
        // D s_3_3: read-reg s_3_2:u8
        let s_3_3: u8 = {
            let value = state.read_register::<u8>(s_3_2 as isize);
            tracer.read_register(s_3_2 as isize, value);
            value
        };
        // D s_3_4: cast zx s_3_3 -> bv
        let s_3_4: Bits = Bits::new(s_3_3 as u128, 2u16);
        // D s_3_5: cmp-eq s_3_1 s_3_4
        let s_3_5: bool = ((s_3_1) == (s_3_4));
        // N s_3_6: branch s_3_5 b13 b4
        if s_3_5 {
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
        // D s_4_0: read-var __PSTATE_EL:u8
        let s_4_0: u8 = fn_state.u__PSTATE_EL;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 2u16);
        // C s_4_2: const #424u : u32
        let s_4_2: u32 = 424;
        // D s_4_3: read-reg s_4_2:u8
        let s_4_3: u8 = {
            let value = state.read_register::<u8>(s_4_2 as isize);
            tracer.read_register(s_4_2 as isize, value);
            value
        };
        // D s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 2u16);
        // D s_4_5: cmp-eq s_4_1 s_4_4
        let s_4_5: bool = ((s_4_1) == (s_4_4));
        // N s_4_6: branch s_4_5 b6 b5
        if s_4_5 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var __PSTATE_M:u8
        let s_6_0: u8 = fn_state.u__PSTATE_M;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 5u16);
        // C s_6_2: const #384u : u32
        let s_6_2: u32 = 384;
        // D s_6_3: read-reg s_6_2:u8
        let s_6_3: u8 = {
            let value = state.read_register::<u8>(s_6_2 as isize);
            tracer.read_register(s_6_2 as isize, value);
            value
        };
        // D s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 5u16);
        // D s_6_5: cmp-ne s_6_1 s_6_4
        let s_6_5: bool = ((s_6_1) != (s_6_4));
        // N s_6_6: branch s_6_5 b12 b7
        if s_6_5 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#136987 <= s_7_0
        fn_state.gs_136987 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#136987:u8
        let s_8_0: bool = fn_state.gs_136987;
        // N s_8_1: branch s_8_0 b11 b9
        if s_8_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #4s : i64
        let s_9_0: i64 = 4;
        // D s_9_1: read-var address:u32
        let s_9_1: u32 = fn_state.address;
        // D s_9_2: call MemA_read(s_9_1, s_9_0)
        let s_9_2: Bits = MemA_read(state, tracer, s_9_1, s_9_0);
        // D s_9_3: write-var gs#670314 <= s_9_2
        fn_state.gs_670314 = s_9_2;
        // N s_9_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#670314:bv
        let s_10_0: Bits = fn_state.gs_670314;
        // D s_10_1: cast reint s_10_0 -> u32
        let s_10_1: u32 = (s_10_0.value() as u32);
        // D s_10_2: call Mk_DBGDTRTXint_Type(s_10_1)
        let s_10_2: ProductType700c18a878c5601b = Mk_DBGDTRTXint_Type(
            state,
            tracer,
            s_10_1,
        );
        // D s_10_3: call DBGDTRTXint_write(s_10_2)
        let s_10_3: () = DBGDTRTXint_write(state, tracer, s_10_2);
        // N s_10_4: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call AArch32_TakeMonitorTrapException(s_11_0)
        let s_11_1: () = AArch32_TakeMonitorTrapException(state, tracer, s_11_0);
        // N s_11_2: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var __SDCR_TDCC:u8
        let s_12_0: bool = fn_state.u__SDCR_TDCC;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 1u16);
        // C s_12_2: const #1u : u8
        let s_12_2: bool = true;
        // C s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 1u16);
        // D s_12_4: cmp-eq s_12_1 s_12_3
        let s_12_4: bool = ((s_12_1) == (s_12_3));
        // D s_12_5: write-var gs#136987 <= s_12_4
        fn_state.gs_136987 = s_12_4;
        // N s_12_6: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #424u : u32
        let s_13_0: u32 = 424;
        // D s_13_1: read-reg s_13_0:u8
        let s_13_1: u8 = {
            let value = state.read_register::<u8>(s_13_0 as isize);
            tracer.read_register(s_13_0 as isize, value);
            value
        };
        // C s_13_2: const #2u : u8
        let s_13_2: u8 = 2;
        // D s_13_3: cmp-lt s_13_1 s_13_2
        let s_13_3: bool = ((s_13_1) < (s_13_2));
        // N s_13_4: branch s_13_3 b38 b14
        if s_13_3 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var gs#136989 <= s_14_0
        fn_state.gs_136989 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#136989:u8
        let s_15_0: bool = fn_state.gs_136989;
        // N s_15_1: branch s_15_0 b37 b16
        if s_15_0 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var gs#136990 <= s_16_0
        fn_state.gs_136990 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#136990:u8
        let s_17_0: bool = fn_state.gs_136990;
        // N s_17_1: branch s_17_0 b36 b18
        if s_17_0 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #424u : u32
        let s_18_0: u32 = 424;
        // D s_18_1: read-reg s_18_0:u8
        let s_18_1: u8 = {
            let value = state.read_register::<u8>(s_18_0 as isize);
            tracer.read_register(s_18_0 as isize, value);
            value
        };
        // C s_18_2: const #2u : u8
        let s_18_2: u8 = 2;
        // D s_18_3: cmp-lt s_18_1 s_18_2
        let s_18_3: bool = ((s_18_1) < (s_18_2));
        // N s_18_4: branch s_18_3 b35 b19
        if s_18_3 {
            return block_35(state, tracer, fn_state);
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
        // D s_19_1: write-var gs#136991 <= s_19_0
        fn_state.gs_136991 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#136991:u8
        let s_20_0: bool = fn_state.gs_136991;
        // N s_20_1: branch s_20_0 b34 b21
        if s_20_0 {
            return block_34(state, tracer, fn_state);
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
        let s_21_0: bool = false;
        // D s_21_1: write-var gs#136992 <= s_21_0
        fn_state.gs_136992 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#136992:u8
        let s_22_0: bool = fn_state.gs_136992;
        // N s_22_1: branch s_22_0 b33 b23
        if s_22_0 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #424u : u32
        let s_23_0: u32 = 424;
        // D s_23_1: read-reg s_23_0:u8
        let s_23_1: u8 = {
            let value = state.read_register::<u8>(s_23_0 as isize);
            tracer.read_register(s_23_0 as isize, value);
            value
        };
        // C s_23_2: const #2u : u8
        let s_23_2: u8 = 2;
        // D s_23_3: cmp-lt s_23_1 s_23_2
        let s_23_3: bool = ((s_23_1) < (s_23_2));
        // N s_23_4: branch s_23_3 b32 b24
        if s_23_3 {
            return block_32(state, tracer, fn_state);
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
        // D s_24_1: write-var gs#136993 <= s_24_0
        fn_state.gs_136993 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#136993:u8
        let s_25_0: bool = fn_state.gs_136993;
        // N s_25_1: branch s_25_0 b31 b26
        if s_25_0 {
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
        // D s_26_1: write-var gs#136994 <= s_26_0
        fn_state.gs_136994 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#136994:u8
        let s_27_0: bool = fn_state.gs_136994;
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
        // C s_28_0: const #4s : i64
        let s_28_0: i64 = 4;
        // D s_28_1: read-var address:u32
        let s_28_1: u32 = fn_state.address;
        // D s_28_2: call MemA_read(s_28_1, s_28_0)
        let s_28_2: Bits = MemA_read(state, tracer, s_28_1, s_28_0);
        // D s_28_3: write-var gs#670324 <= s_28_2
        fn_state.gs_670324 = s_28_2;
        // N s_28_4: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#670324:bv
        let s_29_0: Bits = fn_state.gs_670324;
        // D s_29_1: cast reint s_29_0 -> u32
        let s_29_1: u32 = (s_29_0.value() as u32);
        // D s_29_2: call Mk_DBGDTRTXint_Type(s_29_1)
        let s_29_2: ProductType700c18a878c5601b = Mk_DBGDTRTXint_Type(
            state,
            tracer,
            s_29_1,
        );
        // D s_29_3: call DBGDTRTXint_write(s_29_2)
        let s_29_3: () = DBGDTRTXint_write(state, tracer, s_29_2);
        // N s_29_4: return
        return;
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #6u : u8
        let s_30_0: u8 = 6;
        // C s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 8u16);
        // C s_30_2: cast zx s_30_1 -> i
        let s_30_2: i128 = (s_30_1.value() as i128);
        // C s_30_3: cast reint s_30_2 -> i64
        let s_30_3: i64 = (s_30_2 as i64);
        // C s_30_4: cast zx s_30_3 -> i
        let s_30_4: i128 = (i128::try_from(s_30_3).unwrap());
        // C s_30_5: const #424u : u32
        let s_30_5: u32 = 424;
        // D s_30_6: read-reg s_30_5:u8
        let s_30_6: u8 = {
            let value = state.read_register::<u8>(s_30_5 as isize);
            tracer.read_register(s_30_5 as isize, value);
            value
        };
        // D s_30_7: call AArch64_AArch32SystemAccessTrap(s_30_6, s_30_4)
        let s_30_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_30_6, s_30_4);
        // N s_30_8: return
        return;
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var __MDCR_EL3_TDA:u8
        let s_31_0: bool = fn_state.u__MDCR_EL3_TDA;
        // D s_31_1: cast zx s_31_0 -> bv
        let s_31_1: Bits = Bits::new(s_31_0 as u128, 1u16);
        // C s_31_2: const #1u : u8
        let s_31_2: bool = true;
        // C s_31_3: cast zx s_31_2 -> bv
        let s_31_3: Bits = Bits::new(s_31_2 as u128, 1u16);
        // D s_31_4: cmp-eq s_31_1 s_31_3
        let s_31_4: bool = ((s_31_1) == (s_31_3));
        // D s_31_5: write-var gs#136994 <= s_31_4
        fn_state.gs_136994 = s_31_4;
        // N s_31_6: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #424u : u32
        let s_32_0: u32 = 424;
        // D s_32_1: read-reg s_32_0:u8
        let s_32_1: u8 = {
            let value = state.read_register::<u8>(s_32_0 as isize);
            tracer.read_register(s_32_0 as isize, value);
            value
        };
        // D s_32_2: call ELUsingAArch32(s_32_1)
        let s_32_2: bool = ELUsingAArch32(state, tracer, s_32_1);
        // D s_32_3: not s_32_2
        let s_32_3: bool = !s_32_2;
        // D s_32_4: write-var gs#136993 <= s_32_3
        fn_state.gs_136993 = s_32_3;
        // N s_32_5: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #() : ()
        let s_33_0: () = ();
        // S s_33_1: call AArch32_TakeMonitorTrapException(s_33_0)
        let s_33_1: () = AArch32_TakeMonitorTrapException(state, tracer, s_33_0);
        // N s_33_2: return
        return;
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var __SDCR_TDCC:u8
        let s_34_0: bool = fn_state.u__SDCR_TDCC;
        // D s_34_1: cast zx s_34_0 -> bv
        let s_34_1: Bits = Bits::new(s_34_0 as u128, 1u16);
        // C s_34_2: const #1u : u8
        let s_34_2: bool = true;
        // C s_34_3: cast zx s_34_2 -> bv
        let s_34_3: Bits = Bits::new(s_34_2 as u128, 1u16);
        // D s_34_4: cmp-eq s_34_1 s_34_3
        let s_34_4: bool = ((s_34_1) == (s_34_3));
        // D s_34_5: write-var gs#136992 <= s_34_4
        fn_state.gs_136992 = s_34_4;
        // N s_34_6: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #424u : u32
        let s_35_0: u32 = 424;
        // D s_35_1: read-reg s_35_0:u8
        let s_35_1: u8 = {
            let value = state.read_register::<u8>(s_35_0 as isize);
            tracer.read_register(s_35_0 as isize, value);
            value
        };
        // D s_35_2: call ELUsingAArch32(s_35_1)
        let s_35_2: bool = ELUsingAArch32(state, tracer, s_35_1);
        // D s_35_3: write-var gs#136991 <= s_35_2
        fn_state.gs_136991 = s_35_2;
        // N s_35_4: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #6u : u8
        let s_36_0: u8 = 6;
        // C s_36_1: cast zx s_36_0 -> bv
        let s_36_1: Bits = Bits::new(s_36_0 as u128, 8u16);
        // C s_36_2: cast zx s_36_1 -> i
        let s_36_2: i128 = (s_36_1.value() as i128);
        // C s_36_3: cast reint s_36_2 -> i64
        let s_36_3: i64 = (s_36_2 as i64);
        // C s_36_4: cast zx s_36_3 -> i
        let s_36_4: i128 = (i128::try_from(s_36_3).unwrap());
        // C s_36_5: const #424u : u32
        let s_36_5: u32 = 424;
        // D s_36_6: read-reg s_36_5:u8
        let s_36_6: u8 = {
            let value = state.read_register::<u8>(s_36_5 as isize);
            tracer.read_register(s_36_5 as isize, value);
            value
        };
        // D s_36_7: call AArch64_AArch32SystemAccessTrap(s_36_6, s_36_4)
        let s_36_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_36_6, s_36_4);
        // N s_36_8: return
        return;
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var __MDCR_EL3_TDCC:u8
        let s_37_0: bool = fn_state.u__MDCR_EL3_TDCC;
        // D s_37_1: cast zx s_37_0 -> bv
        let s_37_1: Bits = Bits::new(s_37_0 as u128, 1u16);
        // C s_37_2: const #1u : u8
        let s_37_2: bool = true;
        // C s_37_3: cast zx s_37_2 -> bv
        let s_37_3: Bits = Bits::new(s_37_2 as u128, 1u16);
        // D s_37_4: cmp-eq s_37_1 s_37_3
        let s_37_4: bool = ((s_37_1) == (s_37_3));
        // D s_37_5: write-var gs#136990 <= s_37_4
        fn_state.gs_136990 = s_37_4;
        // N s_37_6: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #424u : u32
        let s_38_0: u32 = 424;
        // D s_38_1: read-reg s_38_0:u8
        let s_38_1: u8 = {
            let value = state.read_register::<u8>(s_38_0 as isize);
            tracer.read_register(s_38_0 as isize, value);
            value
        };
        // D s_38_2: call ELUsingAArch32(s_38_1)
        let s_38_2: bool = ELUsingAArch32(state, tracer, s_38_1);
        // D s_38_3: not s_38_2
        let s_38_3: bool = !s_38_2;
        // D s_38_4: write-var gs#136989 <= s_38_3
        fn_state.gs_136989 = s_38_3;
        // N s_38_5: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #() : ()
        let s_39_0: () = ();
        // S s_39_1: call EL2Enabled(s_39_0)
        let s_39_1: bool = EL2Enabled(state, tracer, s_39_0);
        // N s_39_2: branch s_39_1 b96 b40
        if s_39_1 {
            return block_96(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #0u : u8
        let s_40_0: bool = false;
        // D s_40_1: write-var gs#136996 <= s_40_0
        fn_state.gs_136996 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#136996:u8
        let s_41_0: bool = fn_state.gs_136996;
        // N s_41_1: branch s_41_0 b95 b42
        if s_41_0 {
            return block_95(state, tracer, fn_state);
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
        // D s_42_1: write-var gs#136997 <= s_42_0
        fn_state.gs_136997 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#136997:u8
        let s_43_0: bool = fn_state.gs_136997;
        // N s_43_1: branch s_43_0 b94 b44
        if s_43_0 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #() : ()
        let s_44_0: () = ();
        // S s_44_1: call EL2Enabled(s_44_0)
        let s_44_1: bool = EL2Enabled(state, tracer, s_44_0);
        // N s_44_2: branch s_44_1 b93 b45
        if s_44_1 {
            return block_93(state, tracer, fn_state);
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
        // D s_45_1: write-var gs#136998 <= s_45_0
        fn_state.gs_136998 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#136998:u8
        let s_46_0: bool = fn_state.gs_136998;
        // N s_46_1: branch s_46_0 b92 b47
        if s_46_0 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #0u : u8
        let s_47_0: bool = false;
        // D s_47_1: write-var gs#136999 <= s_47_0
        fn_state.gs_136999 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#136999:u8
        let s_48_0: bool = fn_state.gs_136999;
        // N s_48_1: branch s_48_0 b91 b49
        if s_48_0 {
            return block_91(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #() : ()
        let s_49_0: () = ();
        // S s_49_1: call EL2Enabled(s_49_0)
        let s_49_1: bool = EL2Enabled(state, tracer, s_49_0);
        // N s_49_2: branch s_49_1 b90 b50
        if s_49_1 {
            return block_90(state, tracer, fn_state);
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
        // D s_50_1: write-var gs#137000 <= s_50_0
        fn_state.gs_137000 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#137000:u8
        let s_51_0: bool = fn_state.gs_137000;
        // N s_51_1: branch s_51_0 b89 b52
        if s_51_0 {
            return block_89(state, tracer, fn_state);
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
        // D s_52_1: write-var gs#137001 <= s_52_0
        fn_state.gs_137001 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#137001:u8
        let s_53_0: bool = fn_state.gs_137001;
        // N s_53_1: branch s_53_0 b88 b54
        if s_53_0 {
            return block_88(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #() : ()
        let s_54_0: () = ();
        // S s_54_1: call EL2Enabled(s_54_0)
        let s_54_1: bool = EL2Enabled(state, tracer, s_54_0);
        // N s_54_2: branch s_54_1 b87 b55
        if s_54_1 {
            return block_87(state, tracer, fn_state);
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
        // D s_55_1: write-var gs#137002 <= s_55_0
        fn_state.gs_137002 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var gs#137002:u8
        let s_56_0: bool = fn_state.gs_137002;
        // N s_56_1: branch s_56_0 b86 b57
        if s_56_0 {
            return block_86(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #0u : u8
        let s_57_0: bool = false;
        // D s_57_1: write-var gs#137003 <= s_57_0
        fn_state.gs_137003 = s_57_0;
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var gs#137003:u8
        let s_58_0: bool = fn_state.gs_137003;
        // N s_58_1: branch s_58_0 b85 b59
        if s_58_0 {
            return block_85(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #424u : u32
        let s_59_0: u32 = 424;
        // D s_59_1: read-reg s_59_0:u8
        let s_59_1: u8 = {
            let value = state.read_register::<u8>(s_59_0 as isize);
            tracer.read_register(s_59_0 as isize, value);
            value
        };
        // C s_59_2: const #2u : u8
        let s_59_2: u8 = 2;
        // D s_59_3: cmp-lt s_59_1 s_59_2
        let s_59_3: bool = ((s_59_1) < (s_59_2));
        // N s_59_4: branch s_59_3 b84 b60
        if s_59_3 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_60(state, tracer, fn_state);
        };
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #0u : u8
        let s_60_0: bool = false;
        // D s_60_1: write-var gs#137004 <= s_60_0
        fn_state.gs_137004 = s_60_0;
        // N s_60_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var gs#137004:u8
        let s_61_0: bool = fn_state.gs_137004;
        // N s_61_1: branch s_61_0 b83 b62
        if s_61_0 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #0u : u8
        let s_62_0: bool = false;
        // D s_62_1: write-var gs#137005 <= s_62_0
        fn_state.gs_137005 = s_62_0;
        // N s_62_2: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var gs#137005:u8
        let s_63_0: bool = fn_state.gs_137005;
        // N s_63_1: branch s_63_0 b82 b64
        if s_63_0 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_64(state, tracer, fn_state);
        };
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #424u : u32
        let s_64_0: u32 = 424;
        // D s_64_1: read-reg s_64_0:u8
        let s_64_1: u8 = {
            let value = state.read_register::<u8>(s_64_0 as isize);
            tracer.read_register(s_64_0 as isize, value);
            value
        };
        // C s_64_2: const #2u : u8
        let s_64_2: u8 = 2;
        // D s_64_3: cmp-lt s_64_1 s_64_2
        let s_64_3: bool = ((s_64_1) < (s_64_2));
        // N s_64_4: branch s_64_3 b81 b65
        if s_64_3 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #0u : u8
        let s_65_0: bool = false;
        // D s_65_1: write-var gs#137006 <= s_65_0
        fn_state.gs_137006 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var gs#137006:u8
        let s_66_0: bool = fn_state.gs_137006;
        // N s_66_1: branch s_66_0 b80 b67
        if s_66_0 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #0u : u8
        let s_67_0: bool = false;
        // D s_67_1: write-var gs#137007 <= s_67_0
        fn_state.gs_137007 = s_67_0;
        // N s_67_2: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var gs#137007:u8
        let s_68_0: bool = fn_state.gs_137007;
        // N s_68_1: branch s_68_0 b79 b69
        if s_68_0 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #424u : u32
        let s_69_0: u32 = 424;
        // D s_69_1: read-reg s_69_0:u8
        let s_69_1: u8 = {
            let value = state.read_register::<u8>(s_69_0 as isize);
            tracer.read_register(s_69_0 as isize, value);
            value
        };
        // C s_69_2: const #2u : u8
        let s_69_2: u8 = 2;
        // D s_69_3: cmp-lt s_69_1 s_69_2
        let s_69_3: bool = ((s_69_1) < (s_69_2));
        // N s_69_4: branch s_69_3 b78 b70
        if s_69_3 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_70(state, tracer, fn_state);
        };
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #0u : u8
        let s_70_0: bool = false;
        // D s_70_1: write-var gs#137008 <= s_70_0
        fn_state.gs_137008 = s_70_0;
        // N s_70_2: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var gs#137008:u8
        let s_71_0: bool = fn_state.gs_137008;
        // N s_71_1: branch s_71_0 b77 b72
        if s_71_0 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_72(state, tracer, fn_state);
        };
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #0u : u8
        let s_72_0: bool = false;
        // D s_72_1: write-var gs#137009 <= s_72_0
        fn_state.gs_137009 = s_72_0;
        // N s_72_2: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var gs#137009:u8
        let s_73_0: bool = fn_state.gs_137009;
        // N s_73_1: branch s_73_0 b76 b74
        if s_73_0 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_74(state, tracer, fn_state);
        };
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #4s : i64
        let s_74_0: i64 = 4;
        // D s_74_1: read-var address:u32
        let s_74_1: u32 = fn_state.address;
        // D s_74_2: call MemA_read(s_74_1, s_74_0)
        let s_74_2: Bits = MemA_read(state, tracer, s_74_1, s_74_0);
        // D s_74_3: write-var gs#670354 <= s_74_2
        fn_state.gs_670354 = s_74_2;
        // N s_74_4: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var gs#670354:bv
        let s_75_0: Bits = fn_state.gs_670354;
        // D s_75_1: cast reint s_75_0 -> u32
        let s_75_1: u32 = (s_75_0.value() as u32);
        // D s_75_2: call Mk_DBGDTRTXint_Type(s_75_1)
        let s_75_2: ProductType700c18a878c5601b = Mk_DBGDTRTXint_Type(
            state,
            tracer,
            s_75_1,
        );
        // D s_75_3: call DBGDTRTXint_write(s_75_2)
        let s_75_3: () = DBGDTRTXint_write(state, tracer, s_75_2);
        // N s_75_4: return
        return;
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #6u : u8
        let s_76_0: u8 = 6;
        // C s_76_1: cast zx s_76_0 -> bv
        let s_76_1: Bits = Bits::new(s_76_0 as u128, 8u16);
        // C s_76_2: cast zx s_76_1 -> i
        let s_76_2: i128 = (s_76_1.value() as i128);
        // C s_76_3: cast reint s_76_2 -> i64
        let s_76_3: i64 = (s_76_2 as i64);
        // C s_76_4: cast zx s_76_3 -> i
        let s_76_4: i128 = (i128::try_from(s_76_3).unwrap());
        // C s_76_5: const #424u : u32
        let s_76_5: u32 = 424;
        // D s_76_6: read-reg s_76_5:u8
        let s_76_6: u8 = {
            let value = state.read_register::<u8>(s_76_5 as isize);
            tracer.read_register(s_76_5 as isize, value);
            value
        };
        // D s_76_7: call AArch64_AArch32SystemAccessTrap(s_76_6, s_76_4)
        let s_76_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_76_6, s_76_4);
        // N s_76_8: return
        return;
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var __MDCR_EL3_TDA:u8
        let s_77_0: bool = fn_state.u__MDCR_EL3_TDA;
        // D s_77_1: cast zx s_77_0 -> bv
        let s_77_1: Bits = Bits::new(s_77_0 as u128, 1u16);
        // C s_77_2: const #1u : u8
        let s_77_2: bool = true;
        // C s_77_3: cast zx s_77_2 -> bv
        let s_77_3: Bits = Bits::new(s_77_2 as u128, 1u16);
        // D s_77_4: cmp-eq s_77_1 s_77_3
        let s_77_4: bool = ((s_77_1) == (s_77_3));
        // D s_77_5: write-var gs#137009 <= s_77_4
        fn_state.gs_137009 = s_77_4;
        // N s_77_6: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #424u : u32
        let s_78_0: u32 = 424;
        // D s_78_1: read-reg s_78_0:u8
        let s_78_1: u8 = {
            let value = state.read_register::<u8>(s_78_0 as isize);
            tracer.read_register(s_78_0 as isize, value);
            value
        };
        // D s_78_2: call ELUsingAArch32(s_78_1)
        let s_78_2: bool = ELUsingAArch32(state, tracer, s_78_1);
        // D s_78_3: not s_78_2
        let s_78_3: bool = !s_78_2;
        // D s_78_4: write-var gs#137008 <= s_78_3
        fn_state.gs_137008 = s_78_3;
        // N s_78_5: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #() : ()
        let s_79_0: () = ();
        // S s_79_1: call AArch32_TakeMonitorTrapException(s_79_0)
        let s_79_1: () = AArch32_TakeMonitorTrapException(state, tracer, s_79_0);
        // N s_79_2: return
        return;
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_80_0: read-var __SDCR_TDCC:u8
        let s_80_0: bool = fn_state.u__SDCR_TDCC;
        // D s_80_1: cast zx s_80_0 -> bv
        let s_80_1: Bits = Bits::new(s_80_0 as u128, 1u16);
        // C s_80_2: const #1u : u8
        let s_80_2: bool = true;
        // C s_80_3: cast zx s_80_2 -> bv
        let s_80_3: Bits = Bits::new(s_80_2 as u128, 1u16);
        // D s_80_4: cmp-eq s_80_1 s_80_3
        let s_80_4: bool = ((s_80_1) == (s_80_3));
        // D s_80_5: write-var gs#137007 <= s_80_4
        fn_state.gs_137007 = s_80_4;
        // N s_80_6: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #424u : u32
        let s_81_0: u32 = 424;
        // D s_81_1: read-reg s_81_0:u8
        let s_81_1: u8 = {
            let value = state.read_register::<u8>(s_81_0 as isize);
            tracer.read_register(s_81_0 as isize, value);
            value
        };
        // D s_81_2: call ELUsingAArch32(s_81_1)
        let s_81_2: bool = ELUsingAArch32(state, tracer, s_81_1);
        // D s_81_3: write-var gs#137006 <= s_81_2
        fn_state.gs_137006 = s_81_2;
        // N s_81_4: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #6u : u8
        let s_82_0: u8 = 6;
        // C s_82_1: cast zx s_82_0 -> bv
        let s_82_1: Bits = Bits::new(s_82_0 as u128, 8u16);
        // C s_82_2: cast zx s_82_1 -> i
        let s_82_2: i128 = (s_82_1.value() as i128);
        // C s_82_3: cast reint s_82_2 -> i64
        let s_82_3: i64 = (s_82_2 as i64);
        // C s_82_4: cast zx s_82_3 -> i
        let s_82_4: i128 = (i128::try_from(s_82_3).unwrap());
        // C s_82_5: const #424u : u32
        let s_82_5: u32 = 424;
        // D s_82_6: read-reg s_82_5:u8
        let s_82_6: u8 = {
            let value = state.read_register::<u8>(s_82_5 as isize);
            tracer.read_register(s_82_5 as isize, value);
            value
        };
        // D s_82_7: call AArch64_AArch32SystemAccessTrap(s_82_6, s_82_4)
        let s_82_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_82_6, s_82_4);
        // N s_82_8: return
        return;
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_83_0: read-var __MDCR_EL3_TDCC:u8
        let s_83_0: bool = fn_state.u__MDCR_EL3_TDCC;
        // D s_83_1: cast zx s_83_0 -> bv
        let s_83_1: Bits = Bits::new(s_83_0 as u128, 1u16);
        // C s_83_2: const #1u : u8
        let s_83_2: bool = true;
        // C s_83_3: cast zx s_83_2 -> bv
        let s_83_3: Bits = Bits::new(s_83_2 as u128, 1u16);
        // D s_83_4: cmp-eq s_83_1 s_83_3
        let s_83_4: bool = ((s_83_1) == (s_83_3));
        // D s_83_5: write-var gs#137005 <= s_83_4
        fn_state.gs_137005 = s_83_4;
        // N s_83_6: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #424u : u32
        let s_84_0: u32 = 424;
        // D s_84_1: read-reg s_84_0:u8
        let s_84_1: u8 = {
            let value = state.read_register::<u8>(s_84_0 as isize);
            tracer.read_register(s_84_0 as isize, value);
            value
        };
        // D s_84_2: call ELUsingAArch32(s_84_1)
        let s_84_2: bool = ELUsingAArch32(state, tracer, s_84_1);
        // D s_84_3: not s_84_2
        let s_84_3: bool = !s_84_2;
        // D s_84_4: write-var gs#137004 <= s_84_3
        fn_state.gs_137004 = s_84_3;
        // N s_84_5: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #6u : u8
        let s_85_0: u8 = 6;
        // C s_85_1: cast zx s_85_0 -> bv
        let s_85_1: Bits = Bits::new(s_85_0 as u128, 8u16);
        // C s_85_2: cast zx s_85_1 -> i
        let s_85_2: i128 = (s_85_1.value() as i128);
        // C s_85_3: cast reint s_85_2 -> i64
        let s_85_3: i64 = (s_85_2 as i64);
        // C s_85_4: cast zx s_85_3 -> i
        let s_85_4: i128 = (i128::try_from(s_85_3).unwrap());
        // S s_85_5: call AArch32_TakeHypTrapException(s_85_4)
        let s_85_5: () = AArch32_TakeHypTrapException(state, tracer, s_85_4);
        // N s_85_6: return
        return;
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #() : ()
        let s_86_0: () = ();
        // S s_86_1: call HDCR_read(s_86_0)
        let s_86_1: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_86_0);
        // S s_86_2: call _get_HDCR_Type_TDE(s_86_1)
        let s_86_2: bool = u_get_HDCR_Type_TDE(state, tracer, s_86_1);
        // C s_86_3: const #() : ()
        let s_86_3: () = ();
        // S s_86_4: call HDCR_read(s_86_3)
        let s_86_4: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_86_3);
        // S s_86_5: call _get_HDCR_Type_TDA(s_86_4)
        let s_86_5: bool = u_get_HDCR_Type_TDA(state, tracer, s_86_4);
        // S s_86_6: cast zx s_86_2 -> bv
        let s_86_6: Bits = Bits::new(s_86_2 as u128, 1u16);
        // S s_86_7: cast zx s_86_5 -> bv
        let s_86_7: Bits = Bits::new(s_86_5 as u128, 1u16);
        // S s_86_8: cast reint s_86_6 -> u128
        let s_86_8: u128 = (s_86_6.value() as u128);
        // D s_86_9: size-of s_86_6
        let s_86_9: u16 = s_86_6.length();
        // S s_86_10: cast reint s_86_7 -> u128
        let s_86_10: u128 = (s_86_7.value() as u128);
        // D s_86_11: size-of s_86_7
        let s_86_11: u16 = s_86_7.length();
        // D s_86_12: lsl s_86_8 s_86_11
        let s_86_12: u128 = s_86_8 << s_86_11;
        // D s_86_13: or s_86_12 s_86_10
        let s_86_13: u128 = ((s_86_12) | (s_86_10));
        // D s_86_14: add s_86_9 s_86_11
        let s_86_14: u16 = (s_86_9 + s_86_11);
        // D s_86_15: create-bits s_86_13 s_86_14
        let s_86_15: Bits = Bits::new(s_86_13, s_86_14);
        // D s_86_16: cast reint s_86_15 -> u8
        let s_86_16: u8 = (s_86_15.value() as u8);
        // D s_86_17: cast zx s_86_16 -> bv
        let s_86_17: Bits = Bits::new(s_86_16 as u128, 2u16);
        // C s_86_18: const #0u : u8
        let s_86_18: u8 = 0;
        // C s_86_19: cast zx s_86_18 -> bv
        let s_86_19: Bits = Bits::new(s_86_18 as u128, 2u16);
        // D s_86_20: cmp-ne s_86_17 s_86_19
        let s_86_20: bool = ((s_86_17) != (s_86_19));
        // D s_86_21: write-var gs#137003 <= s_86_20
        fn_state.gs_137003 = s_86_20;
        // N s_86_22: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #432u : u32
        let s_87_0: u32 = 432;
        // D s_87_1: read-reg s_87_0:u8
        let s_87_1: u8 = {
            let value = state.read_register::<u8>(s_87_0 as isize);
            tracer.read_register(s_87_0 as isize, value);
            value
        };
        // D s_87_2: call ELUsingAArch32(s_87_1)
        let s_87_2: bool = ELUsingAArch32(state, tracer, s_87_1);
        // D s_87_3: write-var gs#137002 <= s_87_2
        fn_state.gs_137002 = s_87_2;
        // N s_87_4: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #6u : u8
        let s_88_0: u8 = 6;
        // C s_88_1: cast zx s_88_0 -> bv
        let s_88_1: Bits = Bits::new(s_88_0 as u128, 8u16);
        // C s_88_2: cast zx s_88_1 -> i
        let s_88_2: i128 = (s_88_1.value() as i128);
        // C s_88_3: cast reint s_88_2 -> i64
        let s_88_3: i64 = (s_88_2 as i64);
        // C s_88_4: cast zx s_88_3 -> i
        let s_88_4: i128 = (i128::try_from(s_88_3).unwrap());
        // C s_88_5: const #432u : u32
        let s_88_5: u32 = 432;
        // D s_88_6: read-reg s_88_5:u8
        let s_88_6: u8 = {
            let value = state.read_register::<u8>(s_88_5 as isize);
            tracer.read_register(s_88_5 as isize, value);
            value
        };
        // D s_88_7: call AArch64_AArch32SystemAccessTrap(s_88_6, s_88_4)
        let s_88_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_88_6, s_88_4);
        // N s_88_8: return
        return;
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #104880u : u32
        let s_89_0: u32 = 104880;
        // D s_89_1: read-reg s_89_0:struct
        let s_89_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_89_0 as isize);
            tracer.read_register(s_89_0 as isize, value);
            value
        };
        // D s_89_2: call _get_MDCR_EL2_Type_TDE(s_89_1)
        let s_89_2: bool = u_get_MDCR_EL2_Type_TDE(state, tracer, s_89_1);
        // C s_89_3: const #104880u : u32
        let s_89_3: u32 = 104880;
        // D s_89_4: read-reg s_89_3:struct
        let s_89_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_89_3 as isize);
            tracer.read_register(s_89_3 as isize, value);
            value
        };
        // D s_89_5: call _get_MDCR_EL2_Type_TDA(s_89_4)
        let s_89_5: bool = u_get_MDCR_EL2_Type_TDA(state, tracer, s_89_4);
        // D s_89_6: cast zx s_89_2 -> bv
        let s_89_6: Bits = Bits::new(s_89_2 as u128, 1u16);
        // D s_89_7: cast zx s_89_5 -> bv
        let s_89_7: Bits = Bits::new(s_89_5 as u128, 1u16);
        // D s_89_8: cast reint s_89_6 -> u128
        let s_89_8: u128 = (s_89_6.value() as u128);
        // D s_89_9: size-of s_89_6
        let s_89_9: u16 = s_89_6.length();
        // D s_89_10: cast reint s_89_7 -> u128
        let s_89_10: u128 = (s_89_7.value() as u128);
        // D s_89_11: size-of s_89_7
        let s_89_11: u16 = s_89_7.length();
        // D s_89_12: lsl s_89_8 s_89_11
        let s_89_12: u128 = s_89_8 << s_89_11;
        // D s_89_13: or s_89_12 s_89_10
        let s_89_13: u128 = ((s_89_12) | (s_89_10));
        // D s_89_14: add s_89_9 s_89_11
        let s_89_14: u16 = (s_89_9 + s_89_11);
        // D s_89_15: create-bits s_89_13 s_89_14
        let s_89_15: Bits = Bits::new(s_89_13, s_89_14);
        // D s_89_16: cast reint s_89_15 -> u8
        let s_89_16: u8 = (s_89_15.value() as u8);
        // D s_89_17: cast zx s_89_16 -> bv
        let s_89_17: Bits = Bits::new(s_89_16 as u128, 2u16);
        // C s_89_18: const #0u : u8
        let s_89_18: u8 = 0;
        // C s_89_19: cast zx s_89_18 -> bv
        let s_89_19: Bits = Bits::new(s_89_18 as u128, 2u16);
        // D s_89_20: cmp-ne s_89_17 s_89_19
        let s_89_20: bool = ((s_89_17) != (s_89_19));
        // D s_89_21: write-var gs#137001 <= s_89_20
        fn_state.gs_137001 = s_89_20;
        // N s_89_22: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #432u : u32
        let s_90_0: u32 = 432;
        // D s_90_1: read-reg s_90_0:u8
        let s_90_1: u8 = {
            let value = state.read_register::<u8>(s_90_0 as isize);
            tracer.read_register(s_90_0 as isize, value);
            value
        };
        // D s_90_2: call ELUsingAArch32(s_90_1)
        let s_90_2: bool = ELUsingAArch32(state, tracer, s_90_1);
        // D s_90_3: not s_90_2
        let s_90_3: bool = !s_90_2;
        // D s_90_4: write-var gs#137000 <= s_90_3
        fn_state.gs_137000 = s_90_3;
        // N s_90_5: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #6u : u8
        let s_91_0: u8 = 6;
        // C s_91_1: cast zx s_91_0 -> bv
        let s_91_1: Bits = Bits::new(s_91_0 as u128, 8u16);
        // C s_91_2: cast zx s_91_1 -> i
        let s_91_2: i128 = (s_91_1.value() as i128);
        // C s_91_3: cast reint s_91_2 -> i64
        let s_91_3: i64 = (s_91_2 as i64);
        // C s_91_4: cast zx s_91_3 -> i
        let s_91_4: i128 = (i128::try_from(s_91_3).unwrap());
        // S s_91_5: call AArch32_TakeHypTrapException(s_91_4)
        let s_91_5: () = AArch32_TakeHypTrapException(state, tracer, s_91_4);
        // N s_91_6: return
        return;
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var __HDCR_TDCC:u8
        let s_92_0: bool = fn_state.u__HDCR_TDCC;
        // D s_92_1: cast zx s_92_0 -> bv
        let s_92_1: Bits = Bits::new(s_92_0 as u128, 1u16);
        // C s_92_2: const #1u : u8
        let s_92_2: bool = true;
        // C s_92_3: cast zx s_92_2 -> bv
        let s_92_3: Bits = Bits::new(s_92_2 as u128, 1u16);
        // D s_92_4: cmp-eq s_92_1 s_92_3
        let s_92_4: bool = ((s_92_1) == (s_92_3));
        // D s_92_5: write-var gs#136999 <= s_92_4
        fn_state.gs_136999 = s_92_4;
        // N s_92_6: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #432u : u32
        let s_93_0: u32 = 432;
        // D s_93_1: read-reg s_93_0:u8
        let s_93_1: u8 = {
            let value = state.read_register::<u8>(s_93_0 as isize);
            tracer.read_register(s_93_0 as isize, value);
            value
        };
        // D s_93_2: call ELUsingAArch32(s_93_1)
        let s_93_2: bool = ELUsingAArch32(state, tracer, s_93_1);
        // D s_93_3: write-var gs#136998 <= s_93_2
        fn_state.gs_136998 = s_93_2;
        // N s_93_4: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #6u : u8
        let s_94_0: u8 = 6;
        // C s_94_1: cast zx s_94_0 -> bv
        let s_94_1: Bits = Bits::new(s_94_0 as u128, 8u16);
        // C s_94_2: cast zx s_94_1 -> i
        let s_94_2: i128 = (s_94_1.value() as i128);
        // C s_94_3: cast reint s_94_2 -> i64
        let s_94_3: i64 = (s_94_2 as i64);
        // C s_94_4: cast zx s_94_3 -> i
        let s_94_4: i128 = (i128::try_from(s_94_3).unwrap());
        // C s_94_5: const #432u : u32
        let s_94_5: u32 = 432;
        // D s_94_6: read-reg s_94_5:u8
        let s_94_6: u8 = {
            let value = state.read_register::<u8>(s_94_5 as isize);
            tracer.read_register(s_94_5 as isize, value);
            value
        };
        // D s_94_7: call AArch64_AArch32SystemAccessTrap(s_94_6, s_94_4)
        let s_94_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_94_6, s_94_4);
        // N s_94_8: return
        return;
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_95_0: read-var __MDCR_EL2_TDCC:u8
        let s_95_0: bool = fn_state.u__MDCR_EL2_TDCC;
        // D s_95_1: cast zx s_95_0 -> bv
        let s_95_1: Bits = Bits::new(s_95_0 as u128, 1u16);
        // C s_95_2: const #1u : u8
        let s_95_2: bool = true;
        // C s_95_3: cast zx s_95_2 -> bv
        let s_95_3: Bits = Bits::new(s_95_2 as u128, 1u16);
        // D s_95_4: cmp-eq s_95_1 s_95_3
        let s_95_4: bool = ((s_95_1) == (s_95_3));
        // D s_95_5: write-var gs#136997 <= s_95_4
        fn_state.gs_136997 = s_95_4;
        // N s_95_6: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_96_0: const #432u : u32
        let s_96_0: u32 = 432;
        // D s_96_1: read-reg s_96_0:u8
        let s_96_1: u8 = {
            let value = state.read_register::<u8>(s_96_0 as isize);
            tracer.read_register(s_96_0 as isize, value);
            value
        };
        // D s_96_2: call ELUsingAArch32(s_96_1)
        let s_96_2: bool = ELUsingAArch32(state, tracer, s_96_1);
        // D s_96_3: not s_96_2
        let s_96_3: bool = !s_96_2;
        // D s_96_4: write-var gs#136996 <= s_96_3
        fn_state.gs_136996 = s_96_3;
        // N s_96_5: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #440u : u32
        let s_97_0: u32 = 440;
        // D s_97_1: read-reg s_97_0:u8
        let s_97_1: u8 = {
            let value = state.read_register::<u8>(s_97_0 as isize);
            tracer.read_register(s_97_0 as isize, value);
            value
        };
        // D s_97_2: call ELUsingAArch32(s_97_1)
        let s_97_2: bool = ELUsingAArch32(state, tracer, s_97_1);
        // D s_97_3: not s_97_2
        let s_97_3: bool = !s_97_2;
        // N s_97_4: branch s_97_3 b194 b98
        if s_97_3 {
            return block_194(state, tracer, fn_state);
        } else {
            return block_98(state, tracer, fn_state);
        };
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #0u : u8
        let s_98_0: bool = false;
        // D s_98_1: write-var gs#137011 <= s_98_0
        fn_state.gs_137011 = s_98_0;
        // N s_98_2: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_99_0: read-var gs#137011:u8
        let s_99_0: bool = fn_state.gs_137011;
        // N s_99_1: branch s_99_0 b185 b100
        if s_99_0 {
            return block_185(state, tracer, fn_state);
        } else {
            return block_100(state, tracer, fn_state);
        };
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #440u : u32
        let s_100_0: u32 = 440;
        // D s_100_1: read-reg s_100_0:u8
        let s_100_1: u8 = {
            let value = state.read_register::<u8>(s_100_0 as isize);
            tracer.read_register(s_100_0 as isize, value);
            value
        };
        // D s_100_2: call ELUsingAArch32(s_100_1)
        let s_100_2: bool = ELUsingAArch32(state, tracer, s_100_1);
        // N s_100_3: branch s_100_2 b184 b101
        if s_100_2 {
            return block_184(state, tracer, fn_state);
        } else {
            return block_101(state, tracer, fn_state);
        };
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_101_0: const #0u : u8
        let s_101_0: bool = false;
        // D s_101_1: write-var gs#137012 <= s_101_0
        fn_state.gs_137012 = s_101_0;
        // N s_101_2: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_102_0: read-var gs#137012:u8
        let s_102_0: bool = fn_state.gs_137012;
        // N s_102_1: branch s_102_0 b167 b103
        if s_102_0 {
            return block_167(state, tracer, fn_state);
        } else {
            return block_103(state, tracer, fn_state);
        };
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_103_0: const #() : ()
        let s_103_0: () = ();
        // S s_103_1: call EL2Enabled(s_103_0)
        let s_103_1: bool = EL2Enabled(state, tracer, s_103_0);
        // N s_103_2: branch s_103_1 b166 b104
        if s_103_1 {
            return block_166(state, tracer, fn_state);
        } else {
            return block_104(state, tracer, fn_state);
        };
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_104_0: const #0u : u8
        let s_104_0: bool = false;
        // D s_104_1: write-var gs#137013 <= s_104_0
        fn_state.gs_137013 = s_104_0;
        // N s_104_2: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_105_0: read-var gs#137013:u8
        let s_105_0: bool = fn_state.gs_137013;
        // N s_105_1: branch s_105_0 b165 b106
        if s_105_0 {
            return block_165(state, tracer, fn_state);
        } else {
            return block_106(state, tracer, fn_state);
        };
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #0u : u8
        let s_106_0: bool = false;
        // D s_106_1: write-var gs#137014 <= s_106_0
        fn_state.gs_137014 = s_106_0;
        // N s_106_2: jump b107
        return block_107(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_107_0: read-var gs#137014:u8
        let s_107_0: bool = fn_state.gs_137014;
        // N s_107_1: branch s_107_0 b164 b108
        if s_107_0 {
            return block_164(state, tracer, fn_state);
        } else {
            return block_108(state, tracer, fn_state);
        };
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_108_0: const #() : ()
        let s_108_0: () = ();
        // S s_108_1: call EL2Enabled(s_108_0)
        let s_108_1: bool = EL2Enabled(state, tracer, s_108_0);
        // N s_108_2: branch s_108_1 b163 b109
        if s_108_1 {
            return block_163(state, tracer, fn_state);
        } else {
            return block_109(state, tracer, fn_state);
        };
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #0u : u8
        let s_109_0: bool = false;
        // D s_109_1: write-var gs#137015 <= s_109_0
        fn_state.gs_137015 = s_109_0;
        // N s_109_2: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_110_0: read-var gs#137015:u8
        let s_110_0: bool = fn_state.gs_137015;
        // N s_110_1: branch s_110_0 b162 b111
        if s_110_0 {
            return block_162(state, tracer, fn_state);
        } else {
            return block_111(state, tracer, fn_state);
        };
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_111_0: const #0u : u8
        let s_111_0: bool = false;
        // D s_111_1: write-var gs#137016 <= s_111_0
        fn_state.gs_137016 = s_111_0;
        // N s_111_2: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_112_0: read-var gs#137016:u8
        let s_112_0: bool = fn_state.gs_137016;
        // N s_112_1: branch s_112_0 b161 b113
        if s_112_0 {
            return block_161(state, tracer, fn_state);
        } else {
            return block_113(state, tracer, fn_state);
        };
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_113_0: const #() : ()
        let s_113_0: () = ();
        // S s_113_1: call EL2Enabled(s_113_0)
        let s_113_1: bool = EL2Enabled(state, tracer, s_113_0);
        // N s_113_2: branch s_113_1 b160 b114
        if s_113_1 {
            return block_160(state, tracer, fn_state);
        } else {
            return block_114(state, tracer, fn_state);
        };
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #0u : u8
        let s_114_0: bool = false;
        // D s_114_1: write-var gs#137017 <= s_114_0
        fn_state.gs_137017 = s_114_0;
        // N s_114_2: jump b115
        return block_115(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_115_0: read-var gs#137017:u8
        let s_115_0: bool = fn_state.gs_137017;
        // N s_115_1: branch s_115_0 b156 b116
        if s_115_0 {
            return block_156(state, tracer, fn_state);
        } else {
            return block_116(state, tracer, fn_state);
        };
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_116_0: const #0u : u8
        let s_116_0: bool = false;
        // D s_116_1: write-var gs#137019 <= s_116_0
        fn_state.gs_137019 = s_116_0;
        // N s_116_2: jump b117
        return block_117(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_117_0: read-var gs#137019:u8
        let s_117_0: bool = fn_state.gs_137019;
        // N s_117_1: branch s_117_0 b155 b118
        if s_117_0 {
            return block_155(state, tracer, fn_state);
        } else {
            return block_118(state, tracer, fn_state);
        };
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #() : ()
        let s_118_0: () = ();
        // S s_118_1: call EL2Enabled(s_118_0)
        let s_118_1: bool = EL2Enabled(state, tracer, s_118_0);
        // N s_118_2: branch s_118_1 b154 b119
        if s_118_1 {
            return block_154(state, tracer, fn_state);
        } else {
            return block_119(state, tracer, fn_state);
        };
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_119_0: const #0u : u8
        let s_119_0: bool = false;
        // D s_119_1: write-var gs#137020 <= s_119_0
        fn_state.gs_137020 = s_119_0;
        // N s_119_2: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_120_0: read-var gs#137020:u8
        let s_120_0: bool = fn_state.gs_137020;
        // N s_120_1: branch s_120_0 b150 b121
        if s_120_0 {
            return block_150(state, tracer, fn_state);
        } else {
            return block_121(state, tracer, fn_state);
        };
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_121_0: const #0u : u8
        let s_121_0: bool = false;
        // D s_121_1: write-var gs#137022 <= s_121_0
        fn_state.gs_137022 = s_121_0;
        // N s_121_2: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_122_0: read-var gs#137022:u8
        let s_122_0: bool = fn_state.gs_137022;
        // N s_122_1: branch s_122_0 b149 b123
        if s_122_0 {
            return block_149(state, tracer, fn_state);
        } else {
            return block_123(state, tracer, fn_state);
        };
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_123_0: const #424u : u32
        let s_123_0: u32 = 424;
        // D s_123_1: read-reg s_123_0:u8
        let s_123_1: u8 = {
            let value = state.read_register::<u8>(s_123_0 as isize);
            tracer.read_register(s_123_0 as isize, value);
            value
        };
        // C s_123_2: const #2u : u8
        let s_123_2: u8 = 2;
        // D s_123_3: cmp-lt s_123_1 s_123_2
        let s_123_3: bool = ((s_123_1) < (s_123_2));
        // N s_123_4: branch s_123_3 b148 b124
        if s_123_3 {
            return block_148(state, tracer, fn_state);
        } else {
            return block_124(state, tracer, fn_state);
        };
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_124_0: const #0u : u8
        let s_124_0: bool = false;
        // D s_124_1: write-var gs#137023 <= s_124_0
        fn_state.gs_137023 = s_124_0;
        // N s_124_2: jump b125
        return block_125(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_125_0: read-var gs#137023:u8
        let s_125_0: bool = fn_state.gs_137023;
        // N s_125_1: branch s_125_0 b147 b126
        if s_125_0 {
            return block_147(state, tracer, fn_state);
        } else {
            return block_126(state, tracer, fn_state);
        };
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_126_0: const #0u : u8
        let s_126_0: bool = false;
        // D s_126_1: write-var gs#137024 <= s_126_0
        fn_state.gs_137024 = s_126_0;
        // N s_126_2: jump b127
        return block_127(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_127_0: read-var gs#137024:u8
        let s_127_0: bool = fn_state.gs_137024;
        // N s_127_1: branch s_127_0 b146 b128
        if s_127_0 {
            return block_146(state, tracer, fn_state);
        } else {
            return block_128(state, tracer, fn_state);
        };
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_128_0: const #424u : u32
        let s_128_0: u32 = 424;
        // D s_128_1: read-reg s_128_0:u8
        let s_128_1: u8 = {
            let value = state.read_register::<u8>(s_128_0 as isize);
            tracer.read_register(s_128_0 as isize, value);
            value
        };
        // C s_128_2: const #2u : u8
        let s_128_2: u8 = 2;
        // D s_128_3: cmp-lt s_128_1 s_128_2
        let s_128_3: bool = ((s_128_1) < (s_128_2));
        // N s_128_4: branch s_128_3 b145 b129
        if s_128_3 {
            return block_145(state, tracer, fn_state);
        } else {
            return block_129(state, tracer, fn_state);
        };
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_129_0: const #0u : u8
        let s_129_0: bool = false;
        // D s_129_1: write-var gs#137025 <= s_129_0
        fn_state.gs_137025 = s_129_0;
        // N s_129_2: jump b130
        return block_130(state, tracer, fn_state);
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_130_0: read-var gs#137025:u8
        let s_130_0: bool = fn_state.gs_137025;
        // N s_130_1: branch s_130_0 b144 b131
        if s_130_0 {
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
        // C s_131_0: const #0u : u8
        let s_131_0: bool = false;
        // D s_131_1: write-var gs#137026 <= s_131_0
        fn_state.gs_137026 = s_131_0;
        // N s_131_2: jump b132
        return block_132(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_132_0: read-var gs#137026:u8
        let s_132_0: bool = fn_state.gs_137026;
        // N s_132_1: branch s_132_0 b143 b133
        if s_132_0 {
            return block_143(state, tracer, fn_state);
        } else {
            return block_133(state, tracer, fn_state);
        };
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_133_0: const #424u : u32
        let s_133_0: u32 = 424;
        // D s_133_1: read-reg s_133_0:u8
        let s_133_1: u8 = {
            let value = state.read_register::<u8>(s_133_0 as isize);
            tracer.read_register(s_133_0 as isize, value);
            value
        };
        // C s_133_2: const #2u : u8
        let s_133_2: u8 = 2;
        // D s_133_3: cmp-lt s_133_1 s_133_2
        let s_133_3: bool = ((s_133_1) < (s_133_2));
        // N s_133_4: branch s_133_3 b142 b134
        if s_133_3 {
            return block_142(state, tracer, fn_state);
        } else {
            return block_134(state, tracer, fn_state);
        };
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_134_0: const #0u : u8
        let s_134_0: bool = false;
        // D s_134_1: write-var gs#137027 <= s_134_0
        fn_state.gs_137027 = s_134_0;
        // N s_134_2: jump b135
        return block_135(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_135_0: read-var gs#137027:u8
        let s_135_0: bool = fn_state.gs_137027;
        // N s_135_1: branch s_135_0 b141 b136
        if s_135_0 {
            return block_141(state, tracer, fn_state);
        } else {
            return block_136(state, tracer, fn_state);
        };
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_136_0: const #0u : u8
        let s_136_0: bool = false;
        // D s_136_1: write-var gs#137028 <= s_136_0
        fn_state.gs_137028 = s_136_0;
        // N s_136_2: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_137_0: read-var gs#137028:u8
        let s_137_0: bool = fn_state.gs_137028;
        // N s_137_1: branch s_137_0 b140 b138
        if s_137_0 {
            return block_140(state, tracer, fn_state);
        } else {
            return block_138(state, tracer, fn_state);
        };
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_138_0: const #4s : i64
        let s_138_0: i64 = 4;
        // D s_138_1: read-var address:u32
        let s_138_1: u32 = fn_state.address;
        // D s_138_2: call MemA_read(s_138_1, s_138_0)
        let s_138_2: Bits = MemA_read(state, tracer, s_138_1, s_138_0);
        // D s_138_3: write-var gs#670404 <= s_138_2
        fn_state.gs_670404 = s_138_2;
        // N s_138_4: jump b139
        return block_139(state, tracer, fn_state);
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_139_0: read-var gs#670404:bv
        let s_139_0: Bits = fn_state.gs_670404;
        // D s_139_1: cast reint s_139_0 -> u32
        let s_139_1: u32 = (s_139_0.value() as u32);
        // D s_139_2: call Mk_DBGDTRTXint_Type(s_139_1)
        let s_139_2: ProductType700c18a878c5601b = Mk_DBGDTRTXint_Type(
            state,
            tracer,
            s_139_1,
        );
        // D s_139_3: call DBGDTRTXint_write(s_139_2)
        let s_139_3: () = DBGDTRTXint_write(state, tracer, s_139_2);
        // N s_139_4: return
        return;
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_140_0: const #6u : u8
        let s_140_0: u8 = 6;
        // C s_140_1: cast zx s_140_0 -> bv
        let s_140_1: Bits = Bits::new(s_140_0 as u128, 8u16);
        // C s_140_2: cast zx s_140_1 -> i
        let s_140_2: i128 = (s_140_1.value() as i128);
        // C s_140_3: cast reint s_140_2 -> i64
        let s_140_3: i64 = (s_140_2 as i64);
        // C s_140_4: cast zx s_140_3 -> i
        let s_140_4: i128 = (i128::try_from(s_140_3).unwrap());
        // C s_140_5: const #424u : u32
        let s_140_5: u32 = 424;
        // D s_140_6: read-reg s_140_5:u8
        let s_140_6: u8 = {
            let value = state.read_register::<u8>(s_140_5 as isize);
            tracer.read_register(s_140_5 as isize, value);
            value
        };
        // D s_140_7: call AArch64_AArch32SystemAccessTrap(s_140_6, s_140_4)
        let s_140_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_140_6,
            s_140_4,
        );
        // N s_140_8: return
        return;
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_141_0: read-var __MDCR_EL3_TDA:u8
        let s_141_0: bool = fn_state.u__MDCR_EL3_TDA;
        // D s_141_1: cast zx s_141_0 -> bv
        let s_141_1: Bits = Bits::new(s_141_0 as u128, 1u16);
        // C s_141_2: const #1u : u8
        let s_141_2: bool = true;
        // C s_141_3: cast zx s_141_2 -> bv
        let s_141_3: Bits = Bits::new(s_141_2 as u128, 1u16);
        // D s_141_4: cmp-eq s_141_1 s_141_3
        let s_141_4: bool = ((s_141_1) == (s_141_3));
        // D s_141_5: write-var gs#137028 <= s_141_4
        fn_state.gs_137028 = s_141_4;
        // N s_141_6: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_142_0: const #424u : u32
        let s_142_0: u32 = 424;
        // D s_142_1: read-reg s_142_0:u8
        let s_142_1: u8 = {
            let value = state.read_register::<u8>(s_142_0 as isize);
            tracer.read_register(s_142_0 as isize, value);
            value
        };
        // D s_142_2: call ELUsingAArch32(s_142_1)
        let s_142_2: bool = ELUsingAArch32(state, tracer, s_142_1);
        // D s_142_3: not s_142_2
        let s_142_3: bool = !s_142_2;
        // D s_142_4: write-var gs#137027 <= s_142_3
        fn_state.gs_137027 = s_142_3;
        // N s_142_5: jump b135
        return block_135(state, tracer, fn_state);
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_143_0: const #() : ()
        let s_143_0: () = ();
        // S s_143_1: call AArch32_TakeMonitorTrapException(s_143_0)
        let s_143_1: () = AArch32_TakeMonitorTrapException(state, tracer, s_143_0);
        // N s_143_2: return
        return;
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_144_0: read-var __SDCR_TDCC:u8
        let s_144_0: bool = fn_state.u__SDCR_TDCC;
        // D s_144_1: cast zx s_144_0 -> bv
        let s_144_1: Bits = Bits::new(s_144_0 as u128, 1u16);
        // C s_144_2: const #1u : u8
        let s_144_2: bool = true;
        // C s_144_3: cast zx s_144_2 -> bv
        let s_144_3: Bits = Bits::new(s_144_2 as u128, 1u16);
        // D s_144_4: cmp-eq s_144_1 s_144_3
        let s_144_4: bool = ((s_144_1) == (s_144_3));
        // D s_144_5: write-var gs#137026 <= s_144_4
        fn_state.gs_137026 = s_144_4;
        // N s_144_6: jump b132
        return block_132(state, tracer, fn_state);
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_145_0: const #424u : u32
        let s_145_0: u32 = 424;
        // D s_145_1: read-reg s_145_0:u8
        let s_145_1: u8 = {
            let value = state.read_register::<u8>(s_145_0 as isize);
            tracer.read_register(s_145_0 as isize, value);
            value
        };
        // D s_145_2: call ELUsingAArch32(s_145_1)
        let s_145_2: bool = ELUsingAArch32(state, tracer, s_145_1);
        // D s_145_3: write-var gs#137025 <= s_145_2
        fn_state.gs_137025 = s_145_2;
        // N s_145_4: jump b130
        return block_130(state, tracer, fn_state);
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_146_0: const #6u : u8
        let s_146_0: u8 = 6;
        // C s_146_1: cast zx s_146_0 -> bv
        let s_146_1: Bits = Bits::new(s_146_0 as u128, 8u16);
        // C s_146_2: cast zx s_146_1 -> i
        let s_146_2: i128 = (s_146_1.value() as i128);
        // C s_146_3: cast reint s_146_2 -> i64
        let s_146_3: i64 = (s_146_2 as i64);
        // C s_146_4: cast zx s_146_3 -> i
        let s_146_4: i128 = (i128::try_from(s_146_3).unwrap());
        // C s_146_5: const #424u : u32
        let s_146_5: u32 = 424;
        // D s_146_6: read-reg s_146_5:u8
        let s_146_6: u8 = {
            let value = state.read_register::<u8>(s_146_5 as isize);
            tracer.read_register(s_146_5 as isize, value);
            value
        };
        // D s_146_7: call AArch64_AArch32SystemAccessTrap(s_146_6, s_146_4)
        let s_146_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_146_6,
            s_146_4,
        );
        // N s_146_8: return
        return;
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_147_0: read-var __MDCR_EL3_TDCC:u8
        let s_147_0: bool = fn_state.u__MDCR_EL3_TDCC;
        // D s_147_1: cast zx s_147_0 -> bv
        let s_147_1: Bits = Bits::new(s_147_0 as u128, 1u16);
        // C s_147_2: const #1u : u8
        let s_147_2: bool = true;
        // C s_147_3: cast zx s_147_2 -> bv
        let s_147_3: Bits = Bits::new(s_147_2 as u128, 1u16);
        // D s_147_4: cmp-eq s_147_1 s_147_3
        let s_147_4: bool = ((s_147_1) == (s_147_3));
        // D s_147_5: write-var gs#137024 <= s_147_4
        fn_state.gs_137024 = s_147_4;
        // N s_147_6: jump b127
        return block_127(state, tracer, fn_state);
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_148_0: const #424u : u32
        let s_148_0: u32 = 424;
        // D s_148_1: read-reg s_148_0:u8
        let s_148_1: u8 = {
            let value = state.read_register::<u8>(s_148_0 as isize);
            tracer.read_register(s_148_0 as isize, value);
            value
        };
        // D s_148_2: call ELUsingAArch32(s_148_1)
        let s_148_2: bool = ELUsingAArch32(state, tracer, s_148_1);
        // D s_148_3: not s_148_2
        let s_148_3: bool = !s_148_2;
        // D s_148_4: write-var gs#137023 <= s_148_3
        fn_state.gs_137023 = s_148_3;
        // N s_148_5: jump b125
        return block_125(state, tracer, fn_state);
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_149_0: const #6u : u8
        let s_149_0: u8 = 6;
        // C s_149_1: cast zx s_149_0 -> bv
        let s_149_1: Bits = Bits::new(s_149_0 as u128, 8u16);
        // C s_149_2: cast zx s_149_1 -> i
        let s_149_2: i128 = (s_149_1.value() as i128);
        // C s_149_3: cast reint s_149_2 -> i64
        let s_149_3: i64 = (s_149_2 as i64);
        // C s_149_4: cast zx s_149_3 -> i
        let s_149_4: i128 = (i128::try_from(s_149_3).unwrap());
        // S s_149_5: call AArch32_TakeHypTrapException(s_149_4)
        let s_149_5: () = AArch32_TakeHypTrapException(state, tracer, s_149_4);
        // N s_149_6: return
        return;
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_150_0: const #() : ()
        let s_150_0: () = ();
        // S s_150_1: call HCR_read(s_150_0)
        let s_150_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_150_0);
        // S s_150_2: call _get_HCR_Type_TGE(s_150_1)
        let s_150_2: bool = u_get_HCR_Type_TGE(state, tracer, s_150_1);
        // S s_150_3: cast zx s_150_2 -> bv
        let s_150_3: Bits = Bits::new(s_150_2 as u128, 1u16);
        // C s_150_4: const #1u : u8
        let s_150_4: bool = true;
        // C s_150_5: cast zx s_150_4 -> bv
        let s_150_5: Bits = Bits::new(s_150_4 as u128, 1u16);
        // S s_150_6: cmp-eq s_150_3 s_150_5
        let s_150_6: bool = ((s_150_3) == (s_150_5));
        // N s_150_7: branch s_150_6 b153 b151
        if s_150_6 {
            return block_153(state, tracer, fn_state);
        } else {
            return block_151(state, tracer, fn_state);
        };
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_151_0: const #() : ()
        let s_151_0: () = ();
        // S s_151_1: call HDCR_read(s_151_0)
        let s_151_1: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_151_0);
        // S s_151_2: call _get_HDCR_Type_TDE(s_151_1)
        let s_151_2: bool = u_get_HDCR_Type_TDE(state, tracer, s_151_1);
        // C s_151_3: const #() : ()
        let s_151_3: () = ();
        // S s_151_4: call HDCR_read(s_151_3)
        let s_151_4: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_151_3);
        // S s_151_5: call _get_HDCR_Type_TDA(s_151_4)
        let s_151_5: bool = u_get_HDCR_Type_TDA(state, tracer, s_151_4);
        // S s_151_6: cast zx s_151_2 -> bv
        let s_151_6: Bits = Bits::new(s_151_2 as u128, 1u16);
        // S s_151_7: cast zx s_151_5 -> bv
        let s_151_7: Bits = Bits::new(s_151_5 as u128, 1u16);
        // S s_151_8: cast reint s_151_6 -> u128
        let s_151_8: u128 = (s_151_6.value() as u128);
        // D s_151_9: size-of s_151_6
        let s_151_9: u16 = s_151_6.length();
        // S s_151_10: cast reint s_151_7 -> u128
        let s_151_10: u128 = (s_151_7.value() as u128);
        // D s_151_11: size-of s_151_7
        let s_151_11: u16 = s_151_7.length();
        // D s_151_12: lsl s_151_8 s_151_11
        let s_151_12: u128 = s_151_8 << s_151_11;
        // D s_151_13: or s_151_12 s_151_10
        let s_151_13: u128 = ((s_151_12) | (s_151_10));
        // D s_151_14: add s_151_9 s_151_11
        let s_151_14: u16 = (s_151_9 + s_151_11);
        // D s_151_15: create-bits s_151_13 s_151_14
        let s_151_15: Bits = Bits::new(s_151_13, s_151_14);
        // D s_151_16: cast reint s_151_15 -> u8
        let s_151_16: u8 = (s_151_15.value() as u8);
        // D s_151_17: cast zx s_151_16 -> bv
        let s_151_17: Bits = Bits::new(s_151_16 as u128, 2u16);
        // C s_151_18: const #0u : u8
        let s_151_18: u8 = 0;
        // C s_151_19: cast zx s_151_18 -> bv
        let s_151_19: Bits = Bits::new(s_151_18 as u128, 2u16);
        // D s_151_20: cmp-ne s_151_17 s_151_19
        let s_151_20: bool = ((s_151_17) != (s_151_19));
        // D s_151_21: write-var gs#137021 <= s_151_20
        fn_state.gs_137021 = s_151_20;
        // N s_151_22: jump b152
        return block_152(state, tracer, fn_state);
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_152_0: read-var gs#137021:u8
        let s_152_0: bool = fn_state.gs_137021;
        // D s_152_1: write-var gs#137022 <= s_152_0
        fn_state.gs_137022 = s_152_0;
        // N s_152_2: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_153_0: const #1u : u8
        let s_153_0: bool = true;
        // D s_153_1: write-var gs#137021 <= s_153_0
        fn_state.gs_137021 = s_153_0;
        // N s_153_2: jump b152
        return block_152(state, tracer, fn_state);
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_154_0: const #432u : u32
        let s_154_0: u32 = 432;
        // D s_154_1: read-reg s_154_0:u8
        let s_154_1: u8 = {
            let value = state.read_register::<u8>(s_154_0 as isize);
            tracer.read_register(s_154_0 as isize, value);
            value
        };
        // D s_154_2: call ELUsingAArch32(s_154_1)
        let s_154_2: bool = ELUsingAArch32(state, tracer, s_154_1);
        // D s_154_3: write-var gs#137020 <= s_154_2
        fn_state.gs_137020 = s_154_2;
        // N s_154_4: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_155_0: const #6u : u8
        let s_155_0: u8 = 6;
        // C s_155_1: cast zx s_155_0 -> bv
        let s_155_1: Bits = Bits::new(s_155_0 as u128, 8u16);
        // C s_155_2: cast zx s_155_1 -> i
        let s_155_2: i128 = (s_155_1.value() as i128);
        // C s_155_3: cast reint s_155_2 -> i64
        let s_155_3: i64 = (s_155_2 as i64);
        // C s_155_4: cast zx s_155_3 -> i
        let s_155_4: i128 = (i128::try_from(s_155_3).unwrap());
        // C s_155_5: const #432u : u32
        let s_155_5: u32 = 432;
        // D s_155_6: read-reg s_155_5:u8
        let s_155_6: u8 = {
            let value = state.read_register::<u8>(s_155_5 as isize);
            tracer.read_register(s_155_5 as isize, value);
            value
        };
        // D s_155_7: call AArch64_AArch32SystemAccessTrap(s_155_6, s_155_4)
        let s_155_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_155_6,
            s_155_4,
        );
        // N s_155_8: return
        return;
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_156_0: const #102552u : u32
        let s_156_0: u32 = 102552;
        // D s_156_1: read-reg s_156_0:struct
        let s_156_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_156_0 as isize);
            tracer.read_register(s_156_0 as isize, value);
            value
        };
        // D s_156_2: call _get_HCR_EL2_Type_TGE(s_156_1)
        let s_156_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_156_1);
        // D s_156_3: cast zx s_156_2 -> bv
        let s_156_3: Bits = Bits::new(s_156_2 as u128, 1u16);
        // C s_156_4: const #1u : u8
        let s_156_4: bool = true;
        // C s_156_5: cast zx s_156_4 -> bv
        let s_156_5: Bits = Bits::new(s_156_4 as u128, 1u16);
        // D s_156_6: cmp-eq s_156_3 s_156_5
        let s_156_6: bool = ((s_156_3) == (s_156_5));
        // N s_156_7: branch s_156_6 b159 b157
        if s_156_6 {
            return block_159(state, tracer, fn_state);
        } else {
            return block_157(state, tracer, fn_state);
        };
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_157_0: const #104880u : u32
        let s_157_0: u32 = 104880;
        // D s_157_1: read-reg s_157_0:struct
        let s_157_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_157_0 as isize);
            tracer.read_register(s_157_0 as isize, value);
            value
        };
        // D s_157_2: call _get_MDCR_EL2_Type_TDE(s_157_1)
        let s_157_2: bool = u_get_MDCR_EL2_Type_TDE(state, tracer, s_157_1);
        // C s_157_3: const #104880u : u32
        let s_157_3: u32 = 104880;
        // D s_157_4: read-reg s_157_3:struct
        let s_157_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_157_3 as isize);
            tracer.read_register(s_157_3 as isize, value);
            value
        };
        // D s_157_5: call _get_MDCR_EL2_Type_TDA(s_157_4)
        let s_157_5: bool = u_get_MDCR_EL2_Type_TDA(state, tracer, s_157_4);
        // D s_157_6: cast zx s_157_2 -> bv
        let s_157_6: Bits = Bits::new(s_157_2 as u128, 1u16);
        // D s_157_7: cast zx s_157_5 -> bv
        let s_157_7: Bits = Bits::new(s_157_5 as u128, 1u16);
        // D s_157_8: cast reint s_157_6 -> u128
        let s_157_8: u128 = (s_157_6.value() as u128);
        // D s_157_9: size-of s_157_6
        let s_157_9: u16 = s_157_6.length();
        // D s_157_10: cast reint s_157_7 -> u128
        let s_157_10: u128 = (s_157_7.value() as u128);
        // D s_157_11: size-of s_157_7
        let s_157_11: u16 = s_157_7.length();
        // D s_157_12: lsl s_157_8 s_157_11
        let s_157_12: u128 = s_157_8 << s_157_11;
        // D s_157_13: or s_157_12 s_157_10
        let s_157_13: u128 = ((s_157_12) | (s_157_10));
        // D s_157_14: add s_157_9 s_157_11
        let s_157_14: u16 = (s_157_9 + s_157_11);
        // D s_157_15: create-bits s_157_13 s_157_14
        let s_157_15: Bits = Bits::new(s_157_13, s_157_14);
        // D s_157_16: cast reint s_157_15 -> u8
        let s_157_16: u8 = (s_157_15.value() as u8);
        // D s_157_17: cast zx s_157_16 -> bv
        let s_157_17: Bits = Bits::new(s_157_16 as u128, 2u16);
        // C s_157_18: const #0u : u8
        let s_157_18: u8 = 0;
        // C s_157_19: cast zx s_157_18 -> bv
        let s_157_19: Bits = Bits::new(s_157_18 as u128, 2u16);
        // D s_157_20: cmp-ne s_157_17 s_157_19
        let s_157_20: bool = ((s_157_17) != (s_157_19));
        // D s_157_21: write-var gs#137018 <= s_157_20
        fn_state.gs_137018 = s_157_20;
        // N s_157_22: jump b158
        return block_158(state, tracer, fn_state);
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_158_0: read-var gs#137018:u8
        let s_158_0: bool = fn_state.gs_137018;
        // D s_158_1: write-var gs#137019 <= s_158_0
        fn_state.gs_137019 = s_158_0;
        // N s_158_2: jump b117
        return block_117(state, tracer, fn_state);
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_159_0: const #1u : u8
        let s_159_0: bool = true;
        // D s_159_1: write-var gs#137018 <= s_159_0
        fn_state.gs_137018 = s_159_0;
        // N s_159_2: jump b158
        return block_158(state, tracer, fn_state);
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_160_0: const #432u : u32
        let s_160_0: u32 = 432;
        // D s_160_1: read-reg s_160_0:u8
        let s_160_1: u8 = {
            let value = state.read_register::<u8>(s_160_0 as isize);
            tracer.read_register(s_160_0 as isize, value);
            value
        };
        // D s_160_2: call ELUsingAArch32(s_160_1)
        let s_160_2: bool = ELUsingAArch32(state, tracer, s_160_1);
        // D s_160_3: not s_160_2
        let s_160_3: bool = !s_160_2;
        // D s_160_4: write-var gs#137017 <= s_160_3
        fn_state.gs_137017 = s_160_3;
        // N s_160_5: jump b115
        return block_115(state, tracer, fn_state);
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_161_0: const #6u : u8
        let s_161_0: u8 = 6;
        // C s_161_1: cast zx s_161_0 -> bv
        let s_161_1: Bits = Bits::new(s_161_0 as u128, 8u16);
        // C s_161_2: cast zx s_161_1 -> i
        let s_161_2: i128 = (s_161_1.value() as i128);
        // C s_161_3: cast reint s_161_2 -> i64
        let s_161_3: i64 = (s_161_2 as i64);
        // C s_161_4: cast zx s_161_3 -> i
        let s_161_4: i128 = (i128::try_from(s_161_3).unwrap());
        // S s_161_5: call AArch32_TakeHypTrapException(s_161_4)
        let s_161_5: () = AArch32_TakeHypTrapException(state, tracer, s_161_4);
        // N s_161_6: return
        return;
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_162_0: read-var __HDCR_TDCC:u8
        let s_162_0: bool = fn_state.u__HDCR_TDCC;
        // D s_162_1: cast zx s_162_0 -> bv
        let s_162_1: Bits = Bits::new(s_162_0 as u128, 1u16);
        // C s_162_2: const #1u : u8
        let s_162_2: bool = true;
        // C s_162_3: cast zx s_162_2 -> bv
        let s_162_3: Bits = Bits::new(s_162_2 as u128, 1u16);
        // D s_162_4: cmp-eq s_162_1 s_162_3
        let s_162_4: bool = ((s_162_1) == (s_162_3));
        // D s_162_5: write-var gs#137016 <= s_162_4
        fn_state.gs_137016 = s_162_4;
        // N s_162_6: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_163_0: const #432u : u32
        let s_163_0: u32 = 432;
        // D s_163_1: read-reg s_163_0:u8
        let s_163_1: u8 = {
            let value = state.read_register::<u8>(s_163_0 as isize);
            tracer.read_register(s_163_0 as isize, value);
            value
        };
        // D s_163_2: call ELUsingAArch32(s_163_1)
        let s_163_2: bool = ELUsingAArch32(state, tracer, s_163_1);
        // D s_163_3: write-var gs#137015 <= s_163_2
        fn_state.gs_137015 = s_163_2;
        // N s_163_4: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_164_0: const #6u : u8
        let s_164_0: u8 = 6;
        // C s_164_1: cast zx s_164_0 -> bv
        let s_164_1: Bits = Bits::new(s_164_0 as u128, 8u16);
        // C s_164_2: cast zx s_164_1 -> i
        let s_164_2: i128 = (s_164_1.value() as i128);
        // C s_164_3: cast reint s_164_2 -> i64
        let s_164_3: i64 = (s_164_2 as i64);
        // C s_164_4: cast zx s_164_3 -> i
        let s_164_4: i128 = (i128::try_from(s_164_3).unwrap());
        // C s_164_5: const #432u : u32
        let s_164_5: u32 = 432;
        // D s_164_6: read-reg s_164_5:u8
        let s_164_6: u8 = {
            let value = state.read_register::<u8>(s_164_5 as isize);
            tracer.read_register(s_164_5 as isize, value);
            value
        };
        // D s_164_7: call AArch64_AArch32SystemAccessTrap(s_164_6, s_164_4)
        let s_164_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_164_6,
            s_164_4,
        );
        // N s_164_8: return
        return;
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_165_0: read-var __MDCR_EL2_TDCC:u8
        let s_165_0: bool = fn_state.u__MDCR_EL2_TDCC;
        // D s_165_1: cast zx s_165_0 -> bv
        let s_165_1: Bits = Bits::new(s_165_0 as u128, 1u16);
        // C s_165_2: const #1u : u8
        let s_165_2: bool = true;
        // C s_165_3: cast zx s_165_2 -> bv
        let s_165_3: Bits = Bits::new(s_165_2 as u128, 1u16);
        // D s_165_4: cmp-eq s_165_1 s_165_3
        let s_165_4: bool = ((s_165_1) == (s_165_3));
        // D s_165_5: write-var gs#137014 <= s_165_4
        fn_state.gs_137014 = s_165_4;
        // N s_165_6: jump b107
        return block_107(state, tracer, fn_state);
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_166_0: const #432u : u32
        let s_166_0: u32 = 432;
        // D s_166_1: read-reg s_166_0:u8
        let s_166_1: u8 = {
            let value = state.read_register::<u8>(s_166_0 as isize);
            tracer.read_register(s_166_0 as isize, value);
            value
        };
        // D s_166_2: call ELUsingAArch32(s_166_1)
        let s_166_2: bool = ELUsingAArch32(state, tracer, s_166_1);
        // D s_166_3: not s_166_2
        let s_166_3: bool = !s_166_2;
        // D s_166_4: write-var gs#137013 <= s_166_3
        fn_state.gs_137013 = s_166_3;
        // N s_166_5: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_167_0: const #() : ()
        let s_167_0: () = ();
        // S s_167_1: call EL2Enabled(s_167_0)
        let s_167_1: bool = EL2Enabled(state, tracer, s_167_0);
        // N s_167_2: branch s_167_1 b183 b168
        if s_167_1 {
            return block_183(state, tracer, fn_state);
        } else {
            return block_168(state, tracer, fn_state);
        };
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_168_0: const #0u : u8
        let s_168_0: bool = false;
        // D s_168_1: write-var gs#137030 <= s_168_0
        fn_state.gs_137030 = s_168_0;
        // N s_168_2: jump b169
        return block_169(state, tracer, fn_state);
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_169_0: read-var gs#137030:u8
        let s_169_0: bool = fn_state.gs_137030;
        // N s_169_1: branch s_169_0 b182 b170
        if s_169_0 {
            return block_182(state, tracer, fn_state);
        } else {
            return block_170(state, tracer, fn_state);
        };
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_170_0: const #0u : u8
        let s_170_0: bool = false;
        // D s_170_1: write-var gs#137031 <= s_170_0
        fn_state.gs_137031 = s_170_0;
        // N s_170_2: jump b171
        return block_171(state, tracer, fn_state);
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_171_0: read-var gs#137031:u8
        let s_171_0: bool = fn_state.gs_137031;
        // N s_171_1: branch s_171_0 b181 b172
        if s_171_0 {
            return block_181(state, tracer, fn_state);
        } else {
            return block_172(state, tracer, fn_state);
        };
    }
    fn block_172<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_172_0: const #() : ()
        let s_172_0: () = ();
        // S s_172_1: call EL2Enabled(s_172_0)
        let s_172_1: bool = EL2Enabled(state, tracer, s_172_0);
        // N s_172_2: branch s_172_1 b180 b173
        if s_172_1 {
            return block_180(state, tracer, fn_state);
        } else {
            return block_173(state, tracer, fn_state);
        };
    }
    fn block_173<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_173_0: const #0u : u8
        let s_173_0: bool = false;
        // D s_173_1: write-var gs#137032 <= s_173_0
        fn_state.gs_137032 = s_173_0;
        // N s_173_2: jump b174
        return block_174(state, tracer, fn_state);
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_174_0: read-var gs#137032:u8
        let s_174_0: bool = fn_state.gs_137032;
        // N s_174_1: branch s_174_0 b179 b175
        if s_174_0 {
            return block_179(state, tracer, fn_state);
        } else {
            return block_175(state, tracer, fn_state);
        };
    }
    fn block_175<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_175_0: const #0u : u8
        let s_175_0: bool = false;
        // D s_175_1: write-var gs#137033 <= s_175_0
        fn_state.gs_137033 = s_175_0;
        // N s_175_2: jump b176
        return block_176(state, tracer, fn_state);
    }
    fn block_176<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_176_0: read-var gs#137033:u8
        let s_176_0: bool = fn_state.gs_137033;
        // N s_176_1: branch s_176_0 b178 b177
        if s_176_0 {
            return block_178(state, tracer, fn_state);
        } else {
            return block_177(state, tracer, fn_state);
        };
    }
    fn block_177<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_177_0: panic
        panic!("{:?}", ());
        // N s_177_1: return
        return;
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_178_0: const #0u : u8
        let s_178_0: u8 = 0;
        // C s_178_1: cast zx s_178_0 -> bv
        let s_178_1: Bits = Bits::new(s_178_0 as u128, 8u16);
        // C s_178_2: cast zx s_178_1 -> i
        let s_178_2: i128 = (s_178_1.value() as i128);
        // C s_178_3: cast reint s_178_2 -> i64
        let s_178_3: i64 = (s_178_2 as i64);
        // C s_178_4: cast zx s_178_3 -> i
        let s_178_4: i128 = (i128::try_from(s_178_3).unwrap());
        // S s_178_5: call AArch32_TakeHypTrapException(s_178_4)
        let s_178_5: () = AArch32_TakeHypTrapException(state, tracer, s_178_4);
        // N s_178_6: return
        return;
    }
    fn block_179<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_179_0: const #() : ()
        let s_179_0: () = ();
        // S s_179_1: call HCR_read(s_179_0)
        let s_179_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_179_0);
        // S s_179_2: call _get_HCR_Type_TGE(s_179_1)
        let s_179_2: bool = u_get_HCR_Type_TGE(state, tracer, s_179_1);
        // S s_179_3: cast zx s_179_2 -> bv
        let s_179_3: Bits = Bits::new(s_179_2 as u128, 1u16);
        // C s_179_4: const #1u : u8
        let s_179_4: bool = true;
        // C s_179_5: cast zx s_179_4 -> bv
        let s_179_5: Bits = Bits::new(s_179_4 as u128, 1u16);
        // S s_179_6: cmp-eq s_179_3 s_179_5
        let s_179_6: bool = ((s_179_3) == (s_179_5));
        // D s_179_7: write-var gs#137033 <= s_179_6
        fn_state.gs_137033 = s_179_6;
        // N s_179_8: jump b176
        return block_176(state, tracer, fn_state);
    }
    fn block_180<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_180_0: const #432u : u32
        let s_180_0: u32 = 432;
        // D s_180_1: read-reg s_180_0:u8
        let s_180_1: u8 = {
            let value = state.read_register::<u8>(s_180_0 as isize);
            tracer.read_register(s_180_0 as isize, value);
            value
        };
        // D s_180_2: call ELUsingAArch32(s_180_1)
        let s_180_2: bool = ELUsingAArch32(state, tracer, s_180_1);
        // D s_180_3: write-var gs#137032 <= s_180_2
        fn_state.gs_137032 = s_180_2;
        // N s_180_4: jump b174
        return block_174(state, tracer, fn_state);
    }
    fn block_181<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_181_0: const #6u : u8
        let s_181_0: u8 = 6;
        // C s_181_1: cast zx s_181_0 -> bv
        let s_181_1: Bits = Bits::new(s_181_0 as u128, 8u16);
        // C s_181_2: cast zx s_181_1 -> i
        let s_181_2: i128 = (s_181_1.value() as i128);
        // C s_181_3: cast reint s_181_2 -> i64
        let s_181_3: i64 = (s_181_2 as i64);
        // C s_181_4: cast zx s_181_3 -> i
        let s_181_4: i128 = (i128::try_from(s_181_3).unwrap());
        // C s_181_5: const #432u : u32
        let s_181_5: u32 = 432;
        // D s_181_6: read-reg s_181_5:u8
        let s_181_6: u8 = {
            let value = state.read_register::<u8>(s_181_5 as isize);
            tracer.read_register(s_181_5 as isize, value);
            value
        };
        // D s_181_7: call AArch64_AArch32SystemAccessTrap(s_181_6, s_181_4)
        let s_181_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_181_6,
            s_181_4,
        );
        // N s_181_8: return
        return;
    }
    fn block_182<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_182_0: const #102552u : u32
        let s_182_0: u32 = 102552;
        // D s_182_1: read-reg s_182_0:struct
        let s_182_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_182_0 as isize);
            tracer.read_register(s_182_0 as isize, value);
            value
        };
        // D s_182_2: call _get_HCR_EL2_Type_TGE(s_182_1)
        let s_182_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_182_1);
        // D s_182_3: cast zx s_182_2 -> bv
        let s_182_3: Bits = Bits::new(s_182_2 as u128, 1u16);
        // C s_182_4: const #1u : u8
        let s_182_4: bool = true;
        // C s_182_5: cast zx s_182_4 -> bv
        let s_182_5: Bits = Bits::new(s_182_4 as u128, 1u16);
        // D s_182_6: cmp-eq s_182_3 s_182_5
        let s_182_6: bool = ((s_182_3) == (s_182_5));
        // D s_182_7: write-var gs#137031 <= s_182_6
        fn_state.gs_137031 = s_182_6;
        // N s_182_8: jump b171
        return block_171(state, tracer, fn_state);
    }
    fn block_183<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_183_0: const #432u : u32
        let s_183_0: u32 = 432;
        // D s_183_1: read-reg s_183_0:u8
        let s_183_1: u8 = {
            let value = state.read_register::<u8>(s_183_0 as isize);
            tracer.read_register(s_183_0 as isize, value);
            value
        };
        // D s_183_2: call ELUsingAArch32(s_183_1)
        let s_183_2: bool = ELUsingAArch32(state, tracer, s_183_1);
        // D s_183_3: not s_183_2
        let s_183_3: bool = !s_183_2;
        // D s_183_4: write-var gs#137030 <= s_183_3
        fn_state.gs_137030 = s_183_3;
        // N s_183_5: jump b169
        return block_169(state, tracer, fn_state);
    }
    fn block_184<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_184_0: read-var __DBGDSCRext_UDCCdis:u8
        let s_184_0: bool = fn_state.u__DBGDSCRext_UDCCdis;
        // D s_184_1: cast zx s_184_0 -> bv
        let s_184_1: Bits = Bits::new(s_184_0 as u128, 1u16);
        // C s_184_2: const #1u : u8
        let s_184_2: bool = true;
        // C s_184_3: cast zx s_184_2 -> bv
        let s_184_3: Bits = Bits::new(s_184_2 as u128, 1u16);
        // D s_184_4: cmp-eq s_184_1 s_184_3
        let s_184_4: bool = ((s_184_1) == (s_184_3));
        // D s_184_5: write-var gs#137012 <= s_184_4
        fn_state.gs_137012 = s_184_4;
        // N s_184_6: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_185<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_185_0: const #() : ()
        let s_185_0: () = ();
        // S s_185_1: call EL2Enabled(s_185_0)
        let s_185_1: bool = EL2Enabled(state, tracer, s_185_0);
        // N s_185_2: branch s_185_1 b193 b186
        if s_185_1 {
            return block_193(state, tracer, fn_state);
        } else {
            return block_186(state, tracer, fn_state);
        };
    }
    fn block_186<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_186_0: const #0u : u8
        let s_186_0: bool = false;
        // D s_186_1: write-var gs#137034 <= s_186_0
        fn_state.gs_137034 = s_186_0;
        // N s_186_2: jump b187
        return block_187(state, tracer, fn_state);
    }
    fn block_187<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_187_0: read-var gs#137034:u8
        let s_187_0: bool = fn_state.gs_137034;
        // N s_187_1: branch s_187_0 b192 b188
        if s_187_0 {
            return block_192(state, tracer, fn_state);
        } else {
            return block_188(state, tracer, fn_state);
        };
    }
    fn block_188<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_188_0: const #0u : u8
        let s_188_0: bool = false;
        // D s_188_1: write-var gs#137035 <= s_188_0
        fn_state.gs_137035 = s_188_0;
        // N s_188_2: jump b189
        return block_189(state, tracer, fn_state);
    }
    fn block_189<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_189_0: read-var gs#137035:u8
        let s_189_0: bool = fn_state.gs_137035;
        // N s_189_1: branch s_189_0 b191 b190
        if s_189_0 {
            return block_191(state, tracer, fn_state);
        } else {
            return block_190(state, tracer, fn_state);
        };
    }
    fn block_190<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_190_0: const #6u : u8
        let s_190_0: u8 = 6;
        // C s_190_1: cast zx s_190_0 -> bv
        let s_190_1: Bits = Bits::new(s_190_0 as u128, 8u16);
        // C s_190_2: cast zx s_190_1 -> i
        let s_190_2: i128 = (s_190_1.value() as i128);
        // C s_190_3: cast reint s_190_2 -> i64
        let s_190_3: i64 = (s_190_2 as i64);
        // C s_190_4: cast zx s_190_3 -> i
        let s_190_4: i128 = (i128::try_from(s_190_3).unwrap());
        // C s_190_5: const #440u : u32
        let s_190_5: u32 = 440;
        // D s_190_6: read-reg s_190_5:u8
        let s_190_6: u8 = {
            let value = state.read_register::<u8>(s_190_5 as isize);
            tracer.read_register(s_190_5 as isize, value);
            value
        };
        // D s_190_7: call AArch64_AArch32SystemAccessTrap(s_190_6, s_190_4)
        let s_190_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_190_6,
            s_190_4,
        );
        // N s_190_8: return
        return;
    }
    fn block_191<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_191_0: const #6u : u8
        let s_191_0: u8 = 6;
        // C s_191_1: cast zx s_191_0 -> bv
        let s_191_1: Bits = Bits::new(s_191_0 as u128, 8u16);
        // C s_191_2: cast zx s_191_1 -> i
        let s_191_2: i128 = (s_191_1.value() as i128);
        // C s_191_3: cast reint s_191_2 -> i64
        let s_191_3: i64 = (s_191_2 as i64);
        // C s_191_4: cast zx s_191_3 -> i
        let s_191_4: i128 = (i128::try_from(s_191_3).unwrap());
        // C s_191_5: const #432u : u32
        let s_191_5: u32 = 432;
        // D s_191_6: read-reg s_191_5:u8
        let s_191_6: u8 = {
            let value = state.read_register::<u8>(s_191_5 as isize);
            tracer.read_register(s_191_5 as isize, value);
            value
        };
        // D s_191_7: call AArch64_AArch32SystemAccessTrap(s_191_6, s_191_4)
        let s_191_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_191_6,
            s_191_4,
        );
        // N s_191_8: return
        return;
    }
    fn block_192<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_192_0: const #102552u : u32
        let s_192_0: u32 = 102552;
        // D s_192_1: read-reg s_192_0:struct
        let s_192_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_192_0 as isize);
            tracer.read_register(s_192_0 as isize, value);
            value
        };
        // D s_192_2: call _get_HCR_EL2_Type_TGE(s_192_1)
        let s_192_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_192_1);
        // D s_192_3: cast zx s_192_2 -> bv
        let s_192_3: Bits = Bits::new(s_192_2 as u128, 1u16);
        // C s_192_4: const #1u : u8
        let s_192_4: bool = true;
        // C s_192_5: cast zx s_192_4 -> bv
        let s_192_5: Bits = Bits::new(s_192_4 as u128, 1u16);
        // D s_192_6: cmp-eq s_192_3 s_192_5
        let s_192_6: bool = ((s_192_3) == (s_192_5));
        // D s_192_7: write-var gs#137035 <= s_192_6
        fn_state.gs_137035 = s_192_6;
        // N s_192_8: jump b189
        return block_189(state, tracer, fn_state);
    }
    fn block_193<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_193_0: const #432u : u32
        let s_193_0: u32 = 432;
        // D s_193_1: read-reg s_193_0:u8
        let s_193_1: u8 = {
            let value = state.read_register::<u8>(s_193_0 as isize);
            tracer.read_register(s_193_0 as isize, value);
            value
        };
        // D s_193_2: call ELUsingAArch32(s_193_1)
        let s_193_2: bool = ELUsingAArch32(state, tracer, s_193_1);
        // D s_193_3: not s_193_2
        let s_193_3: bool = !s_193_2;
        // D s_193_4: write-var gs#137034 <= s_193_3
        fn_state.gs_137034 = s_193_3;
        // N s_193_5: jump b187
        return block_187(state, tracer, fn_state);
    }
    fn block_194<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_194_0: read-var __MDSCR_EL1_TDCC:u8
        let s_194_0: bool = fn_state.u__MDSCR_EL1_TDCC;
        // D s_194_1: cast zx s_194_0 -> bv
        let s_194_1: Bits = Bits::new(s_194_0 as u128, 1u16);
        // C s_194_2: const #1u : u8
        let s_194_2: bool = true;
        // C s_194_3: cast zx s_194_2 -> bv
        let s_194_3: Bits = Bits::new(s_194_2 as u128, 1u16);
        // D s_194_4: cmp-eq s_194_1 s_194_3
        let s_194_4: bool = ((s_194_1) == (s_194_3));
        // D s_194_5: write-var gs#137011 <= s_194_4
        fn_state.gs_137011 = s_194_4;
        // N s_194_6: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_195<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_195_0: const #4s : i64
        let s_195_0: i64 = 4;
        // D s_195_1: read-var address:u32
        let s_195_1: u32 = fn_state.address;
        // D s_195_2: call MemA_read(s_195_1, s_195_0)
        let s_195_2: Bits = MemA_read(state, tracer, s_195_1, s_195_0);
        // D s_195_3: write-var gs#670444 <= s_195_2
        fn_state.gs_670444 = s_195_2;
        // N s_195_4: jump b196
        return block_196(state, tracer, fn_state);
    }
    fn block_196<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_196_0: read-var gs#670444:bv
        let s_196_0: Bits = fn_state.gs_670444;
        // D s_196_1: cast reint s_196_0 -> u32
        let s_196_1: u32 = (s_196_0.value() as u32);
        // D s_196_2: call Mk_DBGDTRTXint_Type(s_196_1)
        let s_196_2: ProductType700c18a878c5601b = Mk_DBGDTRTXint_Type(
            state,
            tracer,
            s_196_1,
        );
        // D s_196_3: call DBGDTRTXint_write(s_196_2)
        let s_196_3: () = DBGDTRTXint_write(state, tracer, s_196_2);
        // N s_196_4: return
        return;
    }
}
