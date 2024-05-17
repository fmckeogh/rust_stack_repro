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
use u_get_HCR_EL2_Type_TGE::*;
use DBGDSCRext_read::*;
use u_get_HDCR_Type_TDCC::*;
use Halted::*;
use HDCR_read::*;
use u_get_MDCR_EL2_Type_TDCC::*;
use DBGDTRRXint_read::*;
use u_get_HDCR_Type_TDE::*;
use u_get_MDCR_EL3_Type_TDCC::*;
use u_get_MDCR_EL2_Type_TDE::*;
use AArch64_AArch32SystemAccessTrap::*;
use u_get_HDCR_Type_TDA::*;
use HCR_read::*;
use u_get_HCR_Type_TGE::*;
use u_get_MDCR_EL3_Type_TDA::*;
use u_get_DBGDSCRext_Type_UDCCdis::*;
use ELUsingAArch32::*;
use u_get_SDCR_Type_TDCC::*;
use u_get_MDSCR_EL1_Type_TDCC::*;
use EL2Enabled::*;
use MemA_set::*;
use AArch32_TakeMonitorTrapException::*;
use AArch32_TakeHypTrapException::*;
use common::*;
pub fn DBGDTRRXint_STC_521dc9133ebece9c<T: Tracer>(
    state: &mut State,
    tracer: &T,
    coproc: u8,
    CRd: u8,
    address: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        u__DBGDSCRext_UDCCdis: bool,
        gs_136964: bool,
        gs_136958: bool,
        gs_136938: bool,
        gs_136977: bool,
        gs_136959: bool,
        u__SDCR_TDCC: bool,
        gs_136946: bool,
        gs_136937: bool,
        gs_136980: bool,
        ga_238555: ProductType700c18a878c5601b,
        ga_238685: ProductType700c18a878c5601b,
        ga_238627: ProductType700c18a878c5601b,
        gs_136934: bool,
        gs_136973: bool,
        gs_136940: bool,
        gs_136968: bool,
        gs_136966: bool,
        gs_136955: bool,
        ga_238690: ProductType700c18a878c5601b,
        gs_136939: bool,
        gs_136971: bool,
        u__PSTATE_EL: u8,
        gs_136951: bool,
        gs_136945: bool,
        gs_136972: bool,
        ga_238669: ProductType700c18a878c5601b,
        gs_136967: bool,
        gs_136961: bool,
        gs_136952: bool,
        gs_136970: bool,
        gs_136963: bool,
        gs_136954: bool,
        gs_136969: bool,
        u__MDCR_EL3_TDA: bool,
        gs_136943: bool,
        gs_136978: bool,
        gs_136962: bool,
        gs_136960: bool,
        gs_136981: bool,
        gs_136965: bool,
        gs_136949: bool,
        u__MDCR_EL3_TDCC: bool,
        gs_136950: bool,
        gs_136948: bool,
        gs_136979: bool,
        gs_136947: bool,
        gs_136956: bool,
        u__PSTATE_M: u8,
        gs_136974: bool,
        gs_136944: bool,
        gs_136953: bool,
        gs_136975: bool,
        u__MDSCR_EL1_TDCC: bool,
        u__MDCR_EL2_TDCC: bool,
        gs_136941: bool,
        u__HDCR_TDCC: bool,
        gs_136982: bool,
        gs_136936: bool,
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
        // N s_0_36: branch s_0_35 b191 b1
        if s_0_35 {
            return block_191(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b94 b2
        if s_1_5 {
            return block_94(state, tracer, fn_state);
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
        // N s_2_6: branch s_2_5 b37 b3
        if s_2_5 {
            return block_37(state, tracer, fn_state);
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
        // N s_3_6: branch s_3_5 b12 b4
        if s_3_5 {
            return block_12(state, tracer, fn_state);
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
        // N s_6_6: branch s_6_5 b11 b7
        if s_6_5 {
            return block_11(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#136934 <= s_7_0
        fn_state.gs_136934 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#136934:u8
        let s_8_0: bool = fn_state.gs_136934;
        // N s_8_1: branch s_8_0 b10 b9
        if s_8_0 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call DBGDTRRXint_read(s_9_0)
        let s_9_1: ProductType700c18a878c5601b = DBGDTRRXint_read(state, tracer, s_9_0);
        // D s_9_2: write-var ga#238690 <= s_9_1
        fn_state.ga_238690 = s_9_1;
        // D s_9_3: read-var ga#238690.0:struct
        let s_9_3: u32 = fn_state.ga_238690._0;
        // C s_9_4: const #4s : i
        let s_9_4: i128 = 4;
        // D s_9_5: cast zx s_9_3 -> bv
        let s_9_5: Bits = Bits::new(s_9_3 as u128, 32u16);
        // D s_9_6: read-var address:u32
        let s_9_6: u32 = fn_state.address;
        // D s_9_7: call MemA_set(s_9_6, s_9_4, s_9_5)
        let s_9_7: () = MemA_set(state, tracer, s_9_6, s_9_4, s_9_5);
        // N s_9_8: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call AArch32_TakeMonitorTrapException(s_10_0)
        let s_10_1: () = AArch32_TakeMonitorTrapException(state, tracer, s_10_0);
        // N s_10_2: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var __SDCR_TDCC:u8
        let s_11_0: bool = fn_state.u__SDCR_TDCC;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 1u16);
        // C s_11_2: const #1u : u8
        let s_11_2: bool = true;
        // C s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 1u16);
        // D s_11_4: cmp-eq s_11_1 s_11_3
        let s_11_4: bool = ((s_11_1) == (s_11_3));
        // D s_11_5: write-var gs#136934 <= s_11_4
        fn_state.gs_136934 = s_11_4;
        // N s_11_6: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #424u : u32
        let s_12_0: u32 = 424;
        // D s_12_1: read-reg s_12_0:u8
        let s_12_1: u8 = {
            let value = state.read_register::<u8>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // C s_12_2: const #2u : u8
        let s_12_2: u8 = 2;
        // D s_12_3: cmp-lt s_12_1 s_12_2
        let s_12_3: bool = ((s_12_1) < (s_12_2));
        // N s_12_4: branch s_12_3 b36 b13
        if s_12_3 {
            return block_36(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#136936 <= s_13_0
        fn_state.gs_136936 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#136936:u8
        let s_14_0: bool = fn_state.gs_136936;
        // N s_14_1: branch s_14_0 b35 b15
        if s_14_0 {
            return block_35(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#136937 <= s_15_0
        fn_state.gs_136937 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#136937:u8
        let s_16_0: bool = fn_state.gs_136937;
        // N s_16_1: branch s_16_0 b34 b17
        if s_16_0 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #424u : u32
        let s_17_0: u32 = 424;
        // D s_17_1: read-reg s_17_0:u8
        let s_17_1: u8 = {
            let value = state.read_register::<u8>(s_17_0 as isize);
            tracer.read_register(s_17_0 as isize, value);
            value
        };
        // C s_17_2: const #2u : u8
        let s_17_2: u8 = 2;
        // D s_17_3: cmp-lt s_17_1 s_17_2
        let s_17_3: bool = ((s_17_1) < (s_17_2));
        // N s_17_4: branch s_17_3 b33 b18
        if s_17_3 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #0u : u8
        let s_18_0: bool = false;
        // D s_18_1: write-var gs#136938 <= s_18_0
        fn_state.gs_136938 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#136938:u8
        let s_19_0: bool = fn_state.gs_136938;
        // N s_19_1: branch s_19_0 b32 b20
        if s_19_0 {
            return block_32(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#136939 <= s_20_0
        fn_state.gs_136939 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#136939:u8
        let s_21_0: bool = fn_state.gs_136939;
        // N s_21_1: branch s_21_0 b31 b22
        if s_21_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #424u : u32
        let s_22_0: u32 = 424;
        // D s_22_1: read-reg s_22_0:u8
        let s_22_1: u8 = {
            let value = state.read_register::<u8>(s_22_0 as isize);
            tracer.read_register(s_22_0 as isize, value);
            value
        };
        // C s_22_2: const #2u : u8
        let s_22_2: u8 = 2;
        // D s_22_3: cmp-lt s_22_1 s_22_2
        let s_22_3: bool = ((s_22_1) < (s_22_2));
        // N s_22_4: branch s_22_3 b30 b23
        if s_22_3 {
            return block_30(state, tracer, fn_state);
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
        // D s_23_1: write-var gs#136940 <= s_23_0
        fn_state.gs_136940 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#136940:u8
        let s_24_0: bool = fn_state.gs_136940;
        // N s_24_1: branch s_24_0 b29 b25
        if s_24_0 {
            return block_29(state, tracer, fn_state);
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
        // D s_25_1: write-var gs#136941 <= s_25_0
        fn_state.gs_136941 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#136941:u8
        let s_26_0: bool = fn_state.gs_136941;
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
        // C s_27_0: const #() : ()
        let s_27_0: () = ();
        // S s_27_1: call DBGDTRRXint_read(s_27_0)
        let s_27_1: ProductType700c18a878c5601b = DBGDTRRXint_read(
            state,
            tracer,
            s_27_0,
        );
        // D s_27_2: write-var ga#238685 <= s_27_1
        fn_state.ga_238685 = s_27_1;
        // D s_27_3: read-var ga#238685.0:struct
        let s_27_3: u32 = fn_state.ga_238685._0;
        // C s_27_4: const #4s : i
        let s_27_4: i128 = 4;
        // D s_27_5: cast zx s_27_3 -> bv
        let s_27_5: Bits = Bits::new(s_27_3 as u128, 32u16);
        // D s_27_6: read-var address:u32
        let s_27_6: u32 = fn_state.address;
        // D s_27_7: call MemA_set(s_27_6, s_27_4, s_27_5)
        let s_27_7: () = MemA_set(state, tracer, s_27_6, s_27_4, s_27_5);
        // N s_27_8: return
        return;
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #6u : u8
        let s_28_0: u8 = 6;
        // C s_28_1: cast zx s_28_0 -> bv
        let s_28_1: Bits = Bits::new(s_28_0 as u128, 8u16);
        // C s_28_2: cast zx s_28_1 -> i
        let s_28_2: i128 = (s_28_1.value() as i128);
        // C s_28_3: cast reint s_28_2 -> i64
        let s_28_3: i64 = (s_28_2 as i64);
        // C s_28_4: cast zx s_28_3 -> i
        let s_28_4: i128 = (i128::try_from(s_28_3).unwrap());
        // C s_28_5: const #424u : u32
        let s_28_5: u32 = 424;
        // D s_28_6: read-reg s_28_5:u8
        let s_28_6: u8 = {
            let value = state.read_register::<u8>(s_28_5 as isize);
            tracer.read_register(s_28_5 as isize, value);
            value
        };
        // D s_28_7: call AArch64_AArch32SystemAccessTrap(s_28_6, s_28_4)
        let s_28_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_28_6, s_28_4);
        // N s_28_8: return
        return;
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var __MDCR_EL3_TDA:u8
        let s_29_0: bool = fn_state.u__MDCR_EL3_TDA;
        // D s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 1u16);
        // C s_29_2: const #1u : u8
        let s_29_2: bool = true;
        // C s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 1u16);
        // D s_29_4: cmp-eq s_29_1 s_29_3
        let s_29_4: bool = ((s_29_1) == (s_29_3));
        // D s_29_5: write-var gs#136941 <= s_29_4
        fn_state.gs_136941 = s_29_4;
        // N s_29_6: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #424u : u32
        let s_30_0: u32 = 424;
        // D s_30_1: read-reg s_30_0:u8
        let s_30_1: u8 = {
            let value = state.read_register::<u8>(s_30_0 as isize);
            tracer.read_register(s_30_0 as isize, value);
            value
        };
        // D s_30_2: call ELUsingAArch32(s_30_1)
        let s_30_2: bool = ELUsingAArch32(state, tracer, s_30_1);
        // D s_30_3: not s_30_2
        let s_30_3: bool = !s_30_2;
        // D s_30_4: write-var gs#136940 <= s_30_3
        fn_state.gs_136940 = s_30_3;
        // N s_30_5: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #() : ()
        let s_31_0: () = ();
        // S s_31_1: call AArch32_TakeMonitorTrapException(s_31_0)
        let s_31_1: () = AArch32_TakeMonitorTrapException(state, tracer, s_31_0);
        // N s_31_2: return
        return;
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var __SDCR_TDCC:u8
        let s_32_0: bool = fn_state.u__SDCR_TDCC;
        // D s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 1u16);
        // C s_32_2: const #1u : u8
        let s_32_2: bool = true;
        // C s_32_3: cast zx s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 1u16);
        // D s_32_4: cmp-eq s_32_1 s_32_3
        let s_32_4: bool = ((s_32_1) == (s_32_3));
        // D s_32_5: write-var gs#136939 <= s_32_4
        fn_state.gs_136939 = s_32_4;
        // N s_32_6: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #424u : u32
        let s_33_0: u32 = 424;
        // D s_33_1: read-reg s_33_0:u8
        let s_33_1: u8 = {
            let value = state.read_register::<u8>(s_33_0 as isize);
            tracer.read_register(s_33_0 as isize, value);
            value
        };
        // D s_33_2: call ELUsingAArch32(s_33_1)
        let s_33_2: bool = ELUsingAArch32(state, tracer, s_33_1);
        // D s_33_3: write-var gs#136938 <= s_33_2
        fn_state.gs_136938 = s_33_2;
        // N s_33_4: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #6u : u8
        let s_34_0: u8 = 6;
        // C s_34_1: cast zx s_34_0 -> bv
        let s_34_1: Bits = Bits::new(s_34_0 as u128, 8u16);
        // C s_34_2: cast zx s_34_1 -> i
        let s_34_2: i128 = (s_34_1.value() as i128);
        // C s_34_3: cast reint s_34_2 -> i64
        let s_34_3: i64 = (s_34_2 as i64);
        // C s_34_4: cast zx s_34_3 -> i
        let s_34_4: i128 = (i128::try_from(s_34_3).unwrap());
        // C s_34_5: const #424u : u32
        let s_34_5: u32 = 424;
        // D s_34_6: read-reg s_34_5:u8
        let s_34_6: u8 = {
            let value = state.read_register::<u8>(s_34_5 as isize);
            tracer.read_register(s_34_5 as isize, value);
            value
        };
        // D s_34_7: call AArch64_AArch32SystemAccessTrap(s_34_6, s_34_4)
        let s_34_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_34_6, s_34_4);
        // N s_34_8: return
        return;
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var __MDCR_EL3_TDCC:u8
        let s_35_0: bool = fn_state.u__MDCR_EL3_TDCC;
        // D s_35_1: cast zx s_35_0 -> bv
        let s_35_1: Bits = Bits::new(s_35_0 as u128, 1u16);
        // C s_35_2: const #1u : u8
        let s_35_2: bool = true;
        // C s_35_3: cast zx s_35_2 -> bv
        let s_35_3: Bits = Bits::new(s_35_2 as u128, 1u16);
        // D s_35_4: cmp-eq s_35_1 s_35_3
        let s_35_4: bool = ((s_35_1) == (s_35_3));
        // D s_35_5: write-var gs#136937 <= s_35_4
        fn_state.gs_136937 = s_35_4;
        // N s_35_6: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #424u : u32
        let s_36_0: u32 = 424;
        // D s_36_1: read-reg s_36_0:u8
        let s_36_1: u8 = {
            let value = state.read_register::<u8>(s_36_0 as isize);
            tracer.read_register(s_36_0 as isize, value);
            value
        };
        // D s_36_2: call ELUsingAArch32(s_36_1)
        let s_36_2: bool = ELUsingAArch32(state, tracer, s_36_1);
        // D s_36_3: not s_36_2
        let s_36_3: bool = !s_36_2;
        // D s_36_4: write-var gs#136936 <= s_36_3
        fn_state.gs_136936 = s_36_3;
        // N s_36_5: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #() : ()
        let s_37_0: () = ();
        // S s_37_1: call EL2Enabled(s_37_0)
        let s_37_1: bool = EL2Enabled(state, tracer, s_37_0);
        // N s_37_2: branch s_37_1 b93 b38
        if s_37_1 {
            return block_93(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #0u : u8
        let s_38_0: bool = false;
        // D s_38_1: write-var gs#136943 <= s_38_0
        fn_state.gs_136943 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#136943:u8
        let s_39_0: bool = fn_state.gs_136943;
        // N s_39_1: branch s_39_0 b92 b40
        if s_39_0 {
            return block_92(state, tracer, fn_state);
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
        // D s_40_1: write-var gs#136944 <= s_40_0
        fn_state.gs_136944 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#136944:u8
        let s_41_0: bool = fn_state.gs_136944;
        // N s_41_1: branch s_41_0 b91 b42
        if s_41_0 {
            return block_91(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #() : ()
        let s_42_0: () = ();
        // S s_42_1: call EL2Enabled(s_42_0)
        let s_42_1: bool = EL2Enabled(state, tracer, s_42_0);
        // N s_42_2: branch s_42_1 b90 b43
        if s_42_1 {
            return block_90(state, tracer, fn_state);
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
        // D s_43_1: write-var gs#136945 <= s_43_0
        fn_state.gs_136945 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#136945:u8
        let s_44_0: bool = fn_state.gs_136945;
        // N s_44_1: branch s_44_0 b89 b45
        if s_44_0 {
            return block_89(state, tracer, fn_state);
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
        // D s_45_1: write-var gs#136946 <= s_45_0
        fn_state.gs_136946 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#136946:u8
        let s_46_0: bool = fn_state.gs_136946;
        // N s_46_1: branch s_46_0 b88 b47
        if s_46_0 {
            return block_88(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #() : ()
        let s_47_0: () = ();
        // S s_47_1: call EL2Enabled(s_47_0)
        let s_47_1: bool = EL2Enabled(state, tracer, s_47_0);
        // N s_47_2: branch s_47_1 b87 b48
        if s_47_1 {
            return block_87(state, tracer, fn_state);
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
        // D s_48_1: write-var gs#136947 <= s_48_0
        fn_state.gs_136947 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#136947:u8
        let s_49_0: bool = fn_state.gs_136947;
        // N s_49_1: branch s_49_0 b86 b50
        if s_49_0 {
            return block_86(state, tracer, fn_state);
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
        // D s_50_1: write-var gs#136948 <= s_50_0
        fn_state.gs_136948 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#136948:u8
        let s_51_0: bool = fn_state.gs_136948;
        // N s_51_1: branch s_51_0 b85 b52
        if s_51_0 {
            return block_85(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #() : ()
        let s_52_0: () = ();
        // S s_52_1: call EL2Enabled(s_52_0)
        let s_52_1: bool = EL2Enabled(state, tracer, s_52_0);
        // N s_52_2: branch s_52_1 b84 b53
        if s_52_1 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #0u : u8
        let s_53_0: bool = false;
        // D s_53_1: write-var gs#136949 <= s_53_0
        fn_state.gs_136949 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var gs#136949:u8
        let s_54_0: bool = fn_state.gs_136949;
        // N s_54_1: branch s_54_0 b83 b55
        if s_54_0 {
            return block_83(state, tracer, fn_state);
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
        // D s_55_1: write-var gs#136950 <= s_55_0
        fn_state.gs_136950 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var gs#136950:u8
        let s_56_0: bool = fn_state.gs_136950;
        // N s_56_1: branch s_56_0 b82 b57
        if s_56_0 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #424u : u32
        let s_57_0: u32 = 424;
        // D s_57_1: read-reg s_57_0:u8
        let s_57_1: u8 = {
            let value = state.read_register::<u8>(s_57_0 as isize);
            tracer.read_register(s_57_0 as isize, value);
            value
        };
        // C s_57_2: const #2u : u8
        let s_57_2: u8 = 2;
        // D s_57_3: cmp-lt s_57_1 s_57_2
        let s_57_3: bool = ((s_57_1) < (s_57_2));
        // N s_57_4: branch s_57_3 b81 b58
        if s_57_3 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #0u : u8
        let s_58_0: bool = false;
        // D s_58_1: write-var gs#136951 <= s_58_0
        fn_state.gs_136951 = s_58_0;
        // N s_58_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var gs#136951:u8
        let s_59_0: bool = fn_state.gs_136951;
        // N s_59_1: branch s_59_0 b80 b60
        if s_59_0 {
            return block_80(state, tracer, fn_state);
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
        // D s_60_1: write-var gs#136952 <= s_60_0
        fn_state.gs_136952 = s_60_0;
        // N s_60_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var gs#136952:u8
        let s_61_0: bool = fn_state.gs_136952;
        // N s_61_1: branch s_61_0 b79 b62
        if s_61_0 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #424u : u32
        let s_62_0: u32 = 424;
        // D s_62_1: read-reg s_62_0:u8
        let s_62_1: u8 = {
            let value = state.read_register::<u8>(s_62_0 as isize);
            tracer.read_register(s_62_0 as isize, value);
            value
        };
        // C s_62_2: const #2u : u8
        let s_62_2: u8 = 2;
        // D s_62_3: cmp-lt s_62_1 s_62_2
        let s_62_3: bool = ((s_62_1) < (s_62_2));
        // N s_62_4: branch s_62_3 b78 b63
        if s_62_3 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #0u : u8
        let s_63_0: bool = false;
        // D s_63_1: write-var gs#136953 <= s_63_0
        fn_state.gs_136953 = s_63_0;
        // N s_63_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var gs#136953:u8
        let s_64_0: bool = fn_state.gs_136953;
        // N s_64_1: branch s_64_0 b77 b65
        if s_64_0 {
            return block_77(state, tracer, fn_state);
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
        // D s_65_1: write-var gs#136954 <= s_65_0
        fn_state.gs_136954 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var gs#136954:u8
        let s_66_0: bool = fn_state.gs_136954;
        // N s_66_1: branch s_66_0 b76 b67
        if s_66_0 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #424u : u32
        let s_67_0: u32 = 424;
        // D s_67_1: read-reg s_67_0:u8
        let s_67_1: u8 = {
            let value = state.read_register::<u8>(s_67_0 as isize);
            tracer.read_register(s_67_0 as isize, value);
            value
        };
        // C s_67_2: const #2u : u8
        let s_67_2: u8 = 2;
        // D s_67_3: cmp-lt s_67_1 s_67_2
        let s_67_3: bool = ((s_67_1) < (s_67_2));
        // N s_67_4: branch s_67_3 b75 b68
        if s_67_3 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_68(state, tracer, fn_state);
        };
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #0u : u8
        let s_68_0: bool = false;
        // D s_68_1: write-var gs#136955 <= s_68_0
        fn_state.gs_136955 = s_68_0;
        // N s_68_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var gs#136955:u8
        let s_69_0: bool = fn_state.gs_136955;
        // N s_69_1: branch s_69_0 b74 b70
        if s_69_0 {
            return block_74(state, tracer, fn_state);
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
        // D s_70_1: write-var gs#136956 <= s_70_0
        fn_state.gs_136956 = s_70_0;
        // N s_70_2: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var gs#136956:u8
        let s_71_0: bool = fn_state.gs_136956;
        // N s_71_1: branch s_71_0 b73 b72
        if s_71_0 {
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
        // C s_72_0: const #() : ()
        let s_72_0: () = ();
        // S s_72_1: call DBGDTRRXint_read(s_72_0)
        let s_72_1: ProductType700c18a878c5601b = DBGDTRRXint_read(
            state,
            tracer,
            s_72_0,
        );
        // D s_72_2: write-var ga#238669 <= s_72_1
        fn_state.ga_238669 = s_72_1;
        // D s_72_3: read-var ga#238669.0:struct
        let s_72_3: u32 = fn_state.ga_238669._0;
        // C s_72_4: const #4s : i
        let s_72_4: i128 = 4;
        // D s_72_5: cast zx s_72_3 -> bv
        let s_72_5: Bits = Bits::new(s_72_3 as u128, 32u16);
        // D s_72_6: read-var address:u32
        let s_72_6: u32 = fn_state.address;
        // D s_72_7: call MemA_set(s_72_6, s_72_4, s_72_5)
        let s_72_7: () = MemA_set(state, tracer, s_72_6, s_72_4, s_72_5);
        // N s_72_8: return
        return;
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #6u : u8
        let s_73_0: u8 = 6;
        // C s_73_1: cast zx s_73_0 -> bv
        let s_73_1: Bits = Bits::new(s_73_0 as u128, 8u16);
        // C s_73_2: cast zx s_73_1 -> i
        let s_73_2: i128 = (s_73_1.value() as i128);
        // C s_73_3: cast reint s_73_2 -> i64
        let s_73_3: i64 = (s_73_2 as i64);
        // C s_73_4: cast zx s_73_3 -> i
        let s_73_4: i128 = (i128::try_from(s_73_3).unwrap());
        // C s_73_5: const #424u : u32
        let s_73_5: u32 = 424;
        // D s_73_6: read-reg s_73_5:u8
        let s_73_6: u8 = {
            let value = state.read_register::<u8>(s_73_5 as isize);
            tracer.read_register(s_73_5 as isize, value);
            value
        };
        // D s_73_7: call AArch64_AArch32SystemAccessTrap(s_73_6, s_73_4)
        let s_73_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_73_6, s_73_4);
        // N s_73_8: return
        return;
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_74_0: read-var __MDCR_EL3_TDA:u8
        let s_74_0: bool = fn_state.u__MDCR_EL3_TDA;
        // D s_74_1: cast zx s_74_0 -> bv
        let s_74_1: Bits = Bits::new(s_74_0 as u128, 1u16);
        // C s_74_2: const #1u : u8
        let s_74_2: bool = true;
        // C s_74_3: cast zx s_74_2 -> bv
        let s_74_3: Bits = Bits::new(s_74_2 as u128, 1u16);
        // D s_74_4: cmp-eq s_74_1 s_74_3
        let s_74_4: bool = ((s_74_1) == (s_74_3));
        // D s_74_5: write-var gs#136956 <= s_74_4
        fn_state.gs_136956 = s_74_4;
        // N s_74_6: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #424u : u32
        let s_75_0: u32 = 424;
        // D s_75_1: read-reg s_75_0:u8
        let s_75_1: u8 = {
            let value = state.read_register::<u8>(s_75_0 as isize);
            tracer.read_register(s_75_0 as isize, value);
            value
        };
        // D s_75_2: call ELUsingAArch32(s_75_1)
        let s_75_2: bool = ELUsingAArch32(state, tracer, s_75_1);
        // D s_75_3: not s_75_2
        let s_75_3: bool = !s_75_2;
        // D s_75_4: write-var gs#136955 <= s_75_3
        fn_state.gs_136955 = s_75_3;
        // N s_75_5: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #() : ()
        let s_76_0: () = ();
        // S s_76_1: call AArch32_TakeMonitorTrapException(s_76_0)
        let s_76_1: () = AArch32_TakeMonitorTrapException(state, tracer, s_76_0);
        // N s_76_2: return
        return;
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var __SDCR_TDCC:u8
        let s_77_0: bool = fn_state.u__SDCR_TDCC;
        // D s_77_1: cast zx s_77_0 -> bv
        let s_77_1: Bits = Bits::new(s_77_0 as u128, 1u16);
        // C s_77_2: const #1u : u8
        let s_77_2: bool = true;
        // C s_77_3: cast zx s_77_2 -> bv
        let s_77_3: Bits = Bits::new(s_77_2 as u128, 1u16);
        // D s_77_4: cmp-eq s_77_1 s_77_3
        let s_77_4: bool = ((s_77_1) == (s_77_3));
        // D s_77_5: write-var gs#136954 <= s_77_4
        fn_state.gs_136954 = s_77_4;
        // N s_77_6: jump b66
        return block_66(state, tracer, fn_state);
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
        // D s_78_3: write-var gs#136953 <= s_78_2
        fn_state.gs_136953 = s_78_2;
        // N s_78_4: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #6u : u8
        let s_79_0: u8 = 6;
        // C s_79_1: cast zx s_79_0 -> bv
        let s_79_1: Bits = Bits::new(s_79_0 as u128, 8u16);
        // C s_79_2: cast zx s_79_1 -> i
        let s_79_2: i128 = (s_79_1.value() as i128);
        // C s_79_3: cast reint s_79_2 -> i64
        let s_79_3: i64 = (s_79_2 as i64);
        // C s_79_4: cast zx s_79_3 -> i
        let s_79_4: i128 = (i128::try_from(s_79_3).unwrap());
        // C s_79_5: const #424u : u32
        let s_79_5: u32 = 424;
        // D s_79_6: read-reg s_79_5:u8
        let s_79_6: u8 = {
            let value = state.read_register::<u8>(s_79_5 as isize);
            tracer.read_register(s_79_5 as isize, value);
            value
        };
        // D s_79_7: call AArch64_AArch32SystemAccessTrap(s_79_6, s_79_4)
        let s_79_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_79_6, s_79_4);
        // N s_79_8: return
        return;
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_80_0: read-var __MDCR_EL3_TDCC:u8
        let s_80_0: bool = fn_state.u__MDCR_EL3_TDCC;
        // D s_80_1: cast zx s_80_0 -> bv
        let s_80_1: Bits = Bits::new(s_80_0 as u128, 1u16);
        // C s_80_2: const #1u : u8
        let s_80_2: bool = true;
        // C s_80_3: cast zx s_80_2 -> bv
        let s_80_3: Bits = Bits::new(s_80_2 as u128, 1u16);
        // D s_80_4: cmp-eq s_80_1 s_80_3
        let s_80_4: bool = ((s_80_1) == (s_80_3));
        // D s_80_5: write-var gs#136952 <= s_80_4
        fn_state.gs_136952 = s_80_4;
        // N s_80_6: jump b61
        return block_61(state, tracer, fn_state);
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
        // D s_81_3: not s_81_2
        let s_81_3: bool = !s_81_2;
        // D s_81_4: write-var gs#136951 <= s_81_3
        fn_state.gs_136951 = s_81_3;
        // N s_81_5: jump b59
        return block_59(state, tracer, fn_state);
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
        // S s_82_5: call AArch32_TakeHypTrapException(s_82_4)
        let s_82_5: () = AArch32_TakeHypTrapException(state, tracer, s_82_4);
        // N s_82_6: return
        return;
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #() : ()
        let s_83_0: () = ();
        // S s_83_1: call HDCR_read(s_83_0)
        let s_83_1: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_83_0);
        // S s_83_2: call _get_HDCR_Type_TDE(s_83_1)
        let s_83_2: bool = u_get_HDCR_Type_TDE(state, tracer, s_83_1);
        // C s_83_3: const #() : ()
        let s_83_3: () = ();
        // S s_83_4: call HDCR_read(s_83_3)
        let s_83_4: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_83_3);
        // S s_83_5: call _get_HDCR_Type_TDA(s_83_4)
        let s_83_5: bool = u_get_HDCR_Type_TDA(state, tracer, s_83_4);
        // S s_83_6: cast zx s_83_2 -> bv
        let s_83_6: Bits = Bits::new(s_83_2 as u128, 1u16);
        // S s_83_7: cast zx s_83_5 -> bv
        let s_83_7: Bits = Bits::new(s_83_5 as u128, 1u16);
        // S s_83_8: cast reint s_83_6 -> u128
        let s_83_8: u128 = (s_83_6.value() as u128);
        // D s_83_9: size-of s_83_6
        let s_83_9: u16 = s_83_6.length();
        // S s_83_10: cast reint s_83_7 -> u128
        let s_83_10: u128 = (s_83_7.value() as u128);
        // D s_83_11: size-of s_83_7
        let s_83_11: u16 = s_83_7.length();
        // D s_83_12: lsl s_83_8 s_83_11
        let s_83_12: u128 = s_83_8 << s_83_11;
        // D s_83_13: or s_83_12 s_83_10
        let s_83_13: u128 = ((s_83_12) | (s_83_10));
        // D s_83_14: add s_83_9 s_83_11
        let s_83_14: u16 = (s_83_9 + s_83_11);
        // D s_83_15: create-bits s_83_13 s_83_14
        let s_83_15: Bits = Bits::new(s_83_13, s_83_14);
        // D s_83_16: cast reint s_83_15 -> u8
        let s_83_16: u8 = (s_83_15.value() as u8);
        // D s_83_17: cast zx s_83_16 -> bv
        let s_83_17: Bits = Bits::new(s_83_16 as u128, 2u16);
        // C s_83_18: const #0u : u8
        let s_83_18: u8 = 0;
        // C s_83_19: cast zx s_83_18 -> bv
        let s_83_19: Bits = Bits::new(s_83_18 as u128, 2u16);
        // D s_83_20: cmp-ne s_83_17 s_83_19
        let s_83_20: bool = ((s_83_17) != (s_83_19));
        // D s_83_21: write-var gs#136950 <= s_83_20
        fn_state.gs_136950 = s_83_20;
        // N s_83_22: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #432u : u32
        let s_84_0: u32 = 432;
        // D s_84_1: read-reg s_84_0:u8
        let s_84_1: u8 = {
            let value = state.read_register::<u8>(s_84_0 as isize);
            tracer.read_register(s_84_0 as isize, value);
            value
        };
        // D s_84_2: call ELUsingAArch32(s_84_1)
        let s_84_2: bool = ELUsingAArch32(state, tracer, s_84_1);
        // D s_84_3: write-var gs#136949 <= s_84_2
        fn_state.gs_136949 = s_84_2;
        // N s_84_4: jump b54
        return block_54(state, tracer, fn_state);
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
        // C s_85_5: const #432u : u32
        let s_85_5: u32 = 432;
        // D s_85_6: read-reg s_85_5:u8
        let s_85_6: u8 = {
            let value = state.read_register::<u8>(s_85_5 as isize);
            tracer.read_register(s_85_5 as isize, value);
            value
        };
        // D s_85_7: call AArch64_AArch32SystemAccessTrap(s_85_6, s_85_4)
        let s_85_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_85_6, s_85_4);
        // N s_85_8: return
        return;
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #104880u : u32
        let s_86_0: u32 = 104880;
        // D s_86_1: read-reg s_86_0:struct
        let s_86_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_86_0 as isize);
            tracer.read_register(s_86_0 as isize, value);
            value
        };
        // D s_86_2: call _get_MDCR_EL2_Type_TDE(s_86_1)
        let s_86_2: bool = u_get_MDCR_EL2_Type_TDE(state, tracer, s_86_1);
        // C s_86_3: const #104880u : u32
        let s_86_3: u32 = 104880;
        // D s_86_4: read-reg s_86_3:struct
        let s_86_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_86_3 as isize);
            tracer.read_register(s_86_3 as isize, value);
            value
        };
        // D s_86_5: call _get_MDCR_EL2_Type_TDA(s_86_4)
        let s_86_5: bool = u_get_MDCR_EL2_Type_TDA(state, tracer, s_86_4);
        // D s_86_6: cast zx s_86_2 -> bv
        let s_86_6: Bits = Bits::new(s_86_2 as u128, 1u16);
        // D s_86_7: cast zx s_86_5 -> bv
        let s_86_7: Bits = Bits::new(s_86_5 as u128, 1u16);
        // D s_86_8: cast reint s_86_6 -> u128
        let s_86_8: u128 = (s_86_6.value() as u128);
        // D s_86_9: size-of s_86_6
        let s_86_9: u16 = s_86_6.length();
        // D s_86_10: cast reint s_86_7 -> u128
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
        // D s_86_21: write-var gs#136948 <= s_86_20
        fn_state.gs_136948 = s_86_20;
        // N s_86_22: jump b51
        return block_51(state, tracer, fn_state);
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
        // D s_87_3: not s_87_2
        let s_87_3: bool = !s_87_2;
        // D s_87_4: write-var gs#136947 <= s_87_3
        fn_state.gs_136947 = s_87_3;
        // N s_87_5: jump b49
        return block_49(state, tracer, fn_state);
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
        // S s_88_5: call AArch32_TakeHypTrapException(s_88_4)
        let s_88_5: () = AArch32_TakeHypTrapException(state, tracer, s_88_4);
        // N s_88_6: return
        return;
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_89_0: read-var __HDCR_TDCC:u8
        let s_89_0: bool = fn_state.u__HDCR_TDCC;
        // D s_89_1: cast zx s_89_0 -> bv
        let s_89_1: Bits = Bits::new(s_89_0 as u128, 1u16);
        // C s_89_2: const #1u : u8
        let s_89_2: bool = true;
        // C s_89_3: cast zx s_89_2 -> bv
        let s_89_3: Bits = Bits::new(s_89_2 as u128, 1u16);
        // D s_89_4: cmp-eq s_89_1 s_89_3
        let s_89_4: bool = ((s_89_1) == (s_89_3));
        // D s_89_5: write-var gs#136946 <= s_89_4
        fn_state.gs_136946 = s_89_4;
        // N s_89_6: jump b46
        return block_46(state, tracer, fn_state);
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
        // D s_90_3: write-var gs#136945 <= s_90_2
        fn_state.gs_136945 = s_90_2;
        // N s_90_4: jump b44
        return block_44(state, tracer, fn_state);
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
        // C s_91_5: const #432u : u32
        let s_91_5: u32 = 432;
        // D s_91_6: read-reg s_91_5:u8
        let s_91_6: u8 = {
            let value = state.read_register::<u8>(s_91_5 as isize);
            tracer.read_register(s_91_5 as isize, value);
            value
        };
        // D s_91_7: call AArch64_AArch32SystemAccessTrap(s_91_6, s_91_4)
        let s_91_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_91_6, s_91_4);
        // N s_91_8: return
        return;
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var __MDCR_EL2_TDCC:u8
        let s_92_0: bool = fn_state.u__MDCR_EL2_TDCC;
        // D s_92_1: cast zx s_92_0 -> bv
        let s_92_1: Bits = Bits::new(s_92_0 as u128, 1u16);
        // C s_92_2: const #1u : u8
        let s_92_2: bool = true;
        // C s_92_3: cast zx s_92_2 -> bv
        let s_92_3: Bits = Bits::new(s_92_2 as u128, 1u16);
        // D s_92_4: cmp-eq s_92_1 s_92_3
        let s_92_4: bool = ((s_92_1) == (s_92_3));
        // D s_92_5: write-var gs#136944 <= s_92_4
        fn_state.gs_136944 = s_92_4;
        // N s_92_6: jump b41
        return block_41(state, tracer, fn_state);
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
        // D s_93_3: not s_93_2
        let s_93_3: bool = !s_93_2;
        // D s_93_4: write-var gs#136943 <= s_93_3
        fn_state.gs_136943 = s_93_3;
        // N s_93_5: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #440u : u32
        let s_94_0: u32 = 440;
        // D s_94_1: read-reg s_94_0:u8
        let s_94_1: u8 = {
            let value = state.read_register::<u8>(s_94_0 as isize);
            tracer.read_register(s_94_0 as isize, value);
            value
        };
        // D s_94_2: call ELUsingAArch32(s_94_1)
        let s_94_2: bool = ELUsingAArch32(state, tracer, s_94_1);
        // D s_94_3: not s_94_2
        let s_94_3: bool = !s_94_2;
        // N s_94_4: branch s_94_3 b190 b95
        if s_94_3 {
            return block_190(state, tracer, fn_state);
        } else {
            return block_95(state, tracer, fn_state);
        };
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #0u : u8
        let s_95_0: bool = false;
        // D s_95_1: write-var gs#136958 <= s_95_0
        fn_state.gs_136958 = s_95_0;
        // N s_95_2: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_96_0: read-var gs#136958:u8
        let s_96_0: bool = fn_state.gs_136958;
        // N s_96_1: branch s_96_0 b181 b97
        if s_96_0 {
            return block_181(state, tracer, fn_state);
        } else {
            return block_97(state, tracer, fn_state);
        };
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
        // N s_97_3: branch s_97_2 b180 b98
        if s_97_2 {
            return block_180(state, tracer, fn_state);
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
        // D s_98_1: write-var gs#136959 <= s_98_0
        fn_state.gs_136959 = s_98_0;
        // N s_98_2: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_99_0: read-var gs#136959:u8
        let s_99_0: bool = fn_state.gs_136959;
        // N s_99_1: branch s_99_0 b163 b100
        if s_99_0 {
            return block_163(state, tracer, fn_state);
        } else {
            return block_100(state, tracer, fn_state);
        };
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #() : ()
        let s_100_0: () = ();
        // S s_100_1: call EL2Enabled(s_100_0)
        let s_100_1: bool = EL2Enabled(state, tracer, s_100_0);
        // N s_100_2: branch s_100_1 b162 b101
        if s_100_1 {
            return block_162(state, tracer, fn_state);
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
        // D s_101_1: write-var gs#136960 <= s_101_0
        fn_state.gs_136960 = s_101_0;
        // N s_101_2: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_102_0: read-var gs#136960:u8
        let s_102_0: bool = fn_state.gs_136960;
        // N s_102_1: branch s_102_0 b161 b103
        if s_102_0 {
            return block_161(state, tracer, fn_state);
        } else {
            return block_103(state, tracer, fn_state);
        };
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_103_0: const #0u : u8
        let s_103_0: bool = false;
        // D s_103_1: write-var gs#136961 <= s_103_0
        fn_state.gs_136961 = s_103_0;
        // N s_103_2: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_104_0: read-var gs#136961:u8
        let s_104_0: bool = fn_state.gs_136961;
        // N s_104_1: branch s_104_0 b160 b105
        if s_104_0 {
            return block_160(state, tracer, fn_state);
        } else {
            return block_105(state, tracer, fn_state);
        };
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #() : ()
        let s_105_0: () = ();
        // S s_105_1: call EL2Enabled(s_105_0)
        let s_105_1: bool = EL2Enabled(state, tracer, s_105_0);
        // N s_105_2: branch s_105_1 b159 b106
        if s_105_1 {
            return block_159(state, tracer, fn_state);
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
        // D s_106_1: write-var gs#136962 <= s_106_0
        fn_state.gs_136962 = s_106_0;
        // N s_106_2: jump b107
        return block_107(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_107_0: read-var gs#136962:u8
        let s_107_0: bool = fn_state.gs_136962;
        // N s_107_1: branch s_107_0 b158 b108
        if s_107_0 {
            return block_158(state, tracer, fn_state);
        } else {
            return block_108(state, tracer, fn_state);
        };
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_108_0: const #0u : u8
        let s_108_0: bool = false;
        // D s_108_1: write-var gs#136963 <= s_108_0
        fn_state.gs_136963 = s_108_0;
        // N s_108_2: jump b109
        return block_109(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_109_0: read-var gs#136963:u8
        let s_109_0: bool = fn_state.gs_136963;
        // N s_109_1: branch s_109_0 b157 b110
        if s_109_0 {
            return block_157(state, tracer, fn_state);
        } else {
            return block_110(state, tracer, fn_state);
        };
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_110_0: const #() : ()
        let s_110_0: () = ();
        // S s_110_1: call EL2Enabled(s_110_0)
        let s_110_1: bool = EL2Enabled(state, tracer, s_110_0);
        // N s_110_2: branch s_110_1 b156 b111
        if s_110_1 {
            return block_156(state, tracer, fn_state);
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
        // D s_111_1: write-var gs#136964 <= s_111_0
        fn_state.gs_136964 = s_111_0;
        // N s_111_2: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_112_0: read-var gs#136964:u8
        let s_112_0: bool = fn_state.gs_136964;
        // N s_112_1: branch s_112_0 b152 b113
        if s_112_0 {
            return block_152(state, tracer, fn_state);
        } else {
            return block_113(state, tracer, fn_state);
        };
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_113_0: const #0u : u8
        let s_113_0: bool = false;
        // D s_113_1: write-var gs#136966 <= s_113_0
        fn_state.gs_136966 = s_113_0;
        // N s_113_2: jump b114
        return block_114(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_114_0: read-var gs#136966:u8
        let s_114_0: bool = fn_state.gs_136966;
        // N s_114_1: branch s_114_0 b151 b115
        if s_114_0 {
            return block_151(state, tracer, fn_state);
        } else {
            return block_115(state, tracer, fn_state);
        };
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_115_0: const #() : ()
        let s_115_0: () = ();
        // S s_115_1: call EL2Enabled(s_115_0)
        let s_115_1: bool = EL2Enabled(state, tracer, s_115_0);
        // N s_115_2: branch s_115_1 b150 b116
        if s_115_1 {
            return block_150(state, tracer, fn_state);
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
        // D s_116_1: write-var gs#136967 <= s_116_0
        fn_state.gs_136967 = s_116_0;
        // N s_116_2: jump b117
        return block_117(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_117_0: read-var gs#136967:u8
        let s_117_0: bool = fn_state.gs_136967;
        // N s_117_1: branch s_117_0 b146 b118
        if s_117_0 {
            return block_146(state, tracer, fn_state);
        } else {
            return block_118(state, tracer, fn_state);
        };
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #0u : u8
        let s_118_0: bool = false;
        // D s_118_1: write-var gs#136969 <= s_118_0
        fn_state.gs_136969 = s_118_0;
        // N s_118_2: jump b119
        return block_119(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_119_0: read-var gs#136969:u8
        let s_119_0: bool = fn_state.gs_136969;
        // N s_119_1: branch s_119_0 b145 b120
        if s_119_0 {
            return block_145(state, tracer, fn_state);
        } else {
            return block_120(state, tracer, fn_state);
        };
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_120_0: const #424u : u32
        let s_120_0: u32 = 424;
        // D s_120_1: read-reg s_120_0:u8
        let s_120_1: u8 = {
            let value = state.read_register::<u8>(s_120_0 as isize);
            tracer.read_register(s_120_0 as isize, value);
            value
        };
        // C s_120_2: const #2u : u8
        let s_120_2: u8 = 2;
        // D s_120_3: cmp-lt s_120_1 s_120_2
        let s_120_3: bool = ((s_120_1) < (s_120_2));
        // N s_120_4: branch s_120_3 b144 b121
        if s_120_3 {
            return block_144(state, tracer, fn_state);
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
        // D s_121_1: write-var gs#136970 <= s_121_0
        fn_state.gs_136970 = s_121_0;
        // N s_121_2: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_122_0: read-var gs#136970:u8
        let s_122_0: bool = fn_state.gs_136970;
        // N s_122_1: branch s_122_0 b143 b123
        if s_122_0 {
            return block_143(state, tracer, fn_state);
        } else {
            return block_123(state, tracer, fn_state);
        };
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_123_0: const #0u : u8
        let s_123_0: bool = false;
        // D s_123_1: write-var gs#136971 <= s_123_0
        fn_state.gs_136971 = s_123_0;
        // N s_123_2: jump b124
        return block_124(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_124_0: read-var gs#136971:u8
        let s_124_0: bool = fn_state.gs_136971;
        // N s_124_1: branch s_124_0 b142 b125
        if s_124_0 {
            return block_142(state, tracer, fn_state);
        } else {
            return block_125(state, tracer, fn_state);
        };
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_125_0: const #424u : u32
        let s_125_0: u32 = 424;
        // D s_125_1: read-reg s_125_0:u8
        let s_125_1: u8 = {
            let value = state.read_register::<u8>(s_125_0 as isize);
            tracer.read_register(s_125_0 as isize, value);
            value
        };
        // C s_125_2: const #2u : u8
        let s_125_2: u8 = 2;
        // D s_125_3: cmp-lt s_125_1 s_125_2
        let s_125_3: bool = ((s_125_1) < (s_125_2));
        // N s_125_4: branch s_125_3 b141 b126
        if s_125_3 {
            return block_141(state, tracer, fn_state);
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
        // D s_126_1: write-var gs#136972 <= s_126_0
        fn_state.gs_136972 = s_126_0;
        // N s_126_2: jump b127
        return block_127(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_127_0: read-var gs#136972:u8
        let s_127_0: bool = fn_state.gs_136972;
        // N s_127_1: branch s_127_0 b140 b128
        if s_127_0 {
            return block_140(state, tracer, fn_state);
        } else {
            return block_128(state, tracer, fn_state);
        };
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_128_0: const #0u : u8
        let s_128_0: bool = false;
        // D s_128_1: write-var gs#136973 <= s_128_0
        fn_state.gs_136973 = s_128_0;
        // N s_128_2: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_129_0: read-var gs#136973:u8
        let s_129_0: bool = fn_state.gs_136973;
        // N s_129_1: branch s_129_0 b139 b130
        if s_129_0 {
            return block_139(state, tracer, fn_state);
        } else {
            return block_130(state, tracer, fn_state);
        };
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_130_0: const #424u : u32
        let s_130_0: u32 = 424;
        // D s_130_1: read-reg s_130_0:u8
        let s_130_1: u8 = {
            let value = state.read_register::<u8>(s_130_0 as isize);
            tracer.read_register(s_130_0 as isize, value);
            value
        };
        // C s_130_2: const #2u : u8
        let s_130_2: u8 = 2;
        // D s_130_3: cmp-lt s_130_1 s_130_2
        let s_130_3: bool = ((s_130_1) < (s_130_2));
        // N s_130_4: branch s_130_3 b138 b131
        if s_130_3 {
            return block_138(state, tracer, fn_state);
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
        // D s_131_1: write-var gs#136974 <= s_131_0
        fn_state.gs_136974 = s_131_0;
        // N s_131_2: jump b132
        return block_132(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_132_0: read-var gs#136974:u8
        let s_132_0: bool = fn_state.gs_136974;
        // N s_132_1: branch s_132_0 b137 b133
        if s_132_0 {
            return block_137(state, tracer, fn_state);
        } else {
            return block_133(state, tracer, fn_state);
        };
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_133_0: const #0u : u8
        let s_133_0: bool = false;
        // D s_133_1: write-var gs#136975 <= s_133_0
        fn_state.gs_136975 = s_133_0;
        // N s_133_2: jump b134
        return block_134(state, tracer, fn_state);
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_134_0: read-var gs#136975:u8
        let s_134_0: bool = fn_state.gs_136975;
        // N s_134_1: branch s_134_0 b136 b135
        if s_134_0 {
            return block_136(state, tracer, fn_state);
        } else {
            return block_135(state, tracer, fn_state);
        };
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_135_0: const #() : ()
        let s_135_0: () = ();
        // S s_135_1: call DBGDTRRXint_read(s_135_0)
        let s_135_1: ProductType700c18a878c5601b = DBGDTRRXint_read(
            state,
            tracer,
            s_135_0,
        );
        // D s_135_2: write-var ga#238627 <= s_135_1
        fn_state.ga_238627 = s_135_1;
        // D s_135_3: read-var ga#238627.0:struct
        let s_135_3: u32 = fn_state.ga_238627._0;
        // C s_135_4: const #4s : i
        let s_135_4: i128 = 4;
        // D s_135_5: cast zx s_135_3 -> bv
        let s_135_5: Bits = Bits::new(s_135_3 as u128, 32u16);
        // D s_135_6: read-var address:u32
        let s_135_6: u32 = fn_state.address;
        // D s_135_7: call MemA_set(s_135_6, s_135_4, s_135_5)
        let s_135_7: () = MemA_set(state, tracer, s_135_6, s_135_4, s_135_5);
        // N s_135_8: return
        return;
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_136_0: const #6u : u8
        let s_136_0: u8 = 6;
        // C s_136_1: cast zx s_136_0 -> bv
        let s_136_1: Bits = Bits::new(s_136_0 as u128, 8u16);
        // C s_136_2: cast zx s_136_1 -> i
        let s_136_2: i128 = (s_136_1.value() as i128);
        // C s_136_3: cast reint s_136_2 -> i64
        let s_136_3: i64 = (s_136_2 as i64);
        // C s_136_4: cast zx s_136_3 -> i
        let s_136_4: i128 = (i128::try_from(s_136_3).unwrap());
        // C s_136_5: const #424u : u32
        let s_136_5: u32 = 424;
        // D s_136_6: read-reg s_136_5:u8
        let s_136_6: u8 = {
            let value = state.read_register::<u8>(s_136_5 as isize);
            tracer.read_register(s_136_5 as isize, value);
            value
        };
        // D s_136_7: call AArch64_AArch32SystemAccessTrap(s_136_6, s_136_4)
        let s_136_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_136_6,
            s_136_4,
        );
        // N s_136_8: return
        return;
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_137_0: read-var __MDCR_EL3_TDA:u8
        let s_137_0: bool = fn_state.u__MDCR_EL3_TDA;
        // D s_137_1: cast zx s_137_0 -> bv
        let s_137_1: Bits = Bits::new(s_137_0 as u128, 1u16);
        // C s_137_2: const #1u : u8
        let s_137_2: bool = true;
        // C s_137_3: cast zx s_137_2 -> bv
        let s_137_3: Bits = Bits::new(s_137_2 as u128, 1u16);
        // D s_137_4: cmp-eq s_137_1 s_137_3
        let s_137_4: bool = ((s_137_1) == (s_137_3));
        // D s_137_5: write-var gs#136975 <= s_137_4
        fn_state.gs_136975 = s_137_4;
        // N s_137_6: jump b134
        return block_134(state, tracer, fn_state);
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_138_0: const #424u : u32
        let s_138_0: u32 = 424;
        // D s_138_1: read-reg s_138_0:u8
        let s_138_1: u8 = {
            let value = state.read_register::<u8>(s_138_0 as isize);
            tracer.read_register(s_138_0 as isize, value);
            value
        };
        // D s_138_2: call ELUsingAArch32(s_138_1)
        let s_138_2: bool = ELUsingAArch32(state, tracer, s_138_1);
        // D s_138_3: not s_138_2
        let s_138_3: bool = !s_138_2;
        // D s_138_4: write-var gs#136974 <= s_138_3
        fn_state.gs_136974 = s_138_3;
        // N s_138_5: jump b132
        return block_132(state, tracer, fn_state);
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_139_0: const #() : ()
        let s_139_0: () = ();
        // S s_139_1: call AArch32_TakeMonitorTrapException(s_139_0)
        let s_139_1: () = AArch32_TakeMonitorTrapException(state, tracer, s_139_0);
        // N s_139_2: return
        return;
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_140_0: read-var __SDCR_TDCC:u8
        let s_140_0: bool = fn_state.u__SDCR_TDCC;
        // D s_140_1: cast zx s_140_0 -> bv
        let s_140_1: Bits = Bits::new(s_140_0 as u128, 1u16);
        // C s_140_2: const #1u : u8
        let s_140_2: bool = true;
        // C s_140_3: cast zx s_140_2 -> bv
        let s_140_3: Bits = Bits::new(s_140_2 as u128, 1u16);
        // D s_140_4: cmp-eq s_140_1 s_140_3
        let s_140_4: bool = ((s_140_1) == (s_140_3));
        // D s_140_5: write-var gs#136973 <= s_140_4
        fn_state.gs_136973 = s_140_4;
        // N s_140_6: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_141_0: const #424u : u32
        let s_141_0: u32 = 424;
        // D s_141_1: read-reg s_141_0:u8
        let s_141_1: u8 = {
            let value = state.read_register::<u8>(s_141_0 as isize);
            tracer.read_register(s_141_0 as isize, value);
            value
        };
        // D s_141_2: call ELUsingAArch32(s_141_1)
        let s_141_2: bool = ELUsingAArch32(state, tracer, s_141_1);
        // D s_141_3: write-var gs#136972 <= s_141_2
        fn_state.gs_136972 = s_141_2;
        // N s_141_4: jump b127
        return block_127(state, tracer, fn_state);
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_142_0: const #6u : u8
        let s_142_0: u8 = 6;
        // C s_142_1: cast zx s_142_0 -> bv
        let s_142_1: Bits = Bits::new(s_142_0 as u128, 8u16);
        // C s_142_2: cast zx s_142_1 -> i
        let s_142_2: i128 = (s_142_1.value() as i128);
        // C s_142_3: cast reint s_142_2 -> i64
        let s_142_3: i64 = (s_142_2 as i64);
        // C s_142_4: cast zx s_142_3 -> i
        let s_142_4: i128 = (i128::try_from(s_142_3).unwrap());
        // C s_142_5: const #424u : u32
        let s_142_5: u32 = 424;
        // D s_142_6: read-reg s_142_5:u8
        let s_142_6: u8 = {
            let value = state.read_register::<u8>(s_142_5 as isize);
            tracer.read_register(s_142_5 as isize, value);
            value
        };
        // D s_142_7: call AArch64_AArch32SystemAccessTrap(s_142_6, s_142_4)
        let s_142_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_142_6,
            s_142_4,
        );
        // N s_142_8: return
        return;
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_143_0: read-var __MDCR_EL3_TDCC:u8
        let s_143_0: bool = fn_state.u__MDCR_EL3_TDCC;
        // D s_143_1: cast zx s_143_0 -> bv
        let s_143_1: Bits = Bits::new(s_143_0 as u128, 1u16);
        // C s_143_2: const #1u : u8
        let s_143_2: bool = true;
        // C s_143_3: cast zx s_143_2 -> bv
        let s_143_3: Bits = Bits::new(s_143_2 as u128, 1u16);
        // D s_143_4: cmp-eq s_143_1 s_143_3
        let s_143_4: bool = ((s_143_1) == (s_143_3));
        // D s_143_5: write-var gs#136971 <= s_143_4
        fn_state.gs_136971 = s_143_4;
        // N s_143_6: jump b124
        return block_124(state, tracer, fn_state);
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_144_0: const #424u : u32
        let s_144_0: u32 = 424;
        // D s_144_1: read-reg s_144_0:u8
        let s_144_1: u8 = {
            let value = state.read_register::<u8>(s_144_0 as isize);
            tracer.read_register(s_144_0 as isize, value);
            value
        };
        // D s_144_2: call ELUsingAArch32(s_144_1)
        let s_144_2: bool = ELUsingAArch32(state, tracer, s_144_1);
        // D s_144_3: not s_144_2
        let s_144_3: bool = !s_144_2;
        // D s_144_4: write-var gs#136970 <= s_144_3
        fn_state.gs_136970 = s_144_3;
        // N s_144_5: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_145_0: const #6u : u8
        let s_145_0: u8 = 6;
        // C s_145_1: cast zx s_145_0 -> bv
        let s_145_1: Bits = Bits::new(s_145_0 as u128, 8u16);
        // C s_145_2: cast zx s_145_1 -> i
        let s_145_2: i128 = (s_145_1.value() as i128);
        // C s_145_3: cast reint s_145_2 -> i64
        let s_145_3: i64 = (s_145_2 as i64);
        // C s_145_4: cast zx s_145_3 -> i
        let s_145_4: i128 = (i128::try_from(s_145_3).unwrap());
        // S s_145_5: call AArch32_TakeHypTrapException(s_145_4)
        let s_145_5: () = AArch32_TakeHypTrapException(state, tracer, s_145_4);
        // N s_145_6: return
        return;
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_146_0: const #() : ()
        let s_146_0: () = ();
        // S s_146_1: call HCR_read(s_146_0)
        let s_146_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_146_0);
        // S s_146_2: call _get_HCR_Type_TGE(s_146_1)
        let s_146_2: bool = u_get_HCR_Type_TGE(state, tracer, s_146_1);
        // S s_146_3: cast zx s_146_2 -> bv
        let s_146_3: Bits = Bits::new(s_146_2 as u128, 1u16);
        // C s_146_4: const #1u : u8
        let s_146_4: bool = true;
        // C s_146_5: cast zx s_146_4 -> bv
        let s_146_5: Bits = Bits::new(s_146_4 as u128, 1u16);
        // S s_146_6: cmp-eq s_146_3 s_146_5
        let s_146_6: bool = ((s_146_3) == (s_146_5));
        // N s_146_7: branch s_146_6 b149 b147
        if s_146_6 {
            return block_149(state, tracer, fn_state);
        } else {
            return block_147(state, tracer, fn_state);
        };
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_147_0: const #() : ()
        let s_147_0: () = ();
        // S s_147_1: call HDCR_read(s_147_0)
        let s_147_1: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_147_0);
        // S s_147_2: call _get_HDCR_Type_TDE(s_147_1)
        let s_147_2: bool = u_get_HDCR_Type_TDE(state, tracer, s_147_1);
        // C s_147_3: const #() : ()
        let s_147_3: () = ();
        // S s_147_4: call HDCR_read(s_147_3)
        let s_147_4: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_147_3);
        // S s_147_5: call _get_HDCR_Type_TDA(s_147_4)
        let s_147_5: bool = u_get_HDCR_Type_TDA(state, tracer, s_147_4);
        // S s_147_6: cast zx s_147_2 -> bv
        let s_147_6: Bits = Bits::new(s_147_2 as u128, 1u16);
        // S s_147_7: cast zx s_147_5 -> bv
        let s_147_7: Bits = Bits::new(s_147_5 as u128, 1u16);
        // S s_147_8: cast reint s_147_6 -> u128
        let s_147_8: u128 = (s_147_6.value() as u128);
        // D s_147_9: size-of s_147_6
        let s_147_9: u16 = s_147_6.length();
        // S s_147_10: cast reint s_147_7 -> u128
        let s_147_10: u128 = (s_147_7.value() as u128);
        // D s_147_11: size-of s_147_7
        let s_147_11: u16 = s_147_7.length();
        // D s_147_12: lsl s_147_8 s_147_11
        let s_147_12: u128 = s_147_8 << s_147_11;
        // D s_147_13: or s_147_12 s_147_10
        let s_147_13: u128 = ((s_147_12) | (s_147_10));
        // D s_147_14: add s_147_9 s_147_11
        let s_147_14: u16 = (s_147_9 + s_147_11);
        // D s_147_15: create-bits s_147_13 s_147_14
        let s_147_15: Bits = Bits::new(s_147_13, s_147_14);
        // D s_147_16: cast reint s_147_15 -> u8
        let s_147_16: u8 = (s_147_15.value() as u8);
        // D s_147_17: cast zx s_147_16 -> bv
        let s_147_17: Bits = Bits::new(s_147_16 as u128, 2u16);
        // C s_147_18: const #0u : u8
        let s_147_18: u8 = 0;
        // C s_147_19: cast zx s_147_18 -> bv
        let s_147_19: Bits = Bits::new(s_147_18 as u128, 2u16);
        // D s_147_20: cmp-ne s_147_17 s_147_19
        let s_147_20: bool = ((s_147_17) != (s_147_19));
        // D s_147_21: write-var gs#136968 <= s_147_20
        fn_state.gs_136968 = s_147_20;
        // N s_147_22: jump b148
        return block_148(state, tracer, fn_state);
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_148_0: read-var gs#136968:u8
        let s_148_0: bool = fn_state.gs_136968;
        // D s_148_1: write-var gs#136969 <= s_148_0
        fn_state.gs_136969 = s_148_0;
        // N s_148_2: jump b119
        return block_119(state, tracer, fn_state);
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_149_0: const #1u : u8
        let s_149_0: bool = true;
        // D s_149_1: write-var gs#136968 <= s_149_0
        fn_state.gs_136968 = s_149_0;
        // N s_149_2: jump b148
        return block_148(state, tracer, fn_state);
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_150_0: const #432u : u32
        let s_150_0: u32 = 432;
        // D s_150_1: read-reg s_150_0:u8
        let s_150_1: u8 = {
            let value = state.read_register::<u8>(s_150_0 as isize);
            tracer.read_register(s_150_0 as isize, value);
            value
        };
        // D s_150_2: call ELUsingAArch32(s_150_1)
        let s_150_2: bool = ELUsingAArch32(state, tracer, s_150_1);
        // D s_150_3: write-var gs#136967 <= s_150_2
        fn_state.gs_136967 = s_150_2;
        // N s_150_4: jump b117
        return block_117(state, tracer, fn_state);
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_151_0: const #6u : u8
        let s_151_0: u8 = 6;
        // C s_151_1: cast zx s_151_0 -> bv
        let s_151_1: Bits = Bits::new(s_151_0 as u128, 8u16);
        // C s_151_2: cast zx s_151_1 -> i
        let s_151_2: i128 = (s_151_1.value() as i128);
        // C s_151_3: cast reint s_151_2 -> i64
        let s_151_3: i64 = (s_151_2 as i64);
        // C s_151_4: cast zx s_151_3 -> i
        let s_151_4: i128 = (i128::try_from(s_151_3).unwrap());
        // C s_151_5: const #432u : u32
        let s_151_5: u32 = 432;
        // D s_151_6: read-reg s_151_5:u8
        let s_151_6: u8 = {
            let value = state.read_register::<u8>(s_151_5 as isize);
            tracer.read_register(s_151_5 as isize, value);
            value
        };
        // D s_151_7: call AArch64_AArch32SystemAccessTrap(s_151_6, s_151_4)
        let s_151_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_151_6,
            s_151_4,
        );
        // N s_151_8: return
        return;
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_152_0: const #102552u : u32
        let s_152_0: u32 = 102552;
        // D s_152_1: read-reg s_152_0:struct
        let s_152_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_152_0 as isize);
            tracer.read_register(s_152_0 as isize, value);
            value
        };
        // D s_152_2: call _get_HCR_EL2_Type_TGE(s_152_1)
        let s_152_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_152_1);
        // D s_152_3: cast zx s_152_2 -> bv
        let s_152_3: Bits = Bits::new(s_152_2 as u128, 1u16);
        // C s_152_4: const #1u : u8
        let s_152_4: bool = true;
        // C s_152_5: cast zx s_152_4 -> bv
        let s_152_5: Bits = Bits::new(s_152_4 as u128, 1u16);
        // D s_152_6: cmp-eq s_152_3 s_152_5
        let s_152_6: bool = ((s_152_3) == (s_152_5));
        // N s_152_7: branch s_152_6 b155 b153
        if s_152_6 {
            return block_155(state, tracer, fn_state);
        } else {
            return block_153(state, tracer, fn_state);
        };
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_153_0: const #104880u : u32
        let s_153_0: u32 = 104880;
        // D s_153_1: read-reg s_153_0:struct
        let s_153_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_153_0 as isize);
            tracer.read_register(s_153_0 as isize, value);
            value
        };
        // D s_153_2: call _get_MDCR_EL2_Type_TDE(s_153_1)
        let s_153_2: bool = u_get_MDCR_EL2_Type_TDE(state, tracer, s_153_1);
        // C s_153_3: const #104880u : u32
        let s_153_3: u32 = 104880;
        // D s_153_4: read-reg s_153_3:struct
        let s_153_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_153_3 as isize);
            tracer.read_register(s_153_3 as isize, value);
            value
        };
        // D s_153_5: call _get_MDCR_EL2_Type_TDA(s_153_4)
        let s_153_5: bool = u_get_MDCR_EL2_Type_TDA(state, tracer, s_153_4);
        // D s_153_6: cast zx s_153_2 -> bv
        let s_153_6: Bits = Bits::new(s_153_2 as u128, 1u16);
        // D s_153_7: cast zx s_153_5 -> bv
        let s_153_7: Bits = Bits::new(s_153_5 as u128, 1u16);
        // D s_153_8: cast reint s_153_6 -> u128
        let s_153_8: u128 = (s_153_6.value() as u128);
        // D s_153_9: size-of s_153_6
        let s_153_9: u16 = s_153_6.length();
        // D s_153_10: cast reint s_153_7 -> u128
        let s_153_10: u128 = (s_153_7.value() as u128);
        // D s_153_11: size-of s_153_7
        let s_153_11: u16 = s_153_7.length();
        // D s_153_12: lsl s_153_8 s_153_11
        let s_153_12: u128 = s_153_8 << s_153_11;
        // D s_153_13: or s_153_12 s_153_10
        let s_153_13: u128 = ((s_153_12) | (s_153_10));
        // D s_153_14: add s_153_9 s_153_11
        let s_153_14: u16 = (s_153_9 + s_153_11);
        // D s_153_15: create-bits s_153_13 s_153_14
        let s_153_15: Bits = Bits::new(s_153_13, s_153_14);
        // D s_153_16: cast reint s_153_15 -> u8
        let s_153_16: u8 = (s_153_15.value() as u8);
        // D s_153_17: cast zx s_153_16 -> bv
        let s_153_17: Bits = Bits::new(s_153_16 as u128, 2u16);
        // C s_153_18: const #0u : u8
        let s_153_18: u8 = 0;
        // C s_153_19: cast zx s_153_18 -> bv
        let s_153_19: Bits = Bits::new(s_153_18 as u128, 2u16);
        // D s_153_20: cmp-ne s_153_17 s_153_19
        let s_153_20: bool = ((s_153_17) != (s_153_19));
        // D s_153_21: write-var gs#136965 <= s_153_20
        fn_state.gs_136965 = s_153_20;
        // N s_153_22: jump b154
        return block_154(state, tracer, fn_state);
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_154_0: read-var gs#136965:u8
        let s_154_0: bool = fn_state.gs_136965;
        // D s_154_1: write-var gs#136966 <= s_154_0
        fn_state.gs_136966 = s_154_0;
        // N s_154_2: jump b114
        return block_114(state, tracer, fn_state);
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_155_0: const #1u : u8
        let s_155_0: bool = true;
        // D s_155_1: write-var gs#136965 <= s_155_0
        fn_state.gs_136965 = s_155_0;
        // N s_155_2: jump b154
        return block_154(state, tracer, fn_state);
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_156_0: const #432u : u32
        let s_156_0: u32 = 432;
        // D s_156_1: read-reg s_156_0:u8
        let s_156_1: u8 = {
            let value = state.read_register::<u8>(s_156_0 as isize);
            tracer.read_register(s_156_0 as isize, value);
            value
        };
        // D s_156_2: call ELUsingAArch32(s_156_1)
        let s_156_2: bool = ELUsingAArch32(state, tracer, s_156_1);
        // D s_156_3: not s_156_2
        let s_156_3: bool = !s_156_2;
        // D s_156_4: write-var gs#136964 <= s_156_3
        fn_state.gs_136964 = s_156_3;
        // N s_156_5: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_157_0: const #6u : u8
        let s_157_0: u8 = 6;
        // C s_157_1: cast zx s_157_0 -> bv
        let s_157_1: Bits = Bits::new(s_157_0 as u128, 8u16);
        // C s_157_2: cast zx s_157_1 -> i
        let s_157_2: i128 = (s_157_1.value() as i128);
        // C s_157_3: cast reint s_157_2 -> i64
        let s_157_3: i64 = (s_157_2 as i64);
        // C s_157_4: cast zx s_157_3 -> i
        let s_157_4: i128 = (i128::try_from(s_157_3).unwrap());
        // S s_157_5: call AArch32_TakeHypTrapException(s_157_4)
        let s_157_5: () = AArch32_TakeHypTrapException(state, tracer, s_157_4);
        // N s_157_6: return
        return;
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_158_0: read-var __HDCR_TDCC:u8
        let s_158_0: bool = fn_state.u__HDCR_TDCC;
        // D s_158_1: cast zx s_158_0 -> bv
        let s_158_1: Bits = Bits::new(s_158_0 as u128, 1u16);
        // C s_158_2: const #1u : u8
        let s_158_2: bool = true;
        // C s_158_3: cast zx s_158_2 -> bv
        let s_158_3: Bits = Bits::new(s_158_2 as u128, 1u16);
        // D s_158_4: cmp-eq s_158_1 s_158_3
        let s_158_4: bool = ((s_158_1) == (s_158_3));
        // D s_158_5: write-var gs#136963 <= s_158_4
        fn_state.gs_136963 = s_158_4;
        // N s_158_6: jump b109
        return block_109(state, tracer, fn_state);
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_159_0: const #432u : u32
        let s_159_0: u32 = 432;
        // D s_159_1: read-reg s_159_0:u8
        let s_159_1: u8 = {
            let value = state.read_register::<u8>(s_159_0 as isize);
            tracer.read_register(s_159_0 as isize, value);
            value
        };
        // D s_159_2: call ELUsingAArch32(s_159_1)
        let s_159_2: bool = ELUsingAArch32(state, tracer, s_159_1);
        // D s_159_3: write-var gs#136962 <= s_159_2
        fn_state.gs_136962 = s_159_2;
        // N s_159_4: jump b107
        return block_107(state, tracer, fn_state);
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_160_0: const #6u : u8
        let s_160_0: u8 = 6;
        // C s_160_1: cast zx s_160_0 -> bv
        let s_160_1: Bits = Bits::new(s_160_0 as u128, 8u16);
        // C s_160_2: cast zx s_160_1 -> i
        let s_160_2: i128 = (s_160_1.value() as i128);
        // C s_160_3: cast reint s_160_2 -> i64
        let s_160_3: i64 = (s_160_2 as i64);
        // C s_160_4: cast zx s_160_3 -> i
        let s_160_4: i128 = (i128::try_from(s_160_3).unwrap());
        // C s_160_5: const #432u : u32
        let s_160_5: u32 = 432;
        // D s_160_6: read-reg s_160_5:u8
        let s_160_6: u8 = {
            let value = state.read_register::<u8>(s_160_5 as isize);
            tracer.read_register(s_160_5 as isize, value);
            value
        };
        // D s_160_7: call AArch64_AArch32SystemAccessTrap(s_160_6, s_160_4)
        let s_160_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_160_6,
            s_160_4,
        );
        // N s_160_8: return
        return;
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_161_0: read-var __MDCR_EL2_TDCC:u8
        let s_161_0: bool = fn_state.u__MDCR_EL2_TDCC;
        // D s_161_1: cast zx s_161_0 -> bv
        let s_161_1: Bits = Bits::new(s_161_0 as u128, 1u16);
        // C s_161_2: const #1u : u8
        let s_161_2: bool = true;
        // C s_161_3: cast zx s_161_2 -> bv
        let s_161_3: Bits = Bits::new(s_161_2 as u128, 1u16);
        // D s_161_4: cmp-eq s_161_1 s_161_3
        let s_161_4: bool = ((s_161_1) == (s_161_3));
        // D s_161_5: write-var gs#136961 <= s_161_4
        fn_state.gs_136961 = s_161_4;
        // N s_161_6: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_162_0: const #432u : u32
        let s_162_0: u32 = 432;
        // D s_162_1: read-reg s_162_0:u8
        let s_162_1: u8 = {
            let value = state.read_register::<u8>(s_162_0 as isize);
            tracer.read_register(s_162_0 as isize, value);
            value
        };
        // D s_162_2: call ELUsingAArch32(s_162_1)
        let s_162_2: bool = ELUsingAArch32(state, tracer, s_162_1);
        // D s_162_3: not s_162_2
        let s_162_3: bool = !s_162_2;
        // D s_162_4: write-var gs#136960 <= s_162_3
        fn_state.gs_136960 = s_162_3;
        // N s_162_5: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_163_0: const #() : ()
        let s_163_0: () = ();
        // S s_163_1: call EL2Enabled(s_163_0)
        let s_163_1: bool = EL2Enabled(state, tracer, s_163_0);
        // N s_163_2: branch s_163_1 b179 b164
        if s_163_1 {
            return block_179(state, tracer, fn_state);
        } else {
            return block_164(state, tracer, fn_state);
        };
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_164_0: const #0u : u8
        let s_164_0: bool = false;
        // D s_164_1: write-var gs#136977 <= s_164_0
        fn_state.gs_136977 = s_164_0;
        // N s_164_2: jump b165
        return block_165(state, tracer, fn_state);
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_165_0: read-var gs#136977:u8
        let s_165_0: bool = fn_state.gs_136977;
        // N s_165_1: branch s_165_0 b178 b166
        if s_165_0 {
            return block_178(state, tracer, fn_state);
        } else {
            return block_166(state, tracer, fn_state);
        };
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_166_0: const #0u : u8
        let s_166_0: bool = false;
        // D s_166_1: write-var gs#136978 <= s_166_0
        fn_state.gs_136978 = s_166_0;
        // N s_166_2: jump b167
        return block_167(state, tracer, fn_state);
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_167_0: read-var gs#136978:u8
        let s_167_0: bool = fn_state.gs_136978;
        // N s_167_1: branch s_167_0 b177 b168
        if s_167_0 {
            return block_177(state, tracer, fn_state);
        } else {
            return block_168(state, tracer, fn_state);
        };
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_168_0: const #() : ()
        let s_168_0: () = ();
        // S s_168_1: call EL2Enabled(s_168_0)
        let s_168_1: bool = EL2Enabled(state, tracer, s_168_0);
        // N s_168_2: branch s_168_1 b176 b169
        if s_168_1 {
            return block_176(state, tracer, fn_state);
        } else {
            return block_169(state, tracer, fn_state);
        };
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_169_0: const #0u : u8
        let s_169_0: bool = false;
        // D s_169_1: write-var gs#136979 <= s_169_0
        fn_state.gs_136979 = s_169_0;
        // N s_169_2: jump b170
        return block_170(state, tracer, fn_state);
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_170_0: read-var gs#136979:u8
        let s_170_0: bool = fn_state.gs_136979;
        // N s_170_1: branch s_170_0 b175 b171
        if s_170_0 {
            return block_175(state, tracer, fn_state);
        } else {
            return block_171(state, tracer, fn_state);
        };
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_171_0: const #0u : u8
        let s_171_0: bool = false;
        // D s_171_1: write-var gs#136980 <= s_171_0
        fn_state.gs_136980 = s_171_0;
        // N s_171_2: jump b172
        return block_172(state, tracer, fn_state);
    }
    fn block_172<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_172_0: read-var gs#136980:u8
        let s_172_0: bool = fn_state.gs_136980;
        // N s_172_1: branch s_172_0 b174 b173
        if s_172_0 {
            return block_174(state, tracer, fn_state);
        } else {
            return block_173(state, tracer, fn_state);
        };
    }
    fn block_173<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_173_0: panic
        panic!("{:?}", ());
        // N s_173_1: return
        return;
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_174_0: const #0u : u8
        let s_174_0: u8 = 0;
        // C s_174_1: cast zx s_174_0 -> bv
        let s_174_1: Bits = Bits::new(s_174_0 as u128, 8u16);
        // C s_174_2: cast zx s_174_1 -> i
        let s_174_2: i128 = (s_174_1.value() as i128);
        // C s_174_3: cast reint s_174_2 -> i64
        let s_174_3: i64 = (s_174_2 as i64);
        // C s_174_4: cast zx s_174_3 -> i
        let s_174_4: i128 = (i128::try_from(s_174_3).unwrap());
        // S s_174_5: call AArch32_TakeHypTrapException(s_174_4)
        let s_174_5: () = AArch32_TakeHypTrapException(state, tracer, s_174_4);
        // N s_174_6: return
        return;
    }
    fn block_175<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_175_0: const #() : ()
        let s_175_0: () = ();
        // S s_175_1: call HCR_read(s_175_0)
        let s_175_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_175_0);
        // S s_175_2: call _get_HCR_Type_TGE(s_175_1)
        let s_175_2: bool = u_get_HCR_Type_TGE(state, tracer, s_175_1);
        // S s_175_3: cast zx s_175_2 -> bv
        let s_175_3: Bits = Bits::new(s_175_2 as u128, 1u16);
        // C s_175_4: const #1u : u8
        let s_175_4: bool = true;
        // C s_175_5: cast zx s_175_4 -> bv
        let s_175_5: Bits = Bits::new(s_175_4 as u128, 1u16);
        // S s_175_6: cmp-eq s_175_3 s_175_5
        let s_175_6: bool = ((s_175_3) == (s_175_5));
        // D s_175_7: write-var gs#136980 <= s_175_6
        fn_state.gs_136980 = s_175_6;
        // N s_175_8: jump b172
        return block_172(state, tracer, fn_state);
    }
    fn block_176<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_176_0: const #432u : u32
        let s_176_0: u32 = 432;
        // D s_176_1: read-reg s_176_0:u8
        let s_176_1: u8 = {
            let value = state.read_register::<u8>(s_176_0 as isize);
            tracer.read_register(s_176_0 as isize, value);
            value
        };
        // D s_176_2: call ELUsingAArch32(s_176_1)
        let s_176_2: bool = ELUsingAArch32(state, tracer, s_176_1);
        // D s_176_3: write-var gs#136979 <= s_176_2
        fn_state.gs_136979 = s_176_2;
        // N s_176_4: jump b170
        return block_170(state, tracer, fn_state);
    }
    fn block_177<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_177_0: const #6u : u8
        let s_177_0: u8 = 6;
        // C s_177_1: cast zx s_177_0 -> bv
        let s_177_1: Bits = Bits::new(s_177_0 as u128, 8u16);
        // C s_177_2: cast zx s_177_1 -> i
        let s_177_2: i128 = (s_177_1.value() as i128);
        // C s_177_3: cast reint s_177_2 -> i64
        let s_177_3: i64 = (s_177_2 as i64);
        // C s_177_4: cast zx s_177_3 -> i
        let s_177_4: i128 = (i128::try_from(s_177_3).unwrap());
        // C s_177_5: const #432u : u32
        let s_177_5: u32 = 432;
        // D s_177_6: read-reg s_177_5:u8
        let s_177_6: u8 = {
            let value = state.read_register::<u8>(s_177_5 as isize);
            tracer.read_register(s_177_5 as isize, value);
            value
        };
        // D s_177_7: call AArch64_AArch32SystemAccessTrap(s_177_6, s_177_4)
        let s_177_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_177_6,
            s_177_4,
        );
        // N s_177_8: return
        return;
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_178_0: const #102552u : u32
        let s_178_0: u32 = 102552;
        // D s_178_1: read-reg s_178_0:struct
        let s_178_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_178_0 as isize);
            tracer.read_register(s_178_0 as isize, value);
            value
        };
        // D s_178_2: call _get_HCR_EL2_Type_TGE(s_178_1)
        let s_178_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_178_1);
        // D s_178_3: cast zx s_178_2 -> bv
        let s_178_3: Bits = Bits::new(s_178_2 as u128, 1u16);
        // C s_178_4: const #1u : u8
        let s_178_4: bool = true;
        // C s_178_5: cast zx s_178_4 -> bv
        let s_178_5: Bits = Bits::new(s_178_4 as u128, 1u16);
        // D s_178_6: cmp-eq s_178_3 s_178_5
        let s_178_6: bool = ((s_178_3) == (s_178_5));
        // D s_178_7: write-var gs#136978 <= s_178_6
        fn_state.gs_136978 = s_178_6;
        // N s_178_8: jump b167
        return block_167(state, tracer, fn_state);
    }
    fn block_179<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_179_0: const #432u : u32
        let s_179_0: u32 = 432;
        // D s_179_1: read-reg s_179_0:u8
        let s_179_1: u8 = {
            let value = state.read_register::<u8>(s_179_0 as isize);
            tracer.read_register(s_179_0 as isize, value);
            value
        };
        // D s_179_2: call ELUsingAArch32(s_179_1)
        let s_179_2: bool = ELUsingAArch32(state, tracer, s_179_1);
        // D s_179_3: not s_179_2
        let s_179_3: bool = !s_179_2;
        // D s_179_4: write-var gs#136977 <= s_179_3
        fn_state.gs_136977 = s_179_3;
        // N s_179_5: jump b165
        return block_165(state, tracer, fn_state);
    }
    fn block_180<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_180_0: read-var __DBGDSCRext_UDCCdis:u8
        let s_180_0: bool = fn_state.u__DBGDSCRext_UDCCdis;
        // D s_180_1: cast zx s_180_0 -> bv
        let s_180_1: Bits = Bits::new(s_180_0 as u128, 1u16);
        // C s_180_2: const #1u : u8
        let s_180_2: bool = true;
        // C s_180_3: cast zx s_180_2 -> bv
        let s_180_3: Bits = Bits::new(s_180_2 as u128, 1u16);
        // D s_180_4: cmp-eq s_180_1 s_180_3
        let s_180_4: bool = ((s_180_1) == (s_180_3));
        // D s_180_5: write-var gs#136959 <= s_180_4
        fn_state.gs_136959 = s_180_4;
        // N s_180_6: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_181<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_181_0: const #() : ()
        let s_181_0: () = ();
        // S s_181_1: call EL2Enabled(s_181_0)
        let s_181_1: bool = EL2Enabled(state, tracer, s_181_0);
        // N s_181_2: branch s_181_1 b189 b182
        if s_181_1 {
            return block_189(state, tracer, fn_state);
        } else {
            return block_182(state, tracer, fn_state);
        };
    }
    fn block_182<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_182_0: const #0u : u8
        let s_182_0: bool = false;
        // D s_182_1: write-var gs#136981 <= s_182_0
        fn_state.gs_136981 = s_182_0;
        // N s_182_2: jump b183
        return block_183(state, tracer, fn_state);
    }
    fn block_183<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_183_0: read-var gs#136981:u8
        let s_183_0: bool = fn_state.gs_136981;
        // N s_183_1: branch s_183_0 b188 b184
        if s_183_0 {
            return block_188(state, tracer, fn_state);
        } else {
            return block_184(state, tracer, fn_state);
        };
    }
    fn block_184<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_184_0: const #0u : u8
        let s_184_0: bool = false;
        // D s_184_1: write-var gs#136982 <= s_184_0
        fn_state.gs_136982 = s_184_0;
        // N s_184_2: jump b185
        return block_185(state, tracer, fn_state);
    }
    fn block_185<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_185_0: read-var gs#136982:u8
        let s_185_0: bool = fn_state.gs_136982;
        // N s_185_1: branch s_185_0 b187 b186
        if s_185_0 {
            return block_187(state, tracer, fn_state);
        } else {
            return block_186(state, tracer, fn_state);
        };
    }
    fn block_186<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_186_0: const #6u : u8
        let s_186_0: u8 = 6;
        // C s_186_1: cast zx s_186_0 -> bv
        let s_186_1: Bits = Bits::new(s_186_0 as u128, 8u16);
        // C s_186_2: cast zx s_186_1 -> i
        let s_186_2: i128 = (s_186_1.value() as i128);
        // C s_186_3: cast reint s_186_2 -> i64
        let s_186_3: i64 = (s_186_2 as i64);
        // C s_186_4: cast zx s_186_3 -> i
        let s_186_4: i128 = (i128::try_from(s_186_3).unwrap());
        // C s_186_5: const #440u : u32
        let s_186_5: u32 = 440;
        // D s_186_6: read-reg s_186_5:u8
        let s_186_6: u8 = {
            let value = state.read_register::<u8>(s_186_5 as isize);
            tracer.read_register(s_186_5 as isize, value);
            value
        };
        // D s_186_7: call AArch64_AArch32SystemAccessTrap(s_186_6, s_186_4)
        let s_186_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_186_6,
            s_186_4,
        );
        // N s_186_8: return
        return;
    }
    fn block_187<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_187_0: const #6u : u8
        let s_187_0: u8 = 6;
        // C s_187_1: cast zx s_187_0 -> bv
        let s_187_1: Bits = Bits::new(s_187_0 as u128, 8u16);
        // C s_187_2: cast zx s_187_1 -> i
        let s_187_2: i128 = (s_187_1.value() as i128);
        // C s_187_3: cast reint s_187_2 -> i64
        let s_187_3: i64 = (s_187_2 as i64);
        // C s_187_4: cast zx s_187_3 -> i
        let s_187_4: i128 = (i128::try_from(s_187_3).unwrap());
        // C s_187_5: const #432u : u32
        let s_187_5: u32 = 432;
        // D s_187_6: read-reg s_187_5:u8
        let s_187_6: u8 = {
            let value = state.read_register::<u8>(s_187_5 as isize);
            tracer.read_register(s_187_5 as isize, value);
            value
        };
        // D s_187_7: call AArch64_AArch32SystemAccessTrap(s_187_6, s_187_4)
        let s_187_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_187_6,
            s_187_4,
        );
        // N s_187_8: return
        return;
    }
    fn block_188<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_188_0: const #102552u : u32
        let s_188_0: u32 = 102552;
        // D s_188_1: read-reg s_188_0:struct
        let s_188_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_188_0 as isize);
            tracer.read_register(s_188_0 as isize, value);
            value
        };
        // D s_188_2: call _get_HCR_EL2_Type_TGE(s_188_1)
        let s_188_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_188_1);
        // D s_188_3: cast zx s_188_2 -> bv
        let s_188_3: Bits = Bits::new(s_188_2 as u128, 1u16);
        // C s_188_4: const #1u : u8
        let s_188_4: bool = true;
        // C s_188_5: cast zx s_188_4 -> bv
        let s_188_5: Bits = Bits::new(s_188_4 as u128, 1u16);
        // D s_188_6: cmp-eq s_188_3 s_188_5
        let s_188_6: bool = ((s_188_3) == (s_188_5));
        // D s_188_7: write-var gs#136982 <= s_188_6
        fn_state.gs_136982 = s_188_6;
        // N s_188_8: jump b185
        return block_185(state, tracer, fn_state);
    }
    fn block_189<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_189_0: const #432u : u32
        let s_189_0: u32 = 432;
        // D s_189_1: read-reg s_189_0:u8
        let s_189_1: u8 = {
            let value = state.read_register::<u8>(s_189_0 as isize);
            tracer.read_register(s_189_0 as isize, value);
            value
        };
        // D s_189_2: call ELUsingAArch32(s_189_1)
        let s_189_2: bool = ELUsingAArch32(state, tracer, s_189_1);
        // D s_189_3: not s_189_2
        let s_189_3: bool = !s_189_2;
        // D s_189_4: write-var gs#136981 <= s_189_3
        fn_state.gs_136981 = s_189_3;
        // N s_189_5: jump b183
        return block_183(state, tracer, fn_state);
    }
    fn block_190<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_190_0: read-var __MDSCR_EL1_TDCC:u8
        let s_190_0: bool = fn_state.u__MDSCR_EL1_TDCC;
        // D s_190_1: cast zx s_190_0 -> bv
        let s_190_1: Bits = Bits::new(s_190_0 as u128, 1u16);
        // C s_190_2: const #1u : u8
        let s_190_2: bool = true;
        // C s_190_3: cast zx s_190_2 -> bv
        let s_190_3: Bits = Bits::new(s_190_2 as u128, 1u16);
        // D s_190_4: cmp-eq s_190_1 s_190_3
        let s_190_4: bool = ((s_190_1) == (s_190_3));
        // D s_190_5: write-var gs#136958 <= s_190_4
        fn_state.gs_136958 = s_190_4;
        // N s_190_6: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_191<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_191_0: const #() : ()
        let s_191_0: () = ();
        // S s_191_1: call DBGDTRRXint_read(s_191_0)
        let s_191_1: ProductType700c18a878c5601b = DBGDTRRXint_read(
            state,
            tracer,
            s_191_0,
        );
        // D s_191_2: write-var ga#238555 <= s_191_1
        fn_state.ga_238555 = s_191_1;
        // D s_191_3: read-var ga#238555.0:struct
        let s_191_3: u32 = fn_state.ga_238555._0;
        // C s_191_4: const #4s : i
        let s_191_4: i128 = 4;
        // D s_191_5: cast zx s_191_3 -> bv
        let s_191_5: Bits = Bits::new(s_191_3 as u128, 32u16);
        // D s_191_6: read-var address:u32
        let s_191_6: u32 = fn_state.address;
        // D s_191_7: call MemA_set(s_191_6, s_191_4, s_191_5)
        let s_191_7: () = MemA_set(state, tracer, s_191_6, s_191_4, s_191_5);
        // N s_191_8: return
        return;
    }
}
