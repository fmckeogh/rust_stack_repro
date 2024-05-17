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
use u_update_DBGDSCRint_Type_RXfull::*;
use EDSCR_write::*;
use IsInHost::*;
use DBGDSCRext_read::*;
use CacheConfigRead::*;
use DBGDSCRext_write::*;
use getISR::*;
use MPAM3_EL3_read::*;
use GetNumEventCounters::*;
use HaveRME::*;
use u_get_ID_AA64PFR0_EL1_Type_EL3::*;
use Zeros::*;
use genRandomNum::*;
use AArch64_SystemAccessTrap::*;
use UsePrimarySpaceEL10::*;
use u_get_ERRIDR_EL1_Type_NUM::*;
use u_get_AMCGCR_EL0_Type_CG1NC::*;
use u_get_AMCGCR_EL0_Type_CG0NC::*;
use u_get_HCR_EL2_Type_TDZ::*;
use PMUCounterMask::*;
use EL2Enabled::*;
use u_update_DBGDSCRext_Type_RXfull::*;
use u_update_EDSCR_Type_RXfull::*;
use u_get_MPAMIDR_EL1_Type_HAS_ALTSP::*;
use u_get_BRBFCR_EL1_Type_BANK::*;
use u_get_ERRSELR_EL1_Type_SEL::*;
use ExternalInvasiveDebugEnabled::*;
use ExternalSecureInvasiveDebugEnabled::*;
use SPE_PMBIDR_P_Read::*;
use X_read::*;
use GetBRBENumRecords::*;
use u_get_SCTLR_EL1_Type_DZE::*;
use CurrentSecurityState::*;
use u_get_MPAM3_EL3_Type_FORCE_NS::*;
use u_get_PMSELR_EL0_Type_SEL::*;
use AArch64_AutoGen_SysRegRead::*;
use X_set::*;
use UsePrimarySpaceEL2::*;
use u__id::*;
use DBGDSCRint_write::*;
use ELUsingAArch32::*;
use integer_subrange::*;
use AArch64_GetNumEventCountersAccessible::*;
use AArch64_CheckNVCondsIfCurrentEL::*;
use DBGDSCRint_read::*;
use u_get_SCTLR_EL2_Type_DZE::*;
use u_get_PMUSERENR_EL0_Type_EN::*;
use u_get_PMUSERENR_EL0_Type_ER::*;
use integer_access::*;
use EDSCR_read::*;
use common::*;
pub fn AArch64_SysRegRead<T: Tracer>(
    state: &mut State,
    tracer: &T,
    op0: i128,
    op1: i128,
    crn: i128,
    crm: i128,
    op2: i128,
    t: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_140698: bool,
        gs_140961: bool,
        temp: u64,
        gs_140582: bool,
        gs_140650: bool,
        gs_140411: bool,
        gs_140625: bool,
        gs_140546: bool,
        gs_140617: bool,
        gs_140539: bool,
        gs_140680: bool,
        gs_140583: bool,
        gs_140562: bool,
        gs_140685: bool,
        ga_247142: u64,
        gs_141009: bool,
        gs_140995: bool,
        ga_247123: bool,
        ga_246950: ProductType5c790c8ef59cc8b2,
        gs_140990: bool,
        gs_141011: bool,
        gs_140455: bool,
        gs_140639: bool,
        gs_140721: bool,
        gs_140960: bool,
        gs_140388: bool,
        gs_140661: bool,
        gs_140716: bool,
        gs_140969: bool,
        gs_140665: bool,
        gs_140604: bool,
        gs_140530: bool,
        gs_140497: bool,
        gs_140734: bool,
        gs_140847: bool,
        gs_140428: bool,
        ga_247133: bool,
        gs_140659: bool,
        gs_140503: bool,
        gs_140536: bool,
        gs_140648: bool,
        gs_140390: bool,
        gs_140669: bool,
        gs_140427: bool,
        gs_140860: bool,
        gs_140968: bool,
        gs_140445: bool,
        ga_247132: u64,
        gs_140683: bool,
        gs_140692: bool,
        gs_140446: bool,
        ga_247229: ProductType5c790c8ef59cc8b2,
        gs_140515: bool,
        gs_141039: bool,
        ga_247193: ProductType5c790c8ef59cc8b2,
        gs_140637: bool,
        gs_140431: bool,
        gs_140994: bool,
        ga_246869: bool,
        gs_140425: bool,
        gs_140703: bool,
        gs_140962: bool,
        gs_140596: bool,
        gs_140430: bool,
        gs_140543: bool,
        gs_140519: bool,
        gs_140628: bool,
        gs_140732: bool,
        gs_140598: bool,
        gs_140426: bool,
        gs_140660: bool,
        gs_140645: bool,
        gs_140584: bool,
        gs_140754: bool,
        gs_140591: bool,
        gs_140547: bool,
        gs_140623: bool,
        gs_140406: bool,
        gs_140658: bool,
        gs_140657: bool,
        gs_140401: bool,
        gs_140495: bool,
        gs_140449: bool,
        gs_140417: bool,
        gs_141007: bool,
        gs_140594: bool,
        gs_140992: bool,
        gs_140742: bool,
        gs_140619: bool,
        nshadow_1007: i64,
        gs_141032: bool,
        gs_140886: bool,
        ga_247143: bool,
        gs_140535: bool,
        gs_140701: bool,
        gs_140687: bool,
        gs_140531: bool,
        gs_140551: bool,
        gs_140881: bool,
        gs_140844: bool,
        gs_140678: bool,
        gs_141008: bool,
        ga_246884: bool,
        gs_140521: bool,
        gs_140662: bool,
        gs_140409: bool,
        gs_140723: bool,
        gs_140589: bool,
        gs_140559: bool,
        gs_140634: bool,
        gs_140854: bool,
        gs_140632: bool,
        gs_140622: bool,
        gs_140510: bool,
        gs_140641: bool,
        gs_140712: bool,
        gs_140602: bool,
        gs_140413: bool,
        ga_247159: u64,
        gs_140728: bool,
        gs_140671: bool,
        gs_140558: bool,
        gs_140887: bool,
        ga_247184: ProductType5c790c8ef59cc8b2,
        gs_140674: bool,
        gs_140676: bool,
        gs_140400: bool,
        gs_140456: bool,
        gs_140719: bool,
        gs_140564: bool,
        gs_140694: bool,
        gs_140473: bool,
        ga_246883: u64,
        gs_140442: bool,
        gs_140524: bool,
        gs_140573: bool,
        gs_140612: bool,
        gs_140418: bool,
        gs_140517: bool,
        gs_140725: bool,
        ga_246868: u64,
        gs_140508: bool,
        gs_140538: bool,
        gs_140548: bool,
        gs_140601: bool,
        gs_140553: bool,
        gs_140850: bool,
        gs_140448: bool,
        gs_140710: bool,
        gs_140823: bool,
        gs_140991: bool,
        gs_140644: bool,
        gs_140714: bool,
        ga_247202: ProductType5c790c8ef59cc8b2,
        gs_140568: bool,
        gs_140506: bool,
        gs_140395: bool,
        gs_140499: bool,
        gs_140630: bool,
        gs_140696: bool,
        gs_140416: bool,
        gs_140689: bool,
        gs_140502: bool,
        gs_140707: bool,
        gs_140730: bool,
        gs_140705: bool,
        gs_140512: bool,
        gs_140606: bool,
        gs_140610: bool,
        recordIdx: i64,
        gs_140447: bool,
        gs_140608: bool,
        indexshadow_1006: i64,
        gs_140422: bool,
        gs_140527: bool,
        ga_247122: u64,
        gs_140393: bool,
        gs_140555: bool,
        gs_140614: bool,
        gs_140827: bool,
        gs_140587: bool,
        gs_140571: bool,
        gs_140479: bool,
        gs_140667: bool,
        gs_140566: bool,
        gs_140424: bool,
        gs_140544: bool,
        gs_140866: bool,
        op0: i128,
        op1: i128,
        crn: i128,
        crm: i128,
        op2: i128,
        t: i128,
    }
    let fn_state = FunctionState {
        op0,
        op1,
        crn,
        crm,
        op2,
        t,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #3s : i
        let s_0_0: i128 = 3;
        // D s_0_1: read-var op0:i
        let s_0_1: i128 = fn_state.op0;
        // D s_0_2: cmp-eq s_0_1 s_0_0
        let s_0_2: bool = ((s_0_1) == (s_0_0));
        // N s_0_3: branch s_0_2 b692 b1
        if s_0_2 {
            return block_692(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#140388 <= s_1_0
        fn_state.gs_140388 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#140388:u8
        let s_2_0: bool = fn_state.gs_140388;
        // N s_2_1: branch s_2_0 b691 b3
        if s_2_0 {
            return block_691(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#140390 <= s_3_0
        fn_state.gs_140390 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#140390:u8
        let s_4_0: bool = fn_state.gs_140390;
        // N s_4_1: branch s_4_0 b675 b5
        if s_4_0 {
            return block_675(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #3s : i
        let s_6_0: i128 = 3;
        // D s_6_1: read-var op0:i
        let s_6_1: i128 = fn_state.op0;
        // D s_6_2: cmp-eq s_6_1 s_6_0
        let s_6_2: bool = ((s_6_1) == (s_6_0));
        // N s_6_3: branch s_6_2 b674 b7
        if s_6_2 {
            return block_674(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#140393 <= s_7_0
        fn_state.gs_140393 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#140393:u8
        let s_8_0: bool = fn_state.gs_140393;
        // N s_8_1: branch s_8_0 b673 b9
        if s_8_0 {
            return block_673(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#140395 <= s_9_0
        fn_state.gs_140395 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#140395:u8
        let s_10_0: bool = fn_state.gs_140395;
        // N s_10_1: branch s_10_0 b669 b11
        if s_10_0 {
            return block_669(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#140401 <= s_11_0
        fn_state.gs_140401 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#140401:u8
        let s_12_0: bool = fn_state.gs_140401;
        // N s_12_1: branch s_12_0 b668 b13
        if s_12_0 {
            return block_668(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#140406 <= s_13_0
        fn_state.gs_140406 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#140406:u8
        let s_14_0: bool = fn_state.gs_140406;
        // N s_14_1: branch s_14_0 b638 b15
        if s_14_0 {
            return block_638(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #3s : i
        let s_16_0: i128 = 3;
        // D s_16_1: read-var op0:i
        let s_16_1: i128 = fn_state.op0;
        // D s_16_2: cmp-eq s_16_1 s_16_0
        let s_16_2: bool = ((s_16_1) == (s_16_0));
        // N s_16_3: branch s_16_2 b637 b17
        if s_16_2 {
            return block_637(state, tracer, fn_state);
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
        // D s_17_1: write-var gs#140409 <= s_17_0
        fn_state.gs_140409 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#140409:u8
        let s_18_0: bool = fn_state.gs_140409;
        // N s_18_1: branch s_18_0 b636 b19
        if s_18_0 {
            return block_636(state, tracer, fn_state);
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
        // D s_19_1: write-var gs#140411 <= s_19_0
        fn_state.gs_140411 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#140411:u8
        let s_20_0: bool = fn_state.gs_140411;
        // N s_20_1: branch s_20_0 b635 b21
        if s_20_0 {
            return block_635(state, tracer, fn_state);
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
        // D s_21_1: write-var gs#140413 <= s_21_0
        fn_state.gs_140413 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#140413:u8
        let s_22_0: bool = fn_state.gs_140413;
        // N s_22_1: branch s_22_0 b631 b23
        if s_22_0 {
            return block_631(state, tracer, fn_state);
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
        // D s_23_1: write-var gs#140417 <= s_23_0
        fn_state.gs_140417 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#140417:u8
        let s_24_0: bool = fn_state.gs_140417;
        // N s_24_1: branch s_24_0 b630 b25
        if s_24_0 {
            return block_630(state, tracer, fn_state);
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
        // D s_25_1: write-var gs#140418 <= s_25_0
        fn_state.gs_140418 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#140418:u8
        let s_26_0: bool = fn_state.gs_140418;
        // N s_26_1: branch s_26_0 b600 b27
        if s_26_0 {
            return block_600(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_27_0: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #64s : i64
        let s_28_0: i64 = 64;
        // D s_28_1: read-var t:i
        let s_28_1: i128 = fn_state.t;
        // D s_28_2: call X_read(s_28_1, s_28_0)
        let s_28_2: Bits = X_read(state, tracer, s_28_1, s_28_0);
        // D s_28_3: cast reint s_28_2 -> u64
        let s_28_3: u64 = (s_28_2.value() as u64);
        // D s_28_4: write-var temp <= s_28_3
        fn_state.temp = s_28_3;
        // C s_28_5: const #16975u : u32
        let s_28_5: u32 = 16975;
        // D s_28_6: read-reg s_28_5:u8
        let s_28_6: u8 = {
            let value = state.read_register::<u8>(s_28_5 as isize);
            tracer.read_register(s_28_5 as isize, value);
            value
        };
        // C s_28_7: const #1s : i
        let s_28_7: i128 = 1;
        // C s_28_8: const #0s : i
        let s_28_8: i128 = 0;
        // D s_28_9: read-var op0:i
        let s_28_9: i128 = fn_state.op0;
        // D s_28_10: call integer_subrange(s_28_9, s_28_7, s_28_8)
        let s_28_10: Bits = integer_subrange(state, tracer, s_28_9, s_28_7, s_28_8);
        // D s_28_11: cast reint s_28_10 -> u8
        let s_28_11: u8 = (s_28_10.value() as u8);
        // C s_28_12: const #2s : i
        let s_28_12: i128 = 2;
        // C s_28_13: const #0s : i
        let s_28_13: i128 = 0;
        // D s_28_14: read-var op1:i
        let s_28_14: i128 = fn_state.op1;
        // D s_28_15: call integer_subrange(s_28_14, s_28_12, s_28_13)
        let s_28_15: Bits = integer_subrange(state, tracer, s_28_14, s_28_12, s_28_13);
        // D s_28_16: cast reint s_28_15 -> u8
        let s_28_16: u8 = (s_28_15.value() as u8);
        // C s_28_17: const #3s : i
        let s_28_17: i128 = 3;
        // C s_28_18: const #0s : i
        let s_28_18: i128 = 0;
        // D s_28_19: read-var crn:i
        let s_28_19: i128 = fn_state.crn;
        // D s_28_20: call integer_subrange(s_28_19, s_28_17, s_28_18)
        let s_28_20: Bits = integer_subrange(state, tracer, s_28_19, s_28_17, s_28_18);
        // D s_28_21: cast reint s_28_20 -> u8
        let s_28_21: u8 = (s_28_20.value() as u8);
        // C s_28_22: const #2s : i
        let s_28_22: i128 = 2;
        // C s_28_23: const #0s : i
        let s_28_23: i128 = 0;
        // D s_28_24: read-var op2:i
        let s_28_24: i128 = fn_state.op2;
        // D s_28_25: call integer_subrange(s_28_24, s_28_22, s_28_23)
        let s_28_25: Bits = integer_subrange(state, tracer, s_28_24, s_28_22, s_28_23);
        // D s_28_26: cast reint s_28_25 -> u8
        let s_28_26: u8 = (s_28_25.value() as u8);
        // C s_28_27: const #3s : i
        let s_28_27: i128 = 3;
        // C s_28_28: const #0s : i
        let s_28_28: i128 = 0;
        // D s_28_29: read-var crm:i
        let s_28_29: i128 = fn_state.crm;
        // D s_28_30: call integer_subrange(s_28_29, s_28_27, s_28_28)
        let s_28_30: Bits = integer_subrange(state, tracer, s_28_29, s_28_27, s_28_28);
        // D s_28_31: cast reint s_28_30 -> u8
        let s_28_31: u8 = (s_28_30.value() as u8);
        // D s_28_32: read-var t:i
        let s_28_32: i128 = fn_state.t;
        // D s_28_33: call AArch64_AutoGen_SysRegRead(s_28_6, s_28_11, s_28_16, s_28_21, s_28_26, s_28_31, s_28_32)
        let s_28_33: () = AArch64_AutoGen_SysRegRead(
            state,
            tracer,
            s_28_6,
            s_28_11,
            s_28_16,
            s_28_21,
            s_28_26,
            s_28_31,
            s_28_32,
        );
        // N s_28_34: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #3s : i
        let s_29_0: i128 = 3;
        // D s_29_1: read-var op0:i
        let s_29_1: i128 = fn_state.op0;
        // D s_29_2: cmp-eq s_29_1 s_29_0
        let s_29_2: bool = ((s_29_1) == (s_29_0));
        // N s_29_3: branch s_29_2 b599 b30
        if s_29_2 {
            return block_599(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #0u : u8
        let s_30_0: bool = false;
        // D s_30_1: write-var gs#140495 <= s_30_0
        fn_state.gs_140495 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#140495:u8
        let s_31_0: bool = fn_state.gs_140495;
        // N s_31_1: branch s_31_0 b598 b32
        if s_31_0 {
            return block_598(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #0u : u8
        let s_32_0: bool = false;
        // D s_32_1: write-var gs#140497 <= s_32_0
        fn_state.gs_140497 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#140497:u8
        let s_33_0: bool = fn_state.gs_140497;
        // N s_33_1: branch s_33_0 b597 b34
        if s_33_0 {
            return block_597(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #0u : u8
        let s_34_0: bool = false;
        // D s_34_1: write-var gs#140499 <= s_34_0
        fn_state.gs_140499 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#140499:u8
        let s_35_0: bool = fn_state.gs_140499;
        // N s_35_1: branch s_35_0 b593 b36
        if s_35_0 {
            return block_593(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #0u : u8
        let s_36_0: bool = false;
        // D s_36_1: write-var gs#140503 <= s_36_0
        fn_state.gs_140503 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#140503:u8
        let s_37_0: bool = fn_state.gs_140503;
        // N s_37_1: branch s_37_0 b583 b38
        if s_37_0 {
            return block_583(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_38_0: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #3s : i
        let s_39_0: i128 = 3;
        // D s_39_1: read-var op0:i
        let s_39_1: i128 = fn_state.op0;
        // D s_39_2: cmp-eq s_39_1 s_39_0
        let s_39_2: bool = ((s_39_1) == (s_39_0));
        // N s_39_3: branch s_39_2 b582 b40
        if s_39_2 {
            return block_582(state, tracer, fn_state);
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
        // D s_40_1: write-var gs#140506 <= s_40_0
        fn_state.gs_140506 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#140506:u8
        let s_41_0: bool = fn_state.gs_140506;
        // N s_41_1: branch s_41_0 b581 b42
        if s_41_0 {
            return block_581(state, tracer, fn_state);
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
        // D s_42_1: write-var gs#140508 <= s_42_0
        fn_state.gs_140508 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#140508:u8
        let s_43_0: bool = fn_state.gs_140508;
        // N s_43_1: branch s_43_0 b580 b44
        if s_43_0 {
            return block_580(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #0u : u8
        let s_44_0: bool = false;
        // D s_44_1: write-var gs#140510 <= s_44_0
        fn_state.gs_140510 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#140510:u8
        let s_45_0: bool = fn_state.gs_140510;
        // N s_45_1: branch s_45_0 b579 b46
        if s_45_0 {
            return block_579(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #0u : u8
        let s_46_0: bool = false;
        // D s_46_1: write-var gs#140512 <= s_46_0
        fn_state.gs_140512 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#140512:u8
        let s_47_0: bool = fn_state.gs_140512;
        // N s_47_1: branch s_47_0 b569 b48
        if s_47_0 {
            return block_569(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_48_0: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #3s : i
        let s_49_0: i128 = 3;
        // D s_49_1: read-var op0:i
        let s_49_1: i128 = fn_state.op0;
        // D s_49_2: cmp-eq s_49_1 s_49_0
        let s_49_2: bool = ((s_49_1) == (s_49_0));
        // N s_49_3: branch s_49_2 b568 b50
        if s_49_2 {
            return block_568(state, tracer, fn_state);
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
        // D s_50_1: write-var gs#140515 <= s_50_0
        fn_state.gs_140515 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#140515:u8
        let s_51_0: bool = fn_state.gs_140515;
        // N s_51_1: branch s_51_0 b567 b52
        if s_51_0 {
            return block_567(state, tracer, fn_state);
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
        // D s_52_1: write-var gs#140517 <= s_52_0
        fn_state.gs_140517 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#140517:u8
        let s_53_0: bool = fn_state.gs_140517;
        // N s_53_1: branch s_53_0 b566 b54
        if s_53_0 {
            return block_566(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #0u : u8
        let s_54_0: bool = false;
        // D s_54_1: write-var gs#140519 <= s_54_0
        fn_state.gs_140519 = s_54_0;
        // N s_54_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var gs#140519:u8
        let s_55_0: bool = fn_state.gs_140519;
        // N s_55_1: branch s_55_0 b565 b56
        if s_55_0 {
            return block_565(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #0u : u8
        let s_56_0: bool = false;
        // D s_56_1: write-var gs#140521 <= s_56_0
        fn_state.gs_140521 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#140521:u8
        let s_57_0: bool = fn_state.gs_140521;
        // N s_57_1: branch s_57_0 b564 b58
        if s_57_0 {
            return block_564(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_58_0: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #3s : i
        let s_59_0: i128 = 3;
        // D s_59_1: read-var op0:i
        let s_59_1: i128 = fn_state.op0;
        // D s_59_2: cmp-eq s_59_1 s_59_0
        let s_59_2: bool = ((s_59_1) == (s_59_0));
        // N s_59_3: branch s_59_2 b563 b60
        if s_59_2 {
            return block_563(state, tracer, fn_state);
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
        // D s_60_1: write-var gs#140524 <= s_60_0
        fn_state.gs_140524 = s_60_0;
        // N s_60_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var gs#140524:u8
        let s_61_0: bool = fn_state.gs_140524;
        // N s_61_1: branch s_61_0 b529 b62
        if s_61_0 {
            return block_529(state, tracer, fn_state);
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
        // D s_62_1: write-var gs#140548 <= s_62_0
        fn_state.gs_140548 = s_62_0;
        // N s_62_2: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var gs#140548:u8
        let s_63_0: bool = fn_state.gs_140548;
        // N s_63_1: branch s_63_0 b528 b64
        if s_63_0 {
            return block_528(state, tracer, fn_state);
        } else {
            return block_64(state, tracer, fn_state);
        };
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_64_0: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #3s : i
        let s_65_0: i128 = 3;
        // D s_65_1: read-var op0:i
        let s_65_1: i128 = fn_state.op0;
        // D s_65_2: cmp-eq s_65_1 s_65_0
        let s_65_2: bool = ((s_65_1) == (s_65_0));
        // N s_65_3: branch s_65_2 b527 b66
        if s_65_2 {
            return block_527(state, tracer, fn_state);
        } else {
            return block_66(state, tracer, fn_state);
        };
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #0u : u8
        let s_66_0: bool = false;
        // D s_66_1: write-var gs#140551 <= s_66_0
        fn_state.gs_140551 = s_66_0;
        // N s_66_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var gs#140551:u8
        let s_67_0: bool = fn_state.gs_140551;
        // N s_67_1: branch s_67_0 b526 b68
        if s_67_0 {
            return block_526(state, tracer, fn_state);
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
        // D s_68_1: write-var gs#140553 <= s_68_0
        fn_state.gs_140553 = s_68_0;
        // N s_68_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var gs#140553:u8
        let s_69_0: bool = fn_state.gs_140553;
        // N s_69_1: branch s_69_0 b525 b70
        if s_69_0 {
            return block_525(state, tracer, fn_state);
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
        // D s_70_1: write-var gs#140555 <= s_70_0
        fn_state.gs_140555 = s_70_0;
        // N s_70_2: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var gs#140555:u8
        let s_71_0: bool = fn_state.gs_140555;
        // N s_71_1: branch s_71_0 b521 b72
        if s_71_0 {
            return block_521(state, tracer, fn_state);
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
        // D s_72_1: write-var gs#140559 <= s_72_0
        fn_state.gs_140559 = s_72_0;
        // N s_72_2: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var gs#140559:u8
        let s_73_0: bool = fn_state.gs_140559;
        // N s_73_1: branch s_73_0 b477 b74
        if s_73_0 {
            return block_477(state, tracer, fn_state);
        } else {
            return block_74(state, tracer, fn_state);
        };
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_74_0: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #3s : i
        let s_75_0: i128 = 3;
        // D s_75_1: read-var op0:i
        let s_75_1: i128 = fn_state.op0;
        // D s_75_2: cmp-eq s_75_1 s_75_0
        let s_75_2: bool = ((s_75_1) == (s_75_0));
        // N s_75_3: branch s_75_2 b476 b76
        if s_75_2 {
            return block_476(state, tracer, fn_state);
        } else {
            return block_76(state, tracer, fn_state);
        };
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #0u : u8
        let s_76_0: bool = false;
        // D s_76_1: write-var gs#140562 <= s_76_0
        fn_state.gs_140562 = s_76_0;
        // N s_76_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var gs#140562:u8
        let s_77_0: bool = fn_state.gs_140562;
        // N s_77_1: branch s_77_0 b475 b78
        if s_77_0 {
            return block_475(state, tracer, fn_state);
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
        // D s_78_1: write-var gs#140564 <= s_78_0
        fn_state.gs_140564 = s_78_0;
        // N s_78_2: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var gs#140564:u8
        let s_79_0: bool = fn_state.gs_140564;
        // N s_79_1: branch s_79_0 b474 b80
        if s_79_0 {
            return block_474(state, tracer, fn_state);
        } else {
            return block_80(state, tracer, fn_state);
        };
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #0u : u8
        let s_80_0: bool = false;
        // D s_80_1: write-var gs#140566 <= s_80_0
        fn_state.gs_140566 = s_80_0;
        // N s_80_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var gs#140566:u8
        let s_81_0: bool = fn_state.gs_140566;
        // N s_81_1: branch s_81_0 b473 b82
        if s_81_0 {
            return block_473(state, tracer, fn_state);
        } else {
            return block_82(state, tracer, fn_state);
        };
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #0u : u8
        let s_82_0: bool = false;
        // D s_82_1: write-var gs#140568 <= s_82_0
        fn_state.gs_140568 = s_82_0;
        // N s_82_2: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_83_0: read-var gs#140568:u8
        let s_83_0: bool = fn_state.gs_140568;
        // N s_83_1: branch s_83_0 b472 b84
        if s_83_0 {
            return block_472(state, tracer, fn_state);
        } else {
            return block_84(state, tracer, fn_state);
        };
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_84_0: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #3s : i
        let s_85_0: i128 = 3;
        // D s_85_1: read-var op0:i
        let s_85_1: i128 = fn_state.op0;
        // D s_85_2: cmp-eq s_85_1 s_85_0
        let s_85_2: bool = ((s_85_1) == (s_85_0));
        // N s_85_3: branch s_85_2 b471 b86
        if s_85_2 {
            return block_471(state, tracer, fn_state);
        } else {
            return block_86(state, tracer, fn_state);
        };
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #0u : u8
        let s_86_0: bool = false;
        // D s_86_1: write-var gs#140571 <= s_86_0
        fn_state.gs_140571 = s_86_0;
        // N s_86_2: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_87_0: read-var gs#140571:u8
        let s_87_0: bool = fn_state.gs_140571;
        // N s_87_1: branch s_87_0 b470 b88
        if s_87_0 {
            return block_470(state, tracer, fn_state);
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
        // D s_88_1: write-var gs#140573 <= s_88_0
        fn_state.gs_140573 = s_88_0;
        // N s_88_2: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_89_0: read-var gs#140573:u8
        let s_89_0: bool = fn_state.gs_140573;
        // N s_89_1: branch s_89_0 b463 b90
        if s_89_0 {
            return block_463(state, tracer, fn_state);
        } else {
            return block_90(state, tracer, fn_state);
        };
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #0u : u8
        let s_90_0: bool = false;
        // D s_90_1: write-var gs#140584 <= s_90_0
        fn_state.gs_140584 = s_90_0;
        // N s_90_2: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_91_0: read-var gs#140584:u8
        let s_91_0: bool = fn_state.gs_140584;
        // N s_91_1: branch s_91_0 b442 b92
        if s_91_0 {
            return block_442(state, tracer, fn_state);
        } else {
            return block_92(state, tracer, fn_state);
        };
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_92_0: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #2s : i
        let s_93_0: i128 = 2;
        // D s_93_1: read-var op0:i
        let s_93_1: i128 = fn_state.op0;
        // D s_93_2: cmp-eq s_93_1 s_93_0
        let s_93_2: bool = ((s_93_1) == (s_93_0));
        // N s_93_3: branch s_93_2 b441 b94
        if s_93_2 {
            return block_441(state, tracer, fn_state);
        } else {
            return block_94(state, tracer, fn_state);
        };
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #0u : u8
        let s_94_0: bool = false;
        // D s_94_1: write-var gs#140587 <= s_94_0
        fn_state.gs_140587 = s_94_0;
        // N s_94_2: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_95_0: read-var gs#140587:u8
        let s_95_0: bool = fn_state.gs_140587;
        // N s_95_1: branch s_95_0 b440 b96
        if s_95_0 {
            return block_440(state, tracer, fn_state);
        } else {
            return block_96(state, tracer, fn_state);
        };
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_96_0: const #0u : u8
        let s_96_0: bool = false;
        // D s_96_1: write-var gs#140589 <= s_96_0
        fn_state.gs_140589 = s_96_0;
        // N s_96_2: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_97_0: read-var gs#140589:u8
        let s_97_0: bool = fn_state.gs_140589;
        // N s_97_1: branch s_97_0 b439 b98
        if s_97_0 {
            return block_439(state, tracer, fn_state);
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
        // D s_98_1: write-var gs#140591 <= s_98_0
        fn_state.gs_140591 = s_98_0;
        // N s_98_2: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_99_0: read-var gs#140591:u8
        let s_99_0: bool = fn_state.gs_140591;
        // N s_99_1: branch s_99_0 b434 b100
        if s_99_0 {
            return block_434(state, tracer, fn_state);
        } else {
            return block_100(state, tracer, fn_state);
        };
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_100_0: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_101_0: const #2s : i
        let s_101_0: i128 = 2;
        // D s_101_1: read-var op0:i
        let s_101_1: i128 = fn_state.op0;
        // D s_101_2: cmp-eq s_101_1 s_101_0
        let s_101_2: bool = ((s_101_1) == (s_101_0));
        // N s_101_3: branch s_101_2 b433 b102
        if s_101_2 {
            return block_433(state, tracer, fn_state);
        } else {
            return block_102(state, tracer, fn_state);
        };
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_102_0: const #0u : u8
        let s_102_0: bool = false;
        // D s_102_1: write-var gs#140594 <= s_102_0
        fn_state.gs_140594 = s_102_0;
        // N s_102_2: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_103_0: read-var gs#140594:u8
        let s_103_0: bool = fn_state.gs_140594;
        // N s_103_1: branch s_103_0 b432 b104
        if s_103_0 {
            return block_432(state, tracer, fn_state);
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
        // D s_104_1: write-var gs#140596 <= s_104_0
        fn_state.gs_140596 = s_104_0;
        // N s_104_2: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_105_0: read-var gs#140596:u8
        let s_105_0: bool = fn_state.gs_140596;
        // N s_105_1: branch s_105_0 b431 b106
        if s_105_0 {
            return block_431(state, tracer, fn_state);
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
        // D s_106_1: write-var gs#140598 <= s_106_0
        fn_state.gs_140598 = s_106_0;
        // N s_106_2: jump b107
        return block_107(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_107_0: read-var gs#140598:u8
        let s_107_0: bool = fn_state.gs_140598;
        // N s_107_1: branch s_107_0 b426 b108
        if s_107_0 {
            return block_426(state, tracer, fn_state);
        } else {
            return block_108(state, tracer, fn_state);
        };
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_108_0: jump b109
        return block_109(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #3s : i
        let s_109_0: i128 = 3;
        // D s_109_1: read-var op0:i
        let s_109_1: i128 = fn_state.op0;
        // D s_109_2: cmp-eq s_109_1 s_109_0
        let s_109_2: bool = ((s_109_1) == (s_109_0));
        // N s_109_3: branch s_109_2 b425 b110
        if s_109_2 {
            return block_425(state, tracer, fn_state);
        } else {
            return block_110(state, tracer, fn_state);
        };
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_110_0: const #0u : u8
        let s_110_0: bool = false;
        // D s_110_1: write-var gs#140601 <= s_110_0
        fn_state.gs_140601 = s_110_0;
        // N s_110_2: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_111_0: read-var gs#140601:u8
        let s_111_0: bool = fn_state.gs_140601;
        // N s_111_1: branch s_111_0 b424 b112
        if s_111_0 {
            return block_424(state, tracer, fn_state);
        } else {
            return block_112(state, tracer, fn_state);
        };
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_112_0: const #0u : u8
        let s_112_0: bool = false;
        // D s_112_1: write-var gs#140602 <= s_112_0
        fn_state.gs_140602 = s_112_0;
        // N s_112_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_113_0: read-var gs#140602:u8
        let s_113_0: bool = fn_state.gs_140602;
        // N s_113_1: branch s_113_0 b423 b114
        if s_113_0 {
            return block_423(state, tracer, fn_state);
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
        // D s_114_1: write-var gs#140604 <= s_114_0
        fn_state.gs_140604 = s_114_0;
        // N s_114_2: jump b115
        return block_115(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_115_0: read-var gs#140604:u8
        let s_115_0: bool = fn_state.gs_140604;
        // N s_115_1: branch s_115_0 b422 b116
        if s_115_0 {
            return block_422(state, tracer, fn_state);
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
        // D s_116_1: write-var gs#140606 <= s_116_0
        fn_state.gs_140606 = s_116_0;
        // N s_116_2: jump b117
        return block_117(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_117_0: read-var gs#140606:u8
        let s_117_0: bool = fn_state.gs_140606;
        // N s_117_1: branch s_117_0 b419 b118
        if s_117_0 {
            return block_419(state, tracer, fn_state);
        } else {
            return block_118(state, tracer, fn_state);
        };
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #3s : i
        let s_118_0: i128 = 3;
        // D s_118_1: read-var op0:i
        let s_118_1: i128 = fn_state.op0;
        // D s_118_2: cmp-eq s_118_1 s_118_0
        let s_118_2: bool = ((s_118_1) == (s_118_0));
        // N s_118_3: branch s_118_2 b418 b119
        if s_118_2 {
            return block_418(state, tracer, fn_state);
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
        // D s_119_1: write-var gs#140608 <= s_119_0
        fn_state.gs_140608 = s_119_0;
        // N s_119_2: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_120_0: read-var gs#140608:u8
        let s_120_0: bool = fn_state.gs_140608;
        // N s_120_1: branch s_120_0 b417 b121
        if s_120_0 {
            return block_417(state, tracer, fn_state);
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
        // D s_121_1: write-var gs#140610 <= s_121_0
        fn_state.gs_140610 = s_121_0;
        // N s_121_2: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_122_0: read-var gs#140610:u8
        let s_122_0: bool = fn_state.gs_140610;
        // N s_122_1: branch s_122_0 b416 b123
        if s_122_0 {
            return block_416(state, tracer, fn_state);
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
        // D s_123_1: write-var gs#140612 <= s_123_0
        fn_state.gs_140612 = s_123_0;
        // N s_123_2: jump b124
        return block_124(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_124_0: read-var gs#140612:u8
        let s_124_0: bool = fn_state.gs_140612;
        // N s_124_1: branch s_124_0 b415 b125
        if s_124_0 {
            return block_415(state, tracer, fn_state);
        } else {
            return block_125(state, tracer, fn_state);
        };
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_125_0: const #0u : u8
        let s_125_0: bool = false;
        // D s_125_1: write-var gs#140614 <= s_125_0
        fn_state.gs_140614 = s_125_0;
        // N s_125_2: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_126_0: read-var gs#140614:u8
        let s_126_0: bool = fn_state.gs_140614;
        // N s_126_1: branch s_126_0 b414 b127
        if s_126_0 {
            return block_414(state, tracer, fn_state);
        } else {
            return block_127(state, tracer, fn_state);
        };
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_127_0: const #3s : i
        let s_127_0: i128 = 3;
        // D s_127_1: read-var op0:i
        let s_127_1: i128 = fn_state.op0;
        // D s_127_2: cmp-eq s_127_1 s_127_0
        let s_127_2: bool = ((s_127_1) == (s_127_0));
        // N s_127_3: branch s_127_2 b413 b128
        if s_127_2 {
            return block_413(state, tracer, fn_state);
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
        // D s_128_1: write-var gs#140617 <= s_128_0
        fn_state.gs_140617 = s_128_0;
        // N s_128_2: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_129_0: read-var gs#140617:u8
        let s_129_0: bool = fn_state.gs_140617;
        // N s_129_1: branch s_129_0 b412 b130
        if s_129_0 {
            return block_412(state, tracer, fn_state);
        } else {
            return block_130(state, tracer, fn_state);
        };
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_130_0: const #0u : u8
        let s_130_0: bool = false;
        // D s_130_1: write-var gs#140619 <= s_130_0
        fn_state.gs_140619 = s_130_0;
        // N s_130_2: jump b131
        return block_131(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_131_0: read-var gs#140619:u8
        let s_131_0: bool = fn_state.gs_140619;
        // N s_131_1: branch s_131_0 b408 b132
        if s_131_0 {
            return block_408(state, tracer, fn_state);
        } else {
            return block_132(state, tracer, fn_state);
        };
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_132_0: const #0u : u8
        let s_132_0: bool = false;
        // D s_132_1: write-var gs#140623 <= s_132_0
        fn_state.gs_140623 = s_132_0;
        // N s_132_2: jump b133
        return block_133(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_133_0: read-var gs#140623:u8
        let s_133_0: bool = fn_state.gs_140623;
        // N s_133_1: branch s_133_0 b407 b134
        if s_133_0 {
            return block_407(state, tracer, fn_state);
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
        // D s_134_1: write-var gs#140625 <= s_134_0
        fn_state.gs_140625 = s_134_0;
        // N s_134_2: jump b135
        return block_135(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_135_0: read-var gs#140625:u8
        let s_135_0: bool = fn_state.gs_140625;
        // N s_135_1: branch s_135_0 b404 b136
        if s_135_0 {
            return block_404(state, tracer, fn_state);
        } else {
            return block_136(state, tracer, fn_state);
        };
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_136_0: const #3s : i
        let s_136_0: i128 = 3;
        // D s_136_1: read-var op0:i
        let s_136_1: i128 = fn_state.op0;
        // D s_136_2: cmp-eq s_136_1 s_136_0
        let s_136_2: bool = ((s_136_1) == (s_136_0));
        // N s_136_3: branch s_136_2 b403 b137
        if s_136_2 {
            return block_403(state, tracer, fn_state);
        } else {
            return block_137(state, tracer, fn_state);
        };
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_137_0: const #0u : u8
        let s_137_0: bool = false;
        // D s_137_1: write-var gs#140628 <= s_137_0
        fn_state.gs_140628 = s_137_0;
        // N s_137_2: jump b138
        return block_138(state, tracer, fn_state);
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_138_0: read-var gs#140628:u8
        let s_138_0: bool = fn_state.gs_140628;
        // N s_138_1: branch s_138_0 b402 b139
        if s_138_0 {
            return block_402(state, tracer, fn_state);
        } else {
            return block_139(state, tracer, fn_state);
        };
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_139_0: const #0u : u8
        let s_139_0: bool = false;
        // D s_139_1: write-var gs#140630 <= s_139_0
        fn_state.gs_140630 = s_139_0;
        // N s_139_2: jump b140
        return block_140(state, tracer, fn_state);
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_140_0: read-var gs#140630:u8
        let s_140_0: bool = fn_state.gs_140630;
        // N s_140_1: branch s_140_0 b401 b141
        if s_140_0 {
            return block_401(state, tracer, fn_state);
        } else {
            return block_141(state, tracer, fn_state);
        };
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_141_0: const #0u : u8
        let s_141_0: bool = false;
        // D s_141_1: write-var gs#140632 <= s_141_0
        fn_state.gs_140632 = s_141_0;
        // N s_141_2: jump b142
        return block_142(state, tracer, fn_state);
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_142_0: read-var gs#140632:u8
        let s_142_0: bool = fn_state.gs_140632;
        // N s_142_1: branch s_142_0 b400 b143
        if s_142_0 {
            return block_400(state, tracer, fn_state);
        } else {
            return block_143(state, tracer, fn_state);
        };
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_143_0: const #0u : u8
        let s_143_0: bool = false;
        // D s_143_1: write-var gs#140634 <= s_143_0
        fn_state.gs_140634 = s_143_0;
        // N s_143_2: jump b144
        return block_144(state, tracer, fn_state);
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_144_0: read-var gs#140634:u8
        let s_144_0: bool = fn_state.gs_140634;
        // N s_144_1: branch s_144_0 b375 b145
        if s_144_0 {
            return block_375(state, tracer, fn_state);
        } else {
            return block_145(state, tracer, fn_state);
        };
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_145_0: read-var op0:i
        let s_145_0: i128 = fn_state.op0;
        // D s_145_1: read-var op1:i
        let s_145_1: i128 = fn_state.op1;
        // D s_145_2: read-var crn:i
        let s_145_2: i128 = fn_state.crn;
        // D s_145_3: read-var crm:i
        let s_145_3: i128 = fn_state.crm;
        // D s_145_4: read-var op2:i
        let s_145_4: i128 = fn_state.op2;
        // D s_145_5: call AArch64_CheckNVCondsIfCurrentEL(s_145_0, s_145_1, s_145_2, s_145_3, s_145_4)
        let s_145_5: bool = AArch64_CheckNVCondsIfCurrentEL(
            state,
            tracer,
            s_145_0,
            s_145_1,
            s_145_2,
            s_145_3,
            s_145_4,
        );
        // N s_145_6: branch s_145_5 b374 b146
        if s_145_5 {
            return block_374(state, tracer, fn_state);
        } else {
            return block_146(state, tracer, fn_state);
        };
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_146_0: jump b147
        return block_147(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_147_0: const #3s : i
        let s_147_0: i128 = 3;
        // D s_147_1: read-var op0:i
        let s_147_1: i128 = fn_state.op0;
        // D s_147_2: cmp-eq s_147_1 s_147_0
        let s_147_2: bool = ((s_147_1) == (s_147_0));
        // N s_147_3: branch s_147_2 b373 b148
        if s_147_2 {
            return block_373(state, tracer, fn_state);
        } else {
            return block_148(state, tracer, fn_state);
        };
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_148_0: const #0u : u8
        let s_148_0: bool = false;
        // D s_148_1: write-var gs#140637 <= s_148_0
        fn_state.gs_140637 = s_148_0;
        // N s_148_2: jump b149
        return block_149(state, tracer, fn_state);
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_149_0: read-var gs#140637:u8
        let s_149_0: bool = fn_state.gs_140637;
        // N s_149_1: branch s_149_0 b372 b150
        if s_149_0 {
            return block_372(state, tracer, fn_state);
        } else {
            return block_150(state, tracer, fn_state);
        };
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_150_0: const #0u : u8
        let s_150_0: bool = false;
        // D s_150_1: write-var gs#140639 <= s_150_0
        fn_state.gs_140639 = s_150_0;
        // N s_150_2: jump b151
        return block_151(state, tracer, fn_state);
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_151_0: read-var gs#140639:u8
        let s_151_0: bool = fn_state.gs_140639;
        // N s_151_1: branch s_151_0 b371 b152
        if s_151_0 {
            return block_371(state, tracer, fn_state);
        } else {
            return block_152(state, tracer, fn_state);
        };
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_152_0: const #0u : u8
        let s_152_0: bool = false;
        // D s_152_1: write-var gs#140641 <= s_152_0
        fn_state.gs_140641 = s_152_0;
        // N s_152_2: jump b153
        return block_153(state, tracer, fn_state);
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_153_0: read-var gs#140641:u8
        let s_153_0: bool = fn_state.gs_140641;
        // N s_153_1: branch s_153_0 b367 b154
        if s_153_0 {
            return block_367(state, tracer, fn_state);
        } else {
            return block_154(state, tracer, fn_state);
        };
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_154_0: const #0u : u8
        let s_154_0: bool = false;
        // D s_154_1: write-var gs#140645 <= s_154_0
        fn_state.gs_140645 = s_154_0;
        // N s_154_2: jump b155
        return block_155(state, tracer, fn_state);
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_155_0: read-var gs#140645:u8
        let s_155_0: bool = fn_state.gs_140645;
        // N s_155_1: branch s_155_0 b363 b156
        if s_155_0 {
            return block_363(state, tracer, fn_state);
        } else {
            return block_156(state, tracer, fn_state);
        };
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_156_0: jump b157
        return block_157(state, tracer, fn_state);
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_157_0: const #2s : i
        let s_157_0: i128 = 2;
        // D s_157_1: read-var op0:i
        let s_157_1: i128 = fn_state.op0;
        // D s_157_2: cmp-eq s_157_1 s_157_0
        let s_157_2: bool = ((s_157_1) == (s_157_0));
        // N s_157_3: branch s_157_2 b362 b158
        if s_157_2 {
            return block_362(state, tracer, fn_state);
        } else {
            return block_158(state, tracer, fn_state);
        };
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_158_0: const #0u : u8
        let s_158_0: bool = false;
        // D s_158_1: write-var gs#140648 <= s_158_0
        fn_state.gs_140648 = s_158_0;
        // N s_158_2: jump b159
        return block_159(state, tracer, fn_state);
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_159_0: read-var gs#140648:u8
        let s_159_0: bool = fn_state.gs_140648;
        // N s_159_1: branch s_159_0 b361 b160
        if s_159_0 {
            return block_361(state, tracer, fn_state);
        } else {
            return block_160(state, tracer, fn_state);
        };
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_160_0: const #0u : u8
        let s_160_0: bool = false;
        // D s_160_1: write-var gs#140650 <= s_160_0
        fn_state.gs_140650 = s_160_0;
        // N s_160_2: jump b161
        return block_161(state, tracer, fn_state);
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_161_0: read-var gs#140650:u8
        let s_161_0: bool = fn_state.gs_140650;
        // N s_161_1: branch s_161_0 b345 b162
        if s_161_0 {
            return block_345(state, tracer, fn_state);
        } else {
            return block_162(state, tracer, fn_state);
        };
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_162_0: const #0u : u8
        let s_162_0: bool = false;
        // D s_162_1: write-var gs#140662 <= s_162_0
        fn_state.gs_140662 = s_162_0;
        // N s_162_2: jump b163
        return block_163(state, tracer, fn_state);
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_163_0: read-var gs#140662:u8
        let s_163_0: bool = fn_state.gs_140662;
        // N s_163_1: branch s_163_0 b314 b164
        if s_163_0 {
            return block_314(state, tracer, fn_state);
        } else {
            return block_164(state, tracer, fn_state);
        };
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_164_0: jump b165
        return block_165(state, tracer, fn_state);
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_165_0: const #3s : i
        let s_165_0: i128 = 3;
        // D s_165_1: read-var op0:i
        let s_165_1: i128 = fn_state.op0;
        // D s_165_2: cmp-eq s_165_1 s_165_0
        let s_165_2: bool = ((s_165_1) == (s_165_0));
        // N s_165_3: branch s_165_2 b313 b166
        if s_165_2 {
            return block_313(state, tracer, fn_state);
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
        // D s_166_1: write-var gs#140665 <= s_166_0
        fn_state.gs_140665 = s_166_0;
        // N s_166_2: jump b167
        return block_167(state, tracer, fn_state);
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_167_0: read-var gs#140665:u8
        let s_167_0: bool = fn_state.gs_140665;
        // N s_167_1: branch s_167_0 b312 b168
        if s_167_0 {
            return block_312(state, tracer, fn_state);
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
        // D s_168_1: write-var gs#140667 <= s_168_0
        fn_state.gs_140667 = s_168_0;
        // N s_168_2: jump b169
        return block_169(state, tracer, fn_state);
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_169_0: read-var gs#140667:u8
        let s_169_0: bool = fn_state.gs_140667;
        // N s_169_1: branch s_169_0 b311 b170
        if s_169_0 {
            return block_311(state, tracer, fn_state);
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
        // D s_170_1: write-var gs#140669 <= s_170_0
        fn_state.gs_140669 = s_170_0;
        // N s_170_2: jump b171
        return block_171(state, tracer, fn_state);
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_171_0: read-var gs#140669:u8
        let s_171_0: bool = fn_state.gs_140669;
        // N s_171_1: branch s_171_0 b310 b172
        if s_171_0 {
            return block_310(state, tracer, fn_state);
        } else {
            return block_172(state, tracer, fn_state);
        };
    }
    fn block_172<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_172_0: const #0u : u8
        let s_172_0: bool = false;
        // D s_172_1: write-var gs#140671 <= s_172_0
        fn_state.gs_140671 = s_172_0;
        // N s_172_2: jump b173
        return block_173(state, tracer, fn_state);
    }
    fn block_173<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_173_0: read-var gs#140671:u8
        let s_173_0: bool = fn_state.gs_140671;
        // N s_173_1: branch s_173_0 b301 b174
        if s_173_0 {
            return block_301(state, tracer, fn_state);
        } else {
            return block_174(state, tracer, fn_state);
        };
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_174_0: jump b175
        return block_175(state, tracer, fn_state);
    }
    fn block_175<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_175_0: const #2s : i
        let s_175_0: i128 = 2;
        // D s_175_1: read-var op0:i
        let s_175_1: i128 = fn_state.op0;
        // D s_175_2: cmp-eq s_175_1 s_175_0
        let s_175_2: bool = ((s_175_1) == (s_175_0));
        // N s_175_3: branch s_175_2 b300 b176
        if s_175_2 {
            return block_300(state, tracer, fn_state);
        } else {
            return block_176(state, tracer, fn_state);
        };
    }
    fn block_176<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_176_0: const #0u : u8
        let s_176_0: bool = false;
        // D s_176_1: write-var gs#140674 <= s_176_0
        fn_state.gs_140674 = s_176_0;
        // N s_176_2: jump b177
        return block_177(state, tracer, fn_state);
    }
    fn block_177<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_177_0: read-var gs#140674:u8
        let s_177_0: bool = fn_state.gs_140674;
        // N s_177_1: branch s_177_0 b299 b178
        if s_177_0 {
            return block_299(state, tracer, fn_state);
        } else {
            return block_178(state, tracer, fn_state);
        };
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_178_0: const #0u : u8
        let s_178_0: bool = false;
        // D s_178_1: write-var gs#140676 <= s_178_0
        fn_state.gs_140676 = s_178_0;
        // N s_178_2: jump b179
        return block_179(state, tracer, fn_state);
    }
    fn block_179<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_179_0: read-var gs#140676:u8
        let s_179_0: bool = fn_state.gs_140676;
        // N s_179_1: branch s_179_0 b298 b180
        if s_179_0 {
            return block_298(state, tracer, fn_state);
        } else {
            return block_180(state, tracer, fn_state);
        };
    }
    fn block_180<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_180_0: const #0u : u8
        let s_180_0: bool = false;
        // D s_180_1: write-var gs#140678 <= s_180_0
        fn_state.gs_140678 = s_180_0;
        // N s_180_2: jump b181
        return block_181(state, tracer, fn_state);
    }
    fn block_181<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_181_0: read-var gs#140678:u8
        let s_181_0: bool = fn_state.gs_140678;
        // N s_181_1: branch s_181_0 b297 b182
        if s_181_0 {
            return block_297(state, tracer, fn_state);
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
        // D s_182_1: write-var gs#140680 <= s_182_0
        fn_state.gs_140680 = s_182_0;
        // N s_182_2: jump b183
        return block_183(state, tracer, fn_state);
    }
    fn block_183<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_183_0: read-var gs#140680:u8
        let s_183_0: bool = fn_state.gs_140680;
        // N s_183_1: branch s_183_0 b288 b184
        if s_183_0 {
            return block_288(state, tracer, fn_state);
        } else {
            return block_184(state, tracer, fn_state);
        };
    }
    fn block_184<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_184_0: const #2s : i
        let s_184_0: i128 = 2;
        // D s_184_1: read-var op0:i
        let s_184_1: i128 = fn_state.op0;
        // D s_184_2: cmp-eq s_184_1 s_184_0
        let s_184_2: bool = ((s_184_1) == (s_184_0));
        // N s_184_3: branch s_184_2 b287 b185
        if s_184_2 {
            return block_287(state, tracer, fn_state);
        } else {
            return block_185(state, tracer, fn_state);
        };
    }
    fn block_185<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_185_0: const #0u : u8
        let s_185_0: bool = false;
        // D s_185_1: write-var gs#140683 <= s_185_0
        fn_state.gs_140683 = s_185_0;
        // N s_185_2: jump b186
        return block_186(state, tracer, fn_state);
    }
    fn block_186<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_186_0: read-var gs#140683:u8
        let s_186_0: bool = fn_state.gs_140683;
        // N s_186_1: branch s_186_0 b286 b187
        if s_186_0 {
            return block_286(state, tracer, fn_state);
        } else {
            return block_187(state, tracer, fn_state);
        };
    }
    fn block_187<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_187_0: const #0u : u8
        let s_187_0: bool = false;
        // D s_187_1: write-var gs#140685 <= s_187_0
        fn_state.gs_140685 = s_187_0;
        // N s_187_2: jump b188
        return block_188(state, tracer, fn_state);
    }
    fn block_188<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_188_0: read-var gs#140685:u8
        let s_188_0: bool = fn_state.gs_140685;
        // N s_188_1: branch s_188_0 b285 b189
        if s_188_0 {
            return block_285(state, tracer, fn_state);
        } else {
            return block_189(state, tracer, fn_state);
        };
    }
    fn block_189<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_189_0: const #0u : u8
        let s_189_0: bool = false;
        // D s_189_1: write-var gs#140687 <= s_189_0
        fn_state.gs_140687 = s_189_0;
        // N s_189_2: jump b190
        return block_190(state, tracer, fn_state);
    }
    fn block_190<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_190_0: read-var gs#140687:u8
        let s_190_0: bool = fn_state.gs_140687;
        // N s_190_1: branch s_190_0 b284 b191
        if s_190_0 {
            return block_284(state, tracer, fn_state);
        } else {
            return block_191(state, tracer, fn_state);
        };
    }
    fn block_191<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_191_0: const #0u : u8
        let s_191_0: bool = false;
        // D s_191_1: write-var gs#140689 <= s_191_0
        fn_state.gs_140689 = s_191_0;
        // N s_191_2: jump b192
        return block_192(state, tracer, fn_state);
    }
    fn block_192<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_192_0: read-var gs#140689:u8
        let s_192_0: bool = fn_state.gs_140689;
        // N s_192_1: branch s_192_0 b283 b193
        if s_192_0 {
            return block_283(state, tracer, fn_state);
        } else {
            return block_193(state, tracer, fn_state);
        };
    }
    fn block_193<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_193_0: jump b194
        return block_194(state, tracer, fn_state);
    }
    fn block_194<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_194_0: const #2s : i
        let s_194_0: i128 = 2;
        // D s_194_1: read-var op0:i
        let s_194_1: i128 = fn_state.op0;
        // D s_194_2: cmp-eq s_194_1 s_194_0
        let s_194_2: bool = ((s_194_1) == (s_194_0));
        // N s_194_3: branch s_194_2 b282 b195
        if s_194_2 {
            return block_282(state, tracer, fn_state);
        } else {
            return block_195(state, tracer, fn_state);
        };
    }
    fn block_195<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_195_0: const #0u : u8
        let s_195_0: bool = false;
        // D s_195_1: write-var gs#140692 <= s_195_0
        fn_state.gs_140692 = s_195_0;
        // N s_195_2: jump b196
        return block_196(state, tracer, fn_state);
    }
    fn block_196<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_196_0: read-var gs#140692:u8
        let s_196_0: bool = fn_state.gs_140692;
        // N s_196_1: branch s_196_0 b281 b197
        if s_196_0 {
            return block_281(state, tracer, fn_state);
        } else {
            return block_197(state, tracer, fn_state);
        };
    }
    fn block_197<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_197_0: const #0u : u8
        let s_197_0: bool = false;
        // D s_197_1: write-var gs#140694 <= s_197_0
        fn_state.gs_140694 = s_197_0;
        // N s_197_2: jump b198
        return block_198(state, tracer, fn_state);
    }
    fn block_198<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_198_0: read-var gs#140694:u8
        let s_198_0: bool = fn_state.gs_140694;
        // N s_198_1: branch s_198_0 b280 b199
        if s_198_0 {
            return block_280(state, tracer, fn_state);
        } else {
            return block_199(state, tracer, fn_state);
        };
    }
    fn block_199<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_199_0: const #0u : u8
        let s_199_0: bool = false;
        // D s_199_1: write-var gs#140696 <= s_199_0
        fn_state.gs_140696 = s_199_0;
        // N s_199_2: jump b200
        return block_200(state, tracer, fn_state);
    }
    fn block_200<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_200_0: read-var gs#140696:u8
        let s_200_0: bool = fn_state.gs_140696;
        // N s_200_1: branch s_200_0 b279 b201
        if s_200_0 {
            return block_279(state, tracer, fn_state);
        } else {
            return block_201(state, tracer, fn_state);
        };
    }
    fn block_201<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_201_0: const #0u : u8
        let s_201_0: bool = false;
        // D s_201_1: write-var gs#140698 <= s_201_0
        fn_state.gs_140698 = s_201_0;
        // N s_201_2: jump b202
        return block_202(state, tracer, fn_state);
    }
    fn block_202<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_202_0: read-var gs#140698:u8
        let s_202_0: bool = fn_state.gs_140698;
        // N s_202_1: branch s_202_0 b278 b203
        if s_202_0 {
            return block_278(state, tracer, fn_state);
        } else {
            return block_203(state, tracer, fn_state);
        };
    }
    fn block_203<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_203_0: jump b204
        return block_204(state, tracer, fn_state);
    }
    fn block_204<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_204_0: const #3s : i
        let s_204_0: i128 = 3;
        // D s_204_1: read-var op0:i
        let s_204_1: i128 = fn_state.op0;
        // D s_204_2: cmp-eq s_204_1 s_204_0
        let s_204_2: bool = ((s_204_1) == (s_204_0));
        // N s_204_3: branch s_204_2 b277 b205
        if s_204_2 {
            return block_277(state, tracer, fn_state);
        } else {
            return block_205(state, tracer, fn_state);
        };
    }
    fn block_205<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_205_0: const #0u : u8
        let s_205_0: bool = false;
        // D s_205_1: write-var gs#140701 <= s_205_0
        fn_state.gs_140701 = s_205_0;
        // N s_205_2: jump b206
        return block_206(state, tracer, fn_state);
    }
    fn block_206<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_206_0: read-var gs#140701:u8
        let s_206_0: bool = fn_state.gs_140701;
        // N s_206_1: branch s_206_0 b276 b207
        if s_206_0 {
            return block_276(state, tracer, fn_state);
        } else {
            return block_207(state, tracer, fn_state);
        };
    }
    fn block_207<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_207_0: const #0u : u8
        let s_207_0: bool = false;
        // D s_207_1: write-var gs#140703 <= s_207_0
        fn_state.gs_140703 = s_207_0;
        // N s_207_2: jump b208
        return block_208(state, tracer, fn_state);
    }
    fn block_208<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_208_0: read-var gs#140703:u8
        let s_208_0: bool = fn_state.gs_140703;
        // N s_208_1: branch s_208_0 b275 b209
        if s_208_0 {
            return block_275(state, tracer, fn_state);
        } else {
            return block_209(state, tracer, fn_state);
        };
    }
    fn block_209<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_209_0: const #0u : u8
        let s_209_0: bool = false;
        // D s_209_1: write-var gs#140705 <= s_209_0
        fn_state.gs_140705 = s_209_0;
        // N s_209_2: jump b210
        return block_210(state, tracer, fn_state);
    }
    fn block_210<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_210_0: read-var gs#140705:u8
        let s_210_0: bool = fn_state.gs_140705;
        // N s_210_1: branch s_210_0 b274 b211
        if s_210_0 {
            return block_274(state, tracer, fn_state);
        } else {
            return block_211(state, tracer, fn_state);
        };
    }
    fn block_211<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_211_0: const #0u : u8
        let s_211_0: bool = false;
        // D s_211_1: write-var gs#140707 <= s_211_0
        fn_state.gs_140707 = s_211_0;
        // N s_211_2: jump b212
        return block_212(state, tracer, fn_state);
    }
    fn block_212<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_212_0: read-var gs#140707:u8
        let s_212_0: bool = fn_state.gs_140707;
        // N s_212_1: branch s_212_0 b267 b213
        if s_212_0 {
            return block_267(state, tracer, fn_state);
        } else {
            return block_213(state, tracer, fn_state);
        };
    }
    fn block_213<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_213_0: jump b214
        return block_214(state, tracer, fn_state);
    }
    fn block_214<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_214_0: const #3s : i
        let s_214_0: i128 = 3;
        // D s_214_1: read-var op0:i
        let s_214_1: i128 = fn_state.op0;
        // D s_214_2: cmp-eq s_214_1 s_214_0
        let s_214_2: bool = ((s_214_1) == (s_214_0));
        // N s_214_3: branch s_214_2 b266 b215
        if s_214_2 {
            return block_266(state, tracer, fn_state);
        } else {
            return block_215(state, tracer, fn_state);
        };
    }
    fn block_215<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_215_0: const #0u : u8
        let s_215_0: bool = false;
        // D s_215_1: write-var gs#140710 <= s_215_0
        fn_state.gs_140710 = s_215_0;
        // N s_215_2: jump b216
        return block_216(state, tracer, fn_state);
    }
    fn block_216<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_216_0: read-var gs#140710:u8
        let s_216_0: bool = fn_state.gs_140710;
        // N s_216_1: branch s_216_0 b265 b217
        if s_216_0 {
            return block_265(state, tracer, fn_state);
        } else {
            return block_217(state, tracer, fn_state);
        };
    }
    fn block_217<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_217_0: const #0u : u8
        let s_217_0: bool = false;
        // D s_217_1: write-var gs#140712 <= s_217_0
        fn_state.gs_140712 = s_217_0;
        // N s_217_2: jump b218
        return block_218(state, tracer, fn_state);
    }
    fn block_218<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_218_0: read-var gs#140712:u8
        let s_218_0: bool = fn_state.gs_140712;
        // N s_218_1: branch s_218_0 b264 b219
        if s_218_0 {
            return block_264(state, tracer, fn_state);
        } else {
            return block_219(state, tracer, fn_state);
        };
    }
    fn block_219<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_219_0: const #0u : u8
        let s_219_0: bool = false;
        // D s_219_1: write-var gs#140714 <= s_219_0
        fn_state.gs_140714 = s_219_0;
        // N s_219_2: jump b220
        return block_220(state, tracer, fn_state);
    }
    fn block_220<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_220_0: read-var gs#140714:u8
        let s_220_0: bool = fn_state.gs_140714;
        // N s_220_1: branch s_220_0 b263 b221
        if s_220_0 {
            return block_263(state, tracer, fn_state);
        } else {
            return block_221(state, tracer, fn_state);
        };
    }
    fn block_221<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_221_0: const #0u : u8
        let s_221_0: bool = false;
        // D s_221_1: write-var gs#140716 <= s_221_0
        fn_state.gs_140716 = s_221_0;
        // N s_221_2: jump b222
        return block_222(state, tracer, fn_state);
    }
    fn block_222<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_222_0: read-var gs#140716:u8
        let s_222_0: bool = fn_state.gs_140716;
        // N s_222_1: branch s_222_0 b262 b223
        if s_222_0 {
            return block_262(state, tracer, fn_state);
        } else {
            return block_223(state, tracer, fn_state);
        };
    }
    fn block_223<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_223_0: jump b224
        return block_224(state, tracer, fn_state);
    }
    fn block_224<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_224_0: const #3s : i
        let s_224_0: i128 = 3;
        // D s_224_1: read-var op0:i
        let s_224_1: i128 = fn_state.op0;
        // D s_224_2: cmp-eq s_224_1 s_224_0
        let s_224_2: bool = ((s_224_1) == (s_224_0));
        // N s_224_3: branch s_224_2 b261 b225
        if s_224_2 {
            return block_261(state, tracer, fn_state);
        } else {
            return block_225(state, tracer, fn_state);
        };
    }
    fn block_225<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_225_0: const #0u : u8
        let s_225_0: bool = false;
        // D s_225_1: write-var gs#140719 <= s_225_0
        fn_state.gs_140719 = s_225_0;
        // N s_225_2: jump b226
        return block_226(state, tracer, fn_state);
    }
    fn block_226<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_226_0: read-var gs#140719:u8
        let s_226_0: bool = fn_state.gs_140719;
        // N s_226_1: branch s_226_0 b260 b227
        if s_226_0 {
            return block_260(state, tracer, fn_state);
        } else {
            return block_227(state, tracer, fn_state);
        };
    }
    fn block_227<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_227_0: const #0u : u8
        let s_227_0: bool = false;
        // D s_227_1: write-var gs#140721 <= s_227_0
        fn_state.gs_140721 = s_227_0;
        // N s_227_2: jump b228
        return block_228(state, tracer, fn_state);
    }
    fn block_228<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_228_0: read-var gs#140721:u8
        let s_228_0: bool = fn_state.gs_140721;
        // N s_228_1: branch s_228_0 b259 b229
        if s_228_0 {
            return block_259(state, tracer, fn_state);
        } else {
            return block_229(state, tracer, fn_state);
        };
    }
    fn block_229<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_229_0: const #0u : u8
        let s_229_0: bool = false;
        // D s_229_1: write-var gs#140723 <= s_229_0
        fn_state.gs_140723 = s_229_0;
        // N s_229_2: jump b230
        return block_230(state, tracer, fn_state);
    }
    fn block_230<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_230_0: read-var gs#140723:u8
        let s_230_0: bool = fn_state.gs_140723;
        // N s_230_1: branch s_230_0 b258 b231
        if s_230_0 {
            return block_258(state, tracer, fn_state);
        } else {
            return block_231(state, tracer, fn_state);
        };
    }
    fn block_231<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_231_0: const #0u : u8
        let s_231_0: bool = false;
        // D s_231_1: write-var gs#140725 <= s_231_0
        fn_state.gs_140725 = s_231_0;
        // N s_231_2: jump b232
        return block_232(state, tracer, fn_state);
    }
    fn block_232<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_232_0: read-var gs#140725:u8
        let s_232_0: bool = fn_state.gs_140725;
        // N s_232_1: branch s_232_0 b251 b233
        if s_232_0 {
            return block_251(state, tracer, fn_state);
        } else {
            return block_233(state, tracer, fn_state);
        };
    }
    fn block_233<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_233_0: jump b234
        return block_234(state, tracer, fn_state);
    }
    fn block_234<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_234_0: const #3s : i
        let s_234_0: i128 = 3;
        // D s_234_1: read-var op0:i
        let s_234_1: i128 = fn_state.op0;
        // D s_234_2: cmp-eq s_234_1 s_234_0
        let s_234_2: bool = ((s_234_1) == (s_234_0));
        // N s_234_3: branch s_234_2 b250 b235
        if s_234_2 {
            return block_250(state, tracer, fn_state);
        } else {
            return block_235(state, tracer, fn_state);
        };
    }
    fn block_235<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_235_0: const #0u : u8
        let s_235_0: bool = false;
        // D s_235_1: write-var gs#140728 <= s_235_0
        fn_state.gs_140728 = s_235_0;
        // N s_235_2: jump b236
        return block_236(state, tracer, fn_state);
    }
    fn block_236<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_236_0: read-var gs#140728:u8
        let s_236_0: bool = fn_state.gs_140728;
        // N s_236_1: branch s_236_0 b249 b237
        if s_236_0 {
            return block_249(state, tracer, fn_state);
        } else {
            return block_237(state, tracer, fn_state);
        };
    }
    fn block_237<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_237_0: const #0u : u8
        let s_237_0: bool = false;
        // D s_237_1: write-var gs#140730 <= s_237_0
        fn_state.gs_140730 = s_237_0;
        // N s_237_2: jump b238
        return block_238(state, tracer, fn_state);
    }
    fn block_238<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_238_0: read-var gs#140730:u8
        let s_238_0: bool = fn_state.gs_140730;
        // N s_238_1: branch s_238_0 b248 b239
        if s_238_0 {
            return block_248(state, tracer, fn_state);
        } else {
            return block_239(state, tracer, fn_state);
        };
    }
    fn block_239<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_239_0: const #0u : u8
        let s_239_0: bool = false;
        // D s_239_1: write-var gs#140732 <= s_239_0
        fn_state.gs_140732 = s_239_0;
        // N s_239_2: jump b240
        return block_240(state, tracer, fn_state);
    }
    fn block_240<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_240_0: read-var gs#140732:u8
        let s_240_0: bool = fn_state.gs_140732;
        // N s_240_1: branch s_240_0 b247 b241
        if s_240_0 {
            return block_247(state, tracer, fn_state);
        } else {
            return block_241(state, tracer, fn_state);
        };
    }
    fn block_241<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_241_0: const #0u : u8
        let s_241_0: bool = false;
        // D s_241_1: write-var gs#140734 <= s_241_0
        fn_state.gs_140734 = s_241_0;
        // N s_241_2: jump b242
        return block_242(state, tracer, fn_state);
    }
    fn block_242<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_242_0: read-var gs#140734:u8
        let s_242_0: bool = fn_state.gs_140734;
        // N s_242_1: branch s_242_0 b244 b243
        if s_242_0 {
            return block_244(state, tracer, fn_state);
        } else {
            return block_243(state, tracer, fn_state);
        };
    }
    fn block_243<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_243_0: return
        return;
    }
    fn block_244<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_244_0: const #101824u : u32
        let s_244_0: u32 = 101824;
        // D s_244_1: read-reg s_244_0:struct
        let s_244_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_244_0 as isize);
            tracer.read_register(s_244_0 as isize, value);
            value
        };
        // D s_244_2: call _get_ID_AA64PFR0_EL1_Type_EL3(s_244_1)
        let s_244_2: u8 = u_get_ID_AA64PFR0_EL1_Type_EL3(state, tracer, s_244_1);
        // D s_244_3: cast zx s_244_2 -> bv
        let s_244_3: Bits = Bits::new(s_244_2 as u128, 4u16);
        // C s_244_4: const #1u : u8
        let s_244_4: u8 = 1;
        // C s_244_5: cast zx s_244_4 -> bv
        let s_244_5: Bits = Bits::new(s_244_4 as u128, 4u16);
        // D s_244_6: cmp-eq s_244_3 s_244_5
        let s_244_6: bool = ((s_244_3) == (s_244_5));
        // N s_244_7: branch s_244_6 b246 b245
        if s_244_6 {
            return block_246(state, tracer, fn_state);
        } else {
            return block_245(state, tracer, fn_state);
        };
    }
    fn block_245<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_245_0: return
        return;
    }
    fn block_246<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_246_0: const #64s : i64
        let s_246_0: i64 = 64;
        // C s_246_1: const #64s : i64
        let s_246_1: i64 = 64;
        // D s_246_2: read-var t:i
        let s_246_2: i128 = fn_state.t;
        // D s_246_3: call X_read(s_246_2, s_246_1)
        let s_246_3: Bits = X_read(state, tracer, s_246_2, s_246_1);
        // D s_246_4: cast reint s_246_3 -> u64
        let s_246_4: u64 = (s_246_3.value() as u64);
        // C s_246_5: const #0s : i
        let s_246_5: i128 = 0;
        // D s_246_6: cast zx s_246_4 -> bv
        let s_246_6: Bits = Bits::new(s_246_4 as u128, 64u16);
        // C s_246_7: const #1u : u8
        let s_246_7: bool = true;
        // C s_246_8: cast zx s_246_7 -> bv
        let s_246_8: Bits = Bits::new(s_246_7 as u128, 1u16);
        // C s_246_9: const #0s : i
        let s_246_9: i128 = 0;
        // C s_246_10: const #1u : u64
        let s_246_10: u64 = 1;
        // C s_246_11: cast zx s_246_10 -> bv
        let s_246_11: Bits = Bits::new(s_246_10 as u128, 64u16);
        // C s_246_12: lsl s_246_11 s_246_9
        let s_246_12: Bits = s_246_11 << s_246_9;
        // C s_246_13: sub s_246_12 s_246_11
        let s_246_13: Bits = ((s_246_12) - (s_246_11));
        // C s_246_14: and s_246_8 s_246_13
        let s_246_14: Bits = ((s_246_8) & (s_246_13));
        // C s_246_15: lsl s_246_14 s_246_5
        let s_246_15: Bits = s_246_14 << s_246_5;
        // C s_246_16: lsl s_246_13 s_246_5
        let s_246_16: Bits = s_246_13 << s_246_5;
        // C s_246_17: cmpl s_246_16
        let s_246_17: Bits = !s_246_16;
        // D s_246_18: and s_246_6 s_246_17
        let s_246_18: Bits = ((s_246_6) & (s_246_17));
        // D s_246_19: or s_246_18 s_246_15
        let s_246_19: Bits = ((s_246_18) | (s_246_15));
        // D s_246_20: cast reint s_246_19 -> u64
        let s_246_20: u64 = (s_246_19.value() as u64);
        // D s_246_21: cast zx s_246_20 -> bv
        let s_246_21: Bits = Bits::new(s_246_20 as u128, 64u16);
        // D s_246_22: read-var t:i
        let s_246_22: i128 = fn_state.t;
        // D s_246_23: call X_set(s_246_22, s_246_0, s_246_21)
        let s_246_23: () = X_set(state, tracer, s_246_22, s_246_0, s_246_21);
        // N s_246_24: return
        return;
    }
    fn block_247<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_247_0: const #0s : i
        let s_247_0: i128 = 0;
        // D s_247_1: read-var crm:i
        let s_247_1: i128 = fn_state.crm;
        // D s_247_2: cmp-eq s_247_1 s_247_0
        let s_247_2: bool = ((s_247_1) == (s_247_0));
        // D s_247_3: write-var gs#140734 <= s_247_2
        fn_state.gs_140734 = s_247_2;
        // N s_247_4: jump b242
        return block_242(state, tracer, fn_state);
    }
    fn block_248<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_248_0: const #2s : i
        let s_248_0: i128 = 2;
        // D s_248_1: read-var op2:i
        let s_248_1: i128 = fn_state.op2;
        // D s_248_2: cmp-eq s_248_1 s_248_0
        let s_248_2: bool = ((s_248_1) == (s_248_0));
        // D s_248_3: write-var gs#140732 <= s_248_2
        fn_state.gs_140732 = s_248_2;
        // N s_248_4: jump b240
        return block_240(state, tracer, fn_state);
    }
    fn block_249<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_249_0: const #6s : i
        let s_249_0: i128 = 6;
        // D s_249_1: read-var op1:i
        let s_249_1: i128 = fn_state.op1;
        // D s_249_2: cmp-eq s_249_1 s_249_0
        let s_249_2: bool = ((s_249_1) == (s_249_0));
        // D s_249_3: write-var gs#140730 <= s_249_2
        fn_state.gs_140730 = s_249_2;
        // N s_249_4: jump b238
        return block_238(state, tracer, fn_state);
    }
    fn block_250<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_250_0: const #12s : i
        let s_250_0: i128 = 12;
        // D s_250_1: read-var crn:i
        let s_250_1: i128 = fn_state.crn;
        // D s_250_2: cmp-eq s_250_1 s_250_0
        let s_250_2: bool = ((s_250_1) == (s_250_0));
        // D s_250_3: write-var gs#140728 <= s_250_2
        fn_state.gs_140728 = s_250_2;
        // N s_250_4: jump b236
        return block_236(state, tracer, fn_state);
    }
    fn block_251<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_251_0: const #440u : u32
        let s_251_0: u32 = 440;
        // D s_251_1: read-reg s_251_0:u8
        let s_251_1: u8 = {
            let value = state.read_register::<u8>(s_251_0 as isize);
            tracer.read_register(s_251_0 as isize, value);
            value
        };
        // D s_251_2: call ELUsingAArch32(s_251_1)
        let s_251_2: bool = ELUsingAArch32(state, tracer, s_251_1);
        // D s_251_3: not s_251_2
        let s_251_3: bool = !s_251_2;
        // N s_251_4: branch s_251_3 b257 b252
        if s_251_3 {
            return block_257(state, tracer, fn_state);
        } else {
            return block_252(state, tracer, fn_state);
        };
    }
    fn block_252<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_252_0: const #0u : u8
        let s_252_0: bool = false;
        // D s_252_1: write-var gs#140742 <= s_252_0
        fn_state.gs_140742 = s_252_0;
        // N s_252_2: jump b253
        return block_253(state, tracer, fn_state);
    }
    fn block_253<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_253_0: read-var gs#140742:u8
        let s_253_0: bool = fn_state.gs_140742;
        // N s_253_1: branch s_253_0 b256 b254
        if s_253_0 {
            return block_256(state, tracer, fn_state);
        } else {
            return block_254(state, tracer, fn_state);
        };
    }
    fn block_254<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_254_0: jump b255
        return block_255(state, tracer, fn_state);
    }
    fn block_255<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_255_0: jump b234
        return block_234(state, tracer, fn_state);
    }
    fn block_256<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_256_0: const #64s : i64
        let s_256_0: i64 = 64;
        // C s_256_1: const #64s : i64
        let s_256_1: i64 = 64;
        // D s_256_2: read-var t:i
        let s_256_2: i128 = fn_state.t;
        // D s_256_3: call X_read(s_256_2, s_256_1)
        let s_256_3: Bits = X_read(state, tracer, s_256_2, s_256_1);
        // D s_256_4: cast reint s_256_3 -> u64
        let s_256_4: u64 = (s_256_3.value() as u64);
        // C s_256_5: const #31s : i
        let s_256_5: i128 = 31;
        // D s_256_6: cast zx s_256_4 -> bv
        let s_256_6: Bits = Bits::new(s_256_4 as u128, 64u16);
        // C s_256_7: const #1u : u8
        let s_256_7: bool = true;
        // C s_256_8: cast zx s_256_7 -> bv
        let s_256_8: Bits = Bits::new(s_256_7 as u128, 1u16);
        // C s_256_9: const #0s : i
        let s_256_9: i128 = 0;
        // C s_256_10: const #1u : u64
        let s_256_10: u64 = 1;
        // C s_256_11: cast zx s_256_10 -> bv
        let s_256_11: Bits = Bits::new(s_256_10 as u128, 64u16);
        // C s_256_12: lsl s_256_11 s_256_9
        let s_256_12: Bits = s_256_11 << s_256_9;
        // C s_256_13: sub s_256_12 s_256_11
        let s_256_13: Bits = ((s_256_12) - (s_256_11));
        // C s_256_14: and s_256_8 s_256_13
        let s_256_14: Bits = ((s_256_8) & (s_256_13));
        // C s_256_15: lsl s_256_14 s_256_5
        let s_256_15: Bits = s_256_14 << s_256_5;
        // C s_256_16: lsl s_256_13 s_256_5
        let s_256_16: Bits = s_256_13 << s_256_5;
        // C s_256_17: cmpl s_256_16
        let s_256_17: Bits = !s_256_16;
        // D s_256_18: and s_256_6 s_256_17
        let s_256_18: Bits = ((s_256_6) & (s_256_17));
        // D s_256_19: or s_256_18 s_256_15
        let s_256_19: Bits = ((s_256_18) | (s_256_15));
        // D s_256_20: cast reint s_256_19 -> u64
        let s_256_20: u64 = (s_256_19.value() as u64);
        // D s_256_21: cast zx s_256_20 -> bv
        let s_256_21: Bits = Bits::new(s_256_20 as u128, 64u16);
        // D s_256_22: read-var t:i
        let s_256_22: i128 = fn_state.t;
        // D s_256_23: call X_set(s_256_22, s_256_0, s_256_21)
        let s_256_23: () = X_set(state, tracer, s_256_22, s_256_0, s_256_21);
        // N s_256_24: jump b255
        return block_255(state, tracer, fn_state);
    }
    fn block_257<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_257_0: const #16975u : u32
        let s_257_0: u32 = 16975;
        // D s_257_1: read-reg s_257_0:u8
        let s_257_1: u8 = {
            let value = state.read_register::<u8>(s_257_0 as isize);
            tracer.read_register(s_257_0 as isize, value);
            value
        };
        // D s_257_2: cast zx s_257_1 -> bv
        let s_257_2: Bits = Bits::new(s_257_1 as u128, 2u16);
        // C s_257_3: const #440u : u32
        let s_257_3: u32 = 440;
        // D s_257_4: read-reg s_257_3:u8
        let s_257_4: u8 = {
            let value = state.read_register::<u8>(s_257_3 as isize);
            tracer.read_register(s_257_3 as isize, value);
            value
        };
        // D s_257_5: cast zx s_257_4 -> bv
        let s_257_5: Bits = Bits::new(s_257_4 as u128, 2u16);
        // D s_257_6: cmp-ne s_257_2 s_257_5
        let s_257_6: bool = ((s_257_2) != (s_257_5));
        // D s_257_7: write-var gs#140742 <= s_257_6
        fn_state.gs_140742 = s_257_6;
        // N s_257_8: jump b253
        return block_253(state, tracer, fn_state);
    }
    fn block_258<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_258_0: const #1s : i
        let s_258_0: i128 = 1;
        // D s_258_1: read-var crm:i
        let s_258_1: i128 = fn_state.crm;
        // D s_258_2: cmp-eq s_258_1 s_258_0
        let s_258_2: bool = ((s_258_1) == (s_258_0));
        // D s_258_3: write-var gs#140725 <= s_258_2
        fn_state.gs_140725 = s_258_2;
        // N s_258_4: jump b232
        return block_232(state, tracer, fn_state);
    }
    fn block_259<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_259_0: const #0s : i
        let s_259_0: i128 = 0;
        // D s_259_1: read-var op2:i
        let s_259_1: i128 = fn_state.op2;
        // D s_259_2: cmp-eq s_259_1 s_259_0
        let s_259_2: bool = ((s_259_1) == (s_259_0));
        // D s_259_3: write-var gs#140723 <= s_259_2
        fn_state.gs_140723 = s_259_2;
        // N s_259_4: jump b230
        return block_230(state, tracer, fn_state);
    }
    fn block_260<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_260_0: const #4s : i
        let s_260_0: i128 = 4;
        // D s_260_1: read-var op1:i
        let s_260_1: i128 = fn_state.op1;
        // D s_260_2: cmp-eq s_260_1 s_260_0
        let s_260_2: bool = ((s_260_1) == (s_260_0));
        // D s_260_3: write-var gs#140721 <= s_260_2
        fn_state.gs_140721 = s_260_2;
        // N s_260_4: jump b228
        return block_228(state, tracer, fn_state);
    }
    fn block_261<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_261_0: const #1s : i
        let s_261_0: i128 = 1;
        // D s_261_1: read-var crn:i
        let s_261_1: i128 = fn_state.crn;
        // D s_261_2: cmp-eq s_261_1 s_261_0
        let s_261_2: bool = ((s_261_1) == (s_261_0));
        // D s_261_3: write-var gs#140719 <= s_261_2
        fn_state.gs_140719 = s_261_2;
        // N s_261_4: jump b226
        return block_226(state, tracer, fn_state);
    }
    fn block_262<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_262_0: const #64s : i64
        let s_262_0: i64 = 64;
        // C s_262_1: const #64s : i64
        let s_262_1: i64 = 64;
        // D s_262_2: read-var t:i
        let s_262_2: i128 = fn_state.t;
        // D s_262_3: call X_read(s_262_2, s_262_1)
        let s_262_3: Bits = X_read(state, tracer, s_262_2, s_262_1);
        // D s_262_4: cast reint s_262_3 -> u64
        let s_262_4: u64 = (s_262_3.value() as u64);
        // C s_262_5: const #0s : i
        let s_262_5: i128 = 0;
        // D s_262_6: cast zx s_262_4 -> bv
        let s_262_6: Bits = Bits::new(s_262_4 as u128, 64u16);
        // C s_262_7: const #0u : u8
        let s_262_7: u8 = 0;
        // C s_262_8: cast zx s_262_7 -> bv
        let s_262_8: Bits = Bits::new(s_262_7 as u128, 3u16);
        // C s_262_9: const #2s : i
        let s_262_9: i128 = 2;
        // C s_262_10: const #1u : u64
        let s_262_10: u64 = 1;
        // C s_262_11: cast zx s_262_10 -> bv
        let s_262_11: Bits = Bits::new(s_262_10 as u128, 64u16);
        // C s_262_12: lsl s_262_11 s_262_9
        let s_262_12: Bits = s_262_11 << s_262_9;
        // C s_262_13: sub s_262_12 s_262_11
        let s_262_13: Bits = ((s_262_12) - (s_262_11));
        // C s_262_14: and s_262_8 s_262_13
        let s_262_14: Bits = ((s_262_8) & (s_262_13));
        // C s_262_15: lsl s_262_14 s_262_5
        let s_262_15: Bits = s_262_14 << s_262_5;
        // C s_262_16: lsl s_262_13 s_262_5
        let s_262_16: Bits = s_262_13 << s_262_5;
        // C s_262_17: cmpl s_262_16
        let s_262_17: Bits = !s_262_16;
        // D s_262_18: and s_262_6 s_262_17
        let s_262_18: Bits = ((s_262_6) & (s_262_17));
        // D s_262_19: or s_262_18 s_262_15
        let s_262_19: Bits = ((s_262_18) | (s_262_15));
        // D s_262_20: cast reint s_262_19 -> u64
        let s_262_20: u64 = (s_262_19.value() as u64);
        // D s_262_21: cast zx s_262_20 -> bv
        let s_262_21: Bits = Bits::new(s_262_20 as u128, 64u16);
        // D s_262_22: read-var t:i
        let s_262_22: i128 = fn_state.t;
        // D s_262_23: call X_set(s_262_22, s_262_0, s_262_21)
        let s_262_23: () = X_set(state, tracer, s_262_22, s_262_0, s_262_21);
        // N s_262_24: jump b224
        return block_224(state, tracer, fn_state);
    }
    fn block_263<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_263_0: const #6s : i
        let s_263_0: i128 = 6;
        // D s_263_1: read-var crm:i
        let s_263_1: i128 = fn_state.crm;
        // D s_263_2: cmp-eq s_263_1 s_263_0
        let s_263_2: bool = ((s_263_1) == (s_263_0));
        // D s_263_3: write-var gs#140716 <= s_263_2
        fn_state.gs_140716 = s_263_2;
        // N s_263_4: jump b222
        return block_222(state, tracer, fn_state);
    }
    fn block_264<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_264_0: const #0s : i
        let s_264_0: i128 = 0;
        // D s_264_1: read-var op2:i
        let s_264_1: i128 = fn_state.op2;
        // D s_264_2: cmp-eq s_264_1 s_264_0
        let s_264_2: bool = ((s_264_1) == (s_264_0));
        // D s_264_3: write-var gs#140714 <= s_264_2
        fn_state.gs_140714 = s_264_2;
        // N s_264_4: jump b220
        return block_220(state, tracer, fn_state);
    }
    fn block_265<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_265_0: const #0s : i
        let s_265_0: i128 = 0;
        // D s_265_1: read-var op1:i
        let s_265_1: i128 = fn_state.op1;
        // D s_265_2: cmp-eq s_265_1 s_265_0
        let s_265_2: bool = ((s_265_1) == (s_265_0));
        // D s_265_3: write-var gs#140712 <= s_265_2
        fn_state.gs_140712 = s_265_2;
        // N s_265_4: jump b218
        return block_218(state, tracer, fn_state);
    }
    fn block_266<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_266_0: const #4s : i
        let s_266_0: i128 = 4;
        // D s_266_1: read-var crn:i
        let s_266_1: i128 = fn_state.crn;
        // D s_266_2: cmp-eq s_266_1 s_266_0
        let s_266_2: bool = ((s_266_1) == (s_266_0));
        // D s_266_3: write-var gs#140710 <= s_266_2
        fn_state.gs_140710 = s_266_2;
        // N s_266_4: jump b216
        return block_216(state, tracer, fn_state);
    }
    fn block_267<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_267_0: const #16975u : u32
        let s_267_0: u32 = 16975;
        // D s_267_1: read-reg s_267_0:u8
        let s_267_1: u8 = {
            let value = state.read_register::<u8>(s_267_0 as isize);
            tracer.read_register(s_267_0 as isize, value);
            value
        };
        // D s_267_2: cast zx s_267_1 -> bv
        let s_267_2: Bits = Bits::new(s_267_1 as u128, 2u16);
        // C s_267_3: const #432u : u32
        let s_267_3: u32 = 432;
        // D s_267_4: read-reg s_267_3:u8
        let s_267_4: u8 = {
            let value = state.read_register::<u8>(s_267_3 as isize);
            tracer.read_register(s_267_3 as isize, value);
            value
        };
        // D s_267_5: cast zx s_267_4 -> bv
        let s_267_5: Bits = Bits::new(s_267_4 as u128, 2u16);
        // D s_267_6: cmp-eq s_267_2 s_267_5
        let s_267_6: bool = ((s_267_2) == (s_267_5));
        // N s_267_7: branch s_267_6 b273 b268
        if s_267_6 {
            return block_273(state, tracer, fn_state);
        } else {
            return block_268(state, tracer, fn_state);
        };
    }
    fn block_268<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_268_0: const #16975u : u32
        let s_268_0: u32 = 16975;
        // D s_268_1: read-reg s_268_0:u8
        let s_268_1: u8 = {
            let value = state.read_register::<u8>(s_268_0 as isize);
            tracer.read_register(s_268_0 as isize, value);
            value
        };
        // D s_268_2: cast zx s_268_1 -> bv
        let s_268_2: Bits = Bits::new(s_268_1 as u128, 2u16);
        // C s_268_3: const #424u : u32
        let s_268_3: u32 = 424;
        // D s_268_4: read-reg s_268_3:u8
        let s_268_4: u8 = {
            let value = state.read_register::<u8>(s_268_3 as isize);
            tracer.read_register(s_268_3 as isize, value);
            value
        };
        // D s_268_5: cast zx s_268_4 -> bv
        let s_268_5: Bits = Bits::new(s_268_4 as u128, 2u16);
        // D s_268_6: cmp-eq s_268_2 s_268_5
        let s_268_6: bool = ((s_268_2) == (s_268_5));
        // D s_268_7: write-var gs#140754 <= s_268_6
        fn_state.gs_140754 = s_268_6;
        // N s_268_8: jump b269
        return block_269(state, tracer, fn_state);
    }
    fn block_269<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_269_0: read-var gs#140754:u8
        let s_269_0: bool = fn_state.gs_140754;
        // N s_269_1: branch s_269_0 b272 b270
        if s_269_0 {
            return block_272(state, tracer, fn_state);
        } else {
            return block_270(state, tracer, fn_state);
        };
    }
    fn block_270<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_270_0: jump b271
        return block_271(state, tracer, fn_state);
    }
    fn block_271<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_271_0: jump b214
        return block_214(state, tracer, fn_state);
    }
    fn block_272<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_272_0: const #64s : i64
        let s_272_0: i64 = 64;
        // C s_272_1: const #64s : i64
        let s_272_1: i64 = 64;
        // D s_272_2: read-var t:i
        let s_272_2: i128 = fn_state.t;
        // D s_272_3: call X_read(s_272_2, s_272_1)
        let s_272_3: Bits = X_read(state, tracer, s_272_2, s_272_1);
        // D s_272_4: cast reint s_272_3 -> u64
        let s_272_4: u64 = (s_272_3.value() as u64);
        // C s_272_5: const #24s : i
        let s_272_5: i128 = 24;
        // D s_272_6: cast zx s_272_4 -> bv
        let s_272_6: Bits = Bits::new(s_272_4 as u128, 64u16);
        // C s_272_7: const #0u : u8
        let s_272_7: u8 = 0;
        // C s_272_8: cast zx s_272_7 -> bv
        let s_272_8: Bits = Bits::new(s_272_7 as u128, 3u16);
        // C s_272_9: const #2s : i
        let s_272_9: i128 = 2;
        // C s_272_10: const #1u : u64
        let s_272_10: u64 = 1;
        // C s_272_11: cast zx s_272_10 -> bv
        let s_272_11: Bits = Bits::new(s_272_10 as u128, 64u16);
        // C s_272_12: lsl s_272_11 s_272_9
        let s_272_12: Bits = s_272_11 << s_272_9;
        // C s_272_13: sub s_272_12 s_272_11
        let s_272_13: Bits = ((s_272_12) - (s_272_11));
        // C s_272_14: and s_272_8 s_272_13
        let s_272_14: Bits = ((s_272_8) & (s_272_13));
        // C s_272_15: lsl s_272_14 s_272_5
        let s_272_15: Bits = s_272_14 << s_272_5;
        // C s_272_16: lsl s_272_13 s_272_5
        let s_272_16: Bits = s_272_13 << s_272_5;
        // C s_272_17: cmpl s_272_16
        let s_272_17: Bits = !s_272_16;
        // D s_272_18: and s_272_6 s_272_17
        let s_272_18: Bits = ((s_272_6) & (s_272_17));
        // D s_272_19: or s_272_18 s_272_15
        let s_272_19: Bits = ((s_272_18) | (s_272_15));
        // D s_272_20: cast reint s_272_19 -> u64
        let s_272_20: u64 = (s_272_19.value() as u64);
        // D s_272_21: cast zx s_272_20 -> bv
        let s_272_21: Bits = Bits::new(s_272_20 as u128, 64u16);
        // D s_272_22: read-var t:i
        let s_272_22: i128 = fn_state.t;
        // D s_272_23: call X_set(s_272_22, s_272_0, s_272_21)
        let s_272_23: () = X_set(state, tracer, s_272_22, s_272_0, s_272_21);
        // N s_272_24: jump b271
        return block_271(state, tracer, fn_state);
    }
    fn block_273<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_273_0: const #1u : u8
        let s_273_0: bool = true;
        // D s_273_1: write-var gs#140754 <= s_273_0
        fn_state.gs_140754 = s_273_0;
        // N s_273_2: jump b269
        return block_269(state, tracer, fn_state);
    }
    fn block_274<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_274_0: const #11s : i
        let s_274_0: i128 = 11;
        // D s_274_1: read-var crm:i
        let s_274_1: i128 = fn_state.crm;
        // D s_274_2: cmp-eq s_274_1 s_274_0
        let s_274_2: bool = ((s_274_1) == (s_274_0));
        // D s_274_3: write-var gs#140707 <= s_274_2
        fn_state.gs_140707 = s_274_2;
        // N s_274_4: jump b212
        return block_212(state, tracer, fn_state);
    }
    fn block_275<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_275_0: const #7s : i
        let s_275_0: i128 = 7;
        // D s_275_1: read-var op2:i
        let s_275_1: i128 = fn_state.op2;
        // D s_275_2: cmp-eq s_275_1 s_275_0
        let s_275_2: bool = ((s_275_1) == (s_275_0));
        // D s_275_3: write-var gs#140705 <= s_275_2
        fn_state.gs_140705 = s_275_2;
        // N s_275_4: jump b210
        return block_210(state, tracer, fn_state);
    }
    fn block_276<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_276_0: const #4s : i
        let s_276_0: i128 = 4;
        // D s_276_1: read-var op1:i
        let s_276_1: i128 = fn_state.op1;
        // D s_276_2: cmp-eq s_276_1 s_276_0
        let s_276_2: bool = ((s_276_1) == (s_276_0));
        // D s_276_3: write-var gs#140703 <= s_276_2
        fn_state.gs_140703 = s_276_2;
        // N s_276_4: jump b208
        return block_208(state, tracer, fn_state);
    }
    fn block_277<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_277_0: const #12s : i
        let s_277_0: i128 = 12;
        // D s_277_1: read-var crn:i
        let s_277_1: i128 = fn_state.crn;
        // D s_277_2: cmp-eq s_277_1 s_277_0
        let s_277_2: bool = ((s_277_1) == (s_277_0));
        // D s_277_3: write-var gs#140701 <= s_277_2
        fn_state.gs_140701 = s_277_2;
        // N s_277_4: jump b206
        return block_206(state, tracer, fn_state);
    }
    fn block_278<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_278_0: const #() : ()
        let s_278_0: () = ();
        // S s_278_1: call DBGDSCRint_read(s_278_0)
        let s_278_1: ProductType700c18a878c5601b = DBGDSCRint_read(
            state,
            tracer,
            s_278_0,
        );
        // C s_278_2: const #0u : u8
        let s_278_2: bool = false;
        // S s_278_3: call _update_DBGDSCRint_Type_RXfull(s_278_1, s_278_2)
        let s_278_3: ProductType700c18a878c5601b = u_update_DBGDSCRint_Type_RXfull(
            state,
            tracer,
            s_278_1,
            s_278_2,
        );
        // S s_278_4: call DBGDSCRint_write(s_278_3)
        let s_278_4: () = DBGDSCRint_write(state, tracer, s_278_3);
        // C s_278_5: const #16832u : u32
        let s_278_5: u32 = 16832;
        // D s_278_6: read-reg s_278_5:struct
        let s_278_6: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_278_5 as isize);
            tracer.read_register(s_278_5 as isize, value);
            value
        };
        // C s_278_7: const #16832u : u32
        let s_278_7: u32 = 16832;
        // N s_278_8: write-reg s_278_7 <= s_278_6
        let s_278_8: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_278_7 as isize, s_278_6);
            tracer.write_register(s_278_7 as isize, s_278_6);
        };
        // C s_278_9: const #() : ()
        let s_278_9: () = ();
        // S s_278_10: call DBGDSCRext_read(s_278_9)
        let s_278_10: ProductType700c18a878c5601b = DBGDSCRext_read(
            state,
            tracer,
            s_278_9,
        );
        // C s_278_11: const #0u : u8
        let s_278_11: bool = false;
        // S s_278_12: call _update_DBGDSCRext_Type_RXfull(s_278_10, s_278_11)
        let s_278_12: ProductType700c18a878c5601b = u_update_DBGDSCRext_Type_RXfull(
            state,
            tracer,
            s_278_10,
            s_278_11,
        );
        // S s_278_13: call DBGDSCRext_write(s_278_12)
        let s_278_13: () = DBGDSCRext_write(state, tracer, s_278_12);
        // C s_278_14: const #() : ()
        let s_278_14: () = ();
        // S s_278_15: call EDSCR_read(s_278_14)
        let s_278_15: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_278_14);
        // C s_278_16: const #0u : u8
        let s_278_16: bool = false;
        // S s_278_17: call _update_EDSCR_Type_RXfull(s_278_15, s_278_16)
        let s_278_17: ProductType700c18a878c5601b = u_update_EDSCR_Type_RXfull(
            state,
            tracer,
            s_278_15,
            s_278_16,
        );
        // S s_278_18: call EDSCR_write(s_278_17)
        let s_278_18: () = EDSCR_write(state, tracer, s_278_17);
        // C s_278_19: const #64s : i64
        let s_278_19: i64 = 64;
        // C s_278_20: const #64s : i64
        let s_278_20: i64 = 64;
        // D s_278_21: read-var t:i
        let s_278_21: i128 = fn_state.t;
        // D s_278_22: call X_read(s_278_21, s_278_20)
        let s_278_22: Bits = X_read(state, tracer, s_278_21, s_278_20);
        // D s_278_23: cast reint s_278_22 -> u64
        let s_278_23: u64 = (s_278_22.value() as u64);
        // C s_278_24: const #0s : i
        let s_278_24: i128 = 0;
        // C s_278_25: const #32s : i
        let s_278_25: i128 = 32;
        // C s_278_26: const #17232u : u32
        let s_278_26: u32 = 17232;
        // D s_278_27: read-reg s_278_26:u64
        let s_278_27: u64 = {
            let value = state.read_register::<u64>(s_278_26 as isize);
            tracer.read_register(s_278_26 as isize, value);
            value
        };
        // D s_278_28: cast zx s_278_27 -> bv
        let s_278_28: Bits = Bits::new(s_278_27 as u128, 64u16);
        // D s_278_29: bit-extract s_278_28 s_278_24 s_278_25
        let s_278_29: Bits = (Bits::new(
            ((s_278_28) >> (s_278_24)).value(),
            u16::try_from(s_278_25).unwrap(),
        ));
        // D s_278_30: cast reint s_278_29 -> u32
        let s_278_30: u32 = (s_278_29.value() as u32);
        // C s_278_31: const #32s : i
        let s_278_31: i128 = 32;
        // C s_278_32: const #32s : i
        let s_278_32: i128 = 32;
        // D s_278_33: cast zx s_278_23 -> bv
        let s_278_33: Bits = Bits::new(s_278_23 as u128, 64u16);
        // D s_278_34: cast zx s_278_30 -> bv
        let s_278_34: Bits = Bits::new(s_278_30 as u128, 32u16);
        // C s_278_35: const #1u : u64
        let s_278_35: u64 = 1;
        // C s_278_36: cast zx s_278_35 -> bv
        let s_278_36: Bits = Bits::new(s_278_35 as u128, 64u16);
        // C s_278_37: lsl s_278_36 s_278_31
        let s_278_37: Bits = s_278_36 << s_278_31;
        // C s_278_38: sub s_278_37 s_278_36
        let s_278_38: Bits = ((s_278_37) - (s_278_36));
        // D s_278_39: and s_278_34 s_278_38
        let s_278_39: Bits = ((s_278_34) & (s_278_38));
        // D s_278_40: lsl s_278_39 s_278_32
        let s_278_40: Bits = s_278_39 << s_278_32;
        // C s_278_41: lsl s_278_38 s_278_32
        let s_278_41: Bits = s_278_38 << s_278_32;
        // C s_278_42: cmpl s_278_41
        let s_278_42: Bits = !s_278_41;
        // D s_278_43: and s_278_33 s_278_42
        let s_278_43: Bits = ((s_278_33) & (s_278_42));
        // D s_278_44: or s_278_43 s_278_40
        let s_278_44: Bits = ((s_278_43) | (s_278_40));
        // D s_278_45: cast reint s_278_44 -> u64
        let s_278_45: u64 = (s_278_44.value() as u64);
        // D s_278_46: cast zx s_278_45 -> bv
        let s_278_46: Bits = Bits::new(s_278_45 as u128, 64u16);
        // D s_278_47: read-var t:i
        let s_278_47: i128 = fn_state.t;
        // D s_278_48: call X_set(s_278_47, s_278_19, s_278_46)
        let s_278_48: () = X_set(state, tracer, s_278_47, s_278_19, s_278_46);
        // C s_278_49: const #64s : i64
        let s_278_49: i64 = 64;
        // C s_278_50: const #64s : i64
        let s_278_50: i64 = 64;
        // D s_278_51: read-var t:i
        let s_278_51: i128 = fn_state.t;
        // D s_278_52: call X_read(s_278_51, s_278_50)
        let s_278_52: Bits = X_read(state, tracer, s_278_51, s_278_50);
        // D s_278_53: cast reint s_278_52 -> u64
        let s_278_53: u64 = (s_278_52.value() as u64);
        // C s_278_54: const #0s : i
        let s_278_54: i128 = 0;
        // C s_278_55: const #32s : i
        let s_278_55: i128 = 32;
        // C s_278_56: const #103184u : u32
        let s_278_56: u32 = 103184;
        // D s_278_57: read-reg s_278_56:u64
        let s_278_57: u64 = {
            let value = state.read_register::<u64>(s_278_56 as isize);
            tracer.read_register(s_278_56 as isize, value);
            value
        };
        // D s_278_58: cast zx s_278_57 -> bv
        let s_278_58: Bits = Bits::new(s_278_57 as u128, 64u16);
        // D s_278_59: bit-extract s_278_58 s_278_54 s_278_55
        let s_278_59: Bits = (Bits::new(
            ((s_278_58) >> (s_278_54)).value(),
            u16::try_from(s_278_55).unwrap(),
        ));
        // D s_278_60: cast reint s_278_59 -> u32
        let s_278_60: u32 = (s_278_59.value() as u32);
        // C s_278_61: const #32s : i
        let s_278_61: i128 = 32;
        // C s_278_62: const #0s : i
        let s_278_62: i128 = 0;
        // D s_278_63: cast zx s_278_53 -> bv
        let s_278_63: Bits = Bits::new(s_278_53 as u128, 64u16);
        // D s_278_64: cast zx s_278_60 -> bv
        let s_278_64: Bits = Bits::new(s_278_60 as u128, 32u16);
        // C s_278_65: const #1u : u64
        let s_278_65: u64 = 1;
        // C s_278_66: cast zx s_278_65 -> bv
        let s_278_66: Bits = Bits::new(s_278_65 as u128, 64u16);
        // C s_278_67: lsl s_278_66 s_278_61
        let s_278_67: Bits = s_278_66 << s_278_61;
        // C s_278_68: sub s_278_67 s_278_66
        let s_278_68: Bits = ((s_278_67) - (s_278_66));
        // D s_278_69: and s_278_64 s_278_68
        let s_278_69: Bits = ((s_278_64) & (s_278_68));
        // D s_278_70: lsl s_278_69 s_278_62
        let s_278_70: Bits = s_278_69 << s_278_62;
        // C s_278_71: lsl s_278_68 s_278_62
        let s_278_71: Bits = s_278_68 << s_278_62;
        // C s_278_72: cmpl s_278_71
        let s_278_72: Bits = !s_278_71;
        // D s_278_73: and s_278_63 s_278_72
        let s_278_73: Bits = ((s_278_63) & (s_278_72));
        // D s_278_74: or s_278_73 s_278_70
        let s_278_74: Bits = ((s_278_73) | (s_278_70));
        // D s_278_75: cast reint s_278_74 -> u64
        let s_278_75: u64 = (s_278_74.value() as u64);
        // D s_278_76: cast zx s_278_75 -> bv
        let s_278_76: Bits = Bits::new(s_278_75 as u128, 64u16);
        // D s_278_77: read-var t:i
        let s_278_77: i128 = fn_state.t;
        // D s_278_78: call X_set(s_278_77, s_278_49, s_278_76)
        let s_278_78: () = X_set(state, tracer, s_278_77, s_278_49, s_278_76);
        // N s_278_79: jump b204
        return block_204(state, tracer, fn_state);
    }
    fn block_279<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_279_0: const #4s : i
        let s_279_0: i128 = 4;
        // D s_279_1: read-var crm:i
        let s_279_1: i128 = fn_state.crm;
        // D s_279_2: cmp-eq s_279_1 s_279_0
        let s_279_2: bool = ((s_279_1) == (s_279_0));
        // D s_279_3: write-var gs#140698 <= s_279_2
        fn_state.gs_140698 = s_279_2;
        // N s_279_4: jump b202
        return block_202(state, tracer, fn_state);
    }
    fn block_280<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_280_0: const #0s : i
        let s_280_0: i128 = 0;
        // D s_280_1: read-var op2:i
        let s_280_1: i128 = fn_state.op2;
        // D s_280_2: cmp-eq s_280_1 s_280_0
        let s_280_2: bool = ((s_280_1) == (s_280_0));
        // D s_280_3: write-var gs#140696 <= s_280_2
        fn_state.gs_140696 = s_280_2;
        // N s_280_4: jump b200
        return block_200(state, tracer, fn_state);
    }
    fn block_281<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_281_0: const #3s : i
        let s_281_0: i128 = 3;
        // D s_281_1: read-var op1:i
        let s_281_1: i128 = fn_state.op1;
        // D s_281_2: cmp-eq s_281_1 s_281_0
        let s_281_2: bool = ((s_281_1) == (s_281_0));
        // D s_281_3: write-var gs#140694 <= s_281_2
        fn_state.gs_140694 = s_281_2;
        // N s_281_4: jump b198
        return block_198(state, tracer, fn_state);
    }
    fn block_282<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_282_0: const #0s : i
        let s_282_0: i128 = 0;
        // D s_282_1: read-var crn:i
        let s_282_1: i128 = fn_state.crn;
        // D s_282_2: cmp-eq s_282_1 s_282_0
        let s_282_2: bool = ((s_282_1) == (s_282_0));
        // D s_282_3: write-var gs#140692 <= s_282_2
        fn_state.gs_140692 = s_282_2;
        // N s_282_4: jump b196
        return block_196(state, tracer, fn_state);
    }
    fn block_283<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_283_0: const #() : ()
        let s_283_0: () = ();
        // S s_283_1: call DBGDSCRint_read(s_283_0)
        let s_283_1: ProductType700c18a878c5601b = DBGDSCRint_read(
            state,
            tracer,
            s_283_0,
        );
        // C s_283_2: const #0u : u8
        let s_283_2: bool = false;
        // S s_283_3: call _update_DBGDSCRint_Type_RXfull(s_283_1, s_283_2)
        let s_283_3: ProductType700c18a878c5601b = u_update_DBGDSCRint_Type_RXfull(
            state,
            tracer,
            s_283_1,
            s_283_2,
        );
        // S s_283_4: call DBGDSCRint_write(s_283_3)
        let s_283_4: () = DBGDSCRint_write(state, tracer, s_283_3);
        // C s_283_5: const #16832u : u32
        let s_283_5: u32 = 16832;
        // D s_283_6: read-reg s_283_5:struct
        let s_283_6: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_283_5 as isize);
            tracer.read_register(s_283_5 as isize, value);
            value
        };
        // C s_283_7: const #16832u : u32
        let s_283_7: u32 = 16832;
        // N s_283_8: write-reg s_283_7 <= s_283_6
        let s_283_8: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_283_7 as isize, s_283_6);
            tracer.write_register(s_283_7 as isize, s_283_6);
        };
        // C s_283_9: const #() : ()
        let s_283_9: () = ();
        // S s_283_10: call DBGDSCRext_read(s_283_9)
        let s_283_10: ProductType700c18a878c5601b = DBGDSCRext_read(
            state,
            tracer,
            s_283_9,
        );
        // C s_283_11: const #0u : u8
        let s_283_11: bool = false;
        // S s_283_12: call _update_DBGDSCRext_Type_RXfull(s_283_10, s_283_11)
        let s_283_12: ProductType700c18a878c5601b = u_update_DBGDSCRext_Type_RXfull(
            state,
            tracer,
            s_283_10,
            s_283_11,
        );
        // S s_283_13: call DBGDSCRext_write(s_283_12)
        let s_283_13: () = DBGDSCRext_write(state, tracer, s_283_12);
        // C s_283_14: const #() : ()
        let s_283_14: () = ();
        // S s_283_15: call EDSCR_read(s_283_14)
        let s_283_15: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_283_14);
        // C s_283_16: const #0u : u8
        let s_283_16: bool = false;
        // S s_283_17: call _update_EDSCR_Type_RXfull(s_283_15, s_283_16)
        let s_283_17: ProductType700c18a878c5601b = u_update_EDSCR_Type_RXfull(
            state,
            tracer,
            s_283_15,
            s_283_16,
        );
        // S s_283_18: call EDSCR_write(s_283_17)
        let s_283_18: () = EDSCR_write(state, tracer, s_283_17);
        // N s_283_19: jump b194
        return block_194(state, tracer, fn_state);
    }
    fn block_284<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_284_0: const #5s : i
        let s_284_0: i128 = 5;
        // D s_284_1: read-var crm:i
        let s_284_1: i128 = fn_state.crm;
        // D s_284_2: cmp-eq s_284_1 s_284_0
        let s_284_2: bool = ((s_284_1) == (s_284_0));
        // D s_284_3: write-var gs#140689 <= s_284_2
        fn_state.gs_140689 = s_284_2;
        // N s_284_4: jump b192
        return block_192(state, tracer, fn_state);
    }
    fn block_285<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_285_0: const #0s : i
        let s_285_0: i128 = 0;
        // D s_285_1: read-var op2:i
        let s_285_1: i128 = fn_state.op2;
        // D s_285_2: cmp-eq s_285_1 s_285_0
        let s_285_2: bool = ((s_285_1) == (s_285_0));
        // D s_285_3: write-var gs#140687 <= s_285_2
        fn_state.gs_140687 = s_285_2;
        // N s_285_4: jump b190
        return block_190(state, tracer, fn_state);
    }
    fn block_286<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_286_0: const #3s : i
        let s_286_0: i128 = 3;
        // D s_286_1: read-var op1:i
        let s_286_1: i128 = fn_state.op1;
        // D s_286_2: cmp-eq s_286_1 s_286_0
        let s_286_2: bool = ((s_286_1) == (s_286_0));
        // D s_286_3: write-var gs#140685 <= s_286_2
        fn_state.gs_140685 = s_286_2;
        // N s_286_4: jump b188
        return block_188(state, tracer, fn_state);
    }
    fn block_287<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_287_0: const #0s : i
        let s_287_0: i128 = 0;
        // D s_287_1: read-var crn:i
        let s_287_1: i128 = fn_state.crn;
        // D s_287_2: cmp-eq s_287_1 s_287_0
        let s_287_2: bool = ((s_287_1) == (s_287_0));
        // D s_287_3: write-var gs#140683 <= s_287_2
        fn_state.gs_140683 = s_287_2;
        // N s_287_4: jump b186
        return block_186(state, tracer, fn_state);
    }
    fn block_288<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_288_0: const #424u : u32
        let s_288_0: u32 = 424;
        // D s_288_1: read-reg s_288_0:u8
        let s_288_1: u8 = {
            let value = state.read_register::<u8>(s_288_0 as isize);
            tracer.read_register(s_288_0 as isize, value);
            value
        };
        // C s_288_2: const #2u : u8
        let s_288_2: u8 = 2;
        // D s_288_3: cmp-lt s_288_1 s_288_2
        let s_288_3: bool = ((s_288_1) < (s_288_2));
        // N s_288_4: branch s_288_3 b290 b289
        if s_288_3 {
            return block_290(state, tracer, fn_state);
        } else {
            return block_289(state, tracer, fn_state);
        };
    }
    fn block_289<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_289_0: return
        return;
    }
    fn block_290<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_290_0: const #() : ()
        let s_290_0: () = ();
        // S s_290_1: call ExternalInvasiveDebugEnabled(s_290_0)
        let s_290_1: bool = ExternalInvasiveDebugEnabled(state, tracer, s_290_0);
        // N s_290_2: branch s_290_1 b296 b291
        if s_290_1 {
            return block_296(state, tracer, fn_state);
        } else {
            return block_291(state, tracer, fn_state);
        };
    }
    fn block_291<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_291_0: const #64s : i64
        let s_291_0: i64 = 64;
        // C s_291_1: const #64s : i64
        let s_291_1: i64 = 64;
        // D s_291_2: read-var t:i
        let s_291_2: i128 = fn_state.t;
        // D s_291_3: call X_read(s_291_2, s_291_1)
        let s_291_3: Bits = X_read(state, tracer, s_291_2, s_291_1);
        // D s_291_4: cast reint s_291_3 -> u64
        let s_291_4: u64 = (s_291_3.value() as u64);
        // C s_291_5: const #0s : i
        let s_291_5: i128 = 0;
        // D s_291_6: cast zx s_291_4 -> bv
        let s_291_6: Bits = Bits::new(s_291_4 as u128, 64u16);
        // C s_291_7: const #2u : u8
        let s_291_7: u8 = 2;
        // C s_291_8: cast zx s_291_7 -> bv
        let s_291_8: Bits = Bits::new(s_291_7 as u128, 2u16);
        // C s_291_9: const #1s : i
        let s_291_9: i128 = 1;
        // C s_291_10: const #1u : u64
        let s_291_10: u64 = 1;
        // C s_291_11: cast zx s_291_10 -> bv
        let s_291_11: Bits = Bits::new(s_291_10 as u128, 64u16);
        // C s_291_12: lsl s_291_11 s_291_9
        let s_291_12: Bits = s_291_11 << s_291_9;
        // C s_291_13: sub s_291_12 s_291_11
        let s_291_13: Bits = ((s_291_12) - (s_291_11));
        // C s_291_14: and s_291_8 s_291_13
        let s_291_14: Bits = ((s_291_8) & (s_291_13));
        // C s_291_15: lsl s_291_14 s_291_5
        let s_291_15: Bits = s_291_14 << s_291_5;
        // C s_291_16: lsl s_291_13 s_291_5
        let s_291_16: Bits = s_291_13 << s_291_5;
        // C s_291_17: cmpl s_291_16
        let s_291_17: Bits = !s_291_16;
        // D s_291_18: and s_291_6 s_291_17
        let s_291_18: Bits = ((s_291_6) & (s_291_17));
        // D s_291_19: or s_291_18 s_291_15
        let s_291_19: Bits = ((s_291_18) | (s_291_15));
        // D s_291_20: cast reint s_291_19 -> u64
        let s_291_20: u64 = (s_291_19.value() as u64);
        // D s_291_21: cast zx s_291_20 -> bv
        let s_291_21: Bits = Bits::new(s_291_20 as u128, 64u16);
        // D s_291_22: read-var t:i
        let s_291_22: i128 = fn_state.t;
        // D s_291_23: call X_set(s_291_22, s_291_0, s_291_21)
        let s_291_23: () = X_set(state, tracer, s_291_22, s_291_0, s_291_21);
        // N s_291_24: jump b292
        return block_292(state, tracer, fn_state);
    }
    fn block_292<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_292_0: const #64s : i64
        let s_292_0: i64 = 64;
        // C s_292_1: const #64s : i64
        let s_292_1: i64 = 64;
        // D s_292_2: read-var t:i
        let s_292_2: i128 = fn_state.t;
        // D s_292_3: call X_read(s_292_2, s_292_1)
        let s_292_3: Bits = X_read(state, tracer, s_292_2, s_292_1);
        // D s_292_4: cast reint s_292_3 -> u64
        let s_292_4: u64 = (s_292_3.value() as u64);
        // C s_292_5: const #2s : i
        let s_292_5: i128 = 2;
        // D s_292_6: cast zx s_292_4 -> bv
        let s_292_6: Bits = Bits::new(s_292_4 as u128, 64u16);
        // C s_292_7: const #3u : u8
        let s_292_7: u8 = 3;
        // C s_292_8: cast zx s_292_7 -> bv
        let s_292_8: Bits = Bits::new(s_292_7 as u128, 2u16);
        // C s_292_9: const #1s : i
        let s_292_9: i128 = 1;
        // C s_292_10: const #1u : u64
        let s_292_10: u64 = 1;
        // C s_292_11: cast zx s_292_10 -> bv
        let s_292_11: Bits = Bits::new(s_292_10 as u128, 64u16);
        // C s_292_12: lsl s_292_11 s_292_9
        let s_292_12: Bits = s_292_11 << s_292_9;
        // C s_292_13: sub s_292_12 s_292_11
        let s_292_13: Bits = ((s_292_12) - (s_292_11));
        // C s_292_14: and s_292_8 s_292_13
        let s_292_14: Bits = ((s_292_8) & (s_292_13));
        // C s_292_15: lsl s_292_14 s_292_5
        let s_292_15: Bits = s_292_14 << s_292_5;
        // C s_292_16: lsl s_292_13 s_292_5
        let s_292_16: Bits = s_292_13 << s_292_5;
        // C s_292_17: cmpl s_292_16
        let s_292_17: Bits = !s_292_16;
        // D s_292_18: and s_292_6 s_292_17
        let s_292_18: Bits = ((s_292_6) & (s_292_17));
        // D s_292_19: or s_292_18 s_292_15
        let s_292_19: Bits = ((s_292_18) | (s_292_15));
        // D s_292_20: cast reint s_292_19 -> u64
        let s_292_20: u64 = (s_292_19.value() as u64);
        // D s_292_21: cast zx s_292_20 -> bv
        let s_292_21: Bits = Bits::new(s_292_20 as u128, 64u16);
        // D s_292_22: read-var t:i
        let s_292_22: i128 = fn_state.t;
        // D s_292_23: call X_set(s_292_22, s_292_0, s_292_21)
        let s_292_23: () = X_set(state, tracer, s_292_22, s_292_0, s_292_21);
        // C s_292_24: const #() : ()
        let s_292_24: () = ();
        // S s_292_25: call ExternalSecureInvasiveDebugEnabled(s_292_24)
        let s_292_25: bool = ExternalSecureInvasiveDebugEnabled(state, tracer, s_292_24);
        // N s_292_26: branch s_292_25 b295 b293
        if s_292_25 {
            return block_295(state, tracer, fn_state);
        } else {
            return block_293(state, tracer, fn_state);
        };
    }
    fn block_293<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_293_0: const #64s : i64
        let s_293_0: i64 = 64;
        // C s_293_1: const #64s : i64
        let s_293_1: i64 = 64;
        // D s_293_2: read-var t:i
        let s_293_2: i128 = fn_state.t;
        // D s_293_3: call X_read(s_293_2, s_293_1)
        let s_293_3: Bits = X_read(state, tracer, s_293_2, s_293_1);
        // D s_293_4: cast reint s_293_3 -> u64
        let s_293_4: u64 = (s_293_3.value() as u64);
        // C s_293_5: const #4s : i
        let s_293_5: i128 = 4;
        // D s_293_6: cast zx s_293_4 -> bv
        let s_293_6: Bits = Bits::new(s_293_4 as u128, 64u16);
        // C s_293_7: const #2u : u8
        let s_293_7: u8 = 2;
        // C s_293_8: cast zx s_293_7 -> bv
        let s_293_8: Bits = Bits::new(s_293_7 as u128, 2u16);
        // C s_293_9: const #1s : i
        let s_293_9: i128 = 1;
        // C s_293_10: const #1u : u64
        let s_293_10: u64 = 1;
        // C s_293_11: cast zx s_293_10 -> bv
        let s_293_11: Bits = Bits::new(s_293_10 as u128, 64u16);
        // C s_293_12: lsl s_293_11 s_293_9
        let s_293_12: Bits = s_293_11 << s_293_9;
        // C s_293_13: sub s_293_12 s_293_11
        let s_293_13: Bits = ((s_293_12) - (s_293_11));
        // C s_293_14: and s_293_8 s_293_13
        let s_293_14: Bits = ((s_293_8) & (s_293_13));
        // C s_293_15: lsl s_293_14 s_293_5
        let s_293_15: Bits = s_293_14 << s_293_5;
        // C s_293_16: lsl s_293_13 s_293_5
        let s_293_16: Bits = s_293_13 << s_293_5;
        // C s_293_17: cmpl s_293_16
        let s_293_17: Bits = !s_293_16;
        // D s_293_18: and s_293_6 s_293_17
        let s_293_18: Bits = ((s_293_6) & (s_293_17));
        // D s_293_19: or s_293_18 s_293_15
        let s_293_19: Bits = ((s_293_18) | (s_293_15));
        // D s_293_20: cast reint s_293_19 -> u64
        let s_293_20: u64 = (s_293_19.value() as u64);
        // D s_293_21: cast zx s_293_20 -> bv
        let s_293_21: Bits = Bits::new(s_293_20 as u128, 64u16);
        // D s_293_22: read-var t:i
        let s_293_22: i128 = fn_state.t;
        // D s_293_23: call X_set(s_293_22, s_293_0, s_293_21)
        let s_293_23: () = X_set(state, tracer, s_293_22, s_293_0, s_293_21);
        // N s_293_24: jump b294
        return block_294(state, tracer, fn_state);
    }
    fn block_294<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_294_0: const #64s : i64
        let s_294_0: i64 = 64;
        // C s_294_1: const #64s : i64
        let s_294_1: i64 = 64;
        // D s_294_2: read-var t:i
        let s_294_2: i128 = fn_state.t;
        // D s_294_3: call X_read(s_294_2, s_294_1)
        let s_294_3: Bits = X_read(state, tracer, s_294_2, s_294_1);
        // D s_294_4: cast reint s_294_3 -> u64
        let s_294_4: u64 = (s_294_3.value() as u64);
        // C s_294_5: const #64s : i64
        let s_294_5: i64 = 64;
        // D s_294_6: read-var t:i
        let s_294_6: i128 = fn_state.t;
        // D s_294_7: call X_read(s_294_6, s_294_5)
        let s_294_7: Bits = X_read(state, tracer, s_294_6, s_294_5);
        // D s_294_8: cast reint s_294_7 -> u64
        let s_294_8: u64 = (s_294_7.value() as u64);
        // C s_294_9: const #4s : i
        let s_294_9: i128 = 4;
        // D s_294_10: cast zx s_294_8 -> bv
        let s_294_10: Bits = Bits::new(s_294_8 as u128, 64u16);
        // C s_294_11: const #1s : i64
        let s_294_11: i64 = 1;
        // C s_294_12: cast zx s_294_11 -> i
        let s_294_12: i128 = (i128::try_from(s_294_11).unwrap());
        // C s_294_13: const #1s : i
        let s_294_13: i128 = 1;
        // C s_294_14: add s_294_13 s_294_12
        let s_294_14: i128 = (s_294_13 + s_294_12);
        // D s_294_15: bit-extract s_294_10 s_294_9 s_294_14
        let s_294_15: Bits = (Bits::new(
            ((s_294_10) >> (s_294_9)).value(),
            u16::try_from(s_294_14).unwrap(),
        ));
        // D s_294_16: cast reint s_294_15 -> u8
        let s_294_16: u8 = (s_294_15.value() as u8);
        // C s_294_17: const #6s : i
        let s_294_17: i128 = 6;
        // D s_294_18: cast zx s_294_4 -> bv
        let s_294_18: Bits = Bits::new(s_294_4 as u128, 64u16);
        // D s_294_19: cast zx s_294_16 -> bv
        let s_294_19: Bits = Bits::new(s_294_16 as u128, 2u16);
        // C s_294_20: const #1s : i
        let s_294_20: i128 = 1;
        // C s_294_21: const #1u : u64
        let s_294_21: u64 = 1;
        // C s_294_22: cast zx s_294_21 -> bv
        let s_294_22: Bits = Bits::new(s_294_21 as u128, 64u16);
        // C s_294_23: lsl s_294_22 s_294_20
        let s_294_23: Bits = s_294_22 << s_294_20;
        // C s_294_24: sub s_294_23 s_294_22
        let s_294_24: Bits = ((s_294_23) - (s_294_22));
        // D s_294_25: and s_294_19 s_294_24
        let s_294_25: Bits = ((s_294_19) & (s_294_24));
        // D s_294_26: lsl s_294_25 s_294_17
        let s_294_26: Bits = s_294_25 << s_294_17;
        // C s_294_27: lsl s_294_24 s_294_17
        let s_294_27: Bits = s_294_24 << s_294_17;
        // C s_294_28: cmpl s_294_27
        let s_294_28: Bits = !s_294_27;
        // D s_294_29: and s_294_18 s_294_28
        let s_294_29: Bits = ((s_294_18) & (s_294_28));
        // D s_294_30: or s_294_29 s_294_26
        let s_294_30: Bits = ((s_294_29) | (s_294_26));
        // D s_294_31: cast reint s_294_30 -> u64
        let s_294_31: u64 = (s_294_30.value() as u64);
        // D s_294_32: cast zx s_294_31 -> bv
        let s_294_32: Bits = Bits::new(s_294_31 as u128, 64u16);
        // D s_294_33: read-var t:i
        let s_294_33: i128 = fn_state.t;
        // D s_294_34: call X_set(s_294_33, s_294_0, s_294_32)
        let s_294_34: () = X_set(state, tracer, s_294_33, s_294_0, s_294_32);
        // N s_294_35: return
        return;
    }
    fn block_295<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_295_0: const #64s : i64
        let s_295_0: i64 = 64;
        // C s_295_1: const #64s : i64
        let s_295_1: i64 = 64;
        // D s_295_2: read-var t:i
        let s_295_2: i128 = fn_state.t;
        // D s_295_3: call X_read(s_295_2, s_295_1)
        let s_295_3: Bits = X_read(state, tracer, s_295_2, s_295_1);
        // D s_295_4: cast reint s_295_3 -> u64
        let s_295_4: u64 = (s_295_3.value() as u64);
        // C s_295_5: const #4s : i
        let s_295_5: i128 = 4;
        // D s_295_6: cast zx s_295_4 -> bv
        let s_295_6: Bits = Bits::new(s_295_4 as u128, 64u16);
        // C s_295_7: const #3u : u8
        let s_295_7: u8 = 3;
        // C s_295_8: cast zx s_295_7 -> bv
        let s_295_8: Bits = Bits::new(s_295_7 as u128, 2u16);
        // C s_295_9: const #1s : i
        let s_295_9: i128 = 1;
        // C s_295_10: const #1u : u64
        let s_295_10: u64 = 1;
        // C s_295_11: cast zx s_295_10 -> bv
        let s_295_11: Bits = Bits::new(s_295_10 as u128, 64u16);
        // C s_295_12: lsl s_295_11 s_295_9
        let s_295_12: Bits = s_295_11 << s_295_9;
        // C s_295_13: sub s_295_12 s_295_11
        let s_295_13: Bits = ((s_295_12) - (s_295_11));
        // C s_295_14: and s_295_8 s_295_13
        let s_295_14: Bits = ((s_295_8) & (s_295_13));
        // C s_295_15: lsl s_295_14 s_295_5
        let s_295_15: Bits = s_295_14 << s_295_5;
        // C s_295_16: lsl s_295_13 s_295_5
        let s_295_16: Bits = s_295_13 << s_295_5;
        // C s_295_17: cmpl s_295_16
        let s_295_17: Bits = !s_295_16;
        // D s_295_18: and s_295_6 s_295_17
        let s_295_18: Bits = ((s_295_6) & (s_295_17));
        // D s_295_19: or s_295_18 s_295_15
        let s_295_19: Bits = ((s_295_18) | (s_295_15));
        // D s_295_20: cast reint s_295_19 -> u64
        let s_295_20: u64 = (s_295_19.value() as u64);
        // D s_295_21: cast zx s_295_20 -> bv
        let s_295_21: Bits = Bits::new(s_295_20 as u128, 64u16);
        // D s_295_22: read-var t:i
        let s_295_22: i128 = fn_state.t;
        // D s_295_23: call X_set(s_295_22, s_295_0, s_295_21)
        let s_295_23: () = X_set(state, tracer, s_295_22, s_295_0, s_295_21);
        // N s_295_24: jump b294
        return block_294(state, tracer, fn_state);
    }
    fn block_296<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_296_0: const #64s : i64
        let s_296_0: i64 = 64;
        // C s_296_1: const #64s : i64
        let s_296_1: i64 = 64;
        // D s_296_2: read-var t:i
        let s_296_2: i128 = fn_state.t;
        // D s_296_3: call X_read(s_296_2, s_296_1)
        let s_296_3: Bits = X_read(state, tracer, s_296_2, s_296_1);
        // D s_296_4: cast reint s_296_3 -> u64
        let s_296_4: u64 = (s_296_3.value() as u64);
        // C s_296_5: const #0s : i
        let s_296_5: i128 = 0;
        // D s_296_6: cast zx s_296_4 -> bv
        let s_296_6: Bits = Bits::new(s_296_4 as u128, 64u16);
        // C s_296_7: const #3u : u8
        let s_296_7: u8 = 3;
        // C s_296_8: cast zx s_296_7 -> bv
        let s_296_8: Bits = Bits::new(s_296_7 as u128, 2u16);
        // C s_296_9: const #1s : i
        let s_296_9: i128 = 1;
        // C s_296_10: const #1u : u64
        let s_296_10: u64 = 1;
        // C s_296_11: cast zx s_296_10 -> bv
        let s_296_11: Bits = Bits::new(s_296_10 as u128, 64u16);
        // C s_296_12: lsl s_296_11 s_296_9
        let s_296_12: Bits = s_296_11 << s_296_9;
        // C s_296_13: sub s_296_12 s_296_11
        let s_296_13: Bits = ((s_296_12) - (s_296_11));
        // C s_296_14: and s_296_8 s_296_13
        let s_296_14: Bits = ((s_296_8) & (s_296_13));
        // C s_296_15: lsl s_296_14 s_296_5
        let s_296_15: Bits = s_296_14 << s_296_5;
        // C s_296_16: lsl s_296_13 s_296_5
        let s_296_16: Bits = s_296_13 << s_296_5;
        // C s_296_17: cmpl s_296_16
        let s_296_17: Bits = !s_296_16;
        // D s_296_18: and s_296_6 s_296_17
        let s_296_18: Bits = ((s_296_6) & (s_296_17));
        // D s_296_19: or s_296_18 s_296_15
        let s_296_19: Bits = ((s_296_18) | (s_296_15));
        // D s_296_20: cast reint s_296_19 -> u64
        let s_296_20: u64 = (s_296_19.value() as u64);
        // D s_296_21: cast zx s_296_20 -> bv
        let s_296_21: Bits = Bits::new(s_296_20 as u128, 64u16);
        // D s_296_22: read-var t:i
        let s_296_22: i128 = fn_state.t;
        // D s_296_23: call X_set(s_296_22, s_296_0, s_296_21)
        let s_296_23: () = X_set(state, tracer, s_296_22, s_296_0, s_296_21);
        // N s_296_24: jump b292
        return block_292(state, tracer, fn_state);
    }
    fn block_297<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_297_0: const #14s : i
        let s_297_0: i128 = 14;
        // D s_297_1: read-var crm:i
        let s_297_1: i128 = fn_state.crm;
        // D s_297_2: cmp-eq s_297_1 s_297_0
        let s_297_2: bool = ((s_297_1) == (s_297_0));
        // D s_297_3: write-var gs#140680 <= s_297_2
        fn_state.gs_140680 = s_297_2;
        // N s_297_4: jump b183
        return block_183(state, tracer, fn_state);
    }
    fn block_298<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_298_0: const #6s : i
        let s_298_0: i128 = 6;
        // D s_298_1: read-var op2:i
        let s_298_1: i128 = fn_state.op2;
        // D s_298_2: cmp-eq s_298_1 s_298_0
        let s_298_2: bool = ((s_298_1) == (s_298_0));
        // D s_298_3: write-var gs#140678 <= s_298_2
        fn_state.gs_140678 = s_298_2;
        // N s_298_4: jump b181
        return block_181(state, tracer, fn_state);
    }
    fn block_299<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_299_0: const #0s : i
        let s_299_0: i128 = 0;
        // D s_299_1: read-var op1:i
        let s_299_1: i128 = fn_state.op1;
        // D s_299_2: cmp-eq s_299_1 s_299_0
        let s_299_2: bool = ((s_299_1) == (s_299_0));
        // D s_299_3: write-var gs#140676 <= s_299_2
        fn_state.gs_140676 = s_299_2;
        // N s_299_4: jump b179
        return block_179(state, tracer, fn_state);
    }
    fn block_300<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_300_0: const #7s : i
        let s_300_0: i128 = 7;
        // D s_300_1: read-var crn:i
        let s_300_1: i128 = fn_state.crn;
        // D s_300_2: cmp-eq s_300_1 s_300_0
        let s_300_2: bool = ((s_300_1) == (s_300_0));
        // D s_300_3: write-var gs#140674 <= s_300_2
        fn_state.gs_140674 = s_300_2;
        // N s_300_4: jump b177
        return block_177(state, tracer, fn_state);
    }
    fn block_301<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_301_0: const #15272u : u32
        let s_301_0: u32 = 15272;
        // D s_301_1: read-reg s_301_0:struct
        let s_301_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_301_0 as isize);
            tracer.read_register(s_301_0 as isize, value);
            value
        };
        // D s_301_2: call _get_ERRIDR_EL1_Type_NUM(s_301_1)
        let s_301_2: u16 = u_get_ERRIDR_EL1_Type_NUM(state, tracer, s_301_1);
        // D s_301_3: cast zx s_301_2 -> bv
        let s_301_3: Bits = Bits::new(s_301_2 as u128, 16u16);
        // D s_301_4: cast zx s_301_3 -> i
        let s_301_4: i128 = (s_301_3.value() as i128);
        // D s_301_5: cast reint s_301_4 -> i64
        let s_301_5: i64 = (s_301_4 as i64);
        // C s_301_6: const #0u : u8
        let s_301_6: u8 = 0;
        // C s_301_7: cast zx s_301_6 -> bv
        let s_301_7: Bits = Bits::new(s_301_6 as u128, 4u16);
        // C s_301_8: cast zx s_301_7 -> i
        let s_301_8: i128 = (s_301_7.value() as i128);
        // C s_301_9: cast reint s_301_8 -> i64
        let s_301_9: i64 = (s_301_8 as i64);
        // D s_301_10: cast zx s_301_5 -> i
        let s_301_10: i128 = (i128::try_from(s_301_5).unwrap());
        // C s_301_11: cast zx s_301_9 -> i
        let s_301_11: i128 = (i128::try_from(s_301_9).unwrap());
        // D s_301_12: cmp-eq s_301_10 s_301_11
        let s_301_12: bool = ((s_301_10) == (s_301_11));
        // N s_301_13: branch s_301_12 b309 b302
        if s_301_12 {
            return block_309(state, tracer, fn_state);
        } else {
            return block_302(state, tracer, fn_state);
        };
    }
    fn block_302<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_302_0: const #21792u : u32
        let s_302_0: u32 = 21792;
        // D s_302_1: read-reg s_302_0:struct
        let s_302_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_302_0 as isize);
            tracer.read_register(s_302_0 as isize, value);
            value
        };
        // D s_302_2: call _get_ERRSELR_EL1_Type_SEL(s_302_1)
        let s_302_2: u16 = u_get_ERRSELR_EL1_Type_SEL(state, tracer, s_302_1);
        // D s_302_3: cast zx s_302_2 -> bv
        let s_302_3: Bits = Bits::new(s_302_2 as u128, 16u16);
        // D s_302_4: cast zx s_302_3 -> i
        let s_302_4: i128 = (s_302_3.value() as i128);
        // D s_302_5: cast reint s_302_4 -> i64
        let s_302_5: i64 = (s_302_4 as i64);
        // C s_302_6: const #15272u : u32
        let s_302_6: u32 = 15272;
        // D s_302_7: read-reg s_302_6:struct
        let s_302_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_302_6 as isize);
            tracer.read_register(s_302_6 as isize, value);
            value
        };
        // D s_302_8: call _get_ERRIDR_EL1_Type_NUM(s_302_7)
        let s_302_8: u16 = u_get_ERRIDR_EL1_Type_NUM(state, tracer, s_302_7);
        // D s_302_9: cast zx s_302_8 -> bv
        let s_302_9: Bits = Bits::new(s_302_8 as u128, 16u16);
        // D s_302_10: cast zx s_302_9 -> i
        let s_302_10: i128 = (s_302_9.value() as i128);
        // D s_302_11: cast reint s_302_10 -> i64
        let s_302_11: i64 = (s_302_10 as i64);
        // D s_302_12: cast zx s_302_5 -> i
        let s_302_12: i128 = (i128::try_from(s_302_5).unwrap());
        // D s_302_13: cast zx s_302_11 -> i
        let s_302_13: i128 = (i128::try_from(s_302_11).unwrap());
        // D s_302_14: cmp-ge s_302_12 s_302_13
        let s_302_14: bool = ((s_302_12) >= (s_302_13));
        // D s_302_15: write-var gs#140823 <= s_302_14
        fn_state.gs_140823 = s_302_14;
        // N s_302_16: jump b303
        return block_303(state, tracer, fn_state);
    }
    fn block_303<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_303_0: read-var gs#140823:u8
        let s_303_0: bool = fn_state.gs_140823;
        // N s_303_1: branch s_303_0 b308 b304
        if s_303_0 {
            return block_308(state, tracer, fn_state);
        } else {
            return block_304(state, tracer, fn_state);
        };
    }
    fn block_304<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_304_0: const #21792u : u32
        let s_304_0: u32 = 21792;
        // D s_304_1: read-reg s_304_0:struct
        let s_304_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_304_0 as isize);
            tracer.read_register(s_304_0 as isize, value);
            value
        };
        // D s_304_2: call _get_ERRSELR_EL1_Type_SEL(s_304_1)
        let s_304_2: u16 = u_get_ERRSELR_EL1_Type_SEL(state, tracer, s_304_1);
        // D s_304_3: cast zx s_304_2 -> bv
        let s_304_3: Bits = Bits::new(s_304_2 as u128, 16u16);
        // D s_304_4: cast zx s_304_3 -> i
        let s_304_4: i128 = (s_304_3.value() as i128);
        // D s_304_5: cast reint s_304_4 -> i64
        let s_304_5: i64 = (s_304_4 as i64);
        // D s_304_6: write-var indexshadow#1006 <= s_304_5
        fn_state.indexshadow_1006 = s_304_5;
        // D s_304_7: read-var indexshadow#1006:i64
        let s_304_7: i64 = fn_state.indexshadow_1006;
        // D s_304_8: cast zx s_304_7 -> i
        let s_304_8: i128 = (i128::try_from(s_304_7).unwrap());
        // D s_304_9: call __id(s_304_8)
        let s_304_9: i128 = u__id(state, tracer, s_304_8);
        // D s_304_10: cast reint s_304_9 -> i64
        let s_304_10: i64 = (s_304_9 as i64);
        // C s_304_11: const #0s : i
        let s_304_11: i128 = 0;
        // D s_304_12: cast zx s_304_10 -> i
        let s_304_12: i128 = (i128::try_from(s_304_10).unwrap());
        // D s_304_13: cmp-le s_304_11 s_304_12
        let s_304_13: bool = ((s_304_11) <= (s_304_12));
        // N s_304_14: branch s_304_13 b307 b305
        if s_304_13 {
            return block_307(state, tracer, fn_state);
        } else {
            return block_305(state, tracer, fn_state);
        };
    }
    fn block_305<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_305_0: const #0u : u8
        let s_305_0: bool = false;
        // D s_305_1: write-var gs#140827 <= s_305_0
        fn_state.gs_140827 = s_305_0;
        // N s_305_2: jump b306
        return block_306(state, tracer, fn_state);
    }
    fn block_306<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_306_0: read-var gs#140827:u8
        let s_306_0: bool = fn_state.gs_140827;
        // N s_306_1: assert s_306_0
        let s_306_1: () = assert!(s_306_0);
        // C s_306_2: const #64s : i64
        let s_306_2: i64 = 64;
        // C s_306_3: const #16912u : u32
        let s_306_3: u32 = 16912;
        // D s_306_4: read-reg s_306_3:[struct; 4]
        let s_306_4: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_306_3 as isize);
            tracer.read_register(s_306_3 as isize, value);
            value
        };
        // D s_306_5: read-var indexshadow#1006:i64
        let s_306_5: i64 = fn_state.indexshadow_1006;
        // D s_306_6: cast zx s_306_5 -> i
        let s_306_6: i128 = (i128::try_from(s_306_5).unwrap());
        // D s_306_7: read-element s_306_4[s_306_6]
        let s_306_7: ProductType5c790c8ef59cc8b2 = s_306_4[(s_306_6) as usize];
        // D s_306_8: write-var ga#247229 <= s_306_7
        fn_state.ga_247229 = s_306_7;
        // D s_306_9: read-var ga#247229.0:struct
        let s_306_9: u64 = fn_state.ga_247229._0;
        // D s_306_10: cast zx s_306_9 -> bv
        let s_306_10: Bits = Bits::new(s_306_9 as u128, 64u16);
        // D s_306_11: read-var t:i
        let s_306_11: i128 = fn_state.t;
        // D s_306_12: call X_set(s_306_11, s_306_2, s_306_10)
        let s_306_12: () = X_set(state, tracer, s_306_11, s_306_2, s_306_10);
        // N s_306_13: jump b175
        return block_175(state, tracer, fn_state);
    }
    fn block_307<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_307_0: read-var indexshadow#1006:i64
        let s_307_0: i64 = fn_state.indexshadow_1006;
        // D s_307_1: cast zx s_307_0 -> i
        let s_307_1: i128 = (i128::try_from(s_307_0).unwrap());
        // D s_307_2: call __id(s_307_1)
        let s_307_2: i128 = u__id(state, tracer, s_307_1);
        // D s_307_3: cast reint s_307_2 -> i64
        let s_307_3: i64 = (s_307_2 as i64);
        // C s_307_4: const #4s : i
        let s_307_4: i128 = 4;
        // D s_307_5: cast zx s_307_3 -> i
        let s_307_5: i128 = (i128::try_from(s_307_3).unwrap());
        // D s_307_6: cmp-lt s_307_5 s_307_4
        let s_307_6: bool = ((s_307_5) < (s_307_4));
        // D s_307_7: write-var gs#140827 <= s_307_6
        fn_state.gs_140827 = s_307_6;
        // N s_307_8: jump b306
        return block_306(state, tracer, fn_state);
    }
    fn block_308<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_308_0: const #64s : i64
        let s_308_0: i64 = 64;
        // C s_308_1: const #64s : i
        let s_308_1: i128 = 64;
        // S s_308_2: call Zeros(s_308_1)
        let s_308_2: Bits = Zeros(state, tracer, s_308_1);
        // S s_308_3: cast reint s_308_2 -> u64
        let s_308_3: u64 = (s_308_2.value() as u64);
        // S s_308_4: cast zx s_308_3 -> bv
        let s_308_4: Bits = Bits::new(s_308_3 as u128, 64u16);
        // D s_308_5: read-var t:i
        let s_308_5: i128 = fn_state.t;
        // D s_308_6: call X_set(s_308_5, s_308_0, s_308_4)
        let s_308_6: () = X_set(state, tracer, s_308_5, s_308_0, s_308_4);
        // N s_308_7: jump b175
        return block_175(state, tracer, fn_state);
    }
    fn block_309<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_309_0: const #1u : u8
        let s_309_0: bool = true;
        // D s_309_1: write-var gs#140823 <= s_309_0
        fn_state.gs_140823 = s_309_0;
        // N s_309_2: jump b303
        return block_303(state, tracer, fn_state);
    }
    fn block_310<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_310_0: const #4s : i
        let s_310_0: i128 = 4;
        // D s_310_1: read-var crm:i
        let s_310_1: i128 = fn_state.crm;
        // D s_310_2: cmp-eq s_310_1 s_310_0
        let s_310_2: bool = ((s_310_1) == (s_310_0));
        // D s_310_3: write-var gs#140671 <= s_310_2
        fn_state.gs_140671 = s_310_2;
        // N s_310_4: jump b173
        return block_173(state, tracer, fn_state);
    }
    fn block_311<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_311_0: const #0s : i
        let s_311_0: i128 = 0;
        // D s_311_1: read-var op2:i
        let s_311_1: i128 = fn_state.op2;
        // D s_311_2: cmp-eq s_311_1 s_311_0
        let s_311_2: bool = ((s_311_1) == (s_311_0));
        // D s_311_3: write-var gs#140669 <= s_311_2
        fn_state.gs_140669 = s_311_2;
        // N s_311_4: jump b171
        return block_171(state, tracer, fn_state);
    }
    fn block_312<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_312_0: const #0s : i
        let s_312_0: i128 = 0;
        // D s_312_1: read-var op1:i
        let s_312_1: i128 = fn_state.op1;
        // D s_312_2: cmp-eq s_312_1 s_312_0
        let s_312_2: bool = ((s_312_1) == (s_312_0));
        // D s_312_3: write-var gs#140667 <= s_312_2
        fn_state.gs_140667 = s_312_2;
        // N s_312_4: jump b169
        return block_169(state, tracer, fn_state);
    }
    fn block_313<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_313_0: const #5s : i
        let s_313_0: i128 = 5;
        // D s_313_1: read-var crn:i
        let s_313_1: i128 = fn_state.crn;
        // D s_313_2: cmp-eq s_313_1 s_313_0
        let s_313_2: bool = ((s_313_1) == (s_313_0));
        // D s_313_3: write-var gs#140665 <= s_313_2
        fn_state.gs_140665 = s_313_2;
        // N s_313_4: jump b167
        return block_167(state, tracer, fn_state);
    }
    fn block_314<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_314_0: const #16536u : u32
        let s_314_0: u32 = 16536;
        // D s_314_1: read-reg s_314_0:struct
        let s_314_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_314_0 as isize);
            tracer.read_register(s_314_0 as isize, value);
            value
        };
        // D s_314_2: call _get_BRBFCR_EL1_Type_BANK(s_314_1)
        let s_314_2: u8 = u_get_BRBFCR_EL1_Type_BANK(state, tracer, s_314_1);
        // C s_314_3: const #2s : i
        let s_314_3: i128 = 2;
        // D s_314_4: read-var op2:i
        let s_314_4: i128 = fn_state.op2;
        // D s_314_5: call integer_access(s_314_4, s_314_3)
        let s_314_5: bool = integer_access(state, tracer, s_314_4, s_314_3);
        // C s_314_6: const #0s : i
        let s_314_6: i128 = 0;
        // C s_314_7: const #0u : u64
        let s_314_7: u64 = 0;
        // D s_314_8: cast zx s_314_5 -> u64
        let s_314_8: u64 = (s_314_5 as u64);
        // C s_314_9: const #1u : u64
        let s_314_9: u64 = 1;
        // D s_314_10: and s_314_8 s_314_9
        let s_314_10: u64 = ((s_314_8) & (s_314_9));
        // D s_314_11: cmp-eq s_314_10 s_314_9
        let s_314_11: bool = ((s_314_10) == (s_314_9));
        // D s_314_12: lsl s_314_8 s_314_6
        let s_314_12: u64 = s_314_8 << s_314_6;
        // D s_314_13: or s_314_7 s_314_12
        let s_314_13: u64 = ((s_314_7) | (s_314_12));
        // D s_314_14: cmpl s_314_12
        let s_314_14: u64 = !s_314_12;
        // D s_314_15: and s_314_7 s_314_14
        let s_314_15: u64 = ((s_314_7) & (s_314_14));
        // D s_314_16: select s_314_11 s_314_13 s_314_15
        let s_314_16: u64 = if s_314_11 { s_314_13 } else { s_314_15 };
        // D s_314_17: cast trunc s_314_16 -> u8
        let s_314_17: bool = ((s_314_16) != 0);
        // D s_314_18: cast zx s_314_2 -> bv
        let s_314_18: Bits = Bits::new(s_314_2 as u128, 2u16);
        // D s_314_19: cast zx s_314_17 -> bv
        let s_314_19: Bits = Bits::new(s_314_17 as u128, 1u16);
        // D s_314_20: cast reint s_314_18 -> u128
        let s_314_20: u128 = (s_314_18.value() as u128);
        // D s_314_21: size-of s_314_18
        let s_314_21: u16 = s_314_18.length();
        // D s_314_22: cast reint s_314_19 -> u128
        let s_314_22: u128 = (s_314_19.value() as u128);
        // D s_314_23: size-of s_314_19
        let s_314_23: u16 = s_314_19.length();
        // D s_314_24: lsl s_314_20 s_314_23
        let s_314_24: u128 = s_314_20 << s_314_23;
        // D s_314_25: or s_314_24 s_314_22
        let s_314_25: u128 = ((s_314_24) | (s_314_22));
        // D s_314_26: add s_314_21 s_314_23
        let s_314_26: u16 = (s_314_21 + s_314_23);
        // D s_314_27: create-bits s_314_25 s_314_26
        let s_314_27: Bits = Bits::new(s_314_25, s_314_26);
        // D s_314_28: cast reint s_314_27 -> u8
        let s_314_28: u8 = (s_314_27.value() as u8);
        // C s_314_29: const #3s : i
        let s_314_29: i128 = 3;
        // C s_314_30: const #0s : i
        let s_314_30: i128 = 0;
        // D s_314_31: read-var crm:i
        let s_314_31: i128 = fn_state.crm;
        // D s_314_32: call integer_subrange(s_314_31, s_314_29, s_314_30)
        let s_314_32: Bits = integer_subrange(
            state,
            tracer,
            s_314_31,
            s_314_29,
            s_314_30,
        );
        // D s_314_33: cast reint s_314_32 -> u8
        let s_314_33: u8 = (s_314_32.value() as u8);
        // D s_314_34: cast zx s_314_28 -> bv
        let s_314_34: Bits = Bits::new(s_314_28 as u128, 3u16);
        // D s_314_35: cast zx s_314_33 -> bv
        let s_314_35: Bits = Bits::new(s_314_33 as u128, 4u16);
        // D s_314_36: cast reint s_314_34 -> u128
        let s_314_36: u128 = (s_314_34.value() as u128);
        // D s_314_37: size-of s_314_34
        let s_314_37: u16 = s_314_34.length();
        // D s_314_38: cast reint s_314_35 -> u128
        let s_314_38: u128 = (s_314_35.value() as u128);
        // D s_314_39: size-of s_314_35
        let s_314_39: u16 = s_314_35.length();
        // D s_314_40: lsl s_314_36 s_314_39
        let s_314_40: u128 = s_314_36 << s_314_39;
        // D s_314_41: or s_314_40 s_314_38
        let s_314_41: u128 = ((s_314_40) | (s_314_38));
        // D s_314_42: add s_314_37 s_314_39
        let s_314_42: u16 = (s_314_37 + s_314_39);
        // D s_314_43: create-bits s_314_41 s_314_42
        let s_314_43: Bits = Bits::new(s_314_41, s_314_42);
        // D s_314_44: cast reint s_314_43 -> u8
        let s_314_44: u8 = (s_314_43.value() as u8);
        // D s_314_45: cast zx s_314_44 -> bv
        let s_314_45: Bits = Bits::new(s_314_44 as u128, 7u16);
        // D s_314_46: cast zx s_314_45 -> i
        let s_314_46: i128 = (s_314_45.value() as i128);
        // D s_314_47: cast reint s_314_46 -> i64
        let s_314_47: i64 = (s_314_46 as i64);
        // D s_314_48: write-var recordIdx <= s_314_47
        fn_state.recordIdx = s_314_47;
        // C s_314_49: const #() : ()
        let s_314_49: () = ();
        // S s_314_50: call GetBRBENumRecords(s_314_49)
        let s_314_50: i128 = GetBRBENumRecords(state, tracer, s_314_49);
        // D s_314_51: read-var recordIdx:i64
        let s_314_51: i64 = fn_state.recordIdx;
        // D s_314_52: cast zx s_314_51 -> i
        let s_314_52: i128 = (i128::try_from(s_314_51).unwrap());
        // D s_314_53: cmp-lt s_314_52 s_314_50
        let s_314_53: bool = ((s_314_52) < (s_314_50));
        // N s_314_54: branch s_314_53 b317 b315
        if s_314_53 {
            return block_317(state, tracer, fn_state);
        } else {
            return block_315(state, tracer, fn_state);
        };
    }
    fn block_315<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_315_0: const #64s : i64
        let s_315_0: i64 = 64;
        // C s_315_1: const #64s : i
        let s_315_1: i128 = 64;
        // S s_315_2: call Zeros(s_315_1)
        let s_315_2: Bits = Zeros(state, tracer, s_315_1);
        // S s_315_3: cast reint s_315_2 -> u64
        let s_315_3: u64 = (s_315_2.value() as u64);
        // S s_315_4: cast zx s_315_3 -> bv
        let s_315_4: Bits = Bits::new(s_315_3 as u128, 64u16);
        // D s_315_5: read-var t:i
        let s_315_5: i128 = fn_state.t;
        // D s_315_6: call X_set(s_315_5, s_315_0, s_315_4)
        let s_315_6: () = X_set(state, tracer, s_315_5, s_315_0, s_315_4);
        // N s_315_7: jump b316
        return block_316(state, tracer, fn_state);
    }
    fn block_316<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_316_0: jump b165
        return block_165(state, tracer, fn_state);
    }
    fn block_317<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_317_0: const #0s : i
        let s_317_0: i128 = 0;
        // D s_317_1: read-var op2:i
        let s_317_1: i128 = fn_state.op2;
        // D s_317_2: cmp-eq s_317_1 s_317_0
        let s_317_2: bool = ((s_317_1) == (s_317_0));
        // N s_317_3: branch s_317_2 b344 b318
        if s_317_2 {
            return block_344(state, tracer, fn_state);
        } else {
            return block_318(state, tracer, fn_state);
        };
    }
    fn block_318<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_318_0: const #4s : i
        let s_318_0: i128 = 4;
        // D s_318_1: read-var op2:i
        let s_318_1: i128 = fn_state.op2;
        // D s_318_2: cmp-eq s_318_1 s_318_0
        let s_318_2: bool = ((s_318_1) == (s_318_0));
        // D s_318_3: write-var gs#140844 <= s_318_2
        fn_state.gs_140844 = s_318_2;
        // N s_318_4: jump b319
        return block_319(state, tracer, fn_state);
    }
    fn block_319<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_319_0: read-var gs#140844:u8
        let s_319_0: bool = fn_state.gs_140844;
        // N s_319_1: branch s_319_0 b340 b320
        if s_319_0 {
            return block_340(state, tracer, fn_state);
        } else {
            return block_320(state, tracer, fn_state);
        };
    }
    fn block_320<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_320_0: jump b321
        return block_321(state, tracer, fn_state);
    }
    fn block_321<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_321_0: const #1s : i
        let s_321_0: i128 = 1;
        // D s_321_1: read-var op2:i
        let s_321_1: i128 = fn_state.op2;
        // D s_321_2: cmp-eq s_321_1 s_321_0
        let s_321_2: bool = ((s_321_1) == (s_321_0));
        // N s_321_3: branch s_321_2 b339 b322
        if s_321_2 {
            return block_339(state, tracer, fn_state);
        } else {
            return block_322(state, tracer, fn_state);
        };
    }
    fn block_322<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_322_0: const #5s : i
        let s_322_0: i128 = 5;
        // D s_322_1: read-var op2:i
        let s_322_1: i128 = fn_state.op2;
        // D s_322_2: cmp-eq s_322_1 s_322_0
        let s_322_2: bool = ((s_322_1) == (s_322_0));
        // D s_322_3: write-var gs#140847 <= s_322_2
        fn_state.gs_140847 = s_322_2;
        // N s_322_4: jump b323
        return block_323(state, tracer, fn_state);
    }
    fn block_323<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_323_0: read-var gs#140847:u8
        let s_323_0: bool = fn_state.gs_140847;
        // N s_323_1: branch s_323_0 b335 b324
        if s_323_0 {
            return block_335(state, tracer, fn_state);
        } else {
            return block_324(state, tracer, fn_state);
        };
    }
    fn block_324<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_324_0: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_325<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_325_0: const #2s : i
        let s_325_0: i128 = 2;
        // D s_325_1: read-var op2:i
        let s_325_1: i128 = fn_state.op2;
        // D s_325_2: cmp-eq s_325_1 s_325_0
        let s_325_2: bool = ((s_325_1) == (s_325_0));
        // N s_325_3: branch s_325_2 b334 b326
        if s_325_2 {
            return block_334(state, tracer, fn_state);
        } else {
            return block_326(state, tracer, fn_state);
        };
    }
    fn block_326<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_326_0: const #6s : i
        let s_326_0: i128 = 6;
        // D s_326_1: read-var op2:i
        let s_326_1: i128 = fn_state.op2;
        // D s_326_2: cmp-eq s_326_1 s_326_0
        let s_326_2: bool = ((s_326_1) == (s_326_0));
        // D s_326_3: write-var gs#140850 <= s_326_2
        fn_state.gs_140850 = s_326_2;
        // N s_326_4: jump b327
        return block_327(state, tracer, fn_state);
    }
    fn block_327<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_327_0: read-var gs#140850:u8
        let s_327_0: bool = fn_state.gs_140850;
        // N s_327_1: branch s_327_0 b330 b328
        if s_327_0 {
            return block_330(state, tracer, fn_state);
        } else {
            return block_328(state, tracer, fn_state);
        };
    }
    fn block_328<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_328_0: jump b329
        return block_329(state, tracer, fn_state);
    }
    fn block_329<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_329_0: jump b316
        return block_316(state, tracer, fn_state);
    }
    fn block_330<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_330_0: read-var recordIdx:i64
        let s_330_0: i64 = fn_state.recordIdx;
        // D s_330_1: cast zx s_330_0 -> i
        let s_330_1: i128 = (i128::try_from(s_330_0).unwrap());
        // D s_330_2: call __id(s_330_1)
        let s_330_2: i128 = u__id(state, tracer, s_330_1);
        // D s_330_3: cast reint s_330_2 -> i64
        let s_330_3: i64 = (s_330_2 as i64);
        // C s_330_4: const #0s : i
        let s_330_4: i128 = 0;
        // D s_330_5: cast zx s_330_3 -> i
        let s_330_5: i128 = (i128::try_from(s_330_3).unwrap());
        // D s_330_6: cmp-le s_330_4 s_330_5
        let s_330_6: bool = ((s_330_4) <= (s_330_5));
        // N s_330_7: branch s_330_6 b333 b331
        if s_330_6 {
            return block_333(state, tracer, fn_state);
        } else {
            return block_331(state, tracer, fn_state);
        };
    }
    fn block_331<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_331_0: const #0u : u8
        let s_331_0: bool = false;
        // D s_331_1: write-var gs#140854 <= s_331_0
        fn_state.gs_140854 = s_331_0;
        // N s_331_2: jump b332
        return block_332(state, tracer, fn_state);
    }
    fn block_332<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_332_0: read-var gs#140854:u8
        let s_332_0: bool = fn_state.gs_140854;
        // N s_332_1: assert s_332_0
        let s_332_1: () = assert!(s_332_0);
        // C s_332_2: const #64s : i64
        let s_332_2: i64 = 64;
        // C s_332_3: const #101248u : u32
        let s_332_3: u32 = 101248;
        // D s_332_4: read-reg s_332_3:[struct; 64]
        let s_332_4: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_332_3 as isize);
            tracer.read_register(s_332_3 as isize, value);
            value
        };
        // D s_332_5: read-var recordIdx:i64
        let s_332_5: i64 = fn_state.recordIdx;
        // D s_332_6: cast zx s_332_5 -> i
        let s_332_6: i128 = (i128::try_from(s_332_5).unwrap());
        // D s_332_7: read-element s_332_4[s_332_6]
        let s_332_7: ProductType5c790c8ef59cc8b2 = s_332_4[(s_332_6) as usize];
        // D s_332_8: write-var ga#247202 <= s_332_7
        fn_state.ga_247202 = s_332_7;
        // D s_332_9: read-var ga#247202.0:struct
        let s_332_9: u64 = fn_state.ga_247202._0;
        // D s_332_10: cast zx s_332_9 -> bv
        let s_332_10: Bits = Bits::new(s_332_9 as u128, 64u16);
        // D s_332_11: read-var t:i
        let s_332_11: i128 = fn_state.t;
        // D s_332_12: call X_set(s_332_11, s_332_2, s_332_10)
        let s_332_12: () = X_set(state, tracer, s_332_11, s_332_2, s_332_10);
        // N s_332_13: jump b329
        return block_329(state, tracer, fn_state);
    }
    fn block_333<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_333_0: read-var recordIdx:i64
        let s_333_0: i64 = fn_state.recordIdx;
        // D s_333_1: cast zx s_333_0 -> i
        let s_333_1: i128 = (i128::try_from(s_333_0).unwrap());
        // D s_333_2: call __id(s_333_1)
        let s_333_2: i128 = u__id(state, tracer, s_333_1);
        // D s_333_3: cast reint s_333_2 -> i64
        let s_333_3: i64 = (s_333_2 as i64);
        // C s_333_4: const #64s : i
        let s_333_4: i128 = 64;
        // D s_333_5: cast zx s_333_3 -> i
        let s_333_5: i128 = (i128::try_from(s_333_3).unwrap());
        // D s_333_6: cmp-lt s_333_5 s_333_4
        let s_333_6: bool = ((s_333_5) < (s_333_4));
        // D s_333_7: write-var gs#140854 <= s_333_6
        fn_state.gs_140854 = s_333_6;
        // N s_333_8: jump b332
        return block_332(state, tracer, fn_state);
    }
    fn block_334<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_334_0: const #1u : u8
        let s_334_0: bool = true;
        // D s_334_1: write-var gs#140850 <= s_334_0
        fn_state.gs_140850 = s_334_0;
        // N s_334_2: jump b327
        return block_327(state, tracer, fn_state);
    }
    fn block_335<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_335_0: read-var recordIdx:i64
        let s_335_0: i64 = fn_state.recordIdx;
        // D s_335_1: cast zx s_335_0 -> i
        let s_335_1: i128 = (i128::try_from(s_335_0).unwrap());
        // D s_335_2: call __id(s_335_1)
        let s_335_2: i128 = u__id(state, tracer, s_335_1);
        // D s_335_3: cast reint s_335_2 -> i64
        let s_335_3: i64 = (s_335_2 as i64);
        // C s_335_4: const #0s : i
        let s_335_4: i128 = 0;
        // D s_335_5: cast zx s_335_3 -> i
        let s_335_5: i128 = (i128::try_from(s_335_3).unwrap());
        // D s_335_6: cmp-le s_335_4 s_335_5
        let s_335_6: bool = ((s_335_4) <= (s_335_5));
        // N s_335_7: branch s_335_6 b338 b336
        if s_335_6 {
            return block_338(state, tracer, fn_state);
        } else {
            return block_336(state, tracer, fn_state);
        };
    }
    fn block_336<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_336_0: const #0u : u8
        let s_336_0: bool = false;
        // D s_336_1: write-var gs#140860 <= s_336_0
        fn_state.gs_140860 = s_336_0;
        // N s_336_2: jump b337
        return block_337(state, tracer, fn_state);
    }
    fn block_337<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_337_0: read-var gs#140860:u8
        let s_337_0: bool = fn_state.gs_140860;
        // N s_337_1: assert s_337_0
        let s_337_1: () = assert!(s_337_0);
        // C s_337_2: const #64s : i64
        let s_337_2: i64 = 64;
        // C s_337_3: const #21240u : u32
        let s_337_3: u32 = 21240;
        // D s_337_4: read-reg s_337_3:[struct; 64]
        let s_337_4: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_337_3 as isize);
            tracer.read_register(s_337_3 as isize, value);
            value
        };
        // D s_337_5: read-var recordIdx:i64
        let s_337_5: i64 = fn_state.recordIdx;
        // D s_337_6: cast zx s_337_5 -> i
        let s_337_6: i128 = (i128::try_from(s_337_5).unwrap());
        // D s_337_7: read-element s_337_4[s_337_6]
        let s_337_7: ProductType5c790c8ef59cc8b2 = s_337_4[(s_337_6) as usize];
        // D s_337_8: write-var ga#247193 <= s_337_7
        fn_state.ga_247193 = s_337_7;
        // D s_337_9: read-var ga#247193.0:struct
        let s_337_9: u64 = fn_state.ga_247193._0;
        // D s_337_10: cast zx s_337_9 -> bv
        let s_337_10: Bits = Bits::new(s_337_9 as u128, 64u16);
        // D s_337_11: read-var t:i
        let s_337_11: i128 = fn_state.t;
        // D s_337_12: call X_set(s_337_11, s_337_2, s_337_10)
        let s_337_12: () = X_set(state, tracer, s_337_11, s_337_2, s_337_10);
        // N s_337_13: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_338<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_338_0: read-var recordIdx:i64
        let s_338_0: i64 = fn_state.recordIdx;
        // D s_338_1: cast zx s_338_0 -> i
        let s_338_1: i128 = (i128::try_from(s_338_0).unwrap());
        // D s_338_2: call __id(s_338_1)
        let s_338_2: i128 = u__id(state, tracer, s_338_1);
        // D s_338_3: cast reint s_338_2 -> i64
        let s_338_3: i64 = (s_338_2 as i64);
        // C s_338_4: const #64s : i
        let s_338_4: i128 = 64;
        // D s_338_5: cast zx s_338_3 -> i
        let s_338_5: i128 = (i128::try_from(s_338_3).unwrap());
        // D s_338_6: cmp-lt s_338_5 s_338_4
        let s_338_6: bool = ((s_338_5) < (s_338_4));
        // D s_338_7: write-var gs#140860 <= s_338_6
        fn_state.gs_140860 = s_338_6;
        // N s_338_8: jump b337
        return block_337(state, tracer, fn_state);
    }
    fn block_339<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_339_0: const #1u : u8
        let s_339_0: bool = true;
        // D s_339_1: write-var gs#140847 <= s_339_0
        fn_state.gs_140847 = s_339_0;
        // N s_339_2: jump b323
        return block_323(state, tracer, fn_state);
    }
    fn block_340<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_340_0: read-var recordIdx:i64
        let s_340_0: i64 = fn_state.recordIdx;
        // D s_340_1: cast zx s_340_0 -> i
        let s_340_1: i128 = (i128::try_from(s_340_0).unwrap());
        // D s_340_2: call __id(s_340_1)
        let s_340_2: i128 = u__id(state, tracer, s_340_1);
        // D s_340_3: cast reint s_340_2 -> i64
        let s_340_3: i64 = (s_340_2 as i64);
        // C s_340_4: const #0s : i
        let s_340_4: i128 = 0;
        // D s_340_5: cast zx s_340_3 -> i
        let s_340_5: i128 = (i128::try_from(s_340_3).unwrap());
        // D s_340_6: cmp-le s_340_4 s_340_5
        let s_340_6: bool = ((s_340_4) <= (s_340_5));
        // N s_340_7: branch s_340_6 b343 b341
        if s_340_6 {
            return block_343(state, tracer, fn_state);
        } else {
            return block_341(state, tracer, fn_state);
        };
    }
    fn block_341<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_341_0: const #0u : u8
        let s_341_0: bool = false;
        // D s_341_1: write-var gs#140866 <= s_341_0
        fn_state.gs_140866 = s_341_0;
        // N s_341_2: jump b342
        return block_342(state, tracer, fn_state);
    }
    fn block_342<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_342_0: read-var gs#140866:u8
        let s_342_0: bool = fn_state.gs_140866;
        // N s_342_1: assert s_342_0
        let s_342_1: () = assert!(s_342_0);
        // C s_342_2: const #64s : i64
        let s_342_2: i64 = 64;
        // C s_342_3: const #100288u : u32
        let s_342_3: u32 = 100288;
        // D s_342_4: read-reg s_342_3:[struct; 64]
        let s_342_4: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_342_3 as isize);
            tracer.read_register(s_342_3 as isize, value);
            value
        };
        // D s_342_5: read-var recordIdx:i64
        let s_342_5: i64 = fn_state.recordIdx;
        // D s_342_6: cast zx s_342_5 -> i
        let s_342_6: i128 = (i128::try_from(s_342_5).unwrap());
        // D s_342_7: read-element s_342_4[s_342_6]
        let s_342_7: ProductType5c790c8ef59cc8b2 = s_342_4[(s_342_6) as usize];
        // D s_342_8: write-var ga#247184 <= s_342_7
        fn_state.ga_247184 = s_342_7;
        // D s_342_9: read-var ga#247184.0:struct
        let s_342_9: u64 = fn_state.ga_247184._0;
        // D s_342_10: cast zx s_342_9 -> bv
        let s_342_10: Bits = Bits::new(s_342_9 as u128, 64u16);
        // D s_342_11: read-var t:i
        let s_342_11: i128 = fn_state.t;
        // D s_342_12: call X_set(s_342_11, s_342_2, s_342_10)
        let s_342_12: () = X_set(state, tracer, s_342_11, s_342_2, s_342_10);
        // N s_342_13: jump b321
        return block_321(state, tracer, fn_state);
    }
    fn block_343<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_343_0: read-var recordIdx:i64
        let s_343_0: i64 = fn_state.recordIdx;
        // D s_343_1: cast zx s_343_0 -> i
        let s_343_1: i128 = (i128::try_from(s_343_0).unwrap());
        // D s_343_2: call __id(s_343_1)
        let s_343_2: i128 = u__id(state, tracer, s_343_1);
        // D s_343_3: cast reint s_343_2 -> i64
        let s_343_3: i64 = (s_343_2 as i64);
        // C s_343_4: const #64s : i
        let s_343_4: i128 = 64;
        // D s_343_5: cast zx s_343_3 -> i
        let s_343_5: i128 = (i128::try_from(s_343_3).unwrap());
        // D s_343_6: cmp-lt s_343_5 s_343_4
        let s_343_6: bool = ((s_343_5) < (s_343_4));
        // D s_343_7: write-var gs#140866 <= s_343_6
        fn_state.gs_140866 = s_343_6;
        // N s_343_8: jump b342
        return block_342(state, tracer, fn_state);
    }
    fn block_344<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_344_0: const #1u : u8
        let s_344_0: bool = true;
        // D s_344_1: write-var gs#140844 <= s_344_0
        fn_state.gs_140844 = s_344_0;
        // N s_344_2: jump b319
        return block_319(state, tracer, fn_state);
    }
    fn block_345<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_345_0: const #0s : i
        let s_345_0: i128 = 0;
        // D s_345_1: read-var op2:i
        let s_345_1: i128 = fn_state.op2;
        // D s_345_2: cmp-eq s_345_1 s_345_0
        let s_345_2: bool = ((s_345_1) == (s_345_0));
        // N s_345_3: branch s_345_2 b360 b346
        if s_345_2 {
            return block_360(state, tracer, fn_state);
        } else {
            return block_346(state, tracer, fn_state);
        };
    }
    fn block_346<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_346_0: const #1s : i
        let s_346_0: i128 = 1;
        // D s_346_1: read-var op2:i
        let s_346_1: i128 = fn_state.op2;
        // D s_346_2: cmp-eq s_346_1 s_346_0
        let s_346_2: bool = ((s_346_1) == (s_346_0));
        // N s_346_3: branch s_346_2 b359 b347
        if s_346_2 {
            return block_359(state, tracer, fn_state);
        } else {
            return block_347(state, tracer, fn_state);
        };
    }
    fn block_347<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_347_0: const #2s : i
        let s_347_0: i128 = 2;
        // D s_347_1: read-var op2:i
        let s_347_1: i128 = fn_state.op2;
        // D s_347_2: cmp-eq s_347_1 s_347_0
        let s_347_2: bool = ((s_347_1) == (s_347_0));
        // N s_347_3: branch s_347_2 b358 b348
        if s_347_2 {
            return block_358(state, tracer, fn_state);
        } else {
            return block_348(state, tracer, fn_state);
        };
    }
    fn block_348<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_348_0: const #4s : i
        let s_348_0: i128 = 4;
        // D s_348_1: read-var op2:i
        let s_348_1: i128 = fn_state.op2;
        // D s_348_2: cmp-eq s_348_1 s_348_0
        let s_348_2: bool = ((s_348_1) == (s_348_0));
        // N s_348_3: branch s_348_2 b357 b349
        if s_348_2 {
            return block_357(state, tracer, fn_state);
        } else {
            return block_349(state, tracer, fn_state);
        };
    }
    fn block_349<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_349_0: const #5s : i
        let s_349_0: i128 = 5;
        // D s_349_1: read-var op2:i
        let s_349_1: i128 = fn_state.op2;
        // D s_349_2: cmp-eq s_349_1 s_349_0
        let s_349_2: bool = ((s_349_1) == (s_349_0));
        // N s_349_3: branch s_349_2 b356 b350
        if s_349_2 {
            return block_356(state, tracer, fn_state);
        } else {
            return block_350(state, tracer, fn_state);
        };
    }
    fn block_350<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_350_0: const #6s : i
        let s_350_0: i128 = 6;
        // D s_350_1: read-var op2:i
        let s_350_1: i128 = fn_state.op2;
        // D s_350_2: cmp-eq s_350_1 s_350_0
        let s_350_2: bool = ((s_350_1) == (s_350_0));
        // D s_350_3: write-var gs#140657 <= s_350_2
        fn_state.gs_140657 = s_350_2;
        // N s_350_4: jump b351
        return block_351(state, tracer, fn_state);
    }
    fn block_351<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_351_0: read-var gs#140657:u8
        let s_351_0: bool = fn_state.gs_140657;
        // D s_351_1: write-var gs#140658 <= s_351_0
        fn_state.gs_140658 = s_351_0;
        // N s_351_2: jump b352
        return block_352(state, tracer, fn_state);
    }
    fn block_352<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_352_0: read-var gs#140658:u8
        let s_352_0: bool = fn_state.gs_140658;
        // D s_352_1: write-var gs#140659 <= s_352_0
        fn_state.gs_140659 = s_352_0;
        // N s_352_2: jump b353
        return block_353(state, tracer, fn_state);
    }
    fn block_353<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_353_0: read-var gs#140659:u8
        let s_353_0: bool = fn_state.gs_140659;
        // D s_353_1: write-var gs#140660 <= s_353_0
        fn_state.gs_140660 = s_353_0;
        // N s_353_2: jump b354
        return block_354(state, tracer, fn_state);
    }
    fn block_354<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_354_0: read-var gs#140660:u8
        let s_354_0: bool = fn_state.gs_140660;
        // D s_354_1: write-var gs#140661 <= s_354_0
        fn_state.gs_140661 = s_354_0;
        // N s_354_2: jump b355
        return block_355(state, tracer, fn_state);
    }
    fn block_355<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_355_0: read-var gs#140661:u8
        let s_355_0: bool = fn_state.gs_140661;
        // D s_355_1: write-var gs#140662 <= s_355_0
        fn_state.gs_140662 = s_355_0;
        // N s_355_2: jump b163
        return block_163(state, tracer, fn_state);
    }
    fn block_356<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_356_0: const #1u : u8
        let s_356_0: bool = true;
        // D s_356_1: write-var gs#140657 <= s_356_0
        fn_state.gs_140657 = s_356_0;
        // N s_356_2: jump b351
        return block_351(state, tracer, fn_state);
    }
    fn block_357<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_357_0: const #1u : u8
        let s_357_0: bool = true;
        // D s_357_1: write-var gs#140658 <= s_357_0
        fn_state.gs_140658 = s_357_0;
        // N s_357_2: jump b352
        return block_352(state, tracer, fn_state);
    }
    fn block_358<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_358_0: const #1u : u8
        let s_358_0: bool = true;
        // D s_358_1: write-var gs#140659 <= s_358_0
        fn_state.gs_140659 = s_358_0;
        // N s_358_2: jump b353
        return block_353(state, tracer, fn_state);
    }
    fn block_359<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_359_0: const #1u : u8
        let s_359_0: bool = true;
        // D s_359_1: write-var gs#140660 <= s_359_0
        fn_state.gs_140660 = s_359_0;
        // N s_359_2: jump b354
        return block_354(state, tracer, fn_state);
    }
    fn block_360<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_360_0: const #1u : u8
        let s_360_0: bool = true;
        // D s_360_1: write-var gs#140661 <= s_360_0
        fn_state.gs_140661 = s_360_0;
        // N s_360_2: jump b355
        return block_355(state, tracer, fn_state);
    }
    fn block_361<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_361_0: const #8s : i
        let s_361_0: i128 = 8;
        // D s_361_1: read-var crn:i
        let s_361_1: i128 = fn_state.crn;
        // D s_361_2: cmp-eq s_361_1 s_361_0
        let s_361_2: bool = ((s_361_1) == (s_361_0));
        // D s_361_3: write-var gs#140650 <= s_361_2
        fn_state.gs_140650 = s_361_2;
        // N s_361_4: jump b161
        return block_161(state, tracer, fn_state);
    }
    fn block_362<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_362_0: const #1s : i
        let s_362_0: i128 = 1;
        // D s_362_1: read-var op1:i
        let s_362_1: i128 = fn_state.op1;
        // D s_362_2: cmp-eq s_362_1 s_362_0
        let s_362_2: bool = ((s_362_1) == (s_362_0));
        // D s_362_3: write-var gs#140648 <= s_362_2
        fn_state.gs_140648 = s_362_2;
        // N s_362_4: jump b159
        return block_159(state, tracer, fn_state);
    }
    fn block_363<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_363_0: const #0s : i
        let s_363_0: i128 = 0;
        // D s_363_1: read-var op2:i
        let s_363_1: i128 = fn_state.op2;
        // D s_363_2: cmp-eq s_363_1 s_363_0
        let s_363_2: bool = ((s_363_1) == (s_363_0));
        // N s_363_3: branch s_363_2 b366 b364
        if s_363_2 {
            return block_366(state, tracer, fn_state);
        } else {
            return block_364(state, tracer, fn_state);
        };
    }
    fn block_364<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_364_0: const #1u : u8
        let s_364_0: bool = true;
        // S s_364_1: call genRandomNum(s_364_0)
        let s_364_1: u64 = genRandomNum(state, tracer, s_364_0);
        // D s_364_2: write-var ga#247159 <= s_364_1
        fn_state.ga_247159 = s_364_1;
        // N s_364_3: jump b365
        return block_365(state, tracer, fn_state);
    }
    fn block_365<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_365_0: read-var ga#247159:u64
        let s_365_0: u64 = fn_state.ga_247159;
        // D s_365_1: cast zx s_365_0 -> bv
        let s_365_1: Bits = Bits::new(s_365_0 as u128, 64u16);
        // D s_365_2: read-var t:i
        let s_365_2: i128 = fn_state.t;
        // C s_365_3: const #64s : i64
        let s_365_3: i64 = 64;
        // D s_365_4: call X_set(s_365_2, s_365_3, s_365_1)
        let s_365_4: () = X_set(state, tracer, s_365_2, s_365_3, s_365_1);
        // N s_365_5: jump b157
        return block_157(state, tracer, fn_state);
    }
    fn block_366<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_366_0: const #0u : u8
        let s_366_0: bool = false;
        // S s_366_1: call genRandomNum(s_366_0)
        let s_366_1: u64 = genRandomNum(state, tracer, s_366_0);
        // D s_366_2: write-var ga#247159 <= s_366_1
        fn_state.ga_247159 = s_366_1;
        // N s_366_3: jump b365
        return block_365(state, tracer, fn_state);
    }
    fn block_367<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_367_0: const #0s : i
        let s_367_0: i128 = 0;
        // D s_367_1: read-var op2:i
        let s_367_1: i128 = fn_state.op2;
        // D s_367_2: cmp-eq s_367_1 s_367_0
        let s_367_2: bool = ((s_367_1) == (s_367_0));
        // N s_367_3: branch s_367_2 b370 b368
        if s_367_2 {
            return block_370(state, tracer, fn_state);
        } else {
            return block_368(state, tracer, fn_state);
        };
    }
    fn block_368<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_368_0: const #1s : i
        let s_368_0: i128 = 1;
        // D s_368_1: read-var op2:i
        let s_368_1: i128 = fn_state.op2;
        // D s_368_2: cmp-eq s_368_1 s_368_0
        let s_368_2: bool = ((s_368_1) == (s_368_0));
        // D s_368_3: write-var gs#140644 <= s_368_2
        fn_state.gs_140644 = s_368_2;
        // N s_368_4: jump b369
        return block_369(state, tracer, fn_state);
    }
    fn block_369<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_369_0: read-var gs#140644:u8
        let s_369_0: bool = fn_state.gs_140644;
        // D s_369_1: write-var gs#140645 <= s_369_0
        fn_state.gs_140645 = s_369_0;
        // N s_369_2: jump b155
        return block_155(state, tracer, fn_state);
    }
    fn block_370<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_370_0: const #1u : u8
        let s_370_0: bool = true;
        // D s_370_1: write-var gs#140644 <= s_370_0
        fn_state.gs_140644 = s_370_0;
        // N s_370_2: jump b369
        return block_369(state, tracer, fn_state);
    }
    fn block_371<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_371_0: const #3s : i
        let s_371_0: i128 = 3;
        // D s_371_1: read-var op1:i
        let s_371_1: i128 = fn_state.op1;
        // D s_371_2: cmp-eq s_371_1 s_371_0
        let s_371_2: bool = ((s_371_1) == (s_371_0));
        // D s_371_3: write-var gs#140641 <= s_371_2
        fn_state.gs_140641 = s_371_2;
        // N s_371_4: jump b153
        return block_153(state, tracer, fn_state);
    }
    fn block_372<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_372_0: const #4s : i
        let s_372_0: i128 = 4;
        // D s_372_1: read-var crm:i
        let s_372_1: i128 = fn_state.crm;
        // D s_372_2: cmp-eq s_372_1 s_372_0
        let s_372_2: bool = ((s_372_1) == (s_372_0));
        // D s_372_3: write-var gs#140639 <= s_372_2
        fn_state.gs_140639 = s_372_2;
        // N s_372_4: jump b151
        return block_151(state, tracer, fn_state);
    }
    fn block_373<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_373_0: const #2s : i
        let s_373_0: i128 = 2;
        // D s_373_1: read-var crn:i
        let s_373_1: i128 = fn_state.crn;
        // D s_373_2: cmp-eq s_373_1 s_373_0
        let s_373_2: bool = ((s_373_1) == (s_373_0));
        // D s_373_3: write-var gs#140637 <= s_373_2
        fn_state.gs_140637 = s_373_2;
        // N s_373_4: jump b149
        return block_149(state, tracer, fn_state);
    }
    fn block_374<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_374_0: const #64s : i64
        let s_374_0: i64 = 64;
        // C s_374_1: const #64s : i64
        let s_374_1: i64 = 64;
        // D s_374_2: read-var t:i
        let s_374_2: i128 = fn_state.t;
        // D s_374_3: call X_read(s_374_2, s_374_1)
        let s_374_3: Bits = X_read(state, tracer, s_374_2, s_374_1);
        // D s_374_4: cast reint s_374_3 -> u64
        let s_374_4: u64 = (s_374_3.value() as u64);
        // C s_374_5: const #2s : i
        let s_374_5: i128 = 2;
        // D s_374_6: cast zx s_374_4 -> bv
        let s_374_6: Bits = Bits::new(s_374_4 as u128, 64u16);
        // C s_374_7: const #2u : u8
        let s_374_7: u8 = 2;
        // C s_374_8: cast zx s_374_7 -> bv
        let s_374_8: Bits = Bits::new(s_374_7 as u128, 2u16);
        // C s_374_9: const #1s : i
        let s_374_9: i128 = 1;
        // C s_374_10: const #1u : u64
        let s_374_10: u64 = 1;
        // C s_374_11: cast zx s_374_10 -> bv
        let s_374_11: Bits = Bits::new(s_374_10 as u128, 64u16);
        // C s_374_12: lsl s_374_11 s_374_9
        let s_374_12: Bits = s_374_11 << s_374_9;
        // C s_374_13: sub s_374_12 s_374_11
        let s_374_13: Bits = ((s_374_12) - (s_374_11));
        // C s_374_14: and s_374_8 s_374_13
        let s_374_14: Bits = ((s_374_8) & (s_374_13));
        // C s_374_15: lsl s_374_14 s_374_5
        let s_374_15: Bits = s_374_14 << s_374_5;
        // C s_374_16: lsl s_374_13 s_374_5
        let s_374_16: Bits = s_374_13 << s_374_5;
        // C s_374_17: cmpl s_374_16
        let s_374_17: Bits = !s_374_16;
        // D s_374_18: and s_374_6 s_374_17
        let s_374_18: Bits = ((s_374_6) & (s_374_17));
        // D s_374_19: or s_374_18 s_374_15
        let s_374_19: Bits = ((s_374_18) | (s_374_15));
        // D s_374_20: cast reint s_374_19 -> u64
        let s_374_20: u64 = (s_374_19.value() as u64);
        // D s_374_21: cast zx s_374_20 -> bv
        let s_374_21: Bits = Bits::new(s_374_20 as u128, 64u16);
        // D s_374_22: read-var t:i
        let s_374_22: i128 = fn_state.t;
        // D s_374_23: call X_set(s_374_22, s_374_0, s_374_21)
        let s_374_23: () = X_set(state, tracer, s_374_22, s_374_0, s_374_21);
        // N s_374_24: jump b147
        return block_147(state, tracer, fn_state);
    }
    fn block_375<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_375_0: const #16975u : u32
        let s_375_0: u32 = 16975;
        // D s_375_1: read-reg s_375_0:u8
        let s_375_1: u8 = {
            let value = state.read_register::<u8>(s_375_0 as isize);
            tracer.read_register(s_375_0 as isize, value);
            value
        };
        // D s_375_2: cast zx s_375_1 -> bv
        let s_375_2: Bits = Bits::new(s_375_1 as u128, 2u16);
        // C s_375_3: const #448u : u32
        let s_375_3: u32 = 448;
        // D s_375_4: read-reg s_375_3:u8
        let s_375_4: u8 = {
            let value = state.read_register::<u8>(s_375_3 as isize);
            tracer.read_register(s_375_3 as isize, value);
            value
        };
        // D s_375_5: cast zx s_375_4 -> bv
        let s_375_5: Bits = Bits::new(s_375_4 as u128, 2u16);
        // D s_375_6: cmp-eq s_375_2 s_375_5
        let s_375_6: bool = ((s_375_2) == (s_375_5));
        // N s_375_7: branch s_375_6 b385 b376
        if s_375_6 {
            return block_385(state, tracer, fn_state);
        } else {
            return block_376(state, tracer, fn_state);
        };
    }
    fn block_376<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_376_0: const #16975u : u32
        let s_376_0: u32 = 16975;
        // D s_376_1: read-reg s_376_0:u8
        let s_376_1: u8 = {
            let value = state.read_register::<u8>(s_376_0 as isize);
            tracer.read_register(s_376_0 as isize, value);
            value
        };
        // D s_376_2: cast zx s_376_1 -> bv
        let s_376_2: Bits = Bits::new(s_376_1 as u128, 2u16);
        // C s_376_3: const #440u : u32
        let s_376_3: u32 = 440;
        // D s_376_4: read-reg s_376_3:u8
        let s_376_4: u8 = {
            let value = state.read_register::<u8>(s_376_3 as isize);
            tracer.read_register(s_376_3 as isize, value);
            value
        };
        // D s_376_5: cast zx s_376_4 -> bv
        let s_376_5: Bits = Bits::new(s_376_4 as u128, 2u16);
        // D s_376_6: cmp-eq s_376_2 s_376_5
        let s_376_6: bool = ((s_376_2) == (s_376_5));
        // N s_376_7: branch s_376_6 b378 b377
        if s_376_6 {
            return block_378(state, tracer, fn_state);
        } else {
            return block_377(state, tracer, fn_state);
        };
    }
    fn block_377<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_377_0: return
        return;
    }
    fn block_378<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_378_0: const #64s : i64
        let s_378_0: i64 = 64;
        // D s_378_1: read-var t:i
        let s_378_1: i128 = fn_state.t;
        // D s_378_2: call X_read(s_378_1, s_378_0)
        let s_378_2: Bits = X_read(state, tracer, s_378_1, s_378_0);
        // D s_378_3: cast reint s_378_2 -> u64
        let s_378_3: u64 = (s_378_2.value() as u64);
        // D s_378_4: write-var ga#247142 <= s_378_3
        fn_state.ga_247142 = s_378_3;
        // C s_378_5: const #() : ()
        let s_378_5: () = ();
        // S s_378_6: call EL2Enabled(s_378_5)
        let s_378_6: bool = EL2Enabled(state, tracer, s_378_5);
        // N s_378_7: branch s_378_6 b384 b379
        if s_378_6 {
            return block_384(state, tracer, fn_state);
        } else {
            return block_379(state, tracer, fn_state);
        };
    }
    fn block_379<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_379_0: const #0u : u8
        let s_379_0: bool = false;
        // D s_379_1: write-var gs#140881 <= s_379_0
        fn_state.gs_140881 = s_379_0;
        // N s_379_2: jump b380
        return block_380(state, tracer, fn_state);
    }
    fn block_380<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_380_0: read-var gs#140881:u8
        let s_380_0: bool = fn_state.gs_140881;
        // N s_380_1: branch s_380_0 b383 b381
        if s_380_0 {
            return block_383(state, tracer, fn_state);
        } else {
            return block_381(state, tracer, fn_state);
        };
    }
    fn block_381<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_381_0: const #0u : u8
        let s_381_0: bool = false;
        // D s_381_1: write-var ga#247143 <= s_381_0
        fn_state.ga_247143 = s_381_0;
        // N s_381_2: jump b382
        return block_382(state, tracer, fn_state);
    }
    fn block_382<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_382_0: const #4s : i
        let s_382_0: i128 = 4;
        // D s_382_1: read-var ga#247142:u64
        let s_382_1: u64 = fn_state.ga_247142;
        // D s_382_2: cast zx s_382_1 -> bv
        let s_382_2: Bits = Bits::new(s_382_1 as u128, 64u16);
        // D s_382_3: read-var ga#247143:u8
        let s_382_3: bool = fn_state.ga_247143;
        // D s_382_4: cast zx s_382_3 -> bv
        let s_382_4: Bits = Bits::new(s_382_3 as u128, 1u16);
        // C s_382_5: const #0s : i
        let s_382_5: i128 = 0;
        // C s_382_6: const #1u : u64
        let s_382_6: u64 = 1;
        // C s_382_7: cast zx s_382_6 -> bv
        let s_382_7: Bits = Bits::new(s_382_6 as u128, 64u16);
        // C s_382_8: lsl s_382_7 s_382_5
        let s_382_8: Bits = s_382_7 << s_382_5;
        // C s_382_9: sub s_382_8 s_382_7
        let s_382_9: Bits = ((s_382_8) - (s_382_7));
        // D s_382_10: and s_382_4 s_382_9
        let s_382_10: Bits = ((s_382_4) & (s_382_9));
        // D s_382_11: lsl s_382_10 s_382_0
        let s_382_11: Bits = s_382_10 << s_382_0;
        // C s_382_12: lsl s_382_9 s_382_0
        let s_382_12: Bits = s_382_9 << s_382_0;
        // C s_382_13: cmpl s_382_12
        let s_382_13: Bits = !s_382_12;
        // D s_382_14: and s_382_2 s_382_13
        let s_382_14: Bits = ((s_382_2) & (s_382_13));
        // D s_382_15: or s_382_14 s_382_11
        let s_382_15: Bits = ((s_382_14) | (s_382_11));
        // D s_382_16: cast reint s_382_15 -> u64
        let s_382_16: u64 = (s_382_15.value() as u64);
        // D s_382_17: cast zx s_382_16 -> bv
        let s_382_17: Bits = Bits::new(s_382_16 as u128, 64u16);
        // D s_382_18: read-var t:i
        let s_382_18: i128 = fn_state.t;
        // C s_382_19: const #64s : i64
        let s_382_19: i64 = 64;
        // D s_382_20: call X_set(s_382_18, s_382_19, s_382_17)
        let s_382_20: () = X_set(state, tracer, s_382_18, s_382_19, s_382_17);
        // N s_382_21: return
        return;
    }
    fn block_383<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_383_0: const #1u : u8
        let s_383_0: bool = true;
        // D s_383_1: write-var ga#247143 <= s_383_0
        fn_state.ga_247143 = s_383_0;
        // N s_383_2: jump b382
        return block_382(state, tracer, fn_state);
    }
    fn block_384<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_384_0: const #102552u : u32
        let s_384_0: u32 = 102552;
        // D s_384_1: read-reg s_384_0:struct
        let s_384_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_384_0 as isize);
            tracer.read_register(s_384_0 as isize, value);
            value
        };
        // D s_384_2: call _get_HCR_EL2_Type_TDZ(s_384_1)
        let s_384_2: bool = u_get_HCR_EL2_Type_TDZ(state, tracer, s_384_1);
        // D s_384_3: cast zx s_384_2 -> bv
        let s_384_3: Bits = Bits::new(s_384_2 as u128, 1u16);
        // C s_384_4: const #1u : u8
        let s_384_4: bool = true;
        // C s_384_5: cast zx s_384_4 -> bv
        let s_384_5: Bits = Bits::new(s_384_4 as u128, 1u16);
        // D s_384_6: cmp-eq s_384_3 s_384_5
        let s_384_6: bool = ((s_384_3) == (s_384_5));
        // D s_384_7: write-var gs#140881 <= s_384_6
        fn_state.gs_140881 = s_384_6;
        // N s_384_8: jump b380
        return block_380(state, tracer, fn_state);
    }
    fn block_385<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_385_0: const #() : ()
        let s_385_0: () = ();
        // S s_385_1: call IsInHost(s_385_0)
        let s_385_1: bool = IsInHost(state, tracer, s_385_0);
        // N s_385_2: branch s_385_1 b396 b386
        if s_385_1 {
            return block_396(state, tracer, fn_state);
        } else {
            return block_386(state, tracer, fn_state);
        };
    }
    fn block_386<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_386_0: const #64s : i64
        let s_386_0: i64 = 64;
        // D s_386_1: read-var t:i
        let s_386_1: i128 = fn_state.t;
        // D s_386_2: call X_read(s_386_1, s_386_0)
        let s_386_2: Bits = X_read(state, tracer, s_386_1, s_386_0);
        // D s_386_3: cast reint s_386_2 -> u64
        let s_386_3: u64 = (s_386_2.value() as u64);
        // D s_386_4: write-var ga#247132 <= s_386_3
        fn_state.ga_247132 = s_386_3;
        // C s_386_5: const #90272u : u32
        let s_386_5: u32 = 90272;
        // D s_386_6: read-reg s_386_5:struct
        let s_386_6: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_386_5 as isize);
            tracer.read_register(s_386_5 as isize, value);
            value
        };
        // D s_386_7: call _get_SCTLR_EL1_Type_DZE(s_386_6)
        let s_386_7: bool = u_get_SCTLR_EL1_Type_DZE(state, tracer, s_386_6);
        // D s_386_8: cast zx s_386_7 -> bv
        let s_386_8: Bits = Bits::new(s_386_7 as u128, 1u16);
        // C s_386_9: const #0u : u8
        let s_386_9: bool = false;
        // C s_386_10: cast zx s_386_9 -> bv
        let s_386_10: Bits = Bits::new(s_386_9 as u128, 1u16);
        // D s_386_11: cmp-eq s_386_8 s_386_10
        let s_386_11: bool = ((s_386_8) == (s_386_10));
        // N s_386_12: branch s_386_11 b395 b387
        if s_386_11 {
            return block_395(state, tracer, fn_state);
        } else {
            return block_387(state, tracer, fn_state);
        };
    }
    fn block_387<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_387_0: const #() : ()
        let s_387_0: () = ();
        // S s_387_1: call EL2Enabled(s_387_0)
        let s_387_1: bool = EL2Enabled(state, tracer, s_387_0);
        // N s_387_2: branch s_387_1 b394 b388
        if s_387_1 {
            return block_394(state, tracer, fn_state);
        } else {
            return block_388(state, tracer, fn_state);
        };
    }
    fn block_388<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_388_0: const #0u : u8
        let s_388_0: bool = false;
        // D s_388_1: write-var gs#140886 <= s_388_0
        fn_state.gs_140886 = s_388_0;
        // N s_388_2: jump b389
        return block_389(state, tracer, fn_state);
    }
    fn block_389<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_389_0: read-var gs#140886:u8
        let s_389_0: bool = fn_state.gs_140886;
        // D s_389_1: write-var gs#140887 <= s_389_0
        fn_state.gs_140887 = s_389_0;
        // N s_389_2: jump b390
        return block_390(state, tracer, fn_state);
    }
    fn block_390<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_390_0: read-var gs#140887:u8
        let s_390_0: bool = fn_state.gs_140887;
        // N s_390_1: branch s_390_0 b393 b391
        if s_390_0 {
            return block_393(state, tracer, fn_state);
        } else {
            return block_391(state, tracer, fn_state);
        };
    }
    fn block_391<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_391_0: const #0u : u8
        let s_391_0: bool = false;
        // D s_391_1: write-var ga#247133 <= s_391_0
        fn_state.ga_247133 = s_391_0;
        // N s_391_2: jump b392
        return block_392(state, tracer, fn_state);
    }
    fn block_392<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_392_0: const #4s : i
        let s_392_0: i128 = 4;
        // D s_392_1: read-var ga#247132:u64
        let s_392_1: u64 = fn_state.ga_247132;
        // D s_392_2: cast zx s_392_1 -> bv
        let s_392_2: Bits = Bits::new(s_392_1 as u128, 64u16);
        // D s_392_3: read-var ga#247133:u8
        let s_392_3: bool = fn_state.ga_247133;
        // D s_392_4: cast zx s_392_3 -> bv
        let s_392_4: Bits = Bits::new(s_392_3 as u128, 1u16);
        // C s_392_5: const #0s : i
        let s_392_5: i128 = 0;
        // C s_392_6: const #1u : u64
        let s_392_6: u64 = 1;
        // C s_392_7: cast zx s_392_6 -> bv
        let s_392_7: Bits = Bits::new(s_392_6 as u128, 64u16);
        // C s_392_8: lsl s_392_7 s_392_5
        let s_392_8: Bits = s_392_7 << s_392_5;
        // C s_392_9: sub s_392_8 s_392_7
        let s_392_9: Bits = ((s_392_8) - (s_392_7));
        // D s_392_10: and s_392_4 s_392_9
        let s_392_10: Bits = ((s_392_4) & (s_392_9));
        // D s_392_11: lsl s_392_10 s_392_0
        let s_392_11: Bits = s_392_10 << s_392_0;
        // C s_392_12: lsl s_392_9 s_392_0
        let s_392_12: Bits = s_392_9 << s_392_0;
        // C s_392_13: cmpl s_392_12
        let s_392_13: Bits = !s_392_12;
        // D s_392_14: and s_392_2 s_392_13
        let s_392_14: Bits = ((s_392_2) & (s_392_13));
        // D s_392_15: or s_392_14 s_392_11
        let s_392_15: Bits = ((s_392_14) | (s_392_11));
        // D s_392_16: cast reint s_392_15 -> u64
        let s_392_16: u64 = (s_392_15.value() as u64);
        // D s_392_17: cast zx s_392_16 -> bv
        let s_392_17: Bits = Bits::new(s_392_16 as u128, 64u16);
        // D s_392_18: read-var t:i
        let s_392_18: i128 = fn_state.t;
        // C s_392_19: const #64s : i64
        let s_392_19: i64 = 64;
        // D s_392_20: call X_set(s_392_18, s_392_19, s_392_17)
        let s_392_20: () = X_set(state, tracer, s_392_18, s_392_19, s_392_17);
        // N s_392_21: return
        return;
    }
    fn block_393<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_393_0: const #1u : u8
        let s_393_0: bool = true;
        // D s_393_1: write-var ga#247133 <= s_393_0
        fn_state.ga_247133 = s_393_0;
        // N s_393_2: jump b392
        return block_392(state, tracer, fn_state);
    }
    fn block_394<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_394_0: const #102552u : u32
        let s_394_0: u32 = 102552;
        // D s_394_1: read-reg s_394_0:struct
        let s_394_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_394_0 as isize);
            tracer.read_register(s_394_0 as isize, value);
            value
        };
        // D s_394_2: call _get_HCR_EL2_Type_TDZ(s_394_1)
        let s_394_2: bool = u_get_HCR_EL2_Type_TDZ(state, tracer, s_394_1);
        // D s_394_3: cast zx s_394_2 -> bv
        let s_394_3: Bits = Bits::new(s_394_2 as u128, 1u16);
        // C s_394_4: const #1u : u8
        let s_394_4: bool = true;
        // C s_394_5: cast zx s_394_4 -> bv
        let s_394_5: Bits = Bits::new(s_394_4 as u128, 1u16);
        // D s_394_6: cmp-eq s_394_3 s_394_5
        let s_394_6: bool = ((s_394_3) == (s_394_5));
        // D s_394_7: write-var gs#140886 <= s_394_6
        fn_state.gs_140886 = s_394_6;
        // N s_394_8: jump b389
        return block_389(state, tracer, fn_state);
    }
    fn block_395<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_395_0: const #1u : u8
        let s_395_0: bool = true;
        // D s_395_1: write-var gs#140887 <= s_395_0
        fn_state.gs_140887 = s_395_0;
        // N s_395_2: jump b390
        return block_390(state, tracer, fn_state);
    }
    fn block_396<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_396_0: const #64s : i64
        let s_396_0: i64 = 64;
        // D s_396_1: read-var t:i
        let s_396_1: i128 = fn_state.t;
        // D s_396_2: call X_read(s_396_1, s_396_0)
        let s_396_2: Bits = X_read(state, tracer, s_396_1, s_396_0);
        // D s_396_3: cast reint s_396_2 -> u64
        let s_396_3: u64 = (s_396_2.value() as u64);
        // D s_396_4: write-var ga#247122 <= s_396_3
        fn_state.ga_247122 = s_396_3;
        // C s_396_5: const #20784u : u32
        let s_396_5: u32 = 20784;
        // D s_396_6: read-reg s_396_5:struct
        let s_396_6: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_396_5 as isize);
            tracer.read_register(s_396_5 as isize, value);
            value
        };
        // D s_396_7: call _get_SCTLR_EL2_Type_DZE(s_396_6)
        let s_396_7: bool = u_get_SCTLR_EL2_Type_DZE(state, tracer, s_396_6);
        // D s_396_8: cast zx s_396_7 -> bv
        let s_396_8: Bits = Bits::new(s_396_7 as u128, 1u16);
        // C s_396_9: const #0u : u8
        let s_396_9: bool = false;
        // C s_396_10: cast zx s_396_9 -> bv
        let s_396_10: Bits = Bits::new(s_396_9 as u128, 1u16);
        // D s_396_11: cmp-eq s_396_8 s_396_10
        let s_396_11: bool = ((s_396_8) == (s_396_10));
        // N s_396_12: branch s_396_11 b399 b397
        if s_396_11 {
            return block_399(state, tracer, fn_state);
        } else {
            return block_397(state, tracer, fn_state);
        };
    }
    fn block_397<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_397_0: const #0u : u8
        let s_397_0: bool = false;
        // D s_397_1: write-var ga#247123 <= s_397_0
        fn_state.ga_247123 = s_397_0;
        // N s_397_2: jump b398
        return block_398(state, tracer, fn_state);
    }
    fn block_398<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_398_0: const #4s : i
        let s_398_0: i128 = 4;
        // D s_398_1: read-var ga#247122:u64
        let s_398_1: u64 = fn_state.ga_247122;
        // D s_398_2: cast zx s_398_1 -> bv
        let s_398_2: Bits = Bits::new(s_398_1 as u128, 64u16);
        // D s_398_3: read-var ga#247123:u8
        let s_398_3: bool = fn_state.ga_247123;
        // D s_398_4: cast zx s_398_3 -> bv
        let s_398_4: Bits = Bits::new(s_398_3 as u128, 1u16);
        // C s_398_5: const #0s : i
        let s_398_5: i128 = 0;
        // C s_398_6: const #1u : u64
        let s_398_6: u64 = 1;
        // C s_398_7: cast zx s_398_6 -> bv
        let s_398_7: Bits = Bits::new(s_398_6 as u128, 64u16);
        // C s_398_8: lsl s_398_7 s_398_5
        let s_398_8: Bits = s_398_7 << s_398_5;
        // C s_398_9: sub s_398_8 s_398_7
        let s_398_9: Bits = ((s_398_8) - (s_398_7));
        // D s_398_10: and s_398_4 s_398_9
        let s_398_10: Bits = ((s_398_4) & (s_398_9));
        // D s_398_11: lsl s_398_10 s_398_0
        let s_398_11: Bits = s_398_10 << s_398_0;
        // C s_398_12: lsl s_398_9 s_398_0
        let s_398_12: Bits = s_398_9 << s_398_0;
        // C s_398_13: cmpl s_398_12
        let s_398_13: Bits = !s_398_12;
        // D s_398_14: and s_398_2 s_398_13
        let s_398_14: Bits = ((s_398_2) & (s_398_13));
        // D s_398_15: or s_398_14 s_398_11
        let s_398_15: Bits = ((s_398_14) | (s_398_11));
        // D s_398_16: cast reint s_398_15 -> u64
        let s_398_16: u64 = (s_398_15.value() as u64);
        // D s_398_17: cast zx s_398_16 -> bv
        let s_398_17: Bits = Bits::new(s_398_16 as u128, 64u16);
        // D s_398_18: read-var t:i
        let s_398_18: i128 = fn_state.t;
        // C s_398_19: const #64s : i64
        let s_398_19: i64 = 64;
        // D s_398_20: call X_set(s_398_18, s_398_19, s_398_17)
        let s_398_20: () = X_set(state, tracer, s_398_18, s_398_19, s_398_17);
        // N s_398_21: return
        return;
    }
    fn block_399<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_399_0: const #1u : u8
        let s_399_0: bool = true;
        // D s_399_1: write-var ga#247123 <= s_399_0
        fn_state.ga_247123 = s_399_0;
        // N s_399_2: jump b398
        return block_398(state, tracer, fn_state);
    }
    fn block_400<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_400_0: const #0s : i
        let s_400_0: i128 = 0;
        // D s_400_1: read-var crm:i
        let s_400_1: i128 = fn_state.crm;
        // D s_400_2: cmp-eq s_400_1 s_400_0
        let s_400_2: bool = ((s_400_1) == (s_400_0));
        // D s_400_3: write-var gs#140634 <= s_400_2
        fn_state.gs_140634 = s_400_2;
        // N s_400_4: jump b144
        return block_144(state, tracer, fn_state);
    }
    fn block_401<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_401_0: const #7s : i
        let s_401_0: i128 = 7;
        // D s_401_1: read-var op2:i
        let s_401_1: i128 = fn_state.op2;
        // D s_401_2: cmp-eq s_401_1 s_401_0
        let s_401_2: bool = ((s_401_1) == (s_401_0));
        // D s_401_3: write-var gs#140632 <= s_401_2
        fn_state.gs_140632 = s_401_2;
        // N s_401_4: jump b142
        return block_142(state, tracer, fn_state);
    }
    fn block_402<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_402_0: const #3s : i
        let s_402_0: i128 = 3;
        // D s_402_1: read-var op1:i
        let s_402_1: i128 = fn_state.op1;
        // D s_402_2: cmp-eq s_402_1 s_402_0
        let s_402_2: bool = ((s_402_1) == (s_402_0));
        // D s_402_3: write-var gs#140630 <= s_402_2
        fn_state.gs_140630 = s_402_2;
        // N s_402_4: jump b140
        return block_140(state, tracer, fn_state);
    }
    fn block_403<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_403_0: const #0s : i
        let s_403_0: i128 = 0;
        // D s_403_1: read-var crn:i
        let s_403_1: i128 = fn_state.crn;
        // D s_403_2: cmp-eq s_403_1 s_403_0
        let s_403_2: bool = ((s_403_1) == (s_403_0));
        // D s_403_3: write-var gs#140628 <= s_403_2
        fn_state.gs_140628 = s_403_2;
        // N s_403_4: jump b138
        return block_138(state, tracer, fn_state);
    }
    fn block_404<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_404_0: const #0s : i
        let s_404_0: i128 = 0;
        // D s_404_1: read-var op2:i
        let s_404_1: i128 = fn_state.op2;
        // D s_404_2: cmp-eq s_404_1 s_404_0
        let s_404_2: bool = ((s_404_1) == (s_404_0));
        // N s_404_3: branch s_404_2 b406 b405
        if s_404_2 {
            return block_406(state, tracer, fn_state);
        } else {
            return block_405(state, tracer, fn_state);
        };
    }
    fn block_405<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_405_0: const #64s : i64
        let s_405_0: i64 = 64;
        // C s_405_1: const #22896u : u32
        let s_405_1: u32 = 22896;
        // D s_405_2: read-reg s_405_1:u64
        let s_405_2: u64 = {
            let value = state.read_register::<u64>(s_405_1 as isize);
            tracer.read_register(s_405_1 as isize, value);
            value
        };
        // C s_405_3: const #0s : i
        let s_405_3: i128 = 0;
        // D s_405_4: cast zx s_405_2 -> bv
        let s_405_4: Bits = Bits::new(s_405_2 as u128, 64u16);
        // C s_405_5: const #1s : i64
        let s_405_5: i64 = 1;
        // C s_405_6: cast zx s_405_5 -> i
        let s_405_6: i128 = (i128::try_from(s_405_5).unwrap());
        // C s_405_7: const #3s : i
        let s_405_7: i128 = 3;
        // C s_405_8: add s_405_7 s_405_6
        let s_405_8: i128 = (s_405_7 + s_405_6);
        // D s_405_9: bit-extract s_405_4 s_405_3 s_405_8
        let s_405_9: Bits = (Bits::new(
            ((s_405_4) >> (s_405_3)).value(),
            u16::try_from(s_405_8).unwrap(),
        ));
        // D s_405_10: cast reint s_405_9 -> u8
        let s_405_10: u8 = (s_405_9.value() as u8);
        // D s_405_11: call CacheConfigRead(s_405_10)
        let s_405_11: u64 = CacheConfigRead(state, tracer, s_405_10);
        // C s_405_12: const #32s : i
        let s_405_12: i128 = 32;
        // D s_405_13: cast zx s_405_11 -> bv
        let s_405_13: Bits = Bits::new(s_405_11 as u128, 64u16);
        // C s_405_14: const #1s : i64
        let s_405_14: i64 = 1;
        // C s_405_15: cast zx s_405_14 -> i
        let s_405_15: i128 = (i128::try_from(s_405_14).unwrap());
        // C s_405_16: const #31s : i
        let s_405_16: i128 = 31;
        // C s_405_17: add s_405_16 s_405_15
        let s_405_17: i128 = (s_405_16 + s_405_15);
        // D s_405_18: bit-extract s_405_13 s_405_12 s_405_17
        let s_405_18: Bits = (Bits::new(
            ((s_405_13) >> (s_405_12)).value(),
            u16::try_from(s_405_17).unwrap(),
        ));
        // D s_405_19: cast reint s_405_18 -> u32
        let s_405_19: u32 = (s_405_18.value() as u32);
        // C s_405_20: const #64s : i
        let s_405_20: i128 = 64;
        // D s_405_21: cast zx s_405_19 -> bv
        let s_405_21: Bits = Bits::new(s_405_19 as u128, 32u16);
        // D s_405_22: bits-cast zx s_405_21 -> bv length s_405_20
        let s_405_22: Bits = s_405_21.zero_extend(s_405_20);
        // D s_405_23: cast reint s_405_22 -> u64
        let s_405_23: u64 = (s_405_22.value() as u64);
        // D s_405_24: cast zx s_405_23 -> bv
        let s_405_24: Bits = Bits::new(s_405_23 as u128, 64u16);
        // D s_405_25: read-var t:i
        let s_405_25: i128 = fn_state.t;
        // D s_405_26: call X_set(s_405_25, s_405_0, s_405_24)
        let s_405_26: () = X_set(state, tracer, s_405_25, s_405_0, s_405_24);
        // N s_405_27: return
        return;
    }
    fn block_406<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_406_0: const #64s : i64
        let s_406_0: i64 = 64;
        // C s_406_1: const #22896u : u32
        let s_406_1: u32 = 22896;
        // D s_406_2: read-reg s_406_1:u64
        let s_406_2: u64 = {
            let value = state.read_register::<u64>(s_406_1 as isize);
            tracer.read_register(s_406_1 as isize, value);
            value
        };
        // C s_406_3: const #0s : i
        let s_406_3: i128 = 0;
        // D s_406_4: cast zx s_406_2 -> bv
        let s_406_4: Bits = Bits::new(s_406_2 as u128, 64u16);
        // C s_406_5: const #1s : i64
        let s_406_5: i64 = 1;
        // C s_406_6: cast zx s_406_5 -> i
        let s_406_6: i128 = (i128::try_from(s_406_5).unwrap());
        // C s_406_7: const #3s : i
        let s_406_7: i128 = 3;
        // C s_406_8: add s_406_7 s_406_6
        let s_406_8: i128 = (s_406_7 + s_406_6);
        // D s_406_9: bit-extract s_406_4 s_406_3 s_406_8
        let s_406_9: Bits = (Bits::new(
            ((s_406_4) >> (s_406_3)).value(),
            u16::try_from(s_406_8).unwrap(),
        ));
        // D s_406_10: cast reint s_406_9 -> u8
        let s_406_10: u8 = (s_406_9.value() as u8);
        // D s_406_11: call CacheConfigRead(s_406_10)
        let s_406_11: u64 = CacheConfigRead(state, tracer, s_406_10);
        // D s_406_12: cast zx s_406_11 -> bv
        let s_406_12: Bits = Bits::new(s_406_11 as u128, 64u16);
        // D s_406_13: read-var t:i
        let s_406_13: i128 = fn_state.t;
        // D s_406_14: call X_set(s_406_13, s_406_0, s_406_12)
        let s_406_14: () = X_set(state, tracer, s_406_13, s_406_0, s_406_12);
        // N s_406_15: return
        return;
    }
    fn block_407<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_407_0: const #0s : i
        let s_407_0: i128 = 0;
        // D s_407_1: read-var crm:i
        let s_407_1: i128 = fn_state.crm;
        // D s_407_2: cmp-eq s_407_1 s_407_0
        let s_407_2: bool = ((s_407_1) == (s_407_0));
        // D s_407_3: write-var gs#140625 <= s_407_2
        fn_state.gs_140625 = s_407_2;
        // N s_407_4: jump b135
        return block_135(state, tracer, fn_state);
    }
    fn block_408<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_408_0: const #0s : i
        let s_408_0: i128 = 0;
        // D s_408_1: read-var op2:i
        let s_408_1: i128 = fn_state.op2;
        // D s_408_2: cmp-eq s_408_1 s_408_0
        let s_408_2: bool = ((s_408_1) == (s_408_0));
        // N s_408_3: branch s_408_2 b411 b409
        if s_408_2 {
            return block_411(state, tracer, fn_state);
        } else {
            return block_409(state, tracer, fn_state);
        };
    }
    fn block_409<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_409_0: const #2s : i
        let s_409_0: i128 = 2;
        // D s_409_1: read-var op2:i
        let s_409_1: i128 = fn_state.op2;
        // D s_409_2: cmp-eq s_409_1 s_409_0
        let s_409_2: bool = ((s_409_1) == (s_409_0));
        // D s_409_3: write-var gs#140622 <= s_409_2
        fn_state.gs_140622 = s_409_2;
        // N s_409_4: jump b410
        return block_410(state, tracer, fn_state);
    }
    fn block_410<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_410_0: read-var gs#140622:u8
        let s_410_0: bool = fn_state.gs_140622;
        // D s_410_1: write-var gs#140623 <= s_410_0
        fn_state.gs_140623 = s_410_0;
        // N s_410_2: jump b133
        return block_133(state, tracer, fn_state);
    }
    fn block_411<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_411_0: const #1u : u8
        let s_411_0: bool = true;
        // D s_411_1: write-var gs#140622 <= s_411_0
        fn_state.gs_140622 = s_411_0;
        // N s_411_2: jump b410
        return block_410(state, tracer, fn_state);
    }
    fn block_412<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_412_0: const #1s : i
        let s_412_0: i128 = 1;
        // D s_412_1: read-var op1:i
        let s_412_1: i128 = fn_state.op1;
        // D s_412_2: cmp-eq s_412_1 s_412_0
        let s_412_2: bool = ((s_412_1) == (s_412_0));
        // D s_412_3: write-var gs#140619 <= s_412_2
        fn_state.gs_140619 = s_412_2;
        // N s_412_4: jump b131
        return block_131(state, tracer, fn_state);
    }
    fn block_413<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_413_0: const #0s : i
        let s_413_0: i128 = 0;
        // D s_413_1: read-var crn:i
        let s_413_1: i128 = fn_state.crn;
        // D s_413_2: cmp-eq s_413_1 s_413_0
        let s_413_2: bool = ((s_413_1) == (s_413_0));
        // D s_413_3: write-var gs#140617 <= s_413_2
        fn_state.gs_140617 = s_413_2;
        // N s_413_4: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_414<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_414_0: const #64s : i64
        let s_414_0: i64 = 64;
        // C s_414_1: const #() : ()
        let s_414_1: () = ();
        // S s_414_2: call getISR(s_414_1)
        let s_414_2: u32 = getISR(state, tracer, s_414_1);
        // C s_414_3: const #64s : i
        let s_414_3: i128 = 64;
        // S s_414_4: cast zx s_414_2 -> bv
        let s_414_4: Bits = Bits::new(s_414_2 as u128, 32u16);
        // D s_414_5: bits-cast zx s_414_4 -> bv length s_414_3
        let s_414_5: Bits = s_414_4.zero_extend(s_414_3);
        // D s_414_6: cast reint s_414_5 -> u64
        let s_414_6: u64 = (s_414_5.value() as u64);
        // D s_414_7: cast zx s_414_6 -> bv
        let s_414_7: Bits = Bits::new(s_414_6 as u128, 64u16);
        // D s_414_8: read-var t:i
        let s_414_8: i128 = fn_state.t;
        // D s_414_9: call X_set(s_414_8, s_414_0, s_414_7)
        let s_414_9: () = X_set(state, tracer, s_414_8, s_414_0, s_414_7);
        // N s_414_10: return
        return;
    }
    fn block_415<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_415_0: const #1s : i
        let s_415_0: i128 = 1;
        // D s_415_1: read-var crm:i
        let s_415_1: i128 = fn_state.crm;
        // D s_415_2: cmp-eq s_415_1 s_415_0
        let s_415_2: bool = ((s_415_1) == (s_415_0));
        // D s_415_3: write-var gs#140614 <= s_415_2
        fn_state.gs_140614 = s_415_2;
        // N s_415_4: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_416<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_416_0: const #0s : i
        let s_416_0: i128 = 0;
        // D s_416_1: read-var op2:i
        let s_416_1: i128 = fn_state.op2;
        // D s_416_2: cmp-eq s_416_1 s_416_0
        let s_416_2: bool = ((s_416_1) == (s_416_0));
        // D s_416_3: write-var gs#140612 <= s_416_2
        fn_state.gs_140612 = s_416_2;
        // N s_416_4: jump b124
        return block_124(state, tracer, fn_state);
    }
    fn block_417<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_417_0: const #0s : i
        let s_417_0: i128 = 0;
        // D s_417_1: read-var op1:i
        let s_417_1: i128 = fn_state.op1;
        // D s_417_2: cmp-eq s_417_1 s_417_0
        let s_417_2: bool = ((s_417_1) == (s_417_0));
        // D s_417_3: write-var gs#140610 <= s_417_2
        fn_state.gs_140610 = s_417_2;
        // N s_417_4: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_418<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_418_0: const #12u : u8
        let s_418_0: u8 = 12;
        // C s_418_1: cast zx s_418_0 -> bv
        let s_418_1: Bits = Bits::new(s_418_0 as u128, 4u16);
        // C s_418_2: cast zx s_418_1 -> i
        let s_418_2: i128 = (s_418_1.value() as i128);
        // C s_418_3: cast reint s_418_2 -> i64
        let s_418_3: i64 = (s_418_2 as i64);
        // C s_418_4: cast zx s_418_3 -> i
        let s_418_4: i128 = (i128::try_from(s_418_3).unwrap());
        // D s_418_5: read-var crn:i
        let s_418_5: i128 = fn_state.crn;
        // D s_418_6: cmp-eq s_418_5 s_418_4
        let s_418_6: bool = ((s_418_5) == (s_418_4));
        // D s_418_7: write-var gs#140608 <= s_418_6
        fn_state.gs_140608 = s_418_6;
        // N s_418_8: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_419<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_419_0: const #() : ()
        let s_419_0: () = ();
        // S s_419_1: call CurrentSecurityState(s_419_0)
        let s_419_1: u32 = CurrentSecurityState(state, tracer, s_419_0);
        // C s_419_2: const #3u : u32
        let s_419_2: u32 = 3;
        // S s_419_3: cmp-eq s_419_1 s_419_2
        let s_419_3: bool = ((s_419_1) == (s_419_2));
        // N s_419_4: branch s_419_3 b421 b420
        if s_419_3 {
            return block_421(state, tracer, fn_state);
        } else {
            return block_420(state, tracer, fn_state);
        };
    }
    fn block_420<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_420_0: return
        return;
    }
    fn block_421<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_421_0: const #64s : i64
        let s_421_0: i64 = 64;
        // C s_421_1: const #64s : i64
        let s_421_1: i64 = 64;
        // D s_421_2: read-var t:i
        let s_421_2: i128 = fn_state.t;
        // D s_421_3: call X_read(s_421_2, s_421_1)
        let s_421_3: Bits = X_read(state, tracer, s_421_2, s_421_1);
        // D s_421_4: cast reint s_421_3 -> u64
        let s_421_4: u64 = (s_421_3.value() as u64);
        // C s_421_5: const #() : ()
        let s_421_5: () = ();
        // S s_421_6: call MPAM3_EL3_read(s_421_5)
        let s_421_6: ProductType5c790c8ef59cc8b2 = MPAM3_EL3_read(
            state,
            tracer,
            s_421_5,
        );
        // S s_421_7: call _get_MPAM3_EL3_Type_FORCE_NS(s_421_6)
        let s_421_7: bool = u_get_MPAM3_EL3_Type_FORCE_NS(state, tracer, s_421_6);
        // C s_421_8: const #60s : i
        let s_421_8: i128 = 60;
        // D s_421_9: cast zx s_421_4 -> bv
        let s_421_9: Bits = Bits::new(s_421_4 as u128, 64u16);
        // S s_421_10: cast zx s_421_7 -> bv
        let s_421_10: Bits = Bits::new(s_421_7 as u128, 1u16);
        // C s_421_11: const #0s : i
        let s_421_11: i128 = 0;
        // C s_421_12: const #1u : u64
        let s_421_12: u64 = 1;
        // C s_421_13: cast zx s_421_12 -> bv
        let s_421_13: Bits = Bits::new(s_421_12 as u128, 64u16);
        // C s_421_14: lsl s_421_13 s_421_11
        let s_421_14: Bits = s_421_13 << s_421_11;
        // C s_421_15: sub s_421_14 s_421_13
        let s_421_15: Bits = ((s_421_14) - (s_421_13));
        // S s_421_16: and s_421_10 s_421_15
        let s_421_16: Bits = ((s_421_10) & (s_421_15));
        // S s_421_17: lsl s_421_16 s_421_8
        let s_421_17: Bits = s_421_16 << s_421_8;
        // C s_421_18: lsl s_421_15 s_421_8
        let s_421_18: Bits = s_421_15 << s_421_8;
        // C s_421_19: cmpl s_421_18
        let s_421_19: Bits = !s_421_18;
        // D s_421_20: and s_421_9 s_421_19
        let s_421_20: Bits = ((s_421_9) & (s_421_19));
        // D s_421_21: or s_421_20 s_421_17
        let s_421_21: Bits = ((s_421_20) | (s_421_17));
        // D s_421_22: cast reint s_421_21 -> u64
        let s_421_22: u64 = (s_421_21.value() as u64);
        // D s_421_23: cast zx s_421_22 -> bv
        let s_421_23: Bits = Bits::new(s_421_22 as u128, 64u16);
        // D s_421_24: read-var t:i
        let s_421_24: i128 = fn_state.t;
        // D s_421_25: call X_set(s_421_24, s_421_0, s_421_23)
        let s_421_25: () = X_set(state, tracer, s_421_24, s_421_0, s_421_23);
        // N s_421_26: return
        return;
    }
    fn block_422<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_422_0: const #5s : i
        let s_422_0: i128 = 5;
        // D s_422_1: read-var crm:i
        let s_422_1: i128 = fn_state.crm;
        // D s_422_2: cmp-eq s_422_1 s_422_0
        let s_422_2: bool = ((s_422_1) == (s_422_0));
        // D s_422_3: write-var gs#140606 <= s_422_2
        fn_state.gs_140606 = s_422_2;
        // N s_422_4: jump b117
        return block_117(state, tracer, fn_state);
    }
    fn block_423<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_423_0: const #0s : i
        let s_423_0: i128 = 0;
        // D s_423_1: read-var op2:i
        let s_423_1: i128 = fn_state.op2;
        // D s_423_2: cmp-eq s_423_1 s_423_0
        let s_423_2: bool = ((s_423_1) == (s_423_0));
        // D s_423_3: write-var gs#140604 <= s_423_2
        fn_state.gs_140604 = s_423_2;
        // N s_423_4: jump b115
        return block_115(state, tracer, fn_state);
    }
    fn block_424<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_424_0: const #10u : u8
        let s_424_0: u8 = 10;
        // C s_424_1: cast zx s_424_0 -> bv
        let s_424_1: Bits = Bits::new(s_424_0 as u128, 4u16);
        // C s_424_2: cast zx s_424_1 -> i
        let s_424_2: i128 = (s_424_1.value() as i128);
        // C s_424_3: cast reint s_424_2 -> i64
        let s_424_3: i64 = (s_424_2 as i64);
        // C s_424_4: cast zx s_424_3 -> i
        let s_424_4: i128 = (i128::try_from(s_424_3).unwrap());
        // D s_424_5: read-var crn:i
        let s_424_5: i128 = fn_state.crn;
        // D s_424_6: cmp-eq s_424_5 s_424_4
        let s_424_6: bool = ((s_424_5) == (s_424_4));
        // D s_424_7: write-var gs#140602 <= s_424_6
        fn_state.gs_140602 = s_424_6;
        // N s_424_8: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_425<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_425_0: const #0s : i
        let s_425_0: i128 = 0;
        // D s_425_1: read-var op1:i
        let s_425_1: i128 = fn_state.op1;
        // D s_425_2: cmp-eq s_425_1 s_425_0
        let s_425_2: bool = ((s_425_1) == (s_425_0));
        // D s_425_3: write-var gs#140601 <= s_425_2
        fn_state.gs_140601 = s_425_2;
        // N s_425_4: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_426<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_426_0: const #8s : i
        let s_426_0: i128 = 8;
        // D s_426_1: read-var crm:i
        let s_426_1: i128 = fn_state.crm;
        // D s_426_2: cmp-eq s_426_1 s_426_0
        let s_426_2: bool = ((s_426_1) == (s_426_0));
        // N s_426_3: branch s_426_2 b430 b427
        if s_426_2 {
            return block_430(state, tracer, fn_state);
        } else {
            return block_427(state, tracer, fn_state);
        };
    }
    fn block_427<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_427_0: const #9s : i
        let s_427_0: i128 = 9;
        // D s_427_1: read-var crm:i
        let s_427_1: i128 = fn_state.crm;
        // D s_427_2: cmp-eq s_427_1 s_427_0
        let s_427_2: bool = ((s_427_1) == (s_427_0));
        // N s_427_3: branch s_427_2 b429 b428
        if s_427_2 {
            return block_429(state, tracer, fn_state);
        } else {
            return block_428(state, tracer, fn_state);
        };
    }
    fn block_428<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_428_0: jump b109
        return block_109(state, tracer, fn_state);
    }
    fn block_429<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_429_0: const #64s : i64
        let s_429_0: i64 = 64;
        // C s_429_1: const #64s : i64
        let s_429_1: i64 = 64;
        // D s_429_2: read-var t:i
        let s_429_2: i128 = fn_state.t;
        // D s_429_3: call X_read(s_429_2, s_429_1)
        let s_429_3: Bits = X_read(state, tracer, s_429_2, s_429_1);
        // D s_429_4: cast reint s_429_3 -> u64
        let s_429_4: u64 = (s_429_3.value() as u64);
        // C s_429_5: const #0s : i
        let s_429_5: i128 = 0;
        // D s_429_6: cast zx s_429_4 -> bv
        let s_429_6: Bits = Bits::new(s_429_4 as u128, 64u16);
        // C s_429_7: const #22680u : u32
        let s_429_7: u32 = 22680;
        // D s_429_8: read-reg s_429_7:u32
        let s_429_8: u32 = {
            let value = state.read_register::<u32>(s_429_7 as isize);
            tracer.read_register(s_429_7 as isize, value);
            value
        };
        // D s_429_9: cast zx s_429_8 -> bv
        let s_429_9: Bits = Bits::new(s_429_8 as u128, 32u16);
        // C s_429_10: const #31s : i
        let s_429_10: i128 = 31;
        // C s_429_11: const #1u : u64
        let s_429_11: u64 = 1;
        // C s_429_12: cast zx s_429_11 -> bv
        let s_429_12: Bits = Bits::new(s_429_11 as u128, 64u16);
        // C s_429_13: lsl s_429_12 s_429_10
        let s_429_13: Bits = s_429_12 << s_429_10;
        // C s_429_14: sub s_429_13 s_429_12
        let s_429_14: Bits = ((s_429_13) - (s_429_12));
        // D s_429_15: and s_429_9 s_429_14
        let s_429_15: Bits = ((s_429_9) & (s_429_14));
        // D s_429_16: lsl s_429_15 s_429_5
        let s_429_16: Bits = s_429_15 << s_429_5;
        // C s_429_17: lsl s_429_14 s_429_5
        let s_429_17: Bits = s_429_14 << s_429_5;
        // C s_429_18: cmpl s_429_17
        let s_429_18: Bits = !s_429_17;
        // D s_429_19: and s_429_6 s_429_18
        let s_429_19: Bits = ((s_429_6) & (s_429_18));
        // D s_429_20: or s_429_19 s_429_16
        let s_429_20: Bits = ((s_429_19) | (s_429_16));
        // D s_429_21: cast reint s_429_20 -> u64
        let s_429_21: u64 = (s_429_20.value() as u64);
        // D s_429_22: cast zx s_429_21 -> bv
        let s_429_22: Bits = Bits::new(s_429_21 as u128, 64u16);
        // D s_429_23: read-var t:i
        let s_429_23: i128 = fn_state.t;
        // D s_429_24: call X_set(s_429_23, s_429_0, s_429_22)
        let s_429_24: () = X_set(state, tracer, s_429_23, s_429_0, s_429_22);
        // N s_429_25: return
        return;
    }
    fn block_430<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_430_0: const #64s : i64
        let s_430_0: i64 = 64;
        // C s_430_1: const #64s : i64
        let s_430_1: i64 = 64;
        // D s_430_2: read-var t:i
        let s_430_2: i128 = fn_state.t;
        // D s_430_3: call X_read(s_430_2, s_430_1)
        let s_430_3: Bits = X_read(state, tracer, s_430_2, s_430_1);
        // D s_430_4: cast reint s_430_3 -> u64
        let s_430_4: u64 = (s_430_3.value() as u64);
        // C s_430_5: const #0s : i
        let s_430_5: i128 = 0;
        // D s_430_6: cast zx s_430_4 -> bv
        let s_430_6: Bits = Bits::new(s_430_4 as u128, 64u16);
        // C s_430_7: const #22680u : u32
        let s_430_7: u32 = 22680;
        // D s_430_8: read-reg s_430_7:u32
        let s_430_8: u32 = {
            let value = state.read_register::<u32>(s_430_7 as isize);
            tracer.read_register(s_430_7 as isize, value);
            value
        };
        // D s_430_9: cast zx s_430_8 -> bv
        let s_430_9: Bits = Bits::new(s_430_8 as u128, 32u16);
        // C s_430_10: const #31s : i
        let s_430_10: i128 = 31;
        // C s_430_11: const #1u : u64
        let s_430_11: u64 = 1;
        // C s_430_12: cast zx s_430_11 -> bv
        let s_430_12: Bits = Bits::new(s_430_11 as u128, 64u16);
        // C s_430_13: lsl s_430_12 s_430_10
        let s_430_13: Bits = s_430_12 << s_430_10;
        // C s_430_14: sub s_430_13 s_430_12
        let s_430_14: Bits = ((s_430_13) - (s_430_12));
        // D s_430_15: and s_430_9 s_430_14
        let s_430_15: Bits = ((s_430_9) & (s_430_14));
        // D s_430_16: lsl s_430_15 s_430_5
        let s_430_16: Bits = s_430_15 << s_430_5;
        // C s_430_17: lsl s_430_14 s_430_5
        let s_430_17: Bits = s_430_14 << s_430_5;
        // C s_430_18: cmpl s_430_17
        let s_430_18: Bits = !s_430_17;
        // D s_430_19: and s_430_6 s_430_18
        let s_430_19: Bits = ((s_430_6) & (s_430_18));
        // D s_430_20: or s_430_19 s_430_16
        let s_430_20: Bits = ((s_430_19) | (s_430_16));
        // D s_430_21: cast reint s_430_20 -> u64
        let s_430_21: u64 = (s_430_20.value() as u64);
        // D s_430_22: cast zx s_430_21 -> bv
        let s_430_22: Bits = Bits::new(s_430_21 as u128, 64u16);
        // D s_430_23: read-var t:i
        let s_430_23: i128 = fn_state.t;
        // D s_430_24: call X_set(s_430_23, s_430_0, s_430_22)
        let s_430_24: () = X_set(state, tracer, s_430_23, s_430_0, s_430_22);
        // N s_430_25: return
        return;
    }
    fn block_431<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_431_0: const #6s : i
        let s_431_0: i128 = 6;
        // D s_431_1: read-var op2:i
        let s_431_1: i128 = fn_state.op2;
        // D s_431_2: cmp-eq s_431_1 s_431_0
        let s_431_2: bool = ((s_431_1) == (s_431_0));
        // D s_431_3: write-var gs#140598 <= s_431_2
        fn_state.gs_140598 = s_431_2;
        // N s_431_4: jump b107
        return block_107(state, tracer, fn_state);
    }
    fn block_432<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_432_0: const #7s : i
        let s_432_0: i128 = 7;
        // D s_432_1: read-var crn:i
        let s_432_1: i128 = fn_state.crn;
        // D s_432_2: cmp-eq s_432_1 s_432_0
        let s_432_2: bool = ((s_432_1) == (s_432_0));
        // D s_432_3: write-var gs#140596 <= s_432_2
        fn_state.gs_140596 = s_432_2;
        // N s_432_4: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_433<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_433_0: const #1s : i
        let s_433_0: i128 = 1;
        // D s_433_1: read-var op1:i
        let s_433_1: i128 = fn_state.op1;
        // D s_433_2: cmp-eq s_433_1 s_433_0
        let s_433_2: bool = ((s_433_1) == (s_433_0));
        // D s_433_3: write-var gs#140594 <= s_433_2
        fn_state.gs_140594 = s_433_2;
        // N s_433_4: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_434<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_434_0: const #8s : i
        let s_434_0: i128 = 8;
        // D s_434_1: read-var crm:i
        let s_434_1: i128 = fn_state.crm;
        // D s_434_2: cmp-eq s_434_1 s_434_0
        let s_434_2: bool = ((s_434_1) == (s_434_0));
        // N s_434_3: branch s_434_2 b438 b435
        if s_434_2 {
            return block_438(state, tracer, fn_state);
        } else {
            return block_435(state, tracer, fn_state);
        };
    }
    fn block_435<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_435_0: const #9s : i
        let s_435_0: i128 = 9;
        // D s_435_1: read-var crm:i
        let s_435_1: i128 = fn_state.crm;
        // D s_435_2: cmp-eq s_435_1 s_435_0
        let s_435_2: bool = ((s_435_1) == (s_435_0));
        // N s_435_3: branch s_435_2 b437 b436
        if s_435_2 {
            return block_437(state, tracer, fn_state);
        } else {
            return block_436(state, tracer, fn_state);
        };
    }
    fn block_436<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_436_0: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_437<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_437_0: const #64s : i64
        let s_437_0: i64 = 64;
        // C s_437_1: const #64s : i64
        let s_437_1: i64 = 64;
        // D s_437_2: read-var t:i
        let s_437_2: i128 = fn_state.t;
        // D s_437_3: call X_read(s_437_2, s_437_1)
        let s_437_3: Bits = X_read(state, tracer, s_437_2, s_437_1);
        // D s_437_4: cast reint s_437_3 -> u64
        let s_437_4: u64 = (s_437_3.value() as u64);
        // C s_437_5: const #21088u : u32
        let s_437_5: u32 = 21088;
        // D s_437_6: read-reg s_437_5:u64
        let s_437_6: u64 = {
            let value = state.read_register::<u64>(s_437_5 as isize);
            tracer.read_register(s_437_5 as isize, value);
            value
        };
        // C s_437_7: const #0s : i
        let s_437_7: i128 = 0;
        // D s_437_8: cast zx s_437_6 -> bv
        let s_437_8: Bits = Bits::new(s_437_6 as u128, 64u16);
        // C s_437_9: const #1s : i64
        let s_437_9: i64 = 1;
        // C s_437_10: cast zx s_437_9 -> i
        let s_437_10: i128 = (i128::try_from(s_437_9).unwrap());
        // C s_437_11: const #7s : i
        let s_437_11: i128 = 7;
        // C s_437_12: add s_437_11 s_437_10
        let s_437_12: i128 = (s_437_11 + s_437_10);
        // D s_437_13: bit-extract s_437_8 s_437_7 s_437_12
        let s_437_13: Bits = (Bits::new(
            ((s_437_8) >> (s_437_7)).value(),
            u16::try_from(s_437_12).unwrap(),
        ));
        // D s_437_14: cast reint s_437_13 -> u8
        let s_437_14: u8 = (s_437_13.value() as u8);
        // C s_437_15: const #0s : i
        let s_437_15: i128 = 0;
        // D s_437_16: cast zx s_437_4 -> bv
        let s_437_16: Bits = Bits::new(s_437_4 as u128, 64u16);
        // D s_437_17: cast zx s_437_14 -> bv
        let s_437_17: Bits = Bits::new(s_437_14 as u128, 8u16);
        // C s_437_18: const #7s : i
        let s_437_18: i128 = 7;
        // C s_437_19: const #1u : u64
        let s_437_19: u64 = 1;
        // C s_437_20: cast zx s_437_19 -> bv
        let s_437_20: Bits = Bits::new(s_437_19 as u128, 64u16);
        // C s_437_21: lsl s_437_20 s_437_18
        let s_437_21: Bits = s_437_20 << s_437_18;
        // C s_437_22: sub s_437_21 s_437_20
        let s_437_22: Bits = ((s_437_21) - (s_437_20));
        // D s_437_23: and s_437_17 s_437_22
        let s_437_23: Bits = ((s_437_17) & (s_437_22));
        // D s_437_24: lsl s_437_23 s_437_15
        let s_437_24: Bits = s_437_23 << s_437_15;
        // C s_437_25: lsl s_437_22 s_437_15
        let s_437_25: Bits = s_437_22 << s_437_15;
        // C s_437_26: cmpl s_437_25
        let s_437_26: Bits = !s_437_25;
        // D s_437_27: and s_437_16 s_437_26
        let s_437_27: Bits = ((s_437_16) & (s_437_26));
        // D s_437_28: or s_437_27 s_437_24
        let s_437_28: Bits = ((s_437_27) | (s_437_24));
        // D s_437_29: cast reint s_437_28 -> u64
        let s_437_29: u64 = (s_437_28.value() as u64);
        // D s_437_30: cast zx s_437_29 -> bv
        let s_437_30: Bits = Bits::new(s_437_29 as u128, 64u16);
        // D s_437_31: read-var t:i
        let s_437_31: i128 = fn_state.t;
        // D s_437_32: call X_set(s_437_31, s_437_0, s_437_30)
        let s_437_32: () = X_set(state, tracer, s_437_31, s_437_0, s_437_30);
        // N s_437_33: return
        return;
    }
    fn block_438<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_438_0: const #64s : i64
        let s_438_0: i64 = 64;
        // C s_438_1: const #64s : i64
        let s_438_1: i64 = 64;
        // D s_438_2: read-var t:i
        let s_438_2: i128 = fn_state.t;
        // D s_438_3: call X_read(s_438_2, s_438_1)
        let s_438_3: Bits = X_read(state, tracer, s_438_2, s_438_1);
        // D s_438_4: cast reint s_438_3 -> u64
        let s_438_4: u64 = (s_438_3.value() as u64);
        // C s_438_5: const #0s : i
        let s_438_5: i128 = 0;
        // D s_438_6: cast zx s_438_4 -> bv
        let s_438_6: Bits = Bits::new(s_438_4 as u128, 64u16);
        // C s_438_7: const #255u : u8
        let s_438_7: u8 = 255;
        // C s_438_8: cast zx s_438_7 -> bv
        let s_438_8: Bits = Bits::new(s_438_7 as u128, 8u16);
        // C s_438_9: const #7s : i
        let s_438_9: i128 = 7;
        // C s_438_10: const #1u : u64
        let s_438_10: u64 = 1;
        // C s_438_11: cast zx s_438_10 -> bv
        let s_438_11: Bits = Bits::new(s_438_10 as u128, 64u16);
        // C s_438_12: lsl s_438_11 s_438_9
        let s_438_12: Bits = s_438_11 << s_438_9;
        // C s_438_13: sub s_438_12 s_438_11
        let s_438_13: Bits = ((s_438_12) - (s_438_11));
        // C s_438_14: and s_438_8 s_438_13
        let s_438_14: Bits = ((s_438_8) & (s_438_13));
        // C s_438_15: lsl s_438_14 s_438_5
        let s_438_15: Bits = s_438_14 << s_438_5;
        // C s_438_16: lsl s_438_13 s_438_5
        let s_438_16: Bits = s_438_13 << s_438_5;
        // C s_438_17: cmpl s_438_16
        let s_438_17: Bits = !s_438_16;
        // D s_438_18: and s_438_6 s_438_17
        let s_438_18: Bits = ((s_438_6) & (s_438_17));
        // D s_438_19: or s_438_18 s_438_15
        let s_438_19: Bits = ((s_438_18) | (s_438_15));
        // D s_438_20: cast reint s_438_19 -> u64
        let s_438_20: u64 = (s_438_19.value() as u64);
        // D s_438_21: cast zx s_438_20 -> bv
        let s_438_21: Bits = Bits::new(s_438_20 as u128, 64u16);
        // D s_438_22: read-var t:i
        let s_438_22: i128 = fn_state.t;
        // D s_438_23: call X_set(s_438_22, s_438_0, s_438_21)
        let s_438_23: () = X_set(state, tracer, s_438_22, s_438_0, s_438_21);
        // N s_438_24: return
        return;
    }
    fn block_439<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_439_0: const #6s : i
        let s_439_0: i128 = 6;
        // D s_439_1: read-var op2:i
        let s_439_1: i128 = fn_state.op2;
        // D s_439_2: cmp-eq s_439_1 s_439_0
        let s_439_2: bool = ((s_439_1) == (s_439_0));
        // D s_439_3: write-var gs#140591 <= s_439_2
        fn_state.gs_140591 = s_439_2;
        // N s_439_4: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_440<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_440_0: const #7s : i
        let s_440_0: i128 = 7;
        // D s_440_1: read-var crn:i
        let s_440_1: i128 = fn_state.crn;
        // D s_440_2: cmp-eq s_440_1 s_440_0
        let s_440_2: bool = ((s_440_1) == (s_440_0));
        // D s_440_3: write-var gs#140589 <= s_440_2
        fn_state.gs_140589 = s_440_2;
        // N s_440_4: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_441<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_441_0: const #0s : i
        let s_441_0: i128 = 0;
        // D s_441_1: read-var op1:i
        let s_441_1: i128 = fn_state.op1;
        // D s_441_2: cmp-eq s_441_1 s_441_0
        let s_441_2: bool = ((s_441_1) == (s_441_0));
        // D s_441_3: write-var gs#140587 <= s_441_2
        fn_state.gs_140587 = s_441_2;
        // N s_441_4: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_442<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_442_0: const #1s : i
        let s_442_0: i128 = 1;
        // C s_442_1: const #0s : i
        let s_442_1: i128 = 0;
        // D s_442_2: read-var crm:i
        let s_442_2: i128 = fn_state.crm;
        // D s_442_3: call integer_subrange(s_442_2, s_442_0, s_442_1)
        let s_442_3: Bits = integer_subrange(state, tracer, s_442_2, s_442_0, s_442_1);
        // D s_442_4: cast reint s_442_3 -> u8
        let s_442_4: u8 = (s_442_3.value() as u8);
        // C s_442_5: const #2s : i
        let s_442_5: i128 = 2;
        // C s_442_6: const #0s : i
        let s_442_6: i128 = 0;
        // D s_442_7: read-var op2:i
        let s_442_7: i128 = fn_state.op2;
        // D s_442_8: call integer_subrange(s_442_7, s_442_5, s_442_6)
        let s_442_8: Bits = integer_subrange(state, tracer, s_442_7, s_442_5, s_442_6);
        // D s_442_9: cast reint s_442_8 -> u8
        let s_442_9: u8 = (s_442_8.value() as u8);
        // D s_442_10: cast zx s_442_4 -> bv
        let s_442_10: Bits = Bits::new(s_442_4 as u128, 2u16);
        // D s_442_11: cast zx s_442_9 -> bv
        let s_442_11: Bits = Bits::new(s_442_9 as u128, 3u16);
        // D s_442_12: cast reint s_442_10 -> u128
        let s_442_12: u128 = (s_442_10.value() as u128);
        // D s_442_13: size-of s_442_10
        let s_442_13: u16 = s_442_10.length();
        // D s_442_14: cast reint s_442_11 -> u128
        let s_442_14: u128 = (s_442_11.value() as u128);
        // D s_442_15: size-of s_442_11
        let s_442_15: u16 = s_442_11.length();
        // D s_442_16: lsl s_442_12 s_442_15
        let s_442_16: u128 = s_442_12 << s_442_15;
        // D s_442_17: or s_442_16 s_442_14
        let s_442_17: u128 = ((s_442_16) | (s_442_14));
        // D s_442_18: add s_442_13 s_442_15
        let s_442_18: u16 = (s_442_13 + s_442_15);
        // D s_442_19: create-bits s_442_17 s_442_18
        let s_442_19: Bits = Bits::new(s_442_17, s_442_18);
        // D s_442_20: cast reint s_442_19 -> u8
        let s_442_20: u8 = (s_442_19.value() as u8);
        // D s_442_21: cast zx s_442_20 -> bv
        let s_442_21: Bits = Bits::new(s_442_20 as u128, 5u16);
        // D s_442_22: cast zx s_442_21 -> i
        let s_442_22: i128 = (s_442_21.value() as i128);
        // D s_442_23: cast reint s_442_22 -> i64
        let s_442_23: i64 = (s_442_22 as i64);
        // C s_442_24: const #() : ()
        let s_442_24: () = ();
        // S s_442_25: call GetNumEventCounters(s_442_24)
        let s_442_25: i128 = GetNumEventCounters(state, tracer, s_442_24);
        // C s_442_26: const #1s : i
        let s_442_26: i128 = 1;
        // S s_442_27: sub s_442_25 s_442_26
        let s_442_27: i128 = ((s_442_25) - (s_442_26));
        // D s_442_28: cast zx s_442_23 -> i
        let s_442_28: i128 = (i128::try_from(s_442_23).unwrap());
        // D s_442_29: cmp-gt s_442_28 s_442_27
        let s_442_29: bool = ((s_442_28) > (s_442_27));
        // N s_442_30: branch s_442_29 b462 b443
        if s_442_29 {
            return block_462(state, tracer, fn_state);
        } else {
            return block_443(state, tracer, fn_state);
        };
    }
    fn block_443<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_443_0: const #0u : u8
        let s_443_0: bool = false;
        // D s_443_1: write-var gs#140960 <= s_443_0
        fn_state.gs_140960 = s_443_0;
        // N s_443_2: jump b444
        return block_444(state, tracer, fn_state);
    }
    fn block_444<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_444_0: read-var gs#140960:u8
        let s_444_0: bool = fn_state.gs_140960;
        // N s_444_1: branch s_444_0 b461 b445
        if s_444_0 {
            return block_461(state, tracer, fn_state);
        } else {
            return block_445(state, tracer, fn_state);
        };
    }
    fn block_445<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_445_0: const #() : ()
        let s_445_0: () = ();
        // S s_445_1: call EL2Enabled(s_445_0)
        let s_445_1: bool = EL2Enabled(state, tracer, s_445_0);
        // N s_445_2: branch s_445_1 b457 b446
        if s_445_1 {
            return block_457(state, tracer, fn_state);
        } else {
            return block_446(state, tracer, fn_state);
        };
    }
    fn block_446<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_446_0: const #0u : u8
        let s_446_0: bool = false;
        // D s_446_1: write-var gs#140962 <= s_446_0
        fn_state.gs_140962 = s_446_0;
        // N s_446_2: jump b447
        return block_447(state, tracer, fn_state);
    }
    fn block_447<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_447_0: read-var gs#140962:u8
        let s_447_0: bool = fn_state.gs_140962;
        // N s_447_1: branch s_447_0 b456 b448
        if s_447_0 {
            return block_456(state, tracer, fn_state);
        } else {
            return block_448(state, tracer, fn_state);
        };
    }
    fn block_448<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_448_0: const #0u : u8
        let s_448_0: bool = false;
        // D s_448_1: write-var gs#140968 <= s_448_0
        fn_state.gs_140968 = s_448_0;
        // N s_448_2: jump b449
        return block_449(state, tracer, fn_state);
    }
    fn block_449<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_449_0: read-var gs#140968:u8
        let s_449_0: bool = fn_state.gs_140968;
        // D s_449_1: write-var gs#140969 <= s_449_0
        fn_state.gs_140969 = s_449_0;
        // N s_449_2: jump b450
        return block_450(state, tracer, fn_state);
    }
    fn block_450<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_450_0: read-var gs#140969:u8
        let s_450_0: bool = fn_state.gs_140969;
        // N s_450_1: branch s_450_0 b453 b451
        if s_450_0 {
            return block_453(state, tracer, fn_state);
        } else {
            return block_451(state, tracer, fn_state);
        };
    }
    fn block_451<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_451_0: jump b452
        return block_452(state, tracer, fn_state);
    }
    fn block_452<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_452_0: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_453<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_453_0: const #1s : i
        let s_453_0: i128 = 1;
        // C s_453_1: const #0s : i
        let s_453_1: i128 = 0;
        // D s_453_2: read-var crm:i
        let s_453_2: i128 = fn_state.crm;
        // D s_453_3: call integer_subrange(s_453_2, s_453_0, s_453_1)
        let s_453_3: Bits = integer_subrange(state, tracer, s_453_2, s_453_0, s_453_1);
        // D s_453_4: cast reint s_453_3 -> u8
        let s_453_4: u8 = (s_453_3.value() as u8);
        // C s_453_5: const #2s : i
        let s_453_5: i128 = 2;
        // C s_453_6: const #0s : i
        let s_453_6: i128 = 0;
        // D s_453_7: read-var op2:i
        let s_453_7: i128 = fn_state.op2;
        // D s_453_8: call integer_subrange(s_453_7, s_453_5, s_453_6)
        let s_453_8: Bits = integer_subrange(state, tracer, s_453_7, s_453_5, s_453_6);
        // D s_453_9: cast reint s_453_8 -> u8
        let s_453_9: u8 = (s_453_8.value() as u8);
        // D s_453_10: cast zx s_453_4 -> bv
        let s_453_10: Bits = Bits::new(s_453_4 as u128, 2u16);
        // D s_453_11: cast zx s_453_9 -> bv
        let s_453_11: Bits = Bits::new(s_453_9 as u128, 3u16);
        // D s_453_12: cast reint s_453_10 -> u128
        let s_453_12: u128 = (s_453_10.value() as u128);
        // D s_453_13: size-of s_453_10
        let s_453_13: u16 = s_453_10.length();
        // D s_453_14: cast reint s_453_11 -> u128
        let s_453_14: u128 = (s_453_11.value() as u128);
        // D s_453_15: size-of s_453_11
        let s_453_15: u16 = s_453_11.length();
        // D s_453_16: lsl s_453_12 s_453_15
        let s_453_16: u128 = s_453_12 << s_453_15;
        // D s_453_17: or s_453_16 s_453_14
        let s_453_17: u128 = ((s_453_16) | (s_453_14));
        // D s_453_18: add s_453_13 s_453_15
        let s_453_18: u16 = (s_453_13 + s_453_15);
        // D s_453_19: create-bits s_453_17 s_453_18
        let s_453_19: Bits = Bits::new(s_453_17, s_453_18);
        // D s_453_20: cast reint s_453_19 -> u8
        let s_453_20: u8 = (s_453_19.value() as u8);
        // D s_453_21: cast zx s_453_20 -> bv
        let s_453_21: Bits = Bits::new(s_453_20 as u128, 5u16);
        // D s_453_22: cast zx s_453_21 -> i
        let s_453_22: i128 = (s_453_21.value() as i128);
        // D s_453_23: cast reint s_453_22 -> i64
        let s_453_23: i64 = (s_453_22 as i64);
        // C s_453_24: const #() : ()
        let s_453_24: () = ();
        // S s_453_25: call GetNumEventCounters(s_453_24)
        let s_453_25: i128 = GetNumEventCounters(state, tracer, s_453_24);
        // C s_453_26: const #1s : i
        let s_453_26: i128 = 1;
        // S s_453_27: sub s_453_25 s_453_26
        let s_453_27: i128 = ((s_453_25) - (s_453_26));
        // D s_453_28: cast zx s_453_23 -> i
        let s_453_28: i128 = (i128::try_from(s_453_23).unwrap());
        // D s_453_29: cmp-gt s_453_28 s_453_27
        let s_453_29: bool = ((s_453_28) > (s_453_27));
        // N s_453_30: branch s_453_29 b455 b454
        if s_453_29 {
            return block_455(state, tracer, fn_state);
        } else {
            return block_454(state, tracer, fn_state);
        };
    }
    fn block_454<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_454_0: const #64s : i64
        let s_454_0: i64 = 64;
        // D s_454_1: read-var temp:u64
        let s_454_1: u64 = fn_state.temp;
        // D s_454_2: cast zx s_454_1 -> bv
        let s_454_2: Bits = Bits::new(s_454_1 as u128, 64u16);
        // D s_454_3: read-var t:i
        let s_454_3: i128 = fn_state.t;
        // D s_454_4: call X_set(s_454_3, s_454_0, s_454_2)
        let s_454_4: () = X_set(state, tracer, s_454_3, s_454_0, s_454_2);
        // C s_454_5: const #24u : u8
        let s_454_5: u8 = 24;
        // C s_454_6: cast zx s_454_5 -> bv
        let s_454_6: Bits = Bits::new(s_454_5 as u128, 8u16);
        // C s_454_7: cast zx s_454_6 -> i
        let s_454_7: i128 = (s_454_6.value() as i128);
        // C s_454_8: cast reint s_454_7 -> i64
        let s_454_8: i64 = (s_454_7 as i64);
        // C s_454_9: cast zx s_454_8 -> i
        let s_454_9: i128 = (i128::try_from(s_454_8).unwrap());
        // C s_454_10: const #432u : u32
        let s_454_10: u32 = 432;
        // D s_454_11: read-reg s_454_10:u8
        let s_454_11: u8 = {
            let value = state.read_register::<u8>(s_454_10 as isize);
            tracer.read_register(s_454_10 as isize, value);
            value
        };
        // D s_454_12: call AArch64_SystemAccessTrap(s_454_11, s_454_9)
        let s_454_12: () = AArch64_SystemAccessTrap(state, tracer, s_454_11, s_454_9);
        // N s_454_13: jump b452
        return block_452(state, tracer, fn_state);
    }
    fn block_455<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_455_0: panic
        panic!("{:?}", ());
        // N s_455_1: return
        return;
    }
    fn block_456<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_456_0: const #1s : i
        let s_456_0: i128 = 1;
        // C s_456_1: const #0s : i
        let s_456_1: i128 = 0;
        // D s_456_2: read-var crm:i
        let s_456_2: i128 = fn_state.crm;
        // D s_456_3: call integer_subrange(s_456_2, s_456_0, s_456_1)
        let s_456_3: Bits = integer_subrange(state, tracer, s_456_2, s_456_0, s_456_1);
        // D s_456_4: cast reint s_456_3 -> u8
        let s_456_4: u8 = (s_456_3.value() as u8);
        // C s_456_5: const #2s : i
        let s_456_5: i128 = 2;
        // C s_456_6: const #0s : i
        let s_456_6: i128 = 0;
        // D s_456_7: read-var op2:i
        let s_456_7: i128 = fn_state.op2;
        // D s_456_8: call integer_subrange(s_456_7, s_456_5, s_456_6)
        let s_456_8: Bits = integer_subrange(state, tracer, s_456_7, s_456_5, s_456_6);
        // D s_456_9: cast reint s_456_8 -> u8
        let s_456_9: u8 = (s_456_8.value() as u8);
        // D s_456_10: cast zx s_456_4 -> bv
        let s_456_10: Bits = Bits::new(s_456_4 as u128, 2u16);
        // D s_456_11: cast zx s_456_9 -> bv
        let s_456_11: Bits = Bits::new(s_456_9 as u128, 3u16);
        // D s_456_12: cast reint s_456_10 -> u128
        let s_456_12: u128 = (s_456_10.value() as u128);
        // D s_456_13: size-of s_456_10
        let s_456_13: u16 = s_456_10.length();
        // D s_456_14: cast reint s_456_11 -> u128
        let s_456_14: u128 = (s_456_11.value() as u128);
        // D s_456_15: size-of s_456_11
        let s_456_15: u16 = s_456_11.length();
        // D s_456_16: lsl s_456_12 s_456_15
        let s_456_16: u128 = s_456_12 << s_456_15;
        // D s_456_17: or s_456_16 s_456_14
        let s_456_17: u128 = ((s_456_16) | (s_456_14));
        // D s_456_18: add s_456_13 s_456_15
        let s_456_18: u16 = (s_456_13 + s_456_15);
        // D s_456_19: create-bits s_456_17 s_456_18
        let s_456_19: Bits = Bits::new(s_456_17, s_456_18);
        // D s_456_20: cast reint s_456_19 -> u8
        let s_456_20: u8 = (s_456_19.value() as u8);
        // D s_456_21: cast zx s_456_20 -> bv
        let s_456_21: Bits = Bits::new(s_456_20 as u128, 5u16);
        // D s_456_22: cast zx s_456_21 -> i
        let s_456_22: i128 = (s_456_21.value() as i128);
        // D s_456_23: cast reint s_456_22 -> i64
        let s_456_23: i64 = (s_456_22 as i64);
        // C s_456_24: const #() : ()
        let s_456_24: () = ();
        // S s_456_25: call AArch64_GetNumEventCountersAccessible(s_456_24)
        let s_456_25: i128 = AArch64_GetNumEventCountersAccessible(
            state,
            tracer,
            s_456_24,
        );
        // C s_456_26: const #1s : i
        let s_456_26: i128 = 1;
        // S s_456_27: sub s_456_25 s_456_26
        let s_456_27: i128 = ((s_456_25) - (s_456_26));
        // D s_456_28: cast zx s_456_23 -> i
        let s_456_28: i128 = (i128::try_from(s_456_23).unwrap());
        // D s_456_29: cmp-gt s_456_28 s_456_27
        let s_456_29: bool = ((s_456_28) > (s_456_27));
        // D s_456_30: write-var gs#140968 <= s_456_29
        fn_state.gs_140968 = s_456_29;
        // N s_456_31: jump b449
        return block_449(state, tracer, fn_state);
    }
    fn block_457<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_457_0: const #16975u : u32
        let s_457_0: u32 = 16975;
        // D s_457_1: read-reg s_457_0:u8
        let s_457_1: u8 = {
            let value = state.read_register::<u8>(s_457_0 as isize);
            tracer.read_register(s_457_0 as isize, value);
            value
        };
        // D s_457_2: cast zx s_457_1 -> bv
        let s_457_2: Bits = Bits::new(s_457_1 as u128, 2u16);
        // C s_457_3: const #448u : u32
        let s_457_3: u32 = 448;
        // D s_457_4: read-reg s_457_3:u8
        let s_457_4: u8 = {
            let value = state.read_register::<u8>(s_457_3 as isize);
            tracer.read_register(s_457_3 as isize, value);
            value
        };
        // D s_457_5: cast zx s_457_4 -> bv
        let s_457_5: Bits = Bits::new(s_457_4 as u128, 2u16);
        // D s_457_6: cmp-eq s_457_2 s_457_5
        let s_457_6: bool = ((s_457_2) == (s_457_5));
        // N s_457_7: branch s_457_6 b460 b458
        if s_457_6 {
            return block_460(state, tracer, fn_state);
        } else {
            return block_458(state, tracer, fn_state);
        };
    }
    fn block_458<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_458_0: const #16975u : u32
        let s_458_0: u32 = 16975;
        // D s_458_1: read-reg s_458_0:u8
        let s_458_1: u8 = {
            let value = state.read_register::<u8>(s_458_0 as isize);
            tracer.read_register(s_458_0 as isize, value);
            value
        };
        // D s_458_2: cast zx s_458_1 -> bv
        let s_458_2: Bits = Bits::new(s_458_1 as u128, 2u16);
        // C s_458_3: const #440u : u32
        let s_458_3: u32 = 440;
        // D s_458_4: read-reg s_458_3:u8
        let s_458_4: u8 = {
            let value = state.read_register::<u8>(s_458_3 as isize);
            tracer.read_register(s_458_3 as isize, value);
            value
        };
        // D s_458_5: cast zx s_458_4 -> bv
        let s_458_5: Bits = Bits::new(s_458_4 as u128, 2u16);
        // D s_458_6: cmp-eq s_458_2 s_458_5
        let s_458_6: bool = ((s_458_2) == (s_458_5));
        // D s_458_7: write-var gs#140961 <= s_458_6
        fn_state.gs_140961 = s_458_6;
        // N s_458_8: jump b459
        return block_459(state, tracer, fn_state);
    }
    fn block_459<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_459_0: read-var gs#140961:u8
        let s_459_0: bool = fn_state.gs_140961;
        // D s_459_1: write-var gs#140962 <= s_459_0
        fn_state.gs_140962 = s_459_0;
        // N s_459_2: jump b447
        return block_447(state, tracer, fn_state);
    }
    fn block_460<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_460_0: const #1u : u8
        let s_460_0: bool = true;
        // D s_460_1: write-var gs#140961 <= s_460_0
        fn_state.gs_140961 = s_460_0;
        // N s_460_2: jump b459
        return block_459(state, tracer, fn_state);
    }
    fn block_461<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_461_0: const #1u : u8
        let s_461_0: bool = true;
        // D s_461_1: write-var gs#140969 <= s_461_0
        fn_state.gs_140969 = s_461_0;
        // N s_461_2: jump b450
        return block_450(state, tracer, fn_state);
    }
    fn block_462<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_462_0: const #1s : i
        let s_462_0: i128 = 1;
        // C s_462_1: const #0s : i
        let s_462_1: i128 = 0;
        // D s_462_2: read-var crm:i
        let s_462_2: i128 = fn_state.crm;
        // D s_462_3: call integer_subrange(s_462_2, s_462_0, s_462_1)
        let s_462_3: Bits = integer_subrange(state, tracer, s_462_2, s_462_0, s_462_1);
        // D s_462_4: cast reint s_462_3 -> u8
        let s_462_4: u8 = (s_462_3.value() as u8);
        // C s_462_5: const #2s : i
        let s_462_5: i128 = 2;
        // C s_462_6: const #0s : i
        let s_462_6: i128 = 0;
        // D s_462_7: read-var op2:i
        let s_462_7: i128 = fn_state.op2;
        // D s_462_8: call integer_subrange(s_462_7, s_462_5, s_462_6)
        let s_462_8: Bits = integer_subrange(state, tracer, s_462_7, s_462_5, s_462_6);
        // D s_462_9: cast reint s_462_8 -> u8
        let s_462_9: u8 = (s_462_8.value() as u8);
        // D s_462_10: cast zx s_462_4 -> bv
        let s_462_10: Bits = Bits::new(s_462_4 as u128, 2u16);
        // D s_462_11: cast zx s_462_9 -> bv
        let s_462_11: Bits = Bits::new(s_462_9 as u128, 3u16);
        // D s_462_12: cast reint s_462_10 -> u128
        let s_462_12: u128 = (s_462_10.value() as u128);
        // D s_462_13: size-of s_462_10
        let s_462_13: u16 = s_462_10.length();
        // D s_462_14: cast reint s_462_11 -> u128
        let s_462_14: u128 = (s_462_11.value() as u128);
        // D s_462_15: size-of s_462_11
        let s_462_15: u16 = s_462_11.length();
        // D s_462_16: lsl s_462_12 s_462_15
        let s_462_16: u128 = s_462_12 << s_462_15;
        // D s_462_17: or s_462_16 s_462_14
        let s_462_17: u128 = ((s_462_16) | (s_462_14));
        // D s_462_18: add s_462_13 s_462_15
        let s_462_18: u16 = (s_462_13 + s_462_15);
        // D s_462_19: create-bits s_462_17 s_462_18
        let s_462_19: Bits = Bits::new(s_462_17, s_462_18);
        // D s_462_20: cast reint s_462_19 -> u8
        let s_462_20: u8 = (s_462_19.value() as u8);
        // D s_462_21: cast zx s_462_20 -> bv
        let s_462_21: Bits = Bits::new(s_462_20 as u128, 5u16);
        // C s_462_22: const #31u : u8
        let s_462_22: u8 = 31;
        // C s_462_23: cast zx s_462_22 -> bv
        let s_462_23: Bits = Bits::new(s_462_22 as u128, 5u16);
        // D s_462_24: cmp-ne s_462_21 s_462_23
        let s_462_24: bool = ((s_462_21) != (s_462_23));
        // D s_462_25: write-var gs#140960 <= s_462_24
        fn_state.gs_140960 = s_462_24;
        // N s_462_26: jump b444
        return block_444(state, tracer, fn_state);
    }
    fn block_463<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_463_0: const #3s : i
        let s_463_0: i128 = 3;
        // C s_463_1: const #2s : i
        let s_463_1: i128 = 2;
        // D s_463_2: read-var crm:i
        let s_463_2: i128 = fn_state.crm;
        // D s_463_3: call integer_subrange(s_463_2, s_463_0, s_463_1)
        let s_463_3: Bits = integer_subrange(state, tracer, s_463_2, s_463_0, s_463_1);
        // D s_463_4: cast reint s_463_3 -> u8
        let s_463_4: u8 = (s_463_3.value() as u8);
        // D s_463_5: cast zx s_463_4 -> bv
        let s_463_5: Bits = Bits::new(s_463_4 as u128, 2u16);
        // C s_463_6: const #2u : u8
        let s_463_6: u8 = 2;
        // C s_463_7: cast zx s_463_6 -> bv
        let s_463_7: Bits = Bits::new(s_463_6 as u128, 2u16);
        // D s_463_8: cmp-eq s_463_5 s_463_7
        let s_463_8: bool = ((s_463_5) == (s_463_7));
        // N s_463_9: branch s_463_8 b469 b464
        if s_463_8 {
            return block_469(state, tracer, fn_state);
        } else {
            return block_464(state, tracer, fn_state);
        };
    }
    fn block_464<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_464_0: const #3s : i
        let s_464_0: i128 = 3;
        // C s_464_1: const #2s : i
        let s_464_1: i128 = 2;
        // D s_464_2: read-var crm:i
        let s_464_2: i128 = fn_state.crm;
        // D s_464_3: call integer_subrange(s_464_2, s_464_0, s_464_1)
        let s_464_3: Bits = integer_subrange(state, tracer, s_464_2, s_464_0, s_464_1);
        // D s_464_4: cast reint s_464_3 -> u8
        let s_464_4: u8 = (s_464_3.value() as u8);
        // D s_464_5: cast zx s_464_4 -> bv
        let s_464_5: Bits = Bits::new(s_464_4 as u128, 2u16);
        // C s_464_6: const #3u : u8
        let s_464_6: u8 = 3;
        // C s_464_7: cast zx s_464_6 -> bv
        let s_464_7: Bits = Bits::new(s_464_6 as u128, 2u16);
        // D s_464_8: cmp-eq s_464_5 s_464_7
        let s_464_8: bool = ((s_464_5) == (s_464_7));
        // N s_464_9: branch s_464_8 b468 b465
        if s_464_8 {
            return block_468(state, tracer, fn_state);
        } else {
            return block_465(state, tracer, fn_state);
        };
    }
    fn block_465<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_465_0: const #0u : u8
        let s_465_0: bool = false;
        // D s_465_1: write-var gs#140582 <= s_465_0
        fn_state.gs_140582 = s_465_0;
        // N s_465_2: jump b466
        return block_466(state, tracer, fn_state);
    }
    fn block_466<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_466_0: read-var gs#140582:u8
        let s_466_0: bool = fn_state.gs_140582;
        // D s_466_1: write-var gs#140583 <= s_466_0
        fn_state.gs_140583 = s_466_0;
        // N s_466_2: jump b467
        return block_467(state, tracer, fn_state);
    }
    fn block_467<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_467_0: read-var gs#140583:u8
        let s_467_0: bool = fn_state.gs_140583;
        // D s_467_1: write-var gs#140584 <= s_467_0
        fn_state.gs_140584 = s_467_0;
        // N s_467_2: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_468<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_468_0: const #1s : i
        let s_468_0: i128 = 1;
        // C s_468_1: const #0s : i
        let s_468_1: i128 = 0;
        // D s_468_2: read-var crm:i
        let s_468_2: i128 = fn_state.crm;
        // D s_468_3: call integer_subrange(s_468_2, s_468_0, s_468_1)
        let s_468_3: Bits = integer_subrange(state, tracer, s_468_2, s_468_0, s_468_1);
        // D s_468_4: cast reint s_468_3 -> u8
        let s_468_4: u8 = (s_468_3.value() as u8);
        // C s_468_5: const #2s : i
        let s_468_5: i128 = 2;
        // C s_468_6: const #0s : i
        let s_468_6: i128 = 0;
        // D s_468_7: read-var op2:i
        let s_468_7: i128 = fn_state.op2;
        // D s_468_8: call integer_subrange(s_468_7, s_468_5, s_468_6)
        let s_468_8: Bits = integer_subrange(state, tracer, s_468_7, s_468_5, s_468_6);
        // D s_468_9: cast reint s_468_8 -> u8
        let s_468_9: u8 = (s_468_8.value() as u8);
        // D s_468_10: cast zx s_468_4 -> bv
        let s_468_10: Bits = Bits::new(s_468_4 as u128, 2u16);
        // D s_468_11: cast zx s_468_9 -> bv
        let s_468_11: Bits = Bits::new(s_468_9 as u128, 3u16);
        // D s_468_12: cast reint s_468_10 -> u128
        let s_468_12: u128 = (s_468_10.value() as u128);
        // D s_468_13: size-of s_468_10
        let s_468_13: u16 = s_468_10.length();
        // D s_468_14: cast reint s_468_11 -> u128
        let s_468_14: u128 = (s_468_11.value() as u128);
        // D s_468_15: size-of s_468_11
        let s_468_15: u16 = s_468_11.length();
        // D s_468_16: lsl s_468_12 s_468_15
        let s_468_16: u128 = s_468_12 << s_468_15;
        // D s_468_17: or s_468_16 s_468_14
        let s_468_17: u128 = ((s_468_16) | (s_468_14));
        // D s_468_18: add s_468_13 s_468_15
        let s_468_18: u16 = (s_468_13 + s_468_15);
        // D s_468_19: create-bits s_468_17 s_468_18
        let s_468_19: Bits = Bits::new(s_468_17, s_468_18);
        // D s_468_20: cast reint s_468_19 -> u8
        let s_468_20: u8 = (s_468_19.value() as u8);
        // D s_468_21: cast zx s_468_20 -> bv
        let s_468_21: Bits = Bits::new(s_468_20 as u128, 5u16);
        // C s_468_22: const #31u : u8
        let s_468_22: u8 = 31;
        // C s_468_23: cast zx s_468_22 -> bv
        let s_468_23: Bits = Bits::new(s_468_22 as u128, 5u16);
        // D s_468_24: cmp-ne s_468_21 s_468_23
        let s_468_24: bool = ((s_468_21) != (s_468_23));
        // D s_468_25: write-var gs#140582 <= s_468_24
        fn_state.gs_140582 = s_468_24;
        // N s_468_26: jump b466
        return block_466(state, tracer, fn_state);
    }
    fn block_469<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_469_0: const #1u : u8
        let s_469_0: bool = true;
        // D s_469_1: write-var gs#140583 <= s_469_0
        fn_state.gs_140583 = s_469_0;
        // N s_469_2: jump b467
        return block_467(state, tracer, fn_state);
    }
    fn block_470<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_470_0: const #3s : i
        let s_470_0: i128 = 3;
        // D s_470_1: read-var op1:i
        let s_470_1: i128 = fn_state.op1;
        // D s_470_2: cmp-eq s_470_1 s_470_0
        let s_470_2: bool = ((s_470_1) == (s_470_0));
        // D s_470_3: write-var gs#140573 <= s_470_2
        fn_state.gs_140573 = s_470_2;
        // N s_470_4: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_471<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_471_0: const #14s : i
        let s_471_0: i128 = 14;
        // D s_471_1: read-var crn:i
        let s_471_1: i128 = fn_state.crn;
        // D s_471_2: cmp-eq s_471_1 s_471_0
        let s_471_2: bool = ((s_471_1) == (s_471_0));
        // D s_471_3: write-var gs#140571 <= s_471_2
        fn_state.gs_140571 = s_471_2;
        // N s_471_4: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_472<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_472_0: const #64s : i64
        let s_472_0: i64 = 64;
        // C s_472_1: const #64s : i64
        let s_472_1: i64 = 64;
        // D s_472_2: read-var t:i
        let s_472_2: i128 = fn_state.t;
        // D s_472_3: call X_read(s_472_2, s_472_1)
        let s_472_3: Bits = X_read(state, tracer, s_472_2, s_472_1);
        // D s_472_4: cast reint s_472_3 -> u64
        let s_472_4: u64 = (s_472_3.value() as u64);
        // C s_472_5: const #() : ()
        let s_472_5: () = ();
        // S s_472_6: call AArch64_GetNumEventCountersAccessible(s_472_5)
        let s_472_6: i128 = AArch64_GetNumEventCountersAccessible(
            state,
            tracer,
            s_472_5,
        );
        // C s_472_7: const #4s : i
        let s_472_7: i128 = 4;
        // C s_472_8: const #0s : i
        let s_472_8: i128 = 0;
        // S s_472_9: call integer_subrange(s_472_6, s_472_7, s_472_8)
        let s_472_9: Bits = integer_subrange(state, tracer, s_472_6, s_472_7, s_472_8);
        // S s_472_10: cast reint s_472_9 -> u8
        let s_472_10: u8 = (s_472_9.value() as u8);
        // C s_472_11: const #11s : i
        let s_472_11: i128 = 11;
        // D s_472_12: cast zx s_472_4 -> bv
        let s_472_12: Bits = Bits::new(s_472_4 as u128, 64u16);
        // S s_472_13: cast zx s_472_10 -> bv
        let s_472_13: Bits = Bits::new(s_472_10 as u128, 5u16);
        // C s_472_14: const #4s : i
        let s_472_14: i128 = 4;
        // C s_472_15: const #1u : u64
        let s_472_15: u64 = 1;
        // C s_472_16: cast zx s_472_15 -> bv
        let s_472_16: Bits = Bits::new(s_472_15 as u128, 64u16);
        // C s_472_17: lsl s_472_16 s_472_14
        let s_472_17: Bits = s_472_16 << s_472_14;
        // C s_472_18: sub s_472_17 s_472_16
        let s_472_18: Bits = ((s_472_17) - (s_472_16));
        // S s_472_19: and s_472_13 s_472_18
        let s_472_19: Bits = ((s_472_13) & (s_472_18));
        // S s_472_20: lsl s_472_19 s_472_11
        let s_472_20: Bits = s_472_19 << s_472_11;
        // C s_472_21: lsl s_472_18 s_472_11
        let s_472_21: Bits = s_472_18 << s_472_11;
        // C s_472_22: cmpl s_472_21
        let s_472_22: Bits = !s_472_21;
        // D s_472_23: and s_472_12 s_472_22
        let s_472_23: Bits = ((s_472_12) & (s_472_22));
        // D s_472_24: or s_472_23 s_472_20
        let s_472_24: Bits = ((s_472_23) | (s_472_20));
        // D s_472_25: cast reint s_472_24 -> u64
        let s_472_25: u64 = (s_472_24.value() as u64);
        // D s_472_26: cast zx s_472_25 -> bv
        let s_472_26: Bits = Bits::new(s_472_25 as u128, 64u16);
        // D s_472_27: read-var t:i
        let s_472_27: i128 = fn_state.t;
        // D s_472_28: call X_set(s_472_27, s_472_0, s_472_26)
        let s_472_28: () = X_set(state, tracer, s_472_27, s_472_0, s_472_26);
        // N s_472_29: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_473<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_473_0: const #12s : i
        let s_473_0: i128 = 12;
        // D s_473_1: read-var crm:i
        let s_473_1: i128 = fn_state.crm;
        // D s_473_2: cmp-eq s_473_1 s_473_0
        let s_473_2: bool = ((s_473_1) == (s_473_0));
        // D s_473_3: write-var gs#140568 <= s_473_2
        fn_state.gs_140568 = s_473_2;
        // N s_473_4: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_474<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_474_0: const #0s : i
        let s_474_0: i128 = 0;
        // D s_474_1: read-var op2:i
        let s_474_1: i128 = fn_state.op2;
        // D s_474_2: cmp-eq s_474_1 s_474_0
        let s_474_2: bool = ((s_474_1) == (s_474_0));
        // D s_474_3: write-var gs#140566 <= s_474_2
        fn_state.gs_140566 = s_474_2;
        // N s_474_4: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_475<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_475_0: const #3s : i
        let s_475_0: i128 = 3;
        // D s_475_1: read-var op1:i
        let s_475_1: i128 = fn_state.op1;
        // D s_475_2: cmp-eq s_475_1 s_475_0
        let s_475_2: bool = ((s_475_1) == (s_475_0));
        // D s_475_3: write-var gs#140564 <= s_475_2
        fn_state.gs_140564 = s_475_2;
        // N s_475_4: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_476<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_476_0: const #9s : i
        let s_476_0: i128 = 9;
        // D s_476_1: read-var crn:i
        let s_476_1: i128 = fn_state.crn;
        // D s_476_2: cmp-eq s_476_1 s_476_0
        let s_476_2: bool = ((s_476_1) == (s_476_0));
        // D s_476_3: write-var gs#140562 <= s_476_2
        fn_state.gs_140562 = s_476_2;
        // N s_476_4: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_477<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_477_0: const #1s : i
        let s_477_0: i128 = 1;
        // D s_477_1: read-var op2:i
        let s_477_1: i128 = fn_state.op2;
        // D s_477_2: cmp-eq s_477_1 s_477_0
        let s_477_2: bool = ((s_477_1) == (s_477_0));
        // N s_477_3: branch s_477_2 b502 b478
        if s_477_2 {
            return block_502(state, tracer, fn_state);
        } else {
            return block_478(state, tracer, fn_state);
        };
    }
    fn block_478<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_478_0: jump b479
        return block_479(state, tracer, fn_state);
    }
    fn block_479<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_479_0: const #2s : i
        let s_479_0: i128 = 2;
        // D s_479_1: read-var op2:i
        let s_479_1: i128 = fn_state.op2;
        // D s_479_2: cmp-eq s_479_1 s_479_0
        let s_479_2: bool = ((s_479_1) == (s_479_0));
        // N s_479_3: branch s_479_2 b482 b480
        if s_479_2 {
            return block_482(state, tracer, fn_state);
        } else {
            return block_480(state, tracer, fn_state);
        };
    }
    fn block_480<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_480_0: jump b481
        return block_481(state, tracer, fn_state);
    }
    fn block_481<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_481_0: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_482<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_482_0: const #19136u : u32
        let s_482_0: u32 = 19136;
        // D s_482_1: read-reg s_482_0:struct
        let s_482_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_482_0 as isize);
            tracer.read_register(s_482_0 as isize, value);
            value
        };
        // D s_482_2: call _get_PMSELR_EL0_Type_SEL(s_482_1)
        let s_482_2: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_482_1);
        // D s_482_3: cast zx s_482_2 -> bv
        let s_482_3: Bits = Bits::new(s_482_2 as u128, 5u16);
        // D s_482_4: cast zx s_482_3 -> i
        let s_482_4: i128 = (s_482_3.value() as i128);
        // D s_482_5: cast reint s_482_4 -> i64
        let s_482_5: i64 = (s_482_4 as i64);
        // C s_482_6: const #() : ()
        let s_482_6: () = ();
        // S s_482_7: call GetNumEventCounters(s_482_6)
        let s_482_7: i128 = GetNumEventCounters(state, tracer, s_482_6);
        // C s_482_8: const #1s : i
        let s_482_8: i128 = 1;
        // S s_482_9: sub s_482_7 s_482_8
        let s_482_9: i128 = ((s_482_7) - (s_482_8));
        // D s_482_10: cast zx s_482_5 -> i
        let s_482_10: i128 = (i128::try_from(s_482_5).unwrap());
        // D s_482_11: cmp-gt s_482_10 s_482_9
        let s_482_11: bool = ((s_482_10) > (s_482_9));
        // N s_482_12: branch s_482_11 b501 b483
        if s_482_11 {
            return block_501(state, tracer, fn_state);
        } else {
            return block_483(state, tracer, fn_state);
        };
    }
    fn block_483<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_483_0: const #0u : u8
        let s_483_0: bool = false;
        // D s_483_1: write-var gs#140990 <= s_483_0
        fn_state.gs_140990 = s_483_0;
        // N s_483_2: jump b484
        return block_484(state, tracer, fn_state);
    }
    fn block_484<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_484_0: read-var gs#140990:u8
        let s_484_0: bool = fn_state.gs_140990;
        // N s_484_1: branch s_484_0 b500 b485
        if s_484_0 {
            return block_500(state, tracer, fn_state);
        } else {
            return block_485(state, tracer, fn_state);
        };
    }
    fn block_485<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_485_0: const #() : ()
        let s_485_0: () = ();
        // S s_485_1: call EL2Enabled(s_485_0)
        let s_485_1: bool = EL2Enabled(state, tracer, s_485_0);
        // N s_485_2: branch s_485_1 b496 b486
        if s_485_1 {
            return block_496(state, tracer, fn_state);
        } else {
            return block_486(state, tracer, fn_state);
        };
    }
    fn block_486<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_486_0: const #0u : u8
        let s_486_0: bool = false;
        // D s_486_1: write-var gs#140992 <= s_486_0
        fn_state.gs_140992 = s_486_0;
        // N s_486_2: jump b487
        return block_487(state, tracer, fn_state);
    }
    fn block_487<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_487_0: read-var gs#140992:u8
        let s_487_0: bool = fn_state.gs_140992;
        // N s_487_1: branch s_487_0 b495 b488
        if s_487_0 {
            return block_495(state, tracer, fn_state);
        } else {
            return block_488(state, tracer, fn_state);
        };
    }
    fn block_488<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_488_0: const #0u : u8
        let s_488_0: bool = false;
        // D s_488_1: write-var gs#140994 <= s_488_0
        fn_state.gs_140994 = s_488_0;
        // N s_488_2: jump b489
        return block_489(state, tracer, fn_state);
    }
    fn block_489<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_489_0: read-var gs#140994:u8
        let s_489_0: bool = fn_state.gs_140994;
        // D s_489_1: write-var gs#140995 <= s_489_0
        fn_state.gs_140995 = s_489_0;
        // N s_489_2: jump b490
        return block_490(state, tracer, fn_state);
    }
    fn block_490<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_490_0: read-var gs#140995:u8
        let s_490_0: bool = fn_state.gs_140995;
        // N s_490_1: branch s_490_0 b492 b491
        if s_490_0 {
            return block_492(state, tracer, fn_state);
        } else {
            return block_491(state, tracer, fn_state);
        };
    }
    fn block_491<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_491_0: const #19136u : u32
        let s_491_0: u32 = 19136;
        // D s_491_1: read-reg s_491_0:struct
        let s_491_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_491_0 as isize);
            tracer.read_register(s_491_0 as isize, value);
            value
        };
        // D s_491_2: call _get_PMSELR_EL0_Type_SEL(s_491_1)
        let s_491_2: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_491_1);
        // D s_491_3: cast zx s_491_2 -> bv
        let s_491_3: Bits = Bits::new(s_491_2 as u128, 5u16);
        // D s_491_4: cast zx s_491_3 -> i
        let s_491_4: i128 = (s_491_3.value() as i128);
        // D s_491_5: cast reint s_491_4 -> i64
        let s_491_5: i64 = (s_491_4 as i64);
        // C s_491_6: const #31s : i
        let s_491_6: i128 = 31;
        // D s_491_7: cast zx s_491_5 -> i
        let s_491_7: i128 = (i128::try_from(s_491_5).unwrap());
        // D s_491_8: cmp-lt s_491_7 s_491_6
        let s_491_8: bool = ((s_491_7) < (s_491_6));
        // N s_491_9: assert s_491_8
        let s_491_9: () = assert!(s_491_8);
        // C s_491_10: const #64s : i64
        let s_491_10: i64 = 64;
        // C s_491_11: const #10744u : u32
        let s_491_11: u32 = 10744;
        // D s_491_12: read-reg s_491_11:[u64; 32]
        let s_491_12: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_491_11 as isize);
            tracer.read_register(s_491_11 as isize, value);
            value
        };
        // D s_491_13: cast zx s_491_5 -> i
        let s_491_13: i128 = (i128::try_from(s_491_5).unwrap());
        // D s_491_14: read-element s_491_12[s_491_13]
        let s_491_14: u64 = s_491_12[(s_491_13) as usize];
        // D s_491_15: cast zx s_491_14 -> bv
        let s_491_15: Bits = Bits::new(s_491_14 as u128, 64u16);
        // D s_491_16: read-var t:i
        let s_491_16: i128 = fn_state.t;
        // D s_491_17: call X_set(s_491_16, s_491_10, s_491_15)
        let s_491_17: () = X_set(state, tracer, s_491_16, s_491_10, s_491_15);
        // N s_491_18: jump b481
        return block_481(state, tracer, fn_state);
    }
    fn block_492<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_492_0: const #19136u : u32
        let s_492_0: u32 = 19136;
        // D s_492_1: read-reg s_492_0:struct
        let s_492_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_492_0 as isize);
            tracer.read_register(s_492_0 as isize, value);
            value
        };
        // D s_492_2: call _get_PMSELR_EL0_Type_SEL(s_492_1)
        let s_492_2: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_492_1);
        // D s_492_3: cast zx s_492_2 -> bv
        let s_492_3: Bits = Bits::new(s_492_2 as u128, 5u16);
        // D s_492_4: cast zx s_492_3 -> i
        let s_492_4: i128 = (s_492_3.value() as i128);
        // D s_492_5: cast reint s_492_4 -> i64
        let s_492_5: i64 = (s_492_4 as i64);
        // C s_492_6: const #() : ()
        let s_492_6: () = ();
        // S s_492_7: call GetNumEventCounters(s_492_6)
        let s_492_7: i128 = GetNumEventCounters(state, tracer, s_492_6);
        // C s_492_8: const #1s : i
        let s_492_8: i128 = 1;
        // S s_492_9: sub s_492_7 s_492_8
        let s_492_9: i128 = ((s_492_7) - (s_492_8));
        // D s_492_10: cast zx s_492_5 -> i
        let s_492_10: i128 = (i128::try_from(s_492_5).unwrap());
        // D s_492_11: cmp-gt s_492_10 s_492_9
        let s_492_11: bool = ((s_492_10) > (s_492_9));
        // N s_492_12: branch s_492_11 b494 b493
        if s_492_11 {
            return block_494(state, tracer, fn_state);
        } else {
            return block_493(state, tracer, fn_state);
        };
    }
    fn block_493<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_493_0: const #64s : i64
        let s_493_0: i64 = 64;
        // D s_493_1: read-var temp:u64
        let s_493_1: u64 = fn_state.temp;
        // D s_493_2: cast zx s_493_1 -> bv
        let s_493_2: Bits = Bits::new(s_493_1 as u128, 64u16);
        // D s_493_3: read-var t:i
        let s_493_3: i128 = fn_state.t;
        // D s_493_4: call X_set(s_493_3, s_493_0, s_493_2)
        let s_493_4: () = X_set(state, tracer, s_493_3, s_493_0, s_493_2);
        // C s_493_5: const #24u : u8
        let s_493_5: u8 = 24;
        // C s_493_6: cast zx s_493_5 -> bv
        let s_493_6: Bits = Bits::new(s_493_5 as u128, 8u16);
        // C s_493_7: cast zx s_493_6 -> i
        let s_493_7: i128 = (s_493_6.value() as i128);
        // C s_493_8: cast reint s_493_7 -> i64
        let s_493_8: i64 = (s_493_7 as i64);
        // C s_493_9: cast zx s_493_8 -> i
        let s_493_9: i128 = (i128::try_from(s_493_8).unwrap());
        // C s_493_10: const #432u : u32
        let s_493_10: u32 = 432;
        // D s_493_11: read-reg s_493_10:u8
        let s_493_11: u8 = {
            let value = state.read_register::<u8>(s_493_10 as isize);
            tracer.read_register(s_493_10 as isize, value);
            value
        };
        // D s_493_12: call AArch64_SystemAccessTrap(s_493_11, s_493_9)
        let s_493_12: () = AArch64_SystemAccessTrap(state, tracer, s_493_11, s_493_9);
        // N s_493_13: jump b481
        return block_481(state, tracer, fn_state);
    }
    fn block_494<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_494_0: const #64s : i64
        let s_494_0: i64 = 64;
        // D s_494_1: read-var temp:u64
        let s_494_1: u64 = fn_state.temp;
        // D s_494_2: cast zx s_494_1 -> bv
        let s_494_2: Bits = Bits::new(s_494_1 as u128, 64u16);
        // D s_494_3: read-var t:i
        let s_494_3: i128 = fn_state.t;
        // D s_494_4: call X_set(s_494_3, s_494_0, s_494_2)
        let s_494_4: () = X_set(state, tracer, s_494_3, s_494_0, s_494_2);
        // N s_494_5: panic
        panic!("{:?}", ());
        // N s_494_6: return
        return;
    }
    fn block_495<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_495_0: const #19136u : u32
        let s_495_0: u32 = 19136;
        // D s_495_1: read-reg s_495_0:struct
        let s_495_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_495_0 as isize);
            tracer.read_register(s_495_0 as isize, value);
            value
        };
        // D s_495_2: call _get_PMSELR_EL0_Type_SEL(s_495_1)
        let s_495_2: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_495_1);
        // D s_495_3: cast zx s_495_2 -> bv
        let s_495_3: Bits = Bits::new(s_495_2 as u128, 5u16);
        // D s_495_4: cast zx s_495_3 -> i
        let s_495_4: i128 = (s_495_3.value() as i128);
        // D s_495_5: cast reint s_495_4 -> i64
        let s_495_5: i64 = (s_495_4 as i64);
        // C s_495_6: const #() : ()
        let s_495_6: () = ();
        // S s_495_7: call AArch64_GetNumEventCountersAccessible(s_495_6)
        let s_495_7: i128 = AArch64_GetNumEventCountersAccessible(
            state,
            tracer,
            s_495_6,
        );
        // C s_495_8: const #1s : i
        let s_495_8: i128 = 1;
        // S s_495_9: sub s_495_7 s_495_8
        let s_495_9: i128 = ((s_495_7) - (s_495_8));
        // D s_495_10: cast zx s_495_5 -> i
        let s_495_10: i128 = (i128::try_from(s_495_5).unwrap());
        // D s_495_11: cmp-gt s_495_10 s_495_9
        let s_495_11: bool = ((s_495_10) > (s_495_9));
        // D s_495_12: write-var gs#140994 <= s_495_11
        fn_state.gs_140994 = s_495_11;
        // N s_495_13: jump b489
        return block_489(state, tracer, fn_state);
    }
    fn block_496<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_496_0: const #16975u : u32
        let s_496_0: u32 = 16975;
        // D s_496_1: read-reg s_496_0:u8
        let s_496_1: u8 = {
            let value = state.read_register::<u8>(s_496_0 as isize);
            tracer.read_register(s_496_0 as isize, value);
            value
        };
        // D s_496_2: cast zx s_496_1 -> bv
        let s_496_2: Bits = Bits::new(s_496_1 as u128, 2u16);
        // C s_496_3: const #448u : u32
        let s_496_3: u32 = 448;
        // D s_496_4: read-reg s_496_3:u8
        let s_496_4: u8 = {
            let value = state.read_register::<u8>(s_496_3 as isize);
            tracer.read_register(s_496_3 as isize, value);
            value
        };
        // D s_496_5: cast zx s_496_4 -> bv
        let s_496_5: Bits = Bits::new(s_496_4 as u128, 2u16);
        // D s_496_6: cmp-eq s_496_2 s_496_5
        let s_496_6: bool = ((s_496_2) == (s_496_5));
        // N s_496_7: branch s_496_6 b499 b497
        if s_496_6 {
            return block_499(state, tracer, fn_state);
        } else {
            return block_497(state, tracer, fn_state);
        };
    }
    fn block_497<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_497_0: const #16975u : u32
        let s_497_0: u32 = 16975;
        // D s_497_1: read-reg s_497_0:u8
        let s_497_1: u8 = {
            let value = state.read_register::<u8>(s_497_0 as isize);
            tracer.read_register(s_497_0 as isize, value);
            value
        };
        // D s_497_2: cast zx s_497_1 -> bv
        let s_497_2: Bits = Bits::new(s_497_1 as u128, 2u16);
        // C s_497_3: const #440u : u32
        let s_497_3: u32 = 440;
        // D s_497_4: read-reg s_497_3:u8
        let s_497_4: u8 = {
            let value = state.read_register::<u8>(s_497_3 as isize);
            tracer.read_register(s_497_3 as isize, value);
            value
        };
        // D s_497_5: cast zx s_497_4 -> bv
        let s_497_5: Bits = Bits::new(s_497_4 as u128, 2u16);
        // D s_497_6: cmp-eq s_497_2 s_497_5
        let s_497_6: bool = ((s_497_2) == (s_497_5));
        // D s_497_7: write-var gs#140991 <= s_497_6
        fn_state.gs_140991 = s_497_6;
        // N s_497_8: jump b498
        return block_498(state, tracer, fn_state);
    }
    fn block_498<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_498_0: read-var gs#140991:u8
        let s_498_0: bool = fn_state.gs_140991;
        // D s_498_1: write-var gs#140992 <= s_498_0
        fn_state.gs_140992 = s_498_0;
        // N s_498_2: jump b487
        return block_487(state, tracer, fn_state);
    }
    fn block_499<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_499_0: const #1u : u8
        let s_499_0: bool = true;
        // D s_499_1: write-var gs#140991 <= s_499_0
        fn_state.gs_140991 = s_499_0;
        // N s_499_2: jump b498
        return block_498(state, tracer, fn_state);
    }
    fn block_500<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_500_0: const #1u : u8
        let s_500_0: bool = true;
        // D s_500_1: write-var gs#140995 <= s_500_0
        fn_state.gs_140995 = s_500_0;
        // N s_500_2: jump b490
        return block_490(state, tracer, fn_state);
    }
    fn block_501<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_501_0: const #19136u : u32
        let s_501_0: u32 = 19136;
        // D s_501_1: read-reg s_501_0:struct
        let s_501_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_501_0 as isize);
            tracer.read_register(s_501_0 as isize, value);
            value
        };
        // D s_501_2: call _get_PMSELR_EL0_Type_SEL(s_501_1)
        let s_501_2: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_501_1);
        // D s_501_3: cast zx s_501_2 -> bv
        let s_501_3: Bits = Bits::new(s_501_2 as u128, 5u16);
        // C s_501_4: const #31u : u8
        let s_501_4: u8 = 31;
        // C s_501_5: cast zx s_501_4 -> bv
        let s_501_5: Bits = Bits::new(s_501_4 as u128, 5u16);
        // D s_501_6: cmp-ne s_501_3 s_501_5
        let s_501_6: bool = ((s_501_3) != (s_501_5));
        // D s_501_7: write-var gs#140990 <= s_501_6
        fn_state.gs_140990 = s_501_6;
        // N s_501_8: jump b484
        return block_484(state, tracer, fn_state);
    }
    fn block_502<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_502_0: const #() : ()
        let s_502_0: () = ();
        // S s_502_1: call EL2Enabled(s_502_0)
        let s_502_1: bool = EL2Enabled(state, tracer, s_502_0);
        // N s_502_2: branch s_502_1 b517 b503
        if s_502_1 {
            return block_517(state, tracer, fn_state);
        } else {
            return block_503(state, tracer, fn_state);
        };
    }
    fn block_503<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_503_0: const #0u : u8
        let s_503_0: bool = false;
        // D s_503_1: write-var gs#141008 <= s_503_0
        fn_state.gs_141008 = s_503_0;
        // N s_503_2: jump b504
        return block_504(state, tracer, fn_state);
    }
    fn block_504<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_504_0: read-var gs#141008:u8
        let s_504_0: bool = fn_state.gs_141008;
        // N s_504_1: branch s_504_0 b516 b505
        if s_504_0 {
            return block_516(state, tracer, fn_state);
        } else {
            return block_505(state, tracer, fn_state);
        };
    }
    fn block_505<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_505_0: const #0u : u8
        let s_505_0: bool = false;
        // D s_505_1: write-var gs#141009 <= s_505_0
        fn_state.gs_141009 = s_505_0;
        // N s_505_2: jump b506
        return block_506(state, tracer, fn_state);
    }
    fn block_506<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_506_0: read-var gs#141009:u8
        let s_506_0: bool = fn_state.gs_141009;
        // N s_506_1: branch s_506_0 b515 b507
        if s_506_0 {
            return block_515(state, tracer, fn_state);
        } else {
            return block_507(state, tracer, fn_state);
        };
    }
    fn block_507<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_507_0: const #0u : u8
        let s_507_0: bool = false;
        // D s_507_1: write-var gs#141011 <= s_507_0
        fn_state.gs_141011 = s_507_0;
        // N s_507_2: jump b508
        return block_508(state, tracer, fn_state);
    }
    fn block_508<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_508_0: read-var gs#141011:u8
        let s_508_0: bool = fn_state.gs_141011;
        // N s_508_1: branch s_508_0 b512 b509
        if s_508_0 {
            return block_512(state, tracer, fn_state);
        } else {
            return block_509(state, tracer, fn_state);
        };
    }
    fn block_509<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_509_0: const #19136u : u32
        let s_509_0: u32 = 19136;
        // D s_509_1: read-reg s_509_0:struct
        let s_509_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_509_0 as isize);
            tracer.read_register(s_509_0 as isize, value);
            value
        };
        // D s_509_2: call _get_PMSELR_EL0_Type_SEL(s_509_1)
        let s_509_2: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_509_1);
        // D s_509_3: cast zx s_509_2 -> bv
        let s_509_3: Bits = Bits::new(s_509_2 as u128, 5u16);
        // C s_509_4: const #31u : u8
        let s_509_4: u8 = 31;
        // C s_509_5: cast zx s_509_4 -> bv
        let s_509_5: Bits = Bits::new(s_509_4 as u128, 5u16);
        // D s_509_6: cmp-eq s_509_3 s_509_5
        let s_509_6: bool = ((s_509_3) == (s_509_5));
        // N s_509_7: branch s_509_6 b511 b510
        if s_509_6 {
            return block_511(state, tracer, fn_state);
        } else {
            return block_510(state, tracer, fn_state);
        };
    }
    fn block_510<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_510_0: const #19136u : u32
        let s_510_0: u32 = 19136;
        // D s_510_1: read-reg s_510_0:struct
        let s_510_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_510_0 as isize);
            tracer.read_register(s_510_0 as isize, value);
            value
        };
        // D s_510_2: call _get_PMSELR_EL0_Type_SEL(s_510_1)
        let s_510_2: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_510_1);
        // D s_510_3: cast zx s_510_2 -> bv
        let s_510_3: Bits = Bits::new(s_510_2 as u128, 5u16);
        // D s_510_4: cast zx s_510_3 -> i
        let s_510_4: i128 = (s_510_3.value() as i128);
        // D s_510_5: cast reint s_510_4 -> i64
        let s_510_5: i64 = (s_510_4 as i64);
        // C s_510_6: const #31s : i
        let s_510_6: i128 = 31;
        // D s_510_7: cast zx s_510_5 -> i
        let s_510_7: i128 = (i128::try_from(s_510_5).unwrap());
        // D s_510_8: cmp-lt s_510_7 s_510_6
        let s_510_8: bool = ((s_510_7) < (s_510_6));
        // N s_510_9: assert s_510_8
        let s_510_9: () = assert!(s_510_8);
        // C s_510_10: const #64s : i64
        let s_510_10: i64 = 64;
        // C s_510_11: const #11208u : u32
        let s_510_11: u32 = 11208;
        // D s_510_12: read-reg s_510_11:[struct; 32]
        let s_510_12: [ProductType5c790c8ef59cc8b2; 32usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 32usize],
                >(s_510_11 as isize);
            tracer.read_register(s_510_11 as isize, value);
            value
        };
        // D s_510_13: cast zx s_510_5 -> i
        let s_510_13: i128 = (i128::try_from(s_510_5).unwrap());
        // D s_510_14: read-element s_510_12[s_510_13]
        let s_510_14: ProductType5c790c8ef59cc8b2 = s_510_12[(s_510_13) as usize];
        // D s_510_15: write-var ga#246950 <= s_510_14
        fn_state.ga_246950 = s_510_14;
        // D s_510_16: read-var ga#246950.0:struct
        let s_510_16: u64 = fn_state.ga_246950._0;
        // D s_510_17: cast zx s_510_16 -> bv
        let s_510_17: Bits = Bits::new(s_510_16 as u128, 64u16);
        // D s_510_18: read-var t:i
        let s_510_18: i128 = fn_state.t;
        // D s_510_19: call X_set(s_510_18, s_510_10, s_510_17)
        let s_510_19: () = X_set(state, tracer, s_510_18, s_510_10, s_510_17);
        // N s_510_20: jump b479
        return block_479(state, tracer, fn_state);
    }
    fn block_511<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_511_0: const #64s : i64
        let s_511_0: i64 = 64;
        // C s_511_1: const #15864u : u32
        let s_511_1: u32 = 15864;
        // D s_511_2: read-reg s_511_1:u64
        let s_511_2: u64 = {
            let value = state.read_register::<u64>(s_511_1 as isize);
            tracer.read_register(s_511_1 as isize, value);
            value
        };
        // D s_511_3: cast zx s_511_2 -> bv
        let s_511_3: Bits = Bits::new(s_511_2 as u128, 64u16);
        // D s_511_4: read-var t:i
        let s_511_4: i128 = fn_state.t;
        // D s_511_5: call X_set(s_511_4, s_511_0, s_511_3)
        let s_511_5: () = X_set(state, tracer, s_511_4, s_511_0, s_511_3);
        // N s_511_6: jump b479
        return block_479(state, tracer, fn_state);
    }
    fn block_512<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_512_0: const #19136u : u32
        let s_512_0: u32 = 19136;
        // D s_512_1: read-reg s_512_0:struct
        let s_512_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_512_0 as isize);
            tracer.read_register(s_512_0 as isize, value);
            value
        };
        // D s_512_2: call _get_PMSELR_EL0_Type_SEL(s_512_1)
        let s_512_2: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_512_1);
        // D s_512_3: cast zx s_512_2 -> bv
        let s_512_3: Bits = Bits::new(s_512_2 as u128, 5u16);
        // D s_512_4: cast zx s_512_3 -> i
        let s_512_4: i128 = (s_512_3.value() as i128);
        // D s_512_5: cast reint s_512_4 -> i64
        let s_512_5: i64 = (s_512_4 as i64);
        // C s_512_6: const #() : ()
        let s_512_6: () = ();
        // S s_512_7: call GetNumEventCounters(s_512_6)
        let s_512_7: i128 = GetNumEventCounters(state, tracer, s_512_6);
        // C s_512_8: const #1s : i
        let s_512_8: i128 = 1;
        // S s_512_9: sub s_512_7 s_512_8
        let s_512_9: i128 = ((s_512_7) - (s_512_8));
        // D s_512_10: cast zx s_512_5 -> i
        let s_512_10: i128 = (i128::try_from(s_512_5).unwrap());
        // D s_512_11: cmp-gt s_512_10 s_512_9
        let s_512_11: bool = ((s_512_10) > (s_512_9));
        // N s_512_12: branch s_512_11 b514 b513
        if s_512_11 {
            return block_514(state, tracer, fn_state);
        } else {
            return block_513(state, tracer, fn_state);
        };
    }
    fn block_513<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_513_0: const #64s : i64
        let s_513_0: i64 = 64;
        // D s_513_1: read-var temp:u64
        let s_513_1: u64 = fn_state.temp;
        // D s_513_2: cast zx s_513_1 -> bv
        let s_513_2: Bits = Bits::new(s_513_1 as u128, 64u16);
        // D s_513_3: read-var t:i
        let s_513_3: i128 = fn_state.t;
        // D s_513_4: call X_set(s_513_3, s_513_0, s_513_2)
        let s_513_4: () = X_set(state, tracer, s_513_3, s_513_0, s_513_2);
        // C s_513_5: const #24u : u8
        let s_513_5: u8 = 24;
        // C s_513_6: cast zx s_513_5 -> bv
        let s_513_6: Bits = Bits::new(s_513_5 as u128, 8u16);
        // C s_513_7: cast zx s_513_6 -> i
        let s_513_7: i128 = (s_513_6.value() as i128);
        // C s_513_8: cast reint s_513_7 -> i64
        let s_513_8: i64 = (s_513_7 as i64);
        // C s_513_9: cast zx s_513_8 -> i
        let s_513_9: i128 = (i128::try_from(s_513_8).unwrap());
        // C s_513_10: const #432u : u32
        let s_513_10: u32 = 432;
        // D s_513_11: read-reg s_513_10:u8
        let s_513_11: u8 = {
            let value = state.read_register::<u8>(s_513_10 as isize);
            tracer.read_register(s_513_10 as isize, value);
            value
        };
        // D s_513_12: call AArch64_SystemAccessTrap(s_513_11, s_513_9)
        let s_513_12: () = AArch64_SystemAccessTrap(state, tracer, s_513_11, s_513_9);
        // N s_513_13: jump b479
        return block_479(state, tracer, fn_state);
    }
    fn block_514<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_514_0: const #64s : i64
        let s_514_0: i64 = 64;
        // D s_514_1: read-var temp:u64
        let s_514_1: u64 = fn_state.temp;
        // D s_514_2: cast zx s_514_1 -> bv
        let s_514_2: Bits = Bits::new(s_514_1 as u128, 64u16);
        // D s_514_3: read-var t:i
        let s_514_3: i128 = fn_state.t;
        // D s_514_4: call X_set(s_514_3, s_514_0, s_514_2)
        let s_514_4: () = X_set(state, tracer, s_514_3, s_514_0, s_514_2);
        // N s_514_5: panic
        panic!("{:?}", ());
        // N s_514_6: return
        return;
    }
    fn block_515<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_515_0: const #19136u : u32
        let s_515_0: u32 = 19136;
        // D s_515_1: read-reg s_515_0:struct
        let s_515_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_515_0 as isize);
            tracer.read_register(s_515_0 as isize, value);
            value
        };
        // D s_515_2: call _get_PMSELR_EL0_Type_SEL(s_515_1)
        let s_515_2: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_515_1);
        // D s_515_3: cast zx s_515_2 -> bv
        let s_515_3: Bits = Bits::new(s_515_2 as u128, 5u16);
        // D s_515_4: cast zx s_515_3 -> i
        let s_515_4: i128 = (s_515_3.value() as i128);
        // D s_515_5: cast reint s_515_4 -> i64
        let s_515_5: i64 = (s_515_4 as i64);
        // C s_515_6: const #() : ()
        let s_515_6: () = ();
        // S s_515_7: call AArch64_GetNumEventCountersAccessible(s_515_6)
        let s_515_7: i128 = AArch64_GetNumEventCountersAccessible(
            state,
            tracer,
            s_515_6,
        );
        // C s_515_8: const #1s : i
        let s_515_8: i128 = 1;
        // S s_515_9: sub s_515_7 s_515_8
        let s_515_9: i128 = ((s_515_7) - (s_515_8));
        // D s_515_10: cast zx s_515_5 -> i
        let s_515_10: i128 = (i128::try_from(s_515_5).unwrap());
        // D s_515_11: cmp-gt s_515_10 s_515_9
        let s_515_11: bool = ((s_515_10) > (s_515_9));
        // D s_515_12: write-var gs#141011 <= s_515_11
        fn_state.gs_141011 = s_515_11;
        // N s_515_13: jump b508
        return block_508(state, tracer, fn_state);
    }
    fn block_516<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_516_0: const #19136u : u32
        let s_516_0: u32 = 19136;
        // D s_516_1: read-reg s_516_0:struct
        let s_516_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_516_0 as isize);
            tracer.read_register(s_516_0 as isize, value);
            value
        };
        // D s_516_2: call _get_PMSELR_EL0_Type_SEL(s_516_1)
        let s_516_2: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_516_1);
        // D s_516_3: cast zx s_516_2 -> bv
        let s_516_3: Bits = Bits::new(s_516_2 as u128, 5u16);
        // C s_516_4: const #31u : u8
        let s_516_4: u8 = 31;
        // C s_516_5: cast zx s_516_4 -> bv
        let s_516_5: Bits = Bits::new(s_516_4 as u128, 5u16);
        // D s_516_6: cmp-ne s_516_3 s_516_5
        let s_516_6: bool = ((s_516_3) != (s_516_5));
        // D s_516_7: write-var gs#141009 <= s_516_6
        fn_state.gs_141009 = s_516_6;
        // N s_516_8: jump b506
        return block_506(state, tracer, fn_state);
    }
    fn block_517<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_517_0: const #16975u : u32
        let s_517_0: u32 = 16975;
        // D s_517_1: read-reg s_517_0:u8
        let s_517_1: u8 = {
            let value = state.read_register::<u8>(s_517_0 as isize);
            tracer.read_register(s_517_0 as isize, value);
            value
        };
        // D s_517_2: cast zx s_517_1 -> bv
        let s_517_2: Bits = Bits::new(s_517_1 as u128, 2u16);
        // C s_517_3: const #448u : u32
        let s_517_3: u32 = 448;
        // D s_517_4: read-reg s_517_3:u8
        let s_517_4: u8 = {
            let value = state.read_register::<u8>(s_517_3 as isize);
            tracer.read_register(s_517_3 as isize, value);
            value
        };
        // D s_517_5: cast zx s_517_4 -> bv
        let s_517_5: Bits = Bits::new(s_517_4 as u128, 2u16);
        // D s_517_6: cmp-eq s_517_2 s_517_5
        let s_517_6: bool = ((s_517_2) == (s_517_5));
        // N s_517_7: branch s_517_6 b520 b518
        if s_517_6 {
            return block_520(state, tracer, fn_state);
        } else {
            return block_518(state, tracer, fn_state);
        };
    }
    fn block_518<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_518_0: const #16975u : u32
        let s_518_0: u32 = 16975;
        // D s_518_1: read-reg s_518_0:u8
        let s_518_1: u8 = {
            let value = state.read_register::<u8>(s_518_0 as isize);
            tracer.read_register(s_518_0 as isize, value);
            value
        };
        // D s_518_2: cast zx s_518_1 -> bv
        let s_518_2: Bits = Bits::new(s_518_1 as u128, 2u16);
        // C s_518_3: const #440u : u32
        let s_518_3: u32 = 440;
        // D s_518_4: read-reg s_518_3:u8
        let s_518_4: u8 = {
            let value = state.read_register::<u8>(s_518_3 as isize);
            tracer.read_register(s_518_3 as isize, value);
            value
        };
        // D s_518_5: cast zx s_518_4 -> bv
        let s_518_5: Bits = Bits::new(s_518_4 as u128, 2u16);
        // D s_518_6: cmp-eq s_518_2 s_518_5
        let s_518_6: bool = ((s_518_2) == (s_518_5));
        // D s_518_7: write-var gs#141007 <= s_518_6
        fn_state.gs_141007 = s_518_6;
        // N s_518_8: jump b519
        return block_519(state, tracer, fn_state);
    }
    fn block_519<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_519_0: read-var gs#141007:u8
        let s_519_0: bool = fn_state.gs_141007;
        // D s_519_1: write-var gs#141008 <= s_519_0
        fn_state.gs_141008 = s_519_0;
        // N s_519_2: jump b504
        return block_504(state, tracer, fn_state);
    }
    fn block_520<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_520_0: const #1u : u8
        let s_520_0: bool = true;
        // D s_520_1: write-var gs#141007 <= s_520_0
        fn_state.gs_141007 = s_520_0;
        // N s_520_2: jump b519
        return block_519(state, tracer, fn_state);
    }
    fn block_521<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_521_0: const #1s : i
        let s_521_0: i128 = 1;
        // D s_521_1: read-var op2:i
        let s_521_1: i128 = fn_state.op2;
        // D s_521_2: cmp-eq s_521_1 s_521_0
        let s_521_2: bool = ((s_521_1) == (s_521_0));
        // N s_521_3: branch s_521_2 b524 b522
        if s_521_2 {
            return block_524(state, tracer, fn_state);
        } else {
            return block_522(state, tracer, fn_state);
        };
    }
    fn block_522<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_522_0: const #2s : i
        let s_522_0: i128 = 2;
        // D s_522_1: read-var op2:i
        let s_522_1: i128 = fn_state.op2;
        // D s_522_2: cmp-eq s_522_1 s_522_0
        let s_522_2: bool = ((s_522_1) == (s_522_0));
        // D s_522_3: write-var gs#140558 <= s_522_2
        fn_state.gs_140558 = s_522_2;
        // N s_522_4: jump b523
        return block_523(state, tracer, fn_state);
    }
    fn block_523<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_523_0: read-var gs#140558:u8
        let s_523_0: bool = fn_state.gs_140558;
        // D s_523_1: write-var gs#140559 <= s_523_0
        fn_state.gs_140559 = s_523_0;
        // N s_523_2: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_524<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_524_0: const #1u : u8
        let s_524_0: bool = true;
        // D s_524_1: write-var gs#140558 <= s_524_0
        fn_state.gs_140558 = s_524_0;
        // N s_524_2: jump b523
        return block_523(state, tracer, fn_state);
    }
    fn block_525<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_525_0: const #13s : i
        let s_525_0: i128 = 13;
        // D s_525_1: read-var crm:i
        let s_525_1: i128 = fn_state.crm;
        // D s_525_2: cmp-eq s_525_1 s_525_0
        let s_525_2: bool = ((s_525_1) == (s_525_0));
        // D s_525_3: write-var gs#140555 <= s_525_2
        fn_state.gs_140555 = s_525_2;
        // N s_525_4: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_526<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_526_0: const #9s : i
        let s_526_0: i128 = 9;
        // D s_526_1: read-var crn:i
        let s_526_1: i128 = fn_state.crn;
        // D s_526_2: cmp-eq s_526_1 s_526_0
        let s_526_2: bool = ((s_526_1) == (s_526_0));
        // D s_526_3: write-var gs#140553 <= s_526_2
        fn_state.gs_140553 = s_526_2;
        // N s_526_4: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_527<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_527_0: const #3s : i
        let s_527_0: i128 = 3;
        // D s_527_1: read-var op1:i
        let s_527_1: i128 = fn_state.op1;
        // D s_527_2: cmp-eq s_527_1 s_527_0
        let s_527_2: bool = ((s_527_1) == (s_527_0));
        // D s_527_3: write-var gs#140551 <= s_527_2
        fn_state.gs_140551 = s_527_2;
        // N s_527_4: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_528<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_528_0: const #() : ()
        let s_528_0: () = ();
        // S s_528_1: call PMUCounterMask(s_528_0)
        let s_528_1: u64 = PMUCounterMask(state, tracer, s_528_0);
        // C s_528_2: const #64s : i64
        let s_528_2: i64 = 64;
        // C s_528_3: const #64s : i64
        let s_528_3: i64 = 64;
        // D s_528_4: read-var t:i
        let s_528_4: i128 = fn_state.t;
        // D s_528_5: call X_read(s_528_4, s_528_3)
        let s_528_5: Bits = X_read(state, tracer, s_528_4, s_528_3);
        // D s_528_6: cast reint s_528_5 -> u64
        let s_528_6: u64 = (s_528_5.value() as u64);
        // D s_528_7: cast zx s_528_6 -> bv
        let s_528_7: Bits = Bits::new(s_528_6 as u128, 64u16);
        // S s_528_8: cast zx s_528_1 -> bv
        let s_528_8: Bits = Bits::new(s_528_1 as u128, 64u16);
        // D s_528_9: and s_528_7 s_528_8
        let s_528_9: Bits = ((s_528_7) & (s_528_8));
        // D s_528_10: cast reint s_528_9 -> u64
        let s_528_10: u64 = (s_528_9.value() as u64);
        // D s_528_11: cast zx s_528_10 -> bv
        let s_528_11: Bits = Bits::new(s_528_10 as u128, 64u16);
        // D s_528_12: read-var t:i
        let s_528_12: i128 = fn_state.t;
        // D s_528_13: call X_set(s_528_12, s_528_2, s_528_11)
        let s_528_13: () = X_set(state, tracer, s_528_12, s_528_2, s_528_11);
        // N s_528_14: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_529<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_529_0: const #3s : i
        let s_529_0: i128 = 3;
        // D s_529_1: read-var op1:i
        let s_529_1: i128 = fn_state.op1;
        // D s_529_2: cmp-eq s_529_1 s_529_0
        let s_529_2: bool = ((s_529_1) == (s_529_0));
        // N s_529_3: branch s_529_2 b562 b530
        if s_529_2 {
            return block_562(state, tracer, fn_state);
        } else {
            return block_530(state, tracer, fn_state);
        };
    }
    fn block_530<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_530_0: const #0u : u8
        let s_530_0: bool = false;
        // D s_530_1: write-var gs#140527 <= s_530_0
        fn_state.gs_140527 = s_530_0;
        // N s_530_2: jump b531
        return block_531(state, tracer, fn_state);
    }
    fn block_531<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_531_0: read-var gs#140527:u8
        let s_531_0: bool = fn_state.gs_140527;
        // N s_531_1: branch s_531_0 b558 b532
        if s_531_0 {
            return block_558(state, tracer, fn_state);
        } else {
            return block_532(state, tracer, fn_state);
        };
    }
    fn block_532<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_532_0: const #0u : u8
        let s_532_0: bool = false;
        // D s_532_1: write-var gs#140531 <= s_532_0
        fn_state.gs_140531 = s_532_0;
        // N s_532_2: jump b533
        return block_533(state, tracer, fn_state);
    }
    fn block_533<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_533_0: read-var gs#140531:u8
        let s_533_0: bool = fn_state.gs_140531;
        // N s_533_1: branch s_533_0 b557 b534
        if s_533_0 {
            return block_557(state, tracer, fn_state);
        } else {
            return block_534(state, tracer, fn_state);
        };
    }
    fn block_534<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_534_0: const #3s : i
        let s_534_0: i128 = 3;
        // D s_534_1: read-var op1:i
        let s_534_1: i128 = fn_state.op1;
        // D s_534_2: cmp-eq s_534_1 s_534_0
        let s_534_2: bool = ((s_534_1) == (s_534_0));
        // N s_534_3: branch s_534_2 b553 b535
        if s_534_2 {
            return block_553(state, tracer, fn_state);
        } else {
            return block_535(state, tracer, fn_state);
        };
    }
    fn block_535<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_535_0: const #0u : u8
        let s_535_0: bool = false;
        // D s_535_1: write-var gs#140536 <= s_535_0
        fn_state.gs_140536 = s_535_0;
        // N s_535_2: jump b536
        return block_536(state, tracer, fn_state);
    }
    fn block_536<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_536_0: read-var gs#140536:u8
        let s_536_0: bool = fn_state.gs_140536;
        // N s_536_1: branch s_536_0 b552 b537
        if s_536_0 {
            return block_552(state, tracer, fn_state);
        } else {
            return block_537(state, tracer, fn_state);
        };
    }
    fn block_537<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_537_0: const #0u : u8
        let s_537_0: bool = false;
        // D s_537_1: write-var gs#140538 <= s_537_0
        fn_state.gs_140538 = s_537_0;
        // N s_537_2: jump b538
        return block_538(state, tracer, fn_state);
    }
    fn block_538<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_538_0: read-var gs#140538:u8
        let s_538_0: bool = fn_state.gs_140538;
        // D s_538_1: write-var gs#140539 <= s_538_0
        fn_state.gs_140539 = s_538_0;
        // N s_538_2: jump b539
        return block_539(state, tracer, fn_state);
    }
    fn block_539<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_539_0: read-var gs#140539:u8
        let s_539_0: bool = fn_state.gs_140539;
        // N s_539_1: branch s_539_0 b551 b540
        if s_539_0 {
            return block_551(state, tracer, fn_state);
        } else {
            return block_540(state, tracer, fn_state);
        };
    }
    fn block_540<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_540_0: const #0s : i
        let s_540_0: i128 = 0;
        // D s_540_1: read-var op1:i
        let s_540_1: i128 = fn_state.op1;
        // D s_540_2: cmp-eq s_540_1 s_540_0
        let s_540_2: bool = ((s_540_1) == (s_540_0));
        // N s_540_3: branch s_540_2 b547 b541
        if s_540_2 {
            return block_547(state, tracer, fn_state);
        } else {
            return block_541(state, tracer, fn_state);
        };
    }
    fn block_541<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_541_0: const #0u : u8
        let s_541_0: bool = false;
        // D s_541_1: write-var gs#140544 <= s_541_0
        fn_state.gs_140544 = s_541_0;
        // N s_541_2: jump b542
        return block_542(state, tracer, fn_state);
    }
    fn block_542<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_542_0: read-var gs#140544:u8
        let s_542_0: bool = fn_state.gs_140544;
        // N s_542_1: branch s_542_0 b546 b543
        if s_542_0 {
            return block_546(state, tracer, fn_state);
        } else {
            return block_543(state, tracer, fn_state);
        };
    }
    fn block_543<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_543_0: const #0u : u8
        let s_543_0: bool = false;
        // D s_543_1: write-var gs#140546 <= s_543_0
        fn_state.gs_140546 = s_543_0;
        // N s_543_2: jump b544
        return block_544(state, tracer, fn_state);
    }
    fn block_544<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_544_0: read-var gs#140546:u8
        let s_544_0: bool = fn_state.gs_140546;
        // D s_544_1: write-var gs#140547 <= s_544_0
        fn_state.gs_140547 = s_544_0;
        // N s_544_2: jump b545
        return block_545(state, tracer, fn_state);
    }
    fn block_545<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_545_0: read-var gs#140547:u8
        let s_545_0: bool = fn_state.gs_140547;
        // D s_545_1: write-var gs#140548 <= s_545_0
        fn_state.gs_140548 = s_545_0;
        // N s_545_2: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_546<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_546_0: const #14s : i
        let s_546_0: i128 = 14;
        // D s_546_1: read-var crm:i
        let s_546_1: i128 = fn_state.crm;
        // D s_546_2: cmp-eq s_546_1 s_546_0
        let s_546_2: bool = ((s_546_1) == (s_546_0));
        // D s_546_3: write-var gs#140546 <= s_546_2
        fn_state.gs_140546 = s_546_2;
        // N s_546_4: jump b544
        return block_544(state, tracer, fn_state);
    }
    fn block_547<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_547_0: const #1s : i
        let s_547_0: i128 = 1;
        // D s_547_1: read-var op2:i
        let s_547_1: i128 = fn_state.op2;
        // D s_547_2: cmp-eq s_547_1 s_547_0
        let s_547_2: bool = ((s_547_1) == (s_547_0));
        // N s_547_3: branch s_547_2 b550 b548
        if s_547_2 {
            return block_550(state, tracer, fn_state);
        } else {
            return block_548(state, tracer, fn_state);
        };
    }
    fn block_548<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_548_0: const #2s : i
        let s_548_0: i128 = 2;
        // D s_548_1: read-var op2:i
        let s_548_1: i128 = fn_state.op2;
        // D s_548_2: cmp-eq s_548_1 s_548_0
        let s_548_2: bool = ((s_548_1) == (s_548_0));
        // D s_548_3: write-var gs#140543 <= s_548_2
        fn_state.gs_140543 = s_548_2;
        // N s_548_4: jump b549
        return block_549(state, tracer, fn_state);
    }
    fn block_549<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_549_0: read-var gs#140543:u8
        let s_549_0: bool = fn_state.gs_140543;
        // D s_549_1: write-var gs#140544 <= s_549_0
        fn_state.gs_140544 = s_549_0;
        // N s_549_2: jump b542
        return block_542(state, tracer, fn_state);
    }
    fn block_550<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_550_0: const #1u : u8
        let s_550_0: bool = true;
        // D s_550_1: write-var gs#140543 <= s_550_0
        fn_state.gs_140543 = s_550_0;
        // N s_550_2: jump b549
        return block_549(state, tracer, fn_state);
    }
    fn block_551<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_551_0: const #1u : u8
        let s_551_0: bool = true;
        // D s_551_1: write-var gs#140547 <= s_551_0
        fn_state.gs_140547 = s_551_0;
        // N s_551_2: jump b545
        return block_545(state, tracer, fn_state);
    }
    fn block_552<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_552_0: const #12s : i
        let s_552_0: i128 = 12;
        // D s_552_1: read-var crm:i
        let s_552_1: i128 = fn_state.crm;
        // D s_552_2: cmp-eq s_552_1 s_552_0
        let s_552_2: bool = ((s_552_1) == (s_552_0));
        // D s_552_3: write-var gs#140538 <= s_552_2
        fn_state.gs_140538 = s_552_2;
        // N s_552_4: jump b538
        return block_538(state, tracer, fn_state);
    }
    fn block_553<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_553_0: const #1s : i
        let s_553_0: i128 = 1;
        // D s_553_1: read-var op2:i
        let s_553_1: i128 = fn_state.op2;
        // D s_553_2: cmp-eq s_553_1 s_553_0
        let s_553_2: bool = ((s_553_1) == (s_553_0));
        // N s_553_3: branch s_553_2 b556 b554
        if s_553_2 {
            return block_556(state, tracer, fn_state);
        } else {
            return block_554(state, tracer, fn_state);
        };
    }
    fn block_554<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_554_0: const #2s : i
        let s_554_0: i128 = 2;
        // D s_554_1: read-var op2:i
        let s_554_1: i128 = fn_state.op2;
        // D s_554_2: cmp-eq s_554_1 s_554_0
        let s_554_2: bool = ((s_554_1) == (s_554_0));
        // D s_554_3: write-var gs#140535 <= s_554_2
        fn_state.gs_140535 = s_554_2;
        // N s_554_4: jump b555
        return block_555(state, tracer, fn_state);
    }
    fn block_555<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_555_0: read-var gs#140535:u8
        let s_555_0: bool = fn_state.gs_140535;
        // D s_555_1: write-var gs#140536 <= s_555_0
        fn_state.gs_140536 = s_555_0;
        // N s_555_2: jump b536
        return block_536(state, tracer, fn_state);
    }
    fn block_556<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_556_0: const #1u : u8
        let s_556_0: bool = true;
        // D s_556_1: write-var gs#140535 <= s_556_0
        fn_state.gs_140535 = s_556_0;
        // N s_556_2: jump b555
        return block_555(state, tracer, fn_state);
    }
    fn block_557<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_557_0: const #1u : u8
        let s_557_0: bool = true;
        // D s_557_1: write-var gs#140539 <= s_557_0
        fn_state.gs_140539 = s_557_0;
        // N s_557_2: jump b539
        return block_539(state, tracer, fn_state);
    }
    fn block_558<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_558_0: const #12s : i
        let s_558_0: i128 = 12;
        // D s_558_1: read-var crm:i
        let s_558_1: i128 = fn_state.crm;
        // D s_558_2: cmp-eq s_558_1 s_558_0
        let s_558_2: bool = ((s_558_1) == (s_558_0));
        // N s_558_3: branch s_558_2 b561 b559
        if s_558_2 {
            return block_561(state, tracer, fn_state);
        } else {
            return block_559(state, tracer, fn_state);
        };
    }
    fn block_559<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_559_0: const #14s : i
        let s_559_0: i128 = 14;
        // D s_559_1: read-var crm:i
        let s_559_1: i128 = fn_state.crm;
        // D s_559_2: cmp-eq s_559_1 s_559_0
        let s_559_2: bool = ((s_559_1) == (s_559_0));
        // D s_559_3: write-var gs#140530 <= s_559_2
        fn_state.gs_140530 = s_559_2;
        // N s_559_4: jump b560
        return block_560(state, tracer, fn_state);
    }
    fn block_560<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_560_0: read-var gs#140530:u8
        let s_560_0: bool = fn_state.gs_140530;
        // D s_560_1: write-var gs#140531 <= s_560_0
        fn_state.gs_140531 = s_560_0;
        // N s_560_2: jump b533
        return block_533(state, tracer, fn_state);
    }
    fn block_561<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_561_0: const #1u : u8
        let s_561_0: bool = true;
        // D s_561_1: write-var gs#140530 <= s_561_0
        fn_state.gs_140530 = s_561_0;
        // N s_561_2: jump b560
        return block_560(state, tracer, fn_state);
    }
    fn block_562<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_562_0: const #3s : i
        let s_562_0: i128 = 3;
        // D s_562_1: read-var op2:i
        let s_562_1: i128 = fn_state.op2;
        // D s_562_2: cmp-eq s_562_1 s_562_0
        let s_562_2: bool = ((s_562_1) == (s_562_0));
        // D s_562_3: write-var gs#140527 <= s_562_2
        fn_state.gs_140527 = s_562_2;
        // N s_562_4: jump b531
        return block_531(state, tracer, fn_state);
    }
    fn block_563<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_563_0: const #9s : i
        let s_563_0: i128 = 9;
        // D s_563_1: read-var crn:i
        let s_563_1: i128 = fn_state.crn;
        // D s_563_2: cmp-eq s_563_1 s_563_0
        let s_563_2: bool = ((s_563_1) == (s_563_0));
        // D s_563_3: write-var gs#140524 <= s_563_2
        fn_state.gs_140524 = s_563_2;
        // N s_563_4: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_564<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_564_0: const #64s : i64
        let s_564_0: i64 = 64;
        // C s_564_1: const #64s : i64
        let s_564_1: i64 = 64;
        // D s_564_2: read-var t:i
        let s_564_2: i128 = fn_state.t;
        // D s_564_3: call X_read(s_564_2, s_564_1)
        let s_564_3: Bits = X_read(state, tracer, s_564_2, s_564_1);
        // D s_564_4: cast reint s_564_3 -> u64
        let s_564_4: u64 = (s_564_3.value() as u64);
        // C s_564_5: const #() : ()
        let s_564_5: () = ();
        // S s_564_6: call SPE_PMBIDR_P_Read(s_564_5)
        let s_564_6: bool = SPE_PMBIDR_P_Read(state, tracer, s_564_5);
        // C s_564_7: const #4s : i
        let s_564_7: i128 = 4;
        // D s_564_8: cast zx s_564_4 -> bv
        let s_564_8: Bits = Bits::new(s_564_4 as u128, 64u16);
        // S s_564_9: cast zx s_564_6 -> bv
        let s_564_9: Bits = Bits::new(s_564_6 as u128, 1u16);
        // C s_564_10: const #0s : i
        let s_564_10: i128 = 0;
        // C s_564_11: const #1u : u64
        let s_564_11: u64 = 1;
        // C s_564_12: cast zx s_564_11 -> bv
        let s_564_12: Bits = Bits::new(s_564_11 as u128, 64u16);
        // C s_564_13: lsl s_564_12 s_564_10
        let s_564_13: Bits = s_564_12 << s_564_10;
        // C s_564_14: sub s_564_13 s_564_12
        let s_564_14: Bits = ((s_564_13) - (s_564_12));
        // S s_564_15: and s_564_9 s_564_14
        let s_564_15: Bits = ((s_564_9) & (s_564_14));
        // S s_564_16: lsl s_564_15 s_564_7
        let s_564_16: Bits = s_564_15 << s_564_7;
        // C s_564_17: lsl s_564_14 s_564_7
        let s_564_17: Bits = s_564_14 << s_564_7;
        // C s_564_18: cmpl s_564_17
        let s_564_18: Bits = !s_564_17;
        // D s_564_19: and s_564_8 s_564_18
        let s_564_19: Bits = ((s_564_8) & (s_564_18));
        // D s_564_20: or s_564_19 s_564_16
        let s_564_20: Bits = ((s_564_19) | (s_564_16));
        // D s_564_21: cast reint s_564_20 -> u64
        let s_564_21: u64 = (s_564_20.value() as u64);
        // D s_564_22: cast zx s_564_21 -> bv
        let s_564_22: Bits = Bits::new(s_564_21 as u128, 64u16);
        // D s_564_23: read-var t:i
        let s_564_23: i128 = fn_state.t;
        // D s_564_24: call X_set(s_564_23, s_564_0, s_564_22)
        let s_564_24: () = X_set(state, tracer, s_564_23, s_564_0, s_564_22);
        // N s_564_25: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_565<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_565_0: const #10s : i
        let s_565_0: i128 = 10;
        // D s_565_1: read-var crm:i
        let s_565_1: i128 = fn_state.crm;
        // D s_565_2: cmp-eq s_565_1 s_565_0
        let s_565_2: bool = ((s_565_1) == (s_565_0));
        // D s_565_3: write-var gs#140521 <= s_565_2
        fn_state.gs_140521 = s_565_2;
        // N s_565_4: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_566<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_566_0: const #7s : i
        let s_566_0: i128 = 7;
        // D s_566_1: read-var op2:i
        let s_566_1: i128 = fn_state.op2;
        // D s_566_2: cmp-eq s_566_1 s_566_0
        let s_566_2: bool = ((s_566_1) == (s_566_0));
        // D s_566_3: write-var gs#140519 <= s_566_2
        fn_state.gs_140519 = s_566_2;
        // N s_566_4: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_567<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_567_0: const #0s : i
        let s_567_0: i128 = 0;
        // D s_567_1: read-var op1:i
        let s_567_1: i128 = fn_state.op1;
        // D s_567_2: cmp-eq s_567_1 s_567_0
        let s_567_2: bool = ((s_567_1) == (s_567_0));
        // D s_567_3: write-var gs#140517 <= s_567_2
        fn_state.gs_140517 = s_567_2;
        // N s_567_4: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_568<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_568_0: const #9s : i
        let s_568_0: i128 = 9;
        // D s_568_1: read-var crn:i
        let s_568_1: i128 = fn_state.crn;
        // D s_568_2: cmp-eq s_568_1 s_568_0
        let s_568_2: bool = ((s_568_1) == (s_568_0));
        // D s_568_3: write-var gs#140515 <= s_568_2
        fn_state.gs_140515 = s_568_2;
        // N s_568_4: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_569<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_569_0: const #() : ()
        let s_569_0: () = ();
        // S s_569_1: call HaveRME(s_569_0)
        let s_569_1: bool = HaveRME(state, tracer, s_569_0);
        // N s_569_2: branch s_569_1 b578 b570
        if s_569_1 {
            return block_578(state, tracer, fn_state);
        } else {
            return block_570(state, tracer, fn_state);
        };
    }
    fn block_570<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_570_0: const #0u : u8
        let s_570_0: bool = false;
        // D s_570_1: write-var gs#141032 <= s_570_0
        fn_state.gs_141032 = s_570_0;
        // N s_570_2: jump b571
        return block_571(state, tracer, fn_state);
    }
    fn block_571<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_571_0: read-var gs#141032:u8
        let s_571_0: bool = fn_state.gs_141032;
        // N s_571_1: branch s_571_0 b574 b572
        if s_571_0 {
            return block_574(state, tracer, fn_state);
        } else {
            return block_572(state, tracer, fn_state);
        };
    }
    fn block_572<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_572_0: jump b573
        return block_573(state, tracer, fn_state);
    }
    fn block_573<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_573_0: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_574<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_574_0: const #64s : i64
        let s_574_0: i64 = 64;
        // D s_574_1: read-var t:i
        let s_574_1: i128 = fn_state.t;
        // D s_574_2: call X_read(s_574_1, s_574_0)
        let s_574_2: Bits = X_read(state, tracer, s_574_1, s_574_0);
        // D s_574_3: cast reint s_574_2 -> u64
        let s_574_3: u64 = (s_574_2.value() as u64);
        // D s_574_4: write-var ga#246883 <= s_574_3
        fn_state.ga_246883 = s_574_3;
        // C s_574_5: const #() : ()
        let s_574_5: () = ();
        // S s_574_6: call UsePrimarySpaceEL2(s_574_5)
        let s_574_6: bool = UsePrimarySpaceEL2(state, tracer, s_574_5);
        // S s_574_7: not s_574_6
        let s_574_7: bool = !s_574_6;
        // N s_574_8: branch s_574_7 b577 b575
        if s_574_7 {
            return block_577(state, tracer, fn_state);
        } else {
            return block_575(state, tracer, fn_state);
        };
    }
    fn block_575<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_575_0: const #0u : u8
        let s_575_0: bool = false;
        // D s_575_1: write-var ga#246884 <= s_575_0
        fn_state.ga_246884 = s_575_0;
        // N s_575_2: jump b576
        return block_576(state, tracer, fn_state);
    }
    fn block_576<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_576_0: const #54s : i
        let s_576_0: i128 = 54;
        // D s_576_1: read-var ga#246883:u64
        let s_576_1: u64 = fn_state.ga_246883;
        // D s_576_2: cast zx s_576_1 -> bv
        let s_576_2: Bits = Bits::new(s_576_1 as u128, 64u16);
        // D s_576_3: read-var ga#246884:u8
        let s_576_3: bool = fn_state.ga_246884;
        // D s_576_4: cast zx s_576_3 -> bv
        let s_576_4: Bits = Bits::new(s_576_3 as u128, 1u16);
        // C s_576_5: const #0s : i
        let s_576_5: i128 = 0;
        // C s_576_6: const #1u : u64
        let s_576_6: u64 = 1;
        // C s_576_7: cast zx s_576_6 -> bv
        let s_576_7: Bits = Bits::new(s_576_6 as u128, 64u16);
        // C s_576_8: lsl s_576_7 s_576_5
        let s_576_8: Bits = s_576_7 << s_576_5;
        // C s_576_9: sub s_576_8 s_576_7
        let s_576_9: Bits = ((s_576_8) - (s_576_7));
        // D s_576_10: and s_576_4 s_576_9
        let s_576_10: Bits = ((s_576_4) & (s_576_9));
        // D s_576_11: lsl s_576_10 s_576_0
        let s_576_11: Bits = s_576_10 << s_576_0;
        // C s_576_12: lsl s_576_9 s_576_0
        let s_576_12: Bits = s_576_9 << s_576_0;
        // C s_576_13: cmpl s_576_12
        let s_576_13: Bits = !s_576_12;
        // D s_576_14: and s_576_2 s_576_13
        let s_576_14: Bits = ((s_576_2) & (s_576_13));
        // D s_576_15: or s_576_14 s_576_11
        let s_576_15: Bits = ((s_576_14) | (s_576_11));
        // D s_576_16: cast reint s_576_15 -> u64
        let s_576_16: u64 = (s_576_15.value() as u64);
        // D s_576_17: cast zx s_576_16 -> bv
        let s_576_17: Bits = Bits::new(s_576_16 as u128, 64u16);
        // D s_576_18: read-var t:i
        let s_576_18: i128 = fn_state.t;
        // C s_576_19: const #64s : i64
        let s_576_19: i64 = 64;
        // D s_576_20: call X_set(s_576_18, s_576_19, s_576_17)
        let s_576_20: () = X_set(state, tracer, s_576_18, s_576_19, s_576_17);
        // N s_576_21: jump b573
        return block_573(state, tracer, fn_state);
    }
    fn block_577<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_577_0: const #1u : u8
        let s_577_0: bool = true;
        // D s_577_1: write-var ga#246884 <= s_577_0
        fn_state.ga_246884 = s_577_0;
        // N s_577_2: jump b576
        return block_576(state, tracer, fn_state);
    }
    fn block_578<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_578_0: const #11032u : u32
        let s_578_0: u32 = 11032;
        // D s_578_1: read-reg s_578_0:struct
        let s_578_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_578_0 as isize);
            tracer.read_register(s_578_0 as isize, value);
            value
        };
        // D s_578_2: call _get_MPAMIDR_EL1_Type_HAS_ALTSP(s_578_1)
        let s_578_2: bool = u_get_MPAMIDR_EL1_Type_HAS_ALTSP(state, tracer, s_578_1);
        // D s_578_3: cast zx s_578_2 -> bv
        let s_578_3: Bits = Bits::new(s_578_2 as u128, 1u16);
        // C s_578_4: const #1u : u8
        let s_578_4: bool = true;
        // C s_578_5: cast zx s_578_4 -> bv
        let s_578_5: Bits = Bits::new(s_578_4 as u128, 1u16);
        // D s_578_6: cmp-eq s_578_3 s_578_5
        let s_578_6: bool = ((s_578_3) == (s_578_5));
        // D s_578_7: write-var gs#141032 <= s_578_6
        fn_state.gs_141032 = s_578_6;
        // N s_578_8: jump b571
        return block_571(state, tracer, fn_state);
    }
    fn block_579<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_579_0: const #4s : i
        let s_579_0: i128 = 4;
        // D s_579_1: read-var op1:i
        let s_579_1: i128 = fn_state.op1;
        // D s_579_2: cmp-eq s_579_1 s_579_0
        let s_579_2: bool = ((s_579_1) == (s_579_0));
        // D s_579_3: write-var gs#140512 <= s_579_2
        fn_state.gs_140512 = s_579_2;
        // N s_579_4: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_580<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_580_0: const #0s : i
        let s_580_0: i128 = 0;
        // D s_580_1: read-var op2:i
        let s_580_1: i128 = fn_state.op2;
        // D s_580_2: cmp-eq s_580_1 s_580_0
        let s_580_2: bool = ((s_580_1) == (s_580_0));
        // D s_580_3: write-var gs#140510 <= s_580_2
        fn_state.gs_140510 = s_580_2;
        // N s_580_4: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_581<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_581_0: const #5s : i
        let s_581_0: i128 = 5;
        // D s_581_1: read-var crm:i
        let s_581_1: i128 = fn_state.crm;
        // D s_581_2: cmp-eq s_581_1 s_581_0
        let s_581_2: bool = ((s_581_1) == (s_581_0));
        // D s_581_3: write-var gs#140508 <= s_581_2
        fn_state.gs_140508 = s_581_2;
        // N s_581_4: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_582<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_582_0: const #10s : i
        let s_582_0: i128 = 10;
        // D s_582_1: read-var crn:i
        let s_582_1: i128 = fn_state.crn;
        // D s_582_2: cmp-eq s_582_1 s_582_0
        let s_582_2: bool = ((s_582_1) == (s_582_0));
        // D s_582_3: write-var gs#140506 <= s_582_2
        fn_state.gs_140506 = s_582_2;
        // N s_582_4: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_583<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_583_0: const #() : ()
        let s_583_0: () = ();
        // S s_583_1: call HaveRME(s_583_0)
        let s_583_1: bool = HaveRME(state, tracer, s_583_0);
        // N s_583_2: branch s_583_1 b592 b584
        if s_583_1 {
            return block_592(state, tracer, fn_state);
        } else {
            return block_584(state, tracer, fn_state);
        };
    }
    fn block_584<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_584_0: const #0u : u8
        let s_584_0: bool = false;
        // D s_584_1: write-var gs#141039 <= s_584_0
        fn_state.gs_141039 = s_584_0;
        // N s_584_2: jump b585
        return block_585(state, tracer, fn_state);
    }
    fn block_585<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_585_0: read-var gs#141039:u8
        let s_585_0: bool = fn_state.gs_141039;
        // N s_585_1: branch s_585_0 b588 b586
        if s_585_0 {
            return block_588(state, tracer, fn_state);
        } else {
            return block_586(state, tracer, fn_state);
        };
    }
    fn block_586<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_586_0: jump b587
        return block_587(state, tracer, fn_state);
    }
    fn block_587<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_587_0: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_588<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_588_0: const #64s : i64
        let s_588_0: i64 = 64;
        // D s_588_1: read-var t:i
        let s_588_1: i128 = fn_state.t;
        // D s_588_2: call X_read(s_588_1, s_588_0)
        let s_588_2: Bits = X_read(state, tracer, s_588_1, s_588_0);
        // D s_588_3: cast reint s_588_2 -> u64
        let s_588_3: u64 = (s_588_2.value() as u64);
        // D s_588_4: write-var ga#246868 <= s_588_3
        fn_state.ga_246868 = s_588_3;
        // C s_588_5: const #() : ()
        let s_588_5: () = ();
        // S s_588_6: call UsePrimarySpaceEL10(s_588_5)
        let s_588_6: bool = UsePrimarySpaceEL10(state, tracer, s_588_5);
        // S s_588_7: not s_588_6
        let s_588_7: bool = !s_588_6;
        // N s_588_8: branch s_588_7 b591 b589
        if s_588_7 {
            return block_591(state, tracer, fn_state);
        } else {
            return block_589(state, tracer, fn_state);
        };
    }
    fn block_589<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_589_0: const #0u : u8
        let s_589_0: bool = false;
        // D s_589_1: write-var ga#246869 <= s_589_0
        fn_state.ga_246869 = s_589_0;
        // N s_589_2: jump b590
        return block_590(state, tracer, fn_state);
    }
    fn block_590<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_590_0: const #54s : i
        let s_590_0: i128 = 54;
        // D s_590_1: read-var ga#246868:u64
        let s_590_1: u64 = fn_state.ga_246868;
        // D s_590_2: cast zx s_590_1 -> bv
        let s_590_2: Bits = Bits::new(s_590_1 as u128, 64u16);
        // D s_590_3: read-var ga#246869:u8
        let s_590_3: bool = fn_state.ga_246869;
        // D s_590_4: cast zx s_590_3 -> bv
        let s_590_4: Bits = Bits::new(s_590_3 as u128, 1u16);
        // C s_590_5: const #0s : i
        let s_590_5: i128 = 0;
        // C s_590_6: const #1u : u64
        let s_590_6: u64 = 1;
        // C s_590_7: cast zx s_590_6 -> bv
        let s_590_7: Bits = Bits::new(s_590_6 as u128, 64u16);
        // C s_590_8: lsl s_590_7 s_590_5
        let s_590_8: Bits = s_590_7 << s_590_5;
        // C s_590_9: sub s_590_8 s_590_7
        let s_590_9: Bits = ((s_590_8) - (s_590_7));
        // D s_590_10: and s_590_4 s_590_9
        let s_590_10: Bits = ((s_590_4) & (s_590_9));
        // D s_590_11: lsl s_590_10 s_590_0
        let s_590_11: Bits = s_590_10 << s_590_0;
        // C s_590_12: lsl s_590_9 s_590_0
        let s_590_12: Bits = s_590_9 << s_590_0;
        // C s_590_13: cmpl s_590_12
        let s_590_13: Bits = !s_590_12;
        // D s_590_14: and s_590_2 s_590_13
        let s_590_14: Bits = ((s_590_2) & (s_590_13));
        // D s_590_15: or s_590_14 s_590_11
        let s_590_15: Bits = ((s_590_14) | (s_590_11));
        // D s_590_16: cast reint s_590_15 -> u64
        let s_590_16: u64 = (s_590_15.value() as u64);
        // D s_590_17: cast zx s_590_16 -> bv
        let s_590_17: Bits = Bits::new(s_590_16 as u128, 64u16);
        // D s_590_18: read-var t:i
        let s_590_18: i128 = fn_state.t;
        // C s_590_19: const #64s : i64
        let s_590_19: i64 = 64;
        // D s_590_20: call X_set(s_590_18, s_590_19, s_590_17)
        let s_590_20: () = X_set(state, tracer, s_590_18, s_590_19, s_590_17);
        // N s_590_21: jump b587
        return block_587(state, tracer, fn_state);
    }
    fn block_591<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_591_0: const #1u : u8
        let s_591_0: bool = true;
        // D s_591_1: write-var ga#246869 <= s_591_0
        fn_state.ga_246869 = s_591_0;
        // N s_591_2: jump b590
        return block_590(state, tracer, fn_state);
    }
    fn block_592<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_592_0: const #11032u : u32
        let s_592_0: u32 = 11032;
        // D s_592_1: read-reg s_592_0:struct
        let s_592_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_592_0 as isize);
            tracer.read_register(s_592_0 as isize, value);
            value
        };
        // D s_592_2: call _get_MPAMIDR_EL1_Type_HAS_ALTSP(s_592_1)
        let s_592_2: bool = u_get_MPAMIDR_EL1_Type_HAS_ALTSP(state, tracer, s_592_1);
        // D s_592_3: cast zx s_592_2 -> bv
        let s_592_3: Bits = Bits::new(s_592_2 as u128, 1u16);
        // C s_592_4: const #1u : u8
        let s_592_4: bool = true;
        // C s_592_5: cast zx s_592_4 -> bv
        let s_592_5: Bits = Bits::new(s_592_4 as u128, 1u16);
        // D s_592_6: cmp-eq s_592_3 s_592_5
        let s_592_6: bool = ((s_592_3) == (s_592_5));
        // D s_592_7: write-var gs#141039 <= s_592_6
        fn_state.gs_141039 = s_592_6;
        // N s_592_8: jump b585
        return block_585(state, tracer, fn_state);
    }
    fn block_593<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_593_0: const #0s : i
        let s_593_0: i128 = 0;
        // D s_593_1: read-var op1:i
        let s_593_1: i128 = fn_state.op1;
        // D s_593_2: cmp-eq s_593_1 s_593_0
        let s_593_2: bool = ((s_593_1) == (s_593_0));
        // N s_593_3: branch s_593_2 b596 b594
        if s_593_2 {
            return block_596(state, tracer, fn_state);
        } else {
            return block_594(state, tracer, fn_state);
        };
    }
    fn block_594<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_594_0: const #5s : i
        let s_594_0: i128 = 5;
        // D s_594_1: read-var op1:i
        let s_594_1: i128 = fn_state.op1;
        // D s_594_2: cmp-eq s_594_1 s_594_0
        let s_594_2: bool = ((s_594_1) == (s_594_0));
        // D s_594_3: write-var gs#140502 <= s_594_2
        fn_state.gs_140502 = s_594_2;
        // N s_594_4: jump b595
        return block_595(state, tracer, fn_state);
    }
    fn block_595<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_595_0: read-var gs#140502:u8
        let s_595_0: bool = fn_state.gs_140502;
        // D s_595_1: write-var gs#140503 <= s_595_0
        fn_state.gs_140503 = s_595_0;
        // N s_595_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_596<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_596_0: const #1u : u8
        let s_596_0: bool = true;
        // D s_596_1: write-var gs#140502 <= s_596_0
        fn_state.gs_140502 = s_596_0;
        // N s_596_2: jump b595
        return block_595(state, tracer, fn_state);
    }
    fn block_597<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_597_0: const #0s : i
        let s_597_0: i128 = 0;
        // D s_597_1: read-var op2:i
        let s_597_1: i128 = fn_state.op2;
        // D s_597_2: cmp-eq s_597_1 s_597_0
        let s_597_2: bool = ((s_597_1) == (s_597_0));
        // D s_597_3: write-var gs#140499 <= s_597_2
        fn_state.gs_140499 = s_597_2;
        // N s_597_4: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_598<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_598_0: const #5s : i
        let s_598_0: i128 = 5;
        // D s_598_1: read-var crm:i
        let s_598_1: i128 = fn_state.crm;
        // D s_598_2: cmp-eq s_598_1 s_598_0
        let s_598_2: bool = ((s_598_1) == (s_598_0));
        // D s_598_3: write-var gs#140497 <= s_598_2
        fn_state.gs_140497 = s_598_2;
        // N s_598_4: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_599<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_599_0: const #10s : i
        let s_599_0: i128 = 10;
        // D s_599_1: read-var crn:i
        let s_599_1: i128 = fn_state.crn;
        // D s_599_2: cmp-eq s_599_1 s_599_0
        let s_599_2: bool = ((s_599_1) == (s_599_0));
        // D s_599_3: write-var gs#140495 <= s_599_2
        fn_state.gs_140495 = s_599_2;
        // N s_599_4: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_600<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_600_0: const #19136u : u32
        let s_600_0: u32 = 19136;
        // D s_600_1: read-reg s_600_0:struct
        let s_600_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_600_0 as isize);
            tracer.read_register(s_600_0 as isize, value);
            value
        };
        // D s_600_2: call _get_PMSELR_EL0_Type_SEL(s_600_1)
        let s_600_2: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_600_1);
        // D s_600_3: cast zx s_600_2 -> bv
        let s_600_3: Bits = Bits::new(s_600_2 as u128, 5u16);
        // D s_600_4: cast zx s_600_3 -> i
        let s_600_4: i128 = (s_600_3.value() as i128);
        // D s_600_5: cast reint s_600_4 -> i64
        let s_600_5: i64 = (s_600_4 as i64);
        // C s_600_6: const #() : ()
        let s_600_6: () = ();
        // S s_600_7: call GetNumEventCounters(s_600_6)
        let s_600_7: i128 = GetNumEventCounters(state, tracer, s_600_6);
        // C s_600_8: const #1s : i
        let s_600_8: i128 = 1;
        // S s_600_9: sub s_600_7 s_600_8
        let s_600_9: i128 = ((s_600_7) - (s_600_8));
        // D s_600_10: cast zx s_600_5 -> i
        let s_600_10: i128 = (i128::try_from(s_600_5).unwrap());
        // D s_600_11: cmp-gt s_600_10 s_600_9
        let s_600_11: bool = ((s_600_10) > (s_600_9));
        // N s_600_12: branch s_600_11 b629 b601
        if s_600_11 {
            return block_629(state, tracer, fn_state);
        } else {
            return block_601(state, tracer, fn_state);
        };
    }
    fn block_601<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_601_0: const #() : ()
        let s_601_0: () = ();
        // S s_601_1: call EL2Enabled(s_601_0)
        let s_601_1: bool = EL2Enabled(state, tracer, s_601_0);
        // N s_601_2: branch s_601_1 b613 b602
        if s_601_1 {
            return block_613(state, tracer, fn_state);
        } else {
            return block_602(state, tracer, fn_state);
        };
    }
    fn block_602<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_602_0: const #0u : u8
        let s_602_0: bool = false;
        // D s_602_1: write-var gs#140428 <= s_602_0
        fn_state.gs_140428 = s_602_0;
        // N s_602_2: jump b603
        return block_603(state, tracer, fn_state);
    }
    fn block_603<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_603_0: read-var gs#140428:u8
        let s_603_0: bool = fn_state.gs_140428;
        // N s_603_1: branch s_603_0 b612 b604
        if s_603_0 {
            return block_612(state, tracer, fn_state);
        } else {
            return block_604(state, tracer, fn_state);
        };
    }
    fn block_604<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_604_0: const #0u : u8
        let s_604_0: bool = false;
        // D s_604_1: write-var gs#140430 <= s_604_0
        fn_state.gs_140430 = s_604_0;
        // N s_604_2: jump b605
        return block_605(state, tracer, fn_state);
    }
    fn block_605<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_605_0: read-var gs#140430:u8
        let s_605_0: bool = fn_state.gs_140430;
        // D s_605_1: write-var gs#140431 <= s_605_0
        fn_state.gs_140431 = s_605_0;
        // N s_605_2: jump b606
        return block_606(state, tracer, fn_state);
    }
    fn block_606<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_606_0: read-var gs#140431:u8
        let s_606_0: bool = fn_state.gs_140431;
        // N s_606_1: branch s_606_0 b609 b607
        if s_606_0 {
            return block_609(state, tracer, fn_state);
        } else {
            return block_607(state, tracer, fn_state);
        };
    }
    fn block_607<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_607_0: jump b608
        return block_608(state, tracer, fn_state);
    }
    fn block_608<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_608_0: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_609<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_609_0: const #19136u : u32
        let s_609_0: u32 = 19136;
        // D s_609_1: read-reg s_609_0:struct
        let s_609_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_609_0 as isize);
            tracer.read_register(s_609_0 as isize, value);
            value
        };
        // D s_609_2: call _get_PMSELR_EL0_Type_SEL(s_609_1)
        let s_609_2: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_609_1);
        // D s_609_3: cast zx s_609_2 -> bv
        let s_609_3: Bits = Bits::new(s_609_2 as u128, 5u16);
        // D s_609_4: cast zx s_609_3 -> i
        let s_609_4: i128 = (s_609_3.value() as i128);
        // D s_609_5: cast reint s_609_4 -> i64
        let s_609_5: i64 = (s_609_4 as i64);
        // C s_609_6: const #() : ()
        let s_609_6: () = ();
        // S s_609_7: call GetNumEventCounters(s_609_6)
        let s_609_7: i128 = GetNumEventCounters(state, tracer, s_609_6);
        // C s_609_8: const #1s : i
        let s_609_8: i128 = 1;
        // S s_609_9: sub s_609_7 s_609_8
        let s_609_9: i128 = ((s_609_7) - (s_609_8));
        // D s_609_10: cast zx s_609_5 -> i
        let s_609_10: i128 = (i128::try_from(s_609_5).unwrap());
        // D s_609_11: cmp-gt s_609_10 s_609_9
        let s_609_11: bool = ((s_609_10) > (s_609_9));
        // N s_609_12: branch s_609_11 b611 b610
        if s_609_11 {
            return block_611(state, tracer, fn_state);
        } else {
            return block_610(state, tracer, fn_state);
        };
    }
    fn block_610<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_610_0: const #24u : u8
        let s_610_0: u8 = 24;
        // C s_610_1: cast zx s_610_0 -> bv
        let s_610_1: Bits = Bits::new(s_610_0 as u128, 8u16);
        // C s_610_2: cast zx s_610_1 -> i
        let s_610_2: i128 = (s_610_1.value() as i128);
        // C s_610_3: cast reint s_610_2 -> i64
        let s_610_3: i64 = (s_610_2 as i64);
        // C s_610_4: cast zx s_610_3 -> i
        let s_610_4: i128 = (i128::try_from(s_610_3).unwrap());
        // C s_610_5: const #432u : u32
        let s_610_5: u32 = 432;
        // D s_610_6: read-reg s_610_5:u8
        let s_610_6: u8 = {
            let value = state.read_register::<u8>(s_610_5 as isize);
            tracer.read_register(s_610_5 as isize, value);
            value
        };
        // D s_610_7: call AArch64_SystemAccessTrap(s_610_6, s_610_4)
        let s_610_7: () = AArch64_SystemAccessTrap(state, tracer, s_610_6, s_610_4);
        // N s_610_8: jump b608
        return block_608(state, tracer, fn_state);
    }
    fn block_611<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_611_0: panic
        panic!("{:?}", ());
        // N s_611_1: return
        return;
    }
    fn block_612<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_612_0: const #19136u : u32
        let s_612_0: u32 = 19136;
        // D s_612_1: read-reg s_612_0:struct
        let s_612_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_612_0 as isize);
            tracer.read_register(s_612_0 as isize, value);
            value
        };
        // D s_612_2: call _get_PMSELR_EL0_Type_SEL(s_612_1)
        let s_612_2: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_612_1);
        // D s_612_3: cast zx s_612_2 -> bv
        let s_612_3: Bits = Bits::new(s_612_2 as u128, 5u16);
        // D s_612_4: cast zx s_612_3 -> i
        let s_612_4: i128 = (s_612_3.value() as i128);
        // D s_612_5: cast reint s_612_4 -> i64
        let s_612_5: i64 = (s_612_4 as i64);
        // C s_612_6: const #() : ()
        let s_612_6: () = ();
        // S s_612_7: call AArch64_GetNumEventCountersAccessible(s_612_6)
        let s_612_7: i128 = AArch64_GetNumEventCountersAccessible(
            state,
            tracer,
            s_612_6,
        );
        // C s_612_8: const #1s : i
        let s_612_8: i128 = 1;
        // S s_612_9: sub s_612_7 s_612_8
        let s_612_9: i128 = ((s_612_7) - (s_612_8));
        // D s_612_10: cast zx s_612_5 -> i
        let s_612_10: i128 = (i128::try_from(s_612_5).unwrap());
        // D s_612_11: cmp-gt s_612_10 s_612_9
        let s_612_11: bool = ((s_612_10) > (s_612_9));
        // D s_612_12: write-var gs#140430 <= s_612_11
        fn_state.gs_140430 = s_612_11;
        // N s_612_13: jump b605
        return block_605(state, tracer, fn_state);
    }
    fn block_613<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_613_0: const #16975u : u32
        let s_613_0: u32 = 16975;
        // D s_613_1: read-reg s_613_0:u8
        let s_613_1: u8 = {
            let value = state.read_register::<u8>(s_613_0 as isize);
            tracer.read_register(s_613_0 as isize, value);
            value
        };
        // D s_613_2: cast zx s_613_1 -> bv
        let s_613_2: Bits = Bits::new(s_613_1 as u128, 2u16);
        // C s_613_3: const #440u : u32
        let s_613_3: u32 = 440;
        // D s_613_4: read-reg s_613_3:u8
        let s_613_4: u8 = {
            let value = state.read_register::<u8>(s_613_3 as isize);
            tracer.read_register(s_613_3 as isize, value);
            value
        };
        // D s_613_5: cast zx s_613_4 -> bv
        let s_613_5: Bits = Bits::new(s_613_4 as u128, 2u16);
        // D s_613_6: cmp-eq s_613_2 s_613_5
        let s_613_6: bool = ((s_613_2) == (s_613_5));
        // N s_613_7: branch s_613_6 b628 b614
        if s_613_6 {
            return block_628(state, tracer, fn_state);
        } else {
            return block_614(state, tracer, fn_state);
        };
    }
    fn block_614<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_614_0: const #16975u : u32
        let s_614_0: u32 = 16975;
        // D s_614_1: read-reg s_614_0:u8
        let s_614_1: u8 = {
            let value = state.read_register::<u8>(s_614_0 as isize);
            tracer.read_register(s_614_0 as isize, value);
            value
        };
        // D s_614_2: cast zx s_614_1 -> bv
        let s_614_2: Bits = Bits::new(s_614_1 as u128, 2u16);
        // C s_614_3: const #448u : u32
        let s_614_3: u32 = 448;
        // D s_614_4: read-reg s_614_3:u8
        let s_614_4: u8 = {
            let value = state.read_register::<u8>(s_614_3 as isize);
            tracer.read_register(s_614_3 as isize, value);
            value
        };
        // D s_614_5: cast zx s_614_4 -> bv
        let s_614_5: Bits = Bits::new(s_614_4 as u128, 2u16);
        // D s_614_6: cmp-eq s_614_2 s_614_5
        let s_614_6: bool = ((s_614_2) == (s_614_5));
        // N s_614_7: branch s_614_6 b618 b615
        if s_614_6 {
            return block_618(state, tracer, fn_state);
        } else {
            return block_615(state, tracer, fn_state);
        };
    }
    fn block_615<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_615_0: const #0u : u8
        let s_615_0: bool = false;
        // D s_615_1: write-var gs#140426 <= s_615_0
        fn_state.gs_140426 = s_615_0;
        // N s_615_2: jump b616
        return block_616(state, tracer, fn_state);
    }
    fn block_616<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_616_0: read-var gs#140426:u8
        let s_616_0: bool = fn_state.gs_140426;
        // D s_616_1: write-var gs#140427 <= s_616_0
        fn_state.gs_140427 = s_616_0;
        // N s_616_2: jump b617
        return block_617(state, tracer, fn_state);
    }
    fn block_617<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_617_0: read-var gs#140427:u8
        let s_617_0: bool = fn_state.gs_140427;
        // D s_617_1: write-var gs#140428 <= s_617_0
        fn_state.gs_140428 = s_617_0;
        // N s_617_2: jump b603
        return block_603(state, tracer, fn_state);
    }
    fn block_618<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_618_0: const #2s : i
        let s_618_0: i128 = 2;
        // D s_618_1: read-var op2:i
        let s_618_1: i128 = fn_state.op2;
        // D s_618_2: cmp-eq s_618_1 s_618_0
        let s_618_2: bool = ((s_618_1) == (s_618_0));
        // N s_618_3: branch s_618_2 b627 b619
        if s_618_2 {
            return block_627(state, tracer, fn_state);
        } else {
            return block_619(state, tracer, fn_state);
        };
    }
    fn block_619<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_619_0: const #0u : u8
        let s_619_0: bool = false;
        // D s_619_1: write-var gs#140422 <= s_619_0
        fn_state.gs_140422 = s_619_0;
        // N s_619_2: jump b620
        return block_620(state, tracer, fn_state);
    }
    fn block_620<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_620_0: read-var gs#140422:u8
        let s_620_0: bool = fn_state.gs_140422;
        // N s_620_1: branch s_620_0 b626 b621
        if s_620_0 {
            return block_626(state, tracer, fn_state);
        } else {
            return block_621(state, tracer, fn_state);
        };
    }
    fn block_621<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_621_0: const #1s : i
        let s_621_0: i128 = 1;
        // D s_621_1: read-var op2:i
        let s_621_1: i128 = fn_state.op2;
        // D s_621_2: cmp-eq s_621_1 s_621_0
        let s_621_2: bool = ((s_621_1) == (s_621_0));
        // N s_621_3: branch s_621_2 b625 b622
        if s_621_2 {
            return block_625(state, tracer, fn_state);
        } else {
            return block_622(state, tracer, fn_state);
        };
    }
    fn block_622<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_622_0: const #0u : u8
        let s_622_0: bool = false;
        // D s_622_1: write-var gs#140424 <= s_622_0
        fn_state.gs_140424 = s_622_0;
        // N s_622_2: jump b623
        return block_623(state, tracer, fn_state);
    }
    fn block_623<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_623_0: read-var gs#140424:u8
        let s_623_0: bool = fn_state.gs_140424;
        // D s_623_1: write-var gs#140425 <= s_623_0
        fn_state.gs_140425 = s_623_0;
        // N s_623_2: jump b624
        return block_624(state, tracer, fn_state);
    }
    fn block_624<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_624_0: read-var gs#140425:u8
        let s_624_0: bool = fn_state.gs_140425;
        // D s_624_1: write-var gs#140426 <= s_624_0
        fn_state.gs_140426 = s_624_0;
        // N s_624_2: jump b616
        return block_616(state, tracer, fn_state);
    }
    fn block_625<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_625_0: const #102624u : u32
        let s_625_0: u32 = 102624;
        // D s_625_1: read-reg s_625_0:struct
        let s_625_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_625_0 as isize);
            tracer.read_register(s_625_0 as isize, value);
            value
        };
        // D s_625_2: call _get_PMUSERENR_EL0_Type_EN(s_625_1)
        let s_625_2: bool = u_get_PMUSERENR_EL0_Type_EN(state, tracer, s_625_1);
        // D s_625_3: cast zx s_625_2 -> bv
        let s_625_3: Bits = Bits::new(s_625_2 as u128, 1u16);
        // C s_625_4: const #1u : u8
        let s_625_4: bool = true;
        // C s_625_5: cast zx s_625_4 -> bv
        let s_625_5: Bits = Bits::new(s_625_4 as u128, 1u16);
        // D s_625_6: cmp-eq s_625_3 s_625_5
        let s_625_6: bool = ((s_625_3) == (s_625_5));
        // D s_625_7: write-var gs#140424 <= s_625_6
        fn_state.gs_140424 = s_625_6;
        // N s_625_8: jump b623
        return block_623(state, tracer, fn_state);
    }
    fn block_626<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_626_0: const #1u : u8
        let s_626_0: bool = true;
        // D s_626_1: write-var gs#140425 <= s_626_0
        fn_state.gs_140425 = s_626_0;
        // N s_626_2: jump b624
        return block_624(state, tracer, fn_state);
    }
    fn block_627<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_627_0: const #102624u : u32
        let s_627_0: u32 = 102624;
        // D s_627_1: read-reg s_627_0:struct
        let s_627_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_627_0 as isize);
            tracer.read_register(s_627_0 as isize, value);
            value
        };
        // D s_627_2: call _get_PMUSERENR_EL0_Type_ER(s_627_1)
        let s_627_2: bool = u_get_PMUSERENR_EL0_Type_ER(state, tracer, s_627_1);
        // C s_627_3: const #102624u : u32
        let s_627_3: u32 = 102624;
        // D s_627_4: read-reg s_627_3:struct
        let s_627_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_627_3 as isize);
            tracer.read_register(s_627_3 as isize, value);
            value
        };
        // D s_627_5: call _get_PMUSERENR_EL0_Type_EN(s_627_4)
        let s_627_5: bool = u_get_PMUSERENR_EL0_Type_EN(state, tracer, s_627_4);
        // D s_627_6: cast zx s_627_2 -> bv
        let s_627_6: Bits = Bits::new(s_627_2 as u128, 1u16);
        // D s_627_7: cast zx s_627_5 -> bv
        let s_627_7: Bits = Bits::new(s_627_5 as u128, 1u16);
        // D s_627_8: cast reint s_627_6 -> u128
        let s_627_8: u128 = (s_627_6.value() as u128);
        // D s_627_9: size-of s_627_6
        let s_627_9: u16 = s_627_6.length();
        // D s_627_10: cast reint s_627_7 -> u128
        let s_627_10: u128 = (s_627_7.value() as u128);
        // D s_627_11: size-of s_627_7
        let s_627_11: u16 = s_627_7.length();
        // D s_627_12: lsl s_627_8 s_627_11
        let s_627_12: u128 = s_627_8 << s_627_11;
        // D s_627_13: or s_627_12 s_627_10
        let s_627_13: u128 = ((s_627_12) | (s_627_10));
        // D s_627_14: add s_627_9 s_627_11
        let s_627_14: u16 = (s_627_9 + s_627_11);
        // D s_627_15: create-bits s_627_13 s_627_14
        let s_627_15: Bits = Bits::new(s_627_13, s_627_14);
        // D s_627_16: cast reint s_627_15 -> u8
        let s_627_16: u8 = (s_627_15.value() as u8);
        // D s_627_17: cast zx s_627_16 -> bv
        let s_627_17: Bits = Bits::new(s_627_16 as u128, 2u16);
        // C s_627_18: const #0u : u8
        let s_627_18: u8 = 0;
        // C s_627_19: cast zx s_627_18 -> bv
        let s_627_19: Bits = Bits::new(s_627_18 as u128, 2u16);
        // D s_627_20: cmp-ne s_627_17 s_627_19
        let s_627_20: bool = ((s_627_17) != (s_627_19));
        // D s_627_21: write-var gs#140422 <= s_627_20
        fn_state.gs_140422 = s_627_20;
        // N s_627_22: jump b620
        return block_620(state, tracer, fn_state);
    }
    fn block_628<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_628_0: const #1u : u8
        let s_628_0: bool = true;
        // D s_628_1: write-var gs#140427 <= s_628_0
        fn_state.gs_140427 = s_628_0;
        // N s_628_2: jump b617
        return block_617(state, tracer, fn_state);
    }
    fn block_629<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_629_0: const #1u : u8
        let s_629_0: bool = true;
        // D s_629_1: write-var gs#140431 <= s_629_0
        fn_state.gs_140431 = s_629_0;
        // N s_629_2: jump b606
        return block_606(state, tracer, fn_state);
    }
    fn block_630<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_630_0: const #19136u : u32
        let s_630_0: u32 = 19136;
        // D s_630_1: read-reg s_630_0:struct
        let s_630_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_630_0 as isize);
            tracer.read_register(s_630_0 as isize, value);
            value
        };
        // D s_630_2: call _get_PMSELR_EL0_Type_SEL(s_630_1)
        let s_630_2: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_630_1);
        // D s_630_3: cast zx s_630_2 -> bv
        let s_630_3: Bits = Bits::new(s_630_2 as u128, 5u16);
        // C s_630_4: const #31u : u8
        let s_630_4: u8 = 31;
        // C s_630_5: cast zx s_630_4 -> bv
        let s_630_5: Bits = Bits::new(s_630_4 as u128, 5u16);
        // D s_630_6: cmp-ne s_630_3 s_630_5
        let s_630_6: bool = ((s_630_3) != (s_630_5));
        // D s_630_7: write-var gs#140418 <= s_630_6
        fn_state.gs_140418 = s_630_6;
        // N s_630_8: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_631<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_631_0: const #1s : i
        let s_631_0: i128 = 1;
        // D s_631_1: read-var op2:i
        let s_631_1: i128 = fn_state.op2;
        // D s_631_2: cmp-eq s_631_1 s_631_0
        let s_631_2: bool = ((s_631_1) == (s_631_0));
        // N s_631_3: branch s_631_2 b634 b632
        if s_631_2 {
            return block_634(state, tracer, fn_state);
        } else {
            return block_632(state, tracer, fn_state);
        };
    }
    fn block_632<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_632_0: const #2s : i
        let s_632_0: i128 = 2;
        // D s_632_1: read-var op2:i
        let s_632_1: i128 = fn_state.op2;
        // D s_632_2: cmp-eq s_632_1 s_632_0
        let s_632_2: bool = ((s_632_1) == (s_632_0));
        // D s_632_3: write-var gs#140416 <= s_632_2
        fn_state.gs_140416 = s_632_2;
        // N s_632_4: jump b633
        return block_633(state, tracer, fn_state);
    }
    fn block_633<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_633_0: read-var gs#140416:u8
        let s_633_0: bool = fn_state.gs_140416;
        // D s_633_1: write-var gs#140417 <= s_633_0
        fn_state.gs_140417 = s_633_0;
        // N s_633_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_634<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_634_0: const #1u : u8
        let s_634_0: bool = true;
        // D s_634_1: write-var gs#140416 <= s_634_0
        fn_state.gs_140416 = s_634_0;
        // N s_634_2: jump b633
        return block_633(state, tracer, fn_state);
    }
    fn block_635<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_635_0: const #13s : i
        let s_635_0: i128 = 13;
        // D s_635_1: read-var crm:i
        let s_635_1: i128 = fn_state.crm;
        // D s_635_2: cmp-eq s_635_1 s_635_0
        let s_635_2: bool = ((s_635_1) == (s_635_0));
        // D s_635_3: write-var gs#140413 <= s_635_2
        fn_state.gs_140413 = s_635_2;
        // N s_635_4: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_636<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_636_0: const #9s : i
        let s_636_0: i128 = 9;
        // D s_636_1: read-var crn:i
        let s_636_1: i128 = fn_state.crn;
        // D s_636_2: cmp-eq s_636_1 s_636_0
        let s_636_2: bool = ((s_636_1) == (s_636_0));
        // D s_636_3: write-var gs#140411 <= s_636_2
        fn_state.gs_140411 = s_636_2;
        // N s_636_4: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_637<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_637_0: const #3s : i
        let s_637_0: i128 = 3;
        // D s_637_1: read-var op1:i
        let s_637_1: i128 = fn_state.op1;
        // D s_637_2: cmp-eq s_637_1 s_637_0
        let s_637_2: bool = ((s_637_1) == (s_637_0));
        // D s_637_3: write-var gs#140409 <= s_637_2
        fn_state.gs_140409 = s_637_2;
        // N s_637_4: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_638<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_638_0: const #1s : i
        let s_638_0: i128 = 1;
        // C s_638_1: const #0s : i
        let s_638_1: i128 = 0;
        // D s_638_2: read-var crm:i
        let s_638_2: i128 = fn_state.crm;
        // D s_638_3: call integer_subrange(s_638_2, s_638_0, s_638_1)
        let s_638_3: Bits = integer_subrange(state, tracer, s_638_2, s_638_0, s_638_1);
        // D s_638_4: cast reint s_638_3 -> u8
        let s_638_4: u8 = (s_638_3.value() as u8);
        // C s_638_5: const #2s : i
        let s_638_5: i128 = 2;
        // C s_638_6: const #0s : i
        let s_638_6: i128 = 0;
        // D s_638_7: read-var op2:i
        let s_638_7: i128 = fn_state.op2;
        // D s_638_8: call integer_subrange(s_638_7, s_638_5, s_638_6)
        let s_638_8: Bits = integer_subrange(state, tracer, s_638_7, s_638_5, s_638_6);
        // D s_638_9: cast reint s_638_8 -> u8
        let s_638_9: u8 = (s_638_8.value() as u8);
        // D s_638_10: cast zx s_638_4 -> bv
        let s_638_10: Bits = Bits::new(s_638_4 as u128, 2u16);
        // D s_638_11: cast zx s_638_9 -> bv
        let s_638_11: Bits = Bits::new(s_638_9 as u128, 3u16);
        // D s_638_12: cast reint s_638_10 -> u128
        let s_638_12: u128 = (s_638_10.value() as u128);
        // D s_638_13: size-of s_638_10
        let s_638_13: u16 = s_638_10.length();
        // D s_638_14: cast reint s_638_11 -> u128
        let s_638_14: u128 = (s_638_11.value() as u128);
        // D s_638_15: size-of s_638_11
        let s_638_15: u16 = s_638_11.length();
        // D s_638_16: lsl s_638_12 s_638_15
        let s_638_16: u128 = s_638_12 << s_638_15;
        // D s_638_17: or s_638_16 s_638_14
        let s_638_17: u128 = ((s_638_16) | (s_638_14));
        // D s_638_18: add s_638_13 s_638_15
        let s_638_18: u16 = (s_638_13 + s_638_15);
        // D s_638_19: create-bits s_638_17 s_638_18
        let s_638_19: Bits = Bits::new(s_638_17, s_638_18);
        // D s_638_20: cast reint s_638_19 -> u8
        let s_638_20: u8 = (s_638_19.value() as u8);
        // D s_638_21: cast zx s_638_20 -> bv
        let s_638_21: Bits = Bits::new(s_638_20 as u128, 5u16);
        // D s_638_22: cast zx s_638_21 -> i
        let s_638_22: i128 = (s_638_21.value() as i128);
        // D s_638_23: cast reint s_638_22 -> i64
        let s_638_23: i64 = (s_638_22 as i64);
        // C s_638_24: const #() : ()
        let s_638_24: () = ();
        // S s_638_25: call GetNumEventCounters(s_638_24)
        let s_638_25: i128 = GetNumEventCounters(state, tracer, s_638_24);
        // C s_638_26: const #1s : i
        let s_638_26: i128 = 1;
        // S s_638_27: sub s_638_25 s_638_26
        let s_638_27: i128 = ((s_638_25) - (s_638_26));
        // D s_638_28: cast zx s_638_23 -> i
        let s_638_28: i128 = (i128::try_from(s_638_23).unwrap());
        // D s_638_29: cmp-gt s_638_28 s_638_27
        let s_638_29: bool = ((s_638_28) > (s_638_27));
        // N s_638_30: branch s_638_29 b667 b639
        if s_638_29 {
            return block_667(state, tracer, fn_state);
        } else {
            return block_639(state, tracer, fn_state);
        };
    }
    fn block_639<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_639_0: const #() : ()
        let s_639_0: () = ();
        // S s_639_1: call EL2Enabled(s_639_0)
        let s_639_1: bool = EL2Enabled(state, tracer, s_639_0);
        // N s_639_2: branch s_639_1 b651 b640
        if s_639_1 {
            return block_651(state, tracer, fn_state);
        } else {
            return block_640(state, tracer, fn_state);
        };
    }
    fn block_640<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_640_0: const #0u : u8
        let s_640_0: bool = false;
        // D s_640_1: write-var gs#140449 <= s_640_0
        fn_state.gs_140449 = s_640_0;
        // N s_640_2: jump b641
        return block_641(state, tracer, fn_state);
    }
    fn block_641<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_641_0: read-var gs#140449:u8
        let s_641_0: bool = fn_state.gs_140449;
        // N s_641_1: branch s_641_0 b650 b642
        if s_641_0 {
            return block_650(state, tracer, fn_state);
        } else {
            return block_642(state, tracer, fn_state);
        };
    }
    fn block_642<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_642_0: const #0u : u8
        let s_642_0: bool = false;
        // D s_642_1: write-var gs#140455 <= s_642_0
        fn_state.gs_140455 = s_642_0;
        // N s_642_2: jump b643
        return block_643(state, tracer, fn_state);
    }
    fn block_643<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_643_0: read-var gs#140455:u8
        let s_643_0: bool = fn_state.gs_140455;
        // D s_643_1: write-var gs#140456 <= s_643_0
        fn_state.gs_140456 = s_643_0;
        // N s_643_2: jump b644
        return block_644(state, tracer, fn_state);
    }
    fn block_644<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_644_0: read-var gs#140456:u8
        let s_644_0: bool = fn_state.gs_140456;
        // N s_644_1: branch s_644_0 b647 b645
        if s_644_0 {
            return block_647(state, tracer, fn_state);
        } else {
            return block_645(state, tracer, fn_state);
        };
    }
    fn block_645<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_645_0: jump b646
        return block_646(state, tracer, fn_state);
    }
    fn block_646<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_646_0: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_647<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_647_0: const #1s : i
        let s_647_0: i128 = 1;
        // C s_647_1: const #0s : i
        let s_647_1: i128 = 0;
        // D s_647_2: read-var crm:i
        let s_647_2: i128 = fn_state.crm;
        // D s_647_3: call integer_subrange(s_647_2, s_647_0, s_647_1)
        let s_647_3: Bits = integer_subrange(state, tracer, s_647_2, s_647_0, s_647_1);
        // D s_647_4: cast reint s_647_3 -> u8
        let s_647_4: u8 = (s_647_3.value() as u8);
        // C s_647_5: const #2s : i
        let s_647_5: i128 = 2;
        // C s_647_6: const #0s : i
        let s_647_6: i128 = 0;
        // D s_647_7: read-var op2:i
        let s_647_7: i128 = fn_state.op2;
        // D s_647_8: call integer_subrange(s_647_7, s_647_5, s_647_6)
        let s_647_8: Bits = integer_subrange(state, tracer, s_647_7, s_647_5, s_647_6);
        // D s_647_9: cast reint s_647_8 -> u8
        let s_647_9: u8 = (s_647_8.value() as u8);
        // D s_647_10: cast zx s_647_4 -> bv
        let s_647_10: Bits = Bits::new(s_647_4 as u128, 2u16);
        // D s_647_11: cast zx s_647_9 -> bv
        let s_647_11: Bits = Bits::new(s_647_9 as u128, 3u16);
        // D s_647_12: cast reint s_647_10 -> u128
        let s_647_12: u128 = (s_647_10.value() as u128);
        // D s_647_13: size-of s_647_10
        let s_647_13: u16 = s_647_10.length();
        // D s_647_14: cast reint s_647_11 -> u128
        let s_647_14: u128 = (s_647_11.value() as u128);
        // D s_647_15: size-of s_647_11
        let s_647_15: u16 = s_647_11.length();
        // D s_647_16: lsl s_647_12 s_647_15
        let s_647_16: u128 = s_647_12 << s_647_15;
        // D s_647_17: or s_647_16 s_647_14
        let s_647_17: u128 = ((s_647_16) | (s_647_14));
        // D s_647_18: add s_647_13 s_647_15
        let s_647_18: u16 = (s_647_13 + s_647_15);
        // D s_647_19: create-bits s_647_17 s_647_18
        let s_647_19: Bits = Bits::new(s_647_17, s_647_18);
        // D s_647_20: cast reint s_647_19 -> u8
        let s_647_20: u8 = (s_647_19.value() as u8);
        // D s_647_21: cast zx s_647_20 -> bv
        let s_647_21: Bits = Bits::new(s_647_20 as u128, 5u16);
        // D s_647_22: cast zx s_647_21 -> i
        let s_647_22: i128 = (s_647_21.value() as i128);
        // D s_647_23: cast reint s_647_22 -> i64
        let s_647_23: i64 = (s_647_22 as i64);
        // C s_647_24: const #() : ()
        let s_647_24: () = ();
        // S s_647_25: call GetNumEventCounters(s_647_24)
        let s_647_25: i128 = GetNumEventCounters(state, tracer, s_647_24);
        // C s_647_26: const #1s : i
        let s_647_26: i128 = 1;
        // S s_647_27: sub s_647_25 s_647_26
        let s_647_27: i128 = ((s_647_25) - (s_647_26));
        // D s_647_28: cast zx s_647_23 -> i
        let s_647_28: i128 = (i128::try_from(s_647_23).unwrap());
        // D s_647_29: cmp-gt s_647_28 s_647_27
        let s_647_29: bool = ((s_647_28) > (s_647_27));
        // N s_647_30: branch s_647_29 b649 b648
        if s_647_29 {
            return block_649(state, tracer, fn_state);
        } else {
            return block_648(state, tracer, fn_state);
        };
    }
    fn block_648<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_648_0: const #24u : u8
        let s_648_0: u8 = 24;
        // C s_648_1: cast zx s_648_0 -> bv
        let s_648_1: Bits = Bits::new(s_648_0 as u128, 8u16);
        // C s_648_2: cast zx s_648_1 -> i
        let s_648_2: i128 = (s_648_1.value() as i128);
        // C s_648_3: cast reint s_648_2 -> i64
        let s_648_3: i64 = (s_648_2 as i64);
        // C s_648_4: cast zx s_648_3 -> i
        let s_648_4: i128 = (i128::try_from(s_648_3).unwrap());
        // C s_648_5: const #432u : u32
        let s_648_5: u32 = 432;
        // D s_648_6: read-reg s_648_5:u8
        let s_648_6: u8 = {
            let value = state.read_register::<u8>(s_648_5 as isize);
            tracer.read_register(s_648_5 as isize, value);
            value
        };
        // D s_648_7: call AArch64_SystemAccessTrap(s_648_6, s_648_4)
        let s_648_7: () = AArch64_SystemAccessTrap(state, tracer, s_648_6, s_648_4);
        // N s_648_8: jump b646
        return block_646(state, tracer, fn_state);
    }
    fn block_649<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_649_0: panic
        panic!("{:?}", ());
        // N s_649_1: return
        return;
    }
    fn block_650<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_650_0: const #1s : i
        let s_650_0: i128 = 1;
        // C s_650_1: const #0s : i
        let s_650_1: i128 = 0;
        // D s_650_2: read-var crm:i
        let s_650_2: i128 = fn_state.crm;
        // D s_650_3: call integer_subrange(s_650_2, s_650_0, s_650_1)
        let s_650_3: Bits = integer_subrange(state, tracer, s_650_2, s_650_0, s_650_1);
        // D s_650_4: cast reint s_650_3 -> u8
        let s_650_4: u8 = (s_650_3.value() as u8);
        // C s_650_5: const #2s : i
        let s_650_5: i128 = 2;
        // C s_650_6: const #0s : i
        let s_650_6: i128 = 0;
        // D s_650_7: read-var op2:i
        let s_650_7: i128 = fn_state.op2;
        // D s_650_8: call integer_subrange(s_650_7, s_650_5, s_650_6)
        let s_650_8: Bits = integer_subrange(state, tracer, s_650_7, s_650_5, s_650_6);
        // D s_650_9: cast reint s_650_8 -> u8
        let s_650_9: u8 = (s_650_8.value() as u8);
        // D s_650_10: cast zx s_650_4 -> bv
        let s_650_10: Bits = Bits::new(s_650_4 as u128, 2u16);
        // D s_650_11: cast zx s_650_9 -> bv
        let s_650_11: Bits = Bits::new(s_650_9 as u128, 3u16);
        // D s_650_12: cast reint s_650_10 -> u128
        let s_650_12: u128 = (s_650_10.value() as u128);
        // D s_650_13: size-of s_650_10
        let s_650_13: u16 = s_650_10.length();
        // D s_650_14: cast reint s_650_11 -> u128
        let s_650_14: u128 = (s_650_11.value() as u128);
        // D s_650_15: size-of s_650_11
        let s_650_15: u16 = s_650_11.length();
        // D s_650_16: lsl s_650_12 s_650_15
        let s_650_16: u128 = s_650_12 << s_650_15;
        // D s_650_17: or s_650_16 s_650_14
        let s_650_17: u128 = ((s_650_16) | (s_650_14));
        // D s_650_18: add s_650_13 s_650_15
        let s_650_18: u16 = (s_650_13 + s_650_15);
        // D s_650_19: create-bits s_650_17 s_650_18
        let s_650_19: Bits = Bits::new(s_650_17, s_650_18);
        // D s_650_20: cast reint s_650_19 -> u8
        let s_650_20: u8 = (s_650_19.value() as u8);
        // D s_650_21: cast zx s_650_20 -> bv
        let s_650_21: Bits = Bits::new(s_650_20 as u128, 5u16);
        // D s_650_22: cast zx s_650_21 -> i
        let s_650_22: i128 = (s_650_21.value() as i128);
        // D s_650_23: cast reint s_650_22 -> i64
        let s_650_23: i64 = (s_650_22 as i64);
        // C s_650_24: const #() : ()
        let s_650_24: () = ();
        // S s_650_25: call AArch64_GetNumEventCountersAccessible(s_650_24)
        let s_650_25: i128 = AArch64_GetNumEventCountersAccessible(
            state,
            tracer,
            s_650_24,
        );
        // C s_650_26: const #1s : i
        let s_650_26: i128 = 1;
        // S s_650_27: sub s_650_25 s_650_26
        let s_650_27: i128 = ((s_650_25) - (s_650_26));
        // D s_650_28: cast zx s_650_23 -> i
        let s_650_28: i128 = (i128::try_from(s_650_23).unwrap());
        // D s_650_29: cmp-gt s_650_28 s_650_27
        let s_650_29: bool = ((s_650_28) > (s_650_27));
        // D s_650_30: write-var gs#140455 <= s_650_29
        fn_state.gs_140455 = s_650_29;
        // N s_650_31: jump b643
        return block_643(state, tracer, fn_state);
    }
    fn block_651<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_651_0: const #16975u : u32
        let s_651_0: u32 = 16975;
        // D s_651_1: read-reg s_651_0:u8
        let s_651_1: u8 = {
            let value = state.read_register::<u8>(s_651_0 as isize);
            tracer.read_register(s_651_0 as isize, value);
            value
        };
        // D s_651_2: cast zx s_651_1 -> bv
        let s_651_2: Bits = Bits::new(s_651_1 as u128, 2u16);
        // C s_651_3: const #440u : u32
        let s_651_3: u32 = 440;
        // D s_651_4: read-reg s_651_3:u8
        let s_651_4: u8 = {
            let value = state.read_register::<u8>(s_651_3 as isize);
            tracer.read_register(s_651_3 as isize, value);
            value
        };
        // D s_651_5: cast zx s_651_4 -> bv
        let s_651_5: Bits = Bits::new(s_651_4 as u128, 2u16);
        // D s_651_6: cmp-eq s_651_2 s_651_5
        let s_651_6: bool = ((s_651_2) == (s_651_5));
        // N s_651_7: branch s_651_6 b666 b652
        if s_651_6 {
            return block_666(state, tracer, fn_state);
        } else {
            return block_652(state, tracer, fn_state);
        };
    }
    fn block_652<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_652_0: const #16975u : u32
        let s_652_0: u32 = 16975;
        // D s_652_1: read-reg s_652_0:u8
        let s_652_1: u8 = {
            let value = state.read_register::<u8>(s_652_0 as isize);
            tracer.read_register(s_652_0 as isize, value);
            value
        };
        // D s_652_2: cast zx s_652_1 -> bv
        let s_652_2: Bits = Bits::new(s_652_1 as u128, 2u16);
        // C s_652_3: const #448u : u32
        let s_652_3: u32 = 448;
        // D s_652_4: read-reg s_652_3:u8
        let s_652_4: u8 = {
            let value = state.read_register::<u8>(s_652_3 as isize);
            tracer.read_register(s_652_3 as isize, value);
            value
        };
        // D s_652_5: cast zx s_652_4 -> bv
        let s_652_5: Bits = Bits::new(s_652_4 as u128, 2u16);
        // D s_652_6: cmp-eq s_652_2 s_652_5
        let s_652_6: bool = ((s_652_2) == (s_652_5));
        // N s_652_7: branch s_652_6 b656 b653
        if s_652_6 {
            return block_656(state, tracer, fn_state);
        } else {
            return block_653(state, tracer, fn_state);
        };
    }
    fn block_653<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_653_0: const #0u : u8
        let s_653_0: bool = false;
        // D s_653_1: write-var gs#140447 <= s_653_0
        fn_state.gs_140447 = s_653_0;
        // N s_653_2: jump b654
        return block_654(state, tracer, fn_state);
    }
    fn block_654<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_654_0: read-var gs#140447:u8
        let s_654_0: bool = fn_state.gs_140447;
        // D s_654_1: write-var gs#140448 <= s_654_0
        fn_state.gs_140448 = s_654_0;
        // N s_654_2: jump b655
        return block_655(state, tracer, fn_state);
    }
    fn block_655<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_655_0: read-var gs#140448:u8
        let s_655_0: bool = fn_state.gs_140448;
        // D s_655_1: write-var gs#140449 <= s_655_0
        fn_state.gs_140449 = s_655_0;
        // N s_655_2: jump b641
        return block_641(state, tracer, fn_state);
    }
    fn block_656<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_656_0: const #3s : i
        let s_656_0: i128 = 3;
        // C s_656_1: const #2s : i
        let s_656_1: i128 = 2;
        // D s_656_2: read-var crm:i
        let s_656_2: i128 = fn_state.crm;
        // D s_656_3: call integer_subrange(s_656_2, s_656_0, s_656_1)
        let s_656_3: Bits = integer_subrange(state, tracer, s_656_2, s_656_0, s_656_1);
        // D s_656_4: cast reint s_656_3 -> u8
        let s_656_4: u8 = (s_656_3.value() as u8);
        // D s_656_5: cast zx s_656_4 -> bv
        let s_656_5: Bits = Bits::new(s_656_4 as u128, 2u16);
        // C s_656_6: const #2u : u8
        let s_656_6: u8 = 2;
        // C s_656_7: cast zx s_656_6 -> bv
        let s_656_7: Bits = Bits::new(s_656_6 as u128, 2u16);
        // D s_656_8: cmp-eq s_656_5 s_656_7
        let s_656_8: bool = ((s_656_5) == (s_656_7));
        // N s_656_9: branch s_656_8 b665 b657
        if s_656_8 {
            return block_665(state, tracer, fn_state);
        } else {
            return block_657(state, tracer, fn_state);
        };
    }
    fn block_657<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_657_0: const #0u : u8
        let s_657_0: bool = false;
        // D s_657_1: write-var gs#140442 <= s_657_0
        fn_state.gs_140442 = s_657_0;
        // N s_657_2: jump b658
        return block_658(state, tracer, fn_state);
    }
    fn block_658<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_658_0: read-var gs#140442:u8
        let s_658_0: bool = fn_state.gs_140442;
        // N s_658_1: branch s_658_0 b664 b659
        if s_658_0 {
            return block_664(state, tracer, fn_state);
        } else {
            return block_659(state, tracer, fn_state);
        };
    }
    fn block_659<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_659_0: const #3s : i
        let s_659_0: i128 = 3;
        // C s_659_1: const #2s : i
        let s_659_1: i128 = 2;
        // D s_659_2: read-var crm:i
        let s_659_2: i128 = fn_state.crm;
        // D s_659_3: call integer_subrange(s_659_2, s_659_0, s_659_1)
        let s_659_3: Bits = integer_subrange(state, tracer, s_659_2, s_659_0, s_659_1);
        // D s_659_4: cast reint s_659_3 -> u8
        let s_659_4: u8 = (s_659_3.value() as u8);
        // D s_659_5: cast zx s_659_4 -> bv
        let s_659_5: Bits = Bits::new(s_659_4 as u128, 2u16);
        // C s_659_6: const #3u : u8
        let s_659_6: u8 = 3;
        // C s_659_7: cast zx s_659_6 -> bv
        let s_659_7: Bits = Bits::new(s_659_6 as u128, 2u16);
        // D s_659_8: cmp-eq s_659_5 s_659_7
        let s_659_8: bool = ((s_659_5) == (s_659_7));
        // N s_659_9: branch s_659_8 b663 b660
        if s_659_8 {
            return block_663(state, tracer, fn_state);
        } else {
            return block_660(state, tracer, fn_state);
        };
    }
    fn block_660<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_660_0: const #0u : u8
        let s_660_0: bool = false;
        // D s_660_1: write-var gs#140445 <= s_660_0
        fn_state.gs_140445 = s_660_0;
        // N s_660_2: jump b661
        return block_661(state, tracer, fn_state);
    }
    fn block_661<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_661_0: read-var gs#140445:u8
        let s_661_0: bool = fn_state.gs_140445;
        // D s_661_1: write-var gs#140446 <= s_661_0
        fn_state.gs_140446 = s_661_0;
        // N s_661_2: jump b662
        return block_662(state, tracer, fn_state);
    }
    fn block_662<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_662_0: read-var gs#140446:u8
        let s_662_0: bool = fn_state.gs_140446;
        // D s_662_1: write-var gs#140447 <= s_662_0
        fn_state.gs_140447 = s_662_0;
        // N s_662_2: jump b654
        return block_654(state, tracer, fn_state);
    }
    fn block_663<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_663_0: const #102624u : u32
        let s_663_0: u32 = 102624;
        // D s_663_1: read-reg s_663_0:struct
        let s_663_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_663_0 as isize);
            tracer.read_register(s_663_0 as isize, value);
            value
        };
        // D s_663_2: call _get_PMUSERENR_EL0_Type_EN(s_663_1)
        let s_663_2: bool = u_get_PMUSERENR_EL0_Type_EN(state, tracer, s_663_1);
        // D s_663_3: cast zx s_663_2 -> bv
        let s_663_3: Bits = Bits::new(s_663_2 as u128, 1u16);
        // C s_663_4: const #1u : u8
        let s_663_4: bool = true;
        // C s_663_5: cast zx s_663_4 -> bv
        let s_663_5: Bits = Bits::new(s_663_4 as u128, 1u16);
        // D s_663_6: cmp-eq s_663_3 s_663_5
        let s_663_6: bool = ((s_663_3) == (s_663_5));
        // D s_663_7: write-var gs#140445 <= s_663_6
        fn_state.gs_140445 = s_663_6;
        // N s_663_8: jump b661
        return block_661(state, tracer, fn_state);
    }
    fn block_664<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_664_0: const #1u : u8
        let s_664_0: bool = true;
        // D s_664_1: write-var gs#140446 <= s_664_0
        fn_state.gs_140446 = s_664_0;
        // N s_664_2: jump b662
        return block_662(state, tracer, fn_state);
    }
    fn block_665<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_665_0: const #102624u : u32
        let s_665_0: u32 = 102624;
        // D s_665_1: read-reg s_665_0:struct
        let s_665_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_665_0 as isize);
            tracer.read_register(s_665_0 as isize, value);
            value
        };
        // D s_665_2: call _get_PMUSERENR_EL0_Type_ER(s_665_1)
        let s_665_2: bool = u_get_PMUSERENR_EL0_Type_ER(state, tracer, s_665_1);
        // C s_665_3: const #102624u : u32
        let s_665_3: u32 = 102624;
        // D s_665_4: read-reg s_665_3:struct
        let s_665_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_665_3 as isize);
            tracer.read_register(s_665_3 as isize, value);
            value
        };
        // D s_665_5: call _get_PMUSERENR_EL0_Type_EN(s_665_4)
        let s_665_5: bool = u_get_PMUSERENR_EL0_Type_EN(state, tracer, s_665_4);
        // D s_665_6: cast zx s_665_2 -> bv
        let s_665_6: Bits = Bits::new(s_665_2 as u128, 1u16);
        // D s_665_7: cast zx s_665_5 -> bv
        let s_665_7: Bits = Bits::new(s_665_5 as u128, 1u16);
        // D s_665_8: cast reint s_665_6 -> u128
        let s_665_8: u128 = (s_665_6.value() as u128);
        // D s_665_9: size-of s_665_6
        let s_665_9: u16 = s_665_6.length();
        // D s_665_10: cast reint s_665_7 -> u128
        let s_665_10: u128 = (s_665_7.value() as u128);
        // D s_665_11: size-of s_665_7
        let s_665_11: u16 = s_665_7.length();
        // D s_665_12: lsl s_665_8 s_665_11
        let s_665_12: u128 = s_665_8 << s_665_11;
        // D s_665_13: or s_665_12 s_665_10
        let s_665_13: u128 = ((s_665_12) | (s_665_10));
        // D s_665_14: add s_665_9 s_665_11
        let s_665_14: u16 = (s_665_9 + s_665_11);
        // D s_665_15: create-bits s_665_13 s_665_14
        let s_665_15: Bits = Bits::new(s_665_13, s_665_14);
        // D s_665_16: cast reint s_665_15 -> u8
        let s_665_16: u8 = (s_665_15.value() as u8);
        // D s_665_17: cast zx s_665_16 -> bv
        let s_665_17: Bits = Bits::new(s_665_16 as u128, 2u16);
        // C s_665_18: const #0u : u8
        let s_665_18: u8 = 0;
        // C s_665_19: cast zx s_665_18 -> bv
        let s_665_19: Bits = Bits::new(s_665_18 as u128, 2u16);
        // D s_665_20: cmp-ne s_665_17 s_665_19
        let s_665_20: bool = ((s_665_17) != (s_665_19));
        // D s_665_21: write-var gs#140442 <= s_665_20
        fn_state.gs_140442 = s_665_20;
        // N s_665_22: jump b658
        return block_658(state, tracer, fn_state);
    }
    fn block_666<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_666_0: const #1u : u8
        let s_666_0: bool = true;
        // D s_666_1: write-var gs#140448 <= s_666_0
        fn_state.gs_140448 = s_666_0;
        // N s_666_2: jump b655
        return block_655(state, tracer, fn_state);
    }
    fn block_667<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_667_0: const #1u : u8
        let s_667_0: bool = true;
        // D s_667_1: write-var gs#140456 <= s_667_0
        fn_state.gs_140456 = s_667_0;
        // N s_667_2: jump b644
        return block_644(state, tracer, fn_state);
    }
    fn block_668<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_668_0: const #1s : i
        let s_668_0: i128 = 1;
        // C s_668_1: const #0s : i
        let s_668_1: i128 = 0;
        // D s_668_2: read-var crm:i
        let s_668_2: i128 = fn_state.crm;
        // D s_668_3: call integer_subrange(s_668_2, s_668_0, s_668_1)
        let s_668_3: Bits = integer_subrange(state, tracer, s_668_2, s_668_0, s_668_1);
        // D s_668_4: cast reint s_668_3 -> u8
        let s_668_4: u8 = (s_668_3.value() as u8);
        // C s_668_5: const #2s : i
        let s_668_5: i128 = 2;
        // C s_668_6: const #0s : i
        let s_668_6: i128 = 0;
        // D s_668_7: read-var op2:i
        let s_668_7: i128 = fn_state.op2;
        // D s_668_8: call integer_subrange(s_668_7, s_668_5, s_668_6)
        let s_668_8: Bits = integer_subrange(state, tracer, s_668_7, s_668_5, s_668_6);
        // D s_668_9: cast reint s_668_8 -> u8
        let s_668_9: u8 = (s_668_8.value() as u8);
        // D s_668_10: cast zx s_668_4 -> bv
        let s_668_10: Bits = Bits::new(s_668_4 as u128, 2u16);
        // D s_668_11: cast zx s_668_9 -> bv
        let s_668_11: Bits = Bits::new(s_668_9 as u128, 3u16);
        // D s_668_12: cast reint s_668_10 -> u128
        let s_668_12: u128 = (s_668_10.value() as u128);
        // D s_668_13: size-of s_668_10
        let s_668_13: u16 = s_668_10.length();
        // D s_668_14: cast reint s_668_11 -> u128
        let s_668_14: u128 = (s_668_11.value() as u128);
        // D s_668_15: size-of s_668_11
        let s_668_15: u16 = s_668_11.length();
        // D s_668_16: lsl s_668_12 s_668_15
        let s_668_16: u128 = s_668_12 << s_668_15;
        // D s_668_17: or s_668_16 s_668_14
        let s_668_17: u128 = ((s_668_16) | (s_668_14));
        // D s_668_18: add s_668_13 s_668_15
        let s_668_18: u16 = (s_668_13 + s_668_15);
        // D s_668_19: create-bits s_668_17 s_668_18
        let s_668_19: Bits = Bits::new(s_668_17, s_668_18);
        // D s_668_20: cast reint s_668_19 -> u8
        let s_668_20: u8 = (s_668_19.value() as u8);
        // D s_668_21: cast zx s_668_20 -> bv
        let s_668_21: Bits = Bits::new(s_668_20 as u128, 5u16);
        // C s_668_22: const #31u : u8
        let s_668_22: u8 = 31;
        // C s_668_23: cast zx s_668_22 -> bv
        let s_668_23: Bits = Bits::new(s_668_22 as u128, 5u16);
        // D s_668_24: cmp-ne s_668_21 s_668_23
        let s_668_24: bool = ((s_668_21) != (s_668_23));
        // D s_668_25: write-var gs#140406 <= s_668_24
        fn_state.gs_140406 = s_668_24;
        // N s_668_26: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_669<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_669_0: const #3s : i
        let s_669_0: i128 = 3;
        // C s_669_1: const #2s : i
        let s_669_1: i128 = 2;
        // D s_669_2: read-var crm:i
        let s_669_2: i128 = fn_state.crm;
        // D s_669_3: call integer_subrange(s_669_2, s_669_0, s_669_1)
        let s_669_3: Bits = integer_subrange(state, tracer, s_669_2, s_669_0, s_669_1);
        // D s_669_4: cast reint s_669_3 -> u8
        let s_669_4: u8 = (s_669_3.value() as u8);
        // D s_669_5: cast zx s_669_4 -> bv
        let s_669_5: Bits = Bits::new(s_669_4 as u128, 2u16);
        // C s_669_6: const #2u : u8
        let s_669_6: u8 = 2;
        // C s_669_7: cast zx s_669_6 -> bv
        let s_669_7: Bits = Bits::new(s_669_6 as u128, 2u16);
        // D s_669_8: cmp-eq s_669_5 s_669_7
        let s_669_8: bool = ((s_669_5) == (s_669_7));
        // N s_669_9: branch s_669_8 b672 b670
        if s_669_8 {
            return block_672(state, tracer, fn_state);
        } else {
            return block_670(state, tracer, fn_state);
        };
    }
    fn block_670<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_670_0: const #3s : i
        let s_670_0: i128 = 3;
        // C s_670_1: const #2s : i
        let s_670_1: i128 = 2;
        // D s_670_2: read-var crm:i
        let s_670_2: i128 = fn_state.crm;
        // D s_670_3: call integer_subrange(s_670_2, s_670_0, s_670_1)
        let s_670_3: Bits = integer_subrange(state, tracer, s_670_2, s_670_0, s_670_1);
        // D s_670_4: cast reint s_670_3 -> u8
        let s_670_4: u8 = (s_670_3.value() as u8);
        // D s_670_5: cast zx s_670_4 -> bv
        let s_670_5: Bits = Bits::new(s_670_4 as u128, 2u16);
        // C s_670_6: const #3u : u8
        let s_670_6: u8 = 3;
        // C s_670_7: cast zx s_670_6 -> bv
        let s_670_7: Bits = Bits::new(s_670_6 as u128, 2u16);
        // D s_670_8: cmp-eq s_670_5 s_670_7
        let s_670_8: bool = ((s_670_5) == (s_670_7));
        // D s_670_9: write-var gs#140400 <= s_670_8
        fn_state.gs_140400 = s_670_8;
        // N s_670_10: jump b671
        return block_671(state, tracer, fn_state);
    }
    fn block_671<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_671_0: read-var gs#140400:u8
        let s_671_0: bool = fn_state.gs_140400;
        // D s_671_1: write-var gs#140401 <= s_671_0
        fn_state.gs_140401 = s_671_0;
        // N s_671_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_672<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_672_0: const #1u : u8
        let s_672_0: bool = true;
        // D s_672_1: write-var gs#140400 <= s_672_0
        fn_state.gs_140400 = s_672_0;
        // N s_672_2: jump b671
        return block_671(state, tracer, fn_state);
    }
    fn block_673<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_673_0: const #14s : i
        let s_673_0: i128 = 14;
        // D s_673_1: read-var crn:i
        let s_673_1: i128 = fn_state.crn;
        // D s_673_2: cmp-eq s_673_1 s_673_0
        let s_673_2: bool = ((s_673_1) == (s_673_0));
        // D s_673_3: write-var gs#140395 <= s_673_2
        fn_state.gs_140395 = s_673_2;
        // N s_673_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_674<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_674_0: const #3s : i
        let s_674_0: i128 = 3;
        // D s_674_1: read-var op1:i
        let s_674_1: i128 = fn_state.op1;
        // D s_674_2: cmp-eq s_674_1 s_674_0
        let s_674_2: bool = ((s_674_1) == (s_674_0));
        // D s_674_3: write-var gs#140393 <= s_674_2
        fn_state.gs_140393 = s_674_2;
        // N s_674_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_675<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_675_0: const #0s : i
        let s_675_0: i128 = 0;
        // D s_675_1: read-var crm:i
        let s_675_1: i128 = fn_state.crm;
        // D s_675_2: call integer_access(s_675_1, s_675_0)
        let s_675_2: bool = integer_access(state, tracer, s_675_1, s_675_0);
        // C s_675_3: const #0s : i
        let s_675_3: i128 = 0;
        // C s_675_4: const #0u : u64
        let s_675_4: u64 = 0;
        // D s_675_5: cast zx s_675_2 -> u64
        let s_675_5: u64 = (s_675_2 as u64);
        // C s_675_6: const #1u : u64
        let s_675_6: u64 = 1;
        // D s_675_7: and s_675_5 s_675_6
        let s_675_7: u64 = ((s_675_5) & (s_675_6));
        // D s_675_8: cmp-eq s_675_7 s_675_6
        let s_675_8: bool = ((s_675_7) == (s_675_6));
        // D s_675_9: lsl s_675_5 s_675_3
        let s_675_9: u64 = s_675_5 << s_675_3;
        // D s_675_10: or s_675_4 s_675_9
        let s_675_10: u64 = ((s_675_4) | (s_675_9));
        // D s_675_11: cmpl s_675_9
        let s_675_11: u64 = !s_675_9;
        // D s_675_12: and s_675_4 s_675_11
        let s_675_12: u64 = ((s_675_4) & (s_675_11));
        // D s_675_13: select s_675_8 s_675_10 s_675_12
        let s_675_13: u64 = if s_675_8 { s_675_10 } else { s_675_12 };
        // D s_675_14: cast trunc s_675_13 -> u8
        let s_675_14: bool = ((s_675_13) != 0);
        // C s_675_15: const #2s : i
        let s_675_15: i128 = 2;
        // C s_675_16: const #0s : i
        let s_675_16: i128 = 0;
        // D s_675_17: read-var op2:i
        let s_675_17: i128 = fn_state.op2;
        // D s_675_18: call integer_subrange(s_675_17, s_675_15, s_675_16)
        let s_675_18: Bits = integer_subrange(
            state,
            tracer,
            s_675_17,
            s_675_15,
            s_675_16,
        );
        // D s_675_19: cast reint s_675_18 -> u8
        let s_675_19: u8 = (s_675_18.value() as u8);
        // D s_675_20: cast zx s_675_14 -> bv
        let s_675_20: Bits = Bits::new(s_675_14 as u128, 1u16);
        // D s_675_21: cast zx s_675_19 -> bv
        let s_675_21: Bits = Bits::new(s_675_19 as u128, 3u16);
        // D s_675_22: cast reint s_675_20 -> u128
        let s_675_22: u128 = (s_675_20.value() as u128);
        // D s_675_23: size-of s_675_20
        let s_675_23: u16 = s_675_20.length();
        // D s_675_24: cast reint s_675_21 -> u128
        let s_675_24: u128 = (s_675_21.value() as u128);
        // D s_675_25: size-of s_675_21
        let s_675_25: u16 = s_675_21.length();
        // D s_675_26: lsl s_675_22 s_675_25
        let s_675_26: u128 = s_675_22 << s_675_25;
        // D s_675_27: or s_675_26 s_675_24
        let s_675_27: u128 = ((s_675_26) | (s_675_24));
        // D s_675_28: add s_675_23 s_675_25
        let s_675_28: u16 = (s_675_23 + s_675_25);
        // D s_675_29: create-bits s_675_27 s_675_28
        let s_675_29: Bits = Bits::new(s_675_27, s_675_28);
        // D s_675_30: cast reint s_675_29 -> u8
        let s_675_30: u8 = (s_675_29.value() as u8);
        // D s_675_31: cast zx s_675_30 -> bv
        let s_675_31: Bits = Bits::new(s_675_30 as u128, 4u16);
        // D s_675_32: cast zx s_675_31 -> i
        let s_675_32: i128 = (s_675_31.value() as i128);
        // D s_675_33: cast reint s_675_32 -> i64
        let s_675_33: i64 = (s_675_32 as i64);
        // D s_675_34: write-var nshadow#1007 <= s_675_33
        fn_state.nshadow_1007 = s_675_33;
        // C s_675_35: const #3s : i
        let s_675_35: i128 = 3;
        // C s_675_36: const #1s : i
        let s_675_36: i128 = 1;
        // D s_675_37: read-var crm:i
        let s_675_37: i128 = fn_state.crm;
        // D s_675_38: call integer_subrange(s_675_37, s_675_35, s_675_36)
        let s_675_38: Bits = integer_subrange(
            state,
            tracer,
            s_675_37,
            s_675_35,
            s_675_36,
        );
        // D s_675_39: cast reint s_675_38 -> u8
        let s_675_39: u8 = (s_675_38.value() as u8);
        // D s_675_40: cast zx s_675_39 -> bv
        let s_675_40: Bits = Bits::new(s_675_39 as u128, 3u16);
        // C s_675_41: const #2u : u8
        let s_675_41: u8 = 2;
        // C s_675_42: cast zx s_675_41 -> bv
        let s_675_42: Bits = Bits::new(s_675_41 as u128, 3u16);
        // D s_675_43: cmp-eq s_675_40 s_675_42
        let s_675_43: bool = ((s_675_40) == (s_675_42));
        // N s_675_44: branch s_675_43 b690 b676
        if s_675_43 {
            return block_690(state, tracer, fn_state);
        } else {
            return block_676(state, tracer, fn_state);
        };
    }
    fn block_676<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_676_0: const #3s : i
        let s_676_0: i128 = 3;
        // C s_676_1: const #1s : i
        let s_676_1: i128 = 1;
        // D s_676_2: read-var crm:i
        let s_676_2: i128 = fn_state.crm;
        // D s_676_3: call integer_subrange(s_676_2, s_676_0, s_676_1)
        let s_676_3: Bits = integer_subrange(state, tracer, s_676_2, s_676_0, s_676_1);
        // D s_676_4: cast reint s_676_3 -> u8
        let s_676_4: u8 = (s_676_3.value() as u8);
        // D s_676_5: cast zx s_676_4 -> bv
        let s_676_5: Bits = Bits::new(s_676_4 as u128, 3u16);
        // C s_676_6: const #3u : u8
        let s_676_6: u8 = 3;
        // C s_676_7: cast zx s_676_6 -> bv
        let s_676_7: Bits = Bits::new(s_676_6 as u128, 3u16);
        // D s_676_8: cmp-eq s_676_5 s_676_7
        let s_676_8: bool = ((s_676_5) == (s_676_7));
        // D s_676_9: write-var gs#140473 <= s_676_8
        fn_state.gs_140473 = s_676_8;
        // N s_676_10: jump b677
        return block_677(state, tracer, fn_state);
    }
    fn block_677<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_677_0: read-var gs#140473:u8
        let s_677_0: bool = fn_state.gs_140473;
        // N s_677_1: branch s_677_0 b687 b678
        if s_677_0 {
            return block_687(state, tracer, fn_state);
        } else {
            return block_678(state, tracer, fn_state);
        };
    }
    fn block_678<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_678_0: const #3s : i
        let s_678_0: i128 = 3;
        // C s_678_1: const #1s : i
        let s_678_1: i128 = 1;
        // D s_678_2: read-var crm:i
        let s_678_2: i128 = fn_state.crm;
        // D s_678_3: call integer_subrange(s_678_2, s_678_0, s_678_1)
        let s_678_3: Bits = integer_subrange(state, tracer, s_678_2, s_678_0, s_678_1);
        // D s_678_4: cast reint s_678_3 -> u8
        let s_678_4: u8 = (s_678_3.value() as u8);
        // D s_678_5: cast zx s_678_4 -> bv
        let s_678_5: Bits = Bits::new(s_678_4 as u128, 3u16);
        // C s_678_6: const #6u : u8
        let s_678_6: u8 = 6;
        // C s_678_7: cast zx s_678_6 -> bv
        let s_678_7: Bits = Bits::new(s_678_6 as u128, 3u16);
        // D s_678_8: cmp-eq s_678_5 s_678_7
        let s_678_8: bool = ((s_678_5) == (s_678_7));
        // N s_678_9: branch s_678_8 b686 b679
        if s_678_8 {
            return block_686(state, tracer, fn_state);
        } else {
            return block_679(state, tracer, fn_state);
        };
    }
    fn block_679<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_679_0: const #3s : i
        let s_679_0: i128 = 3;
        // C s_679_1: const #1s : i
        let s_679_1: i128 = 1;
        // D s_679_2: read-var crm:i
        let s_679_2: i128 = fn_state.crm;
        // D s_679_3: call integer_subrange(s_679_2, s_679_0, s_679_1)
        let s_679_3: Bits = integer_subrange(state, tracer, s_679_2, s_679_0, s_679_1);
        // D s_679_4: cast reint s_679_3 -> u8
        let s_679_4: u8 = (s_679_3.value() as u8);
        // D s_679_5: cast zx s_679_4 -> bv
        let s_679_5: Bits = Bits::new(s_679_4 as u128, 3u16);
        // C s_679_6: const #7u : u8
        let s_679_6: u8 = 7;
        // C s_679_7: cast zx s_679_6 -> bv
        let s_679_7: Bits = Bits::new(s_679_6 as u128, 3u16);
        // D s_679_8: cmp-eq s_679_5 s_679_7
        let s_679_8: bool = ((s_679_5) == (s_679_7));
        // D s_679_9: write-var gs#140479 <= s_679_8
        fn_state.gs_140479 = s_679_8;
        // N s_679_10: jump b680
        return block_680(state, tracer, fn_state);
    }
    fn block_680<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_680_0: read-var gs#140479:u8
        let s_680_0: bool = fn_state.gs_140479;
        // N s_680_1: branch s_680_0 b683 b681
        if s_680_0 {
            return block_683(state, tracer, fn_state);
        } else {
            return block_681(state, tracer, fn_state);
        };
    }
    fn block_681<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_681_0: jump b682
        return block_682(state, tracer, fn_state);
    }
    fn block_682<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_682_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_683<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_683_0: const #14656u : u32
        let s_683_0: u32 = 14656;
        // D s_683_1: read-reg s_683_0:struct
        let s_683_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_683_0 as isize);
            tracer.read_register(s_683_0 as isize, value);
            value
        };
        // D s_683_2: call _get_AMCGCR_EL0_Type_CG1NC(s_683_1)
        let s_683_2: u8 = u_get_AMCGCR_EL0_Type_CG1NC(state, tracer, s_683_1);
        // D s_683_3: cast zx s_683_2 -> bv
        let s_683_3: Bits = Bits::new(s_683_2 as u128, 8u16);
        // D s_683_4: cast zx s_683_3 -> i
        let s_683_4: i128 = (s_683_3.value() as i128);
        // D s_683_5: cast reint s_683_4 -> i64
        let s_683_5: i64 = (s_683_4 as i64);
        // D s_683_6: read-var nshadow#1007:i64
        let s_683_6: i64 = fn_state.nshadow_1007;
        // D s_683_7: cast zx s_683_6 -> i
        let s_683_7: i128 = (i128::try_from(s_683_6).unwrap());
        // D s_683_8: cast zx s_683_5 -> i
        let s_683_8: i128 = (i128::try_from(s_683_5).unwrap());
        // D s_683_9: cmp-ge s_683_7 s_683_8
        let s_683_9: bool = ((s_683_7) >= (s_683_8));
        // N s_683_10: branch s_683_9 b685 b684
        if s_683_9 {
            return block_685(state, tracer, fn_state);
        } else {
            return block_684(state, tracer, fn_state);
        };
    }
    fn block_684<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_684_0: jump b682
        return block_682(state, tracer, fn_state);
    }
    fn block_685<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_685_0: panic
        panic!("{:?}", ());
        // N s_685_1: return
        return;
    }
    fn block_686<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_686_0: const #1u : u8
        let s_686_0: bool = true;
        // D s_686_1: write-var gs#140479 <= s_686_0
        fn_state.gs_140479 = s_686_0;
        // N s_686_2: jump b680
        return block_680(state, tracer, fn_state);
    }
    fn block_687<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_687_0: const #14656u : u32
        let s_687_0: u32 = 14656;
        // D s_687_1: read-reg s_687_0:struct
        let s_687_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_687_0 as isize);
            tracer.read_register(s_687_0 as isize, value);
            value
        };
        // D s_687_2: call _get_AMCGCR_EL0_Type_CG0NC(s_687_1)
        let s_687_2: u8 = u_get_AMCGCR_EL0_Type_CG0NC(state, tracer, s_687_1);
        // D s_687_3: cast zx s_687_2 -> bv
        let s_687_3: Bits = Bits::new(s_687_2 as u128, 8u16);
        // D s_687_4: cast zx s_687_3 -> i
        let s_687_4: i128 = (s_687_3.value() as i128);
        // D s_687_5: cast reint s_687_4 -> i64
        let s_687_5: i64 = (s_687_4 as i64);
        // D s_687_6: read-var nshadow#1007:i64
        let s_687_6: i64 = fn_state.nshadow_1007;
        // D s_687_7: cast zx s_687_6 -> i
        let s_687_7: i128 = (i128::try_from(s_687_6).unwrap());
        // D s_687_8: cast zx s_687_5 -> i
        let s_687_8: i128 = (i128::try_from(s_687_5).unwrap());
        // D s_687_9: cmp-ge s_687_7 s_687_8
        let s_687_9: bool = ((s_687_7) >= (s_687_8));
        // N s_687_10: branch s_687_9 b689 b688
        if s_687_9 {
            return block_689(state, tracer, fn_state);
        } else {
            return block_688(state, tracer, fn_state);
        };
    }
    fn block_688<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_688_0: jump b682
        return block_682(state, tracer, fn_state);
    }
    fn block_689<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_689_0: panic
        panic!("{:?}", ());
        // N s_689_1: return
        return;
    }
    fn block_690<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_690_0: const #1u : u8
        let s_690_0: bool = true;
        // D s_690_1: write-var gs#140473 <= s_690_0
        fn_state.gs_140473 = s_690_0;
        // N s_690_2: jump b677
        return block_677(state, tracer, fn_state);
    }
    fn block_691<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_691_0: const #13s : i
        let s_691_0: i128 = 13;
        // D s_691_1: read-var crn:i
        let s_691_1: i128 = fn_state.crn;
        // D s_691_2: cmp-eq s_691_1 s_691_0
        let s_691_2: bool = ((s_691_1) == (s_691_0));
        // D s_691_3: write-var gs#140390 <= s_691_2
        fn_state.gs_140390 = s_691_2;
        // N s_691_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_692<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_692_0: const #3s : i
        let s_692_0: i128 = 3;
        // D s_692_1: read-var op1:i
        let s_692_1: i128 = fn_state.op1;
        // D s_692_2: cmp-eq s_692_1 s_692_0
        let s_692_2: bool = ((s_692_1) == (s_692_0));
        // D s_692_3: write-var gs#140388 <= s_692_2
        fn_state.gs_140388 = s_692_2;
        // N s_692_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
