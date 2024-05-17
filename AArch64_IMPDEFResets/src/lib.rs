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
use HaveDoubleFault2Ext::*;
use HavePMUv3TH::*;
use HaveRME::*;
use u_get_ID_AA64DFR0_EL1_Type_WRPs::*;
use HaveSVE2p1::*;
use u_get_ID_AA64PFR0_EL1_Type_EL3::*;
use Zeros::*;
use u_update_ICC_CTLR_EL1_Type_IDbits::*;
use u_get_ERRIDR_EL1_Type_NUM::*;
use u_get_ID_AA64DFR0_EL1_Type_PMUVer::*;
use u_get_ID_AA64DFR0_EL1_Type_CTX_CMPs::*;
use HaveFeatLS64_V::*;
use IsZero::*;
use Mk_PMCEID1_EL0_Type::*;
use HaveSME2p1::*;
use ICC_CTLR_EL1_read::*;
use HaveMTE4Ext::*;
use u_get_ID_DFR0_EL1_Type_CopDbg::*;
use EncodeVARange::*;
use HaveFeatLS64::*;
use u_get_ID_AA64DFR0_EL1_Type_BRPs::*;
use HaveFeatEBEP::*;
use IsFeatureImplemented::*;
use Mk_PMCEID0_EL0_Type::*;
use IsG1ActivityMonitorOffsetImplemented::*;
use Unreachable::*;
use HavePFAR::*;
use u__id::*;
use ICC_CTLR_EL1_write::*;
use integer_subrange::*;
use u_get_ID_AA64PFR0_EL1_Type_EL2::*;
use Bit::*;
use VMPIDR_read::*;
use Mk_CNTFID0_Type::*;
use u_get_ID_AA64PFR0_EL1_Type_EL1::*;
use HaveDoPD::*;
use HasArchVersion::*;
use u_update_ICC_CTLR_EL1_Type_ExtRange::*;
use HaveGCS::*;
use u_get_ID_AA64PFR0_EL1_Type_AdvSIMD::*;
use u_get_ID_AA64PFR0_EL1_Type_GIC::*;
use u_update_VMPIDR_Type_M::*;
use Mk_CCSIDR_EL1_Type::*;
use IsG1ActivityMonitorImplemented::*;
use u_update_ICC_CTLR_EL1_Type_PRIbits::*;
use Have52BitIPAAndPASpaceExt::*;
use VMPIDR_write::*;
use HaveFeatLS64_ACCDATA::*;
use u_get_PMCR_EL0_Type_N::*;
use HaveTHExt::*;
use EL2Enabled::*;
use u_update_ICC_CTLR_EL1_Type_A3V::*;
use EncodePARange::*;
use u_get_ID_AA64PFR0_EL1_Type_EL0::*;
use u_get_HCR_EL2_Type_E2H::*;
use HaveMTE2Ext::*;
use HaveMTEPermExt::*;
use HaveLSE128::*;
use HaveCNTSCExt::*;
use u_get_SCR_Type_NS::*;
use u_get_ID_AA64DFR0_EL1_Type_TraceVer::*;
use HaveMPAMExt::*;
use u_get_ID_AA64DFR0_EL1_Type_TraceFilt::*;
use HaveRASv2Ext::*;
use Mk_HCRX_EL2_Type::*;
use u_get_ID_AA64PFR1_EL1_Type_MPAM_frac::*;
use HaveStatisticalProfiling::*;
use HavePMUv3p4::*;
use u_get_PMCR_EL0_Type_IMP::*;
use u_get_ID_AA64PFR0_EL1_Type_MPAM::*;
use common::*;
pub fn AArch64_IMPDEFResets<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_328144: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_330418: bool,
        gs_328160: i64,
        gs_330101: bool,
        gs_328667: bool,
        ga_369678: i64,
        gs_328165: bool,
        gs_328666: bool,
        ga_369677: u64,
        i: i64,
        gs_330245: bool,
        gs_328784: bool,
        gs_330002: bool,
        gs_330417: bool,
        gs_330412: bool,
        gs_329443: bool,
        gs_328144: (),
    }
    let fn_state = FunctionState {
        gs_328144,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #102208u : u32
        let s_0_0: u32 = 102208;
        // D s_0_1: read-reg s_0_0:struct
        let s_0_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // C s_0_2: const #102208u : u32
        let s_0_2: u32 = 102208;
        // N s_0_3: write-reg s_0_2 <= s_0_1
        let s_0_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_2 as isize, s_0_1);
            tracer.write_register(s_0_2 as isize, s_0_1);
        };
        // C s_0_4: const #3s : i
        let s_0_4: i128 = 3;
        // C s_0_5: const #0s : i
        let s_0_5: i128 = 0;
        // C s_0_6: const #12952u : u32
        let s_0_6: u32 = 12952;
        // D s_0_7: read-reg s_0_6:i
        let s_0_7: i128 = {
            let value = state.read_register::<i128>(s_0_6 as isize);
            tracer.read_register(s_0_6 as isize, value);
            value
        };
        // D s_0_8: call integer_subrange(s_0_7, s_0_4, s_0_5)
        let s_0_8: Bits = integer_subrange(state, tracer, s_0_7, s_0_4, s_0_5);
        // C s_0_9: const #102208u : u32
        let s_0_9: u32 = 102208;
        // D s_0_10: read-reg s_0_9:struct
        let s_0_10: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_9 as isize);
            tracer.read_register(s_0_9 as isize, value);
            value
        };
        // C s_0_11: const #102208u : u32
        let s_0_11: u32 = 102208;
        // N s_0_12: write-reg s_0_11 <= s_0_10
        let s_0_12: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize, s_0_10);
            tracer.write_register(s_0_11 as isize, s_0_10);
        };
        // C s_0_13: const #15272u : u32
        let s_0_13: u32 = 15272;
        // D s_0_14: read-reg s_0_13:struct
        let s_0_14: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_13 as isize);
            tracer.read_register(s_0_13 as isize, value);
            value
        };
        // C s_0_15: const #15272u : u32
        let s_0_15: u32 = 15272;
        // N s_0_16: write-reg s_0_15 <= s_0_14
        let s_0_16: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize, s_0_14);
            tracer.write_register(s_0_15 as isize, s_0_14);
        };
        // C s_0_17: const #0s : i64
        let s_0_17: i64 = 0;
        // C s_0_18: const #15272u : u32
        let s_0_18: u32 = 15272;
        // D s_0_19: read-reg s_0_18:struct
        let s_0_19: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_18 as isize);
            tracer.read_register(s_0_18 as isize, value);
            value
        };
        // D s_0_20: call _get_ERRIDR_EL1_Type_NUM(s_0_19)
        let s_0_20: u16 = u_get_ERRIDR_EL1_Type_NUM(state, tracer, s_0_19);
        // D s_0_21: cast zx s_0_20 -> bv
        let s_0_21: Bits = Bits::new(s_0_20 as u128, 16u16);
        // D s_0_22: cast zx s_0_21 -> i
        let s_0_22: i128 = (s_0_21.value() as i128);
        // D s_0_23: cast reint s_0_22 -> i64
        let s_0_23: i64 = (s_0_22 as i64);
        // C s_0_24: const #1s : i
        let s_0_24: i128 = 1;
        // D s_0_25: cast zx s_0_23 -> i
        let s_0_25: i128 = (i128::try_from(s_0_23).unwrap());
        // D s_0_26: sub s_0_25 s_0_24
        let s_0_26: i128 = ((s_0_25) - (s_0_24));
        // D s_0_27: cast reint s_0_26 -> i64
        let s_0_27: i64 = (s_0_26 as i64);
        // D s_0_28: write-var gs#328160 <= s_0_27
        fn_state.gs_328160 = s_0_27;
        // D s_0_29: write-var i <= s_0_17
        fn_state.i = s_0_17;
        // N s_0_30: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var i:i64
        let s_1_0: i64 = fn_state.i;
        // D s_1_1: read-var gs#328160:i64
        let s_1_1: i64 = fn_state.gs_328160;
        // D s_1_2: cmp-gt s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) > (s_1_1));
        // N s_1_3: branch s_1_2 b6 b2
        if s_1_2 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var i:i64
        let s_2_0: i64 = fn_state.i;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: call __id(s_2_1)
        let s_2_2: i128 = u__id(state, tracer, s_2_1);
        // D s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // C s_2_4: const #0s : i
        let s_2_4: i128 = 0;
        // D s_2_5: cast zx s_2_3 -> i
        let s_2_5: i128 = (i128::try_from(s_2_3).unwrap());
        // D s_2_6: cmp-le s_2_4 s_2_5
        let s_2_6: bool = ((s_2_4) <= (s_2_5));
        // N s_2_7: branch s_2_6 b5 b3
        if s_2_6 {
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
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#328165 <= s_3_0
        fn_state.gs_328165 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#328165:u8
        let s_4_0: bool = fn_state.gs_328165;
        // N s_4_1: assert s_4_0
        let s_4_1: () = assert!(s_4_0);
        // C s_4_2: const #16912u : u32
        let s_4_2: u32 = 16912;
        // D s_4_3: read-reg s_4_2:[struct; 4]
        let s_4_3: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_4_2 as isize);
            tracer.read_register(s_4_2 as isize, value);
            value
        };
        // D s_4_4: read-var i:i64
        let s_4_4: i64 = fn_state.i;
        // D s_4_5: cast zx s_4_4 -> i
        let s_4_5: i128 = (i128::try_from(s_4_4).unwrap());
        // D s_4_6: read-element s_4_3[s_4_5]
        let s_4_6: ProductType5c790c8ef59cc8b2 = s_4_3[(s_4_5) as usize];
        // C s_4_7: const #16912u : u32
        let s_4_7: u32 = 16912;
        // D s_4_8: read-reg s_4_7:[struct; 4]
        let s_4_8: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_4_7 as isize);
            tracer.read_register(s_4_7 as isize, value);
            value
        };
        // D s_4_9: read-var i:i64
        let s_4_9: i64 = fn_state.i;
        // D s_4_10: cast zx s_4_9 -> i
        let s_4_10: i128 = (i128::try_from(s_4_9).unwrap());
        // D s_4_11: mutate-element s_4_8[s_4_10] <= s_4_6
        let s_4_11: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut local = s_4_8.clone();
            local[(s_4_10) as usize] = s_4_6;
            local
        };
        // D s_4_12: cast cvt s_4_11 -> [struct; 0]
        let s_4_12: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_4_11,
        );
        // D s_4_13: cast cvt s_4_12 -> [struct; 4]
        let s_4_13: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_4_12);
            buf
        };
        // C s_4_14: const #16912u : u32
        let s_4_14: u32 = 16912;
        // N s_4_15: write-reg s_4_14 <= s_4_13
        let s_4_15: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_4_14 as isize, s_4_13);
            tracer.write_register(s_4_14 as isize, s_4_13);
        };
        // C s_4_16: const #16912u : u32
        let s_4_16: u32 = 16912;
        // D s_4_17: read-reg s_4_16:[struct; 4]
        let s_4_17: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_4_16 as isize);
            tracer.read_register(s_4_16 as isize, value);
            value
        };
        // D s_4_18: read-var i:i64
        let s_4_18: i64 = fn_state.i;
        // D s_4_19: cast zx s_4_18 -> i
        let s_4_19: i128 = (i128::try_from(s_4_18).unwrap());
        // D s_4_20: read-element s_4_17[s_4_19]
        let s_4_20: ProductType5c790c8ef59cc8b2 = s_4_17[(s_4_19) as usize];
        // C s_4_21: const #16912u : u32
        let s_4_21: u32 = 16912;
        // D s_4_22: read-reg s_4_21:[struct; 4]
        let s_4_22: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_4_21 as isize);
            tracer.read_register(s_4_21 as isize, value);
            value
        };
        // D s_4_23: read-var i:i64
        let s_4_23: i64 = fn_state.i;
        // D s_4_24: cast zx s_4_23 -> i
        let s_4_24: i128 = (i128::try_from(s_4_23).unwrap());
        // D s_4_25: mutate-element s_4_22[s_4_24] <= s_4_20
        let s_4_25: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut local = s_4_22.clone();
            local[(s_4_24) as usize] = s_4_20;
            local
        };
        // D s_4_26: cast cvt s_4_25 -> [struct; 0]
        let s_4_26: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_4_25,
        );
        // D s_4_27: cast cvt s_4_26 -> [struct; 4]
        let s_4_27: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_4_26);
            buf
        };
        // C s_4_28: const #16912u : u32
        let s_4_28: u32 = 16912;
        // N s_4_29: write-reg s_4_28 <= s_4_27
        let s_4_29: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_4_28 as isize, s_4_27);
            tracer.write_register(s_4_28 as isize, s_4_27);
        };
        // C s_4_30: const #16912u : u32
        let s_4_30: u32 = 16912;
        // D s_4_31: read-reg s_4_30:[struct; 4]
        let s_4_31: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_4_30 as isize);
            tracer.read_register(s_4_30 as isize, value);
            value
        };
        // D s_4_32: read-var i:i64
        let s_4_32: i64 = fn_state.i;
        // D s_4_33: cast zx s_4_32 -> i
        let s_4_33: i128 = (i128::try_from(s_4_32).unwrap());
        // D s_4_34: read-element s_4_31[s_4_33]
        let s_4_34: ProductType5c790c8ef59cc8b2 = s_4_31[(s_4_33) as usize];
        // C s_4_35: const #16912u : u32
        let s_4_35: u32 = 16912;
        // D s_4_36: read-reg s_4_35:[struct; 4]
        let s_4_36: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_4_35 as isize);
            tracer.read_register(s_4_35 as isize, value);
            value
        };
        // D s_4_37: read-var i:i64
        let s_4_37: i64 = fn_state.i;
        // D s_4_38: cast zx s_4_37 -> i
        let s_4_38: i128 = (i128::try_from(s_4_37).unwrap());
        // D s_4_39: mutate-element s_4_36[s_4_38] <= s_4_34
        let s_4_39: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut local = s_4_36.clone();
            local[(s_4_38) as usize] = s_4_34;
            local
        };
        // D s_4_40: cast cvt s_4_39 -> [struct; 0]
        let s_4_40: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_4_39,
        );
        // D s_4_41: cast cvt s_4_40 -> [struct; 4]
        let s_4_41: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_4_40);
            buf
        };
        // C s_4_42: const #16912u : u32
        let s_4_42: u32 = 16912;
        // N s_4_43: write-reg s_4_42 <= s_4_41
        let s_4_43: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_4_42 as isize, s_4_41);
            tracer.write_register(s_4_42 as isize, s_4_41);
        };
        // C s_4_44: const #16912u : u32
        let s_4_44: u32 = 16912;
        // D s_4_45: read-reg s_4_44:[struct; 4]
        let s_4_45: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_4_44 as isize);
            tracer.read_register(s_4_44 as isize, value);
            value
        };
        // D s_4_46: read-var i:i64
        let s_4_46: i64 = fn_state.i;
        // D s_4_47: cast zx s_4_46 -> i
        let s_4_47: i128 = (i128::try_from(s_4_46).unwrap());
        // D s_4_48: read-element s_4_45[s_4_47]
        let s_4_48: ProductType5c790c8ef59cc8b2 = s_4_45[(s_4_47) as usize];
        // C s_4_49: const #16912u : u32
        let s_4_49: u32 = 16912;
        // D s_4_50: read-reg s_4_49:[struct; 4]
        let s_4_50: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_4_49 as isize);
            tracer.read_register(s_4_49 as isize, value);
            value
        };
        // D s_4_51: read-var i:i64
        let s_4_51: i64 = fn_state.i;
        // D s_4_52: cast zx s_4_51 -> i
        let s_4_52: i128 = (i128::try_from(s_4_51).unwrap());
        // D s_4_53: mutate-element s_4_50[s_4_52] <= s_4_48
        let s_4_53: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut local = s_4_50.clone();
            local[(s_4_52) as usize] = s_4_48;
            local
        };
        // D s_4_54: cast cvt s_4_53 -> [struct; 0]
        let s_4_54: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_4_53,
        );
        // D s_4_55: cast cvt s_4_54 -> [struct; 4]
        let s_4_55: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_4_54);
            buf
        };
        // C s_4_56: const #16912u : u32
        let s_4_56: u32 = 16912;
        // N s_4_57: write-reg s_4_56 <= s_4_55
        let s_4_57: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_4_56 as isize, s_4_55);
            tracer.write_register(s_4_56 as isize, s_4_55);
        };
        // C s_4_58: const #16912u : u32
        let s_4_58: u32 = 16912;
        // D s_4_59: read-reg s_4_58:[struct; 4]
        let s_4_59: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_4_58 as isize);
            tracer.read_register(s_4_58 as isize, value);
            value
        };
        // D s_4_60: read-var i:i64
        let s_4_60: i64 = fn_state.i;
        // D s_4_61: cast zx s_4_60 -> i
        let s_4_61: i128 = (i128::try_from(s_4_60).unwrap());
        // D s_4_62: read-element s_4_59[s_4_61]
        let s_4_62: ProductType5c790c8ef59cc8b2 = s_4_59[(s_4_61) as usize];
        // C s_4_63: const #16912u : u32
        let s_4_63: u32 = 16912;
        // D s_4_64: read-reg s_4_63:[struct; 4]
        let s_4_64: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_4_63 as isize);
            tracer.read_register(s_4_63 as isize, value);
            value
        };
        // D s_4_65: read-var i:i64
        let s_4_65: i64 = fn_state.i;
        // D s_4_66: cast zx s_4_65 -> i
        let s_4_66: i128 = (i128::try_from(s_4_65).unwrap());
        // D s_4_67: mutate-element s_4_64[s_4_66] <= s_4_62
        let s_4_67: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut local = s_4_64.clone();
            local[(s_4_66) as usize] = s_4_62;
            local
        };
        // D s_4_68: cast cvt s_4_67 -> [struct; 0]
        let s_4_68: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_4_67,
        );
        // D s_4_69: cast cvt s_4_68 -> [struct; 4]
        let s_4_69: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_4_68);
            buf
        };
        // C s_4_70: const #16912u : u32
        let s_4_70: u32 = 16912;
        // N s_4_71: write-reg s_4_70 <= s_4_69
        let s_4_71: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_4_70 as isize, s_4_69);
            tracer.write_register(s_4_70 as isize, s_4_69);
        };
        // C s_4_72: const #16912u : u32
        let s_4_72: u32 = 16912;
        // D s_4_73: read-reg s_4_72:[struct; 4]
        let s_4_73: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_4_72 as isize);
            tracer.read_register(s_4_72 as isize, value);
            value
        };
        // D s_4_74: read-var i:i64
        let s_4_74: i64 = fn_state.i;
        // D s_4_75: cast zx s_4_74 -> i
        let s_4_75: i128 = (i128::try_from(s_4_74).unwrap());
        // D s_4_76: read-element s_4_73[s_4_75]
        let s_4_76: ProductType5c790c8ef59cc8b2 = s_4_73[(s_4_75) as usize];
        // C s_4_77: const #16912u : u32
        let s_4_77: u32 = 16912;
        // D s_4_78: read-reg s_4_77:[struct; 4]
        let s_4_78: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_4_77 as isize);
            tracer.read_register(s_4_77 as isize, value);
            value
        };
        // D s_4_79: read-var i:i64
        let s_4_79: i64 = fn_state.i;
        // D s_4_80: cast zx s_4_79 -> i
        let s_4_80: i128 = (i128::try_from(s_4_79).unwrap());
        // D s_4_81: mutate-element s_4_78[s_4_80] <= s_4_76
        let s_4_81: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut local = s_4_78.clone();
            local[(s_4_80) as usize] = s_4_76;
            local
        };
        // D s_4_82: cast cvt s_4_81 -> [struct; 0]
        let s_4_82: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_4_81,
        );
        // D s_4_83: cast cvt s_4_82 -> [struct; 4]
        let s_4_83: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_4_82);
            buf
        };
        // C s_4_84: const #16912u : u32
        let s_4_84: u32 = 16912;
        // N s_4_85: write-reg s_4_84 <= s_4_83
        let s_4_85: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_4_84 as isize, s_4_83);
            tracer.write_register(s_4_84 as isize, s_4_83);
        };
        // C s_4_86: const #16912u : u32
        let s_4_86: u32 = 16912;
        // D s_4_87: read-reg s_4_86:[struct; 4]
        let s_4_87: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_4_86 as isize);
            tracer.read_register(s_4_86 as isize, value);
            value
        };
        // D s_4_88: read-var i:i64
        let s_4_88: i64 = fn_state.i;
        // D s_4_89: cast zx s_4_88 -> i
        let s_4_89: i128 = (i128::try_from(s_4_88).unwrap());
        // D s_4_90: read-element s_4_87[s_4_89]
        let s_4_90: ProductType5c790c8ef59cc8b2 = s_4_87[(s_4_89) as usize];
        // C s_4_91: const #16912u : u32
        let s_4_91: u32 = 16912;
        // D s_4_92: read-reg s_4_91:[struct; 4]
        let s_4_92: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_4_91 as isize);
            tracer.read_register(s_4_91 as isize, value);
            value
        };
        // D s_4_93: read-var i:i64
        let s_4_93: i64 = fn_state.i;
        // D s_4_94: cast zx s_4_93 -> i
        let s_4_94: i128 = (i128::try_from(s_4_93).unwrap());
        // D s_4_95: mutate-element s_4_92[s_4_94] <= s_4_90
        let s_4_95: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut local = s_4_92.clone();
            local[(s_4_94) as usize] = s_4_90;
            local
        };
        // D s_4_96: cast cvt s_4_95 -> [struct; 0]
        let s_4_96: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_4_95,
        );
        // D s_4_97: cast cvt s_4_96 -> [struct; 4]
        let s_4_97: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_4_96);
            buf
        };
        // C s_4_98: const #16912u : u32
        let s_4_98: u32 = 16912;
        // N s_4_99: write-reg s_4_98 <= s_4_97
        let s_4_99: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_4_98 as isize, s_4_97);
            tracer.write_register(s_4_98 as isize, s_4_97);
        };
        // D s_4_100: read-var i:i64
        let s_4_100: i64 = fn_state.i;
        // C s_4_101: const #1s : i64
        let s_4_101: i64 = 1;
        // D s_4_102: add s_4_100 s_4_101
        let s_4_102: i64 = (s_4_100 + s_4_101);
        // D s_4_103: write-var i <= s_4_102
        fn_state.i = s_4_102;
        // N s_4_104: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var i:i64
        let s_5_0: i64 = fn_state.i;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: call __id(s_5_1)
        let s_5_2: i128 = u__id(state, tracer, s_5_1);
        // D s_5_3: cast reint s_5_2 -> i64
        let s_5_3: i64 = (s_5_2 as i64);
        // C s_5_4: const #4s : i
        let s_5_4: i128 = 4;
        // D s_5_5: cast zx s_5_3 -> i
        let s_5_5: i128 = (i128::try_from(s_5_3).unwrap());
        // D s_5_6: cmp-lt s_5_5 s_5_4
        let s_5_6: bool = ((s_5_5) < (s_5_4));
        // D s_5_7: write-var gs#328165 <= s_5_6
        fn_state.gs_328165 = s_5_6;
        // N s_5_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0s : i
        let s_6_0: i128 = 0;
        // C s_6_1: const #16912u : u32
        let s_6_1: u32 = 16912;
        // D s_6_2: read-reg s_6_1:[struct; 4]
        let s_6_2: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_6_1 as isize);
            tracer.read_register(s_6_1 as isize, value);
            value
        };
        // D s_6_3: read-element s_6_2[s_6_0]
        let s_6_3: ProductType5c790c8ef59cc8b2 = s_6_2[(s_6_0) as usize];
        // C s_6_4: const #0s : i
        let s_6_4: i128 = 0;
        // C s_6_5: const #16912u : u32
        let s_6_5: u32 = 16912;
        // D s_6_6: read-reg s_6_5:[struct; 4]
        let s_6_6: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_6_5 as isize);
            tracer.read_register(s_6_5 as isize, value);
            value
        };
        // D s_6_7: mutate-element s_6_6[s_6_4] <= s_6_3
        let s_6_7: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut local = s_6_6.clone();
            local[(s_6_4) as usize] = s_6_3;
            local
        };
        // D s_6_8: cast cvt s_6_7 -> [struct; 0]
        let s_6_8: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_6_7,
        );
        // D s_6_9: cast cvt s_6_8 -> [struct; 4]
        let s_6_9: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_6_8);
            buf
        };
        // C s_6_10: const #16912u : u32
        let s_6_10: u32 = 16912;
        // N s_6_11: write-reg s_6_10 <= s_6_9
        let s_6_11: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_6_10 as isize, s_6_9);
            tracer.write_register(s_6_10 as isize, s_6_9);
        };
        // C s_6_12: const #0s : i
        let s_6_12: i128 = 0;
        // C s_6_13: const #16912u : u32
        let s_6_13: u32 = 16912;
        // D s_6_14: read-reg s_6_13:[struct; 4]
        let s_6_14: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_6_13 as isize);
            tracer.read_register(s_6_13 as isize, value);
            value
        };
        // D s_6_15: read-element s_6_14[s_6_12]
        let s_6_15: ProductType5c790c8ef59cc8b2 = s_6_14[(s_6_12) as usize];
        // C s_6_16: const #0s : i
        let s_6_16: i128 = 0;
        // C s_6_17: const #16912u : u32
        let s_6_17: u32 = 16912;
        // D s_6_18: read-reg s_6_17:[struct; 4]
        let s_6_18: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_6_17 as isize);
            tracer.read_register(s_6_17 as isize, value);
            value
        };
        // D s_6_19: mutate-element s_6_18[s_6_16] <= s_6_15
        let s_6_19: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut local = s_6_18.clone();
            local[(s_6_16) as usize] = s_6_15;
            local
        };
        // D s_6_20: cast cvt s_6_19 -> [struct; 0]
        let s_6_20: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_6_19,
        );
        // D s_6_21: cast cvt s_6_20 -> [struct; 4]
        let s_6_21: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_6_20);
            buf
        };
        // C s_6_22: const #16912u : u32
        let s_6_22: u32 = 16912;
        // N s_6_23: write-reg s_6_22 <= s_6_21
        let s_6_23: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_6_22 as isize, s_6_21);
            tracer.write_register(s_6_22 as isize, s_6_21);
        };
        // C s_6_24: const #0s : i
        let s_6_24: i128 = 0;
        // C s_6_25: const #16912u : u32
        let s_6_25: u32 = 16912;
        // D s_6_26: read-reg s_6_25:[struct; 4]
        let s_6_26: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_6_25 as isize);
            tracer.read_register(s_6_25 as isize, value);
            value
        };
        // D s_6_27: read-element s_6_26[s_6_24]
        let s_6_27: ProductType5c790c8ef59cc8b2 = s_6_26[(s_6_24) as usize];
        // C s_6_28: const #0s : i
        let s_6_28: i128 = 0;
        // C s_6_29: const #16912u : u32
        let s_6_29: u32 = 16912;
        // D s_6_30: read-reg s_6_29:[struct; 4]
        let s_6_30: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_6_29 as isize);
            tracer.read_register(s_6_29 as isize, value);
            value
        };
        // D s_6_31: mutate-element s_6_30[s_6_28] <= s_6_27
        let s_6_31: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut local = s_6_30.clone();
            local[(s_6_28) as usize] = s_6_27;
            local
        };
        // D s_6_32: cast cvt s_6_31 -> [struct; 0]
        let s_6_32: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_6_31,
        );
        // D s_6_33: cast cvt s_6_32 -> [struct; 4]
        let s_6_33: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_6_32);
            buf
        };
        // C s_6_34: const #16912u : u32
        let s_6_34: u32 = 16912;
        // N s_6_35: write-reg s_6_34 <= s_6_33
        let s_6_35: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_6_34 as isize, s_6_33);
            tracer.write_register(s_6_34 as isize, s_6_33);
        };
        // C s_6_36: const #0s : i
        let s_6_36: i128 = 0;
        // C s_6_37: const #16912u : u32
        let s_6_37: u32 = 16912;
        // D s_6_38: read-reg s_6_37:[struct; 4]
        let s_6_38: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_6_37 as isize);
            tracer.read_register(s_6_37 as isize, value);
            value
        };
        // D s_6_39: read-element s_6_38[s_6_36]
        let s_6_39: ProductType5c790c8ef59cc8b2 = s_6_38[(s_6_36) as usize];
        // C s_6_40: const #0s : i
        let s_6_40: i128 = 0;
        // C s_6_41: const #16912u : u32
        let s_6_41: u32 = 16912;
        // D s_6_42: read-reg s_6_41:[struct; 4]
        let s_6_42: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_6_41 as isize);
            tracer.read_register(s_6_41 as isize, value);
            value
        };
        // D s_6_43: mutate-element s_6_42[s_6_40] <= s_6_39
        let s_6_43: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut local = s_6_42.clone();
            local[(s_6_40) as usize] = s_6_39;
            local
        };
        // D s_6_44: cast cvt s_6_43 -> [struct; 0]
        let s_6_44: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_6_43,
        );
        // D s_6_45: cast cvt s_6_44 -> [struct; 4]
        let s_6_45: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_6_44);
            buf
        };
        // C s_6_46: const #16912u : u32
        let s_6_46: u32 = 16912;
        // N s_6_47: write-reg s_6_46 <= s_6_45
        let s_6_47: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_6_46 as isize, s_6_45);
            tracer.write_register(s_6_46 as isize, s_6_45);
        };
        // C s_6_48: const #0s : i
        let s_6_48: i128 = 0;
        // C s_6_49: const #16912u : u32
        let s_6_49: u32 = 16912;
        // D s_6_50: read-reg s_6_49:[struct; 4]
        let s_6_50: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_6_49 as isize);
            tracer.read_register(s_6_49 as isize, value);
            value
        };
        // D s_6_51: read-element s_6_50[s_6_48]
        let s_6_51: ProductType5c790c8ef59cc8b2 = s_6_50[(s_6_48) as usize];
        // C s_6_52: const #0s : i
        let s_6_52: i128 = 0;
        // C s_6_53: const #16912u : u32
        let s_6_53: u32 = 16912;
        // D s_6_54: read-reg s_6_53:[struct; 4]
        let s_6_54: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_6_53 as isize);
            tracer.read_register(s_6_53 as isize, value);
            value
        };
        // D s_6_55: mutate-element s_6_54[s_6_52] <= s_6_51
        let s_6_55: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut local = s_6_54.clone();
            local[(s_6_52) as usize] = s_6_51;
            local
        };
        // D s_6_56: cast cvt s_6_55 -> [struct; 0]
        let s_6_56: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_6_55,
        );
        // D s_6_57: cast cvt s_6_56 -> [struct; 4]
        let s_6_57: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_6_56);
            buf
        };
        // C s_6_58: const #16912u : u32
        let s_6_58: u32 = 16912;
        // N s_6_59: write-reg s_6_58 <= s_6_57
        let s_6_59: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_6_58 as isize, s_6_57);
            tracer.write_register(s_6_58 as isize, s_6_57);
        };
        // C s_6_60: const #0s : i
        let s_6_60: i128 = 0;
        // C s_6_61: const #16912u : u32
        let s_6_61: u32 = 16912;
        // D s_6_62: read-reg s_6_61:[struct; 4]
        let s_6_62: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_6_61 as isize);
            tracer.read_register(s_6_61 as isize, value);
            value
        };
        // D s_6_63: read-element s_6_62[s_6_60]
        let s_6_63: ProductType5c790c8ef59cc8b2 = s_6_62[(s_6_60) as usize];
        // C s_6_64: const #0s : i
        let s_6_64: i128 = 0;
        // C s_6_65: const #16912u : u32
        let s_6_65: u32 = 16912;
        // D s_6_66: read-reg s_6_65:[struct; 4]
        let s_6_66: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_6_65 as isize);
            tracer.read_register(s_6_65 as isize, value);
            value
        };
        // D s_6_67: mutate-element s_6_66[s_6_64] <= s_6_63
        let s_6_67: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut local = s_6_66.clone();
            local[(s_6_64) as usize] = s_6_63;
            local
        };
        // D s_6_68: cast cvt s_6_67 -> [struct; 0]
        let s_6_68: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_6_67,
        );
        // D s_6_69: cast cvt s_6_68 -> [struct; 4]
        let s_6_69: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_6_68);
            buf
        };
        // C s_6_70: const #16912u : u32
        let s_6_70: u32 = 16912;
        // N s_6_71: write-reg s_6_70 <= s_6_69
        let s_6_71: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_6_70 as isize, s_6_69);
            tracer.write_register(s_6_70 as isize, s_6_69);
        };
        // C s_6_72: const #0s : i
        let s_6_72: i128 = 0;
        // C s_6_73: const #16912u : u32
        let s_6_73: u32 = 16912;
        // D s_6_74: read-reg s_6_73:[struct; 4]
        let s_6_74: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_6_73 as isize);
            tracer.read_register(s_6_73 as isize, value);
            value
        };
        // D s_6_75: read-element s_6_74[s_6_72]
        let s_6_75: ProductType5c790c8ef59cc8b2 = s_6_74[(s_6_72) as usize];
        // C s_6_76: const #0s : i
        let s_6_76: i128 = 0;
        // C s_6_77: const #16912u : u32
        let s_6_77: u32 = 16912;
        // D s_6_78: read-reg s_6_77:[struct; 4]
        let s_6_78: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_6_77 as isize);
            tracer.read_register(s_6_77 as isize, value);
            value
        };
        // D s_6_79: mutate-element s_6_78[s_6_76] <= s_6_75
        let s_6_79: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut local = s_6_78.clone();
            local[(s_6_76) as usize] = s_6_75;
            local
        };
        // D s_6_80: cast cvt s_6_79 -> [struct; 0]
        let s_6_80: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_6_79,
        );
        // D s_6_81: cast cvt s_6_80 -> [struct; 4]
        let s_6_81: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_6_80);
            buf
        };
        // C s_6_82: const #16912u : u32
        let s_6_82: u32 = 16912;
        // N s_6_83: write-reg s_6_82 <= s_6_81
        let s_6_83: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_6_82 as isize, s_6_81);
            tracer.write_register(s_6_82 as isize, s_6_81);
        };
        // C s_6_84: const #0s : i
        let s_6_84: i128 = 0;
        // C s_6_85: const #16912u : u32
        let s_6_85: u32 = 16912;
        // D s_6_86: read-reg s_6_85:[struct; 4]
        let s_6_86: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_6_85 as isize);
            tracer.read_register(s_6_85 as isize, value);
            value
        };
        // D s_6_87: read-element s_6_86[s_6_84]
        let s_6_87: ProductType5c790c8ef59cc8b2 = s_6_86[(s_6_84) as usize];
        // C s_6_88: const #0s : i
        let s_6_88: i128 = 0;
        // C s_6_89: const #16912u : u32
        let s_6_89: u32 = 16912;
        // D s_6_90: read-reg s_6_89:[struct; 4]
        let s_6_90: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_6_89 as isize);
            tracer.read_register(s_6_89 as isize, value);
            value
        };
        // D s_6_91: mutate-element s_6_90[s_6_88] <= s_6_87
        let s_6_91: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut local = s_6_90.clone();
            local[(s_6_88) as usize] = s_6_87;
            local
        };
        // D s_6_92: cast cvt s_6_91 -> [struct; 0]
        let s_6_92: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_6_91,
        );
        // D s_6_93: cast cvt s_6_92 -> [struct; 4]
        let s_6_93: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_6_92);
            buf
        };
        // C s_6_94: const #16912u : u32
        let s_6_94: u32 = 16912;
        // N s_6_95: write-reg s_6_94 <= s_6_93
        let s_6_95: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_6_94 as isize, s_6_93);
            tracer.write_register(s_6_94 as isize, s_6_93);
        };
        // C s_6_96: const #0s : i
        let s_6_96: i128 = 0;
        // C s_6_97: const #16912u : u32
        let s_6_97: u32 = 16912;
        // D s_6_98: read-reg s_6_97:[struct; 4]
        let s_6_98: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_6_97 as isize);
            tracer.read_register(s_6_97 as isize, value);
            value
        };
        // D s_6_99: read-element s_6_98[s_6_96]
        let s_6_99: ProductType5c790c8ef59cc8b2 = s_6_98[(s_6_96) as usize];
        // C s_6_100: const #0s : i
        let s_6_100: i128 = 0;
        // C s_6_101: const #16912u : u32
        let s_6_101: u32 = 16912;
        // D s_6_102: read-reg s_6_101:[struct; 4]
        let s_6_102: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_6_101 as isize);
            tracer.read_register(s_6_101 as isize, value);
            value
        };
        // D s_6_103: mutate-element s_6_102[s_6_100] <= s_6_99
        let s_6_103: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut local = s_6_102.clone();
            local[(s_6_100) as usize] = s_6_99;
            local
        };
        // D s_6_104: cast cvt s_6_103 -> [struct; 0]
        let s_6_104: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_6_103,
        );
        // D s_6_105: cast cvt s_6_104 -> [struct; 4]
        let s_6_105: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_6_104);
            buf
        };
        // C s_6_106: const #16912u : u32
        let s_6_106: u32 = 16912;
        // N s_6_107: write-reg s_6_106 <= s_6_105
        let s_6_107: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_6_106 as isize, s_6_105);
            tracer.write_register(s_6_106 as isize, s_6_105);
        };
        // C s_6_108: const #0s : i
        let s_6_108: i128 = 0;
        // C s_6_109: const #16912u : u32
        let s_6_109: u32 = 16912;
        // D s_6_110: read-reg s_6_109:[struct; 4]
        let s_6_110: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_6_109 as isize);
            tracer.read_register(s_6_109 as isize, value);
            value
        };
        // D s_6_111: read-element s_6_110[s_6_108]
        let s_6_111: ProductType5c790c8ef59cc8b2 = s_6_110[(s_6_108) as usize];
        // C s_6_112: const #0s : i
        let s_6_112: i128 = 0;
        // C s_6_113: const #16912u : u32
        let s_6_113: u32 = 16912;
        // D s_6_114: read-reg s_6_113:[struct; 4]
        let s_6_114: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_6_113 as isize);
            tracer.read_register(s_6_113 as isize, value);
            value
        };
        // D s_6_115: mutate-element s_6_114[s_6_112] <= s_6_111
        let s_6_115: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut local = s_6_114.clone();
            local[(s_6_112) as usize] = s_6_111;
            local
        };
        // D s_6_116: cast cvt s_6_115 -> [struct; 0]
        let s_6_116: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_6_115,
        );
        // D s_6_117: cast cvt s_6_116 -> [struct; 4]
        let s_6_117: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_6_116);
            buf
        };
        // C s_6_118: const #16912u : u32
        let s_6_118: u32 = 16912;
        // N s_6_119: write-reg s_6_118 <= s_6_117
        let s_6_119: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_6_118 as isize, s_6_117);
            tracer.write_register(s_6_118 as isize, s_6_117);
        };
        // C s_6_120: const #0s : i
        let s_6_120: i128 = 0;
        // C s_6_121: const #16912u : u32
        let s_6_121: u32 = 16912;
        // D s_6_122: read-reg s_6_121:[struct; 4]
        let s_6_122: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_6_121 as isize);
            tracer.read_register(s_6_121 as isize, value);
            value
        };
        // D s_6_123: read-element s_6_122[s_6_120]
        let s_6_123: ProductType5c790c8ef59cc8b2 = s_6_122[(s_6_120) as usize];
        // C s_6_124: const #0s : i
        let s_6_124: i128 = 0;
        // C s_6_125: const #16912u : u32
        let s_6_125: u32 = 16912;
        // D s_6_126: read-reg s_6_125:[struct; 4]
        let s_6_126: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_6_125 as isize);
            tracer.read_register(s_6_125 as isize, value);
            value
        };
        // D s_6_127: mutate-element s_6_126[s_6_124] <= s_6_123
        let s_6_127: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut local = s_6_126.clone();
            local[(s_6_124) as usize] = s_6_123;
            local
        };
        // D s_6_128: cast cvt s_6_127 -> [struct; 0]
        let s_6_128: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_6_127,
        );
        // D s_6_129: cast cvt s_6_128 -> [struct; 4]
        let s_6_129: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_6_128);
            buf
        };
        // C s_6_130: const #16912u : u32
        let s_6_130: u32 = 16912;
        // N s_6_131: write-reg s_6_130 <= s_6_129
        let s_6_131: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_6_130 as isize, s_6_129);
            tracer.write_register(s_6_130 as isize, s_6_129);
        };
        // C s_6_132: const #0s : i
        let s_6_132: i128 = 0;
        // C s_6_133: const #16912u : u32
        let s_6_133: u32 = 16912;
        // D s_6_134: read-reg s_6_133:[struct; 4]
        let s_6_134: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_6_133 as isize);
            tracer.read_register(s_6_133 as isize, value);
            value
        };
        // D s_6_135: read-element s_6_134[s_6_132]
        let s_6_135: ProductType5c790c8ef59cc8b2 = s_6_134[(s_6_132) as usize];
        // C s_6_136: const #0s : i
        let s_6_136: i128 = 0;
        // C s_6_137: const #16912u : u32
        let s_6_137: u32 = 16912;
        // D s_6_138: read-reg s_6_137:[struct; 4]
        let s_6_138: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_6_137 as isize);
            tracer.read_register(s_6_137 as isize, value);
            value
        };
        // D s_6_139: mutate-element s_6_138[s_6_136] <= s_6_135
        let s_6_139: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut local = s_6_138.clone();
            local[(s_6_136) as usize] = s_6_135;
            local
        };
        // D s_6_140: cast cvt s_6_139 -> [struct; 0]
        let s_6_140: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_6_139,
        );
        // D s_6_141: cast cvt s_6_140 -> [struct; 4]
        let s_6_141: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_6_140);
            buf
        };
        // C s_6_142: const #16912u : u32
        let s_6_142: u32 = 16912;
        // N s_6_143: write-reg s_6_142 <= s_6_141
        let s_6_143: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_6_142 as isize, s_6_141);
            tracer.write_register(s_6_142 as isize, s_6_141);
        };
        // C s_6_144: const #8s : i
        let s_6_144: i128 = 8;
        // S s_6_145: call Zeros(s_6_144)
        let s_6_145: Bits = Zeros(state, tracer, s_6_144);
        // C s_6_146: const #21016u : u32
        let s_6_146: u32 = 21016;
        // D s_6_147: read-reg s_6_146:struct
        let s_6_147: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_6_146 as isize);
            tracer.read_register(s_6_146 as isize, value);
            value
        };
        // C s_6_148: const #21016u : u32
        let s_6_148: u32 = 21016;
        // N s_6_149: write-reg s_6_148 <= s_6_147
        let s_6_149: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_6_148 as isize, s_6_147);
            tracer.write_register(s_6_148 as isize, s_6_147);
        };
        // C s_6_150: const #21016u : u32
        let s_6_150: u32 = 21016;
        // D s_6_151: read-reg s_6_150:struct
        let s_6_151: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_6_150 as isize);
            tracer.read_register(s_6_150 as isize, value);
            value
        };
        // D s_6_152: call _get_PMCR_EL0_Type_IMP(s_6_151)
        let s_6_152: u8 = u_get_PMCR_EL0_Type_IMP(state, tracer, s_6_151);
        // D s_6_153: cast zx s_6_152 -> bv
        let s_6_153: Bits = Bits::new(s_6_152 as u128, 8u16);
        // D s_6_154: call IsZero(s_6_153)
        let s_6_154: bool = IsZero(state, tracer, s_6_153);
        // N s_6_155: branch s_6_154 b353 b7
        if s_6_154 {
            return block_353(state, tracer, fn_state);
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
        // C s_8_0: const #21016u : u32
        let s_8_0: u32 = 21016;
        // D s_8_1: read-reg s_8_0:struct
        let s_8_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // C s_8_2: const #21016u : u32
        let s_8_2: u32 = 21016;
        // N s_8_3: write-reg s_8_2 <= s_8_1
        let s_8_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_8_2 as isize, s_8_1);
            tracer.write_register(s_8_2 as isize, s_8_1);
        };
        // C s_8_4: const #4s : i
        let s_8_4: i128 = 4;
        // C s_8_5: const #0s : i
        let s_8_5: i128 = 0;
        // C s_8_6: const #11104u : u32
        let s_8_6: u32 = 11104;
        // D s_8_7: read-reg s_8_6:i
        let s_8_7: i128 = {
            let value = state.read_register::<i128>(s_8_6 as isize);
            tracer.read_register(s_8_6 as isize, value);
            value
        };
        // D s_8_8: call integer_subrange(s_8_7, s_8_4, s_8_5)
        let s_8_8: Bits = integer_subrange(state, tracer, s_8_7, s_8_4, s_8_5);
        // C s_8_9: const #21016u : u32
        let s_8_9: u32 = 21016;
        // D s_8_10: read-reg s_8_9:struct
        let s_8_10: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_9 as isize);
            tracer.read_register(s_8_9 as isize, value);
            value
        };
        // C s_8_11: const #21016u : u32
        let s_8_11: u32 = 21016;
        // N s_8_12: write-reg s_8_11 <= s_8_10
        let s_8_12: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_8_11 as isize, s_8_10);
            tracer.write_register(s_8_11 as isize, s_8_10);
        };
        // C s_8_13: const #21016u : u32
        let s_8_13: u32 = 21016;
        // D s_8_14: read-reg s_8_13:struct
        let s_8_14: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_13 as isize);
            tracer.read_register(s_8_13 as isize, value);
            value
        };
        // C s_8_15: const #21016u : u32
        let s_8_15: u32 = 21016;
        // N s_8_16: write-reg s_8_15 <= s_8_14
        let s_8_16: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_8_15 as isize, s_8_14);
            tracer.write_register(s_8_15 as isize, s_8_14);
        };
        // C s_8_17: const #21016u : u32
        let s_8_17: u32 = 21016;
        // D s_8_18: read-reg s_8_17:struct
        let s_8_18: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_17 as isize);
            tracer.read_register(s_8_17 as isize, value);
            value
        };
        // C s_8_19: const #21016u : u32
        let s_8_19: u32 = 21016;
        // N s_8_20: write-reg s_8_19 <= s_8_18
        let s_8_20: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_8_19 as isize, s_8_18);
            tracer.write_register(s_8_19 as isize, s_8_18);
        };
        // C s_8_21: const #64s : i
        let s_8_21: i128 = 64;
        // C s_8_22: const #65632600321u : u36
        let s_8_22: u64 = 65632600321;
        // C s_8_23: cast zx s_8_22 -> bv
        let s_8_23: Bits = Bits::new(s_8_22 as u128, 36u16);
        // D s_8_24: bits-cast zx s_8_23 -> bv length s_8_21
        let s_8_24: Bits = s_8_23.zero_extend(s_8_21);
        // D s_8_25: cast reint s_8_24 -> u64
        let s_8_25: u64 = (s_8_24.value() as u64);
        // D s_8_26: call Mk_PMCEID0_EL0_Type(s_8_25)
        let s_8_26: ProductType5c790c8ef59cc8b2 = Mk_PMCEID0_EL0_Type(
            state,
            tracer,
            s_8_25,
        );
        // C s_8_27: const #14968u : u32
        let s_8_27: u32 = 14968;
        // N s_8_28: write-reg s_8_27 <= s_8_26
        let s_8_28: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_8_27 as isize, s_8_26);
            tracer.write_register(s_8_27 as isize, s_8_26);
        };
        // C s_8_29: const #64s : i
        let s_8_29: i128 = 64;
        // S s_8_30: call Zeros(s_8_29)
        let s_8_30: Bits = Zeros(state, tracer, s_8_29);
        // S s_8_31: cast reint s_8_30 -> u64
        let s_8_31: u64 = (s_8_30.value() as u64);
        // S s_8_32: call Mk_PMCEID1_EL0_Type(s_8_31)
        let s_8_32: ProductType5c790c8ef59cc8b2 = Mk_PMCEID1_EL0_Type(
            state,
            tracer,
            s_8_31,
        );
        // C s_8_33: const #10176u : u32
        let s_8_33: u32 = 10176;
        // N s_8_34: write-reg s_8_33 <= s_8_32
        let s_8_34: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_8_33 as isize, s_8_32);
            tracer.write_register(s_8_33 as isize, s_8_32);
        };
        // C s_8_35: const #16960u : u32
        let s_8_35: u32 = 16960;
        // D s_8_36: read-reg s_8_35:struct
        let s_8_36: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_35 as isize);
            tracer.read_register(s_8_35 as isize, value);
            value
        };
        // C s_8_37: const #16960u : u32
        let s_8_37: u32 = 16960;
        // N s_8_38: write-reg s_8_37 <= s_8_36
        let s_8_38: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_8_37 as isize, s_8_36);
            tracer.write_register(s_8_37 as isize, s_8_36);
        };
        // C s_8_39: const #16960u : u32
        let s_8_39: u32 = 16960;
        // D s_8_40: read-reg s_8_39:struct
        let s_8_40: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_39 as isize);
            tracer.read_register(s_8_39 as isize, value);
            value
        };
        // C s_8_41: const #16960u : u32
        let s_8_41: u32 = 16960;
        // N s_8_42: write-reg s_8_41 <= s_8_40
        let s_8_42: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_8_41 as isize, s_8_40);
            tracer.write_register(s_8_41 as isize, s_8_40);
        };
        // C s_8_43: const #16960u : u32
        let s_8_43: u32 = 16960;
        // D s_8_44: read-reg s_8_43:struct
        let s_8_44: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_43 as isize);
            tracer.read_register(s_8_43 as isize, value);
            value
        };
        // C s_8_45: const #16960u : u32
        let s_8_45: u32 = 16960;
        // N s_8_46: write-reg s_8_45 <= s_8_44
        let s_8_46: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_8_45 as isize, s_8_44);
            tracer.write_register(s_8_45 as isize, s_8_44);
        };
        // C s_8_47: const #16960u : u32
        let s_8_47: u32 = 16960;
        // D s_8_48: read-reg s_8_47:struct
        let s_8_48: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_47 as isize);
            tracer.read_register(s_8_47 as isize, value);
            value
        };
        // C s_8_49: const #16960u : u32
        let s_8_49: u32 = 16960;
        // N s_8_50: write-reg s_8_49 <= s_8_48
        let s_8_50: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_8_49 as isize, s_8_48);
            tracer.write_register(s_8_49 as isize, s_8_48);
        };
        // C s_8_51: const #16960u : u32
        let s_8_51: u32 = 16960;
        // D s_8_52: read-reg s_8_51:struct
        let s_8_52: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_51 as isize);
            tracer.read_register(s_8_51 as isize, value);
            value
        };
        // C s_8_53: const #16960u : u32
        let s_8_53: u32 = 16960;
        // N s_8_54: write-reg s_8_53 <= s_8_52
        let s_8_54: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_8_53 as isize, s_8_52);
            tracer.write_register(s_8_53 as isize, s_8_52);
        };
        // C s_8_55: const #16960u : u32
        let s_8_55: u32 = 16960;
        // D s_8_56: read-reg s_8_55:struct
        let s_8_56: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_55 as isize);
            tracer.read_register(s_8_55 as isize, value);
            value
        };
        // C s_8_57: const #16960u : u32
        let s_8_57: u32 = 16960;
        // N s_8_58: write-reg s_8_57 <= s_8_56
        let s_8_58: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_8_57 as isize, s_8_56);
            tracer.write_register(s_8_57 as isize, s_8_56);
        };
        // C s_8_59: const #16960u : u32
        let s_8_59: u32 = 16960;
        // D s_8_60: read-reg s_8_59:struct
        let s_8_60: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_59 as isize);
            tracer.read_register(s_8_59 as isize, value);
            value
        };
        // C s_8_61: const #16960u : u32
        let s_8_61: u32 = 16960;
        // N s_8_62: write-reg s_8_61 <= s_8_60
        let s_8_62: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_8_61 as isize, s_8_60);
            tracer.write_register(s_8_61 as isize, s_8_60);
        };
        // C s_8_63: const #16960u : u32
        let s_8_63: u32 = 16960;
        // D s_8_64: read-reg s_8_63:struct
        let s_8_64: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_63 as isize);
            tracer.read_register(s_8_63 as isize, value);
            value
        };
        // C s_8_65: const #16960u : u32
        let s_8_65: u32 = 16960;
        // N s_8_66: write-reg s_8_65 <= s_8_64
        let s_8_66: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_8_65 as isize, s_8_64);
            tracer.write_register(s_8_65 as isize, s_8_64);
        };
        // C s_8_67: const #16960u : u32
        let s_8_67: u32 = 16960;
        // D s_8_68: read-reg s_8_67:struct
        let s_8_68: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_67 as isize);
            tracer.read_register(s_8_67 as isize, value);
            value
        };
        // C s_8_69: const #16960u : u32
        let s_8_69: u32 = 16960;
        // N s_8_70: write-reg s_8_69 <= s_8_68
        let s_8_70: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_8_69 as isize, s_8_68);
            tracer.write_register(s_8_69 as isize, s_8_68);
        };
        // C s_8_71: const #4s : i
        let s_8_71: i128 = 4;
        // C s_8_72: const #0s : i
        let s_8_72: i128 = 0;
        // C s_8_73: const #11104u : u32
        let s_8_73: u32 = 11104;
        // D s_8_74: read-reg s_8_73:i
        let s_8_74: i128 = {
            let value = state.read_register::<i128>(s_8_73 as isize);
            tracer.read_register(s_8_73 as isize, value);
            value
        };
        // D s_8_75: call integer_subrange(s_8_74, s_8_71, s_8_72)
        let s_8_75: Bits = integer_subrange(state, tracer, s_8_74, s_8_71, s_8_72);
        // C s_8_76: const #16960u : u32
        let s_8_76: u32 = 16960;
        // D s_8_77: read-reg s_8_76:struct
        let s_8_77: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_76 as isize);
            tracer.read_register(s_8_76 as isize, value);
            value
        };
        // C s_8_78: const #16960u : u32
        let s_8_78: u32 = 16960;
        // N s_8_79: write-reg s_8_78 <= s_8_77
        let s_8_79: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_8_78 as isize, s_8_77);
            tracer.write_register(s_8_78 as isize, s_8_77);
        };
        // C s_8_80: const #18992u : u32
        let s_8_80: u32 = 18992;
        // D s_8_81: read-reg s_8_80:struct
        let s_8_81: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_8_80 as isize);
            tracer.read_register(s_8_80 as isize, value);
            value
        };
        // C s_8_82: const #18992u : u32
        let s_8_82: u32 = 18992;
        // N s_8_83: write-reg s_8_82 <= s_8_81
        let s_8_83: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_8_82 as isize, s_8_81);
            tracer.write_register(s_8_82 as isize, s_8_81);
        };
        // C s_8_84: const #18992u : u32
        let s_8_84: u32 = 18992;
        // D s_8_85: read-reg s_8_84:struct
        let s_8_85: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_8_84 as isize);
            tracer.read_register(s_8_84 as isize, value);
            value
        };
        // C s_8_86: const #18992u : u32
        let s_8_86: u32 = 18992;
        // N s_8_87: write-reg s_8_86 <= s_8_85
        let s_8_87: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_8_86 as isize, s_8_85);
            tracer.write_register(s_8_86 as isize, s_8_85);
        };
        // C s_8_88: const #15776u : u32
        let s_8_88: u32 = 15776;
        // D s_8_89: read-reg s_8_88:struct
        let s_8_89: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_8_88 as isize);
            tracer.read_register(s_8_88 as isize, value);
            value
        };
        // C s_8_90: const #15776u : u32
        let s_8_90: u32 = 15776;
        // N s_8_91: write-reg s_8_90 <= s_8_89
        let s_8_91: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_8_90 as isize, s_8_89);
            tracer.write_register(s_8_90 as isize, s_8_89);
        };
        // C s_8_92: const #104984u : u32
        let s_8_92: u32 = 104984;
        // D s_8_93: read-reg s_8_92:struct
        let s_8_93: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_8_92 as isize);
            tracer.read_register(s_8_92 as isize, value);
            value
        };
        // C s_8_94: const #104984u : u32
        let s_8_94: u32 = 104984;
        // N s_8_95: write-reg s_8_94 <= s_8_93
        let s_8_95: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_8_94 as isize, s_8_93);
            tracer.write_register(s_8_94 as isize, s_8_93);
        };
        // C s_8_96: const #104984u : u32
        let s_8_96: u32 = 104984;
        // D s_8_97: read-reg s_8_96:struct
        let s_8_97: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_8_96 as isize);
            tracer.read_register(s_8_96 as isize, value);
            value
        };
        // C s_8_98: const #104984u : u32
        let s_8_98: u32 = 104984;
        // N s_8_99: write-reg s_8_98 <= s_8_97
        let s_8_99: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_8_98 as isize, s_8_97);
            tracer.write_register(s_8_98 as isize, s_8_97);
        };
        // C s_8_100: const #14072u : u32
        let s_8_100: u32 = 14072;
        // D s_8_101: read-reg s_8_100:struct
        let s_8_101: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_8_100 as isize);
            tracer.read_register(s_8_100 as isize, value);
            value
        };
        // C s_8_102: const #14072u : u32
        let s_8_102: u32 = 14072;
        // N s_8_103: write-reg s_8_102 <= s_8_101
        let s_8_103: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_8_102 as isize, s_8_101);
            tracer.write_register(s_8_102 as isize, s_8_101);
        };
        // C s_8_104: const #14072u : u32
        let s_8_104: u32 = 14072;
        // D s_8_105: read-reg s_8_104:struct
        let s_8_105: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_8_104 as isize);
            tracer.read_register(s_8_104 as isize, value);
            value
        };
        // C s_8_106: const #14072u : u32
        let s_8_106: u32 = 14072;
        // N s_8_107: write-reg s_8_106 <= s_8_105
        let s_8_107: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_8_106 as isize, s_8_105);
            tracer.write_register(s_8_106 as isize, s_8_105);
        };
        // C s_8_108: const #14072u : u32
        let s_8_108: u32 = 14072;
        // D s_8_109: read-reg s_8_108:struct
        let s_8_109: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_8_108 as isize);
            tracer.read_register(s_8_108 as isize, value);
            value
        };
        // C s_8_110: const #14072u : u32
        let s_8_110: u32 = 14072;
        // N s_8_111: write-reg s_8_110 <= s_8_109
        let s_8_111: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_8_110 as isize, s_8_109);
            tracer.write_register(s_8_110 as isize, s_8_109);
        };
        // C s_8_112: const #16384u : u32
        let s_8_112: u32 = 16384;
        // D s_8_113: read-reg s_8_112:struct
        let s_8_113: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_8_112 as isize);
            tracer.read_register(s_8_112 as isize, value);
            value
        };
        // C s_8_114: const #16384u : u32
        let s_8_114: u32 = 16384;
        // N s_8_115: write-reg s_8_114 <= s_8_113
        let s_8_115: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_8_114 as isize, s_8_113);
            tracer.write_register(s_8_114 as isize, s_8_113);
        };
        // C s_8_116: const #16384u : u32
        let s_8_116: u32 = 16384;
        // D s_8_117: read-reg s_8_116:struct
        let s_8_117: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_8_116 as isize);
            tracer.read_register(s_8_116 as isize, value);
            value
        };
        // C s_8_118: const #16384u : u32
        let s_8_118: u32 = 16384;
        // N s_8_119: write-reg s_8_118 <= s_8_117
        let s_8_119: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_8_118 as isize, s_8_117);
            tracer.write_register(s_8_118 as isize, s_8_117);
        };
        // C s_8_120: const #104592u : u32
        let s_8_120: u32 = 104592;
        // D s_8_121: read-reg s_8_120:struct
        let s_8_121: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_8_120 as isize);
            tracer.read_register(s_8_120 as isize, value);
            value
        };
        // C s_8_122: const #104592u : u32
        let s_8_122: u32 = 104592;
        // N s_8_123: write-reg s_8_122 <= s_8_121
        let s_8_123: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_8_122 as isize, s_8_121);
            tracer.write_register(s_8_122 as isize, s_8_121);
        };
        // C s_8_124: const #104592u : u32
        let s_8_124: u32 = 104592;
        // D s_8_125: read-reg s_8_124:struct
        let s_8_125: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_8_124 as isize);
            tracer.read_register(s_8_124 as isize, value);
            value
        };
        // C s_8_126: const #104592u : u32
        let s_8_126: u32 = 104592;
        // N s_8_127: write-reg s_8_126 <= s_8_125
        let s_8_127: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_8_126 as isize, s_8_125);
            tracer.write_register(s_8_126 as isize, s_8_125);
        };
        // C s_8_128: const #17256u : u32
        let s_8_128: u32 = 17256;
        // D s_8_129: read-reg s_8_128:struct
        let s_8_129: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_8_128 as isize);
            tracer.read_register(s_8_128 as isize, value);
            value
        };
        // C s_8_130: const #17256u : u32
        let s_8_130: u32 = 17256;
        // N s_8_131: write-reg s_8_130 <= s_8_129
        let s_8_131: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_8_130 as isize, s_8_129);
            tracer.write_register(s_8_130 as isize, s_8_129);
        };
        // C s_8_132: const #100872u : u32
        let s_8_132: u32 = 100872;
        // D s_8_133: read-reg s_8_132:struct
        let s_8_133: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_8_132 as isize);
            tracer.read_register(s_8_132 as isize, value);
            value
        };
        // C s_8_134: const #100872u : u32
        let s_8_134: u32 = 100872;
        // N s_8_135: write-reg s_8_134 <= s_8_133
        let s_8_135: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_8_134 as isize, s_8_133);
            tracer.write_register(s_8_134 as isize, s_8_133);
        };
        // C s_8_136: const #100872u : u32
        let s_8_136: u32 = 100872;
        // D s_8_137: read-reg s_8_136:struct
        let s_8_137: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_8_136 as isize);
            tracer.read_register(s_8_136 as isize, value);
            value
        };
        // C s_8_138: const #100872u : u32
        let s_8_138: u32 = 100872;
        // N s_8_139: write-reg s_8_138 <= s_8_137
        let s_8_139: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_8_138 as isize, s_8_137);
            tracer.write_register(s_8_138 as isize, s_8_137);
        };
        // C s_8_140: const #10520u : u32
        let s_8_140: u32 = 10520;
        // D s_8_141: read-reg s_8_140:struct
        let s_8_141: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_8_140 as isize);
            tracer.read_register(s_8_140 as isize, value);
            value
        };
        // C s_8_142: const #10520u : u32
        let s_8_142: u32 = 10520;
        // N s_8_143: write-reg s_8_142 <= s_8_141
        let s_8_143: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_8_142 as isize, s_8_141);
            tracer.write_register(s_8_142 as isize, s_8_141);
        };
        // C s_8_144: const #20224u : u32
        let s_8_144: u32 = 20224;
        // D s_8_145: read-reg s_8_144:struct
        let s_8_145: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_8_144 as isize);
            tracer.read_register(s_8_144 as isize, value);
            value
        };
        // C s_8_146: const #20224u : u32
        let s_8_146: u32 = 20224;
        // N s_8_147: write-reg s_8_146 <= s_8_145
        let s_8_147: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_8_146 as isize, s_8_145);
            tracer.write_register(s_8_146 as isize, s_8_145);
        };
        // C s_8_148: const #14240u : u32
        let s_8_148: u32 = 14240;
        // D s_8_149: read-reg s_8_148:u32
        let s_8_149: u32 = {
            let value = state.read_register::<u32>(s_8_148 as isize);
            tracer.read_register(s_8_148 as isize, value);
            value
        };
        // D s_8_150: call Mk_CNTFID0_Type(s_8_149)
        let s_8_150: ProductType700c18a878c5601b = Mk_CNTFID0_Type(
            state,
            tracer,
            s_8_149,
        );
        // C s_8_151: const #11968u : u32
        let s_8_151: u32 = 11968;
        // N s_8_152: write-reg s_8_151 <= s_8_150
        let s_8_152: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_8_151 as isize, s_8_150);
            tracer.write_register(s_8_151 as isize, s_8_150);
        };
        // C s_8_153: const #103144u : u32
        let s_8_153: u32 = 103144;
        // D s_8_154: read-reg s_8_153:struct
        let s_8_154: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_153 as isize);
            tracer.read_register(s_8_153 as isize, value);
            value
        };
        // C s_8_155: const #103144u : u32
        let s_8_155: u32 = 103144;
        // N s_8_156: write-reg s_8_155 <= s_8_154
        let s_8_156: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_8_155 as isize, s_8_154);
            tracer.write_register(s_8_155 as isize, s_8_154);
        };
        // C s_8_157: const #20112u : u32
        let s_8_157: u32 = 20112;
        // D s_8_158: read-reg s_8_157:struct
        let s_8_158: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_157 as isize);
            tracer.read_register(s_8_157 as isize, value);
            value
        };
        // C s_8_159: const #20112u : u32
        let s_8_159: u32 = 20112;
        // N s_8_160: write-reg s_8_159 <= s_8_158
        let s_8_160: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_8_159 as isize, s_8_158);
            tracer.write_register(s_8_159 as isize, s_8_158);
        };
        // C s_8_161: const #20112u : u32
        let s_8_161: u32 = 20112;
        // D s_8_162: read-reg s_8_161:struct
        let s_8_162: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_161 as isize);
            tracer.read_register(s_8_161 as isize, value);
            value
        };
        // C s_8_163: const #20112u : u32
        let s_8_163: u32 = 20112;
        // N s_8_164: write-reg s_8_163 <= s_8_162
        let s_8_164: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_8_163 as isize, s_8_162);
            tracer.write_register(s_8_163 as isize, s_8_162);
        };
        // C s_8_165: const #20112u : u32
        let s_8_165: u32 = 20112;
        // D s_8_166: read-reg s_8_165:struct
        let s_8_166: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_165 as isize);
            tracer.read_register(s_8_165 as isize, value);
            value
        };
        // C s_8_167: const #20112u : u32
        let s_8_167: u32 = 20112;
        // N s_8_168: write-reg s_8_167 <= s_8_166
        let s_8_168: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_8_167 as isize, s_8_166);
            tracer.write_register(s_8_167 as isize, s_8_166);
        };
        // C s_8_169: const #1s : i
        let s_8_169: i128 = 1;
        // C s_8_170: const #102792u : u32
        let s_8_170: u32 = 102792;
        // D s_8_171: read-reg s_8_170:i
        let s_8_171: i128 = {
            let value = state.read_register::<i128>(s_8_170 as isize);
            tracer.read_register(s_8_170 as isize, value);
            value
        };
        // D s_8_172: sub s_8_171 s_8_169
        let s_8_172: i128 = ((s_8_171) - (s_8_169));
        // C s_8_173: const #3s : i
        let s_8_173: i128 = 3;
        // C s_8_174: const #0s : i
        let s_8_174: i128 = 0;
        // D s_8_175: call integer_subrange(s_8_172, s_8_173, s_8_174)
        let s_8_175: Bits = integer_subrange(state, tracer, s_8_172, s_8_173, s_8_174);
        // C s_8_176: const #20112u : u32
        let s_8_176: u32 = 20112;
        // D s_8_177: read-reg s_8_176:struct
        let s_8_177: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_176 as isize);
            tracer.read_register(s_8_176 as isize, value);
            value
        };
        // C s_8_178: const #20112u : u32
        let s_8_178: u32 = 20112;
        // N s_8_179: write-reg s_8_178 <= s_8_177
        let s_8_179: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_8_178 as isize, s_8_177);
            tracer.write_register(s_8_178 as isize, s_8_177);
        };
        // C s_8_180: const #1s : i
        let s_8_180: i128 = 1;
        // C s_8_181: const #19360u : u32
        let s_8_181: u32 = 19360;
        // D s_8_182: read-reg s_8_181:i
        let s_8_182: i128 = {
            let value = state.read_register::<i128>(s_8_181 as isize);
            tracer.read_register(s_8_181 as isize, value);
            value
        };
        // D s_8_183: sub s_8_182 s_8_180
        let s_8_183: i128 = ((s_8_182) - (s_8_180));
        // C s_8_184: const #3s : i
        let s_8_184: i128 = 3;
        // C s_8_185: const #0s : i
        let s_8_185: i128 = 0;
        // D s_8_186: call integer_subrange(s_8_183, s_8_184, s_8_185)
        let s_8_186: Bits = integer_subrange(state, tracer, s_8_183, s_8_184, s_8_185);
        // C s_8_187: const #20112u : u32
        let s_8_187: u32 = 20112;
        // D s_8_188: read-reg s_8_187:struct
        let s_8_188: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_187 as isize);
            tracer.read_register(s_8_187 as isize, value);
            value
        };
        // C s_8_189: const #20112u : u32
        let s_8_189: u32 = 20112;
        // N s_8_190: write-reg s_8_189 <= s_8_188
        let s_8_190: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_8_189 as isize, s_8_188);
            tracer.write_register(s_8_189 as isize, s_8_188);
        };
        // C s_8_191: const #1s : i
        let s_8_191: i128 = 1;
        // C s_8_192: const #22776u : u32
        let s_8_192: u32 = 22776;
        // D s_8_193: read-reg s_8_192:i
        let s_8_193: i128 = {
            let value = state.read_register::<i128>(s_8_192 as isize);
            tracer.read_register(s_8_192 as isize, value);
            value
        };
        // D s_8_194: sub s_8_193 s_8_191
        let s_8_194: i128 = ((s_8_193) - (s_8_191));
        // C s_8_195: const #3s : i
        let s_8_195: i128 = 3;
        // C s_8_196: const #0s : i
        let s_8_196: i128 = 0;
        // D s_8_197: call integer_subrange(s_8_194, s_8_195, s_8_196)
        let s_8_197: Bits = integer_subrange(state, tracer, s_8_194, s_8_195, s_8_196);
        // C s_8_198: const #20112u : u32
        let s_8_198: u32 = 20112;
        // D s_8_199: read-reg s_8_198:struct
        let s_8_199: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_198 as isize);
            tracer.read_register(s_8_198 as isize, value);
            value
        };
        // C s_8_200: const #20112u : u32
        let s_8_200: u32 = 20112;
        // N s_8_201: write-reg s_8_200 <= s_8_199
        let s_8_201: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_8_200 as isize, s_8_199);
            tracer.write_register(s_8_200 as isize, s_8_199);
        };
        // C s_8_202: const #20112u : u32
        let s_8_202: u32 = 20112;
        // D s_8_203: read-reg s_8_202:struct
        let s_8_203: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_202 as isize);
            tracer.read_register(s_8_202 as isize, value);
            value
        };
        // C s_8_204: const #20112u : u32
        let s_8_204: u32 = 20112;
        // N s_8_205: write-reg s_8_204 <= s_8_203
        let s_8_205: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_8_204 as isize, s_8_203);
            tracer.write_register(s_8_204 as isize, s_8_203);
        };
        // C s_8_206: const #20112u : u32
        let s_8_206: u32 = 20112;
        // D s_8_207: read-reg s_8_206:struct
        let s_8_207: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_206 as isize);
            tracer.read_register(s_8_206 as isize, value);
            value
        };
        // C s_8_208: const #20112u : u32
        let s_8_208: u32 = 20112;
        // N s_8_209: write-reg s_8_208 <= s_8_207
        let s_8_209: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_8_208 as isize, s_8_207);
            tracer.write_register(s_8_208 as isize, s_8_207);
        };
        // C s_8_210: const #232u : u32
        let s_8_210: u32 = 232;
        // S s_8_211: call IsFeatureImplemented(s_8_210)
        let s_8_211: bool = IsFeatureImplemented(state, tracer, s_8_210);
        // N s_8_212: branch s_8_211 b352 b9
        if s_8_211 {
            return block_352(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #101824u : u32
        let s_9_0: u32 = 101824;
        // D s_9_1: read-reg s_9_0:struct
        let s_9_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // C s_9_2: const #101824u : u32
        let s_9_2: u32 = 101824;
        // N s_9_3: write-reg s_9_2 <= s_9_1
        let s_9_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_9_2 as isize, s_9_1);
            tracer.write_register(s_9_2 as isize, s_9_1);
        };
        // N s_9_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #230u : u32
        let s_10_0: u32 = 230;
        // S s_10_1: call IsFeatureImplemented(s_10_0)
        let s_10_1: bool = IsFeatureImplemented(state, tracer, s_10_0);
        // N s_10_2: branch s_10_1 b351 b11
        if s_10_1 {
            return block_351(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #90608u : u32
        let s_12_0: u32 = 90608;
        // D s_12_1: read-reg s_12_0:struct
        let s_12_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // C s_12_2: const #90608u : u32
        let s_12_2: u32 = 90608;
        // N s_12_3: write-reg s_12_2 <= s_12_1
        let s_12_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_12_2 as isize, s_12_1);
            tracer.write_register(s_12_2 as isize, s_12_1);
        };
        // C s_12_4: const #90608u : u32
        let s_12_4: u32 = 90608;
        // D s_12_5: read-reg s_12_4:struct
        let s_12_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_4 as isize);
            tracer.read_register(s_12_4 as isize, value);
            value
        };
        // C s_12_6: const #90608u : u32
        let s_12_6: u32 = 90608;
        // N s_12_7: write-reg s_12_6 <= s_12_5
        let s_12_7: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_12_6 as isize, s_12_5);
            tracer.write_register(s_12_6 as isize, s_12_5);
        };
        // C s_12_8: const #90608u : u32
        let s_12_8: u32 = 90608;
        // D s_12_9: read-reg s_12_8:struct
        let s_12_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_8 as isize);
            tracer.read_register(s_12_8 as isize, value);
            value
        };
        // C s_12_10: const #90608u : u32
        let s_12_10: u32 = 90608;
        // N s_12_11: write-reg s_12_10 <= s_12_9
        let s_12_11: () = {
            state
                .write_register::<ProductType5c790c8ef59cc8b2>(s_12_10 as isize, s_12_9);
            tracer.write_register(s_12_10 as isize, s_12_9);
        };
        // C s_12_12: const #90608u : u32
        let s_12_12: u32 = 90608;
        // D s_12_13: read-reg s_12_12:struct
        let s_12_13: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_12 as isize);
            tracer.read_register(s_12_12 as isize, value);
            value
        };
        // C s_12_14: const #90608u : u32
        let s_12_14: u32 = 90608;
        // N s_12_15: write-reg s_12_14 <= s_12_13
        let s_12_15: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_12_14 as isize, s_12_13);
            tracer.write_register(s_12_14 as isize, s_12_13);
        };
        // C s_12_16: const #90608u : u32
        let s_12_16: u32 = 90608;
        // D s_12_17: read-reg s_12_16:struct
        let s_12_17: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_16 as isize);
            tracer.read_register(s_12_16 as isize, value);
            value
        };
        // C s_12_18: const #90608u : u32
        let s_12_18: u32 = 90608;
        // N s_12_19: write-reg s_12_18 <= s_12_17
        let s_12_19: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_12_18 as isize, s_12_17);
            tracer.write_register(s_12_18 as isize, s_12_17);
        };
        // C s_12_20: const #90608u : u32
        let s_12_20: u32 = 90608;
        // D s_12_21: read-reg s_12_20:struct
        let s_12_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_20 as isize);
            tracer.read_register(s_12_20 as isize, value);
            value
        };
        // C s_12_22: const #90608u : u32
        let s_12_22: u32 = 90608;
        // N s_12_23: write-reg s_12_22 <= s_12_21
        let s_12_23: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_12_22 as isize, s_12_21);
            tracer.write_register(s_12_22 as isize, s_12_21);
        };
        // C s_12_24: const #90608u : u32
        let s_12_24: u32 = 90608;
        // D s_12_25: read-reg s_12_24:struct
        let s_12_25: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_24 as isize);
            tracer.read_register(s_12_24 as isize, value);
            value
        };
        // C s_12_26: const #90608u : u32
        let s_12_26: u32 = 90608;
        // N s_12_27: write-reg s_12_26 <= s_12_25
        let s_12_27: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_12_26 as isize, s_12_25);
            tracer.write_register(s_12_26 as isize, s_12_25);
        };
        // C s_12_28: const #90608u : u32
        let s_12_28: u32 = 90608;
        // D s_12_29: read-reg s_12_28:struct
        let s_12_29: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_28 as isize);
            tracer.read_register(s_12_28 as isize, value);
            value
        };
        // C s_12_30: const #90608u : u32
        let s_12_30: u32 = 90608;
        // N s_12_31: write-reg s_12_30 <= s_12_29
        let s_12_31: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_12_30 as isize, s_12_29);
            tracer.write_register(s_12_30 as isize, s_12_29);
        };
        // C s_12_32: const #90608u : u32
        let s_12_32: u32 = 90608;
        // D s_12_33: read-reg s_12_32:struct
        let s_12_33: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_32 as isize);
            tracer.read_register(s_12_32 as isize, value);
            value
        };
        // C s_12_34: const #90608u : u32
        let s_12_34: u32 = 90608;
        // N s_12_35: write-reg s_12_34 <= s_12_33
        let s_12_35: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_12_34 as isize, s_12_33);
            tracer.write_register(s_12_34 as isize, s_12_33);
        };
        // C s_12_36: const #90608u : u32
        let s_12_36: u32 = 90608;
        // D s_12_37: read-reg s_12_36:struct
        let s_12_37: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_36 as isize);
            tracer.read_register(s_12_36 as isize, value);
            value
        };
        // C s_12_38: const #90608u : u32
        let s_12_38: u32 = 90608;
        // N s_12_39: write-reg s_12_38 <= s_12_37
        let s_12_39: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_12_38 as isize, s_12_37);
            tracer.write_register(s_12_38 as isize, s_12_37);
        };
        // C s_12_40: const #14600u : u32
        let s_12_40: u32 = 14600;
        // D s_12_41: read-reg s_12_40:struct
        let s_12_41: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_40 as isize);
            tracer.read_register(s_12_40 as isize, value);
            value
        };
        // C s_12_42: const #14600u : u32
        let s_12_42: u32 = 14600;
        // N s_12_43: write-reg s_12_42 <= s_12_41
        let s_12_43: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_12_42 as isize, s_12_41);
            tracer.write_register(s_12_42 as isize, s_12_41);
        };
        // C s_12_44: const #14896u : u32
        let s_12_44: u32 = 14896;
        // D s_12_45: read-reg s_12_44:struct
        let s_12_45: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_44 as isize);
            tracer.read_register(s_12_44 as isize, value);
            value
        };
        // C s_12_46: const #14896u : u32
        let s_12_46: u32 = 14896;
        // N s_12_47: write-reg s_12_46 <= s_12_45
        let s_12_47: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_12_46 as isize, s_12_45);
            tracer.write_register(s_12_46 as isize, s_12_45);
        };
        // C s_12_48: const #14896u : u32
        let s_12_48: u32 = 14896;
        // D s_12_49: read-reg s_12_48:struct
        let s_12_49: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_48 as isize);
            tracer.read_register(s_12_48 as isize, value);
            value
        };
        // C s_12_50: const #14896u : u32
        let s_12_50: u32 = 14896;
        // N s_12_51: write-reg s_12_50 <= s_12_49
        let s_12_51: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_12_50 as isize, s_12_49);
            tracer.write_register(s_12_50 as isize, s_12_49);
        };
        // C s_12_52: const #14896u : u32
        let s_12_52: u32 = 14896;
        // D s_12_53: read-reg s_12_52:struct
        let s_12_53: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_52 as isize);
            tracer.read_register(s_12_52 as isize, value);
            value
        };
        // C s_12_54: const #14896u : u32
        let s_12_54: u32 = 14896;
        // N s_12_55: write-reg s_12_54 <= s_12_53
        let s_12_55: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_12_54 as isize, s_12_53);
            tracer.write_register(s_12_54 as isize, s_12_53);
        };
        // C s_12_56: const #14896u : u32
        let s_12_56: u32 = 14896;
        // D s_12_57: read-reg s_12_56:struct
        let s_12_57: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_56 as isize);
            tracer.read_register(s_12_56 as isize, value);
            value
        };
        // C s_12_58: const #14896u : u32
        let s_12_58: u32 = 14896;
        // N s_12_59: write-reg s_12_58 <= s_12_57
        let s_12_59: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_12_58 as isize, s_12_57);
            tracer.write_register(s_12_58 as isize, s_12_57);
        };
        // C s_12_60: const #14896u : u32
        let s_12_60: u32 = 14896;
        // D s_12_61: read-reg s_12_60:struct
        let s_12_61: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_60 as isize);
            tracer.read_register(s_12_60 as isize, value);
            value
        };
        // C s_12_62: const #14896u : u32
        let s_12_62: u32 = 14896;
        // N s_12_63: write-reg s_12_62 <= s_12_61
        let s_12_63: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_12_62 as isize, s_12_61);
            tracer.write_register(s_12_62 as isize, s_12_61);
        };
        // C s_12_64: const #14896u : u32
        let s_12_64: u32 = 14896;
        // D s_12_65: read-reg s_12_64:struct
        let s_12_65: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_64 as isize);
            tracer.read_register(s_12_64 as isize, value);
            value
        };
        // C s_12_66: const #14896u : u32
        let s_12_66: u32 = 14896;
        // N s_12_67: write-reg s_12_66 <= s_12_65
        let s_12_67: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_12_66 as isize, s_12_65);
            tracer.write_register(s_12_66 as isize, s_12_65);
        };
        // C s_12_68: const #14896u : u32
        let s_12_68: u32 = 14896;
        // D s_12_69: read-reg s_12_68:struct
        let s_12_69: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_68 as isize);
            tracer.read_register(s_12_68 as isize, value);
            value
        };
        // C s_12_70: const #14896u : u32
        let s_12_70: u32 = 14896;
        // N s_12_71: write-reg s_12_70 <= s_12_69
        let s_12_71: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_12_70 as isize, s_12_69);
            tracer.write_register(s_12_70 as isize, s_12_69);
        };
        // C s_12_72: const #14896u : u32
        let s_12_72: u32 = 14896;
        // D s_12_73: read-reg s_12_72:struct
        let s_12_73: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_72 as isize);
            tracer.read_register(s_12_72 as isize, value);
            value
        };
        // C s_12_74: const #14896u : u32
        let s_12_74: u32 = 14896;
        // N s_12_75: write-reg s_12_74 <= s_12_73
        let s_12_75: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_12_74 as isize, s_12_73);
            tracer.write_register(s_12_74 as isize, s_12_73);
        };
        // C s_12_76: const #14896u : u32
        let s_12_76: u32 = 14896;
        // D s_12_77: read-reg s_12_76:struct
        let s_12_77: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_76 as isize);
            tracer.read_register(s_12_76 as isize, value);
            value
        };
        // C s_12_78: const #14896u : u32
        let s_12_78: u32 = 14896;
        // N s_12_79: write-reg s_12_78 <= s_12_77
        let s_12_79: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_12_78 as isize, s_12_77);
            tracer.write_register(s_12_78 as isize, s_12_77);
        };
        // C s_12_80: const #14896u : u32
        let s_12_80: u32 = 14896;
        // D s_12_81: read-reg s_12_80:struct
        let s_12_81: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_80 as isize);
            tracer.read_register(s_12_80 as isize, value);
            value
        };
        // C s_12_82: const #14896u : u32
        let s_12_82: u32 = 14896;
        // N s_12_83: write-reg s_12_82 <= s_12_81
        let s_12_83: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_12_82 as isize, s_12_81);
            tracer.write_register(s_12_82 as isize, s_12_81);
        };
        // C s_12_84: const #14896u : u32
        let s_12_84: u32 = 14896;
        // D s_12_85: read-reg s_12_84:struct
        let s_12_85: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_84 as isize);
            tracer.read_register(s_12_84 as isize, value);
            value
        };
        // C s_12_86: const #14896u : u32
        let s_12_86: u32 = 14896;
        // N s_12_87: write-reg s_12_86 <= s_12_85
        let s_12_87: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_12_86 as isize, s_12_85);
            tracer.write_register(s_12_86 as isize, s_12_85);
        };
        // C s_12_88: const #14896u : u32
        let s_12_88: u32 = 14896;
        // D s_12_89: read-reg s_12_88:struct
        let s_12_89: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_88 as isize);
            tracer.read_register(s_12_88 as isize, value);
            value
        };
        // C s_12_90: const #14896u : u32
        let s_12_90: u32 = 14896;
        // N s_12_91: write-reg s_12_90 <= s_12_89
        let s_12_91: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_12_90 as isize, s_12_89);
            tracer.write_register(s_12_90 as isize, s_12_89);
        };
        // C s_12_92: const #14896u : u32
        let s_12_92: u32 = 14896;
        // D s_12_93: read-reg s_12_92:struct
        let s_12_93: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_92 as isize);
            tracer.read_register(s_12_92 as isize, value);
            value
        };
        // C s_12_94: const #14896u : u32
        let s_12_94: u32 = 14896;
        // N s_12_95: write-reg s_12_94 <= s_12_93
        let s_12_95: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_12_94 as isize, s_12_93);
            tracer.write_register(s_12_94 as isize, s_12_93);
        };
        // C s_12_96: const #() : ()
        let s_12_96: () = ();
        // S s_12_97: call EncodePARange(s_12_96)
        let s_12_97: u8 = EncodePARange(state, tracer, s_12_96);
        // C s_12_98: const #14896u : u32
        let s_12_98: u32 = 14896;
        // D s_12_99: read-reg s_12_98:struct
        let s_12_99: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_98 as isize);
            tracer.read_register(s_12_98 as isize, value);
            value
        };
        // C s_12_100: const #14896u : u32
        let s_12_100: u32 = 14896;
        // N s_12_101: write-reg s_12_100 <= s_12_99
        let s_12_101: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_12_100 as isize, s_12_99);
            tracer.write_register(s_12_100 as isize, s_12_99);
        };
        // C s_12_102: const #() : ()
        let s_12_102: () = ();
        // S s_12_103: call EncodeVARange(s_12_102)
        let s_12_103: u8 = EncodeVARange(state, tracer, s_12_102);
        // C s_12_104: const #90600u : u32
        let s_12_104: u32 = 90600;
        // D s_12_105: read-reg s_12_104:struct
        let s_12_105: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_104 as isize);
            tracer.read_register(s_12_104 as isize, value);
            value
        };
        // C s_12_106: const #90600u : u32
        let s_12_106: u32 = 90600;
        // N s_12_107: write-reg s_12_106 <= s_12_105
        let s_12_107: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_12_106 as isize, s_12_105);
            tracer.write_register(s_12_106 as isize, s_12_105);
        };
        // C s_12_108: const #103144u : u32
        let s_12_108: u32 = 103144;
        // D s_12_109: read-reg s_12_108:struct
        let s_12_109: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_108 as isize);
            tracer.read_register(s_12_108 as isize, value);
            value
        };
        // C s_12_110: const #103144u : u32
        let s_12_110: u32 = 103144;
        // N s_12_111: write-reg s_12_110 <= s_12_109
        let s_12_111: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_12_110 as isize, s_12_109);
            tracer.write_register(s_12_110 as isize, s_12_109);
        };
        // C s_12_112: const #103144u : u32
        let s_12_112: u32 = 103144;
        // D s_12_113: read-reg s_12_112:struct
        let s_12_113: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_112 as isize);
            tracer.read_register(s_12_112 as isize, value);
            value
        };
        // C s_12_114: const #103144u : u32
        let s_12_114: u32 = 103144;
        // N s_12_115: write-reg s_12_114 <= s_12_113
        let s_12_115: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_12_114 as isize, s_12_113);
            tracer.write_register(s_12_114 as isize, s_12_113);
        };
        // C s_12_116: const #103144u : u32
        let s_12_116: u32 = 103144;
        // D s_12_117: read-reg s_12_116:struct
        let s_12_117: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_116 as isize);
            tracer.read_register(s_12_116 as isize, value);
            value
        };
        // C s_12_118: const #103144u : u32
        let s_12_118: u32 = 103144;
        // N s_12_119: write-reg s_12_118 <= s_12_117
        let s_12_119: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_12_118 as isize, s_12_117);
            tracer.write_register(s_12_118 as isize, s_12_117);
        };
        // C s_12_120: const #103144u : u32
        let s_12_120: u32 = 103144;
        // D s_12_121: read-reg s_12_120:struct
        let s_12_121: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_120 as isize);
            tracer.read_register(s_12_120 as isize, value);
            value
        };
        // C s_12_122: const #103144u : u32
        let s_12_122: u32 = 103144;
        // N s_12_123: write-reg s_12_122 <= s_12_121
        let s_12_123: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_12_122 as isize, s_12_121);
            tracer.write_register(s_12_122 as isize, s_12_121);
        };
        // C s_12_124: const #103144u : u32
        let s_12_124: u32 = 103144;
        // D s_12_125: read-reg s_12_124:struct
        let s_12_125: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_124 as isize);
            tracer.read_register(s_12_124 as isize, value);
            value
        };
        // C s_12_126: const #103144u : u32
        let s_12_126: u32 = 103144;
        // N s_12_127: write-reg s_12_126 <= s_12_125
        let s_12_127: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_12_126 as isize, s_12_125);
            tracer.write_register(s_12_126 as isize, s_12_125);
        };
        // C s_12_128: const #103144u : u32
        let s_12_128: u32 = 103144;
        // D s_12_129: read-reg s_12_128:struct
        let s_12_129: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_128 as isize);
            tracer.read_register(s_12_128 as isize, value);
            value
        };
        // C s_12_130: const #103144u : u32
        let s_12_130: u32 = 103144;
        // N s_12_131: write-reg s_12_130 <= s_12_129
        let s_12_131: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_12_130 as isize, s_12_129);
            tracer.write_register(s_12_130 as isize, s_12_129);
        };
        // C s_12_132: const #103144u : u32
        let s_12_132: u32 = 103144;
        // D s_12_133: read-reg s_12_132:struct
        let s_12_133: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_132 as isize);
            tracer.read_register(s_12_132 as isize, value);
            value
        };
        // C s_12_134: const #103144u : u32
        let s_12_134: u32 = 103144;
        // N s_12_135: write-reg s_12_134 <= s_12_133
        let s_12_135: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_12_134 as isize, s_12_133);
            tracer.write_register(s_12_134 as isize, s_12_133);
        };
        // C s_12_136: const #103144u : u32
        let s_12_136: u32 = 103144;
        // D s_12_137: read-reg s_12_136:struct
        let s_12_137: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_136 as isize);
            tracer.read_register(s_12_136 as isize, value);
            value
        };
        // C s_12_138: const #103144u : u32
        let s_12_138: u32 = 103144;
        // N s_12_139: write-reg s_12_138 <= s_12_137
        let s_12_139: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_12_138 as isize, s_12_137);
            tracer.write_register(s_12_138 as isize, s_12_137);
        };
        // C s_12_140: const #101824u : u32
        let s_12_140: u32 = 101824;
        // D s_12_141: read-reg s_12_140:struct
        let s_12_141: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_140 as isize);
            tracer.read_register(s_12_140 as isize, value);
            value
        };
        // C s_12_142: const #101824u : u32
        let s_12_142: u32 = 101824;
        // N s_12_143: write-reg s_12_142 <= s_12_141
        let s_12_143: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_12_142 as isize, s_12_141);
            tracer.write_register(s_12_142 as isize, s_12_141);
        };
        // C s_12_144: const #101824u : u32
        let s_12_144: u32 = 101824;
        // D s_12_145: read-reg s_12_144:struct
        let s_12_145: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_144 as isize);
            tracer.read_register(s_12_144 as isize, value);
            value
        };
        // C s_12_146: const #101824u : u32
        let s_12_146: u32 = 101824;
        // N s_12_147: write-reg s_12_146 <= s_12_145
        let s_12_147: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_12_146 as isize, s_12_145);
            tracer.write_register(s_12_146 as isize, s_12_145);
        };
        // C s_12_148: const #101824u : u32
        let s_12_148: u32 = 101824;
        // D s_12_149: read-reg s_12_148:struct
        let s_12_149: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_148 as isize);
            tracer.read_register(s_12_148 as isize, value);
            value
        };
        // C s_12_150: const #101824u : u32
        let s_12_150: u32 = 101824;
        // N s_12_151: write-reg s_12_150 <= s_12_149
        let s_12_151: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_12_150 as isize, s_12_149);
            tracer.write_register(s_12_150 as isize, s_12_149);
        };
        // C s_12_152: const #101824u : u32
        let s_12_152: u32 = 101824;
        // D s_12_153: read-reg s_12_152:struct
        let s_12_153: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_152 as isize);
            tracer.read_register(s_12_152 as isize, value);
            value
        };
        // C s_12_154: const #101824u : u32
        let s_12_154: u32 = 101824;
        // N s_12_155: write-reg s_12_154 <= s_12_153
        let s_12_155: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_12_154 as isize, s_12_153);
            tracer.write_register(s_12_154 as isize, s_12_153);
        };
        // C s_12_156: const #101824u : u32
        let s_12_156: u32 = 101824;
        // D s_12_157: read-reg s_12_156:struct
        let s_12_157: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_156 as isize);
            tracer.read_register(s_12_156 as isize, value);
            value
        };
        // C s_12_158: const #101824u : u32
        let s_12_158: u32 = 101824;
        // N s_12_159: write-reg s_12_158 <= s_12_157
        let s_12_159: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_12_158 as isize, s_12_157);
            tracer.write_register(s_12_158 as isize, s_12_157);
        };
        // C s_12_160: const #101824u : u32
        let s_12_160: u32 = 101824;
        // D s_12_161: read-reg s_12_160:struct
        let s_12_161: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_160 as isize);
            tracer.read_register(s_12_160 as isize, value);
            value
        };
        // C s_12_162: const #101824u : u32
        let s_12_162: u32 = 101824;
        // N s_12_163: write-reg s_12_162 <= s_12_161
        let s_12_163: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_12_162 as isize, s_12_161);
            tracer.write_register(s_12_162 as isize, s_12_161);
        };
        // C s_12_164: const #101824u : u32
        let s_12_164: u32 = 101824;
        // D s_12_165: read-reg s_12_164:struct
        let s_12_165: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_164 as isize);
            tracer.read_register(s_12_164 as isize, value);
            value
        };
        // C s_12_166: const #101824u : u32
        let s_12_166: u32 = 101824;
        // N s_12_167: write-reg s_12_166 <= s_12_165
        let s_12_167: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_12_166 as isize, s_12_165);
            tracer.write_register(s_12_166 as isize, s_12_165);
        };
        // C s_12_168: const #11744u : u32
        let s_12_168: u32 = 11744;
        // D s_12_169: read-reg s_12_168:struct
        let s_12_169: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_168 as isize);
            tracer.read_register(s_12_168 as isize, value);
            value
        };
        // C s_12_170: const #11744u : u32
        let s_12_170: u32 = 11744;
        // N s_12_171: write-reg s_12_170 <= s_12_169
        let s_12_171: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_12_170 as isize, s_12_169);
            tracer.write_register(s_12_170 as isize, s_12_169);
        };
        // C s_12_172: const #11744u : u32
        let s_12_172: u32 = 11744;
        // D s_12_173: read-reg s_12_172:struct
        let s_12_173: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_172 as isize);
            tracer.read_register(s_12_172 as isize, value);
            value
        };
        // C s_12_174: const #11744u : u32
        let s_12_174: u32 = 11744;
        // N s_12_175: write-reg s_12_174 <= s_12_173
        let s_12_175: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_12_174 as isize, s_12_173);
            tracer.write_register(s_12_174 as isize, s_12_173);
        };
        // C s_12_176: const #() : ()
        let s_12_176: () = ();
        // S s_12_177: call HaveCNTSCExt(s_12_176)
        let s_12_177: bool = HaveCNTSCExt(state, tracer, s_12_176);
        // N s_12_178: branch s_12_177 b350 b13
        if s_12_177 {
            return block_350(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #101824u : u32
        let s_14_0: u32 = 101824;
        // D s_14_1: read-reg s_14_0:struct
        let s_14_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_14_0 as isize);
            tracer.read_register(s_14_0 as isize, value);
            value
        };
        // C s_14_2: const #101824u : u32
        let s_14_2: u32 = 101824;
        // N s_14_3: write-reg s_14_2 <= s_14_1
        let s_14_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_14_2 as isize, s_14_1);
            tracer.write_register(s_14_2 as isize, s_14_1);
        };
        // C s_14_4: const #21960u : u32
        let s_14_4: u32 = 21960;
        // D s_14_5: read-reg s_14_4:struct
        let s_14_5: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_14_4 as isize);
            tracer.read_register(s_14_4 as isize, value);
            value
        };
        // C s_14_6: const #21960u : u32
        let s_14_6: u32 = 21960;
        // N s_14_7: write-reg s_14_6 <= s_14_5
        let s_14_7: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_14_6 as isize, s_14_5);
            tracer.write_register(s_14_6 as isize, s_14_5);
        };
        // C s_14_8: const #19384u : u32
        let s_14_8: u32 = 19384;
        // D s_14_9: read-reg s_14_8:struct
        let s_14_9: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_14_8 as isize);
            tracer.read_register(s_14_8 as isize, value);
            value
        };
        // C s_14_10: const #19384u : u32
        let s_14_10: u32 = 19384;
        // N s_14_11: write-reg s_14_10 <= s_14_9
        let s_14_11: () = {
            state
                .write_register::<ProductType700c18a878c5601b>(s_14_10 as isize, s_14_9);
            tracer.write_register(s_14_10 as isize, s_14_9);
        };
        // C s_14_12: const #20080u : u32
        let s_14_12: u32 = 20080;
        // D s_14_13: read-reg s_14_12:struct
        let s_14_13: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_14_12 as isize);
            tracer.read_register(s_14_12 as isize, value);
            value
        };
        // C s_14_14: const #20080u : u32
        let s_14_14: u32 = 20080;
        // N s_14_15: write-reg s_14_14 <= s_14_13
        let s_14_15: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_14_14 as isize, s_14_13);
            tracer.write_register(s_14_14 as isize, s_14_13);
        };
        // C s_14_16: const #22504u : u32
        let s_14_16: u32 = 22504;
        // D s_14_17: read-reg s_14_16:struct
        let s_14_17: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_14_16 as isize);
            tracer.read_register(s_14_16 as isize, value);
            value
        };
        // C s_14_18: const #22504u : u32
        let s_14_18: u32 = 22504;
        // N s_14_19: write-reg s_14_18 <= s_14_17
        let s_14_19: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_14_18 as isize, s_14_17);
            tracer.write_register(s_14_18 as isize, s_14_17);
        };
        // C s_14_20: const #10512u : u32
        let s_14_20: u32 = 10512;
        // D s_14_21: read-reg s_14_20:struct
        let s_14_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_14_20 as isize);
            tracer.read_register(s_14_20 as isize, value);
            value
        };
        // C s_14_22: const #10512u : u32
        let s_14_22: u32 = 10512;
        // N s_14_23: write-reg s_14_22 <= s_14_21
        let s_14_23: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_14_22 as isize, s_14_21);
            tracer.write_register(s_14_22 as isize, s_14_21);
        };
        // C s_14_24: const #10512u : u32
        let s_14_24: u32 = 10512;
        // D s_14_25: read-reg s_14_24:struct
        let s_14_25: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_14_24 as isize);
            tracer.read_register(s_14_24 as isize, value);
            value
        };
        // C s_14_26: const #10512u : u32
        let s_14_26: u32 = 10512;
        // N s_14_27: write-reg s_14_26 <= s_14_25
        let s_14_27: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_14_26 as isize, s_14_25);
            tracer.write_register(s_14_26 as isize, s_14_25);
        };
        // C s_14_28: const #10512u : u32
        let s_14_28: u32 = 10512;
        // D s_14_29: read-reg s_14_28:struct
        let s_14_29: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_14_28 as isize);
            tracer.read_register(s_14_28 as isize, value);
            value
        };
        // C s_14_30: const #10512u : u32
        let s_14_30: u32 = 10512;
        // N s_14_31: write-reg s_14_30 <= s_14_29
        let s_14_31: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_14_30 as isize, s_14_29);
            tracer.write_register(s_14_30 as isize, s_14_29);
        };
        // C s_14_32: const #1s : i
        let s_14_32: i128 = 1;
        // C s_14_33: const #12760u : u32
        let s_14_33: u32 = 12760;
        // D s_14_34: read-reg s_14_33:i
        let s_14_34: i128 = {
            let value = state.read_register::<i128>(s_14_33 as isize);
            tracer.read_register(s_14_33 as isize, value);
            value
        };
        // D s_14_35: sub s_14_34 s_14_32
        let s_14_35: i128 = ((s_14_34) - (s_14_32));
        // C s_14_36: const #3s : i
        let s_14_36: i128 = 3;
        // C s_14_37: const #0s : i
        let s_14_37: i128 = 0;
        // D s_14_38: call integer_subrange(s_14_35, s_14_36, s_14_37)
        let s_14_38: Bits = integer_subrange(state, tracer, s_14_35, s_14_36, s_14_37);
        // C s_14_39: const #10512u : u32
        let s_14_39: u32 = 10512;
        // D s_14_40: read-reg s_14_39:struct
        let s_14_40: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_14_39 as isize);
            tracer.read_register(s_14_39 as isize, value);
            value
        };
        // C s_14_41: const #10512u : u32
        let s_14_41: u32 = 10512;
        // N s_14_42: write-reg s_14_41 <= s_14_40
        let s_14_42: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_14_41 as isize, s_14_40);
            tracer.write_register(s_14_41 as isize, s_14_40);
        };
        // C s_14_43: const #7s : i
        let s_14_43: i128 = 7;
        // C s_14_44: const #0s : i
        let s_14_44: i128 = 0;
        // C s_14_45: const #90472u : u32
        let s_14_45: u32 = 90472;
        // D s_14_46: read-reg s_14_45:i
        let s_14_46: i128 = {
            let value = state.read_register::<i128>(s_14_45 as isize);
            tracer.read_register(s_14_45 as isize, value);
            value
        };
        // D s_14_47: call integer_subrange(s_14_46, s_14_43, s_14_44)
        let s_14_47: Bits = integer_subrange(state, tracer, s_14_46, s_14_43, s_14_44);
        // C s_14_48: const #14656u : u32
        let s_14_48: u32 = 14656;
        // D s_14_49: read-reg s_14_48:struct
        let s_14_49: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_14_48 as isize);
            tracer.read_register(s_14_48 as isize, value);
            value
        };
        // C s_14_50: const #14656u : u32
        let s_14_50: u32 = 14656;
        // N s_14_51: write-reg s_14_50 <= s_14_49
        let s_14_51: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_14_50 as isize, s_14_49);
            tracer.write_register(s_14_50 as isize, s_14_49);
        };
        // C s_14_52: const #7s : i
        let s_14_52: i128 = 7;
        // C s_14_53: const #0s : i
        let s_14_53: i128 = 0;
        // C s_14_54: const #90784u : u32
        let s_14_54: u32 = 90784;
        // D s_14_55: read-reg s_14_54:i
        let s_14_55: i128 = {
            let value = state.read_register::<i128>(s_14_54 as isize);
            tracer.read_register(s_14_54 as isize, value);
            value
        };
        // D s_14_56: call integer_subrange(s_14_55, s_14_52, s_14_53)
        let s_14_56: Bits = integer_subrange(state, tracer, s_14_55, s_14_52, s_14_53);
        // C s_14_57: const #14656u : u32
        let s_14_57: u32 = 14656;
        // D s_14_58: read-reg s_14_57:struct
        let s_14_58: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_14_57 as isize);
            tracer.read_register(s_14_57 as isize, value);
            value
        };
        // C s_14_59: const #14656u : u32
        let s_14_59: u32 = 14656;
        // N s_14_60: write-reg s_14_59 <= s_14_58
        let s_14_60: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_14_59 as isize, s_14_58);
            tracer.write_register(s_14_59 as isize, s_14_58);
        };
        // C s_14_61: const #19296u : u32
        let s_14_61: u32 = 19296;
        // D s_14_62: read-reg s_14_61:struct
        let s_14_62: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_14_61 as isize);
            tracer.read_register(s_14_61 as isize, value);
            value
        };
        // C s_14_63: const #19296u : u32
        let s_14_63: u32 = 19296;
        // N s_14_64: write-reg s_14_63 <= s_14_62
        let s_14_64: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_14_63 as isize, s_14_62);
            tracer.write_register(s_14_63 as isize, s_14_62);
        };
        // C s_14_65: const #19296u : u32
        let s_14_65: u32 = 19296;
        // D s_14_66: read-reg s_14_65:struct
        let s_14_66: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_14_65 as isize);
            tracer.read_register(s_14_65 as isize, value);
            value
        };
        // C s_14_67: const #19296u : u32
        let s_14_67: u32 = 19296;
        // N s_14_68: write-reg s_14_67 <= s_14_66
        let s_14_68: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_14_67 as isize, s_14_66);
            tracer.write_register(s_14_67 as isize, s_14_66);
        };
        // C s_14_69: const #19296u : u32
        let s_14_69: u32 = 19296;
        // D s_14_70: read-reg s_14_69:struct
        let s_14_70: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_14_69 as isize);
            tracer.read_register(s_14_69 as isize, value);
            value
        };
        // C s_14_71: const #19296u : u32
        let s_14_71: u32 = 19296;
        // N s_14_72: write-reg s_14_71 <= s_14_70
        let s_14_72: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_14_71 as isize, s_14_70);
            tracer.write_register(s_14_71 as isize, s_14_70);
        };
        // C s_14_73: const #14192u : u32
        let s_14_73: u32 = 14192;
        // D s_14_74: read-reg s_14_73:struct
        let s_14_74: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_14_73 as isize);
            tracer.read_register(s_14_73 as isize, value);
            value
        };
        // C s_14_75: const #14192u : u32
        let s_14_75: u32 = 14192;
        // N s_14_76: write-reg s_14_75 <= s_14_74
        let s_14_76: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_14_75 as isize, s_14_74);
            tracer.write_register(s_14_75 as isize, s_14_74);
        };
        // C s_14_77: const #14192u : u32
        let s_14_77: u32 = 14192;
        // D s_14_78: read-reg s_14_77:struct
        let s_14_78: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_14_77 as isize);
            tracer.read_register(s_14_77 as isize, value);
            value
        };
        // C s_14_79: const #14192u : u32
        let s_14_79: u32 = 14192;
        // N s_14_80: write-reg s_14_79 <= s_14_78
        let s_14_80: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_14_79 as isize, s_14_78);
            tracer.write_register(s_14_79 as isize, s_14_78);
        };
        // C s_14_81: const #0s : i
        let s_14_81: i128 = 0;
        // C s_14_82: const #20888u : u32
        let s_14_82: u32 = 20888;
        // D s_14_83: read-reg s_14_82:[struct; 4]
        let s_14_83: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_14_82 as isize);
            tracer.read_register(s_14_82 as isize, value);
            value
        };
        // D s_14_84: read-element s_14_83[s_14_81]
        let s_14_84: ProductType5c790c8ef59cc8b2 = s_14_83[(s_14_81) as usize];
        // C s_14_85: const #0s : i
        let s_14_85: i128 = 0;
        // C s_14_86: const #20888u : u32
        let s_14_86: u32 = 20888;
        // D s_14_87: read-reg s_14_86:[struct; 4]
        let s_14_87: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_14_86 as isize);
            tracer.read_register(s_14_86 as isize, value);
            value
        };
        // D s_14_88: mutate-element s_14_87[s_14_85] <= s_14_84
        let s_14_88: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut local = s_14_87.clone();
            local[(s_14_85) as usize] = s_14_84;
            local
        };
        // D s_14_89: cast cvt s_14_88 -> [struct; 0]
        let s_14_89: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_14_88,
        );
        // D s_14_90: cast cvt s_14_89 -> [struct; 4]
        let s_14_90: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_14_89);
            buf
        };
        // C s_14_91: const #20888u : u32
        let s_14_91: u32 = 20888;
        // N s_14_92: write-reg s_14_91 <= s_14_90
        let s_14_92: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_14_91 as isize, s_14_90);
            tracer.write_register(s_14_91 as isize, s_14_90);
        };
        // C s_14_93: const #1s : i
        let s_14_93: i128 = 1;
        // C s_14_94: const #20888u : u32
        let s_14_94: u32 = 20888;
        // D s_14_95: read-reg s_14_94:[struct; 4]
        let s_14_95: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_14_94 as isize);
            tracer.read_register(s_14_94 as isize, value);
            value
        };
        // D s_14_96: read-element s_14_95[s_14_93]
        let s_14_96: ProductType5c790c8ef59cc8b2 = s_14_95[(s_14_93) as usize];
        // C s_14_97: const #1s : i
        let s_14_97: i128 = 1;
        // C s_14_98: const #20888u : u32
        let s_14_98: u32 = 20888;
        // D s_14_99: read-reg s_14_98:[struct; 4]
        let s_14_99: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_14_98 as isize);
            tracer.read_register(s_14_98 as isize, value);
            value
        };
        // D s_14_100: mutate-element s_14_99[s_14_97] <= s_14_96
        let s_14_100: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut local = s_14_99.clone();
            local[(s_14_97) as usize] = s_14_96;
            local
        };
        // D s_14_101: cast cvt s_14_100 -> [struct; 0]
        let s_14_101: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_14_100,
        );
        // D s_14_102: cast cvt s_14_101 -> [struct; 4]
        let s_14_102: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_14_101);
            buf
        };
        // C s_14_103: const #20888u : u32
        let s_14_103: u32 = 20888;
        // N s_14_104: write-reg s_14_103 <= s_14_102
        let s_14_104: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_14_103 as isize, s_14_102);
            tracer.write_register(s_14_103 as isize, s_14_102);
        };
        // C s_14_105: const #2s : i
        let s_14_105: i128 = 2;
        // C s_14_106: const #20888u : u32
        let s_14_106: u32 = 20888;
        // D s_14_107: read-reg s_14_106:[struct; 4]
        let s_14_107: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_14_106 as isize);
            tracer.read_register(s_14_106 as isize, value);
            value
        };
        // D s_14_108: read-element s_14_107[s_14_105]
        let s_14_108: ProductType5c790c8ef59cc8b2 = s_14_107[(s_14_105) as usize];
        // C s_14_109: const #2s : i
        let s_14_109: i128 = 2;
        // C s_14_110: const #20888u : u32
        let s_14_110: u32 = 20888;
        // D s_14_111: read-reg s_14_110:[struct; 4]
        let s_14_111: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_14_110 as isize);
            tracer.read_register(s_14_110 as isize, value);
            value
        };
        // D s_14_112: mutate-element s_14_111[s_14_109] <= s_14_108
        let s_14_112: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut local = s_14_111.clone();
            local[(s_14_109) as usize] = s_14_108;
            local
        };
        // D s_14_113: cast cvt s_14_112 -> [struct; 0]
        let s_14_113: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_14_112,
        );
        // D s_14_114: cast cvt s_14_113 -> [struct; 4]
        let s_14_114: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_14_113);
            buf
        };
        // C s_14_115: const #20888u : u32
        let s_14_115: u32 = 20888;
        // N s_14_116: write-reg s_14_115 <= s_14_114
        let s_14_116: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_14_115 as isize, s_14_114);
            tracer.write_register(s_14_115 as isize, s_14_114);
        };
        // C s_14_117: const #3s : i
        let s_14_117: i128 = 3;
        // C s_14_118: const #20888u : u32
        let s_14_118: u32 = 20888;
        // D s_14_119: read-reg s_14_118:[struct; 4]
        let s_14_119: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_14_118 as isize);
            tracer.read_register(s_14_118 as isize, value);
            value
        };
        // D s_14_120: read-element s_14_119[s_14_117]
        let s_14_120: ProductType5c790c8ef59cc8b2 = s_14_119[(s_14_117) as usize];
        // C s_14_121: const #3s : i
        let s_14_121: i128 = 3;
        // C s_14_122: const #20888u : u32
        let s_14_122: u32 = 20888;
        // D s_14_123: read-reg s_14_122:[struct; 4]
        let s_14_123: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_14_122 as isize);
            tracer.read_register(s_14_122 as isize, value);
            value
        };
        // D s_14_124: mutate-element s_14_123[s_14_121] <= s_14_120
        let s_14_124: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut local = s_14_123.clone();
            local[(s_14_121) as usize] = s_14_120;
            local
        };
        // D s_14_125: cast cvt s_14_124 -> [struct; 0]
        let s_14_125: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_14_124,
        );
        // D s_14_126: cast cvt s_14_125 -> [struct; 4]
        let s_14_126: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_14_125);
            buf
        };
        // C s_14_127: const #20888u : u32
        let s_14_127: u32 = 20888;
        // N s_14_128: write-reg s_14_127 <= s_14_126
        let s_14_128: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_14_127 as isize, s_14_126);
            tracer.write_register(s_14_127 as isize, s_14_126);
        };
        // C s_14_129: const #15616u : u32
        let s_14_129: u32 = 15616;
        // D s_14_130: read-reg s_14_129:struct
        let s_14_130: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_14_129 as isize);
            tracer.read_register(s_14_129 as isize, value);
            value
        };
        // C s_14_131: const #15616u : u32
        let s_14_131: u32 = 15616;
        // N s_14_132: write-reg s_14_131 <= s_14_130
        let s_14_132: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_14_131 as isize, s_14_130);
            tracer.write_register(s_14_131 as isize, s_14_130);
        };
        // C s_14_133: const #15472u : u32
        let s_14_133: u32 = 15472;
        // D s_14_134: read-reg s_14_133:struct
        let s_14_134: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_14_133 as isize);
            tracer.read_register(s_14_133 as isize, value);
            value
        };
        // C s_14_135: const #15472u : u32
        let s_14_135: u32 = 15472;
        // N s_14_136: write-reg s_14_135 <= s_14_134
        let s_14_136: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_14_135 as isize, s_14_134);
            tracer.write_register(s_14_135 as isize, s_14_134);
        };
        // C s_14_137: const #1504u : u32
        let s_14_137: u32 = 1504;
        // D s_14_138: read-reg s_14_137:struct
        let s_14_138: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_14_137 as isize);
            tracer.read_register(s_14_137 as isize, value);
            value
        };
        // C s_14_139: const #1504u : u32
        let s_14_139: u32 = 1504;
        // N s_14_140: write-reg s_14_139 <= s_14_138
        let s_14_140: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_14_139 as isize, s_14_138);
            tracer.write_register(s_14_139 as isize, s_14_138);
        };
        // C s_14_141: const #1504u : u32
        let s_14_141: u32 = 1504;
        // D s_14_142: read-reg s_14_141:struct
        let s_14_142: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_14_141 as isize);
            tracer.read_register(s_14_141 as isize, value);
            value
        };
        // C s_14_143: const #1504u : u32
        let s_14_143: u32 = 1504;
        // N s_14_144: write-reg s_14_143 <= s_14_142
        let s_14_144: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_14_143 as isize, s_14_142);
            tracer.write_register(s_14_143 as isize, s_14_142);
        };
        // C s_14_145: const #90624u : u32
        let s_14_145: u32 = 90624;
        // D s_14_146: read-reg s_14_145:struct
        let s_14_146: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_14_145 as isize);
            tracer.read_register(s_14_145 as isize, value);
            value
        };
        // C s_14_147: const #90624u : u32
        let s_14_147: u32 = 90624;
        // N s_14_148: write-reg s_14_147 <= s_14_146
        let s_14_148: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_14_147 as isize, s_14_146);
            tracer.write_register(s_14_147 as isize, s_14_146);
        };
        // C s_14_149: const #101824u : u32
        let s_14_149: u32 = 101824;
        // D s_14_150: read-reg s_14_149:struct
        let s_14_150: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_14_149 as isize);
            tracer.read_register(s_14_149 as isize, value);
            value
        };
        // C s_14_151: const #101824u : u32
        let s_14_151: u32 = 101824;
        // N s_14_152: write-reg s_14_151 <= s_14_150
        let s_14_152: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_14_151 as isize, s_14_150);
            tracer.write_register(s_14_151 as isize, s_14_150);
        };
        // C s_14_153: const #101824u : u32
        let s_14_153: u32 = 101824;
        // D s_14_154: read-reg s_14_153:struct
        let s_14_154: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_14_153 as isize);
            tracer.read_register(s_14_153 as isize, value);
            value
        };
        // C s_14_155: const #101824u : u32
        let s_14_155: u32 = 101824;
        // N s_14_156: write-reg s_14_155 <= s_14_154
        let s_14_156: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_14_155 as isize, s_14_154);
            tracer.write_register(s_14_155 as isize, s_14_154);
        };
        // C s_14_157: const #101824u : u32
        let s_14_157: u32 = 101824;
        // D s_14_158: read-reg s_14_157:struct
        let s_14_158: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_14_157 as isize);
            tracer.read_register(s_14_157 as isize, value);
            value
        };
        // C s_14_159: const #101824u : u32
        let s_14_159: u32 = 101824;
        // N s_14_160: write-reg s_14_159 <= s_14_158
        let s_14_160: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_14_159 as isize, s_14_158);
            tracer.write_register(s_14_159 as isize, s_14_158);
        };
        // C s_14_161: const #101824u : u32
        let s_14_161: u32 = 101824;
        // D s_14_162: read-reg s_14_161:struct
        let s_14_162: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_14_161 as isize);
            tracer.read_register(s_14_161 as isize, value);
            value
        };
        // C s_14_163: const #101824u : u32
        let s_14_163: u32 = 101824;
        // N s_14_164: write-reg s_14_163 <= s_14_162
        let s_14_164: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_14_163 as isize, s_14_162);
            tracer.write_register(s_14_163 as isize, s_14_162);
        };
        // C s_14_165: const #101824u : u32
        let s_14_165: u32 = 101824;
        // D s_14_166: read-reg s_14_165:struct
        let s_14_166: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_14_165 as isize);
            tracer.read_register(s_14_165 as isize, value);
            value
        };
        // C s_14_167: const #101824u : u32
        let s_14_167: u32 = 101824;
        // N s_14_168: write-reg s_14_167 <= s_14_166
        let s_14_168: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_14_167 as isize, s_14_166);
            tracer.write_register(s_14_167 as isize, s_14_166);
        };
        // C s_14_169: const #17376u : u32
        let s_14_169: u32 = 17376;
        // D s_14_170: read-reg s_14_169:struct
        let s_14_170: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_14_169 as isize);
            tracer.read_register(s_14_169 as isize, value);
            value
        };
        // C s_14_171: const #17376u : u32
        let s_14_171: u32 = 17376;
        // N s_14_172: write-reg s_14_171 <= s_14_170
        let s_14_172: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_14_171 as isize, s_14_170);
            tracer.write_register(s_14_171 as isize, s_14_170);
        };
        // C s_14_173: const #20672u : u32
        let s_14_173: u32 = 20672;
        // D s_14_174: read-reg s_14_173:struct
        let s_14_174: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_14_173 as isize);
            tracer.read_register(s_14_173 as isize, value);
            value
        };
        // C s_14_175: const #20672u : u32
        let s_14_175: u32 = 20672;
        // N s_14_176: write-reg s_14_175 <= s_14_174
        let s_14_176: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_14_175 as isize, s_14_174);
            tracer.write_register(s_14_175 as isize, s_14_174);
        };
        // C s_14_177: const #20672u : u32
        let s_14_177: u32 = 20672;
        // D s_14_178: read-reg s_14_177:struct
        let s_14_178: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_14_177 as isize);
            tracer.read_register(s_14_177 as isize, value);
            value
        };
        // C s_14_179: const #20672u : u32
        let s_14_179: u32 = 20672;
        // N s_14_180: write-reg s_14_179 <= s_14_178
        let s_14_180: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_14_179 as isize, s_14_178);
            tracer.write_register(s_14_179 as isize, s_14_178);
        };
        // C s_14_181: const #20672u : u32
        let s_14_181: u32 = 20672;
        // D s_14_182: read-reg s_14_181:struct
        let s_14_182: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_14_181 as isize);
            tracer.read_register(s_14_181 as isize, value);
            value
        };
        // C s_14_183: const #20672u : u32
        let s_14_183: u32 = 20672;
        // N s_14_184: write-reg s_14_183 <= s_14_182
        let s_14_184: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_14_183 as isize, s_14_182);
            tracer.write_register(s_14_183 as isize, s_14_182);
        };
        // C s_14_185: const #20672u : u32
        let s_14_185: u32 = 20672;
        // D s_14_186: read-reg s_14_185:struct
        let s_14_186: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_14_185 as isize);
            tracer.read_register(s_14_185 as isize, value);
            value
        };
        // C s_14_187: const #20672u : u32
        let s_14_187: u32 = 20672;
        // N s_14_188: write-reg s_14_187 <= s_14_186
        let s_14_188: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_14_187 as isize, s_14_186);
            tracer.write_register(s_14_187 as isize, s_14_186);
        };
        // C s_14_189: const #20672u : u32
        let s_14_189: u32 = 20672;
        // D s_14_190: read-reg s_14_189:struct
        let s_14_190: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_14_189 as isize);
            tracer.read_register(s_14_189 as isize, value);
            value
        };
        // C s_14_191: const #20672u : u32
        let s_14_191: u32 = 20672;
        // N s_14_192: write-reg s_14_191 <= s_14_190
        let s_14_192: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_14_191 as isize, s_14_190);
            tracer.write_register(s_14_191 as isize, s_14_190);
        };
        // C s_14_193: const #20672u : u32
        let s_14_193: u32 = 20672;
        // D s_14_194: read-reg s_14_193:struct
        let s_14_194: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_14_193 as isize);
            tracer.read_register(s_14_193 as isize, value);
            value
        };
        // C s_14_195: const #20672u : u32
        let s_14_195: u32 = 20672;
        // N s_14_196: write-reg s_14_195 <= s_14_194
        let s_14_196: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_14_195 as isize, s_14_194);
            tracer.write_register(s_14_195 as isize, s_14_194);
        };
        // C s_14_197: const #() : ()
        let s_14_197: () = ();
        // S s_14_198: call VMPIDR_read(s_14_197)
        let s_14_198: ProductType700c18a878c5601b = VMPIDR_read(state, tracer, s_14_197);
        // C s_14_199: const #1u : u8
        let s_14_199: bool = true;
        // S s_14_200: call _update_VMPIDR_Type_M(s_14_198, s_14_199)
        let s_14_200: ProductType700c18a878c5601b = u_update_VMPIDR_Type_M(
            state,
            tracer,
            s_14_198,
            s_14_199,
        );
        // S s_14_201: call VMPIDR_write(s_14_200)
        let s_14_201: () = VMPIDR_write(state, tracer, s_14_200);
        // C s_14_202: const #1u : u32
        let s_14_202: u32 = 1;
        // S s_14_203: call HasArchVersion(s_14_202)
        let s_14_203: bool = HasArchVersion(state, tracer, s_14_202);
        // N s_14_204: branch s_14_203 b349 b15
        if s_14_203 {
            return block_349(state, tracer, fn_state);
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
        // C s_16_0: const #45u : u32
        let s_16_0: u32 = 45;
        // S s_16_1: call IsFeatureImplemented(s_16_0)
        let s_16_1: bool = IsFeatureImplemented(state, tracer, s_16_0);
        // N s_16_2: branch s_16_1 b348 b17
        if s_16_1 {
            return block_348(state, tracer, fn_state);
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
        // C s_18_0: const #55u : u32
        let s_18_0: u32 = 55;
        // S s_18_1: call IsFeatureImplemented(s_18_0)
        let s_18_1: bool = IsFeatureImplemented(state, tracer, s_18_0);
        // N s_18_2: branch s_18_1 b347 b19
        if s_18_1 {
            return block_347(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_19_0: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #51u : u32
        let s_20_0: u32 = 51;
        // S s_20_1: call IsFeatureImplemented(s_20_0)
        let s_20_1: bool = IsFeatureImplemented(state, tracer, s_20_0);
        // N s_20_2: branch s_20_1 b346 b21
        if s_20_1 {
            return block_346(state, tracer, fn_state);
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
        // C s_22_0: const #12u : u32
        let s_22_0: u32 = 12;
        // S s_22_1: call IsFeatureImplemented(s_22_0)
        let s_22_1: bool = IsFeatureImplemented(state, tracer, s_22_0);
        // S s_22_2: not s_22_1
        let s_22_2: bool = !s_22_1;
        // N s_22_3: branch s_22_2 b345 b23
        if s_22_2 {
            return block_345(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #12u : u32
        let s_24_0: u32 = 12;
        // S s_24_1: call IsFeatureImplemented(s_24_0)
        let s_24_1: bool = IsFeatureImplemented(state, tracer, s_24_0);
        // N s_24_2: branch s_24_1 b344 b25
        if s_24_1 {
            return block_344(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_25_0: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #12u : u32
        let s_26_0: u32 = 12;
        // S s_26_1: call IsFeatureImplemented(s_26_0)
        let s_26_1: bool = IsFeatureImplemented(state, tracer, s_26_0);
        // N s_26_2: branch s_26_1 b343 b27
        if s_26_1 {
            return block_343(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #0u : u8
        let s_27_0: bool = false;
        // D s_27_1: write-var gs#328666 <= s_27_0
        fn_state.gs_328666 = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var gs#328666:u8
        let s_28_0: bool = fn_state.gs_328666;
        // N s_28_1: branch s_28_0 b342 b29
        if s_28_0 {
            return block_342(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_29_0: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #40u : u32
        let s_30_0: u32 = 40;
        // S s_30_1: call IsFeatureImplemented(s_30_0)
        let s_30_1: bool = IsFeatureImplemented(state, tracer, s_30_0);
        // S s_30_2: not s_30_1
        let s_30_2: bool = !s_30_1;
        // N s_30_3: branch s_30_2 b341 b31
        if s_30_2 {
            return block_341(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_31_0: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #41u : u32
        let s_32_0: u32 = 41;
        // S s_32_1: call IsFeatureImplemented(s_32_0)
        let s_32_1: bool = IsFeatureImplemented(state, tracer, s_32_0);
        // S s_32_2: not s_32_1
        let s_32_2: bool = !s_32_1;
        // N s_32_3: branch s_32_2 b340 b33
        if s_32_2 {
            return block_340(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_33_0: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #2u : u32
        let s_34_0: u32 = 2;
        // S s_34_1: call HasArchVersion(s_34_0)
        let s_34_1: bool = HasArchVersion(state, tracer, s_34_0);
        // N s_34_2: branch s_34_1 b333 b35
        if s_34_1 {
            return block_333(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_35_0: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #6u : u32
        let s_36_0: u32 = 6;
        // S s_36_1: call HasArchVersion(s_36_0)
        let s_36_1: bool = HasArchVersion(state, tracer, s_36_0);
        // N s_36_2: branch s_36_1 b332 b37
        if s_36_1 {
            return block_332(state, tracer, fn_state);
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
        // C s_38_0: const #105u : u32
        let s_38_0: u32 = 105;
        // S s_38_1: call IsFeatureImplemented(s_38_0)
        let s_38_1: bool = IsFeatureImplemented(state, tracer, s_38_0);
        // N s_38_2: branch s_38_1 b331 b39
        if s_38_1 {
            return block_331(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_39_0: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #65u : u32
        let s_40_0: u32 = 65;
        // S s_40_1: call IsFeatureImplemented(s_40_0)
        let s_40_1: bool = IsFeatureImplemented(state, tracer, s_40_0);
        // N s_40_2: branch s_40_1 b330 b41
        if s_40_1 {
            return block_330(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_41_0: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #57u : u32
        let s_42_0: u32 = 57;
        // S s_42_1: call IsFeatureImplemented(s_42_0)
        let s_42_1: bool = IsFeatureImplemented(state, tracer, s_42_0);
        // N s_42_2: branch s_42_1 b329 b43
        if s_42_1 {
            return block_329(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_43_0: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #() : ()
        let s_44_0: () = ();
        // S s_44_1: call HaveMPAMExt(s_44_0)
        let s_44_1: bool = HaveMPAMExt(state, tracer, s_44_0);
        // N s_44_2: branch s_44_1 b307 b45
        if s_44_1 {
            return block_307(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_45_0: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #12u : u32
        let s_46_0: u32 = 12;
        // S s_46_1: call HasArchVersion(s_46_0)
        let s_46_1: bool = HasArchVersion(state, tracer, s_46_0);
        // N s_46_2: branch s_46_1 b306 b47
        if s_46_1 {
            return block_306(state, tracer, fn_state);
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
        // D s_47_1: write-var gs#328667 <= s_47_0
        fn_state.gs_328667 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#328667:u8
        let s_48_0: bool = fn_state.gs_328667;
        // N s_48_1: branch s_48_0 b302 b49
        if s_48_0 {
            return block_302(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_49_0: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #3u : u32
        let s_50_0: u32 = 3;
        // S s_50_1: call HasArchVersion(s_50_0)
        let s_50_1: bool = HasArchVersion(state, tracer, s_50_0);
        // N s_50_2: branch s_50_1 b292 b51
        if s_50_1 {
            return block_292(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_51_0: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #4u : u32
        let s_52_0: u32 = 4;
        // S s_52_1: call HasArchVersion(s_52_0)
        let s_52_1: bool = HasArchVersion(state, tracer, s_52_0);
        // N s_52_2: branch s_52_1 b267 b53
        if s_52_1 {
            return block_267(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_53_0: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #22552u : u32
        let s_54_0: u32 = 22552;
        // D s_54_1: read-reg s_54_0:struct
        let s_54_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_54_0 as isize);
            tracer.read_register(s_54_0 as isize, value);
            value
        };
        // C s_54_2: const #22552u : u32
        let s_54_2: u32 = 22552;
        // N s_54_3: write-reg s_54_2 <= s_54_1
        let s_54_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_54_2 as isize, s_54_1);
            tracer.write_register(s_54_2 as isize, s_54_1);
        };
        // C s_54_4: const #5u : u32
        let s_54_4: u32 = 5;
        // S s_54_5: call HasArchVersion(s_54_4)
        let s_54_5: bool = HasArchVersion(state, tracer, s_54_4);
        // N s_54_6: branch s_54_5 b263 b55
        if s_54_5 {
            return block_263(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_55_0: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #20416u : u32
        let s_56_0: u32 = 20416;
        // D s_56_1: read-reg s_56_0:struct
        let s_56_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_56_0 as isize);
            tracer.read_register(s_56_0 as isize, value);
            value
        };
        // C s_56_2: const #20416u : u32
        let s_56_2: u32 = 20416;
        // N s_56_3: write-reg s_56_2 <= s_56_1
        let s_56_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_56_2 as isize, s_56_1);
            tracer.write_register(s_56_2 as isize, s_56_1);
        };
        // C s_56_4: const #20416u : u32
        let s_56_4: u32 = 20416;
        // D s_56_5: read-reg s_56_4:struct
        let s_56_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_56_4 as isize);
            tracer.read_register(s_56_4 as isize, value);
            value
        };
        // C s_56_6: const #20416u : u32
        let s_56_6: u32 = 20416;
        // N s_56_7: write-reg s_56_6 <= s_56_5
        let s_56_7: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_56_6 as isize, s_56_5);
            tracer.write_register(s_56_6 as isize, s_56_5);
        };
        // C s_56_8: const #6u : u32
        let s_56_8: u32 = 6;
        // S s_56_9: call HasArchVersion(s_56_8)
        let s_56_9: bool = HasArchVersion(state, tracer, s_56_8);
        // N s_56_10: branch s_56_9 b262 b57
        if s_56_9 {
            return block_262(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_57_0: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #90944u : u32
        let s_58_0: u32 = 90944;
        // D s_58_1: read-reg s_58_0:struct
        let s_58_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_58_0 as isize);
            tracer.read_register(s_58_0 as isize, value);
            value
        };
        // C s_58_2: const #90944u : u32
        let s_58_2: u32 = 90944;
        // N s_58_3: write-reg s_58_2 <= s_58_1
        let s_58_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_58_2 as isize, s_58_1);
            tracer.write_register(s_58_2 as isize, s_58_1);
        };
        // C s_58_4: const #20112u : u32
        let s_58_4: u32 = 20112;
        // D s_58_5: read-reg s_58_4:struct
        let s_58_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_58_4 as isize);
            tracer.read_register(s_58_4 as isize, value);
            value
        };
        // C s_58_6: const #20112u : u32
        let s_58_6: u32 = 20112;
        // N s_58_7: write-reg s_58_6 <= s_58_5
        let s_58_7: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_58_6 as isize, s_58_5);
            tracer.write_register(s_58_6 as isize, s_58_5);
        };
        // C s_58_8: const #104720u : u32
        let s_58_8: u32 = 104720;
        // D s_58_9: read-reg s_58_8:struct
        let s_58_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_58_8 as isize);
            tracer.read_register(s_58_8 as isize, value);
            value
        };
        // C s_58_10: const #104720u : u32
        let s_58_10: u32 = 104720;
        // N s_58_11: write-reg s_58_10 <= s_58_9
        let s_58_11: () = {
            state
                .write_register::<ProductType5c790c8ef59cc8b2>(s_58_10 as isize, s_58_9);
            tracer.write_register(s_58_10 as isize, s_58_9);
        };
        // C s_58_12: const #7u : u32
        let s_58_12: u32 = 7;
        // S s_58_13: call HasArchVersion(s_58_12)
        let s_58_13: bool = HasArchVersion(state, tracer, s_58_12);
        // N s_58_14: branch s_58_13 b243 b59
        if s_58_13 {
            return block_243(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_59_0: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #1u : u8
        let s_60_0: bool = true;
        // S s_60_1: call Bit(s_60_0)
        let s_60_1: bool = Bit(state, tracer, s_60_0);
        // C s_60_2: const #89488u : u32
        let s_60_2: u32 = 89488;
        // D s_60_3: read-reg s_60_2:struct
        let s_60_3: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_60_2 as isize);
            tracer.read_register(s_60_2 as isize, value);
            value
        };
        // C s_60_4: const #89488u : u32
        let s_60_4: u32 = 89488;
        // N s_60_5: write-reg s_60_4 <= s_60_3
        let s_60_5: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_60_4 as isize, s_60_3);
            tracer.write_register(s_60_4 as isize, s_60_3);
        };
        // C s_60_6: const #89488u : u32
        let s_60_6: u32 = 89488;
        // D s_60_7: read-reg s_60_6:struct
        let s_60_7: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_60_6 as isize);
            tracer.read_register(s_60_6 as isize, value);
            value
        };
        // C s_60_8: const #89488u : u32
        let s_60_8: u32 = 89488;
        // N s_60_9: write-reg s_60_8 <= s_60_7
        let s_60_9: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_60_8 as isize, s_60_7);
            tracer.write_register(s_60_8 as isize, s_60_7);
        };
        // C s_60_10: const #89488u : u32
        let s_60_10: u32 = 89488;
        // D s_60_11: read-reg s_60_10:struct
        let s_60_11: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_60_10 as isize);
            tracer.read_register(s_60_10 as isize, value);
            value
        };
        // C s_60_12: const #89488u : u32
        let s_60_12: u32 = 89488;
        // N s_60_13: write-reg s_60_12 <= s_60_11
        let s_60_13: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_60_12 as isize, s_60_11);
            tracer.write_register(s_60_12 as isize, s_60_11);
        };
        // C s_60_14: const #89488u : u32
        let s_60_14: u32 = 89488;
        // D s_60_15: read-reg s_60_14:struct
        let s_60_15: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_60_14 as isize);
            tracer.read_register(s_60_14 as isize, value);
            value
        };
        // C s_60_16: const #89488u : u32
        let s_60_16: u32 = 89488;
        // N s_60_17: write-reg s_60_16 <= s_60_15
        let s_60_17: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_60_16 as isize, s_60_15);
            tracer.write_register(s_60_16 as isize, s_60_15);
        };
        // C s_60_18: const #89488u : u32
        let s_60_18: u32 = 89488;
        // D s_60_19: read-reg s_60_18:struct
        let s_60_19: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_60_18 as isize);
            tracer.read_register(s_60_18 as isize, value);
            value
        };
        // C s_60_20: const #89488u : u32
        let s_60_20: u32 = 89488;
        // N s_60_21: write-reg s_60_20 <= s_60_19
        let s_60_21: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_60_20 as isize, s_60_19);
            tracer.write_register(s_60_20 as isize, s_60_19);
        };
        // C s_60_22: const #89488u : u32
        let s_60_22: u32 = 89488;
        // D s_60_23: read-reg s_60_22:struct
        let s_60_23: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_60_22 as isize);
            tracer.read_register(s_60_22 as isize, value);
            value
        };
        // C s_60_24: const #89488u : u32
        let s_60_24: u32 = 89488;
        // N s_60_25: write-reg s_60_24 <= s_60_23
        let s_60_25: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_60_24 as isize, s_60_23);
            tracer.write_register(s_60_24 as isize, s_60_23);
        };
        // C s_60_26: const #89488u : u32
        let s_60_26: u32 = 89488;
        // D s_60_27: read-reg s_60_26:struct
        let s_60_27: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_60_26 as isize);
            tracer.read_register(s_60_26 as isize, value);
            value
        };
        // C s_60_28: const #89488u : u32
        let s_60_28: u32 = 89488;
        // N s_60_29: write-reg s_60_28 <= s_60_27
        let s_60_29: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_60_28 as isize, s_60_27);
            tracer.write_register(s_60_28 as isize, s_60_27);
        };
        // C s_60_30: const #89488u : u32
        let s_60_30: u32 = 89488;
        // D s_60_31: read-reg s_60_30:struct
        let s_60_31: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_60_30 as isize);
            tracer.read_register(s_60_30 as isize, value);
            value
        };
        // C s_60_32: const #89488u : u32
        let s_60_32: u32 = 89488;
        // N s_60_33: write-reg s_60_32 <= s_60_31
        let s_60_33: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_60_32 as isize, s_60_31);
            tracer.write_register(s_60_32 as isize, s_60_31);
        };
        // C s_60_34: const #89488u : u32
        let s_60_34: u32 = 89488;
        // D s_60_35: read-reg s_60_34:struct
        let s_60_35: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_60_34 as isize);
            tracer.read_register(s_60_34 as isize, value);
            value
        };
        // C s_60_36: const #89488u : u32
        let s_60_36: u32 = 89488;
        // N s_60_37: write-reg s_60_36 <= s_60_35
        let s_60_37: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_60_36 as isize, s_60_35);
            tracer.write_register(s_60_36 as isize, s_60_35);
        };
        // C s_60_38: const #15704u : u32
        let s_60_38: u32 = 15704;
        // D s_60_39: read-reg s_60_38:struct
        let s_60_39: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_60_38 as isize);
            tracer.read_register(s_60_38 as isize, value);
            value
        };
        // C s_60_40: const #15704u : u32
        let s_60_40: u32 = 15704;
        // N s_60_41: write-reg s_60_40 <= s_60_39
        let s_60_41: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_60_40 as isize, s_60_39);
            tracer.write_register(s_60_40 as isize, s_60_39);
        };
        // C s_60_42: const #15704u : u32
        let s_60_42: u32 = 15704;
        // D s_60_43: read-reg s_60_42:struct
        let s_60_43: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_60_42 as isize);
            tracer.read_register(s_60_42 as isize, value);
            value
        };
        // C s_60_44: const #15704u : u32
        let s_60_44: u32 = 15704;
        // N s_60_45: write-reg s_60_44 <= s_60_43
        let s_60_45: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_60_44 as isize, s_60_43);
            tracer.write_register(s_60_44 as isize, s_60_43);
        };
        // C s_60_46: const #15704u : u32
        let s_60_46: u32 = 15704;
        // D s_60_47: read-reg s_60_46:struct
        let s_60_47: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_60_46 as isize);
            tracer.read_register(s_60_46 as isize, value);
            value
        };
        // C s_60_48: const #15704u : u32
        let s_60_48: u32 = 15704;
        // N s_60_49: write-reg s_60_48 <= s_60_47
        let s_60_49: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_60_48 as isize, s_60_47);
            tracer.write_register(s_60_48 as isize, s_60_47);
        };
        // C s_60_50: const #21016u : u32
        let s_60_50: u32 = 21016;
        // D s_60_51: read-reg s_60_50:struct
        let s_60_51: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_60_50 as isize);
            tracer.read_register(s_60_50 as isize, value);
            value
        };
        // C s_60_52: const #21016u : u32
        let s_60_52: u32 = 21016;
        // N s_60_53: write-reg s_60_52 <= s_60_51
        let s_60_53: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_60_52 as isize, s_60_51);
            tracer.write_register(s_60_52 as isize, s_60_51);
        };
        // C s_60_54: const #233u : u32
        let s_60_54: u32 = 233;
        // S s_60_55: call IsFeatureImplemented(s_60_54)
        let s_60_55: bool = IsFeatureImplemented(state, tracer, s_60_54);
        // N s_60_56: branch s_60_55 b242 b61
        if s_60_55 {
            return block_242(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_61_0: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #13u : u32
        let s_62_0: u32 = 13;
        // S s_62_1: call HasArchVersion(s_62_0)
        let s_62_1: bool = HasArchVersion(state, tracer, s_62_0);
        // N s_62_2: branch s_62_1 b238 b63
        if s_62_1 {
            return block_238(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_63_0: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #239u : u32
        let s_64_0: u32 = 239;
        // S s_64_1: call IsFeatureImplemented(s_64_0)
        let s_64_1: bool = IsFeatureImplemented(state, tracer, s_64_0);
        // N s_64_2: branch s_64_1 b237 b65
        if s_64_1 {
            return block_237(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_65_0: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #8u : u32
        let s_66_0: u32 = 8;
        // S s_66_1: call HasArchVersion(s_66_0)
        let s_66_1: bool = HasArchVersion(state, tracer, s_66_0);
        // N s_66_2: branch s_66_1 b227 b67
        if s_66_1 {
            return block_227(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_67_0: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #90608u : u32
        let s_68_0: u32 = 90608;
        // D s_68_1: read-reg s_68_0:struct
        let s_68_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_68_0 as isize);
            tracer.read_register(s_68_0 as isize, value);
            value
        };
        // C s_68_2: const #90608u : u32
        let s_68_2: u32 = 90608;
        // N s_68_3: write-reg s_68_2 <= s_68_1
        let s_68_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_68_2 as isize, s_68_1);
            tracer.write_register(s_68_2 as isize, s_68_1);
        };
        // C s_68_4: const #20416u : u32
        let s_68_4: u32 = 20416;
        // D s_68_5: read-reg s_68_4:struct
        let s_68_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_68_4 as isize);
            tracer.read_register(s_68_4 as isize, value);
            value
        };
        // C s_68_6: const #20416u : u32
        let s_68_6: u32 = 20416;
        // N s_68_7: write-reg s_68_6 <= s_68_5
        let s_68_7: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_68_6 as isize, s_68_5);
            tracer.write_register(s_68_6 as isize, s_68_5);
        };
        // C s_68_8: const #20416u : u32
        let s_68_8: u32 = 20416;
        // D s_68_9: read-reg s_68_8:struct
        let s_68_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_68_8 as isize);
            tracer.read_register(s_68_8 as isize, value);
            value
        };
        // C s_68_10: const #20416u : u32
        let s_68_10: u32 = 20416;
        // N s_68_11: write-reg s_68_10 <= s_68_9
        let s_68_11: () = {
            state
                .write_register::<ProductType5c790c8ef59cc8b2>(s_68_10 as isize, s_68_9);
            tracer.write_register(s_68_10 as isize, s_68_9);
        };
        // C s_68_12: const #90704u : u32
        let s_68_12: u32 = 90704;
        // D s_68_13: read-reg s_68_12:struct
        let s_68_13: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_68_12 as isize);
            tracer.read_register(s_68_12 as isize, value);
            value
        };
        // C s_68_14: const #90704u : u32
        let s_68_14: u32 = 90704;
        // N s_68_15: write-reg s_68_14 <= s_68_13
        let s_68_15: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_68_14 as isize, s_68_13);
            tracer.write_register(s_68_14 as isize, s_68_13);
        };
        // C s_68_16: const #90704u : u32
        let s_68_16: u32 = 90704;
        // D s_68_17: read-reg s_68_16:struct
        let s_68_17: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_68_16 as isize);
            tracer.read_register(s_68_16 as isize, value);
            value
        };
        // C s_68_18: const #90704u : u32
        let s_68_18: u32 = 90704;
        // N s_68_19: write-reg s_68_18 <= s_68_17
        let s_68_19: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_68_18 as isize, s_68_17);
            tracer.write_register(s_68_18 as isize, s_68_17);
        };
        // C s_68_20: const #90704u : u32
        let s_68_20: u32 = 90704;
        // D s_68_21: read-reg s_68_20:struct
        let s_68_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_68_20 as isize);
            tracer.read_register(s_68_20 as isize, value);
            value
        };
        // C s_68_22: const #90704u : u32
        let s_68_22: u32 = 90704;
        // N s_68_23: write-reg s_68_22 <= s_68_21
        let s_68_23: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_68_22 as isize, s_68_21);
            tracer.write_register(s_68_22 as isize, s_68_21);
        };
        // C s_68_24: const #90704u : u32
        let s_68_24: u32 = 90704;
        // D s_68_25: read-reg s_68_24:struct
        let s_68_25: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_68_24 as isize);
            tracer.read_register(s_68_24 as isize, value);
            value
        };
        // C s_68_26: const #90704u : u32
        let s_68_26: u32 = 90704;
        // N s_68_27: write-reg s_68_26 <= s_68_25
        let s_68_27: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_68_26 as isize, s_68_25);
            tracer.write_register(s_68_26 as isize, s_68_25);
        };
        // C s_68_28: const #90704u : u32
        let s_68_28: u32 = 90704;
        // D s_68_29: read-reg s_68_28:struct
        let s_68_29: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_68_28 as isize);
            tracer.read_register(s_68_28 as isize, value);
            value
        };
        // C s_68_30: const #90704u : u32
        let s_68_30: u32 = 90704;
        // N s_68_31: write-reg s_68_30 <= s_68_29
        let s_68_31: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_68_30 as isize, s_68_29);
            tracer.write_register(s_68_30 as isize, s_68_29);
        };
        // C s_68_32: const #90704u : u32
        let s_68_32: u32 = 90704;
        // D s_68_33: read-reg s_68_32:struct
        let s_68_33: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_68_32 as isize);
            tracer.read_register(s_68_32 as isize, value);
            value
        };
        // C s_68_34: const #90704u : u32
        let s_68_34: u32 = 90704;
        // N s_68_35: write-reg s_68_34 <= s_68_33
        let s_68_35: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_68_34 as isize, s_68_33);
            tracer.write_register(s_68_34 as isize, s_68_33);
        };
        // C s_68_36: const #90704u : u32
        let s_68_36: u32 = 90704;
        // D s_68_37: read-reg s_68_36:struct
        let s_68_37: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_68_36 as isize);
            tracer.read_register(s_68_36 as isize, value);
            value
        };
        // C s_68_38: const #90704u : u32
        let s_68_38: u32 = 90704;
        // N s_68_39: write-reg s_68_38 <= s_68_37
        let s_68_39: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_68_38 as isize, s_68_37);
            tracer.write_register(s_68_38 as isize, s_68_37);
        };
        // C s_68_40: const #90704u : u32
        let s_68_40: u32 = 90704;
        // D s_68_41: read-reg s_68_40:struct
        let s_68_41: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_68_40 as isize);
            tracer.read_register(s_68_40 as isize, value);
            value
        };
        // C s_68_42: const #90704u : u32
        let s_68_42: u32 = 90704;
        // N s_68_43: write-reg s_68_42 <= s_68_41
        let s_68_43: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_68_42 as isize, s_68_41);
            tracer.write_register(s_68_42 as isize, s_68_41);
        };
        // C s_68_44: const #90704u : u32
        let s_68_44: u32 = 90704;
        // D s_68_45: read-reg s_68_44:struct
        let s_68_45: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_68_44 as isize);
            tracer.read_register(s_68_44 as isize, value);
            value
        };
        // C s_68_46: const #90704u : u32
        let s_68_46: u32 = 90704;
        // N s_68_47: write-reg s_68_46 <= s_68_45
        let s_68_47: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_68_46 as isize, s_68_45);
            tracer.write_register(s_68_46 as isize, s_68_45);
        };
        // C s_68_48: const #90704u : u32
        let s_68_48: u32 = 90704;
        // D s_68_49: read-reg s_68_48:struct
        let s_68_49: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_68_48 as isize);
            tracer.read_register(s_68_48 as isize, value);
            value
        };
        // C s_68_50: const #90704u : u32
        let s_68_50: u32 = 90704;
        // N s_68_51: write-reg s_68_50 <= s_68_49
        let s_68_51: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_68_50 as isize, s_68_49);
            tracer.write_register(s_68_50 as isize, s_68_49);
        };
        // C s_68_52: const #90704u : u32
        let s_68_52: u32 = 90704;
        // D s_68_53: read-reg s_68_52:struct
        let s_68_53: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_68_52 as isize);
            tracer.read_register(s_68_52 as isize, value);
            value
        };
        // C s_68_54: const #90704u : u32
        let s_68_54: u32 = 90704;
        // N s_68_55: write-reg s_68_54 <= s_68_53
        let s_68_55: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_68_54 as isize, s_68_53);
            tracer.write_register(s_68_54 as isize, s_68_53);
        };
        // C s_68_56: const #90704u : u32
        let s_68_56: u32 = 90704;
        // D s_68_57: read-reg s_68_56:struct
        let s_68_57: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_68_56 as isize);
            tracer.read_register(s_68_56 as isize, value);
            value
        };
        // C s_68_58: const #90704u : u32
        let s_68_58: u32 = 90704;
        // N s_68_59: write-reg s_68_58 <= s_68_57
        let s_68_59: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_68_58 as isize, s_68_57);
            tracer.write_register(s_68_58 as isize, s_68_57);
        };
        // C s_68_60: const #90704u : u32
        let s_68_60: u32 = 90704;
        // D s_68_61: read-reg s_68_60:struct
        let s_68_61: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_68_60 as isize);
            tracer.read_register(s_68_60 as isize, value);
            value
        };
        // C s_68_62: const #90704u : u32
        let s_68_62: u32 = 90704;
        // N s_68_63: write-reg s_68_62 <= s_68_61
        let s_68_63: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_68_62 as isize, s_68_61);
            tracer.write_register(s_68_62 as isize, s_68_61);
        };
        // C s_68_64: const #90704u : u32
        let s_68_64: u32 = 90704;
        // D s_68_65: read-reg s_68_64:struct
        let s_68_65: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_68_64 as isize);
            tracer.read_register(s_68_64 as isize, value);
            value
        };
        // C s_68_66: const #90704u : u32
        let s_68_66: u32 = 90704;
        // N s_68_67: write-reg s_68_66 <= s_68_65
        let s_68_67: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_68_66 as isize, s_68_65);
            tracer.write_register(s_68_66 as isize, s_68_65);
        };
        // C s_68_68: const #90704u : u32
        let s_68_68: u32 = 90704;
        // D s_68_69: read-reg s_68_68:struct
        let s_68_69: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_68_68 as isize);
            tracer.read_register(s_68_68 as isize, value);
            value
        };
        // C s_68_70: const #90704u : u32
        let s_68_70: u32 = 90704;
        // N s_68_71: write-reg s_68_70 <= s_68_69
        let s_68_71: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_68_70 as isize, s_68_69);
            tracer.write_register(s_68_70 as isize, s_68_69);
        };
        // C s_68_72: const #90704u : u32
        let s_68_72: u32 = 90704;
        // D s_68_73: read-reg s_68_72:struct
        let s_68_73: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_68_72 as isize);
            tracer.read_register(s_68_72 as isize, value);
            value
        };
        // C s_68_74: const #90704u : u32
        let s_68_74: u32 = 90704;
        // N s_68_75: write-reg s_68_74 <= s_68_73
        let s_68_75: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_68_74 as isize, s_68_73);
            tracer.write_register(s_68_74 as isize, s_68_73);
        };
        // C s_68_76: const #11088u : u32
        let s_68_76: u32 = 11088;
        // D s_68_77: read-reg s_68_76:struct
        let s_68_77: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_68_76 as isize);
            tracer.read_register(s_68_76 as isize, value);
            value
        };
        // C s_68_78: const #11088u : u32
        let s_68_78: u32 = 11088;
        // N s_68_79: write-reg s_68_78 <= s_68_77
        let s_68_79: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_68_78 as isize, s_68_77);
            tracer.write_register(s_68_78 as isize, s_68_77);
        };
        // C s_68_80: const #54u : u32
        let s_68_80: u32 = 54;
        // S s_68_81: call IsFeatureImplemented(s_68_80)
        let s_68_81: bool = IsFeatureImplemented(state, tracer, s_68_80);
        // N s_68_82: branch s_68_81 b226 b69
        if s_68_81 {
            return block_226(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #0u : u8
        let s_69_0: bool = false;
        // D s_69_1: write-var gs#328784 <= s_69_0
        fn_state.gs_328784 = s_69_0;
        // N s_69_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var gs#328784:u8
        let s_70_0: bool = fn_state.gs_328784;
        // N s_70_1: branch s_70_0 b225 b71
        if s_70_0 {
            return block_225(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_71_0: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #0u : u8
        let s_72_0: bool = false;
        // S s_72_1: call Bit(s_72_0)
        let s_72_1: bool = Bit(state, tracer, s_72_0);
        // C s_72_2: const #11088u : u32
        let s_72_2: u32 = 11088;
        // D s_72_3: read-reg s_72_2:struct
        let s_72_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_2 as isize);
            tracer.read_register(s_72_2 as isize, value);
            value
        };
        // C s_72_4: const #11088u : u32
        let s_72_4: u32 = 11088;
        // N s_72_5: write-reg s_72_4 <= s_72_3
        let s_72_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_72_4 as isize, s_72_3);
            tracer.write_register(s_72_4 as isize, s_72_3);
        };
        // C s_72_6: const #102552u : u32
        let s_72_6: u32 = 102552;
        // D s_72_7: read-reg s_72_6:struct
        let s_72_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_6 as isize);
            tracer.read_register(s_72_6 as isize, value);
            value
        };
        // C s_72_8: const #102552u : u32
        let s_72_8: u32 = 102552;
        // N s_72_9: write-reg s_72_8 <= s_72_7
        let s_72_9: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_72_8 as isize, s_72_7);
            tracer.write_register(s_72_8 as isize, s_72_7);
        };
        // C s_72_10: const #102552u : u32
        let s_72_10: u32 = 102552;
        // D s_72_11: read-reg s_72_10:struct
        let s_72_11: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_10 as isize);
            tracer.read_register(s_72_10 as isize, value);
            value
        };
        // C s_72_12: const #102552u : u32
        let s_72_12: u32 = 102552;
        // N s_72_13: write-reg s_72_12 <= s_72_11
        let s_72_13: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_12 as isize, s_72_11);
            tracer.write_register(s_72_12 as isize, s_72_11);
        };
        // C s_72_14: const #102552u : u32
        let s_72_14: u32 = 102552;
        // D s_72_15: read-reg s_72_14:struct
        let s_72_15: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_14 as isize);
            tracer.read_register(s_72_14 as isize, value);
            value
        };
        // C s_72_16: const #102552u : u32
        let s_72_16: u32 = 102552;
        // N s_72_17: write-reg s_72_16 <= s_72_15
        let s_72_17: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_16 as isize, s_72_15);
            tracer.write_register(s_72_16 as isize, s_72_15);
        };
        // C s_72_18: const #102552u : u32
        let s_72_18: u32 = 102552;
        // D s_72_19: read-reg s_72_18:struct
        let s_72_19: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_18 as isize);
            tracer.read_register(s_72_18 as isize, value);
            value
        };
        // C s_72_20: const #102552u : u32
        let s_72_20: u32 = 102552;
        // N s_72_21: write-reg s_72_20 <= s_72_19
        let s_72_21: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_20 as isize, s_72_19);
            tracer.write_register(s_72_20 as isize, s_72_19);
        };
        // C s_72_22: const #102552u : u32
        let s_72_22: u32 = 102552;
        // D s_72_23: read-reg s_72_22:struct
        let s_72_23: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_22 as isize);
            tracer.read_register(s_72_22 as isize, value);
            value
        };
        // C s_72_24: const #102552u : u32
        let s_72_24: u32 = 102552;
        // N s_72_25: write-reg s_72_24 <= s_72_23
        let s_72_25: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_24 as isize, s_72_23);
            tracer.write_register(s_72_24 as isize, s_72_23);
        };
        // C s_72_26: const #102552u : u32
        let s_72_26: u32 = 102552;
        // D s_72_27: read-reg s_72_26:struct
        let s_72_27: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_26 as isize);
            tracer.read_register(s_72_26 as isize, value);
            value
        };
        // C s_72_28: const #102552u : u32
        let s_72_28: u32 = 102552;
        // N s_72_29: write-reg s_72_28 <= s_72_27
        let s_72_29: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_28 as isize, s_72_27);
            tracer.write_register(s_72_28 as isize, s_72_27);
        };
        // C s_72_30: const #102552u : u32
        let s_72_30: u32 = 102552;
        // D s_72_31: read-reg s_72_30:struct
        let s_72_31: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_30 as isize);
            tracer.read_register(s_72_30 as isize, value);
            value
        };
        // C s_72_32: const #102552u : u32
        let s_72_32: u32 = 102552;
        // N s_72_33: write-reg s_72_32 <= s_72_31
        let s_72_33: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_32 as isize, s_72_31);
            tracer.write_register(s_72_32 as isize, s_72_31);
        };
        // C s_72_34: const #102552u : u32
        let s_72_34: u32 = 102552;
        // D s_72_35: read-reg s_72_34:struct
        let s_72_35: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_34 as isize);
            tracer.read_register(s_72_34 as isize, value);
            value
        };
        // C s_72_36: const #102552u : u32
        let s_72_36: u32 = 102552;
        // N s_72_37: write-reg s_72_36 <= s_72_35
        let s_72_37: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_36 as isize, s_72_35);
            tracer.write_register(s_72_36 as isize, s_72_35);
        };
        // C s_72_38: const #102552u : u32
        let s_72_38: u32 = 102552;
        // D s_72_39: read-reg s_72_38:struct
        let s_72_39: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_38 as isize);
            tracer.read_register(s_72_38 as isize, value);
            value
        };
        // C s_72_40: const #102552u : u32
        let s_72_40: u32 = 102552;
        // N s_72_41: write-reg s_72_40 <= s_72_39
        let s_72_41: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_40 as isize, s_72_39);
            tracer.write_register(s_72_40 as isize, s_72_39);
        };
        // C s_72_42: const #102552u : u32
        let s_72_42: u32 = 102552;
        // D s_72_43: read-reg s_72_42:struct
        let s_72_43: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_42 as isize);
            tracer.read_register(s_72_42 as isize, value);
            value
        };
        // C s_72_44: const #102552u : u32
        let s_72_44: u32 = 102552;
        // N s_72_45: write-reg s_72_44 <= s_72_43
        let s_72_45: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_44 as isize, s_72_43);
            tracer.write_register(s_72_44 as isize, s_72_43);
        };
        // C s_72_46: const #102552u : u32
        let s_72_46: u32 = 102552;
        // D s_72_47: read-reg s_72_46:struct
        let s_72_47: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_46 as isize);
            tracer.read_register(s_72_46 as isize, value);
            value
        };
        // C s_72_48: const #102552u : u32
        let s_72_48: u32 = 102552;
        // N s_72_49: write-reg s_72_48 <= s_72_47
        let s_72_49: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_48 as isize, s_72_47);
            tracer.write_register(s_72_48 as isize, s_72_47);
        };
        // C s_72_50: const #102552u : u32
        let s_72_50: u32 = 102552;
        // D s_72_51: read-reg s_72_50:struct
        let s_72_51: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_50 as isize);
            tracer.read_register(s_72_50 as isize, value);
            value
        };
        // C s_72_52: const #102552u : u32
        let s_72_52: u32 = 102552;
        // N s_72_53: write-reg s_72_52 <= s_72_51
        let s_72_53: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_52 as isize, s_72_51);
            tracer.write_register(s_72_52 as isize, s_72_51);
        };
        // C s_72_54: const #102552u : u32
        let s_72_54: u32 = 102552;
        // D s_72_55: read-reg s_72_54:struct
        let s_72_55: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_54 as isize);
            tracer.read_register(s_72_54 as isize, value);
            value
        };
        // C s_72_56: const #102552u : u32
        let s_72_56: u32 = 102552;
        // N s_72_57: write-reg s_72_56 <= s_72_55
        let s_72_57: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_56 as isize, s_72_55);
            tracer.write_register(s_72_56 as isize, s_72_55);
        };
        // C s_72_58: const #102552u : u32
        let s_72_58: u32 = 102552;
        // D s_72_59: read-reg s_72_58:struct
        let s_72_59: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_58 as isize);
            tracer.read_register(s_72_58 as isize, value);
            value
        };
        // C s_72_60: const #102552u : u32
        let s_72_60: u32 = 102552;
        // N s_72_61: write-reg s_72_60 <= s_72_59
        let s_72_61: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_60 as isize, s_72_59);
            tracer.write_register(s_72_60 as isize, s_72_59);
        };
        // C s_72_62: const #102552u : u32
        let s_72_62: u32 = 102552;
        // D s_72_63: read-reg s_72_62:struct
        let s_72_63: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_62 as isize);
            tracer.read_register(s_72_62 as isize, value);
            value
        };
        // C s_72_64: const #102552u : u32
        let s_72_64: u32 = 102552;
        // N s_72_65: write-reg s_72_64 <= s_72_63
        let s_72_65: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_64 as isize, s_72_63);
            tracer.write_register(s_72_64 as isize, s_72_63);
        };
        // C s_72_66: const #102552u : u32
        let s_72_66: u32 = 102552;
        // D s_72_67: read-reg s_72_66:struct
        let s_72_67: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_66 as isize);
            tracer.read_register(s_72_66 as isize, value);
            value
        };
        // C s_72_68: const #102552u : u32
        let s_72_68: u32 = 102552;
        // N s_72_69: write-reg s_72_68 <= s_72_67
        let s_72_69: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_68 as isize, s_72_67);
            tracer.write_register(s_72_68 as isize, s_72_67);
        };
        // C s_72_70: const #102552u : u32
        let s_72_70: u32 = 102552;
        // D s_72_71: read-reg s_72_70:struct
        let s_72_71: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_70 as isize);
            tracer.read_register(s_72_70 as isize, value);
            value
        };
        // C s_72_72: const #102552u : u32
        let s_72_72: u32 = 102552;
        // N s_72_73: write-reg s_72_72 <= s_72_71
        let s_72_73: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_72 as isize, s_72_71);
            tracer.write_register(s_72_72 as isize, s_72_71);
        };
        // C s_72_74: const #102552u : u32
        let s_72_74: u32 = 102552;
        // D s_72_75: read-reg s_72_74:struct
        let s_72_75: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_74 as isize);
            tracer.read_register(s_72_74 as isize, value);
            value
        };
        // C s_72_76: const #102552u : u32
        let s_72_76: u32 = 102552;
        // N s_72_77: write-reg s_72_76 <= s_72_75
        let s_72_77: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_76 as isize, s_72_75);
            tracer.write_register(s_72_76 as isize, s_72_75);
        };
        // C s_72_78: const #102552u : u32
        let s_72_78: u32 = 102552;
        // D s_72_79: read-reg s_72_78:struct
        let s_72_79: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_78 as isize);
            tracer.read_register(s_72_78 as isize, value);
            value
        };
        // C s_72_80: const #102552u : u32
        let s_72_80: u32 = 102552;
        // N s_72_81: write-reg s_72_80 <= s_72_79
        let s_72_81: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_80 as isize, s_72_79);
            tracer.write_register(s_72_80 as isize, s_72_79);
        };
        // C s_72_82: const #102552u : u32
        let s_72_82: u32 = 102552;
        // D s_72_83: read-reg s_72_82:struct
        let s_72_83: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_82 as isize);
            tracer.read_register(s_72_82 as isize, value);
            value
        };
        // C s_72_84: const #102552u : u32
        let s_72_84: u32 = 102552;
        // N s_72_85: write-reg s_72_84 <= s_72_83
        let s_72_85: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_84 as isize, s_72_83);
            tracer.write_register(s_72_84 as isize, s_72_83);
        };
        // C s_72_86: const #102552u : u32
        let s_72_86: u32 = 102552;
        // D s_72_87: read-reg s_72_86:struct
        let s_72_87: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_86 as isize);
            tracer.read_register(s_72_86 as isize, value);
            value
        };
        // C s_72_88: const #102552u : u32
        let s_72_88: u32 = 102552;
        // N s_72_89: write-reg s_72_88 <= s_72_87
        let s_72_89: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_88 as isize, s_72_87);
            tracer.write_register(s_72_88 as isize, s_72_87);
        };
        // C s_72_90: const #102552u : u32
        let s_72_90: u32 = 102552;
        // D s_72_91: read-reg s_72_90:struct
        let s_72_91: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_90 as isize);
            tracer.read_register(s_72_90 as isize, value);
            value
        };
        // C s_72_92: const #102552u : u32
        let s_72_92: u32 = 102552;
        // N s_72_93: write-reg s_72_92 <= s_72_91
        let s_72_93: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_92 as isize, s_72_91);
            tracer.write_register(s_72_92 as isize, s_72_91);
        };
        // C s_72_94: const #102552u : u32
        let s_72_94: u32 = 102552;
        // D s_72_95: read-reg s_72_94:struct
        let s_72_95: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_94 as isize);
            tracer.read_register(s_72_94 as isize, value);
            value
        };
        // C s_72_96: const #102552u : u32
        let s_72_96: u32 = 102552;
        // N s_72_97: write-reg s_72_96 <= s_72_95
        let s_72_97: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_96 as isize, s_72_95);
            tracer.write_register(s_72_96 as isize, s_72_95);
        };
        // C s_72_98: const #102552u : u32
        let s_72_98: u32 = 102552;
        // D s_72_99: read-reg s_72_98:struct
        let s_72_99: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_98 as isize);
            tracer.read_register(s_72_98 as isize, value);
            value
        };
        // C s_72_100: const #102552u : u32
        let s_72_100: u32 = 102552;
        // N s_72_101: write-reg s_72_100 <= s_72_99
        let s_72_101: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_100 as isize, s_72_99);
            tracer.write_register(s_72_100 as isize, s_72_99);
        };
        // C s_72_102: const #102552u : u32
        let s_72_102: u32 = 102552;
        // D s_72_103: read-reg s_72_102:struct
        let s_72_103: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_102 as isize);
            tracer.read_register(s_72_102 as isize, value);
            value
        };
        // C s_72_104: const #102552u : u32
        let s_72_104: u32 = 102552;
        // N s_72_105: write-reg s_72_104 <= s_72_103
        let s_72_105: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_104 as isize, s_72_103);
            tracer.write_register(s_72_104 as isize, s_72_103);
        };
        // C s_72_106: const #102552u : u32
        let s_72_106: u32 = 102552;
        // D s_72_107: read-reg s_72_106:struct
        let s_72_107: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_106 as isize);
            tracer.read_register(s_72_106 as isize, value);
            value
        };
        // C s_72_108: const #102552u : u32
        let s_72_108: u32 = 102552;
        // N s_72_109: write-reg s_72_108 <= s_72_107
        let s_72_109: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_108 as isize, s_72_107);
            tracer.write_register(s_72_108 as isize, s_72_107);
        };
        // C s_72_110: const #102552u : u32
        let s_72_110: u32 = 102552;
        // D s_72_111: read-reg s_72_110:struct
        let s_72_111: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_110 as isize);
            tracer.read_register(s_72_110 as isize, value);
            value
        };
        // C s_72_112: const #102552u : u32
        let s_72_112: u32 = 102552;
        // N s_72_113: write-reg s_72_112 <= s_72_111
        let s_72_113: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_112 as isize, s_72_111);
            tracer.write_register(s_72_112 as isize, s_72_111);
        };
        // C s_72_114: const #102552u : u32
        let s_72_114: u32 = 102552;
        // D s_72_115: read-reg s_72_114:struct
        let s_72_115: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_114 as isize);
            tracer.read_register(s_72_114 as isize, value);
            value
        };
        // C s_72_116: const #102552u : u32
        let s_72_116: u32 = 102552;
        // N s_72_117: write-reg s_72_116 <= s_72_115
        let s_72_117: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_116 as isize, s_72_115);
            tracer.write_register(s_72_116 as isize, s_72_115);
        };
        // C s_72_118: const #102552u : u32
        let s_72_118: u32 = 102552;
        // D s_72_119: read-reg s_72_118:struct
        let s_72_119: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_118 as isize);
            tracer.read_register(s_72_118 as isize, value);
            value
        };
        // C s_72_120: const #102552u : u32
        let s_72_120: u32 = 102552;
        // N s_72_121: write-reg s_72_120 <= s_72_119
        let s_72_121: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_120 as isize, s_72_119);
            tracer.write_register(s_72_120 as isize, s_72_119);
        };
        // C s_72_122: const #102552u : u32
        let s_72_122: u32 = 102552;
        // D s_72_123: read-reg s_72_122:struct
        let s_72_123: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_122 as isize);
            tracer.read_register(s_72_122 as isize, value);
            value
        };
        // C s_72_124: const #102552u : u32
        let s_72_124: u32 = 102552;
        // N s_72_125: write-reg s_72_124 <= s_72_123
        let s_72_125: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_124 as isize, s_72_123);
            tracer.write_register(s_72_124 as isize, s_72_123);
        };
        // C s_72_126: const #102552u : u32
        let s_72_126: u32 = 102552;
        // D s_72_127: read-reg s_72_126:struct
        let s_72_127: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_126 as isize);
            tracer.read_register(s_72_126 as isize, value);
            value
        };
        // C s_72_128: const #102552u : u32
        let s_72_128: u32 = 102552;
        // N s_72_129: write-reg s_72_128 <= s_72_127
        let s_72_129: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_128 as isize, s_72_127);
            tracer.write_register(s_72_128 as isize, s_72_127);
        };
        // C s_72_130: const #102552u : u32
        let s_72_130: u32 = 102552;
        // D s_72_131: read-reg s_72_130:struct
        let s_72_131: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_130 as isize);
            tracer.read_register(s_72_130 as isize, value);
            value
        };
        // C s_72_132: const #102552u : u32
        let s_72_132: u32 = 102552;
        // N s_72_133: write-reg s_72_132 <= s_72_131
        let s_72_133: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_132 as isize, s_72_131);
            tracer.write_register(s_72_132 as isize, s_72_131);
        };
        // C s_72_134: const #102552u : u32
        let s_72_134: u32 = 102552;
        // D s_72_135: read-reg s_72_134:struct
        let s_72_135: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_134 as isize);
            tracer.read_register(s_72_134 as isize, value);
            value
        };
        // C s_72_136: const #102552u : u32
        let s_72_136: u32 = 102552;
        // N s_72_137: write-reg s_72_136 <= s_72_135
        let s_72_137: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_136 as isize, s_72_135);
            tracer.write_register(s_72_136 as isize, s_72_135);
        };
        // C s_72_138: const #102552u : u32
        let s_72_138: u32 = 102552;
        // D s_72_139: read-reg s_72_138:struct
        let s_72_139: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_138 as isize);
            tracer.read_register(s_72_138 as isize, value);
            value
        };
        // C s_72_140: const #102552u : u32
        let s_72_140: u32 = 102552;
        // N s_72_141: write-reg s_72_140 <= s_72_139
        let s_72_141: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_140 as isize, s_72_139);
            tracer.write_register(s_72_140 as isize, s_72_139);
        };
        // C s_72_142: const #102552u : u32
        let s_72_142: u32 = 102552;
        // D s_72_143: read-reg s_72_142:struct
        let s_72_143: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_142 as isize);
            tracer.read_register(s_72_142 as isize, value);
            value
        };
        // C s_72_144: const #102552u : u32
        let s_72_144: u32 = 102552;
        // N s_72_145: write-reg s_72_144 <= s_72_143
        let s_72_145: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_144 as isize, s_72_143);
            tracer.write_register(s_72_144 as isize, s_72_143);
        };
        // C s_72_146: const #102552u : u32
        let s_72_146: u32 = 102552;
        // D s_72_147: read-reg s_72_146:struct
        let s_72_147: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_146 as isize);
            tracer.read_register(s_72_146 as isize, value);
            value
        };
        // C s_72_148: const #102552u : u32
        let s_72_148: u32 = 102552;
        // N s_72_149: write-reg s_72_148 <= s_72_147
        let s_72_149: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_148 as isize, s_72_147);
            tracer.write_register(s_72_148 as isize, s_72_147);
        };
        // C s_72_150: const #102552u : u32
        let s_72_150: u32 = 102552;
        // D s_72_151: read-reg s_72_150:struct
        let s_72_151: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_150 as isize);
            tracer.read_register(s_72_150 as isize, value);
            value
        };
        // C s_72_152: const #102552u : u32
        let s_72_152: u32 = 102552;
        // N s_72_153: write-reg s_72_152 <= s_72_151
        let s_72_153: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_152 as isize, s_72_151);
            tracer.write_register(s_72_152 as isize, s_72_151);
        };
        // C s_72_154: const #102552u : u32
        let s_72_154: u32 = 102552;
        // D s_72_155: read-reg s_72_154:struct
        let s_72_155: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_154 as isize);
            tracer.read_register(s_72_154 as isize, value);
            value
        };
        // C s_72_156: const #102552u : u32
        let s_72_156: u32 = 102552;
        // N s_72_157: write-reg s_72_156 <= s_72_155
        let s_72_157: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_156 as isize, s_72_155);
            tracer.write_register(s_72_156 as isize, s_72_155);
        };
        // C s_72_158: const #102552u : u32
        let s_72_158: u32 = 102552;
        // D s_72_159: read-reg s_72_158:struct
        let s_72_159: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_158 as isize);
            tracer.read_register(s_72_158 as isize, value);
            value
        };
        // C s_72_160: const #102552u : u32
        let s_72_160: u32 = 102552;
        // N s_72_161: write-reg s_72_160 <= s_72_159
        let s_72_161: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_160 as isize, s_72_159);
            tracer.write_register(s_72_160 as isize, s_72_159);
        };
        // C s_72_162: const #102552u : u32
        let s_72_162: u32 = 102552;
        // D s_72_163: read-reg s_72_162:struct
        let s_72_163: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_162 as isize);
            tracer.read_register(s_72_162 as isize, value);
            value
        };
        // C s_72_164: const #102552u : u32
        let s_72_164: u32 = 102552;
        // N s_72_165: write-reg s_72_164 <= s_72_163
        let s_72_165: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_164 as isize, s_72_163);
            tracer.write_register(s_72_164 as isize, s_72_163);
        };
        // C s_72_166: const #102552u : u32
        let s_72_166: u32 = 102552;
        // D s_72_167: read-reg s_72_166:struct
        let s_72_167: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_166 as isize);
            tracer.read_register(s_72_166 as isize, value);
            value
        };
        // C s_72_168: const #102552u : u32
        let s_72_168: u32 = 102552;
        // N s_72_169: write-reg s_72_168 <= s_72_167
        let s_72_169: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_168 as isize, s_72_167);
            tracer.write_register(s_72_168 as isize, s_72_167);
        };
        // C s_72_170: const #102552u : u32
        let s_72_170: u32 = 102552;
        // D s_72_171: read-reg s_72_170:struct
        let s_72_171: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_170 as isize);
            tracer.read_register(s_72_170 as isize, value);
            value
        };
        // C s_72_172: const #102552u : u32
        let s_72_172: u32 = 102552;
        // N s_72_173: write-reg s_72_172 <= s_72_171
        let s_72_173: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_172 as isize, s_72_171);
            tracer.write_register(s_72_172 as isize, s_72_171);
        };
        // C s_72_174: const #102552u : u32
        let s_72_174: u32 = 102552;
        // D s_72_175: read-reg s_72_174:struct
        let s_72_175: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_174 as isize);
            tracer.read_register(s_72_174 as isize, value);
            value
        };
        // C s_72_176: const #102552u : u32
        let s_72_176: u32 = 102552;
        // N s_72_177: write-reg s_72_176 <= s_72_175
        let s_72_177: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_176 as isize, s_72_175);
            tracer.write_register(s_72_176 as isize, s_72_175);
        };
        // C s_72_178: const #102552u : u32
        let s_72_178: u32 = 102552;
        // D s_72_179: read-reg s_72_178:struct
        let s_72_179: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_178 as isize);
            tracer.read_register(s_72_178 as isize, value);
            value
        };
        // C s_72_180: const #102552u : u32
        let s_72_180: u32 = 102552;
        // N s_72_181: write-reg s_72_180 <= s_72_179
        let s_72_181: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_180 as isize, s_72_179);
            tracer.write_register(s_72_180 as isize, s_72_179);
        };
        // C s_72_182: const #16s : i
        let s_72_182: i128 = 16;
        // S s_72_183: call Zeros(s_72_182)
        let s_72_183: Bits = Zeros(state, tracer, s_72_182);
        // C s_72_184: const #104936u : u32
        let s_72_184: u32 = 104936;
        // D s_72_185: read-reg s_72_184:struct
        let s_72_185: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_184 as isize);
            tracer.read_register(s_72_184 as isize, value);
            value
        };
        // C s_72_186: const #104936u : u32
        let s_72_186: u32 = 104936;
        // N s_72_187: write-reg s_72_186 <= s_72_185
        let s_72_187: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_186 as isize, s_72_185);
            tracer.write_register(s_72_186 as isize, s_72_185);
        };
        // C s_72_188: const #20784u : u32
        let s_72_188: u32 = 20784;
        // D s_72_189: read-reg s_72_188:struct
        let s_72_189: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_188 as isize);
            tracer.read_register(s_72_188 as isize, value);
            value
        };
        // C s_72_190: const #20784u : u32
        let s_72_190: u32 = 20784;
        // N s_72_191: write-reg s_72_190 <= s_72_189
        let s_72_191: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_190 as isize, s_72_189);
            tracer.write_register(s_72_190 as isize, s_72_189);
        };
        // C s_72_192: const #20784u : u32
        let s_72_192: u32 = 20784;
        // D s_72_193: read-reg s_72_192:struct
        let s_72_193: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_192 as isize);
            tracer.read_register(s_72_192 as isize, value);
            value
        };
        // C s_72_194: const #20784u : u32
        let s_72_194: u32 = 20784;
        // N s_72_195: write-reg s_72_194 <= s_72_193
        let s_72_195: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_194 as isize, s_72_193);
            tracer.write_register(s_72_194 as isize, s_72_193);
        };
        // C s_72_196: const #20784u : u32
        let s_72_196: u32 = 20784;
        // D s_72_197: read-reg s_72_196:struct
        let s_72_197: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_196 as isize);
            tracer.read_register(s_72_196 as isize, value);
            value
        };
        // C s_72_198: const #20784u : u32
        let s_72_198: u32 = 20784;
        // N s_72_199: write-reg s_72_198 <= s_72_197
        let s_72_199: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_198 as isize, s_72_197);
            tracer.write_register(s_72_198 as isize, s_72_197);
        };
        // C s_72_200: const #20784u : u32
        let s_72_200: u32 = 20784;
        // D s_72_201: read-reg s_72_200:struct
        let s_72_201: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_200 as isize);
            tracer.read_register(s_72_200 as isize, value);
            value
        };
        // C s_72_202: const #20784u : u32
        let s_72_202: u32 = 20784;
        // N s_72_203: write-reg s_72_202 <= s_72_201
        let s_72_203: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_202 as isize, s_72_201);
            tracer.write_register(s_72_202 as isize, s_72_201);
        };
        // C s_72_204: const #20784u : u32
        let s_72_204: u32 = 20784;
        // D s_72_205: read-reg s_72_204:struct
        let s_72_205: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_204 as isize);
            tracer.read_register(s_72_204 as isize, value);
            value
        };
        // C s_72_206: const #20784u : u32
        let s_72_206: u32 = 20784;
        // N s_72_207: write-reg s_72_206 <= s_72_205
        let s_72_207: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_206 as isize, s_72_205);
            tracer.write_register(s_72_206 as isize, s_72_205);
        };
        // C s_72_208: const #20784u : u32
        let s_72_208: u32 = 20784;
        // D s_72_209: read-reg s_72_208:struct
        let s_72_209: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_208 as isize);
            tracer.read_register(s_72_208 as isize, value);
            value
        };
        // C s_72_210: const #20784u : u32
        let s_72_210: u32 = 20784;
        // N s_72_211: write-reg s_72_210 <= s_72_209
        let s_72_211: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_210 as isize, s_72_209);
            tracer.write_register(s_72_210 as isize, s_72_209);
        };
        // C s_72_212: const #20784u : u32
        let s_72_212: u32 = 20784;
        // D s_72_213: read-reg s_72_212:struct
        let s_72_213: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_212 as isize);
            tracer.read_register(s_72_212 as isize, value);
            value
        };
        // C s_72_214: const #20784u : u32
        let s_72_214: u32 = 20784;
        // N s_72_215: write-reg s_72_214 <= s_72_213
        let s_72_215: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_214 as isize, s_72_213);
            tracer.write_register(s_72_214 as isize, s_72_213);
        };
        // C s_72_216: const #20784u : u32
        let s_72_216: u32 = 20784;
        // D s_72_217: read-reg s_72_216:struct
        let s_72_217: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_216 as isize);
            tracer.read_register(s_72_216 as isize, value);
            value
        };
        // C s_72_218: const #20784u : u32
        let s_72_218: u32 = 20784;
        // N s_72_219: write-reg s_72_218 <= s_72_217
        let s_72_219: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_218 as isize, s_72_217);
            tracer.write_register(s_72_218 as isize, s_72_217);
        };
        // C s_72_220: const #20784u : u32
        let s_72_220: u32 = 20784;
        // D s_72_221: read-reg s_72_220:struct
        let s_72_221: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_220 as isize);
            tracer.read_register(s_72_220 as isize, value);
            value
        };
        // C s_72_222: const #20784u : u32
        let s_72_222: u32 = 20784;
        // N s_72_223: write-reg s_72_222 <= s_72_221
        let s_72_223: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_222 as isize, s_72_221);
            tracer.write_register(s_72_222 as isize, s_72_221);
        };
        // C s_72_224: const #20784u : u32
        let s_72_224: u32 = 20784;
        // D s_72_225: read-reg s_72_224:struct
        let s_72_225: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_224 as isize);
            tracer.read_register(s_72_224 as isize, value);
            value
        };
        // C s_72_226: const #20784u : u32
        let s_72_226: u32 = 20784;
        // N s_72_227: write-reg s_72_226 <= s_72_225
        let s_72_227: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_226 as isize, s_72_225);
            tracer.write_register(s_72_226 as isize, s_72_225);
        };
        // C s_72_228: const #12088u : u32
        let s_72_228: u32 = 12088;
        // D s_72_229: read-reg s_72_228:struct
        let s_72_229: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_228 as isize);
            tracer.read_register(s_72_228 as isize, value);
            value
        };
        // C s_72_230: const #12088u : u32
        let s_72_230: u32 = 12088;
        // N s_72_231: write-reg s_72_230 <= s_72_229
        let s_72_231: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_230 as isize, s_72_229);
            tracer.write_register(s_72_230 as isize, s_72_229);
        };
        // C s_72_232: const #12088u : u32
        let s_72_232: u32 = 12088;
        // D s_72_233: read-reg s_72_232:struct
        let s_72_233: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_232 as isize);
            tracer.read_register(s_72_232 as isize, value);
            value
        };
        // C s_72_234: const #12088u : u32
        let s_72_234: u32 = 12088;
        // N s_72_235: write-reg s_72_234 <= s_72_233
        let s_72_235: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_234 as isize, s_72_233);
            tracer.write_register(s_72_234 as isize, s_72_233);
        };
        // C s_72_236: const #90272u : u32
        let s_72_236: u32 = 90272;
        // D s_72_237: read-reg s_72_236:struct
        let s_72_237: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_236 as isize);
            tracer.read_register(s_72_236 as isize, value);
            value
        };
        // C s_72_238: const #90272u : u32
        let s_72_238: u32 = 90272;
        // N s_72_239: write-reg s_72_238 <= s_72_237
        let s_72_239: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_238 as isize, s_72_237);
            tracer.write_register(s_72_238 as isize, s_72_237);
        };
        // C s_72_240: const #90272u : u32
        let s_72_240: u32 = 90272;
        // D s_72_241: read-reg s_72_240:struct
        let s_72_241: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_240 as isize);
            tracer.read_register(s_72_240 as isize, value);
            value
        };
        // C s_72_242: const #90272u : u32
        let s_72_242: u32 = 90272;
        // N s_72_243: write-reg s_72_242 <= s_72_241
        let s_72_243: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_242 as isize, s_72_241);
            tracer.write_register(s_72_242 as isize, s_72_241);
        };
        // C s_72_244: const #90272u : u32
        let s_72_244: u32 = 90272;
        // D s_72_245: read-reg s_72_244:struct
        let s_72_245: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_244 as isize);
            tracer.read_register(s_72_244 as isize, value);
            value
        };
        // C s_72_246: const #90272u : u32
        let s_72_246: u32 = 90272;
        // N s_72_247: write-reg s_72_246 <= s_72_245
        let s_72_247: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_246 as isize, s_72_245);
            tracer.write_register(s_72_246 as isize, s_72_245);
        };
        // C s_72_248: const #90272u : u32
        let s_72_248: u32 = 90272;
        // D s_72_249: read-reg s_72_248:struct
        let s_72_249: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_248 as isize);
            tracer.read_register(s_72_248 as isize, value);
            value
        };
        // C s_72_250: const #90272u : u32
        let s_72_250: u32 = 90272;
        // N s_72_251: write-reg s_72_250 <= s_72_249
        let s_72_251: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_250 as isize, s_72_249);
            tracer.write_register(s_72_250 as isize, s_72_249);
        };
        // C s_72_252: const #90272u : u32
        let s_72_252: u32 = 90272;
        // D s_72_253: read-reg s_72_252:struct
        let s_72_253: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_252 as isize);
            tracer.read_register(s_72_252 as isize, value);
            value
        };
        // C s_72_254: const #90272u : u32
        let s_72_254: u32 = 90272;
        // N s_72_255: write-reg s_72_254 <= s_72_253
        let s_72_255: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_254 as isize, s_72_253);
            tracer.write_register(s_72_254 as isize, s_72_253);
        };
        // C s_72_256: const #17072u : u32
        let s_72_256: u32 = 17072;
        // D s_72_257: read-reg s_72_256:struct
        let s_72_257: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_256 as isize);
            tracer.read_register(s_72_256 as isize, value);
            value
        };
        // C s_72_258: const #17072u : u32
        let s_72_258: u32 = 17072;
        // N s_72_259: write-reg s_72_258 <= s_72_257
        let s_72_259: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_258 as isize, s_72_257);
            tracer.write_register(s_72_258 as isize, s_72_257);
        };
        // C s_72_260: const #17072u : u32
        let s_72_260: u32 = 17072;
        // D s_72_261: read-reg s_72_260:struct
        let s_72_261: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_260 as isize);
            tracer.read_register(s_72_260 as isize, value);
            value
        };
        // C s_72_262: const #17072u : u32
        let s_72_262: u32 = 17072;
        // N s_72_263: write-reg s_72_262 <= s_72_261
        let s_72_263: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_262 as isize, s_72_261);
            tracer.write_register(s_72_262 as isize, s_72_261);
        };
        // C s_72_264: const #17072u : u32
        let s_72_264: u32 = 17072;
        // D s_72_265: read-reg s_72_264:struct
        let s_72_265: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_264 as isize);
            tracer.read_register(s_72_264 as isize, value);
            value
        };
        // C s_72_266: const #17072u : u32
        let s_72_266: u32 = 17072;
        // N s_72_267: write-reg s_72_266 <= s_72_265
        let s_72_267: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_266 as isize, s_72_265);
            tracer.write_register(s_72_266 as isize, s_72_265);
        };
        // C s_72_268: const #17072u : u32
        let s_72_268: u32 = 17072;
        // D s_72_269: read-reg s_72_268:struct
        let s_72_269: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_268 as isize);
            tracer.read_register(s_72_268 as isize, value);
            value
        };
        // C s_72_270: const #17072u : u32
        let s_72_270: u32 = 17072;
        // N s_72_271: write-reg s_72_270 <= s_72_269
        let s_72_271: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_270 as isize, s_72_269);
            tracer.write_register(s_72_270 as isize, s_72_269);
        };
        // C s_72_272: const #17072u : u32
        let s_72_272: u32 = 17072;
        // D s_72_273: read-reg s_72_272:struct
        let s_72_273: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_272 as isize);
            tracer.read_register(s_72_272 as isize, value);
            value
        };
        // C s_72_274: const #17072u : u32
        let s_72_274: u32 = 17072;
        // N s_72_275: write-reg s_72_274 <= s_72_273
        let s_72_275: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_274 as isize, s_72_273);
            tracer.write_register(s_72_274 as isize, s_72_273);
        };
        // C s_72_276: const #14736u : u32
        let s_72_276: u32 = 14736;
        // D s_72_277: read-reg s_72_276:struct
        let s_72_277: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_276 as isize);
            tracer.read_register(s_72_276 as isize, value);
            value
        };
        // C s_72_278: const #14736u : u32
        let s_72_278: u32 = 14736;
        // N s_72_279: write-reg s_72_278 <= s_72_277
        let s_72_279: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_278 as isize, s_72_277);
            tracer.write_register(s_72_278 as isize, s_72_277);
        };
        // C s_72_280: const #14736u : u32
        let s_72_280: u32 = 14736;
        // D s_72_281: read-reg s_72_280:struct
        let s_72_281: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_280 as isize);
            tracer.read_register(s_72_280 as isize, value);
            value
        };
        // C s_72_282: const #14736u : u32
        let s_72_282: u32 = 14736;
        // N s_72_283: write-reg s_72_282 <= s_72_281
        let s_72_283: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_282 as isize, s_72_281);
            tracer.write_register(s_72_282 as isize, s_72_281);
        };
        // C s_72_284: const #14736u : u32
        let s_72_284: u32 = 14736;
        // D s_72_285: read-reg s_72_284:struct
        let s_72_285: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_284 as isize);
            tracer.read_register(s_72_284 as isize, value);
            value
        };
        // C s_72_286: const #14736u : u32
        let s_72_286: u32 = 14736;
        // N s_72_287: write-reg s_72_286 <= s_72_285
        let s_72_287: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_72_286 as isize, s_72_285);
            tracer.write_register(s_72_286 as isize, s_72_285);
        };
        // C s_72_288: const #1008u : u32
        let s_72_288: u32 = 1008;
        // D s_72_289: read-reg s_72_288:i64
        let s_72_289: i64 = {
            let value = state.read_register::<i64>(s_72_288 as isize);
            tracer.read_register(s_72_288 as isize, value);
            value
        };
        // C s_72_290: const #64s : i
        let s_72_290: i128 = 64;
        // D s_72_291: cast zx s_72_289 -> i
        let s_72_291: i128 = (i128::try_from(s_72_289).unwrap());
        // D s_72_292: cmp-eq s_72_291 s_72_290
        let s_72_292: bool = ((s_72_291) == (s_72_290));
        // D s_72_293: not s_72_292
        let s_72_293: bool = !s_72_292;
        // N s_72_294: branch s_72_293 b224 b73
        if s_72_293 {
            return block_224(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #14736u : u32
        let s_73_0: u32 = 14736;
        // D s_73_1: read-reg s_73_0:struct
        let s_73_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_73_0 as isize);
            tracer.read_register(s_73_0 as isize, value);
            value
        };
        // C s_73_2: const #14736u : u32
        let s_73_2: u32 = 14736;
        // N s_73_3: write-reg s_73_2 <= s_73_1
        let s_73_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_73_2 as isize, s_73_1);
            tracer.write_register(s_73_2 as isize, s_73_1);
        };
        // N s_73_4: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #14736u : u32
        let s_74_0: u32 = 14736;
        // D s_74_1: read-reg s_74_0:struct
        let s_74_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_74_0 as isize);
            tracer.read_register(s_74_0 as isize, value);
            value
        };
        // C s_74_2: const #14736u : u32
        let s_74_2: u32 = 14736;
        // N s_74_3: write-reg s_74_2 <= s_74_1
        let s_74_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_74_2 as isize, s_74_1);
            tracer.write_register(s_74_2 as isize, s_74_1);
        };
        // C s_74_4: const #14736u : u32
        let s_74_4: u32 = 14736;
        // D s_74_5: read-reg s_74_4:struct
        let s_74_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_74_4 as isize);
            tracer.read_register(s_74_4 as isize, value);
            value
        };
        // C s_74_6: const #14736u : u32
        let s_74_6: u32 = 14736;
        // N s_74_7: write-reg s_74_6 <= s_74_5
        let s_74_7: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_74_6 as isize, s_74_5);
            tracer.write_register(s_74_6 as isize, s_74_5);
        };
        // C s_74_8: const #14736u : u32
        let s_74_8: u32 = 14736;
        // D s_74_9: read-reg s_74_8:struct
        let s_74_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_74_8 as isize);
            tracer.read_register(s_74_8 as isize, value);
            value
        };
        // C s_74_10: const #14736u : u32
        let s_74_10: u32 = 14736;
        // N s_74_11: write-reg s_74_10 <= s_74_9
        let s_74_11: () = {
            state
                .write_register::<ProductType5c790c8ef59cc8b2>(s_74_10 as isize, s_74_9);
            tracer.write_register(s_74_10 as isize, s_74_9);
        };
        // C s_74_12: const #14736u : u32
        let s_74_12: u32 = 14736;
        // D s_74_13: read-reg s_74_12:struct
        let s_74_13: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_74_12 as isize);
            tracer.read_register(s_74_12 as isize, value);
            value
        };
        // C s_74_14: const #14736u : u32
        let s_74_14: u32 = 14736;
        // N s_74_15: write-reg s_74_14 <= s_74_13
        let s_74_15: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_74_14 as isize, s_74_13);
            tracer.write_register(s_74_14 as isize, s_74_13);
        };
        // C s_74_16: const #() : ()
        let s_74_16: () = ();
        // S s_74_17: call EL2Enabled(s_74_16)
        let s_74_17: bool = EL2Enabled(state, tracer, s_74_16);
        // N s_74_18: branch s_74_17 b223 b75
        if s_74_17 {
            return block_223(state, tracer, fn_state);
        } else {
            return block_75(state, tracer, fn_state);
        };
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_75_0: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #21016u : u32
        let s_76_0: u32 = 21016;
        // D s_76_1: read-reg s_76_0:struct
        let s_76_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_76_0 as isize);
            tracer.read_register(s_76_0 as isize, value);
            value
        };
        // D s_76_2: call _get_PMCR_EL0_Type_N(s_76_1)
        let s_76_2: u8 = u_get_PMCR_EL0_Type_N(state, tracer, s_76_1);
        // C s_76_3: const #104880u : u32
        let s_76_3: u32 = 104880;
        // D s_76_4: read-reg s_76_3:struct
        let s_76_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_76_3 as isize);
            tracer.read_register(s_76_3 as isize, value);
            value
        };
        // C s_76_5: const #104880u : u32
        let s_76_5: u32 = 104880;
        // N s_76_6: write-reg s_76_5 <= s_76_4
        let s_76_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_76_5 as isize, s_76_4);
            tracer.write_register(s_76_5 as isize, s_76_4);
        };
        // C s_76_7: const #10200u : u32
        let s_76_7: u32 = 10200;
        // D s_76_8: read-reg s_76_7:struct
        let s_76_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_76_7 as isize);
            tracer.read_register(s_76_7 as isize, value);
            value
        };
        // C s_76_9: const #10200u : u32
        let s_76_9: u32 = 10200;
        // N s_76_10: write-reg s_76_9 <= s_76_8
        let s_76_10: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_76_9 as isize, s_76_8);
            tracer.write_register(s_76_9 as isize, s_76_8);
        };
        // C s_76_11: const #10200u : u32
        let s_76_11: u32 = 10200;
        // D s_76_12: read-reg s_76_11:struct
        let s_76_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_76_11 as isize);
            tracer.read_register(s_76_11 as isize, value);
            value
        };
        // C s_76_13: const #10200u : u32
        let s_76_13: u32 = 10200;
        // N s_76_14: write-reg s_76_13 <= s_76_12
        let s_76_14: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_76_13 as isize, s_76_12);
            tracer.write_register(s_76_13 as isize, s_76_12);
        };
        // C s_76_15: const #11904u : u32
        let s_76_15: u32 = 11904;
        // D s_76_16: read-reg s_76_15:struct
        let s_76_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_76_15 as isize);
            tracer.read_register(s_76_15 as isize, value);
            value
        };
        // C s_76_17: const #11904u : u32
        let s_76_17: u32 = 11904;
        // N s_76_18: write-reg s_76_17 <= s_76_16
        let s_76_18: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_76_17 as isize, s_76_16);
            tracer.write_register(s_76_17 as isize, s_76_16);
        };
        // C s_76_19: const #22960u : u32
        let s_76_19: u32 = 22960;
        // D s_76_20: read-reg s_76_19:struct
        let s_76_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_76_19 as isize);
            tracer.read_register(s_76_19 as isize, value);
            value
        };
        // C s_76_21: const #22960u : u32
        let s_76_21: u32 = 22960;
        // N s_76_22: write-reg s_76_21 <= s_76_20
        let s_76_22: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_76_21 as isize, s_76_20);
            tracer.write_register(s_76_21 as isize, s_76_20);
        };
        // C s_76_23: const #16368u : u32
        let s_76_23: u32 = 16368;
        // D s_76_24: read-reg s_76_23:struct
        let s_76_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_76_23 as isize);
            tracer.read_register(s_76_23 as isize, value);
            value
        };
        // C s_76_25: const #16368u : u32
        let s_76_25: u32 = 16368;
        // N s_76_26: write-reg s_76_25 <= s_76_24
        let s_76_26: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_76_25 as isize, s_76_24);
            tracer.write_register(s_76_25 as isize, s_76_24);
        };
        // C s_76_27: const #19056u : u32
        let s_76_27: u32 = 19056;
        // D s_76_28: read-reg s_76_27:struct
        let s_76_28: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_76_27 as isize);
            tracer.read_register(s_76_27 as isize, value);
            value
        };
        // C s_76_29: const #19056u : u32
        let s_76_29: u32 = 19056;
        // N s_76_30: write-reg s_76_29 <= s_76_28
        let s_76_30: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_76_29 as isize, s_76_28);
            tracer.write_register(s_76_29 as isize, s_76_28);
        };
        // C s_76_31: const #19056u : u32
        let s_76_31: u32 = 19056;
        // D s_76_32: read-reg s_76_31:struct
        let s_76_32: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_76_31 as isize);
            tracer.read_register(s_76_31 as isize, value);
            value
        };
        // C s_76_33: const #19056u : u32
        let s_76_33: u32 = 19056;
        // N s_76_34: write-reg s_76_33 <= s_76_32
        let s_76_34: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_76_33 as isize, s_76_32);
            tracer.write_register(s_76_33 as isize, s_76_32);
        };
        // C s_76_35: const #19056u : u32
        let s_76_35: u32 = 19056;
        // D s_76_36: read-reg s_76_35:struct
        let s_76_36: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_76_35 as isize);
            tracer.read_register(s_76_35 as isize, value);
            value
        };
        // C s_76_37: const #19056u : u32
        let s_76_37: u32 = 19056;
        // N s_76_38: write-reg s_76_37 <= s_76_36
        let s_76_38: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_76_37 as isize, s_76_36);
            tracer.write_register(s_76_37 as isize, s_76_36);
        };
        // C s_76_39: const #19056u : u32
        let s_76_39: u32 = 19056;
        // D s_76_40: read-reg s_76_39:struct
        let s_76_40: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_76_39 as isize);
            tracer.read_register(s_76_39 as isize, value);
            value
        };
        // C s_76_41: const #19056u : u32
        let s_76_41: u32 = 19056;
        // N s_76_42: write-reg s_76_41 <= s_76_40
        let s_76_42: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_76_41 as isize, s_76_40);
            tracer.write_register(s_76_41 as isize, s_76_40);
        };
        // C s_76_43: const #19056u : u32
        let s_76_43: u32 = 19056;
        // D s_76_44: read-reg s_76_43:struct
        let s_76_44: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_76_43 as isize);
            tracer.read_register(s_76_43 as isize, value);
            value
        };
        // C s_76_45: const #19056u : u32
        let s_76_45: u32 = 19056;
        // N s_76_46: write-reg s_76_45 <= s_76_44
        let s_76_46: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_76_45 as isize, s_76_44);
            tracer.write_register(s_76_45 as isize, s_76_44);
        };
        // C s_76_47: const #() : ()
        let s_76_47: () = ();
        // S s_76_48: call HaveMTE2Ext(s_76_47)
        let s_76_48: bool = HaveMTE2Ext(state, tracer, s_76_47);
        // N s_76_49: branch s_76_48 b222 b77
        if s_76_48 {
            return block_222(state, tracer, fn_state);
        } else {
            return block_77(state, tracer, fn_state);
        };
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_77_0: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #() : ()
        let s_78_0: () = ();
        // S s_78_1: call HaveMTE2Ext(s_78_0)
        let s_78_1: bool = HaveMTE2Ext(state, tracer, s_78_0);
        // N s_78_2: branch s_78_1 b221 b79
        if s_78_1 {
            return block_221(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_79_0: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #15376u : u32
        let s_80_0: u32 = 15376;
        // D s_80_1: read-reg s_80_0:struct
        let s_80_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_80_0 as isize);
            tracer.read_register(s_80_0 as isize, value);
            value
        };
        // C s_80_2: const #15376u : u32
        let s_80_2: u32 = 15376;
        // N s_80_3: write-reg s_80_2 <= s_80_1
        let s_80_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_80_2 as isize, s_80_1);
            tracer.write_register(s_80_2 as isize, s_80_1);
        };
        // C s_80_4: const #15376u : u32
        let s_80_4: u32 = 15376;
        // D s_80_5: read-reg s_80_4:struct
        let s_80_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_80_4 as isize);
            tracer.read_register(s_80_4 as isize, value);
            value
        };
        // C s_80_6: const #15376u : u32
        let s_80_6: u32 = 15376;
        // N s_80_7: write-reg s_80_6 <= s_80_5
        let s_80_7: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_80_6 as isize, s_80_5);
            tracer.write_register(s_80_6 as isize, s_80_5);
        };
        // C s_80_8: const #() : ()
        let s_80_8: () = ();
        // S s_80_9: call ICC_CTLR_EL1_read(s_80_8)
        let s_80_9: ProductType5c790c8ef59cc8b2 = ICC_CTLR_EL1_read(
            state,
            tracer,
            s_80_8,
        );
        // C s_80_10: const #1u : u8
        let s_80_10: bool = true;
        // S s_80_11: call _update_ICC_CTLR_EL1_Type_ExtRange(s_80_9, s_80_10)
        let s_80_11: ProductType5c790c8ef59cc8b2 = u_update_ICC_CTLR_EL1_Type_ExtRange(
            state,
            tracer,
            s_80_9,
            s_80_10,
        );
        // S s_80_12: call ICC_CTLR_EL1_write(s_80_11)
        let s_80_12: () = ICC_CTLR_EL1_write(state, tracer, s_80_11);
        // C s_80_13: const #() : ()
        let s_80_13: () = ();
        // S s_80_14: call ICC_CTLR_EL1_read(s_80_13)
        let s_80_14: ProductType5c790c8ef59cc8b2 = ICC_CTLR_EL1_read(
            state,
            tracer,
            s_80_13,
        );
        // C s_80_15: const #4u : u8
        let s_80_15: u8 = 4;
        // S s_80_16: call _update_ICC_CTLR_EL1_Type_PRIbits(s_80_14, s_80_15)
        let s_80_16: ProductType5c790c8ef59cc8b2 = u_update_ICC_CTLR_EL1_Type_PRIbits(
            state,
            tracer,
            s_80_14,
            s_80_15,
        );
        // S s_80_17: call ICC_CTLR_EL1_write(s_80_16)
        let s_80_17: () = ICC_CTLR_EL1_write(state, tracer, s_80_16);
        // C s_80_18: const #() : ()
        let s_80_18: () = ();
        // S s_80_19: call ICC_CTLR_EL1_read(s_80_18)
        let s_80_19: ProductType5c790c8ef59cc8b2 = ICC_CTLR_EL1_read(
            state,
            tracer,
            s_80_18,
        );
        // C s_80_20: const #1u : u8
        let s_80_20: bool = true;
        // S s_80_21: call _update_ICC_CTLR_EL1_Type_A3V(s_80_19, s_80_20)
        let s_80_21: ProductType5c790c8ef59cc8b2 = u_update_ICC_CTLR_EL1_Type_A3V(
            state,
            tracer,
            s_80_19,
            s_80_20,
        );
        // S s_80_22: call ICC_CTLR_EL1_write(s_80_21)
        let s_80_22: () = ICC_CTLR_EL1_write(state, tracer, s_80_21);
        // C s_80_23: const #() : ()
        let s_80_23: () = ();
        // S s_80_24: call ICC_CTLR_EL1_read(s_80_23)
        let s_80_24: ProductType5c790c8ef59cc8b2 = ICC_CTLR_EL1_read(
            state,
            tracer,
            s_80_23,
        );
        // C s_80_25: const #1u : u8
        let s_80_25: u8 = 1;
        // S s_80_26: call _update_ICC_CTLR_EL1_Type_IDbits(s_80_24, s_80_25)
        let s_80_26: ProductType5c790c8ef59cc8b2 = u_update_ICC_CTLR_EL1_Type_IDbits(
            state,
            tracer,
            s_80_24,
            s_80_25,
        );
        // S s_80_27: call ICC_CTLR_EL1_write(s_80_26)
        let s_80_27: () = ICC_CTLR_EL1_write(state, tracer, s_80_26);
        // C s_80_28: const #12824u : u32
        let s_80_28: u32 = 12824;
        // D s_80_29: read-reg s_80_28:struct
        let s_80_29: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_80_28 as isize);
            tracer.read_register(s_80_28 as isize, value);
            value
        };
        // C s_80_30: const #12824u : u32
        let s_80_30: u32 = 12824;
        // N s_80_31: write-reg s_80_30 <= s_80_29
        let s_80_31: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_80_30 as isize, s_80_29);
            tracer.write_register(s_80_30 as isize, s_80_29);
        };
        // C s_80_32: const #12824u : u32
        let s_80_32: u32 = 12824;
        // D s_80_33: read-reg s_80_32:struct
        let s_80_33: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_80_32 as isize);
            tracer.read_register(s_80_32 as isize, value);
            value
        };
        // C s_80_34: const #12824u : u32
        let s_80_34: u32 = 12824;
        // N s_80_35: write-reg s_80_34 <= s_80_33
        let s_80_35: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_80_34 as isize, s_80_33);
            tracer.write_register(s_80_34 as isize, s_80_33);
        };
        // C s_80_36: const #12824u : u32
        let s_80_36: u32 = 12824;
        // D s_80_37: read-reg s_80_36:struct
        let s_80_37: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_80_36 as isize);
            tracer.read_register(s_80_36 as isize, value);
            value
        };
        // C s_80_38: const #12824u : u32
        let s_80_38: u32 = 12824;
        // N s_80_39: write-reg s_80_38 <= s_80_37
        let s_80_39: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_80_38 as isize, s_80_37);
            tracer.write_register(s_80_38 as isize, s_80_37);
        };
        // C s_80_40: const #12824u : u32
        let s_80_40: u32 = 12824;
        // D s_80_41: read-reg s_80_40:struct
        let s_80_41: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_80_40 as isize);
            tracer.read_register(s_80_40 as isize, value);
            value
        };
        // C s_80_42: const #12824u : u32
        let s_80_42: u32 = 12824;
        // N s_80_43: write-reg s_80_42 <= s_80_41
        let s_80_43: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_80_42 as isize, s_80_41);
            tracer.write_register(s_80_42 as isize, s_80_41);
        };
        // C s_80_44: const #17352u : u32
        let s_80_44: u32 = 17352;
        // D s_80_45: read-reg s_80_44:struct
        let s_80_45: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_80_44 as isize);
            tracer.read_register(s_80_44 as isize, value);
            value
        };
        // C s_80_46: const #17352u : u32
        let s_80_46: u32 = 17352;
        // N s_80_47: write-reg s_80_46 <= s_80_45
        let s_80_47: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_80_46 as isize, s_80_45);
            tracer.write_register(s_80_46 as isize, s_80_45);
        };
        // C s_80_48: const #17352u : u32
        let s_80_48: u32 = 17352;
        // D s_80_49: read-reg s_80_48:struct
        let s_80_49: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_80_48 as isize);
            tracer.read_register(s_80_48 as isize, value);
            value
        };
        // C s_80_50: const #17352u : u32
        let s_80_50: u32 = 17352;
        // N s_80_51: write-reg s_80_50 <= s_80_49
        let s_80_51: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_80_50 as isize, s_80_49);
            tracer.write_register(s_80_50 as isize, s_80_49);
        };
        // C s_80_52: const #17352u : u32
        let s_80_52: u32 = 17352;
        // D s_80_53: read-reg s_80_52:struct
        let s_80_53: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_80_52 as isize);
            tracer.read_register(s_80_52 as isize, value);
            value
        };
        // C s_80_54: const #17352u : u32
        let s_80_54: u32 = 17352;
        // N s_80_55: write-reg s_80_54 <= s_80_53
        let s_80_55: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_80_54 as isize, s_80_53);
            tracer.write_register(s_80_54 as isize, s_80_53);
        };
        // C s_80_56: const #17352u : u32
        let s_80_56: u32 = 17352;
        // D s_80_57: read-reg s_80_56:struct
        let s_80_57: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_80_56 as isize);
            tracer.read_register(s_80_56 as isize, value);
            value
        };
        // C s_80_58: const #17352u : u32
        let s_80_58: u32 = 17352;
        // N s_80_59: write-reg s_80_58 <= s_80_57
        let s_80_59: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_80_58 as isize, s_80_57);
            tracer.write_register(s_80_58 as isize, s_80_57);
        };
        // C s_80_60: const #23280u : u32
        let s_80_60: u32 = 23280;
        // D s_80_61: read-reg s_80_60:u64
        let s_80_61: u64 = {
            let value = state.read_register::<u64>(s_80_60 as isize);
            tracer.read_register(s_80_60 as isize, value);
            value
        };
        // D s_80_62: write-var ga#369677 <= s_80_61
        fn_state.ga_369677 = s_80_61;
        // C s_80_63: const #86u : u32
        let s_80_63: u32 = 86;
        // S s_80_64: call IsFeatureImplemented(s_80_63)
        let s_80_64: bool = IsFeatureImplemented(state, tracer, s_80_63);
        // N s_80_65: branch s_80_64 b220 b81
        if s_80_64 {
            return block_220(state, tracer, fn_state);
        } else {
            return block_81(state, tracer, fn_state);
        };
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #13s : i64
        let s_81_0: i64 = 13;
        // D s_81_1: write-var ga#369678 <= s_81_0
        fn_state.ga_369678 = s_81_0;
        // N s_81_2: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #24s : i
        let s_82_0: i128 = 24;
        // C s_82_1: const #255u : u8
        let s_82_1: u8 = 255;
        // C s_82_2: cast zx s_82_1 -> bv
        let s_82_2: Bits = Bits::new(s_82_1 as u128, 8u16);
        // D s_82_3: bits-cast zx s_82_2 -> bv length s_82_0
        let s_82_3: Bits = s_82_2.zero_extend(s_82_0);
        // D s_82_4: cast reint s_82_3 -> u24
        let s_82_4: u32 = (s_82_3.value() as u32);
        // C s_82_5: const #24s : i
        let s_82_5: i128 = 24;
        // D s_82_6: read-var ga#369677:u64
        let s_82_6: u64 = fn_state.ga_369677;
        // D s_82_7: cast zx s_82_6 -> bv
        let s_82_7: Bits = Bits::new(s_82_6 as u128, 64u16);
        // D s_82_8: read-var ga#369678:i64
        let s_82_8: i64 = fn_state.ga_369678;
        // D s_82_9: cast zx s_82_8 -> i
        let s_82_9: i128 = (i128::try_from(s_82_8).unwrap());
        // D s_82_10: cast zx s_82_4 -> bv
        let s_82_10: Bits = Bits::new(s_82_4 as u128, 24u16);
        // C s_82_11: const #1u : u64
        let s_82_11: u64 = 1;
        // C s_82_12: cast zx s_82_11 -> bv
        let s_82_12: Bits = Bits::new(s_82_11 as u128, 64u16);
        // C s_82_13: lsl s_82_12 s_82_5
        let s_82_13: Bits = s_82_12 << s_82_5;
        // C s_82_14: sub s_82_13 s_82_12
        let s_82_14: Bits = ((s_82_13) - (s_82_12));
        // D s_82_15: and s_82_10 s_82_14
        let s_82_15: Bits = ((s_82_10) & (s_82_14));
        // D s_82_16: lsl s_82_15 s_82_9
        let s_82_16: Bits = s_82_15 << s_82_9;
        // D s_82_17: lsl s_82_14 s_82_9
        let s_82_17: Bits = s_82_14 << s_82_9;
        // D s_82_18: cmpl s_82_17
        let s_82_18: Bits = !s_82_17;
        // D s_82_19: and s_82_7 s_82_18
        let s_82_19: Bits = ((s_82_7) & (s_82_18));
        // D s_82_20: or s_82_19 s_82_16
        let s_82_20: Bits = ((s_82_19) | (s_82_16));
        // D s_82_21: cast reint s_82_20 -> u64
        let s_82_21: u64 = (s_82_20.value() as u64);
        // D s_82_22: call Mk_CCSIDR_EL1_Type(s_82_21)
        let s_82_22: ProductType5c790c8ef59cc8b2 = Mk_CCSIDR_EL1_Type(
            state,
            tracer,
            s_82_21,
        );
        // C s_82_23: const #23280u : u32
        let s_82_23: u32 = 23280;
        // N s_82_24: write-reg s_82_23 <= s_82_22
        let s_82_24: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_82_23 as isize, s_82_22);
            tracer.write_register(s_82_23 as isize, s_82_22);
        };
        // C s_82_25: const #23280u : u32
        let s_82_25: u32 = 23280;
        // D s_82_26: read-reg s_82_25:struct
        let s_82_26: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_25 as isize);
            tracer.read_register(s_82_25 as isize, value);
            value
        };
        // C s_82_27: const #23280u : u32
        let s_82_27: u32 = 23280;
        // N s_82_28: write-reg s_82_27 <= s_82_26
        let s_82_28: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_82_27 as isize, s_82_26);
            tracer.write_register(s_82_27 as isize, s_82_26);
        };
        // C s_82_29: const #23280u : u32
        let s_82_29: u32 = 23280;
        // D s_82_30: read-reg s_82_29:struct
        let s_82_30: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_29 as isize);
            tracer.read_register(s_82_29 as isize, value);
            value
        };
        // C s_82_31: const #23280u : u32
        let s_82_31: u32 = 23280;
        // N s_82_32: write-reg s_82_31 <= s_82_30
        let s_82_32: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_82_31 as isize, s_82_30);
            tracer.write_register(s_82_31 as isize, s_82_30);
        };
        // C s_82_33: const #100976u : u32
        let s_82_33: u32 = 100976;
        // D s_82_34: read-reg s_82_33:struct
        let s_82_34: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_33 as isize);
            tracer.read_register(s_82_33 as isize, value);
            value
        };
        // C s_82_35: const #100976u : u32
        let s_82_35: u32 = 100976;
        // N s_82_36: write-reg s_82_35 <= s_82_34
        let s_82_36: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_82_35 as isize, s_82_34);
            tracer.write_register(s_82_35 as isize, s_82_34);
        };
        // C s_82_37: const #90944u : u32
        let s_82_37: u32 = 90944;
        // D s_82_38: read-reg s_82_37:struct
        let s_82_38: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_37 as isize);
            tracer.read_register(s_82_37 as isize, value);
            value
        };
        // C s_82_39: const #90944u : u32
        let s_82_39: u32 = 90944;
        // N s_82_40: write-reg s_82_39 <= s_82_38
        let s_82_40: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_82_39 as isize, s_82_38);
            tracer.write_register(s_82_39 as isize, s_82_38);
        };
        // C s_82_41: const #90944u : u32
        let s_82_41: u32 = 90944;
        // D s_82_42: read-reg s_82_41:struct
        let s_82_42: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_41 as isize);
            tracer.read_register(s_82_41 as isize, value);
            value
        };
        // C s_82_43: const #90944u : u32
        let s_82_43: u32 = 90944;
        // N s_82_44: write-reg s_82_43 <= s_82_42
        let s_82_44: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_82_43 as isize, s_82_42);
            tracer.write_register(s_82_43 as isize, s_82_42);
        };
        // C s_82_45: const #90944u : u32
        let s_82_45: u32 = 90944;
        // D s_82_46: read-reg s_82_45:struct
        let s_82_46: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_45 as isize);
            tracer.read_register(s_82_45 as isize, value);
            value
        };
        // C s_82_47: const #90944u : u32
        let s_82_47: u32 = 90944;
        // N s_82_48: write-reg s_82_47 <= s_82_46
        let s_82_48: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_82_47 as isize, s_82_46);
            tracer.write_register(s_82_47 as isize, s_82_46);
        };
        // C s_82_49: const #10128u : u32
        let s_82_49: u32 = 10128;
        // D s_82_50: read-reg s_82_49:struct
        let s_82_50: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_49 as isize);
            tracer.read_register(s_82_49 as isize, value);
            value
        };
        // C s_82_51: const #10128u : u32
        let s_82_51: u32 = 10128;
        // N s_82_52: write-reg s_82_51 <= s_82_50
        let s_82_52: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_82_51 as isize, s_82_50);
            tracer.write_register(s_82_51 as isize, s_82_50);
        };
        // C s_82_53: const #10128u : u32
        let s_82_53: u32 = 10128;
        // D s_82_54: read-reg s_82_53:struct
        let s_82_54: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_53 as isize);
            tracer.read_register(s_82_53 as isize, value);
            value
        };
        // C s_82_55: const #10128u : u32
        let s_82_55: u32 = 10128;
        // N s_82_56: write-reg s_82_55 <= s_82_54
        let s_82_56: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_82_55 as isize, s_82_54);
            tracer.write_register(s_82_55 as isize, s_82_54);
        };
        // C s_82_57: const #14960u : u32
        let s_82_57: u32 = 14960;
        // D s_82_58: read-reg s_82_57:struct
        let s_82_58: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_57 as isize);
            tracer.read_register(s_82_57 as isize, value);
            value
        };
        // C s_82_59: const #14960u : u32
        let s_82_59: u32 = 14960;
        // N s_82_60: write-reg s_82_59 <= s_82_58
        let s_82_60: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_82_59 as isize, s_82_58);
            tracer.write_register(s_82_59 as isize, s_82_58);
        };
        // C s_82_61: const #14960u : u32
        let s_82_61: u32 = 14960;
        // D s_82_62: read-reg s_82_61:struct
        let s_82_62: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_61 as isize);
            tracer.read_register(s_82_61 as isize, value);
            value
        };
        // C s_82_63: const #14960u : u32
        let s_82_63: u32 = 14960;
        // N s_82_64: write-reg s_82_63 <= s_82_62
        let s_82_64: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_82_63 as isize, s_82_62);
            tracer.write_register(s_82_63 as isize, s_82_62);
        };
        // C s_82_65: const #20112u : u32
        let s_82_65: u32 = 20112;
        // D s_82_66: read-reg s_82_65:struct
        let s_82_66: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_65 as isize);
            tracer.read_register(s_82_65 as isize, value);
            value
        };
        // D s_82_67: call _get_ID_AA64DFR0_EL1_Type_PMUVer(s_82_66)
        let s_82_67: u8 = u_get_ID_AA64DFR0_EL1_Type_PMUVer(state, tracer, s_82_66);
        // C s_82_68: const #22800u : u32
        let s_82_68: u32 = 22800;
        // D s_82_69: read-reg s_82_68:struct
        let s_82_69: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_68 as isize);
            tracer.read_register(s_82_68 as isize, value);
            value
        };
        // C s_82_70: const #22800u : u32
        let s_82_70: u32 = 22800;
        // N s_82_71: write-reg s_82_70 <= s_82_69
        let s_82_71: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_82_70 as isize, s_82_69);
            tracer.write_register(s_82_70 as isize, s_82_69);
        };
        // C s_82_72: const #17000u : u32
        let s_82_72: u32 = 17000;
        // D s_82_73: read-reg s_82_72:struct
        let s_82_73: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_82_72 as isize);
            tracer.read_register(s_82_72 as isize, value);
            value
        };
        // C s_82_74: const #17000u : u32
        let s_82_74: u32 = 17000;
        // N s_82_75: write-reg s_82_74 <= s_82_73
        let s_82_75: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_82_74 as isize, s_82_73);
            tracer.write_register(s_82_74 as isize, s_82_73);
        };
        // C s_82_76: const #100864u : u32
        let s_82_76: u32 = 100864;
        // D s_82_77: read-reg s_82_76:struct
        let s_82_77: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_82_76 as isize);
            tracer.read_register(s_82_76 as isize, value);
            value
        };
        // C s_82_78: const #100864u : u32
        let s_82_78: u32 = 100864;
        // N s_82_79: write-reg s_82_78 <= s_82_77
        let s_82_79: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_82_78 as isize, s_82_77);
            tracer.write_register(s_82_78 as isize, s_82_77);
        };
        // C s_82_80: const #17000u : u32
        let s_82_80: u32 = 17000;
        // D s_82_81: read-reg s_82_80:struct
        let s_82_81: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_82_80 as isize);
            tracer.read_register(s_82_80 as isize, value);
            value
        };
        // C s_82_82: const #17000u : u32
        let s_82_82: u32 = 17000;
        // N s_82_83: write-reg s_82_82 <= s_82_81
        let s_82_83: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_82_82 as isize, s_82_81);
            tracer.write_register(s_82_82 as isize, s_82_81);
        };
        // C s_82_84: const #17000u : u32
        let s_82_84: u32 = 17000;
        // D s_82_85: read-reg s_82_84:struct
        let s_82_85: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_82_84 as isize);
            tracer.read_register(s_82_84 as isize, value);
            value
        };
        // C s_82_86: const #17000u : u32
        let s_82_86: u32 = 17000;
        // N s_82_87: write-reg s_82_86 <= s_82_85
        let s_82_87: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_82_86 as isize, s_82_85);
            tracer.write_register(s_82_86 as isize, s_82_85);
        };
        // C s_82_88: const #17000u : u32
        let s_82_88: u32 = 17000;
        // D s_82_89: read-reg s_82_88:struct
        let s_82_89: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_82_88 as isize);
            tracer.read_register(s_82_88 as isize, value);
            value
        };
        // C s_82_90: const #17000u : u32
        let s_82_90: u32 = 17000;
        // N s_82_91: write-reg s_82_90 <= s_82_89
        let s_82_91: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_82_90 as isize, s_82_89);
            tracer.write_register(s_82_90 as isize, s_82_89);
        };
        // C s_82_92: const #23432u : u32
        let s_82_92: u32 = 23432;
        // D s_82_93: read-reg s_82_92:struct
        let s_82_93: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_82_92 as isize);
            tracer.read_register(s_82_92 as isize, value);
            value
        };
        // C s_82_94: const #23432u : u32
        let s_82_94: u32 = 23432;
        // N s_82_95: write-reg s_82_94 <= s_82_93
        let s_82_95: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_82_94 as isize, s_82_93);
            tracer.write_register(s_82_94 as isize, s_82_93);
        };
        // C s_82_96: const #11832u : u32
        let s_82_96: u32 = 11832;
        // D s_82_97: read-reg s_82_96:struct
        let s_82_97: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_82_96 as isize);
            tracer.read_register(s_82_96 as isize, value);
            value
        };
        // C s_82_98: const #11832u : u32
        let s_82_98: u32 = 11832;
        // N s_82_99: write-reg s_82_98 <= s_82_97
        let s_82_99: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_82_98 as isize, s_82_97);
            tracer.write_register(s_82_98 as isize, s_82_97);
        };
        // C s_82_100: const #11832u : u32
        let s_82_100: u32 = 11832;
        // D s_82_101: read-reg s_82_100:struct
        let s_82_101: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_82_100 as isize);
            tracer.read_register(s_82_100 as isize, value);
            value
        };
        // C s_82_102: const #11832u : u32
        let s_82_102: u32 = 11832;
        // N s_82_103: write-reg s_82_102 <= s_82_101
        let s_82_103: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_82_102 as isize, s_82_101);
            tracer.write_register(s_82_102 as isize, s_82_101);
        };
        // C s_82_104: const #20264u : u32
        let s_82_104: u32 = 20264;
        // D s_82_105: read-reg s_82_104:struct
        let s_82_105: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_82_104 as isize);
            tracer.read_register(s_82_104 as isize, value);
            value
        };
        // C s_82_106: const #20264u : u32
        let s_82_106: u32 = 20264;
        // N s_82_107: write-reg s_82_106 <= s_82_105
        let s_82_107: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_82_106 as isize, s_82_105);
            tracer.write_register(s_82_106 as isize, s_82_105);
        };
        // C s_82_108: const #11512u : u32
        let s_82_108: u32 = 11512;
        // D s_82_109: read-reg s_82_108:struct
        let s_82_109: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_82_108 as isize);
            tracer.read_register(s_82_108 as isize, value);
            value
        };
        // C s_82_110: const #11512u : u32
        let s_82_110: u32 = 11512;
        // N s_82_111: write-reg s_82_110 <= s_82_109
        let s_82_111: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_82_110 as isize, s_82_109);
            tracer.write_register(s_82_110 as isize, s_82_109);
        };
        // C s_82_112: const #11512u : u32
        let s_82_112: u32 = 11512;
        // D s_82_113: read-reg s_82_112:struct
        let s_82_113: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_82_112 as isize);
            tracer.read_register(s_82_112 as isize, value);
            value
        };
        // C s_82_114: const #11512u : u32
        let s_82_114: u32 = 11512;
        // N s_82_115: write-reg s_82_114 <= s_82_113
        let s_82_115: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_82_114 as isize, s_82_113);
            tracer.write_register(s_82_114 as isize, s_82_113);
        };
        // C s_82_116: const #12968u : u32
        let s_82_116: u32 = 12968;
        // D s_82_117: read-reg s_82_116:struct
        let s_82_117: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_82_116 as isize);
            tracer.read_register(s_82_116 as isize, value);
            value
        };
        // C s_82_118: const #12968u : u32
        let s_82_118: u32 = 12968;
        // N s_82_119: write-reg s_82_118 <= s_82_117
        let s_82_119: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_82_118 as isize, s_82_117);
            tracer.write_register(s_82_118 as isize, s_82_117);
        };
        // C s_82_120: const #12968u : u32
        let s_82_120: u32 = 12968;
        // D s_82_121: read-reg s_82_120:struct
        let s_82_121: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_82_120 as isize);
            tracer.read_register(s_82_120 as isize, value);
            value
        };
        // C s_82_122: const #12968u : u32
        let s_82_122: u32 = 12968;
        // N s_82_123: write-reg s_82_122 <= s_82_121
        let s_82_123: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_82_122 as isize, s_82_121);
            tracer.write_register(s_82_122 as isize, s_82_121);
        };
        // C s_82_124: const #102376u : u32
        let s_82_124: u32 = 102376;
        // D s_82_125: read-reg s_82_124:struct
        let s_82_125: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_82_124 as isize);
            tracer.read_register(s_82_124 as isize, value);
            value
        };
        // C s_82_126: const #102376u : u32
        let s_82_126: u32 = 102376;
        // N s_82_127: write-reg s_82_126 <= s_82_125
        let s_82_127: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_82_126 as isize, s_82_125);
            tracer.write_register(s_82_126 as isize, s_82_125);
        };
        // C s_82_128: const #13744u : u32
        let s_82_128: u32 = 13744;
        // D s_82_129: read-reg s_82_128:struct
        let s_82_129: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_82_128 as isize);
            tracer.read_register(s_82_128 as isize, value);
            value
        };
        // C s_82_130: const #13744u : u32
        let s_82_130: u32 = 13744;
        // N s_82_131: write-reg s_82_130 <= s_82_129
        let s_82_131: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_82_130 as isize, s_82_129);
            tracer.write_register(s_82_130 as isize, s_82_129);
        };
        // C s_82_132: const #1640u : u32
        let s_82_132: u32 = 1640;
        // D s_82_133: read-reg s_82_132:struct
        let s_82_133: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_82_132 as isize);
            tracer.read_register(s_82_132 as isize, value);
            value
        };
        // C s_82_134: const #1640u : u32
        let s_82_134: u32 = 1640;
        // N s_82_135: write-reg s_82_134 <= s_82_133
        let s_82_135: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_82_134 as isize, s_82_133);
            tracer.write_register(s_82_134 as isize, s_82_133);
        };
        // C s_82_136: const #16808u : u32
        let s_82_136: u32 = 16808;
        // D s_82_137: read-reg s_82_136:struct
        let s_82_137: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_82_136 as isize);
            tracer.read_register(s_82_136 as isize, value);
            value
        };
        // C s_82_138: const #16808u : u32
        let s_82_138: u32 = 16808;
        // N s_82_139: write-reg s_82_138 <= s_82_137
        let s_82_139: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_82_138 as isize, s_82_137);
            tracer.write_register(s_82_138 as isize, s_82_137);
        };
        // C s_82_140: const #20112u : u32
        let s_82_140: u32 = 20112;
        // D s_82_141: read-reg s_82_140:struct
        let s_82_141: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_140 as isize);
            tracer.read_register(s_82_140 as isize, value);
            value
        };
        // D s_82_142: call _get_ID_AA64DFR0_EL1_Type_TraceVer(s_82_141)
        let s_82_142: u8 = u_get_ID_AA64DFR0_EL1_Type_TraceVer(state, tracer, s_82_141);
        // C s_82_143: const #22800u : u32
        let s_82_143: u32 = 22800;
        // D s_82_144: read-reg s_82_143:struct
        let s_82_144: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_143 as isize);
            tracer.read_register(s_82_143 as isize, value);
            value
        };
        // C s_82_145: const #22800u : u32
        let s_82_145: u32 = 22800;
        // N s_82_146: write-reg s_82_145 <= s_82_144
        let s_82_146: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_82_145 as isize, s_82_144);
            tracer.write_register(s_82_145 as isize, s_82_144);
        };
        // C s_82_147: const #20112u : u32
        let s_82_147: u32 = 20112;
        // D s_82_148: read-reg s_82_147:struct
        let s_82_148: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_147 as isize);
            tracer.read_register(s_82_147 as isize, value);
            value
        };
        // D s_82_149: call _get_ID_AA64DFR0_EL1_Type_BRPs(s_82_148)
        let s_82_149: u8 = u_get_ID_AA64DFR0_EL1_Type_BRPs(state, tracer, s_82_148);
        // C s_82_150: const #22800u : u32
        let s_82_150: u32 = 22800;
        // D s_82_151: read-reg s_82_150:struct
        let s_82_151: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_150 as isize);
            tracer.read_register(s_82_150 as isize, value);
            value
        };
        // C s_82_152: const #22800u : u32
        let s_82_152: u32 = 22800;
        // N s_82_153: write-reg s_82_152 <= s_82_151
        let s_82_153: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_82_152 as isize, s_82_151);
            tracer.write_register(s_82_152 as isize, s_82_151);
        };
        // C s_82_154: const #20112u : u32
        let s_82_154: u32 = 20112;
        // D s_82_155: read-reg s_82_154:struct
        let s_82_155: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_154 as isize);
            tracer.read_register(s_82_154 as isize, value);
            value
        };
        // D s_82_156: call _get_ID_AA64DFR0_EL1_Type_WRPs(s_82_155)
        let s_82_156: u8 = u_get_ID_AA64DFR0_EL1_Type_WRPs(state, tracer, s_82_155);
        // C s_82_157: const #22800u : u32
        let s_82_157: u32 = 22800;
        // D s_82_158: read-reg s_82_157:struct
        let s_82_158: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_157 as isize);
            tracer.read_register(s_82_157 as isize, value);
            value
        };
        // C s_82_159: const #22800u : u32
        let s_82_159: u32 = 22800;
        // N s_82_160: write-reg s_82_159 <= s_82_158
        let s_82_160: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_82_159 as isize, s_82_158);
            tracer.write_register(s_82_159 as isize, s_82_158);
        };
        // C s_82_161: const #20112u : u32
        let s_82_161: u32 = 20112;
        // D s_82_162: read-reg s_82_161:struct
        let s_82_162: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_161 as isize);
            tracer.read_register(s_82_161 as isize, value);
            value
        };
        // D s_82_163: call _get_ID_AA64DFR0_EL1_Type_CTX_CMPs(s_82_162)
        let s_82_163: u8 = u_get_ID_AA64DFR0_EL1_Type_CTX_CMPs(state, tracer, s_82_162);
        // C s_82_164: const #22800u : u32
        let s_82_164: u32 = 22800;
        // D s_82_165: read-reg s_82_164:struct
        let s_82_165: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_164 as isize);
            tracer.read_register(s_82_164 as isize, value);
            value
        };
        // C s_82_166: const #22800u : u32
        let s_82_166: u32 = 22800;
        // N s_82_167: write-reg s_82_166 <= s_82_165
        let s_82_167: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_82_166 as isize, s_82_165);
            tracer.write_register(s_82_166 as isize, s_82_165);
        };
        // C s_82_168: const #20112u : u32
        let s_82_168: u32 = 20112;
        // D s_82_169: read-reg s_82_168:struct
        let s_82_169: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_168 as isize);
            tracer.read_register(s_82_168 as isize, value);
            value
        };
        // D s_82_170: call _get_ID_AA64DFR0_EL1_Type_TraceFilt(s_82_169)
        let s_82_170: u8 = u_get_ID_AA64DFR0_EL1_Type_TraceFilt(state, tracer, s_82_169);
        // C s_82_171: const #22800u : u32
        let s_82_171: u32 = 22800;
        // D s_82_172: read-reg s_82_171:struct
        let s_82_172: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_171 as isize);
            tracer.read_register(s_82_171 as isize, value);
            value
        };
        // C s_82_173: const #22800u : u32
        let s_82_173: u32 = 22800;
        // N s_82_174: write-reg s_82_173 <= s_82_172
        let s_82_174: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_82_173 as isize, s_82_172);
            tracer.write_register(s_82_173 as isize, s_82_172);
        };
        // C s_82_175: const #14040u : u32
        let s_82_175: u32 = 14040;
        // D s_82_176: read-reg s_82_175:struct
        let s_82_176: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_82_175 as isize);
            tracer.read_register(s_82_175 as isize, value);
            value
        };
        // C s_82_177: const #14040u : u32
        let s_82_177: u32 = 14040;
        // N s_82_178: write-reg s_82_177 <= s_82_176
        let s_82_178: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_82_177 as isize, s_82_176);
            tracer.write_register(s_82_177 as isize, s_82_176);
        };
        // C s_82_179: const #101824u : u32
        let s_82_179: u32 = 101824;
        // D s_82_180: read-reg s_82_179:struct
        let s_82_180: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_179 as isize);
            tracer.read_register(s_82_179 as isize, value);
            value
        };
        // D s_82_181: call _get_ID_AA64PFR0_EL1_Type_EL0(s_82_180)
        let s_82_181: u8 = u_get_ID_AA64PFR0_EL1_Type_EL0(state, tracer, s_82_180);
        // C s_82_182: const #15424u : u32
        let s_82_182: u32 = 15424;
        // D s_82_183: read-reg s_82_182:struct
        let s_82_183: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_182 as isize);
            tracer.read_register(s_82_182 as isize, value);
            value
        };
        // C s_82_184: const #15424u : u32
        let s_82_184: u32 = 15424;
        // N s_82_185: write-reg s_82_184 <= s_82_183
        let s_82_185: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_82_184 as isize, s_82_183);
            tracer.write_register(s_82_184 as isize, s_82_183);
        };
        // C s_82_186: const #101824u : u32
        let s_82_186: u32 = 101824;
        // D s_82_187: read-reg s_82_186:struct
        let s_82_187: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_186 as isize);
            tracer.read_register(s_82_186 as isize, value);
            value
        };
        // D s_82_188: call _get_ID_AA64PFR0_EL1_Type_EL1(s_82_187)
        let s_82_188: u8 = u_get_ID_AA64PFR0_EL1_Type_EL1(state, tracer, s_82_187);
        // C s_82_189: const #15424u : u32
        let s_82_189: u32 = 15424;
        // D s_82_190: read-reg s_82_189:struct
        let s_82_190: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_189 as isize);
            tracer.read_register(s_82_189 as isize, value);
            value
        };
        // C s_82_191: const #15424u : u32
        let s_82_191: u32 = 15424;
        // N s_82_192: write-reg s_82_191 <= s_82_190
        let s_82_192: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_82_191 as isize, s_82_190);
            tracer.write_register(s_82_191 as isize, s_82_190);
        };
        // C s_82_193: const #101824u : u32
        let s_82_193: u32 = 101824;
        // D s_82_194: read-reg s_82_193:struct
        let s_82_194: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_193 as isize);
            tracer.read_register(s_82_193 as isize, value);
            value
        };
        // D s_82_195: call _get_ID_AA64PFR0_EL1_Type_EL2(s_82_194)
        let s_82_195: u8 = u_get_ID_AA64PFR0_EL1_Type_EL2(state, tracer, s_82_194);
        // C s_82_196: const #15424u : u32
        let s_82_196: u32 = 15424;
        // D s_82_197: read-reg s_82_196:struct
        let s_82_197: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_196 as isize);
            tracer.read_register(s_82_196 as isize, value);
            value
        };
        // C s_82_198: const #15424u : u32
        let s_82_198: u32 = 15424;
        // N s_82_199: write-reg s_82_198 <= s_82_197
        let s_82_199: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_82_198 as isize, s_82_197);
            tracer.write_register(s_82_198 as isize, s_82_197);
        };
        // C s_82_200: const #101824u : u32
        let s_82_200: u32 = 101824;
        // D s_82_201: read-reg s_82_200:struct
        let s_82_201: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_200 as isize);
            tracer.read_register(s_82_200 as isize, value);
            value
        };
        // D s_82_202: call _get_ID_AA64PFR0_EL1_Type_EL3(s_82_201)
        let s_82_202: u8 = u_get_ID_AA64PFR0_EL1_Type_EL3(state, tracer, s_82_201);
        // C s_82_203: const #15424u : u32
        let s_82_203: u32 = 15424;
        // D s_82_204: read-reg s_82_203:struct
        let s_82_204: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_203 as isize);
            tracer.read_register(s_82_203 as isize, value);
            value
        };
        // C s_82_205: const #15424u : u32
        let s_82_205: u32 = 15424;
        // N s_82_206: write-reg s_82_205 <= s_82_204
        let s_82_206: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_82_205 as isize, s_82_204);
            tracer.write_register(s_82_205 as isize, s_82_204);
        };
        // C s_82_207: const #101824u : u32
        let s_82_207: u32 = 101824;
        // D s_82_208: read-reg s_82_207:struct
        let s_82_208: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_207 as isize);
            tracer.read_register(s_82_207 as isize, value);
            value
        };
        // D s_82_209: call _get_ID_AA64PFR0_EL1_Type_AdvSIMD(s_82_208)
        let s_82_209: u8 = u_get_ID_AA64PFR0_EL1_Type_AdvSIMD(state, tracer, s_82_208);
        // C s_82_210: const #15424u : u32
        let s_82_210: u32 = 15424;
        // D s_82_211: read-reg s_82_210:struct
        let s_82_211: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_210 as isize);
            tracer.read_register(s_82_210 as isize, value);
            value
        };
        // C s_82_212: const #15424u : u32
        let s_82_212: u32 = 15424;
        // N s_82_213: write-reg s_82_212 <= s_82_211
        let s_82_213: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_82_212 as isize, s_82_211);
            tracer.write_register(s_82_212 as isize, s_82_211);
        };
        // C s_82_214: const #101824u : u32
        let s_82_214: u32 = 101824;
        // D s_82_215: read-reg s_82_214:struct
        let s_82_215: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_214 as isize);
            tracer.read_register(s_82_214 as isize, value);
            value
        };
        // D s_82_216: call _get_ID_AA64PFR0_EL1_Type_GIC(s_82_215)
        let s_82_216: u8 = u_get_ID_AA64PFR0_EL1_Type_GIC(state, tracer, s_82_215);
        // C s_82_217: const #15424u : u32
        let s_82_217: u32 = 15424;
        // D s_82_218: read-reg s_82_217:struct
        let s_82_218: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_217 as isize);
            tracer.read_register(s_82_217 as isize, value);
            value
        };
        // C s_82_219: const #15424u : u32
        let s_82_219: u32 = 15424;
        // N s_82_220: write-reg s_82_219 <= s_82_218
        let s_82_220: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_82_219 as isize, s_82_218);
            tracer.write_register(s_82_219 as isize, s_82_218);
        };
        // C s_82_221: const #102296u : u32
        let s_82_221: u32 = 102296;
        // D s_82_222: read-reg s_82_221:struct
        let s_82_222: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_82_221 as isize);
            tracer.read_register(s_82_221 as isize, value);
            value
        };
        // C s_82_223: const #102296u : u32
        let s_82_223: u32 = 102296;
        // N s_82_224: write-reg s_82_223 <= s_82_222
        let s_82_224: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_82_223 as isize, s_82_222);
            tracer.write_register(s_82_223 as isize, s_82_222);
        };
        // C s_82_225: const #102296u : u32
        let s_82_225: u32 = 102296;
        // D s_82_226: read-reg s_82_225:struct
        let s_82_226: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_82_225 as isize);
            tracer.read_register(s_82_225 as isize, value);
            value
        };
        // C s_82_227: const #102296u : u32
        let s_82_227: u32 = 102296;
        // N s_82_228: write-reg s_82_227 <= s_82_226
        let s_82_228: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_82_227 as isize, s_82_226);
            tracer.write_register(s_82_227 as isize, s_82_226);
        };
        // C s_82_229: const #102296u : u32
        let s_82_229: u32 = 102296;
        // D s_82_230: read-reg s_82_229:struct
        let s_82_230: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_82_229 as isize);
            tracer.read_register(s_82_229 as isize, value);
            value
        };
        // C s_82_231: const #102296u : u32
        let s_82_231: u32 = 102296;
        // N s_82_232: write-reg s_82_231 <= s_82_230
        let s_82_232: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_82_231 as isize, s_82_230);
            tracer.write_register(s_82_231 as isize, s_82_230);
        };
        // C s_82_233: const #101032u : u32
        let s_82_233: u32 = 101032;
        // D s_82_234: read-reg s_82_233:struct
        let s_82_234: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_82_233 as isize);
            tracer.read_register(s_82_233 as isize, value);
            value
        };
        // C s_82_235: const #101032u : u32
        let s_82_235: u32 = 101032;
        // N s_82_236: write-reg s_82_235 <= s_82_234
        let s_82_236: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_82_235 as isize, s_82_234);
            tracer.write_register(s_82_235 as isize, s_82_234);
        };
        // C s_82_237: const #101032u : u32
        let s_82_237: u32 = 101032;
        // D s_82_238: read-reg s_82_237:struct
        let s_82_238: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_82_237 as isize);
            tracer.read_register(s_82_237 as isize, value);
            value
        };
        // C s_82_239: const #101032u : u32
        let s_82_239: u32 = 101032;
        // N s_82_240: write-reg s_82_239 <= s_82_238
        let s_82_240: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_82_239 as isize, s_82_238);
            tracer.write_register(s_82_239 as isize, s_82_238);
        };
        // C s_82_241: const #101032u : u32
        let s_82_241: u32 = 101032;
        // D s_82_242: read-reg s_82_241:struct
        let s_82_242: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_82_241 as isize);
            tracer.read_register(s_82_241 as isize, value);
            value
        };
        // C s_82_243: const #101032u : u32
        let s_82_243: u32 = 101032;
        // N s_82_244: write-reg s_82_243 <= s_82_242
        let s_82_244: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_82_243 as isize, s_82_242);
            tracer.write_register(s_82_243 as isize, s_82_242);
        };
        // C s_82_245: const #13736u : u32
        let s_82_245: u32 = 13736;
        // D s_82_246: read-reg s_82_245:struct
        let s_82_246: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_82_245 as isize);
            tracer.read_register(s_82_245 as isize, value);
            value
        };
        // C s_82_247: const #13736u : u32
        let s_82_247: u32 = 13736;
        // N s_82_248: write-reg s_82_247 <= s_82_246
        let s_82_248: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_82_247 as isize, s_82_246);
            tracer.write_register(s_82_247 as isize, s_82_246);
        };
        // C s_82_249: const #13736u : u32
        let s_82_249: u32 = 13736;
        // D s_82_250: read-reg s_82_249:struct
        let s_82_250: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_82_249 as isize);
            tracer.read_register(s_82_249 as isize, value);
            value
        };
        // C s_82_251: const #13736u : u32
        let s_82_251: u32 = 13736;
        // N s_82_252: write-reg s_82_251 <= s_82_250
        let s_82_252: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_82_251 as isize, s_82_250);
            tracer.write_register(s_82_251 as isize, s_82_250);
        };
        // C s_82_253: const #90488u : u32
        let s_82_253: u32 = 90488;
        // D s_82_254: read-reg s_82_253:struct
        let s_82_254: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_82_253 as isize);
            tracer.read_register(s_82_253 as isize, value);
            value
        };
        // C s_82_255: const #90488u : u32
        let s_82_255: u32 = 90488;
        // N s_82_256: write-reg s_82_255 <= s_82_254
        let s_82_256: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_82_255 as isize, s_82_254);
            tracer.write_register(s_82_255 as isize, s_82_254);
        };
        // C s_82_257: const #90488u : u32
        let s_82_257: u32 = 90488;
        // D s_82_258: read-reg s_82_257:struct
        let s_82_258: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_82_257 as isize);
            tracer.read_register(s_82_257 as isize, value);
            value
        };
        // C s_82_259: const #90488u : u32
        let s_82_259: u32 = 90488;
        // N s_82_260: write-reg s_82_259 <= s_82_258
        let s_82_260: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_82_259 as isize, s_82_258);
            tracer.write_register(s_82_259 as isize, s_82_258);
        };
        // C s_82_261: const #17696u : u32
        let s_82_261: u32 = 17696;
        // D s_82_262: read-reg s_82_261:struct
        let s_82_262: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_82_261 as isize);
            tracer.read_register(s_82_261 as isize, value);
            value
        };
        // C s_82_263: const #17696u : u32
        let s_82_263: u32 = 17696;
        // N s_82_264: write-reg s_82_263 <= s_82_262
        let s_82_264: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_82_263 as isize, s_82_262);
            tracer.write_register(s_82_263 as isize, s_82_262);
        };
        // C s_82_265: const #17696u : u32
        let s_82_265: u32 = 17696;
        // D s_82_266: read-reg s_82_265:struct
        let s_82_266: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_82_265 as isize);
            tracer.read_register(s_82_265 as isize, value);
            value
        };
        // C s_82_267: const #17696u : u32
        let s_82_267: u32 = 17696;
        // N s_82_268: write-reg s_82_267 <= s_82_266
        let s_82_268: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_82_267 as isize, s_82_266);
            tracer.write_register(s_82_267 as isize, s_82_266);
        };
        // C s_82_269: const #101232u : u32
        let s_82_269: u32 = 101232;
        // D s_82_270: read-reg s_82_269:struct
        let s_82_270: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_82_269 as isize);
            tracer.read_register(s_82_269 as isize, value);
            value
        };
        // C s_82_271: const #101232u : u32
        let s_82_271: u32 = 101232;
        // N s_82_272: write-reg s_82_271 <= s_82_270
        let s_82_272: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_82_271 as isize, s_82_270);
            tracer.write_register(s_82_271 as isize, s_82_270);
        };
        // C s_82_273: const #101232u : u32
        let s_82_273: u32 = 101232;
        // D s_82_274: read-reg s_82_273:struct
        let s_82_274: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_82_273 as isize);
            tracer.read_register(s_82_273 as isize, value);
            value
        };
        // C s_82_275: const #101232u : u32
        let s_82_275: u32 = 101232;
        // N s_82_276: write-reg s_82_275 <= s_82_274
        let s_82_276: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_82_275 as isize, s_82_274);
            tracer.write_register(s_82_275 as isize, s_82_274);
        };
        // C s_82_277: const #19048u : u32
        let s_82_277: u32 = 19048;
        // D s_82_278: read-reg s_82_277:struct
        let s_82_278: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_82_277 as isize);
            tracer.read_register(s_82_277 as isize, value);
            value
        };
        // C s_82_279: const #19048u : u32
        let s_82_279: u32 = 19048;
        // N s_82_280: write-reg s_82_279 <= s_82_278
        let s_82_280: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_82_279 as isize, s_82_278);
            tracer.write_register(s_82_279 as isize, s_82_278);
        };
        // C s_82_281: const #23216u : u32
        let s_82_281: u32 = 23216;
        // D s_82_282: read-reg s_82_281:struct
        let s_82_282: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_82_281 as isize);
            tracer.read_register(s_82_281 as isize, value);
            value
        };
        // C s_82_283: const #23216u : u32
        let s_82_283: u32 = 23216;
        // N s_82_284: write-reg s_82_283 <= s_82_282
        let s_82_284: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_82_283 as isize, s_82_282);
            tracer.write_register(s_82_283 as isize, s_82_282);
        };
        // C s_82_285: const #104944u : u32
        let s_82_285: u32 = 104944;
        // D s_82_286: read-reg s_82_285:struct
        let s_82_286: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_82_285 as isize);
            tracer.read_register(s_82_285 as isize, value);
            value
        };
        // C s_82_287: const #104944u : u32
        let s_82_287: u32 = 104944;
        // N s_82_288: write-reg s_82_287 <= s_82_286
        let s_82_288: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_82_287 as isize, s_82_286);
            tracer.write_register(s_82_287 as isize, s_82_286);
        };
        // C s_82_289: const #20016u : u32
        let s_82_289: u32 = 20016;
        // D s_82_290: read-reg s_82_289:struct
        let s_82_290: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_82_289 as isize);
            tracer.read_register(s_82_289 as isize, value);
            value
        };
        // C s_82_291: const #20016u : u32
        let s_82_291: u32 = 20016;
        // N s_82_292: write-reg s_82_291 <= s_82_290
        let s_82_292: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_82_291 as isize, s_82_290);
            tracer.write_register(s_82_291 as isize, s_82_290);
        };
        // C s_82_293: const #20016u : u32
        let s_82_293: u32 = 20016;
        // D s_82_294: read-reg s_82_293:struct
        let s_82_294: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_82_293 as isize);
            tracer.read_register(s_82_293 as isize, value);
            value
        };
        // C s_82_295: const #20016u : u32
        let s_82_295: u32 = 20016;
        // N s_82_296: write-reg s_82_295 <= s_82_294
        let s_82_296: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_82_295 as isize, s_82_294);
            tracer.write_register(s_82_295 as isize, s_82_294);
        };
        // C s_82_297: const #13680u : u32
        let s_82_297: u32 = 13680;
        // D s_82_298: read-reg s_82_297:struct
        let s_82_298: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_82_297 as isize);
            tracer.read_register(s_82_297 as isize, value);
            value
        };
        // C s_82_299: const #13680u : u32
        let s_82_299: u32 = 13680;
        // N s_82_300: write-reg s_82_299 <= s_82_298
        let s_82_300: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_82_299 as isize, s_82_298);
            tracer.write_register(s_82_299 as isize, s_82_298);
        };
        // C s_82_301: const #1s : i
        let s_82_301: i128 = 1;
        // C s_82_302: const #14328u : u32
        let s_82_302: u32 = 14328;
        // D s_82_303: read-reg s_82_302:i
        let s_82_303: i128 = {
            let value = state.read_register::<i128>(s_82_302 as isize);
            tracer.read_register(s_82_302 as isize, value);
            value
        };
        // D s_82_304: sub s_82_303 s_82_301
        let s_82_304: i128 = ((s_82_303) - (s_82_301));
        // C s_82_305: const #2s : i
        let s_82_305: i128 = 2;
        // C s_82_306: const #0s : i
        let s_82_306: i128 = 0;
        // D s_82_307: call integer_subrange(s_82_304, s_82_305, s_82_306)
        let s_82_307: Bits = integer_subrange(
            state,
            tracer,
            s_82_304,
            s_82_305,
            s_82_306,
        );
        // C s_82_308: const #17352u : u32
        let s_82_308: u32 = 17352;
        // D s_82_309: read-reg s_82_308:struct
        let s_82_309: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_308 as isize);
            tracer.read_register(s_82_308 as isize, value);
            value
        };
        // C s_82_310: const #17352u : u32
        let s_82_310: u32 = 17352;
        // N s_82_311: write-reg s_82_310 <= s_82_309
        let s_82_311: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_82_310 as isize, s_82_309);
            tracer.write_register(s_82_310 as isize, s_82_309);
        };
        // C s_82_312: const #1s : i
        let s_82_312: i128 = 1;
        // C s_82_313: const #17568u : u32
        let s_82_313: u32 = 17568;
        // D s_82_314: read-reg s_82_313:i
        let s_82_314: i128 = {
            let value = state.read_register::<i128>(s_82_313 as isize);
            tracer.read_register(s_82_313 as isize, value);
            value
        };
        // D s_82_315: sub s_82_314 s_82_312
        let s_82_315: i128 = ((s_82_314) - (s_82_312));
        // C s_82_316: const #4s : i
        let s_82_316: i128 = 4;
        // C s_82_317: const #0s : i
        let s_82_317: i128 = 0;
        // D s_82_318: call integer_subrange(s_82_315, s_82_316, s_82_317)
        let s_82_318: Bits = integer_subrange(
            state,
            tracer,
            s_82_315,
            s_82_316,
            s_82_317,
        );
        // C s_82_319: const #17352u : u32
        let s_82_319: u32 = 17352;
        // D s_82_320: read-reg s_82_319:struct
        let s_82_320: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_319 as isize);
            tracer.read_register(s_82_319 as isize, value);
            value
        };
        // C s_82_321: const #17352u : u32
        let s_82_321: u32 = 17352;
        // N s_82_322: write-reg s_82_321 <= s_82_320
        let s_82_322: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_82_321 as isize, s_82_320);
            tracer.write_register(s_82_321 as isize, s_82_320);
        };
        // C s_82_323: const #1s : i
        let s_82_323: i128 = 1;
        // C s_82_324: const #102176u : u32
        let s_82_324: u32 = 102176;
        // D s_82_325: read-reg s_82_324:i
        let s_82_325: i128 = {
            let value = state.read_register::<i128>(s_82_324 as isize);
            tracer.read_register(s_82_324 as isize, value);
            value
        };
        // D s_82_326: sub s_82_325 s_82_323
        let s_82_326: i128 = ((s_82_325) - (s_82_323));
        // C s_82_327: const #2s : i
        let s_82_327: i128 = 2;
        // C s_82_328: const #0s : i
        let s_82_328: i128 = 0;
        // D s_82_329: call integer_subrange(s_82_326, s_82_327, s_82_328)
        let s_82_329: Bits = integer_subrange(
            state,
            tracer,
            s_82_326,
            s_82_327,
            s_82_328,
        );
        // C s_82_330: const #17352u : u32
        let s_82_330: u32 = 17352;
        // D s_82_331: read-reg s_82_330:struct
        let s_82_331: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_330 as isize);
            tracer.read_register(s_82_330 as isize, value);
            value
        };
        // C s_82_332: const #17352u : u32
        let s_82_332: u32 = 17352;
        // N s_82_333: write-reg s_82_332 <= s_82_331
        let s_82_333: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_82_332 as isize, s_82_331);
            tracer.write_register(s_82_332 as isize, s_82_331);
        };
        // C s_82_334: const #0s : i
        let s_82_334: i128 = 0;
        // S s_82_335: call IsG1ActivityMonitorImplemented(s_82_334)
        let s_82_335: bool = IsG1ActivityMonitorImplemented(state, tracer, s_82_334);
        // N s_82_336: branch s_82_335 b219 b83
        if s_82_335 {
            return block_219(state, tracer, fn_state);
        } else {
            return block_83(state, tracer, fn_state);
        };
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_83_0: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #104848u : u32
        let s_84_0: u32 = 104848;
        // D s_84_1: read-reg s_84_0:struct
        let s_84_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_84_0 as isize);
            tracer.read_register(s_84_0 as isize, value);
            value
        };
        // C s_84_2: const #104848u : u32
        let s_84_2: u32 = 104848;
        // N s_84_3: write-reg s_84_2 <= s_84_1
        let s_84_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_84_2 as isize, s_84_1);
            tracer.write_register(s_84_2 as isize, s_84_1);
        };
        // C s_84_4: const #1s : i
        let s_84_4: i128 = 1;
        // S s_84_5: call IsG1ActivityMonitorImplemented(s_84_4)
        let s_84_5: bool = IsG1ActivityMonitorImplemented(state, tracer, s_84_4);
        // N s_84_6: branch s_84_5 b218 b85
        if s_84_5 {
            return block_218(state, tracer, fn_state);
        } else {
            return block_85(state, tracer, fn_state);
        };
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_85_0: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #104848u : u32
        let s_86_0: u32 = 104848;
        // D s_86_1: read-reg s_86_0:struct
        let s_86_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_86_0 as isize);
            tracer.read_register(s_86_0 as isize, value);
            value
        };
        // C s_86_2: const #104848u : u32
        let s_86_2: u32 = 104848;
        // N s_86_3: write-reg s_86_2 <= s_86_1
        let s_86_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_86_2 as isize, s_86_1);
            tracer.write_register(s_86_2 as isize, s_86_1);
        };
        // C s_86_4: const #2s : i
        let s_86_4: i128 = 2;
        // S s_86_5: call IsG1ActivityMonitorImplemented(s_86_4)
        let s_86_5: bool = IsG1ActivityMonitorImplemented(state, tracer, s_86_4);
        // N s_86_6: branch s_86_5 b217 b87
        if s_86_5 {
            return block_217(state, tracer, fn_state);
        } else {
            return block_87(state, tracer, fn_state);
        };
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_87_0: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #104848u : u32
        let s_88_0: u32 = 104848;
        // D s_88_1: read-reg s_88_0:struct
        let s_88_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_88_0 as isize);
            tracer.read_register(s_88_0 as isize, value);
            value
        };
        // C s_88_2: const #104848u : u32
        let s_88_2: u32 = 104848;
        // N s_88_3: write-reg s_88_2 <= s_88_1
        let s_88_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_88_2 as isize, s_88_1);
            tracer.write_register(s_88_2 as isize, s_88_1);
        };
        // C s_88_4: const #3s : i
        let s_88_4: i128 = 3;
        // S s_88_5: call IsG1ActivityMonitorImplemented(s_88_4)
        let s_88_5: bool = IsG1ActivityMonitorImplemented(state, tracer, s_88_4);
        // N s_88_6: branch s_88_5 b216 b89
        if s_88_5 {
            return block_216(state, tracer, fn_state);
        } else {
            return block_89(state, tracer, fn_state);
        };
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_89_0: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #104848u : u32
        let s_90_0: u32 = 104848;
        // D s_90_1: read-reg s_90_0:struct
        let s_90_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_90_0 as isize);
            tracer.read_register(s_90_0 as isize, value);
            value
        };
        // C s_90_2: const #104848u : u32
        let s_90_2: u32 = 104848;
        // N s_90_3: write-reg s_90_2 <= s_90_1
        let s_90_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_90_2 as isize, s_90_1);
            tracer.write_register(s_90_2 as isize, s_90_1);
        };
        // C s_90_4: const #4s : i
        let s_90_4: i128 = 4;
        // S s_90_5: call IsG1ActivityMonitorImplemented(s_90_4)
        let s_90_5: bool = IsG1ActivityMonitorImplemented(state, tracer, s_90_4);
        // N s_90_6: branch s_90_5 b215 b91
        if s_90_5 {
            return block_215(state, tracer, fn_state);
        } else {
            return block_91(state, tracer, fn_state);
        };
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_91_0: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #104848u : u32
        let s_92_0: u32 = 104848;
        // D s_92_1: read-reg s_92_0:struct
        let s_92_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_92_0 as isize);
            tracer.read_register(s_92_0 as isize, value);
            value
        };
        // C s_92_2: const #104848u : u32
        let s_92_2: u32 = 104848;
        // N s_92_3: write-reg s_92_2 <= s_92_1
        let s_92_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_92_2 as isize, s_92_1);
            tracer.write_register(s_92_2 as isize, s_92_1);
        };
        // C s_92_4: const #5s : i
        let s_92_4: i128 = 5;
        // S s_92_5: call IsG1ActivityMonitorImplemented(s_92_4)
        let s_92_5: bool = IsG1ActivityMonitorImplemented(state, tracer, s_92_4);
        // N s_92_6: branch s_92_5 b214 b93
        if s_92_5 {
            return block_214(state, tracer, fn_state);
        } else {
            return block_93(state, tracer, fn_state);
        };
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_93_0: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #104848u : u32
        let s_94_0: u32 = 104848;
        // D s_94_1: read-reg s_94_0:struct
        let s_94_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_94_0 as isize);
            tracer.read_register(s_94_0 as isize, value);
            value
        };
        // C s_94_2: const #104848u : u32
        let s_94_2: u32 = 104848;
        // N s_94_3: write-reg s_94_2 <= s_94_1
        let s_94_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_94_2 as isize, s_94_1);
            tracer.write_register(s_94_2 as isize, s_94_1);
        };
        // C s_94_4: const #6s : i
        let s_94_4: i128 = 6;
        // S s_94_5: call IsG1ActivityMonitorImplemented(s_94_4)
        let s_94_5: bool = IsG1ActivityMonitorImplemented(state, tracer, s_94_4);
        // N s_94_6: branch s_94_5 b213 b95
        if s_94_5 {
            return block_213(state, tracer, fn_state);
        } else {
            return block_95(state, tracer, fn_state);
        };
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_95_0: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_96_0: const #104848u : u32
        let s_96_0: u32 = 104848;
        // D s_96_1: read-reg s_96_0:struct
        let s_96_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_96_0 as isize);
            tracer.read_register(s_96_0 as isize, value);
            value
        };
        // C s_96_2: const #104848u : u32
        let s_96_2: u32 = 104848;
        // N s_96_3: write-reg s_96_2 <= s_96_1
        let s_96_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_96_2 as isize, s_96_1);
            tracer.write_register(s_96_2 as isize, s_96_1);
        };
        // C s_96_4: const #7s : i
        let s_96_4: i128 = 7;
        // S s_96_5: call IsG1ActivityMonitorImplemented(s_96_4)
        let s_96_5: bool = IsG1ActivityMonitorImplemented(state, tracer, s_96_4);
        // N s_96_6: branch s_96_5 b212 b97
        if s_96_5 {
            return block_212(state, tracer, fn_state);
        } else {
            return block_97(state, tracer, fn_state);
        };
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_97_0: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #104848u : u32
        let s_98_0: u32 = 104848;
        // D s_98_1: read-reg s_98_0:struct
        let s_98_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_98_0 as isize);
            tracer.read_register(s_98_0 as isize, value);
            value
        };
        // C s_98_2: const #104848u : u32
        let s_98_2: u32 = 104848;
        // N s_98_3: write-reg s_98_2 <= s_98_1
        let s_98_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_98_2 as isize, s_98_1);
            tracer.write_register(s_98_2 as isize, s_98_1);
        };
        // C s_98_4: const #8s : i
        let s_98_4: i128 = 8;
        // S s_98_5: call IsG1ActivityMonitorImplemented(s_98_4)
        let s_98_5: bool = IsG1ActivityMonitorImplemented(state, tracer, s_98_4);
        // N s_98_6: branch s_98_5 b211 b99
        if s_98_5 {
            return block_211(state, tracer, fn_state);
        } else {
            return block_99(state, tracer, fn_state);
        };
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_99_0: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #104848u : u32
        let s_100_0: u32 = 104848;
        // D s_100_1: read-reg s_100_0:struct
        let s_100_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_100_0 as isize);
            tracer.read_register(s_100_0 as isize, value);
            value
        };
        // C s_100_2: const #104848u : u32
        let s_100_2: u32 = 104848;
        // N s_100_3: write-reg s_100_2 <= s_100_1
        let s_100_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_100_2 as isize, s_100_1);
            tracer.write_register(s_100_2 as isize, s_100_1);
        };
        // C s_100_4: const #9s : i
        let s_100_4: i128 = 9;
        // S s_100_5: call IsG1ActivityMonitorImplemented(s_100_4)
        let s_100_5: bool = IsG1ActivityMonitorImplemented(state, tracer, s_100_4);
        // N s_100_6: branch s_100_5 b210 b101
        if s_100_5 {
            return block_210(state, tracer, fn_state);
        } else {
            return block_101(state, tracer, fn_state);
        };
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_101_0: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_102_0: const #104848u : u32
        let s_102_0: u32 = 104848;
        // D s_102_1: read-reg s_102_0:struct
        let s_102_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_102_0 as isize);
            tracer.read_register(s_102_0 as isize, value);
            value
        };
        // C s_102_2: const #104848u : u32
        let s_102_2: u32 = 104848;
        // N s_102_3: write-reg s_102_2 <= s_102_1
        let s_102_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_102_2 as isize, s_102_1);
            tracer.write_register(s_102_2 as isize, s_102_1);
        };
        // C s_102_4: const #10s : i
        let s_102_4: i128 = 10;
        // S s_102_5: call IsG1ActivityMonitorImplemented(s_102_4)
        let s_102_5: bool = IsG1ActivityMonitorImplemented(state, tracer, s_102_4);
        // N s_102_6: branch s_102_5 b209 b103
        if s_102_5 {
            return block_209(state, tracer, fn_state);
        } else {
            return block_103(state, tracer, fn_state);
        };
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_103_0: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_104_0: const #104848u : u32
        let s_104_0: u32 = 104848;
        // D s_104_1: read-reg s_104_0:struct
        let s_104_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_104_0 as isize);
            tracer.read_register(s_104_0 as isize, value);
            value
        };
        // C s_104_2: const #104848u : u32
        let s_104_2: u32 = 104848;
        // N s_104_3: write-reg s_104_2 <= s_104_1
        let s_104_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_104_2 as isize, s_104_1);
            tracer.write_register(s_104_2 as isize, s_104_1);
        };
        // C s_104_4: const #11s : i
        let s_104_4: i128 = 11;
        // S s_104_5: call IsG1ActivityMonitorImplemented(s_104_4)
        let s_104_5: bool = IsG1ActivityMonitorImplemented(state, tracer, s_104_4);
        // N s_104_6: branch s_104_5 b208 b105
        if s_104_5 {
            return block_208(state, tracer, fn_state);
        } else {
            return block_105(state, tracer, fn_state);
        };
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_105_0: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #104848u : u32
        let s_106_0: u32 = 104848;
        // D s_106_1: read-reg s_106_0:struct
        let s_106_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_106_0 as isize);
            tracer.read_register(s_106_0 as isize, value);
            value
        };
        // C s_106_2: const #104848u : u32
        let s_106_2: u32 = 104848;
        // N s_106_3: write-reg s_106_2 <= s_106_1
        let s_106_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_106_2 as isize, s_106_1);
            tracer.write_register(s_106_2 as isize, s_106_1);
        };
        // C s_106_4: const #12s : i
        let s_106_4: i128 = 12;
        // S s_106_5: call IsG1ActivityMonitorImplemented(s_106_4)
        let s_106_5: bool = IsG1ActivityMonitorImplemented(state, tracer, s_106_4);
        // N s_106_6: branch s_106_5 b207 b107
        if s_106_5 {
            return block_207(state, tracer, fn_state);
        } else {
            return block_107(state, tracer, fn_state);
        };
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_107_0: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_108_0: const #104848u : u32
        let s_108_0: u32 = 104848;
        // D s_108_1: read-reg s_108_0:struct
        let s_108_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_108_0 as isize);
            tracer.read_register(s_108_0 as isize, value);
            value
        };
        // C s_108_2: const #104848u : u32
        let s_108_2: u32 = 104848;
        // N s_108_3: write-reg s_108_2 <= s_108_1
        let s_108_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_108_2 as isize, s_108_1);
            tracer.write_register(s_108_2 as isize, s_108_1);
        };
        // C s_108_4: const #13s : i
        let s_108_4: i128 = 13;
        // S s_108_5: call IsG1ActivityMonitorImplemented(s_108_4)
        let s_108_5: bool = IsG1ActivityMonitorImplemented(state, tracer, s_108_4);
        // N s_108_6: branch s_108_5 b206 b109
        if s_108_5 {
            return block_206(state, tracer, fn_state);
        } else {
            return block_109(state, tracer, fn_state);
        };
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_109_0: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_110_0: const #104848u : u32
        let s_110_0: u32 = 104848;
        // D s_110_1: read-reg s_110_0:struct
        let s_110_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_110_0 as isize);
            tracer.read_register(s_110_0 as isize, value);
            value
        };
        // C s_110_2: const #104848u : u32
        let s_110_2: u32 = 104848;
        // N s_110_3: write-reg s_110_2 <= s_110_1
        let s_110_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_110_2 as isize, s_110_1);
            tracer.write_register(s_110_2 as isize, s_110_1);
        };
        // C s_110_4: const #14s : i
        let s_110_4: i128 = 14;
        // S s_110_5: call IsG1ActivityMonitorImplemented(s_110_4)
        let s_110_5: bool = IsG1ActivityMonitorImplemented(state, tracer, s_110_4);
        // N s_110_6: branch s_110_5 b205 b111
        if s_110_5 {
            return block_205(state, tracer, fn_state);
        } else {
            return block_111(state, tracer, fn_state);
        };
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_111_0: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_112_0: const #104848u : u32
        let s_112_0: u32 = 104848;
        // D s_112_1: read-reg s_112_0:struct
        let s_112_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_112_0 as isize);
            tracer.read_register(s_112_0 as isize, value);
            value
        };
        // C s_112_2: const #104848u : u32
        let s_112_2: u32 = 104848;
        // N s_112_3: write-reg s_112_2 <= s_112_1
        let s_112_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_112_2 as isize, s_112_1);
            tracer.write_register(s_112_2 as isize, s_112_1);
        };
        // C s_112_4: const #15s : i
        let s_112_4: i128 = 15;
        // S s_112_5: call IsG1ActivityMonitorImplemented(s_112_4)
        let s_112_5: bool = IsG1ActivityMonitorImplemented(state, tracer, s_112_4);
        // N s_112_6: branch s_112_5 b204 b113
        if s_112_5 {
            return block_204(state, tracer, fn_state);
        } else {
            return block_113(state, tracer, fn_state);
        };
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_113_0: jump b114
        return block_114(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #104848u : u32
        let s_114_0: u32 = 104848;
        // D s_114_1: read-reg s_114_0:struct
        let s_114_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_114_0 as isize);
            tracer.read_register(s_114_0 as isize, value);
            value
        };
        // C s_114_2: const #104848u : u32
        let s_114_2: u32 = 104848;
        // N s_114_3: write-reg s_114_2 <= s_114_1
        let s_114_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_114_2 as isize, s_114_1);
            tracer.write_register(s_114_2 as isize, s_114_1);
        };
        // C s_114_4: const #0s : i
        let s_114_4: i128 = 0;
        // S s_114_5: call IsG1ActivityMonitorOffsetImplemented(s_114_4)
        let s_114_5: bool = IsG1ActivityMonitorOffsetImplemented(state, tracer, s_114_4);
        // N s_114_6: branch s_114_5 b203 b115
        if s_114_5 {
            return block_203(state, tracer, fn_state);
        } else {
            return block_115(state, tracer, fn_state);
        };
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_115_0: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_116_0: const #104848u : u32
        let s_116_0: u32 = 104848;
        // D s_116_1: read-reg s_116_0:struct
        let s_116_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_116_0 as isize);
            tracer.read_register(s_116_0 as isize, value);
            value
        };
        // C s_116_2: const #104848u : u32
        let s_116_2: u32 = 104848;
        // N s_116_3: write-reg s_116_2 <= s_116_1
        let s_116_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_116_2 as isize, s_116_1);
            tracer.write_register(s_116_2 as isize, s_116_1);
        };
        // C s_116_4: const #1s : i
        let s_116_4: i128 = 1;
        // S s_116_5: call IsG1ActivityMonitorOffsetImplemented(s_116_4)
        let s_116_5: bool = IsG1ActivityMonitorOffsetImplemented(state, tracer, s_116_4);
        // N s_116_6: branch s_116_5 b202 b117
        if s_116_5 {
            return block_202(state, tracer, fn_state);
        } else {
            return block_117(state, tracer, fn_state);
        };
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_117_0: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #104848u : u32
        let s_118_0: u32 = 104848;
        // D s_118_1: read-reg s_118_0:struct
        let s_118_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_118_0 as isize);
            tracer.read_register(s_118_0 as isize, value);
            value
        };
        // C s_118_2: const #104848u : u32
        let s_118_2: u32 = 104848;
        // N s_118_3: write-reg s_118_2 <= s_118_1
        let s_118_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_118_2 as isize, s_118_1);
            tracer.write_register(s_118_2 as isize, s_118_1);
        };
        // C s_118_4: const #2s : i
        let s_118_4: i128 = 2;
        // S s_118_5: call IsG1ActivityMonitorOffsetImplemented(s_118_4)
        let s_118_5: bool = IsG1ActivityMonitorOffsetImplemented(state, tracer, s_118_4);
        // N s_118_6: branch s_118_5 b201 b119
        if s_118_5 {
            return block_201(state, tracer, fn_state);
        } else {
            return block_119(state, tracer, fn_state);
        };
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_119_0: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_120_0: const #104848u : u32
        let s_120_0: u32 = 104848;
        // D s_120_1: read-reg s_120_0:struct
        let s_120_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_120_0 as isize);
            tracer.read_register(s_120_0 as isize, value);
            value
        };
        // C s_120_2: const #104848u : u32
        let s_120_2: u32 = 104848;
        // N s_120_3: write-reg s_120_2 <= s_120_1
        let s_120_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_120_2 as isize, s_120_1);
            tracer.write_register(s_120_2 as isize, s_120_1);
        };
        // C s_120_4: const #3s : i
        let s_120_4: i128 = 3;
        // S s_120_5: call IsG1ActivityMonitorOffsetImplemented(s_120_4)
        let s_120_5: bool = IsG1ActivityMonitorOffsetImplemented(state, tracer, s_120_4);
        // N s_120_6: branch s_120_5 b200 b121
        if s_120_5 {
            return block_200(state, tracer, fn_state);
        } else {
            return block_121(state, tracer, fn_state);
        };
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_121_0: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_122_0: const #104848u : u32
        let s_122_0: u32 = 104848;
        // D s_122_1: read-reg s_122_0:struct
        let s_122_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_122_0 as isize);
            tracer.read_register(s_122_0 as isize, value);
            value
        };
        // C s_122_2: const #104848u : u32
        let s_122_2: u32 = 104848;
        // N s_122_3: write-reg s_122_2 <= s_122_1
        let s_122_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_122_2 as isize, s_122_1);
            tracer.write_register(s_122_2 as isize, s_122_1);
        };
        // C s_122_4: const #4s : i
        let s_122_4: i128 = 4;
        // S s_122_5: call IsG1ActivityMonitorOffsetImplemented(s_122_4)
        let s_122_5: bool = IsG1ActivityMonitorOffsetImplemented(state, tracer, s_122_4);
        // N s_122_6: branch s_122_5 b199 b123
        if s_122_5 {
            return block_199(state, tracer, fn_state);
        } else {
            return block_123(state, tracer, fn_state);
        };
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_123_0: jump b124
        return block_124(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_124_0: const #104848u : u32
        let s_124_0: u32 = 104848;
        // D s_124_1: read-reg s_124_0:struct
        let s_124_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_124_0 as isize);
            tracer.read_register(s_124_0 as isize, value);
            value
        };
        // C s_124_2: const #104848u : u32
        let s_124_2: u32 = 104848;
        // N s_124_3: write-reg s_124_2 <= s_124_1
        let s_124_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_124_2 as isize, s_124_1);
            tracer.write_register(s_124_2 as isize, s_124_1);
        };
        // C s_124_4: const #5s : i
        let s_124_4: i128 = 5;
        // S s_124_5: call IsG1ActivityMonitorOffsetImplemented(s_124_4)
        let s_124_5: bool = IsG1ActivityMonitorOffsetImplemented(state, tracer, s_124_4);
        // N s_124_6: branch s_124_5 b198 b125
        if s_124_5 {
            return block_198(state, tracer, fn_state);
        } else {
            return block_125(state, tracer, fn_state);
        };
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_125_0: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_126_0: const #104848u : u32
        let s_126_0: u32 = 104848;
        // D s_126_1: read-reg s_126_0:struct
        let s_126_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_126_0 as isize);
            tracer.read_register(s_126_0 as isize, value);
            value
        };
        // C s_126_2: const #104848u : u32
        let s_126_2: u32 = 104848;
        // N s_126_3: write-reg s_126_2 <= s_126_1
        let s_126_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_126_2 as isize, s_126_1);
            tracer.write_register(s_126_2 as isize, s_126_1);
        };
        // C s_126_4: const #6s : i
        let s_126_4: i128 = 6;
        // S s_126_5: call IsG1ActivityMonitorOffsetImplemented(s_126_4)
        let s_126_5: bool = IsG1ActivityMonitorOffsetImplemented(state, tracer, s_126_4);
        // N s_126_6: branch s_126_5 b197 b127
        if s_126_5 {
            return block_197(state, tracer, fn_state);
        } else {
            return block_127(state, tracer, fn_state);
        };
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_127_0: jump b128
        return block_128(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_128_0: const #104848u : u32
        let s_128_0: u32 = 104848;
        // D s_128_1: read-reg s_128_0:struct
        let s_128_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_128_0 as isize);
            tracer.read_register(s_128_0 as isize, value);
            value
        };
        // C s_128_2: const #104848u : u32
        let s_128_2: u32 = 104848;
        // N s_128_3: write-reg s_128_2 <= s_128_1
        let s_128_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_128_2 as isize, s_128_1);
            tracer.write_register(s_128_2 as isize, s_128_1);
        };
        // C s_128_4: const #7s : i
        let s_128_4: i128 = 7;
        // S s_128_5: call IsG1ActivityMonitorOffsetImplemented(s_128_4)
        let s_128_5: bool = IsG1ActivityMonitorOffsetImplemented(state, tracer, s_128_4);
        // N s_128_6: branch s_128_5 b196 b129
        if s_128_5 {
            return block_196(state, tracer, fn_state);
        } else {
            return block_129(state, tracer, fn_state);
        };
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_129_0: jump b130
        return block_130(state, tracer, fn_state);
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_130_0: const #104848u : u32
        let s_130_0: u32 = 104848;
        // D s_130_1: read-reg s_130_0:struct
        let s_130_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_130_0 as isize);
            tracer.read_register(s_130_0 as isize, value);
            value
        };
        // C s_130_2: const #104848u : u32
        let s_130_2: u32 = 104848;
        // N s_130_3: write-reg s_130_2 <= s_130_1
        let s_130_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_130_2 as isize, s_130_1);
            tracer.write_register(s_130_2 as isize, s_130_1);
        };
        // C s_130_4: const #8s : i
        let s_130_4: i128 = 8;
        // S s_130_5: call IsG1ActivityMonitorOffsetImplemented(s_130_4)
        let s_130_5: bool = IsG1ActivityMonitorOffsetImplemented(state, tracer, s_130_4);
        // N s_130_6: branch s_130_5 b195 b131
        if s_130_5 {
            return block_195(state, tracer, fn_state);
        } else {
            return block_131(state, tracer, fn_state);
        };
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_131_0: jump b132
        return block_132(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_132_0: const #104848u : u32
        let s_132_0: u32 = 104848;
        // D s_132_1: read-reg s_132_0:struct
        let s_132_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_132_0 as isize);
            tracer.read_register(s_132_0 as isize, value);
            value
        };
        // C s_132_2: const #104848u : u32
        let s_132_2: u32 = 104848;
        // N s_132_3: write-reg s_132_2 <= s_132_1
        let s_132_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_132_2 as isize, s_132_1);
            tracer.write_register(s_132_2 as isize, s_132_1);
        };
        // C s_132_4: const #9s : i
        let s_132_4: i128 = 9;
        // S s_132_5: call IsG1ActivityMonitorOffsetImplemented(s_132_4)
        let s_132_5: bool = IsG1ActivityMonitorOffsetImplemented(state, tracer, s_132_4);
        // N s_132_6: branch s_132_5 b194 b133
        if s_132_5 {
            return block_194(state, tracer, fn_state);
        } else {
            return block_133(state, tracer, fn_state);
        };
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_133_0: jump b134
        return block_134(state, tracer, fn_state);
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_134_0: const #104848u : u32
        let s_134_0: u32 = 104848;
        // D s_134_1: read-reg s_134_0:struct
        let s_134_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_134_0 as isize);
            tracer.read_register(s_134_0 as isize, value);
            value
        };
        // C s_134_2: const #104848u : u32
        let s_134_2: u32 = 104848;
        // N s_134_3: write-reg s_134_2 <= s_134_1
        let s_134_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_134_2 as isize, s_134_1);
            tracer.write_register(s_134_2 as isize, s_134_1);
        };
        // C s_134_4: const #10s : i
        let s_134_4: i128 = 10;
        // S s_134_5: call IsG1ActivityMonitorOffsetImplemented(s_134_4)
        let s_134_5: bool = IsG1ActivityMonitorOffsetImplemented(state, tracer, s_134_4);
        // N s_134_6: branch s_134_5 b193 b135
        if s_134_5 {
            return block_193(state, tracer, fn_state);
        } else {
            return block_135(state, tracer, fn_state);
        };
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_135_0: jump b136
        return block_136(state, tracer, fn_state);
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_136_0: const #104848u : u32
        let s_136_0: u32 = 104848;
        // D s_136_1: read-reg s_136_0:struct
        let s_136_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_136_0 as isize);
            tracer.read_register(s_136_0 as isize, value);
            value
        };
        // C s_136_2: const #104848u : u32
        let s_136_2: u32 = 104848;
        // N s_136_3: write-reg s_136_2 <= s_136_1
        let s_136_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_136_2 as isize, s_136_1);
            tracer.write_register(s_136_2 as isize, s_136_1);
        };
        // C s_136_4: const #11s : i
        let s_136_4: i128 = 11;
        // S s_136_5: call IsG1ActivityMonitorOffsetImplemented(s_136_4)
        let s_136_5: bool = IsG1ActivityMonitorOffsetImplemented(state, tracer, s_136_4);
        // N s_136_6: branch s_136_5 b192 b137
        if s_136_5 {
            return block_192(state, tracer, fn_state);
        } else {
            return block_137(state, tracer, fn_state);
        };
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_137_0: jump b138
        return block_138(state, tracer, fn_state);
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_138_0: const #104848u : u32
        let s_138_0: u32 = 104848;
        // D s_138_1: read-reg s_138_0:struct
        let s_138_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_138_0 as isize);
            tracer.read_register(s_138_0 as isize, value);
            value
        };
        // C s_138_2: const #104848u : u32
        let s_138_2: u32 = 104848;
        // N s_138_3: write-reg s_138_2 <= s_138_1
        let s_138_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_138_2 as isize, s_138_1);
            tracer.write_register(s_138_2 as isize, s_138_1);
        };
        // C s_138_4: const #12s : i
        let s_138_4: i128 = 12;
        // S s_138_5: call IsG1ActivityMonitorOffsetImplemented(s_138_4)
        let s_138_5: bool = IsG1ActivityMonitorOffsetImplemented(state, tracer, s_138_4);
        // N s_138_6: branch s_138_5 b191 b139
        if s_138_5 {
            return block_191(state, tracer, fn_state);
        } else {
            return block_139(state, tracer, fn_state);
        };
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_139_0: jump b140
        return block_140(state, tracer, fn_state);
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_140_0: const #104848u : u32
        let s_140_0: u32 = 104848;
        // D s_140_1: read-reg s_140_0:struct
        let s_140_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_140_0 as isize);
            tracer.read_register(s_140_0 as isize, value);
            value
        };
        // C s_140_2: const #104848u : u32
        let s_140_2: u32 = 104848;
        // N s_140_3: write-reg s_140_2 <= s_140_1
        let s_140_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_140_2 as isize, s_140_1);
            tracer.write_register(s_140_2 as isize, s_140_1);
        };
        // C s_140_4: const #13s : i
        let s_140_4: i128 = 13;
        // S s_140_5: call IsG1ActivityMonitorOffsetImplemented(s_140_4)
        let s_140_5: bool = IsG1ActivityMonitorOffsetImplemented(state, tracer, s_140_4);
        // N s_140_6: branch s_140_5 b190 b141
        if s_140_5 {
            return block_190(state, tracer, fn_state);
        } else {
            return block_141(state, tracer, fn_state);
        };
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_141_0: jump b142
        return block_142(state, tracer, fn_state);
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_142_0: const #104848u : u32
        let s_142_0: u32 = 104848;
        // D s_142_1: read-reg s_142_0:struct
        let s_142_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_142_0 as isize);
            tracer.read_register(s_142_0 as isize, value);
            value
        };
        // C s_142_2: const #104848u : u32
        let s_142_2: u32 = 104848;
        // N s_142_3: write-reg s_142_2 <= s_142_1
        let s_142_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_142_2 as isize, s_142_1);
            tracer.write_register(s_142_2 as isize, s_142_1);
        };
        // C s_142_4: const #14s : i
        let s_142_4: i128 = 14;
        // S s_142_5: call IsG1ActivityMonitorOffsetImplemented(s_142_4)
        let s_142_5: bool = IsG1ActivityMonitorOffsetImplemented(state, tracer, s_142_4);
        // N s_142_6: branch s_142_5 b189 b143
        if s_142_5 {
            return block_189(state, tracer, fn_state);
        } else {
            return block_143(state, tracer, fn_state);
        };
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_143_0: jump b144
        return block_144(state, tracer, fn_state);
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_144_0: const #104848u : u32
        let s_144_0: u32 = 104848;
        // D s_144_1: read-reg s_144_0:struct
        let s_144_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_144_0 as isize);
            tracer.read_register(s_144_0 as isize, value);
            value
        };
        // C s_144_2: const #104848u : u32
        let s_144_2: u32 = 104848;
        // N s_144_3: write-reg s_144_2 <= s_144_1
        let s_144_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_144_2 as isize, s_144_1);
            tracer.write_register(s_144_2 as isize, s_144_1);
        };
        // C s_144_4: const #15s : i
        let s_144_4: i128 = 15;
        // S s_144_5: call IsG1ActivityMonitorOffsetImplemented(s_144_4)
        let s_144_5: bool = IsG1ActivityMonitorOffsetImplemented(state, tracer, s_144_4);
        // N s_144_6: branch s_144_5 b188 b145
        if s_144_5 {
            return block_188(state, tracer, fn_state);
        } else {
            return block_145(state, tracer, fn_state);
        };
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_145_0: jump b146
        return block_146(state, tracer, fn_state);
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_146_0: const #104848u : u32
        let s_146_0: u32 = 104848;
        // D s_146_1: read-reg s_146_0:struct
        let s_146_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_146_0 as isize);
            tracer.read_register(s_146_0 as isize, value);
            value
        };
        // C s_146_2: const #104848u : u32
        let s_146_2: u32 = 104848;
        // N s_146_3: write-reg s_146_2 <= s_146_1
        let s_146_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_146_2 as isize, s_146_1);
            tracer.write_register(s_146_2 as isize, s_146_1);
        };
        // C s_146_4: const #14u : u32
        let s_146_4: u32 = 14;
        // S s_146_5: call HasArchVersion(s_146_4)
        let s_146_5: bool = HasArchVersion(state, tracer, s_146_4);
        // N s_146_6: branch s_146_5 b148 b147
        if s_146_5 {
            return block_148(state, tracer, fn_state);
        } else {
            return block_147(state, tracer, fn_state);
        };
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_147_0: return
        return;
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_148_0: const #104800u : u32
        let s_148_0: u32 = 104800;
        // D s_148_1: read-reg s_148_0:struct
        let s_148_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_148_0 as isize);
            tracer.read_register(s_148_0 as isize, value);
            value
        };
        // C s_148_2: const #104800u : u32
        let s_148_2: u32 = 104800;
        // N s_148_3: write-reg s_148_2 <= s_148_1
        let s_148_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_148_2 as isize, s_148_1);
            tracer.write_register(s_148_2 as isize, s_148_1);
        };
        // C s_148_4: const #104800u : u32
        let s_148_4: u32 = 104800;
        // D s_148_5: read-reg s_148_4:struct
        let s_148_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_148_4 as isize);
            tracer.read_register(s_148_4 as isize, value);
            value
        };
        // C s_148_6: const #104800u : u32
        let s_148_6: u32 = 104800;
        // N s_148_7: write-reg s_148_6 <= s_148_5
        let s_148_7: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_148_6 as isize, s_148_5);
            tracer.write_register(s_148_6 as isize, s_148_5);
        };
        // C s_148_8: const #14864u : u32
        let s_148_8: u32 = 14864;
        // D s_148_9: read-reg s_148_8:struct
        let s_148_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_148_8 as isize);
            tracer.read_register(s_148_8 as isize, value);
            value
        };
        // C s_148_10: const #14864u : u32
        let s_148_10: u32 = 14864;
        // N s_148_11: write-reg s_148_10 <= s_148_9
        let s_148_11: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_148_10 as isize, s_148_9);
            tracer.write_register(s_148_10 as isize, s_148_9);
        };
        // C s_148_12: const #14864u : u32
        let s_148_12: u32 = 14864;
        // D s_148_13: read-reg s_148_12:struct
        let s_148_13: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_148_12 as isize);
            tracer.read_register(s_148_12 as isize, value);
            value
        };
        // C s_148_14: const #14864u : u32
        let s_148_14: u32 = 14864;
        // N s_148_15: write-reg s_148_14 <= s_148_13
        let s_148_15: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_148_14 as isize, s_148_13);
            tracer.write_register(s_148_14 as isize, s_148_13);
        };
        // C s_148_16: const #() : ()
        let s_148_16: () = ();
        // S s_148_17: call HaveLSE128(s_148_16)
        let s_148_17: bool = HaveLSE128(state, tracer, s_148_16);
        // N s_148_18: branch s_148_17 b187 b149
        if s_148_17 {
            return block_187(state, tracer, fn_state);
        } else {
            return block_149(state, tracer, fn_state);
        };
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_149_0: jump b150
        return block_150(state, tracer, fn_state);
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_150_0: const #() : ()
        let s_150_0: () = ();
        // S s_150_1: call HaveGCS(s_150_0)
        let s_150_1: bool = HaveGCS(state, tracer, s_150_0);
        // N s_150_2: branch s_150_1 b186 b151
        if s_150_1 {
            return block_186(state, tracer, fn_state);
        } else {
            return block_151(state, tracer, fn_state);
        };
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_151_0: jump b152
        return block_152(state, tracer, fn_state);
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_152_0: const #104664u : u32
        let s_152_0: u32 = 104664;
        // D s_152_1: read-reg s_152_0:struct
        let s_152_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_152_0 as isize);
            tracer.read_register(s_152_0 as isize, value);
            value
        };
        // C s_152_2: const #104664u : u32
        let s_152_2: u32 = 104664;
        // N s_152_3: write-reg s_152_2 <= s_152_1
        let s_152_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_152_2 as isize, s_152_1);
            tracer.write_register(s_152_2 as isize, s_152_1);
        };
        // C s_152_4: const #104800u : u32
        let s_152_4: u32 = 104800;
        // D s_152_5: read-reg s_152_4:struct
        let s_152_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_152_4 as isize);
            tracer.read_register(s_152_4 as isize, value);
            value
        };
        // C s_152_6: const #104800u : u32
        let s_152_6: u32 = 104800;
        // N s_152_7: write-reg s_152_6 <= s_152_5
        let s_152_7: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_152_6 as isize, s_152_5);
            tracer.write_register(s_152_6 as isize, s_152_5);
        };
        // C s_152_8: const #100256u : u32
        let s_152_8: u32 = 100256;
        // D s_152_9: read-reg s_152_8:struct
        let s_152_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_152_8 as isize);
            tracer.read_register(s_152_8 as isize, value);
            value
        };
        // C s_152_10: const #100256u : u32
        let s_152_10: u32 = 100256;
        // N s_152_11: write-reg s_152_10 <= s_152_9
        let s_152_11: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_152_10 as isize, s_152_9);
            tracer.write_register(s_152_10 as isize, s_152_9);
        };
        // C s_152_12: const #() : ()
        let s_152_12: () = ();
        // S s_152_13: call HaveSVE2p1(s_152_12)
        let s_152_13: bool = HaveSVE2p1(state, tracer, s_152_12);
        // N s_152_14: branch s_152_13 b185 b153
        if s_152_13 {
            return block_185(state, tracer, fn_state);
        } else {
            return block_153(state, tracer, fn_state);
        };
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_153_0: const #() : ()
        let s_153_0: () = ();
        // S s_153_1: call HaveSME2p1(s_153_0)
        let s_153_1: bool = HaveSME2p1(state, tracer, s_153_0);
        // D s_153_2: write-var gs#329443 <= s_153_1
        fn_state.gs_329443 = s_153_1;
        // N s_153_3: jump b154
        return block_154(state, tracer, fn_state);
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_154_0: read-var gs#329443:u8
        let s_154_0: bool = fn_state.gs_329443;
        // N s_154_1: branch s_154_0 b184 b155
        if s_154_0 {
            return block_184(state, tracer, fn_state);
        } else {
            return block_155(state, tracer, fn_state);
        };
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_155_0: jump b156
        return block_156(state, tracer, fn_state);
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_156_0: const #100256u : u32
        let s_156_0: u32 = 100256;
        // D s_156_1: read-reg s_156_0:struct
        let s_156_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_156_0 as isize);
            tracer.read_register(s_156_0 as isize, value);
            value
        };
        // C s_156_2: const #100256u : u32
        let s_156_2: u32 = 100256;
        // N s_156_3: write-reg s_156_2 <= s_156_1
        let s_156_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_156_2 as isize, s_156_1);
            tracer.write_register(s_156_2 as isize, s_156_1);
        };
        // C s_156_4: const #102832u : u32
        let s_156_4: u32 = 102832;
        // D s_156_5: read-reg s_156_4:struct
        let s_156_5: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_156_4 as isize);
            tracer.read_register(s_156_4 as isize, value);
            value
        };
        // C s_156_6: const #102832u : u32
        let s_156_6: u32 = 102832;
        // N s_156_7: write-reg s_156_6 <= s_156_5
        let s_156_7: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_156_6 as isize, s_156_5);
            tracer.write_register(s_156_6 as isize, s_156_5);
        };
        // C s_156_8: const #20112u : u32
        let s_156_8: u32 = 20112;
        // D s_156_9: read-reg s_156_8:struct
        let s_156_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_156_8 as isize);
            tracer.read_register(s_156_8 as isize, value);
            value
        };
        // C s_156_10: const #20112u : u32
        let s_156_10: u32 = 20112;
        // N s_156_11: write-reg s_156_10 <= s_156_9
        let s_156_11: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_156_10 as isize, s_156_9);
            tracer.write_register(s_156_10 as isize, s_156_9);
        };
        // C s_156_12: const #22552u : u32
        let s_156_12: u32 = 22552;
        // D s_156_13: read-reg s_156_12:struct
        let s_156_13: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_156_12 as isize);
            tracer.read_register(s_156_12 as isize, value);
            value
        };
        // C s_156_14: const #22552u : u32
        let s_156_14: u32 = 22552;
        // N s_156_15: write-reg s_156_14 <= s_156_13
        let s_156_15: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_156_14 as isize, s_156_13);
            tracer.write_register(s_156_14 as isize, s_156_13);
        };
        // C s_156_16: const #() : ()
        let s_156_16: () = ();
        // S s_156_17: call HaveFeatEBEP(s_156_16)
        let s_156_17: bool = HaveFeatEBEP(state, tracer, s_156_16);
        // N s_156_18: branch s_156_17 b183 b157
        if s_156_17 {
            return block_183(state, tracer, fn_state);
        } else {
            return block_157(state, tracer, fn_state);
        };
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_157_0: jump b158
        return block_158(state, tracer, fn_state);
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_158_0: const #90600u : u32
        let s_158_0: u32 = 90600;
        // D s_158_1: read-reg s_158_0:struct
        let s_158_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_158_0 as isize);
            tracer.read_register(s_158_0 as isize, value);
            value
        };
        // C s_158_2: const #90600u : u32
        let s_158_2: u32 = 90600;
        // N s_158_3: write-reg s_158_2 <= s_158_1
        let s_158_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_158_2 as isize, s_158_1);
            tracer.write_register(s_158_2 as isize, s_158_1);
        };
        // C s_158_4: const #14896u : u32
        let s_158_4: u32 = 14896;
        // D s_158_5: read-reg s_158_4:struct
        let s_158_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_158_4 as isize);
            tracer.read_register(s_158_4 as isize, value);
            value
        };
        // C s_158_6: const #14896u : u32
        let s_158_6: u32 = 14896;
        // N s_158_7: write-reg s_158_6 <= s_158_5
        let s_158_7: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_158_6 as isize, s_158_5);
            tracer.write_register(s_158_6 as isize, s_158_5);
        };
        // C s_158_8: const #22112u : u32
        let s_158_8: u32 = 22112;
        // D s_158_9: read-reg s_158_8:struct
        let s_158_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_158_8 as isize);
            tracer.read_register(s_158_8 as isize, value);
            value
        };
        // C s_158_10: const #22112u : u32
        let s_158_10: u32 = 22112;
        // N s_158_11: write-reg s_158_10 <= s_158_9
        let s_158_11: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_158_10 as isize, s_158_9);
            tracer.write_register(s_158_10 as isize, s_158_9);
        };
        // C s_158_12: const #22112u : u32
        let s_158_12: u32 = 22112;
        // D s_158_13: read-reg s_158_12:struct
        let s_158_13: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_158_12 as isize);
            tracer.read_register(s_158_12 as isize, value);
            value
        };
        // C s_158_14: const #22112u : u32
        let s_158_14: u32 = 22112;
        // N s_158_15: write-reg s_158_14 <= s_158_13
        let s_158_15: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_158_14 as isize, s_158_13);
            tracer.write_register(s_158_14 as isize, s_158_13);
        };
        // C s_158_16: const #100864u : u32
        let s_158_16: u32 = 100864;
        // D s_158_17: read-reg s_158_16:struct
        let s_158_17: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_158_16 as isize);
            tracer.read_register(s_158_16 as isize, value);
            value
        };
        // C s_158_18: const #100864u : u32
        let s_158_18: u32 = 100864;
        // N s_158_19: write-reg s_158_18 <= s_158_17
        let s_158_19: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_158_18 as isize, s_158_17);
            tracer.write_register(s_158_18 as isize, s_158_17);
        };
        // C s_158_20: const #() : ()
        let s_158_20: () = ();
        // S s_158_21: call HaveMTEPermExt(s_158_20)
        let s_158_21: bool = HaveMTEPermExt(state, tracer, s_158_20);
        // N s_158_22: branch s_158_21 b182 b159
        if s_158_21 {
            return block_182(state, tracer, fn_state);
        } else {
            return block_159(state, tracer, fn_state);
        };
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_159_0: jump b160
        return block_160(state, tracer, fn_state);
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_160_0: const #() : ()
        let s_160_0: () = ();
        // S s_160_1: call HavePFAR(s_160_0)
        let s_160_1: bool = HavePFAR(state, tracer, s_160_0);
        // N s_160_2: branch s_160_1 b181 b161
        if s_160_1 {
            return block_181(state, tracer, fn_state);
        } else {
            return block_161(state, tracer, fn_state);
        };
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_161_0: jump b162
        return block_162(state, tracer, fn_state);
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_162_0: const #() : ()
        let s_162_0: () = ();
        // S s_162_1: call HaveDoubleFault2Ext(s_162_0)
        let s_162_1: bool = HaveDoubleFault2Ext(state, tracer, s_162_0);
        // N s_162_2: branch s_162_1 b180 b163
        if s_162_1 {
            return block_180(state, tracer, fn_state);
        } else {
            return block_163(state, tracer, fn_state);
        };
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_163_0: jump b164
        return block_164(state, tracer, fn_state);
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_164_0: const #() : ()
        let s_164_0: () = ();
        // S s_164_1: call HaveTHExt(s_164_0)
        let s_164_1: bool = HaveTHExt(state, tracer, s_164_0);
        // N s_164_2: branch s_164_1 b179 b165
        if s_164_1 {
            return block_179(state, tracer, fn_state);
        } else {
            return block_165(state, tracer, fn_state);
        };
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_165_0: jump b166
        return block_166(state, tracer, fn_state);
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_166_0: const #() : ()
        let s_166_0: () = ();
        // S s_166_1: call HaveMTE4Ext(s_166_0)
        let s_166_1: bool = HaveMTE4Ext(state, tracer, s_166_0);
        // N s_166_2: branch s_166_1 b178 b167
        if s_166_1 {
            return block_178(state, tracer, fn_state);
        } else {
            return block_167(state, tracer, fn_state);
        };
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_167_0: jump b168
        return block_168(state, tracer, fn_state);
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_168_0: const #20944u : u32
        let s_168_0: u32 = 20944;
        // D s_168_1: read-reg s_168_0:struct
        let s_168_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_168_0 as isize);
            tracer.read_register(s_168_0 as isize, value);
            value
        };
        // C s_168_2: const #20944u : u32
        let s_168_2: u32 = 20944;
        // N s_168_3: write-reg s_168_2 <= s_168_1
        let s_168_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_168_2 as isize, s_168_1);
            tracer.write_register(s_168_2 as isize, s_168_1);
        };
        // C s_168_4: const #() : ()
        let s_168_4: () = ();
        // S s_168_5: call HaveRASv2Ext(s_168_4)
        let s_168_5: bool = HaveRASv2Ext(state, tracer, s_168_4);
        // N s_168_6: branch s_168_5 b177 b169
        if s_168_5 {
            return block_177(state, tracer, fn_state);
        } else {
            return block_169(state, tracer, fn_state);
        };
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_169_0: jump b170
        return block_170(state, tracer, fn_state);
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_170_0: const #22112u : u32
        let s_170_0: u32 = 22112;
        // D s_170_1: read-reg s_170_0:struct
        let s_170_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_170_0 as isize);
            tracer.read_register(s_170_0 as isize, value);
            value
        };
        // C s_170_2: const #22112u : u32
        let s_170_2: u32 = 22112;
        // N s_170_3: write-reg s_170_2 <= s_170_1
        let s_170_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_170_2 as isize, s_170_1);
            tracer.write_register(s_170_2 as isize, s_170_1);
        };
        // C s_170_4: const #22112u : u32
        let s_170_4: u32 = 22112;
        // D s_170_5: read-reg s_170_4:struct
        let s_170_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_170_4 as isize);
            tracer.read_register(s_170_4 as isize, value);
            value
        };
        // C s_170_6: const #22112u : u32
        let s_170_6: u32 = 22112;
        // N s_170_7: write-reg s_170_6 <= s_170_5
        let s_170_7: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_170_6 as isize, s_170_5);
            tracer.write_register(s_170_6 as isize, s_170_5);
        };
        // C s_170_8: const #22112u : u32
        let s_170_8: u32 = 22112;
        // D s_170_9: read-reg s_170_8:struct
        let s_170_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_170_8 as isize);
            tracer.read_register(s_170_8 as isize, value);
            value
        };
        // C s_170_10: const #22112u : u32
        let s_170_10: u32 = 22112;
        // N s_170_11: write-reg s_170_10 <= s_170_9
        let s_170_11: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_170_10 as isize, s_170_9);
            tracer.write_register(s_170_10 as isize, s_170_9);
        };
        // C s_170_12: const #22112u : u32
        let s_170_12: u32 = 22112;
        // D s_170_13: read-reg s_170_12:struct
        let s_170_13: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_170_12 as isize);
            tracer.read_register(s_170_12 as isize, value);
            value
        };
        // C s_170_14: const #22112u : u32
        let s_170_14: u32 = 22112;
        // N s_170_15: write-reg s_170_14 <= s_170_13
        let s_170_15: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_170_14 as isize, s_170_13);
            tracer.write_register(s_170_14 as isize, s_170_13);
        };
        // C s_170_16: const #104800u : u32
        let s_170_16: u32 = 104800;
        // D s_170_17: read-reg s_170_16:struct
        let s_170_17: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_170_16 as isize);
            tracer.read_register(s_170_16 as isize, value);
            value
        };
        // C s_170_18: const #104800u : u32
        let s_170_18: u32 = 104800;
        // N s_170_19: write-reg s_170_18 <= s_170_17
        let s_170_19: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_170_18 as isize, s_170_17);
            tracer.write_register(s_170_18 as isize, s_170_17);
        };
        // C s_170_20: const #104800u : u32
        let s_170_20: u32 = 104800;
        // D s_170_21: read-reg s_170_20:struct
        let s_170_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_170_20 as isize);
            tracer.read_register(s_170_20 as isize, value);
            value
        };
        // C s_170_22: const #104800u : u32
        let s_170_22: u32 = 104800;
        // N s_170_23: write-reg s_170_22 <= s_170_21
        let s_170_23: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_170_22 as isize, s_170_21);
            tracer.write_register(s_170_22 as isize, s_170_21);
        };
        // C s_170_24: const #104800u : u32
        let s_170_24: u32 = 104800;
        // D s_170_25: read-reg s_170_24:struct
        let s_170_25: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_170_24 as isize);
            tracer.read_register(s_170_24 as isize, value);
            value
        };
        // C s_170_26: const #104800u : u32
        let s_170_26: u32 = 104800;
        // N s_170_27: write-reg s_170_26 <= s_170_25
        let s_170_27: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_170_26 as isize, s_170_25);
            tracer.write_register(s_170_26 as isize, s_170_25);
        };
        // C s_170_28: const #104800u : u32
        let s_170_28: u32 = 104800;
        // D s_170_29: read-reg s_170_28:struct
        let s_170_29: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_170_28 as isize);
            tracer.read_register(s_170_28 as isize, value);
            value
        };
        // C s_170_30: const #104800u : u32
        let s_170_30: u32 = 104800;
        // N s_170_31: write-reg s_170_30 <= s_170_29
        let s_170_31: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_170_30 as isize, s_170_29);
            tracer.write_register(s_170_30 as isize, s_170_29);
        };
        // C s_170_32: const #104800u : u32
        let s_170_32: u32 = 104800;
        // D s_170_33: read-reg s_170_32:struct
        let s_170_33: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_170_32 as isize);
            tracer.read_register(s_170_32 as isize, value);
            value
        };
        // C s_170_34: const #104800u : u32
        let s_170_34: u32 = 104800;
        // N s_170_35: write-reg s_170_34 <= s_170_33
        let s_170_35: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_170_34 as isize, s_170_33);
            tracer.write_register(s_170_34 as isize, s_170_33);
        };
        // C s_170_36: const #104800u : u32
        let s_170_36: u32 = 104800;
        // D s_170_37: read-reg s_170_36:struct
        let s_170_37: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_170_36 as isize);
            tracer.read_register(s_170_36 as isize, value);
            value
        };
        // C s_170_38: const #104800u : u32
        let s_170_38: u32 = 104800;
        // N s_170_39: write-reg s_170_38 <= s_170_37
        let s_170_39: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_170_38 as isize, s_170_37);
            tracer.write_register(s_170_38 as isize, s_170_37);
        };
        // C s_170_40: const #22552u : u32
        let s_170_40: u32 = 22552;
        // D s_170_41: read-reg s_170_40:struct
        let s_170_41: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_170_40 as isize);
            tracer.read_register(s_170_40 as isize, value);
            value
        };
        // C s_170_42: const #22552u : u32
        let s_170_42: u32 = 22552;
        // N s_170_43: write-reg s_170_42 <= s_170_41
        let s_170_43: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_170_42 as isize, s_170_41);
            tracer.write_register(s_170_42 as isize, s_170_41);
        };
        // C s_170_44: const #14600u : u32
        let s_170_44: u32 = 14600;
        // D s_170_45: read-reg s_170_44:struct
        let s_170_45: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_170_44 as isize);
            tracer.read_register(s_170_44 as isize, value);
            value
        };
        // C s_170_46: const #14600u : u32
        let s_170_46: u32 = 14600;
        // N s_170_47: write-reg s_170_46 <= s_170_45
        let s_170_47: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_170_46 as isize, s_170_45);
            tracer.write_register(s_170_46 as isize, s_170_45);
        };
        // C s_170_48: const #104664u : u32
        let s_170_48: u32 = 104664;
        // D s_170_49: read-reg s_170_48:struct
        let s_170_49: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_170_48 as isize);
            tracer.read_register(s_170_48 as isize, value);
            value
        };
        // C s_170_50: const #104664u : u32
        let s_170_50: u32 = 104664;
        // N s_170_51: write-reg s_170_50 <= s_170_49
        let s_170_51: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_170_50 as isize, s_170_49);
            tracer.write_register(s_170_50 as isize, s_170_49);
        };
        // C s_170_52: const #14600u : u32
        let s_170_52: u32 = 14600;
        // D s_170_53: read-reg s_170_52:struct
        let s_170_53: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_170_52 as isize);
            tracer.read_register(s_170_52 as isize, value);
            value
        };
        // C s_170_54: const #14600u : u32
        let s_170_54: u32 = 14600;
        // N s_170_55: write-reg s_170_54 <= s_170_53
        let s_170_55: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_170_54 as isize, s_170_53);
            tracer.write_register(s_170_54 as isize, s_170_53);
        };
        // C s_170_56: const #14600u : u32
        let s_170_56: u32 = 14600;
        // D s_170_57: read-reg s_170_56:struct
        let s_170_57: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_170_56 as isize);
            tracer.read_register(s_170_56 as isize, value);
            value
        };
        // C s_170_58: const #14600u : u32
        let s_170_58: u32 = 14600;
        // N s_170_59: write-reg s_170_58 <= s_170_57
        let s_170_59: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_170_58 as isize, s_170_57);
            tracer.write_register(s_170_58 as isize, s_170_57);
        };
        // C s_170_60: const #14864u : u32
        let s_170_60: u32 = 14864;
        // D s_170_61: read-reg s_170_60:struct
        let s_170_61: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_170_60 as isize);
            tracer.read_register(s_170_60 as isize, value);
            value
        };
        // C s_170_62: const #14864u : u32
        let s_170_62: u32 = 14864;
        // N s_170_63: write-reg s_170_62 <= s_170_61
        let s_170_63: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_170_62 as isize, s_170_61);
            tracer.write_register(s_170_62 as isize, s_170_61);
        };
        // C s_170_64: const #14864u : u32
        let s_170_64: u32 = 14864;
        // D s_170_65: read-reg s_170_64:struct
        let s_170_65: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_170_64 as isize);
            tracer.read_register(s_170_64 as isize, value);
            value
        };
        // C s_170_66: const #14864u : u32
        let s_170_66: u32 = 14864;
        // N s_170_67: write-reg s_170_66 <= s_170_65
        let s_170_67: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_170_66 as isize, s_170_65);
            tracer.write_register(s_170_66 as isize, s_170_65);
        };
        // C s_170_68: const #14864u : u32
        let s_170_68: u32 = 14864;
        // D s_170_69: read-reg s_170_68:struct
        let s_170_69: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_170_68 as isize);
            tracer.read_register(s_170_68 as isize, value);
            value
        };
        // C s_170_70: const #14864u : u32
        let s_170_70: u32 = 14864;
        // N s_170_71: write-reg s_170_70 <= s_170_69
        let s_170_71: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_170_70 as isize, s_170_69);
            tracer.write_register(s_170_70 as isize, s_170_69);
        };
        // C s_170_72: const #14864u : u32
        let s_170_72: u32 = 14864;
        // D s_170_73: read-reg s_170_72:struct
        let s_170_73: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_170_72 as isize);
            tracer.read_register(s_170_72 as isize, value);
            value
        };
        // C s_170_74: const #14864u : u32
        let s_170_74: u32 = 14864;
        // N s_170_75: write-reg s_170_74 <= s_170_73
        let s_170_75: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_170_74 as isize, s_170_73);
            tracer.write_register(s_170_74 as isize, s_170_73);
        };
        // C s_170_76: const #103144u : u32
        let s_170_76: u32 = 103144;
        // D s_170_77: read-reg s_170_76:struct
        let s_170_77: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_170_76 as isize);
            tracer.read_register(s_170_76 as isize, value);
            value
        };
        // C s_170_78: const #103144u : u32
        let s_170_78: u32 = 103144;
        // N s_170_79: write-reg s_170_78 <= s_170_77
        let s_170_79: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_170_78 as isize, s_170_77);
            tracer.write_register(s_170_78 as isize, s_170_77);
        };
        // C s_170_80: const #101824u : u32
        let s_170_80: u32 = 101824;
        // D s_170_81: read-reg s_170_80:struct
        let s_170_81: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_170_80 as isize);
            tracer.read_register(s_170_80 as isize, value);
            value
        };
        // C s_170_82: const #101824u : u32
        let s_170_82: u32 = 101824;
        // N s_170_83: write-reg s_170_82 <= s_170_81
        let s_170_83: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_170_82 as isize, s_170_81);
            tracer.write_register(s_170_82 as isize, s_170_81);
        };
        // C s_170_84: const #103144u : u32
        let s_170_84: u32 = 103144;
        // D s_170_85: read-reg s_170_84:struct
        let s_170_85: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_170_84 as isize);
            tracer.read_register(s_170_84 as isize, value);
            value
        };
        // C s_170_86: const #103144u : u32
        let s_170_86: u32 = 103144;
        // N s_170_87: write-reg s_170_86 <= s_170_85
        let s_170_87: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_170_86 as isize, s_170_85);
            tracer.write_register(s_170_86 as isize, s_170_85);
        };
        // C s_170_88: const #22112u : u32
        let s_170_88: u32 = 22112;
        // D s_170_89: read-reg s_170_88:struct
        let s_170_89: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_170_88 as isize);
            tracer.read_register(s_170_88 as isize, value);
            value
        };
        // C s_170_90: const #22112u : u32
        let s_170_90: u32 = 22112;
        // N s_170_91: write-reg s_170_90 <= s_170_89
        let s_170_91: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_170_90 as isize, s_170_89);
            tracer.write_register(s_170_90 as isize, s_170_89);
        };
        // C s_170_92: const #22112u : u32
        let s_170_92: u32 = 22112;
        // D s_170_93: read-reg s_170_92:struct
        let s_170_93: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_170_92 as isize);
            tracer.read_register(s_170_92 as isize, value);
            value
        };
        // C s_170_94: const #22112u : u32
        let s_170_94: u32 = 22112;
        // N s_170_95: write-reg s_170_94 <= s_170_93
        let s_170_95: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_170_94 as isize, s_170_93);
            tracer.write_register(s_170_94 as isize, s_170_93);
        };
        // C s_170_96: const #22112u : u32
        let s_170_96: u32 = 22112;
        // D s_170_97: read-reg s_170_96:struct
        let s_170_97: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_170_96 as isize);
            tracer.read_register(s_170_96 as isize, value);
            value
        };
        // C s_170_98: const #22112u : u32
        let s_170_98: u32 = 22112;
        // N s_170_99: write-reg s_170_98 <= s_170_97
        let s_170_99: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_170_98 as isize, s_170_97);
            tracer.write_register(s_170_98 as isize, s_170_97);
        };
        // C s_170_100: const #22112u : u32
        let s_170_100: u32 = 22112;
        // D s_170_101: read-reg s_170_100:struct
        let s_170_101: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_170_100 as isize);
            tracer.read_register(s_170_100 as isize, value);
            value
        };
        // C s_170_102: const #22112u : u32
        let s_170_102: u32 = 22112;
        // N s_170_103: write-reg s_170_102 <= s_170_101
        let s_170_103: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_170_102 as isize, s_170_101);
            tracer.write_register(s_170_102 as isize, s_170_101);
        };
        // C s_170_104: const #22112u : u32
        let s_170_104: u32 = 22112;
        // D s_170_105: read-reg s_170_104:struct
        let s_170_105: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_170_104 as isize);
            tracer.read_register(s_170_104 as isize, value);
            value
        };
        // C s_170_106: const #22112u : u32
        let s_170_106: u32 = 22112;
        // N s_170_107: write-reg s_170_106 <= s_170_105
        let s_170_107: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_170_106 as isize, s_170_105);
            tracer.write_register(s_170_106 as isize, s_170_105);
        };
        // C s_170_108: const #22112u : u32
        let s_170_108: u32 = 22112;
        // D s_170_109: read-reg s_170_108:struct
        let s_170_109: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_170_108 as isize);
            tracer.read_register(s_170_108 as isize, value);
            value
        };
        // C s_170_110: const #22112u : u32
        let s_170_110: u32 = 22112;
        // N s_170_111: write-reg s_170_110 <= s_170_109
        let s_170_111: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_170_110 as isize, s_170_109);
            tracer.write_register(s_170_110 as isize, s_170_109);
        };
        // C s_170_112: const #22112u : u32
        let s_170_112: u32 = 22112;
        // D s_170_113: read-reg s_170_112:struct
        let s_170_113: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_170_112 as isize);
            tracer.read_register(s_170_112 as isize, value);
            value
        };
        // C s_170_114: const #22112u : u32
        let s_170_114: u32 = 22112;
        // N s_170_115: write-reg s_170_114 <= s_170_113
        let s_170_115: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_170_114 as isize, s_170_113);
            tracer.write_register(s_170_114 as isize, s_170_113);
        };
        // C s_170_116: const #22112u : u32
        let s_170_116: u32 = 22112;
        // D s_170_117: read-reg s_170_116:struct
        let s_170_117: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_170_116 as isize);
            tracer.read_register(s_170_116 as isize, value);
            value
        };
        // C s_170_118: const #22112u : u32
        let s_170_118: u32 = 22112;
        // N s_170_119: write-reg s_170_118 <= s_170_117
        let s_170_119: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_170_118 as isize, s_170_117);
            tracer.write_register(s_170_118 as isize, s_170_117);
        };
        // C s_170_120: const #20112u : u32
        let s_170_120: u32 = 20112;
        // D s_170_121: read-reg s_170_120:struct
        let s_170_121: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_170_120 as isize);
            tracer.read_register(s_170_120 as isize, value);
            value
        };
        // C s_170_122: const #20112u : u32
        let s_170_122: u32 = 20112;
        // N s_170_123: write-reg s_170_122 <= s_170_121
        let s_170_123: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_170_122 as isize, s_170_121);
            tracer.write_register(s_170_122 as isize, s_170_121);
        };
        // C s_170_124: const #20112u : u32
        let s_170_124: u32 = 20112;
        // D s_170_125: read-reg s_170_124:struct
        let s_170_125: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_170_124 as isize);
            tracer.read_register(s_170_124 as isize, value);
            value
        };
        // C s_170_126: const #20112u : u32
        let s_170_126: u32 = 20112;
        // N s_170_127: write-reg s_170_126 <= s_170_125
        let s_170_127: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_170_126 as isize, s_170_125);
            tracer.write_register(s_170_126 as isize, s_170_125);
        };
        // C s_170_128: const #20112u : u32
        let s_170_128: u32 = 20112;
        // D s_170_129: read-reg s_170_128:struct
        let s_170_129: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_170_128 as isize);
            tracer.read_register(s_170_128 as isize, value);
            value
        };
        // C s_170_130: const #20112u : u32
        let s_170_130: u32 = 20112;
        // N s_170_131: write-reg s_170_130 <= s_170_129
        let s_170_131: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_170_130 as isize, s_170_129);
            tracer.write_register(s_170_130 as isize, s_170_129);
        };
        // C s_170_132: const #20112u : u32
        let s_170_132: u32 = 20112;
        // D s_170_133: read-reg s_170_132:struct
        let s_170_133: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_170_132 as isize);
            tracer.read_register(s_170_132 as isize, value);
            value
        };
        // C s_170_134: const #20112u : u32
        let s_170_134: u32 = 20112;
        // N s_170_135: write-reg s_170_134 <= s_170_133
        let s_170_135: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_170_134 as isize, s_170_133);
            tracer.write_register(s_170_134 as isize, s_170_133);
        };
        // C s_170_136: const #20112u : u32
        let s_170_136: u32 = 20112;
        // D s_170_137: read-reg s_170_136:struct
        let s_170_137: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_170_136 as isize);
            tracer.read_register(s_170_136 as isize, value);
            value
        };
        // C s_170_138: const #20112u : u32
        let s_170_138: u32 = 20112;
        // N s_170_139: write-reg s_170_138 <= s_170_137
        let s_170_139: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_170_138 as isize, s_170_137);
            tracer.write_register(s_170_138 as isize, s_170_137);
        };
        // C s_170_140: const #102832u : u32
        let s_170_140: u32 = 102832;
        // D s_170_141: read-reg s_170_140:struct
        let s_170_141: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_170_140 as isize);
            tracer.read_register(s_170_140 as isize, value);
            value
        };
        // C s_170_142: const #102832u : u32
        let s_170_142: u32 = 102832;
        // N s_170_143: write-reg s_170_142 <= s_170_141
        let s_170_143: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_170_142 as isize, s_170_141);
            tracer.write_register(s_170_142 as isize, s_170_141);
        };
        // C s_170_144: const #20432u : u32
        let s_170_144: u32 = 20432;
        // D s_170_145: read-reg s_170_144:struct
        let s_170_145: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_170_144 as isize);
            tracer.read_register(s_170_144 as isize, value);
            value
        };
        // C s_170_146: const #20432u : u32
        let s_170_146: u32 = 20432;
        // N s_170_147: write-reg s_170_146 <= s_170_145
        let s_170_147: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_170_146 as isize, s_170_145);
            tracer.write_register(s_170_146 as isize, s_170_145);
        };
        // C s_170_148: const #1u : u8
        let s_170_148: bool = true;
        // S s_170_149: call Bit(s_170_148)
        let s_170_149: bool = Bit(state, tracer, s_170_148);
        // C s_170_150: const #16960u : u32
        let s_170_150: u32 = 16960;
        // D s_170_151: read-reg s_170_150:struct
        let s_170_151: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_170_150 as isize);
            tracer.read_register(s_170_150 as isize, value);
            value
        };
        // C s_170_152: const #16960u : u32
        let s_170_152: u32 = 16960;
        // N s_170_153: write-reg s_170_152 <= s_170_151
        let s_170_153: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_170_152 as isize, s_170_151);
            tracer.write_register(s_170_152 as isize, s_170_151);
        };
        // C s_170_154: const #14896u : u32
        let s_170_154: u32 = 14896;
        // D s_170_155: read-reg s_170_154:struct
        let s_170_155: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_170_154 as isize);
            tracer.read_register(s_170_154 as isize, value);
            value
        };
        // C s_170_156: const #14896u : u32
        let s_170_156: u32 = 14896;
        // N s_170_157: write-reg s_170_156 <= s_170_155
        let s_170_157: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_170_156 as isize, s_170_155);
            tracer.write_register(s_170_156 as isize, s_170_155);
        };
        // C s_170_158: const #20416u : u32
        let s_170_158: u32 = 20416;
        // D s_170_159: read-reg s_170_158:struct
        let s_170_159: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_170_158 as isize);
            tracer.read_register(s_170_158 as isize, value);
            value
        };
        // C s_170_160: const #20416u : u32
        let s_170_160: u32 = 20416;
        // N s_170_161: write-reg s_170_160 <= s_170_159
        let s_170_161: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_170_160 as isize, s_170_159);
            tracer.write_register(s_170_160 as isize, s_170_159);
        };
        // C s_170_162: const #20416u : u32
        let s_170_162: u32 = 20416;
        // D s_170_163: read-reg s_170_162:struct
        let s_170_163: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_170_162 as isize);
            tracer.read_register(s_170_162 as isize, value);
            value
        };
        // C s_170_164: const #20416u : u32
        let s_170_164: u32 = 20416;
        // N s_170_165: write-reg s_170_164 <= s_170_163
        let s_170_165: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_170_164 as isize, s_170_163);
            tracer.write_register(s_170_164 as isize, s_170_163);
        };
        // C s_170_166: const #20416u : u32
        let s_170_166: u32 = 20416;
        // D s_170_167: read-reg s_170_166:struct
        let s_170_167: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_170_166 as isize);
            tracer.read_register(s_170_166 as isize, value);
            value
        };
        // C s_170_168: const #20416u : u32
        let s_170_168: u32 = 20416;
        // N s_170_169: write-reg s_170_168 <= s_170_167
        let s_170_169: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_170_168 as isize, s_170_167);
            tracer.write_register(s_170_168 as isize, s_170_167);
        };
        // C s_170_170: const #90408u : u32
        let s_170_170: u32 = 90408;
        // D s_170_171: read-reg s_170_170:struct
        let s_170_171: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_170_170 as isize);
            tracer.read_register(s_170_170 as isize, value);
            value
        };
        // C s_170_172: const #90408u : u32
        let s_170_172: u32 = 90408;
        // N s_170_173: write-reg s_170_172 <= s_170_171
        let s_170_173: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_170_172 as isize, s_170_171);
            tracer.write_register(s_170_172 as isize, s_170_171);
        };
        // C s_170_174: const #90408u : u32
        let s_170_174: u32 = 90408;
        // D s_170_175: read-reg s_170_174:struct
        let s_170_175: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_170_174 as isize);
            tracer.read_register(s_170_174 as isize, value);
            value
        };
        // C s_170_176: const #90408u : u32
        let s_170_176: u32 = 90408;
        // N s_170_177: write-reg s_170_176 <= s_170_175
        let s_170_177: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_170_176 as isize, s_170_175);
            tracer.write_register(s_170_176 as isize, s_170_175);
        };
        // C s_170_178: const #16s : i
        let s_170_178: i128 = 16;
        // C s_170_179: const #19360u : u32
        let s_170_179: u32 = 19360;
        // D s_170_180: read-reg s_170_179:i
        let s_170_180: i128 = {
            let value = state.read_register::<i128>(s_170_179 as isize);
            tracer.read_register(s_170_179 as isize, value);
            value
        };
        // D s_170_181: cmp-ge s_170_180 s_170_178
        let s_170_181: bool = ((s_170_180) >= (s_170_178));
        // N s_170_182: branch s_170_181 b176 b171
        if s_170_181 {
            return block_176(state, tracer, fn_state);
        } else {
            return block_171(state, tracer, fn_state);
        };
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_171_0: const #1s : i
        let s_171_0: i128 = 1;
        // C s_171_1: const #19360u : u32
        let s_171_1: u32 = 19360;
        // D s_171_2: read-reg s_171_1:i
        let s_171_2: i128 = {
            let value = state.read_register::<i128>(s_171_1 as isize);
            tracer.read_register(s_171_1 as isize, value);
            value
        };
        // D s_171_3: sub s_171_2 s_171_0
        let s_171_3: i128 = ((s_171_2) - (s_171_0));
        // C s_171_4: const #3s : i
        let s_171_4: i128 = 3;
        // C s_171_5: const #0s : i
        let s_171_5: i128 = 0;
        // D s_171_6: call integer_subrange(s_171_3, s_171_4, s_171_5)
        let s_171_6: Bits = integer_subrange(state, tracer, s_171_3, s_171_4, s_171_5);
        // N s_171_7: jump b172
        return block_172(state, tracer, fn_state);
    }
    fn block_172<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_172_0: const #20112u : u32
        let s_172_0: u32 = 20112;
        // D s_172_1: read-reg s_172_0:struct
        let s_172_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_172_0 as isize);
            tracer.read_register(s_172_0 as isize, value);
            value
        };
        // C s_172_2: const #20112u : u32
        let s_172_2: u32 = 20112;
        // N s_172_3: write-reg s_172_2 <= s_172_1
        let s_172_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_172_2 as isize, s_172_1);
            tracer.write_register(s_172_2 as isize, s_172_1);
        };
        // C s_172_4: const #16s : i
        let s_172_4: i128 = 16;
        // C s_172_5: const #22776u : u32
        let s_172_5: u32 = 22776;
        // D s_172_6: read-reg s_172_5:i
        let s_172_6: i128 = {
            let value = state.read_register::<i128>(s_172_5 as isize);
            tracer.read_register(s_172_5 as isize, value);
            value
        };
        // D s_172_7: cmp-ge s_172_6 s_172_4
        let s_172_7: bool = ((s_172_6) >= (s_172_4));
        // N s_172_8: branch s_172_7 b175 b173
        if s_172_7 {
            return block_175(state, tracer, fn_state);
        } else {
            return block_173(state, tracer, fn_state);
        };
    }
    fn block_173<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_173_0: const #1s : i
        let s_173_0: i128 = 1;
        // C s_173_1: const #22776u : u32
        let s_173_1: u32 = 22776;
        // D s_173_2: read-reg s_173_1:i
        let s_173_2: i128 = {
            let value = state.read_register::<i128>(s_173_1 as isize);
            tracer.read_register(s_173_1 as isize, value);
            value
        };
        // D s_173_3: sub s_173_2 s_173_0
        let s_173_3: i128 = ((s_173_2) - (s_173_0));
        // C s_173_4: const #3s : i
        let s_173_4: i128 = 3;
        // C s_173_5: const #0s : i
        let s_173_5: i128 = 0;
        // D s_173_6: call integer_subrange(s_173_3, s_173_4, s_173_5)
        let s_173_6: Bits = integer_subrange(state, tracer, s_173_3, s_173_4, s_173_5);
        // N s_173_7: jump b174
        return block_174(state, tracer, fn_state);
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_174_0: const #20112u : u32
        let s_174_0: u32 = 20112;
        // D s_174_1: read-reg s_174_0:struct
        let s_174_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_174_0 as isize);
            tracer.read_register(s_174_0 as isize, value);
            value
        };
        // C s_174_2: const #20112u : u32
        let s_174_2: u32 = 20112;
        // N s_174_3: write-reg s_174_2 <= s_174_1
        let s_174_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_174_2 as isize, s_174_1);
            tracer.write_register(s_174_2 as isize, s_174_1);
        };
        // N s_174_4: return
        return;
    }
    fn block_175<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_175_0: jump b174
        return block_174(state, tracer, fn_state);
    }
    fn block_176<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_176_0: jump b172
        return block_172(state, tracer, fn_state);
    }
    fn block_177<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_177_0: const #101824u : u32
        let s_177_0: u32 = 101824;
        // D s_177_1: read-reg s_177_0:struct
        let s_177_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_177_0 as isize);
            tracer.read_register(s_177_0 as isize, value);
            value
        };
        // C s_177_2: const #101824u : u32
        let s_177_2: u32 = 101824;
        // N s_177_3: write-reg s_177_2 <= s_177_1
        let s_177_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_177_2 as isize, s_177_1);
            tracer.write_register(s_177_2 as isize, s_177_1);
        };
        // C s_177_4: const #16288u : u32
        let s_177_4: u32 = 16288;
        // D s_177_5: read-reg s_177_4:struct
        let s_177_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_177_4 as isize);
            tracer.read_register(s_177_4 as isize, value);
            value
        };
        // C s_177_6: const #16288u : u32
        let s_177_6: u32 = 16288;
        // N s_177_7: write-reg s_177_6 <= s_177_5
        let s_177_7: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_177_6 as isize, s_177_5);
            tracer.write_register(s_177_6 as isize, s_177_5);
        };
        // N s_177_8: jump b170
        return block_170(state, tracer, fn_state);
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_178_0: const #22552u : u32
        let s_178_0: u32 = 22552;
        // D s_178_1: read-reg s_178_0:struct
        let s_178_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_178_0 as isize);
            tracer.read_register(s_178_0 as isize, value);
            value
        };
        // C s_178_2: const #22552u : u32
        let s_178_2: u32 = 22552;
        // N s_178_3: write-reg s_178_2 <= s_178_1
        let s_178_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_178_2 as isize, s_178_1);
            tracer.write_register(s_178_2 as isize, s_178_1);
        };
        // C s_178_4: const #13480u : u32
        let s_178_4: u32 = 13480;
        // D s_178_5: read-reg s_178_4:struct
        let s_178_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_178_4 as isize);
            tracer.read_register(s_178_4 as isize, value);
            value
        };
        // C s_178_6: const #13480u : u32
        let s_178_6: u32 = 13480;
        // N s_178_7: write-reg s_178_6 <= s_178_5
        let s_178_7: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_178_6 as isize, s_178_5);
            tracer.write_register(s_178_6 as isize, s_178_5);
        };
        // C s_178_8: const #13480u : u32
        let s_178_8: u32 = 13480;
        // D s_178_9: read-reg s_178_8:struct
        let s_178_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_178_8 as isize);
            tracer.read_register(s_178_8 as isize, value);
            value
        };
        // C s_178_10: const #13480u : u32
        let s_178_10: u32 = 13480;
        // N s_178_11: write-reg s_178_10 <= s_178_9
        let s_178_11: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_178_10 as isize, s_178_9);
            tracer.write_register(s_178_10 as isize, s_178_9);
        };
        // N s_178_12: jump b168
        return block_168(state, tracer, fn_state);
    }
    fn block_179<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_179_0: const #22552u : u32
        let s_179_0: u32 = 22552;
        // D s_179_1: read-reg s_179_0:struct
        let s_179_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_179_0 as isize);
            tracer.read_register(s_179_0 as isize, value);
            value
        };
        // C s_179_2: const #22552u : u32
        let s_179_2: u32 = 22552;
        // N s_179_3: write-reg s_179_2 <= s_179_1
        let s_179_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_179_2 as isize, s_179_1);
            tracer.write_register(s_179_2 as isize, s_179_1);
        };
        // N s_179_4: jump b166
        return block_166(state, tracer, fn_state);
    }
    fn block_180<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_180_0: const #22552u : u32
        let s_180_0: u32 = 22552;
        // D s_180_1: read-reg s_180_0:struct
        let s_180_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_180_0 as isize);
            tracer.read_register(s_180_0 as isize, value);
            value
        };
        // C s_180_2: const #22552u : u32
        let s_180_2: u32 = 22552;
        // N s_180_3: write-reg s_180_2 <= s_180_1
        let s_180_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_180_2 as isize, s_180_1);
            tracer.write_register(s_180_2 as isize, s_180_1);
        };
        // N s_180_4: jump b164
        return block_164(state, tracer, fn_state);
    }
    fn block_181<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_181_0: const #22552u : u32
        let s_181_0: u32 = 22552;
        // D s_181_1: read-reg s_181_0:struct
        let s_181_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_181_0 as isize);
            tracer.read_register(s_181_0 as isize, value);
            value
        };
        // C s_181_2: const #22552u : u32
        let s_181_2: u32 = 22552;
        // N s_181_3: write-reg s_181_2 <= s_181_1
        let s_181_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_181_2 as isize, s_181_1);
            tracer.write_register(s_181_2 as isize, s_181_1);
        };
        // N s_181_4: jump b162
        return block_162(state, tracer, fn_state);
    }
    fn block_182<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_182_0: const #13480u : u32
        let s_182_0: u32 = 13480;
        // D s_182_1: read-reg s_182_0:struct
        let s_182_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_182_0 as isize);
            tracer.read_register(s_182_0 as isize, value);
            value
        };
        // C s_182_2: const #13480u : u32
        let s_182_2: u32 = 13480;
        // N s_182_3: write-reg s_182_2 <= s_182_1
        let s_182_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_182_2 as isize, s_182_1);
            tracer.write_register(s_182_2 as isize, s_182_1);
        };
        // N s_182_4: jump b160
        return block_160(state, tracer, fn_state);
    }
    fn block_183<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_183_0: const #104800u : u32
        let s_183_0: u32 = 104800;
        // D s_183_1: read-reg s_183_0:struct
        let s_183_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_183_0 as isize);
            tracer.read_register(s_183_0 as isize, value);
            value
        };
        // C s_183_2: const #104800u : u32
        let s_183_2: u32 = 104800;
        // N s_183_3: write-reg s_183_2 <= s_183_1
        let s_183_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_183_2 as isize, s_183_1);
            tracer.write_register(s_183_2 as isize, s_183_1);
        };
        // N s_183_4: jump b158
        return block_158(state, tracer, fn_state);
    }
    fn block_184<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_184_0: const #100256u : u32
        let s_184_0: u32 = 100256;
        // D s_184_1: read-reg s_184_0:struct
        let s_184_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_184_0 as isize);
            tracer.read_register(s_184_0 as isize, value);
            value
        };
        // C s_184_2: const #100256u : u32
        let s_184_2: u32 = 100256;
        // N s_184_3: write-reg s_184_2 <= s_184_1
        let s_184_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_184_2 as isize, s_184_1);
            tracer.write_register(s_184_2 as isize, s_184_1);
        };
        // C s_184_4: const #104664u : u32
        let s_184_4: u32 = 104664;
        // D s_184_5: read-reg s_184_4:struct
        let s_184_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_184_4 as isize);
            tracer.read_register(s_184_4 as isize, value);
            value
        };
        // C s_184_6: const #104664u : u32
        let s_184_6: u32 = 104664;
        // N s_184_7: write-reg s_184_6 <= s_184_5
        let s_184_7: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_184_6 as isize, s_184_5);
            tracer.write_register(s_184_6 as isize, s_184_5);
        };
        // N s_184_8: jump b156
        return block_156(state, tracer, fn_state);
    }
    fn block_185<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_185_0: const #1u : u8
        let s_185_0: bool = true;
        // D s_185_1: write-var gs#329443 <= s_185_0
        fn_state.gs_329443 = s_185_0;
        // N s_185_2: jump b154
        return block_154(state, tracer, fn_state);
    }
    fn block_186<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_186_0: const #22552u : u32
        let s_186_0: u32 = 22552;
        // D s_186_1: read-reg s_186_0:struct
        let s_186_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_186_0 as isize);
            tracer.read_register(s_186_0 as isize, value);
            value
        };
        // C s_186_2: const #22552u : u32
        let s_186_2: u32 = 22552;
        // N s_186_3: write-reg s_186_2 <= s_186_1
        let s_186_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_186_2 as isize, s_186_1);
            tracer.write_register(s_186_2 as isize, s_186_1);
        };
        // N s_186_4: jump b152
        return block_152(state, tracer, fn_state);
    }
    fn block_187<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_187_0: const #90608u : u32
        let s_187_0: u32 = 90608;
        // D s_187_1: read-reg s_187_0:struct
        let s_187_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_187_0 as isize);
            tracer.read_register(s_187_0 as isize, value);
            value
        };
        // C s_187_2: const #90608u : u32
        let s_187_2: u32 = 90608;
        // N s_187_3: write-reg s_187_2 <= s_187_1
        let s_187_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_187_2 as isize, s_187_1);
            tracer.write_register(s_187_2 as isize, s_187_1);
        };
        // N s_187_4: jump b150
        return block_150(state, tracer, fn_state);
    }
    fn block_188<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_188_0: jump b146
        return block_146(state, tracer, fn_state);
    }
    fn block_189<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_189_0: jump b144
        return block_144(state, tracer, fn_state);
    }
    fn block_190<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_190_0: jump b142
        return block_142(state, tracer, fn_state);
    }
    fn block_191<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_191_0: jump b140
        return block_140(state, tracer, fn_state);
    }
    fn block_192<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_192_0: jump b138
        return block_138(state, tracer, fn_state);
    }
    fn block_193<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_193_0: jump b136
        return block_136(state, tracer, fn_state);
    }
    fn block_194<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_194_0: jump b134
        return block_134(state, tracer, fn_state);
    }
    fn block_195<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_195_0: jump b132
        return block_132(state, tracer, fn_state);
    }
    fn block_196<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_196_0: jump b130
        return block_130(state, tracer, fn_state);
    }
    fn block_197<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_197_0: jump b128
        return block_128(state, tracer, fn_state);
    }
    fn block_198<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_198_0: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_199<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_199_0: jump b124
        return block_124(state, tracer, fn_state);
    }
    fn block_200<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_200_0: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_201<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_201_0: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_202<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_202_0: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_203<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_203_0: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_204<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_204_0: jump b114
        return block_114(state, tracer, fn_state);
    }
    fn block_205<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_205_0: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_206<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_206_0: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_207<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_207_0: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_208<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_208_0: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_209<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_209_0: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_210<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_210_0: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_211<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_211_0: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_212<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_212_0: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_213<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_213_0: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_214<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_214_0: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_215<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_215_0: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_216<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_216_0: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_217<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_217_0: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_218<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_218_0: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_219<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_219_0: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_220<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_220_0: const #32s : i64
        let s_220_0: i64 = 32;
        // D s_220_1: write-var ga#369678 <= s_220_0
        fn_state.ga_369678 = s_220_0;
        // N s_220_2: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_221<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_221_0: const #3s : i
        let s_221_0: i128 = 3;
        // C s_221_1: const #0s : i
        let s_221_1: i128 = 0;
        // C s_221_2: const #23376u : u32
        let s_221_2: u32 = 23376;
        // D s_221_3: read-reg s_221_2:i
        let s_221_3: i128 = {
            let value = state.read_register::<i128>(s_221_2 as isize);
            tracer.read_register(s_221_2 as isize, value);
            value
        };
        // D s_221_4: call integer_subrange(s_221_3, s_221_0, s_221_1)
        let s_221_4: Bits = integer_subrange(state, tracer, s_221_3, s_221_0, s_221_1);
        // C s_221_5: const #100816u : u32
        let s_221_5: u32 = 100816;
        // D s_221_6: read-reg s_221_5:struct
        let s_221_6: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_221_5 as isize);
            tracer.read_register(s_221_5 as isize, value);
            value
        };
        // C s_221_7: const #100816u : u32
        let s_221_7: u32 = 100816;
        // N s_221_8: write-reg s_221_7 <= s_221_6
        let s_221_8: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_221_7 as isize, s_221_6);
            tracer.write_register(s_221_7 as isize, s_221_6);
        };
        // N s_221_9: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_222<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_222_0: const #19056u : u32
        let s_222_0: u32 = 19056;
        // D s_222_1: read-reg s_222_0:struct
        let s_222_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_222_0 as isize);
            tracer.read_register(s_222_0 as isize, value);
            value
        };
        // C s_222_2: const #19056u : u32
        let s_222_2: u32 = 19056;
        // N s_222_3: write-reg s_222_2 <= s_222_1
        let s_222_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_222_2 as isize, s_222_1);
            tracer.write_register(s_222_2 as isize, s_222_1);
        };
        // N s_222_4: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_223<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_223_0: const #104880u : u32
        let s_223_0: u32 = 104880;
        // D s_223_1: read-reg s_223_0:struct
        let s_223_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_223_0 as isize);
            tracer.read_register(s_223_0 as isize, value);
            value
        };
        // C s_223_2: const #104880u : u32
        let s_223_2: u32 = 104880;
        // N s_223_3: write-reg s_223_2 <= s_223_1
        let s_223_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_223_2 as isize, s_223_1);
            tracer.write_register(s_223_2 as isize, s_223_1);
        };
        // C s_223_4: const #104880u : u32
        let s_223_4: u32 = 104880;
        // D s_223_5: read-reg s_223_4:struct
        let s_223_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_223_4 as isize);
            tracer.read_register(s_223_4 as isize, value);
            value
        };
        // C s_223_6: const #104880u : u32
        let s_223_6: u32 = 104880;
        // N s_223_7: write-reg s_223_6 <= s_223_5
        let s_223_7: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_223_6 as isize, s_223_5);
            tracer.write_register(s_223_6 as isize, s_223_5);
        };
        // N s_223_8: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_224<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_224_0: const #() : ()
        let s_224_0: () = ();
        // S s_224_1: call Unreachable(s_224_0)
        let s_224_1: () = Unreachable(state, tracer, s_224_0);
        // N s_224_2: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_225<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_225_0: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_226<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_226_0: const #102552u : u32
        let s_226_0: u32 = 102552;
        // D s_226_1: read-reg s_226_0:struct
        let s_226_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_226_0 as isize);
            tracer.read_register(s_226_0 as isize, value);
            value
        };
        // D s_226_2: call _get_HCR_EL2_Type_E2H(s_226_1)
        let s_226_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_226_1);
        // D s_226_3: cast zx s_226_2 -> bv
        let s_226_3: Bits = Bits::new(s_226_2 as u128, 1u16);
        // C s_226_4: const #1u : u8
        let s_226_4: bool = true;
        // C s_226_5: cast zx s_226_4 -> bv
        let s_226_5: Bits = Bits::new(s_226_4 as u128, 1u16);
        // D s_226_6: cmp-eq s_226_3 s_226_5
        let s_226_6: bool = ((s_226_3) == (s_226_5));
        // D s_226_7: write-var gs#328784 <= s_226_6
        fn_state.gs_328784 = s_226_6;
        // N s_226_8: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_227<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_227_0: const #103144u : u32
        let s_227_0: u32 = 103144;
        // D s_227_1: read-reg s_227_0:struct
        let s_227_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_227_0 as isize);
            tracer.read_register(s_227_0 as isize, value);
            value
        };
        // C s_227_2: const #103144u : u32
        let s_227_2: u32 = 103144;
        // N s_227_3: write-reg s_227_2 <= s_227_1
        let s_227_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_227_2 as isize, s_227_1);
            tracer.write_register(s_227_2 as isize, s_227_1);
        };
        // C s_227_4: const #20216u : u32
        let s_227_4: u32 = 20216;
        // D s_227_5: read-reg s_227_4:struct
        let s_227_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_227_4 as isize);
            tracer.read_register(s_227_4 as isize, value);
            value
        };
        // C s_227_6: const #20216u : u32
        let s_227_6: u32 = 20216;
        // N s_227_7: write-reg s_227_6 <= s_227_5
        let s_227_7: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_227_6 as isize, s_227_5);
            tracer.write_register(s_227_6 as isize, s_227_5);
        };
        // C s_227_8: const #22552u : u32
        let s_227_8: u32 = 22552;
        // D s_227_9: read-reg s_227_8:struct
        let s_227_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_227_8 as isize);
            tracer.read_register(s_227_8 as isize, value);
            value
        };
        // C s_227_10: const #22552u : u32
        let s_227_10: u32 = 22552;
        // N s_227_11: write-reg s_227_10 <= s_227_9
        let s_227_11: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_227_10 as isize, s_227_9);
            tracer.write_register(s_227_10 as isize, s_227_9);
        };
        // C s_227_12: const #22552u : u32
        let s_227_12: u32 = 22552;
        // D s_227_13: read-reg s_227_12:struct
        let s_227_13: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_227_12 as isize);
            tracer.read_register(s_227_12 as isize, value);
            value
        };
        // C s_227_14: const #22552u : u32
        let s_227_14: u32 = 22552;
        // N s_227_15: write-reg s_227_14 <= s_227_13
        let s_227_15: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_227_14 as isize, s_227_13);
            tracer.write_register(s_227_14 as isize, s_227_13);
        };
        // C s_227_16: const #14864u : u32
        let s_227_16: u32 = 14864;
        // D s_227_17: read-reg s_227_16:struct
        let s_227_17: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_227_16 as isize);
            tracer.read_register(s_227_16 as isize, value);
            value
        };
        // C s_227_18: const #14864u : u32
        let s_227_18: u32 = 14864;
        // N s_227_19: write-reg s_227_18 <= s_227_17
        let s_227_19: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_227_18 as isize, s_227_17);
            tracer.write_register(s_227_18 as isize, s_227_17);
        };
        // C s_227_20: const #14864u : u32
        let s_227_20: u32 = 14864;
        // D s_227_21: read-reg s_227_20:struct
        let s_227_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_227_20 as isize);
            tracer.read_register(s_227_20 as isize, value);
            value
        };
        // C s_227_22: const #14864u : u32
        let s_227_22: u32 = 14864;
        // N s_227_23: write-reg s_227_22 <= s_227_21
        let s_227_23: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_227_22 as isize, s_227_21);
            tracer.write_register(s_227_22 as isize, s_227_21);
        };
        // C s_227_24: const #() : ()
        let s_227_24: () = ();
        // S s_227_25: call HaveStatisticalProfiling(s_227_24)
        let s_227_25: bool = HaveStatisticalProfiling(state, tracer, s_227_24);
        // N s_227_26: branch s_227_25 b236 b228
        if s_227_25 {
            return block_236(state, tracer, fn_state);
        } else {
            return block_228(state, tracer, fn_state);
        };
    }
    fn block_228<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_228_0: jump b229
        return block_229(state, tracer, fn_state);
    }
    fn block_229<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_229_0: const #20112u : u32
        let s_229_0: u32 = 20112;
        // D s_229_1: read-reg s_229_0:struct
        let s_229_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_229_0 as isize);
            tracer.read_register(s_229_0 as isize, value);
            value
        };
        // C s_229_2: const #20112u : u32
        let s_229_2: u32 = 20112;
        // N s_229_3: write-reg s_229_2 <= s_229_1
        let s_229_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_229_2 as isize, s_229_1);
            tracer.write_register(s_229_2 as isize, s_229_1);
        };
        // C s_229_4: const #104720u : u32
        let s_229_4: u32 = 104720;
        // D s_229_5: read-reg s_229_4:struct
        let s_229_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_229_4 as isize);
            tracer.read_register(s_229_4 as isize, value);
            value
        };
        // C s_229_6: const #104720u : u32
        let s_229_6: u32 = 104720;
        // N s_229_7: write-reg s_229_6 <= s_229_5
        let s_229_7: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_229_6 as isize, s_229_5);
            tracer.write_register(s_229_6 as isize, s_229_5);
        };
        // C s_229_8: const #20112u : u32
        let s_229_8: u32 = 20112;
        // D s_229_9: read-reg s_229_8:struct
        let s_229_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_229_8 as isize);
            tracer.read_register(s_229_8 as isize, value);
            value
        };
        // C s_229_10: const #20112u : u32
        let s_229_10: u32 = 20112;
        // N s_229_11: write-reg s_229_10 <= s_229_9
        let s_229_11: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_229_10 as isize, s_229_9);
            tracer.write_register(s_229_10 as isize, s_229_9);
        };
        // C s_229_12: const #20416u : u32
        let s_229_12: u32 = 20416;
        // D s_229_13: read-reg s_229_12:struct
        let s_229_13: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_229_12 as isize);
            tracer.read_register(s_229_12 as isize, value);
            value
        };
        // C s_229_14: const #20416u : u32
        let s_229_14: u32 = 20416;
        // N s_229_15: write-reg s_229_14 <= s_229_13
        let s_229_15: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_229_14 as isize, s_229_13);
            tracer.write_register(s_229_14 as isize, s_229_13);
        };
        // C s_229_16: const #424u : u32
        let s_229_16: u32 = 424;
        // D s_229_17: read-reg s_229_16:u8
        let s_229_17: u8 = {
            let value = state.read_register::<u8>(s_229_16 as isize);
            tracer.read_register(s_229_16 as isize, value);
            value
        };
        // C s_229_18: const #2u : u8
        let s_229_18: u8 = 2;
        // D s_229_19: cmp-lt s_229_17 s_229_18
        let s_229_19: bool = ((s_229_17) < (s_229_18));
        // D s_229_20: not s_229_19
        let s_229_20: bool = !s_229_19;
        // N s_229_21: branch s_229_20 b235 b230
        if s_229_20 {
            return block_235(state, tracer, fn_state);
        } else {
            return block_230(state, tracer, fn_state);
        };
    }
    fn block_230<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_230_0: const #0u : u8
        let s_230_0: bool = false;
        // D s_230_1: write-var gs#330002 <= s_230_0
        fn_state.gs_330002 = s_230_0;
        // N s_230_2: jump b231
        return block_231(state, tracer, fn_state);
    }
    fn block_231<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_231_0: read-var gs#330002:u8
        let s_231_0: bool = fn_state.gs_330002;
        // N s_231_1: branch s_231_0 b234 b232
        if s_231_0 {
            return block_234(state, tracer, fn_state);
        } else {
            return block_232(state, tracer, fn_state);
        };
    }
    fn block_232<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_232_0: const #20416u : u32
        let s_232_0: u32 = 20416;
        // D s_232_1: read-reg s_232_0:struct
        let s_232_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_232_0 as isize);
            tracer.read_register(s_232_0 as isize, value);
            value
        };
        // D s_232_2: call _get_ID_DFR0_EL1_Type_CopDbg(s_232_1)
        let s_232_2: u8 = u_get_ID_DFR0_EL1_Type_CopDbg(state, tracer, s_232_1);
        // C s_232_3: const #20416u : u32
        let s_232_3: u32 = 20416;
        // D s_232_4: read-reg s_232_3:struct
        let s_232_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_232_3 as isize);
            tracer.read_register(s_232_3 as isize, value);
            value
        };
        // C s_232_5: const #20416u : u32
        let s_232_5: u32 = 20416;
        // N s_232_6: write-reg s_232_5 <= s_232_4
        let s_232_6: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_232_5 as isize, s_232_4);
            tracer.write_register(s_232_5 as isize, s_232_4);
        };
        // N s_232_7: jump b233
        return block_233(state, tracer, fn_state);
    }
    fn block_233<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_233_0: const #103144u : u32
        let s_233_0: u32 = 103144;
        // D s_233_1: read-reg s_233_0:struct
        let s_233_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_233_0 as isize);
            tracer.read_register(s_233_0 as isize, value);
            value
        };
        // C s_233_2: const #103144u : u32
        let s_233_2: u32 = 103144;
        // N s_233_3: write-reg s_233_2 <= s_233_1
        let s_233_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_233_2 as isize, s_233_1);
            tracer.write_register(s_233_2 as isize, s_233_1);
        };
        // C s_233_4: const #103144u : u32
        let s_233_4: u32 = 103144;
        // D s_233_5: read-reg s_233_4:struct
        let s_233_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_233_4 as isize);
            tracer.read_register(s_233_4 as isize, value);
            value
        };
        // C s_233_6: const #103144u : u32
        let s_233_6: u32 = 103144;
        // N s_233_7: write-reg s_233_6 <= s_233_5
        let s_233_7: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_233_6 as isize, s_233_5);
            tracer.write_register(s_233_6 as isize, s_233_5);
        };
        // C s_233_8: const #103144u : u32
        let s_233_8: u32 = 103144;
        // D s_233_9: read-reg s_233_8:struct
        let s_233_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_233_8 as isize);
            tracer.read_register(s_233_8 as isize, value);
            value
        };
        // C s_233_10: const #103144u : u32
        let s_233_10: u32 = 103144;
        // N s_233_11: write-reg s_233_10 <= s_233_9
        let s_233_11: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_233_10 as isize, s_233_9);
            tracer.write_register(s_233_10 as isize, s_233_9);
        };
        // C s_233_12: const #20112u : u32
        let s_233_12: u32 = 20112;
        // D s_233_13: read-reg s_233_12:struct
        let s_233_13: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_233_12 as isize);
            tracer.read_register(s_233_12 as isize, value);
            value
        };
        // C s_233_14: const #20112u : u32
        let s_233_14: u32 = 20112;
        // N s_233_15: write-reg s_233_14 <= s_233_13
        let s_233_15: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_233_14 as isize, s_233_13);
            tracer.write_register(s_233_14 as isize, s_233_13);
        };
        // C s_233_16: const #20416u : u32
        let s_233_16: u32 = 20416;
        // D s_233_17: read-reg s_233_16:struct
        let s_233_17: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_233_16 as isize);
            tracer.read_register(s_233_16 as isize, value);
            value
        };
        // C s_233_18: const #20416u : u32
        let s_233_18: u32 = 20416;
        // N s_233_19: write-reg s_233_18 <= s_233_17
        let s_233_19: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_233_18 as isize, s_233_17);
            tracer.write_register(s_233_18 as isize, s_233_17);
        };
        // N s_233_20: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_234<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_234_0: const #20416u : u32
        let s_234_0: u32 = 20416;
        // D s_234_1: read-reg s_234_0:struct
        let s_234_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_234_0 as isize);
            tracer.read_register(s_234_0 as isize, value);
            value
        };
        // C s_234_2: const #20416u : u32
        let s_234_2: u32 = 20416;
        // N s_234_3: write-reg s_234_2 <= s_234_1
        let s_234_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_234_2 as isize, s_234_1);
            tracer.write_register(s_234_2 as isize, s_234_1);
        };
        // N s_234_4: jump b233
        return block_233(state, tracer, fn_state);
    }
    fn block_235<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_235_0: const #20920u : u32
        let s_235_0: u32 = 20920;
        // D s_235_1: read-reg s_235_0:struct
        let s_235_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_235_0 as isize);
            tracer.read_register(s_235_0 as isize, value);
            value
        };
        // D s_235_2: call _get_SCR_Type_NS(s_235_1)
        let s_235_2: bool = u_get_SCR_Type_NS(state, tracer, s_235_1);
        // D s_235_3: cast zx s_235_2 -> bv
        let s_235_3: Bits = Bits::new(s_235_2 as u128, 1u16);
        // C s_235_4: const #1u : u8
        let s_235_4: bool = true;
        // C s_235_5: cast zx s_235_4 -> bv
        let s_235_5: Bits = Bits::new(s_235_4 as u128, 1u16);
        // D s_235_6: cmp-eq s_235_3 s_235_5
        let s_235_6: bool = ((s_235_3) == (s_235_5));
        // D s_235_7: write-var gs#330002 <= s_235_6
        fn_state.gs_330002 = s_235_6;
        // N s_235_8: jump b231
        return block_231(state, tracer, fn_state);
    }
    fn block_236<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_236_0: const #20112u : u32
        let s_236_0: u32 = 20112;
        // D s_236_1: read-reg s_236_0:struct
        let s_236_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_236_0 as isize);
            tracer.read_register(s_236_0 as isize, value);
            value
        };
        // C s_236_2: const #20112u : u32
        let s_236_2: u32 = 20112;
        // N s_236_3: write-reg s_236_2 <= s_236_1
        let s_236_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_236_2 as isize, s_236_1);
            tracer.write_register(s_236_2 as isize, s_236_1);
        };
        // N s_236_4: jump b229
        return block_229(state, tracer, fn_state);
    }
    fn block_237<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_237_0: const #22552u : u32
        let s_237_0: u32 = 22552;
        // D s_237_1: read-reg s_237_0:struct
        let s_237_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_237_0 as isize);
            tracer.read_register(s_237_0 as isize, value);
            value
        };
        // C s_237_2: const #22552u : u32
        let s_237_2: u32 = 22552;
        // N s_237_3: write-reg s_237_2 <= s_237_1
        let s_237_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_237_2 as isize, s_237_1);
            tracer.write_register(s_237_2 as isize, s_237_1);
        };
        // C s_237_4: const #100256u : u32
        let s_237_4: u32 = 100256;
        // D s_237_5: read-reg s_237_4:struct
        let s_237_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_237_4 as isize);
            tracer.read_register(s_237_4 as isize, value);
            value
        };
        // C s_237_6: const #100256u : u32
        let s_237_6: u32 = 100256;
        // N s_237_7: write-reg s_237_6 <= s_237_5
        let s_237_7: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_237_6 as isize, s_237_5);
            tracer.write_register(s_237_6 as isize, s_237_5);
        };
        // C s_237_8: const #100256u : u32
        let s_237_8: u32 = 100256;
        // D s_237_9: read-reg s_237_8:struct
        let s_237_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_237_8 as isize);
            tracer.read_register(s_237_8 as isize, value);
            value
        };
        // C s_237_10: const #100256u : u32
        let s_237_10: u32 = 100256;
        // N s_237_11: write-reg s_237_10 <= s_237_9
        let s_237_11: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_237_10 as isize, s_237_9);
            tracer.write_register(s_237_10 as isize, s_237_9);
        };
        // C s_237_12: const #100256u : u32
        let s_237_12: u32 = 100256;
        // D s_237_13: read-reg s_237_12:struct
        let s_237_13: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_237_12 as isize);
            tracer.read_register(s_237_12 as isize, value);
            value
        };
        // C s_237_14: const #100256u : u32
        let s_237_14: u32 = 100256;
        // N s_237_15: write-reg s_237_14 <= s_237_13
        let s_237_15: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_237_14 as isize, s_237_13);
            tracer.write_register(s_237_14 as isize, s_237_13);
        };
        // N s_237_16: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_238<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_238_0: const #238u : u32
        let s_238_0: u32 = 238;
        // S s_238_1: call IsFeatureImplemented(s_238_0)
        let s_238_1: bool = IsFeatureImplemented(state, tracer, s_238_0);
        // N s_238_2: branch s_238_1 b241 b239
        if s_238_1 {
            return block_241(state, tracer, fn_state);
        } else {
            return block_239(state, tracer, fn_state);
        };
    }
    fn block_239<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_239_0: jump b240
        return block_240(state, tracer, fn_state);
    }
    fn block_240<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_240_0: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_241<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_241_0: const #90808u : u32
        let s_241_0: u32 = 90808;
        // D s_241_1: read-reg s_241_0:struct
        let s_241_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_241_0 as isize);
            tracer.read_register(s_241_0 as isize, value);
            value
        };
        // C s_241_2: const #90808u : u32
        let s_241_2: u32 = 90808;
        // N s_241_3: write-reg s_241_2 <= s_241_1
        let s_241_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_241_2 as isize, s_241_1);
            tracer.write_register(s_241_2 as isize, s_241_1);
        };
        // C s_241_4: const #22112u : u32
        let s_241_4: u32 = 22112;
        // D s_241_5: read-reg s_241_4:struct
        let s_241_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_241_4 as isize);
            tracer.read_register(s_241_4 as isize, value);
            value
        };
        // C s_241_6: const #22112u : u32
        let s_241_6: u32 = 22112;
        // N s_241_7: write-reg s_241_6 <= s_241_5
        let s_241_7: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_241_6 as isize, s_241_5);
            tracer.write_register(s_241_6 as isize, s_241_5);
        };
        // N s_241_8: jump b240
        return block_240(state, tracer, fn_state);
    }
    fn block_242<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_242_0: const #101000u : u32
        let s_242_0: u32 = 101000;
        // D s_242_1: read-reg s_242_0:struct
        let s_242_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_242_0 as isize);
            tracer.read_register(s_242_0 as isize, value);
            value
        };
        // C s_242_2: const #101000u : u32
        let s_242_2: u32 = 101000;
        // N s_242_3: write-reg s_242_2 <= s_242_1
        let s_242_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_242_2 as isize, s_242_1);
            tracer.write_register(s_242_2 as isize, s_242_1);
        };
        // C s_242_4: const #101000u : u32
        let s_242_4: u32 = 101000;
        // D s_242_5: read-reg s_242_4:struct
        let s_242_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_242_4 as isize);
            tracer.read_register(s_242_4 as isize, value);
            value
        };
        // C s_242_6: const #101000u : u32
        let s_242_6: u32 = 101000;
        // N s_242_7: write-reg s_242_6 <= s_242_5
        let s_242_7: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_242_6 as isize, s_242_5);
            tracer.write_register(s_242_6 as isize, s_242_5);
        };
        // C s_242_8: const #22552u : u32
        let s_242_8: u32 = 22552;
        // D s_242_9: read-reg s_242_8:struct
        let s_242_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_242_8 as isize);
            tracer.read_register(s_242_8 as isize, value);
            value
        };
        // C s_242_10: const #22552u : u32
        let s_242_10: u32 = 22552;
        // N s_242_11: write-reg s_242_10 <= s_242_9
        let s_242_11: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_242_10 as isize, s_242_9);
            tracer.write_register(s_242_10 as isize, s_242_9);
        };
        // C s_242_12: const #100864u : u32
        let s_242_12: u32 = 100864;
        // D s_242_13: read-reg s_242_12:struct
        let s_242_13: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_242_12 as isize);
            tracer.read_register(s_242_12 as isize, value);
            value
        };
        // C s_242_14: const #100864u : u32
        let s_242_14: u32 = 100864;
        // N s_242_15: write-reg s_242_14 <= s_242_13
        let s_242_15: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_242_14 as isize, s_242_13);
            tracer.write_register(s_242_14 as isize, s_242_13);
        };
        // N s_242_16: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_243<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_243_0: const #103144u : u32
        let s_243_0: u32 = 103144;
        // D s_243_1: read-reg s_243_0:struct
        let s_243_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_243_0 as isize);
            tracer.read_register(s_243_0 as isize, value);
            value
        };
        // C s_243_2: const #103144u : u32
        let s_243_2: u32 = 103144;
        // N s_243_3: write-reg s_243_2 <= s_243_1
        let s_243_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_243_2 as isize, s_243_1);
            tracer.write_register(s_243_2 as isize, s_243_1);
        };
        // C s_243_4: const #424u : u32
        let s_243_4: u32 = 424;
        // D s_243_5: read-reg s_243_4:u8
        let s_243_5: u8 = {
            let value = state.read_register::<u8>(s_243_4 as isize);
            tracer.read_register(s_243_4 as isize, value);
            value
        };
        // C s_243_6: const #2u : u8
        let s_243_6: u8 = 2;
        // D s_243_7: cmp-lt s_243_5 s_243_6
        let s_243_7: bool = ((s_243_5) < (s_243_6));
        // D s_243_8: not s_243_7
        let s_243_8: bool = !s_243_7;
        // N s_243_9: branch s_243_8 b261 b244
        if s_243_8 {
            return block_261(state, tracer, fn_state);
        } else {
            return block_244(state, tracer, fn_state);
        };
    }
    fn block_244<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_244_0: const #0u : u8
        let s_244_0: bool = false;
        // D s_244_1: write-var gs#330101 <= s_244_0
        fn_state.gs_330101 = s_244_0;
        // N s_244_2: jump b245
        return block_245(state, tracer, fn_state);
    }
    fn block_245<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_245_0: read-var gs#330101:u8
        let s_245_0: bool = fn_state.gs_330101;
        // N s_245_1: branch s_245_0 b260 b246
        if s_245_0 {
            return block_260(state, tracer, fn_state);
        } else {
            return block_246(state, tracer, fn_state);
        };
    }
    fn block_246<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_246_0: jump b247
        return block_247(state, tracer, fn_state);
    }
    fn block_247<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_247_0: const #103144u : u32
        let s_247_0: u32 = 103144;
        // D s_247_1: read-reg s_247_0:struct
        let s_247_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_247_0 as isize);
            tracer.read_register(s_247_0 as isize, value);
            value
        };
        // C s_247_2: const #103144u : u32
        let s_247_2: u32 = 103144;
        // N s_247_3: write-reg s_247_2 <= s_247_1
        let s_247_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_247_2 as isize, s_247_1);
            tracer.write_register(s_247_2 as isize, s_247_1);
        };
        // C s_247_4: const #20112u : u32
        let s_247_4: u32 = 20112;
        // D s_247_5: read-reg s_247_4:struct
        let s_247_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_247_4 as isize);
            tracer.read_register(s_247_4 as isize, value);
            value
        };
        // C s_247_6: const #20112u : u32
        let s_247_6: u32 = 20112;
        // N s_247_7: write-reg s_247_6 <= s_247_5
        let s_247_7: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_247_6 as isize, s_247_5);
            tracer.write_register(s_247_6 as isize, s_247_5);
        };
        // C s_247_8: const #20416u : u32
        let s_247_8: u32 = 20416;
        // D s_247_9: read-reg s_247_8:struct
        let s_247_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_247_8 as isize);
            tracer.read_register(s_247_8 as isize, value);
            value
        };
        // C s_247_10: const #20416u : u32
        let s_247_10: u32 = 20416;
        // N s_247_11: write-reg s_247_10 <= s_247_9
        let s_247_11: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_247_10 as isize, s_247_9);
            tracer.write_register(s_247_10 as isize, s_247_9);
        };
        // C s_247_12: const #20112u : u32
        let s_247_12: u32 = 20112;
        // D s_247_13: read-reg s_247_12:struct
        let s_247_13: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_247_12 as isize);
            tracer.read_register(s_247_12 as isize, value);
            value
        };
        // C s_247_14: const #20112u : u32
        let s_247_14: u32 = 20112;
        // N s_247_15: write-reg s_247_14 <= s_247_13
        let s_247_15: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_247_14 as isize, s_247_13);
            tracer.write_register(s_247_14 as isize, s_247_13);
        };
        // C s_247_16: const #23360u : u32
        let s_247_16: u32 = 23360;
        // D s_247_17: read-reg s_247_16:u8
        let s_247_17: bool = {
            let value = state.read_register::<bool>(s_247_16 as isize);
            tracer.read_register(s_247_16 as isize, value);
            value
        };
        // N s_247_18: branch s_247_17 b259 b248
        if s_247_17 {
            return block_259(state, tracer, fn_state);
        } else {
            return block_248(state, tracer, fn_state);
        };
    }
    fn block_248<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_248_0: jump b249
        return block_249(state, tracer, fn_state);
    }
    fn block_249<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_249_0: const #() : ()
        let s_249_0: () = ();
        // S s_249_1: call Have52BitIPAAndPASpaceExt(s_249_0)
        let s_249_1: bool = Have52BitIPAAndPASpaceExt(state, tracer, s_249_0);
        // N s_249_2: branch s_249_1 b258 b250
        if s_249_1 {
            return block_258(state, tracer, fn_state);
        } else {
            return block_250(state, tracer, fn_state);
        };
    }
    fn block_250<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_250_0: jump b251
        return block_251(state, tracer, fn_state);
    }
    fn block_251<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_251_0: const #14600u : u32
        let s_251_0: u32 = 14600;
        // D s_251_1: read-reg s_251_0:struct
        let s_251_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_251_0 as isize);
            tracer.read_register(s_251_0 as isize, value);
            value
        };
        // C s_251_2: const #14600u : u32
        let s_251_2: u32 = 14600;
        // N s_251_3: write-reg s_251_2 <= s_251_1
        let s_251_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_251_2 as isize, s_251_1);
            tracer.write_register(s_251_2 as isize, s_251_1);
        };
        // C s_251_4: const #14864u : u32
        let s_251_4: u32 = 14864;
        // D s_251_5: read-reg s_251_4:struct
        let s_251_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_251_4 as isize);
            tracer.read_register(s_251_4 as isize, value);
            value
        };
        // C s_251_6: const #14864u : u32
        let s_251_6: u32 = 14864;
        // N s_251_7: write-reg s_251_6 <= s_251_5
        let s_251_7: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_251_6 as isize, s_251_5);
            tracer.write_register(s_251_6 as isize, s_251_5);
        };
        // C s_251_8: const #103144u : u32
        let s_251_8: u32 = 103144;
        // D s_251_9: read-reg s_251_8:struct
        let s_251_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_251_8 as isize);
            tracer.read_register(s_251_8 as isize, value);
            value
        };
        // C s_251_10: const #103144u : u32
        let s_251_10: u32 = 103144;
        // N s_251_11: write-reg s_251_10 <= s_251_9
        let s_251_11: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_251_10 as isize, s_251_9);
            tracer.write_register(s_251_10 as isize, s_251_9);
        };
        // C s_251_12: const #22552u : u32
        let s_251_12: u32 = 22552;
        // D s_251_13: read-reg s_251_12:struct
        let s_251_13: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_251_12 as isize);
            tracer.read_register(s_251_12 as isize, value);
            value
        };
        // C s_251_14: const #22552u : u32
        let s_251_14: u32 = 22552;
        // N s_251_15: write-reg s_251_14 <= s_251_13
        let s_251_15: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_251_14 as isize, s_251_13);
            tracer.write_register(s_251_14 as isize, s_251_13);
        };
        // C s_251_16: const #() : ()
        let s_251_16: () = ();
        // S s_251_17: call HaveFeatLS64_ACCDATA(s_251_16)
        let s_251_17: bool = HaveFeatLS64_ACCDATA(state, tracer, s_251_16);
        // N s_251_18: branch s_251_17 b257 b252
        if s_251_17 {
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
        // C s_252_0: const #() : ()
        let s_252_0: () = ();
        // S s_252_1: call HaveFeatLS64_V(s_252_0)
        let s_252_1: bool = HaveFeatLS64_V(state, tracer, s_252_0);
        // N s_252_2: branch s_252_1 b256 b253
        if s_252_1 {
            return block_256(state, tracer, fn_state);
        } else {
            return block_253(state, tracer, fn_state);
        };
    }
    fn block_253<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_253_0: const #() : ()
        let s_253_0: () = ();
        // S s_253_1: call HaveFeatLS64(s_253_0)
        let s_253_1: bool = HaveFeatLS64(state, tracer, s_253_0);
        // N s_253_2: branch s_253_1 b255 b254
        if s_253_1 {
            return block_255(state, tracer, fn_state);
        } else {
            return block_254(state, tracer, fn_state);
        };
    }
    fn block_254<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_254_0: const #14600u : u32
        let s_254_0: u32 = 14600;
        // D s_254_1: read-reg s_254_0:struct
        let s_254_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_254_0 as isize);
            tracer.read_register(s_254_0 as isize, value);
            value
        };
        // C s_254_2: const #14600u : u32
        let s_254_2: u32 = 14600;
        // N s_254_3: write-reg s_254_2 <= s_254_1
        let s_254_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_254_2 as isize, s_254_1);
            tracer.write_register(s_254_2 as isize, s_254_1);
        };
        // N s_254_4: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_255<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_255_0: const #14600u : u32
        let s_255_0: u32 = 14600;
        // D s_255_1: read-reg s_255_0:struct
        let s_255_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_255_0 as isize);
            tracer.read_register(s_255_0 as isize, value);
            value
        };
        // C s_255_2: const #14600u : u32
        let s_255_2: u32 = 14600;
        // N s_255_3: write-reg s_255_2 <= s_255_1
        let s_255_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_255_2 as isize, s_255_1);
            tracer.write_register(s_255_2 as isize, s_255_1);
        };
        // N s_255_4: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_256<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_256_0: const #14600u : u32
        let s_256_0: u32 = 14600;
        // D s_256_1: read-reg s_256_0:struct
        let s_256_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_256_0 as isize);
            tracer.read_register(s_256_0 as isize, value);
            value
        };
        // C s_256_2: const #14600u : u32
        let s_256_2: u32 = 14600;
        // N s_256_3: write-reg s_256_2 <= s_256_1
        let s_256_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_256_2 as isize, s_256_1);
            tracer.write_register(s_256_2 as isize, s_256_1);
        };
        // N s_256_4: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_257<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_257_0: const #14600u : u32
        let s_257_0: u32 = 14600;
        // D s_257_1: read-reg s_257_0:struct
        let s_257_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_257_0 as isize);
            tracer.read_register(s_257_0 as isize, value);
            value
        };
        // C s_257_2: const #14600u : u32
        let s_257_2: u32 = 14600;
        // N s_257_3: write-reg s_257_2 <= s_257_1
        let s_257_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_257_2 as isize, s_257_1);
            tracer.write_register(s_257_2 as isize, s_257_1);
        };
        // N s_257_4: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_258<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_258_0: const #14896u : u32
        let s_258_0: u32 = 14896;
        // D s_258_1: read-reg s_258_0:struct
        let s_258_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_258_0 as isize);
            tracer.read_register(s_258_0 as isize, value);
            value
        };
        // C s_258_2: const #14896u : u32
        let s_258_2: u32 = 14896;
        // N s_258_3: write-reg s_258_2 <= s_258_1
        let s_258_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_258_2 as isize, s_258_1);
            tracer.write_register(s_258_2 as isize, s_258_1);
        };
        // C s_258_4: const #14896u : u32
        let s_258_4: u32 = 14896;
        // D s_258_5: read-reg s_258_4:struct
        let s_258_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_258_4 as isize);
            tracer.read_register(s_258_4 as isize, value);
            value
        };
        // C s_258_6: const #14896u : u32
        let s_258_6: u32 = 14896;
        // N s_258_7: write-reg s_258_6 <= s_258_5
        let s_258_7: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_258_6 as isize, s_258_5);
            tracer.write_register(s_258_6 as isize, s_258_5);
        };
        // C s_258_8: const #14896u : u32
        let s_258_8: u32 = 14896;
        // D s_258_9: read-reg s_258_8:struct
        let s_258_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_258_8 as isize);
            tracer.read_register(s_258_8 as isize, value);
            value
        };
        // C s_258_10: const #14896u : u32
        let s_258_10: u32 = 14896;
        // N s_258_11: write-reg s_258_10 <= s_258_9
        let s_258_11: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_258_10 as isize, s_258_9);
            tracer.write_register(s_258_10 as isize, s_258_9);
        };
        // C s_258_12: const #14896u : u32
        let s_258_12: u32 = 14896;
        // D s_258_13: read-reg s_258_12:struct
        let s_258_13: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_258_12 as isize);
            tracer.read_register(s_258_12 as isize, value);
            value
        };
        // C s_258_14: const #14896u : u32
        let s_258_14: u32 = 14896;
        // N s_258_15: write-reg s_258_14 <= s_258_13
        let s_258_15: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_258_14 as isize, s_258_13);
            tracer.write_register(s_258_14 as isize, s_258_13);
        };
        // N s_258_16: jump b251
        return block_251(state, tracer, fn_state);
    }
    fn block_259<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_259_0: const #14864u : u32
        let s_259_0: u32 = 14864;
        // D s_259_1: read-reg s_259_0:struct
        let s_259_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_259_0 as isize);
            tracer.read_register(s_259_0 as isize, value);
            value
        };
        // C s_259_2: const #14864u : u32
        let s_259_2: u32 = 14864;
        // N s_259_3: write-reg s_259_2 <= s_259_1
        let s_259_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_259_2 as isize, s_259_1);
            tracer.write_register(s_259_2 as isize, s_259_1);
        };
        // N s_259_4: jump b249
        return block_249(state, tracer, fn_state);
    }
    fn block_260<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_260_0: const #64s : i
        let s_260_0: i128 = 64;
        // S s_260_1: call Zeros(s_260_0)
        let s_260_1: Bits = Zeros(state, tracer, s_260_0);
        // S s_260_2: cast reint s_260_1 -> u64
        let s_260_2: u64 = (s_260_1.value() as u64);
        // S s_260_3: call Mk_HCRX_EL2_Type(s_260_2)
        let s_260_3: ProductType5c790c8ef59cc8b2 = Mk_HCRX_EL2_Type(
            state,
            tracer,
            s_260_2,
        );
        // C s_260_4: const #22528u : u32
        let s_260_4: u32 = 22528;
        // N s_260_5: write-reg s_260_4 <= s_260_3
        let s_260_5: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_260_4 as isize, s_260_3);
            tracer.write_register(s_260_4 as isize, s_260_3);
        };
        // N s_260_6: jump b247
        return block_247(state, tracer, fn_state);
    }
    fn block_261<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_261_0: const #432u : u32
        let s_261_0: u32 = 432;
        // D s_261_1: read-reg s_261_0:u8
        let s_261_1: u8 = {
            let value = state.read_register::<u8>(s_261_0 as isize);
            tracer.read_register(s_261_0 as isize, value);
            value
        };
        // C s_261_2: const #2u : u8
        let s_261_2: u8 = 2;
        // D s_261_3: cmp-lt s_261_1 s_261_2
        let s_261_3: bool = ((s_261_1) < (s_261_2));
        // D s_261_4: write-var gs#330101 <= s_261_3
        fn_state.gs_330101 = s_261_3;
        // N s_261_5: jump b245
        return block_245(state, tracer, fn_state);
    }
    fn block_262<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_262_0: const #103144u : u32
        let s_262_0: u32 = 103144;
        // D s_262_1: read-reg s_262_0:struct
        let s_262_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_262_0 as isize);
            tracer.read_register(s_262_0 as isize, value);
            value
        };
        // C s_262_2: const #103144u : u32
        let s_262_2: u32 = 103144;
        // N s_262_3: write-reg s_262_2 <= s_262_1
        let s_262_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_262_2 as isize, s_262_1);
            tracer.write_register(s_262_2 as isize, s_262_1);
        };
        // N s_262_4: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_263<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_263_0: const #90608u : u32
        let s_263_0: u32 = 90608;
        // D s_263_1: read-reg s_263_0:struct
        let s_263_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_263_0 as isize);
            tracer.read_register(s_263_0 as isize, value);
            value
        };
        // C s_263_2: const #90608u : u32
        let s_263_2: u32 = 90608;
        // N s_263_3: write-reg s_263_2 <= s_263_1
        let s_263_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_263_2 as isize, s_263_1);
            tracer.write_register(s_263_2 as isize, s_263_1);
        };
        // C s_263_4: const #14600u : u32
        let s_263_4: u32 = 14600;
        // D s_263_5: read-reg s_263_4:struct
        let s_263_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_263_4 as isize);
            tracer.read_register(s_263_4 as isize, value);
            value
        };
        // C s_263_6: const #14600u : u32
        let s_263_6: u32 = 14600;
        // N s_263_7: write-reg s_263_6 <= s_263_5
        let s_263_7: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_263_6 as isize, s_263_5);
            tracer.write_register(s_263_6 as isize, s_263_5);
        };
        // C s_263_8: const #22552u : u32
        let s_263_8: u32 = 22552;
        // D s_263_9: read-reg s_263_8:struct
        let s_263_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_263_8 as isize);
            tracer.read_register(s_263_8 as isize, value);
            value
        };
        // C s_263_10: const #22552u : u32
        let s_263_10: u32 = 22552;
        // N s_263_11: write-reg s_263_10 <= s_263_9
        let s_263_11: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_263_10 as isize, s_263_9);
            tracer.write_register(s_263_10 as isize, s_263_9);
        };
        // C s_263_12: const #20416u : u32
        let s_263_12: u32 = 20416;
        // D s_263_13: read-reg s_263_12:struct
        let s_263_13: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_263_12 as isize);
            tracer.read_register(s_263_12 as isize, value);
            value
        };
        // C s_263_14: const #20416u : u32
        let s_263_14: u32 = 20416;
        // N s_263_15: write-reg s_263_14 <= s_263_13
        let s_263_15: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_263_14 as isize, s_263_13);
            tracer.write_register(s_263_14 as isize, s_263_13);
        };
        // C s_263_16: const #90600u : u32
        let s_263_16: u32 = 90600;
        // D s_263_17: read-reg s_263_16:struct
        let s_263_17: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_263_16 as isize);
            tracer.read_register(s_263_16 as isize, value);
            value
        };
        // C s_263_18: const #90600u : u32
        let s_263_18: u32 = 90600;
        // N s_263_19: write-reg s_263_18 <= s_263_17
        let s_263_19: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_263_18 as isize, s_263_17);
            tracer.write_register(s_263_18 as isize, s_263_17);
        };
        // C s_263_20: const #90608u : u32
        let s_263_20: u32 = 90608;
        // D s_263_21: read-reg s_263_20:struct
        let s_263_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_263_20 as isize);
            tracer.read_register(s_263_20 as isize, value);
            value
        };
        // C s_263_22: const #90608u : u32
        let s_263_22: u32 = 90608;
        // N s_263_23: write-reg s_263_22 <= s_263_21
        let s_263_23: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_263_22 as isize, s_263_21);
            tracer.write_register(s_263_22 as isize, s_263_21);
        };
        // C s_263_24: const #() : ()
        let s_263_24: () = ();
        // S s_263_25: call HaveMTE2Ext(s_263_24)
        let s_263_25: bool = HaveMTE2Ext(state, tracer, s_263_24);
        // N s_263_26: branch s_263_25 b266 b264
        if s_263_25 {
            return block_266(state, tracer, fn_state);
        } else {
            return block_264(state, tracer, fn_state);
        };
    }
    fn block_264<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_264_0: jump b265
        return block_265(state, tracer, fn_state);
    }
    fn block_265<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_265_0: const #20112u : u32
        let s_265_0: u32 = 20112;
        // D s_265_1: read-reg s_265_0:struct
        let s_265_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_265_0 as isize);
            tracer.read_register(s_265_0 as isize, value);
            value
        };
        // C s_265_2: const #20112u : u32
        let s_265_2: u32 = 20112;
        // N s_265_3: write-reg s_265_2 <= s_265_1
        let s_265_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_265_2 as isize, s_265_1);
            tracer.write_register(s_265_2 as isize, s_265_1);
        };
        // C s_265_4: const #22800u : u32
        let s_265_4: u32 = 22800;
        // D s_265_5: read-reg s_265_4:struct
        let s_265_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_265_4 as isize);
            tracer.read_register(s_265_4 as isize, value);
            value
        };
        // C s_265_6: const #22800u : u32
        let s_265_6: u32 = 22800;
        // N s_265_7: write-reg s_265_6 <= s_265_5
        let s_265_7: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_265_6 as isize, s_265_5);
            tracer.write_register(s_265_6 as isize, s_265_5);
        };
        // C s_265_8: const #90408u : u32
        let s_265_8: u32 = 90408;
        // D s_265_9: read-reg s_265_8:struct
        let s_265_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_265_8 as isize);
            tracer.read_register(s_265_8 as isize, value);
            value
        };
        // C s_265_10: const #90408u : u32
        let s_265_10: u32 = 90408;
        // N s_265_11: write-reg s_265_10 <= s_265_9
        let s_265_11: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_265_10 as isize, s_265_9);
            tracer.write_register(s_265_10 as isize, s_265_9);
        };
        // C s_265_12: const #90408u : u32
        let s_265_12: u32 = 90408;
        // D s_265_13: read-reg s_265_12:struct
        let s_265_13: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_265_12 as isize);
            tracer.read_register(s_265_12 as isize, value);
            value
        };
        // C s_265_14: const #90408u : u32
        let s_265_14: u32 = 90408;
        // N s_265_15: write-reg s_265_14 <= s_265_13
        let s_265_15: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_265_14 as isize, s_265_13);
            tracer.write_register(s_265_14 as isize, s_265_13);
        };
        // N s_265_16: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_266<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_266_0: const #22552u : u32
        let s_266_0: u32 = 22552;
        // D s_266_1: read-reg s_266_0:struct
        let s_266_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_266_0 as isize);
            tracer.read_register(s_266_0 as isize, value);
            value
        };
        // C s_266_2: const #22552u : u32
        let s_266_2: u32 = 22552;
        // N s_266_3: write-reg s_266_2 <= s_266_1
        let s_266_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_266_2 as isize, s_266_1);
            tracer.write_register(s_266_2 as isize, s_266_1);
        };
        // N s_266_4: jump b265
        return block_265(state, tracer, fn_state);
    }
    fn block_267<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_267_0: const #14600u : u32
        let s_267_0: u32 = 14600;
        // D s_267_1: read-reg s_267_0:struct
        let s_267_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_267_0 as isize);
            tracer.read_register(s_267_0 as isize, value);
            value
        };
        // C s_267_2: const #14600u : u32
        let s_267_2: u32 = 14600;
        // N s_267_3: write-reg s_267_2 <= s_267_1
        let s_267_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_267_2 as isize, s_267_1);
            tracer.write_register(s_267_2 as isize, s_267_1);
        };
        // C s_267_4: const #90600u : u32
        let s_267_4: u32 = 90600;
        // D s_267_5: read-reg s_267_4:struct
        let s_267_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_267_4 as isize);
            tracer.read_register(s_267_4 as isize, value);
            value
        };
        // C s_267_6: const #90600u : u32
        let s_267_6: u32 = 90600;
        // N s_267_7: write-reg s_267_6 <= s_267_5
        let s_267_7: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_267_6 as isize, s_267_5);
            tracer.write_register(s_267_6 as isize, s_267_5);
        };
        // C s_267_8: const #90600u : u32
        let s_267_8: u32 = 90600;
        // D s_267_9: read-reg s_267_8:struct
        let s_267_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_267_8 as isize);
            tracer.read_register(s_267_8 as isize, value);
            value
        };
        // C s_267_10: const #90600u : u32
        let s_267_10: u32 = 90600;
        // N s_267_11: write-reg s_267_10 <= s_267_9
        let s_267_11: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_267_10 as isize, s_267_9);
            tracer.write_register(s_267_10 as isize, s_267_9);
        };
        // C s_267_12: const #90600u : u32
        let s_267_12: u32 = 90600;
        // D s_267_13: read-reg s_267_12:struct
        let s_267_13: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_267_12 as isize);
            tracer.read_register(s_267_12 as isize, value);
            value
        };
        // C s_267_14: const #90600u : u32
        let s_267_14: u32 = 90600;
        // N s_267_15: write-reg s_267_14 <= s_267_13
        let s_267_15: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_267_14 as isize, s_267_13);
            tracer.write_register(s_267_14 as isize, s_267_13);
        };
        // C s_267_16: const #90600u : u32
        let s_267_16: u32 = 90600;
        // D s_267_17: read-reg s_267_16:struct
        let s_267_17: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_267_16 as isize);
            tracer.read_register(s_267_16 as isize, value);
            value
        };
        // C s_267_18: const #90600u : u32
        let s_267_18: u32 = 90600;
        // N s_267_19: write-reg s_267_18 <= s_267_17
        let s_267_19: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_267_18 as isize, s_267_17);
            tracer.write_register(s_267_18 as isize, s_267_17);
        };
        // C s_267_20: const #90600u : u32
        let s_267_20: u32 = 90600;
        // D s_267_21: read-reg s_267_20:struct
        let s_267_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_267_20 as isize);
            tracer.read_register(s_267_20 as isize, value);
            value
        };
        // C s_267_22: const #90600u : u32
        let s_267_22: u32 = 90600;
        // N s_267_23: write-reg s_267_22 <= s_267_21
        let s_267_23: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_267_22 as isize, s_267_21);
            tracer.write_register(s_267_22 as isize, s_267_21);
        };
        // C s_267_24: const #20416u : u32
        let s_267_24: u32 = 20416;
        // D s_267_25: read-reg s_267_24:struct
        let s_267_25: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_267_24 as isize);
            tracer.read_register(s_267_24 as isize, value);
            value
        };
        // C s_267_26: const #20416u : u32
        let s_267_26: u32 = 20416;
        // N s_267_27: write-reg s_267_26 <= s_267_25
        let s_267_27: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_267_26 as isize, s_267_25);
            tracer.write_register(s_267_26 as isize, s_267_25);
        };
        // C s_267_28: const #424u : u32
        let s_267_28: u32 = 424;
        // D s_267_29: read-reg s_267_28:u8
        let s_267_29: u8 = {
            let value = state.read_register::<u8>(s_267_28 as isize);
            tracer.read_register(s_267_28 as isize, value);
            value
        };
        // C s_267_30: const #2u : u8
        let s_267_30: u8 = 2;
        // D s_267_31: cmp-lt s_267_29 s_267_30
        let s_267_31: bool = ((s_267_29) < (s_267_30));
        // D s_267_32: not s_267_31
        let s_267_32: bool = !s_267_31;
        // N s_267_33: branch s_267_32 b291 b268
        if s_267_32 {
            return block_291(state, tracer, fn_state);
        } else {
            return block_268(state, tracer, fn_state);
        };
    }
    fn block_268<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_268_0: const #0u : u8
        let s_268_0: bool = false;
        // D s_268_1: write-var gs#330245 <= s_268_0
        fn_state.gs_330245 = s_268_0;
        // N s_268_2: jump b269
        return block_269(state, tracer, fn_state);
    }
    fn block_269<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_269_0: read-var gs#330245:u8
        let s_269_0: bool = fn_state.gs_330245;
        // N s_269_1: branch s_269_0 b290 b270
        if s_269_0 {
            return block_290(state, tracer, fn_state);
        } else {
            return block_270(state, tracer, fn_state);
        };
    }
    fn block_270<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_270_0: const #20416u : u32
        let s_270_0: u32 = 20416;
        // D s_270_1: read-reg s_270_0:struct
        let s_270_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_270_0 as isize);
            tracer.read_register(s_270_0 as isize, value);
            value
        };
        // D s_270_2: call _get_ID_DFR0_EL1_Type_CopDbg(s_270_1)
        let s_270_2: u8 = u_get_ID_DFR0_EL1_Type_CopDbg(state, tracer, s_270_1);
        // C s_270_3: const #20416u : u32
        let s_270_3: u32 = 20416;
        // D s_270_4: read-reg s_270_3:struct
        let s_270_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_270_3 as isize);
            tracer.read_register(s_270_3 as isize, value);
            value
        };
        // C s_270_5: const #20416u : u32
        let s_270_5: u32 = 20416;
        // N s_270_6: write-reg s_270_5 <= s_270_4
        let s_270_6: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_270_5 as isize, s_270_4);
            tracer.write_register(s_270_5 as isize, s_270_4);
        };
        // N s_270_7: jump b271
        return block_271(state, tracer, fn_state);
    }
    fn block_271<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_271_0: const #101824u : u32
        let s_271_0: u32 = 101824;
        // D s_271_1: read-reg s_271_0:struct
        let s_271_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_271_0 as isize);
            tracer.read_register(s_271_0 as isize, value);
            value
        };
        // C s_271_2: const #101824u : u32
        let s_271_2: u32 = 101824;
        // N s_271_3: write-reg s_271_2 <= s_271_1
        let s_271_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_271_2 as isize, s_271_1);
            tracer.write_register(s_271_2 as isize, s_271_1);
        };
        // C s_271_4: const #90608u : u32
        let s_271_4: u32 = 90608;
        // D s_271_5: read-reg s_271_4:struct
        let s_271_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_271_4 as isize);
            tracer.read_register(s_271_4 as isize, value);
            value
        };
        // C s_271_6: const #90608u : u32
        let s_271_6: u32 = 90608;
        // N s_271_7: write-reg s_271_6 <= s_271_5
        let s_271_7: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_271_6 as isize, s_271_5);
            tracer.write_register(s_271_6 as isize, s_271_5);
        };
        // C s_271_8: const #90608u : u32
        let s_271_8: u32 = 90608;
        // D s_271_9: read-reg s_271_8:struct
        let s_271_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_271_8 as isize);
            tracer.read_register(s_271_8 as isize, value);
            value
        };
        // C s_271_10: const #90608u : u32
        let s_271_10: u32 = 90608;
        // N s_271_11: write-reg s_271_10 <= s_271_9
        let s_271_11: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_271_10 as isize, s_271_9);
            tracer.write_register(s_271_10 as isize, s_271_9);
        };
        // C s_271_12: const #90608u : u32
        let s_271_12: u32 = 90608;
        // D s_271_13: read-reg s_271_12:struct
        let s_271_13: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_271_12 as isize);
            tracer.read_register(s_271_12 as isize, value);
            value
        };
        // C s_271_14: const #90608u : u32
        let s_271_14: u32 = 90608;
        // N s_271_15: write-reg s_271_14 <= s_271_13
        let s_271_15: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_271_14 as isize, s_271_13);
            tracer.write_register(s_271_14 as isize, s_271_13);
        };
        // C s_271_16: const #90408u : u32
        let s_271_16: u32 = 90408;
        // D s_271_17: read-reg s_271_16:struct
        let s_271_17: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_271_16 as isize);
            tracer.read_register(s_271_16 as isize, value);
            value
        };
        // C s_271_18: const #90408u : u32
        let s_271_18: u32 = 90408;
        // N s_271_19: write-reg s_271_18 <= s_271_17
        let s_271_19: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_271_18 as isize, s_271_17);
            tracer.write_register(s_271_18 as isize, s_271_17);
        };
        // C s_271_20: const #101824u : u32
        let s_271_20: u32 = 101824;
        // D s_271_21: read-reg s_271_20:struct
        let s_271_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_271_20 as isize);
            tracer.read_register(s_271_20 as isize, value);
            value
        };
        // C s_271_22: const #101824u : u32
        let s_271_22: u32 = 101824;
        // N s_271_23: write-reg s_271_22 <= s_271_21
        let s_271_23: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_271_22 as isize, s_271_21);
            tracer.write_register(s_271_22 as isize, s_271_21);
        };
        // C s_271_24: const #3s : i
        let s_271_24: i128 = 3;
        // C s_271_25: const #0s : i
        let s_271_25: i128 = 0;
        // C s_271_26: const #15624u : u32
        let s_271_26: u32 = 15624;
        // D s_271_27: read-reg s_271_26:i
        let s_271_27: i128 = {
            let value = state.read_register::<i128>(s_271_26 as isize);
            tracer.read_register(s_271_26 as isize, value);
            value
        };
        // D s_271_28: call integer_subrange(s_271_27, s_271_24, s_271_25)
        let s_271_28: Bits = integer_subrange(
            state,
            tracer,
            s_271_27,
            s_271_24,
            s_271_25,
        );
        // C s_271_29: const #90600u : u32
        let s_271_29: u32 = 90600;
        // D s_271_30: read-reg s_271_29:struct
        let s_271_30: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_271_29 as isize);
            tracer.read_register(s_271_29 as isize, value);
            value
        };
        // C s_271_31: const #90600u : u32
        let s_271_31: u32 = 90600;
        // N s_271_32: write-reg s_271_31 <= s_271_30
        let s_271_32: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_271_31 as isize, s_271_30);
            tracer.write_register(s_271_31 as isize, s_271_30);
        };
        // C s_271_33: const #77u : u32
        let s_271_33: u32 = 77;
        // S s_271_34: call IsFeatureImplemented(s_271_33)
        let s_271_34: bool = IsFeatureImplemented(state, tracer, s_271_33);
        // N s_271_35: branch s_271_34 b289 b272
        if s_271_34 {
            return block_289(state, tracer, fn_state);
        } else {
            return block_272(state, tracer, fn_state);
        };
    }
    fn block_272<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_272_0: jump b273
        return block_273(state, tracer, fn_state);
    }
    fn block_273<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_273_0: const #76u : u32
        let s_273_0: u32 = 76;
        // S s_273_1: call IsFeatureImplemented(s_273_0)
        let s_273_1: bool = IsFeatureImplemented(state, tracer, s_273_0);
        // N s_273_2: branch s_273_1 b288 b274
        if s_273_1 {
            return block_288(state, tracer, fn_state);
        } else {
            return block_274(state, tracer, fn_state);
        };
    }
    fn block_274<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_274_0: jump b275
        return block_275(state, tracer, fn_state);
    }
    fn block_275<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_275_0: const #78u : u32
        let s_275_0: u32 = 78;
        // S s_275_1: call IsFeatureImplemented(s_275_0)
        let s_275_1: bool = IsFeatureImplemented(state, tracer, s_275_0);
        // N s_275_2: branch s_275_1 b287 b276
        if s_275_1 {
            return block_287(state, tracer, fn_state);
        } else {
            return block_276(state, tracer, fn_state);
        };
    }
    fn block_276<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_276_0: jump b277
        return block_277(state, tracer, fn_state);
    }
    fn block_277<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_277_0: const #79u : u32
        let s_277_0: u32 = 79;
        // S s_277_1: call IsFeatureImplemented(s_277_0)
        let s_277_1: bool = IsFeatureImplemented(state, tracer, s_277_0);
        // N s_277_2: branch s_277_1 b286 b278
        if s_277_1 {
            return block_286(state, tracer, fn_state);
        } else {
            return block_278(state, tracer, fn_state);
        };
    }
    fn block_278<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_278_0: jump b279
        return block_279(state, tracer, fn_state);
    }
    fn block_279<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_279_0: const #() : ()
        let s_279_0: () = ();
        // S s_279_1: call HavePMUv3p4(s_279_0)
        let s_279_1: bool = HavePMUv3p4(state, tracer, s_279_0);
        // N s_279_2: branch s_279_1 b282 b280
        if s_279_1 {
            return block_282(state, tracer, fn_state);
        } else {
            return block_280(state, tracer, fn_state);
        };
    }
    fn block_280<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_280_0: jump b281
        return block_281(state, tracer, fn_state);
    }
    fn block_281<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_281_0: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_282<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_282_0: const #() : ()
        let s_282_0: () = ();
        // S s_282_1: call HavePMUv3TH(s_282_0)
        let s_282_1: bool = HavePMUv3TH(state, tracer, s_282_0);
        // N s_282_2: branch s_282_1 b285 b283
        if s_282_1 {
            return block_285(state, tracer, fn_state);
        } else {
            return block_283(state, tracer, fn_state);
        };
    }
    fn block_283<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_283_0: jump b284
        return block_284(state, tracer, fn_state);
    }
    fn block_284<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_284_0: const #20432u : u32
        let s_284_0: u32 = 20432;
        // D s_284_1: read-reg s_284_0:struct
        let s_284_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_284_0 as isize);
            tracer.read_register(s_284_0 as isize, value);
            value
        };
        // C s_284_2: const #20432u : u32
        let s_284_2: u32 = 20432;
        // N s_284_3: write-reg s_284_2 <= s_284_1
        let s_284_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_284_2 as isize, s_284_1);
            tracer.write_register(s_284_2 as isize, s_284_1);
        };
        // C s_284_4: const #20432u : u32
        let s_284_4: u32 = 20432;
        // D s_284_5: read-reg s_284_4:struct
        let s_284_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_284_4 as isize);
            tracer.read_register(s_284_4 as isize, value);
            value
        };
        // C s_284_6: const #20432u : u32
        let s_284_6: u32 = 20432;
        // N s_284_7: write-reg s_284_6 <= s_284_5
        let s_284_7: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_284_6 as isize, s_284_5);
            tracer.write_register(s_284_6 as isize, s_284_5);
        };
        // N s_284_8: jump b281
        return block_281(state, tracer, fn_state);
    }
    fn block_285<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_285_0: const #20432u : u32
        let s_285_0: u32 = 20432;
        // D s_285_1: read-reg s_285_0:struct
        let s_285_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_285_0 as isize);
            tracer.read_register(s_285_0 as isize, value);
            value
        };
        // C s_285_2: const #20432u : u32
        let s_285_2: u32 = 20432;
        // N s_285_3: write-reg s_285_2 <= s_285_1
        let s_285_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_285_2 as isize, s_285_1);
            tracer.write_register(s_285_2 as isize, s_285_1);
        };
        // N s_285_4: jump b284
        return block_284(state, tracer, fn_state);
    }
    fn block_286<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_286_0: const #90608u : u32
        let s_286_0: u32 = 90608;
        // D s_286_1: read-reg s_286_0:struct
        let s_286_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_286_0 as isize);
            tracer.read_register(s_286_0 as isize, value);
            value
        };
        // C s_286_2: const #90608u : u32
        let s_286_2: u32 = 90608;
        // N s_286_3: write-reg s_286_2 <= s_286_1
        let s_286_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_286_2 as isize, s_286_1);
            tracer.write_register(s_286_2 as isize, s_286_1);
        };
        // N s_286_4: jump b279
        return block_279(state, tracer, fn_state);
    }
    fn block_287<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_287_0: const #90608u : u32
        let s_287_0: u32 = 90608;
        // D s_287_1: read-reg s_287_0:struct
        let s_287_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_287_0 as isize);
            tracer.read_register(s_287_0 as isize, value);
            value
        };
        // C s_287_2: const #90608u : u32
        let s_287_2: u32 = 90608;
        // N s_287_3: write-reg s_287_2 <= s_287_1
        let s_287_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_287_2 as isize, s_287_1);
            tracer.write_register(s_287_2 as isize, s_287_1);
        };
        // N s_287_4: jump b277
        return block_277(state, tracer, fn_state);
    }
    fn block_288<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_288_0: const #90608u : u32
        let s_288_0: u32 = 90608;
        // D s_288_1: read-reg s_288_0:struct
        let s_288_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_288_0 as isize);
            tracer.read_register(s_288_0 as isize, value);
            value
        };
        // C s_288_2: const #90608u : u32
        let s_288_2: u32 = 90608;
        // N s_288_3: write-reg s_288_2 <= s_288_1
        let s_288_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_288_2 as isize, s_288_1);
            tracer.write_register(s_288_2 as isize, s_288_1);
        };
        // N s_288_4: jump b275
        return block_275(state, tracer, fn_state);
    }
    fn block_289<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_289_0: const #90608u : u32
        let s_289_0: u32 = 90608;
        // D s_289_1: read-reg s_289_0:struct
        let s_289_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_289_0 as isize);
            tracer.read_register(s_289_0 as isize, value);
            value
        };
        // C s_289_2: const #90608u : u32
        let s_289_2: u32 = 90608;
        // N s_289_3: write-reg s_289_2 <= s_289_1
        let s_289_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_289_2 as isize, s_289_1);
            tracer.write_register(s_289_2 as isize, s_289_1);
        };
        // N s_289_4: jump b273
        return block_273(state, tracer, fn_state);
    }
    fn block_290<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_290_0: const #20416u : u32
        let s_290_0: u32 = 20416;
        // D s_290_1: read-reg s_290_0:struct
        let s_290_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_290_0 as isize);
            tracer.read_register(s_290_0 as isize, value);
            value
        };
        // C s_290_2: const #20416u : u32
        let s_290_2: u32 = 20416;
        // N s_290_3: write-reg s_290_2 <= s_290_1
        let s_290_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_290_2 as isize, s_290_1);
            tracer.write_register(s_290_2 as isize, s_290_1);
        };
        // N s_290_4: jump b271
        return block_271(state, tracer, fn_state);
    }
    fn block_291<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_291_0: const #20920u : u32
        let s_291_0: u32 = 20920;
        // D s_291_1: read-reg s_291_0:struct
        let s_291_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_291_0 as isize);
            tracer.read_register(s_291_0 as isize, value);
            value
        };
        // D s_291_2: call _get_SCR_Type_NS(s_291_1)
        let s_291_2: bool = u_get_SCR_Type_NS(state, tracer, s_291_1);
        // D s_291_3: cast zx s_291_2 -> bv
        let s_291_3: Bits = Bits::new(s_291_2 as u128, 1u16);
        // C s_291_4: const #1u : u8
        let s_291_4: bool = true;
        // C s_291_5: cast zx s_291_4 -> bv
        let s_291_5: Bits = Bits::new(s_291_4 as u128, 1u16);
        // D s_291_6: cmp-eq s_291_3 s_291_5
        let s_291_6: bool = ((s_291_3) == (s_291_5));
        // D s_291_7: write-var gs#330245 <= s_291_6
        fn_state.gs_330245 = s_291_6;
        // N s_291_8: jump b269
        return block_269(state, tracer, fn_state);
    }
    fn block_292<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_292_0: const #96u : u32
        let s_292_0: u32 = 96;
        // S s_292_1: call IsFeatureImplemented(s_292_0)
        let s_292_1: bool = IsFeatureImplemented(state, tracer, s_292_0);
        // N s_292_2: branch s_292_1 b301 b293
        if s_292_1 {
            return block_301(state, tracer, fn_state);
        } else {
            return block_293(state, tracer, fn_state);
        };
    }
    fn block_293<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_293_0: jump b294
        return block_294(state, tracer, fn_state);
    }
    fn block_294<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_294_0: const #97u : u32
        let s_294_0: u32 = 97;
        // S s_294_1: call IsFeatureImplemented(s_294_0)
        let s_294_1: bool = IsFeatureImplemented(state, tracer, s_294_0);
        // N s_294_2: branch s_294_1 b300 b295
        if s_294_1 {
            return block_300(state, tracer, fn_state);
        } else {
            return block_295(state, tracer, fn_state);
        };
    }
    fn block_295<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_295_0: jump b296
        return block_296(state, tracer, fn_state);
    }
    fn block_296<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_296_0: const #87u : u32
        let s_296_0: u32 = 87;
        // S s_296_1: call IsFeatureImplemented(s_296_0)
        let s_296_1: bool = IsFeatureImplemented(state, tracer, s_296_0);
        // N s_296_2: branch s_296_1 b299 b297
        if s_296_1 {
            return block_299(state, tracer, fn_state);
        } else {
            return block_297(state, tracer, fn_state);
        };
    }
    fn block_297<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_297_0: const #14864u : u32
        let s_297_0: u32 = 14864;
        // D s_297_1: read-reg s_297_0:struct
        let s_297_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_297_0 as isize);
            tracer.read_register(s_297_0 as isize, value);
            value
        };
        // C s_297_2: const #14864u : u32
        let s_297_2: u32 = 14864;
        // N s_297_3: write-reg s_297_2 <= s_297_1
        let s_297_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_297_2 as isize, s_297_1);
            tracer.write_register(s_297_2 as isize, s_297_1);
        };
        // N s_297_4: jump b298
        return block_298(state, tracer, fn_state);
    }
    fn block_298<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_298_0: const #14600u : u32
        let s_298_0: u32 = 14600;
        // D s_298_1: read-reg s_298_0:struct
        let s_298_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_298_0 as isize);
            tracer.read_register(s_298_0 as isize, value);
            value
        };
        // C s_298_2: const #14600u : u32
        let s_298_2: u32 = 14600;
        // N s_298_3: write-reg s_298_2 <= s_298_1
        let s_298_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_298_2 as isize, s_298_1);
            tracer.write_register(s_298_2 as isize, s_298_1);
        };
        // C s_298_4: const #14600u : u32
        let s_298_4: u32 = 14600;
        // D s_298_5: read-reg s_298_4:struct
        let s_298_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_298_4 as isize);
            tracer.read_register(s_298_4 as isize, value);
            value
        };
        // C s_298_6: const #14600u : u32
        let s_298_6: u32 = 14600;
        // N s_298_7: write-reg s_298_6 <= s_298_5
        let s_298_7: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_298_6 as isize, s_298_5);
            tracer.write_register(s_298_6 as isize, s_298_5);
        };
        // C s_298_8: const #90408u : u32
        let s_298_8: u32 = 90408;
        // D s_298_9: read-reg s_298_8:struct
        let s_298_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_298_8 as isize);
            tracer.read_register(s_298_8 as isize, value);
            value
        };
        // C s_298_10: const #90408u : u32
        let s_298_10: u32 = 90408;
        // N s_298_11: write-reg s_298_10 <= s_298_9
        let s_298_11: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_298_10 as isize, s_298_9);
            tracer.write_register(s_298_10 as isize, s_298_9);
        };
        // C s_298_12: const #90312u : u32
        let s_298_12: u32 = 90312;
        // D s_298_13: read-reg s_298_12:struct
        let s_298_13: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_298_12 as isize);
            tracer.read_register(s_298_12 as isize, value);
            value
        };
        // C s_298_14: const #90312u : u32
        let s_298_14: u32 = 90312;
        // N s_298_15: write-reg s_298_14 <= s_298_13
        let s_298_15: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_298_14 as isize, s_298_13);
            tracer.write_register(s_298_14 as isize, s_298_13);
        };
        // C s_298_16: const #90600u : u32
        let s_298_16: u32 = 90600;
        // D s_298_17: read-reg s_298_16:struct
        let s_298_17: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_298_16 as isize);
            tracer.read_register(s_298_16 as isize, value);
            value
        };
        // C s_298_18: const #90600u : u32
        let s_298_18: u32 = 90600;
        // N s_298_19: write-reg s_298_18 <= s_298_17
        let s_298_19: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_298_18 as isize, s_298_17);
            tracer.write_register(s_298_18 as isize, s_298_17);
        };
        // C s_298_20: const #20416u : u32
        let s_298_20: u32 = 20416;
        // D s_298_21: read-reg s_298_20:struct
        let s_298_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_298_20 as isize);
            tracer.read_register(s_298_20 as isize, value);
            value
        };
        // C s_298_22: const #20416u : u32
        let s_298_22: u32 = 20416;
        // N s_298_23: write-reg s_298_22 <= s_298_21
        let s_298_23: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_298_22 as isize, s_298_21);
            tracer.write_register(s_298_22 as isize, s_298_21);
        };
        // C s_298_24: const #90600u : u32
        let s_298_24: u32 = 90600;
        // D s_298_25: read-reg s_298_24:struct
        let s_298_25: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_298_24 as isize);
            tracer.read_register(s_298_24 as isize, value);
            value
        };
        // C s_298_26: const #90600u : u32
        let s_298_26: u32 = 90600;
        // N s_298_27: write-reg s_298_26 <= s_298_25
        let s_298_27: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_298_26 as isize, s_298_25);
            tracer.write_register(s_298_26 as isize, s_298_25);
        };
        // C s_298_28: const #14600u : u32
        let s_298_28: u32 = 14600;
        // D s_298_29: read-reg s_298_28:struct
        let s_298_29: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_298_28 as isize);
            tracer.read_register(s_298_28 as isize, value);
            value
        };
        // C s_298_30: const #14600u : u32
        let s_298_30: u32 = 14600;
        // N s_298_31: write-reg s_298_30 <= s_298_29
        let s_298_31: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_298_30 as isize, s_298_29);
            tracer.write_register(s_298_30 as isize, s_298_29);
        };
        // N s_298_32: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_299<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_299_0: const #14864u : u32
        let s_299_0: u32 = 14864;
        // D s_299_1: read-reg s_299_0:struct
        let s_299_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_299_0 as isize);
            tracer.read_register(s_299_0 as isize, value);
            value
        };
        // C s_299_2: const #14864u : u32
        let s_299_2: u32 = 14864;
        // N s_299_3: write-reg s_299_2 <= s_299_1
        let s_299_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_299_2 as isize, s_299_1);
            tracer.write_register(s_299_2 as isize, s_299_1);
        };
        // N s_299_4: jump b298
        return block_298(state, tracer, fn_state);
    }
    fn block_300<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_300_0: const #14864u : u32
        let s_300_0: u32 = 14864;
        // D s_300_1: read-reg s_300_0:struct
        let s_300_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_300_0 as isize);
            tracer.read_register(s_300_0 as isize, value);
            value
        };
        // C s_300_2: const #14864u : u32
        let s_300_2: u32 = 14864;
        // N s_300_3: write-reg s_300_2 <= s_300_1
        let s_300_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_300_2 as isize, s_300_1);
            tracer.write_register(s_300_2 as isize, s_300_1);
        };
        // C s_300_4: const #14864u : u32
        let s_300_4: u32 = 14864;
        // D s_300_5: read-reg s_300_4:struct
        let s_300_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_300_4 as isize);
            tracer.read_register(s_300_4 as isize, value);
            value
        };
        // C s_300_6: const #14864u : u32
        let s_300_6: u32 = 14864;
        // N s_300_7: write-reg s_300_6 <= s_300_5
        let s_300_7: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_300_6 as isize, s_300_5);
            tracer.write_register(s_300_6 as isize, s_300_5);
        };
        // C s_300_8: const #14600u : u32
        let s_300_8: u32 = 14600;
        // D s_300_9: read-reg s_300_8:struct
        let s_300_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_300_8 as isize);
            tracer.read_register(s_300_8 as isize, value);
            value
        };
        // C s_300_10: const #14600u : u32
        let s_300_10: u32 = 14600;
        // N s_300_11: write-reg s_300_10 <= s_300_9
        let s_300_11: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_300_10 as isize, s_300_9);
            tracer.write_register(s_300_10 as isize, s_300_9);
        };
        // C s_300_12: const #14600u : u32
        let s_300_12: u32 = 14600;
        // D s_300_13: read-reg s_300_12:struct
        let s_300_13: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_300_12 as isize);
            tracer.read_register(s_300_12 as isize, value);
            value
        };
        // C s_300_14: const #14600u : u32
        let s_300_14: u32 = 14600;
        // N s_300_15: write-reg s_300_14 <= s_300_13
        let s_300_15: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_300_14 as isize, s_300_13);
            tracer.write_register(s_300_14 as isize, s_300_13);
        };
        // N s_300_16: jump b296
        return block_296(state, tracer, fn_state);
    }
    fn block_301<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_301_0: const #14864u : u32
        let s_301_0: u32 = 14864;
        // D s_301_1: read-reg s_301_0:struct
        let s_301_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_301_0 as isize);
            tracer.read_register(s_301_0 as isize, value);
            value
        };
        // C s_301_2: const #14864u : u32
        let s_301_2: u32 = 14864;
        // N s_301_3: write-reg s_301_2 <= s_301_1
        let s_301_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_301_2 as isize, s_301_1);
            tracer.write_register(s_301_2 as isize, s_301_1);
        };
        // C s_301_4: const #14864u : u32
        let s_301_4: u32 = 14864;
        // D s_301_5: read-reg s_301_4:struct
        let s_301_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_301_4 as isize);
            tracer.read_register(s_301_4 as isize, value);
            value
        };
        // C s_301_6: const #14864u : u32
        let s_301_6: u32 = 14864;
        // N s_301_7: write-reg s_301_6 <= s_301_5
        let s_301_7: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_301_6 as isize, s_301_5);
            tracer.write_register(s_301_6 as isize, s_301_5);
        };
        // C s_301_8: const #14600u : u32
        let s_301_8: u32 = 14600;
        // D s_301_9: read-reg s_301_8:struct
        let s_301_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_301_8 as isize);
            tracer.read_register(s_301_8 as isize, value);
            value
        };
        // C s_301_10: const #14600u : u32
        let s_301_10: u32 = 14600;
        // N s_301_11: write-reg s_301_10 <= s_301_9
        let s_301_11: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_301_10 as isize, s_301_9);
            tracer.write_register(s_301_10 as isize, s_301_9);
        };
        // C s_301_12: const #14600u : u32
        let s_301_12: u32 = 14600;
        // D s_301_13: read-reg s_301_12:struct
        let s_301_13: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_301_12 as isize);
            tracer.read_register(s_301_12 as isize, value);
            value
        };
        // C s_301_14: const #14600u : u32
        let s_301_14: u32 = 14600;
        // N s_301_15: write-reg s_301_14 <= s_301_13
        let s_301_15: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_301_14 as isize, s_301_13);
            tracer.write_register(s_301_14 as isize, s_301_13);
        };
        // N s_301_16: jump b294
        return block_294(state, tracer, fn_state);
    }
    fn block_302<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_302_0: const #11032u : u32
        let s_302_0: u32 = 11032;
        // D s_302_1: read-reg s_302_0:struct
        let s_302_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_302_0 as isize);
            tracer.read_register(s_302_0 as isize, value);
            value
        };
        // C s_302_2: const #11032u : u32
        let s_302_2: u32 = 11032;
        // N s_302_3: write-reg s_302_2 <= s_302_1
        let s_302_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_302_2 as isize, s_302_1);
            tracer.write_register(s_302_2 as isize, s_302_1);
        };
        // C s_302_4: const #21144u : u32
        let s_302_4: u32 = 21144;
        // D s_302_5: read-reg s_302_4:u8
        let s_302_5: bool = {
            let value = state.read_register::<bool>(s_302_4 as isize);
            tracer.read_register(s_302_4 as isize, value);
            value
        };
        // N s_302_6: branch s_302_5 b305 b303
        if s_302_5 {
            return block_305(state, tracer, fn_state);
        } else {
            return block_303(state, tracer, fn_state);
        };
    }
    fn block_303<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_303_0: jump b304
        return block_304(state, tracer, fn_state);
    }
    fn block_304<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_304_0: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_305<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_305_0: const #11032u : u32
        let s_305_0: u32 = 11032;
        // D s_305_1: read-reg s_305_0:struct
        let s_305_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_305_0 as isize);
            tracer.read_register(s_305_0 as isize, value);
            value
        };
        // C s_305_2: const #11032u : u32
        let s_305_2: u32 = 11032;
        // N s_305_3: write-reg s_305_2 <= s_305_1
        let s_305_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_305_2 as isize, s_305_1);
            tracer.write_register(s_305_2 as isize, s_305_1);
        };
        // N s_305_4: jump b304
        return block_304(state, tracer, fn_state);
    }
    fn block_306<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_306_0: const #() : ()
        let s_306_0: () = ();
        // S s_306_1: call HaveRME(s_306_0)
        let s_306_1: bool = HaveRME(state, tracer, s_306_0);
        // D s_306_2: write-var gs#328667 <= s_306_1
        fn_state.gs_328667 = s_306_1;
        // N s_306_3: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_307<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_307_0: const #91056u : u32
        let s_307_0: u32 = 91056;
        // D s_307_1: read-reg s_307_0:u8
        let s_307_1: bool = {
            let value = state.read_register::<bool>(s_307_0 as isize);
            tracer.read_register(s_307_0 as isize, value);
            value
        };
        // N s_307_2: branch s_307_1 b328 b308
        if s_307_1 {
            return block_328(state, tracer, fn_state);
        } else {
            return block_308(state, tracer, fn_state);
        };
    }
    fn block_308<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_308_0: const #1280u : u32
        let s_308_0: u32 = 1280;
        // D s_308_1: read-reg s_308_0:u8
        let s_308_1: u8 = {
            let value = state.read_register::<u8>(s_308_0 as isize);
            tracer.read_register(s_308_0 as isize, value);
            value
        };
        // C s_308_2: const #90144u : u32
        let s_308_2: u32 = 90144;
        // N s_308_3: write-reg s_308_2 <= s_308_1
        let s_308_3: () = {
            state.write_register::<u8>(s_308_2 as isize, s_308_1);
            tracer.write_register(s_308_2 as isize, s_308_1);
        };
        // C s_308_4: const #1304u : u32
        let s_308_4: u32 = 1304;
        // D s_308_5: read-reg s_308_4:u8
        let s_308_5: u8 = {
            let value = state.read_register::<u8>(s_308_4 as isize);
            tracer.read_register(s_308_4 as isize, value);
            value
        };
        // C s_308_6: const #14256u : u32
        let s_308_6: u32 = 14256;
        // N s_308_7: write-reg s_308_6 <= s_308_5
        let s_308_7: () = {
            state.write_register::<u8>(s_308_6 as isize, s_308_5);
            tracer.write_register(s_308_6 as isize, s_308_5);
        };
        // N s_308_8: jump b309
        return block_309(state, tracer, fn_state);
    }
    fn block_309<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_309_0: const #101824u : u32
        let s_309_0: u32 = 101824;
        // D s_309_1: read-reg s_309_0:struct
        let s_309_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_309_0 as isize);
            tracer.read_register(s_309_0 as isize, value);
            value
        };
        // C s_309_2: const #101824u : u32
        let s_309_2: u32 = 101824;
        // N s_309_3: write-reg s_309_2 <= s_309_1
        let s_309_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_309_2 as isize, s_309_1);
            tracer.write_register(s_309_2 as isize, s_309_1);
        };
        // C s_309_4: const #11032u : u32
        let s_309_4: u32 = 11032;
        // D s_309_5: read-reg s_309_4:struct
        let s_309_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_309_4 as isize);
            tracer.read_register(s_309_4 as isize, value);
            value
        };
        // C s_309_6: const #11032u : u32
        let s_309_6: u32 = 11032;
        // N s_309_7: write-reg s_309_6 <= s_309_5
        let s_309_7: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_309_6 as isize, s_309_5);
            tracer.write_register(s_309_6 as isize, s_309_5);
        };
        // C s_309_8: const #11032u : u32
        let s_309_8: u32 = 11032;
        // D s_309_9: read-reg s_309_8:struct
        let s_309_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_309_8 as isize);
            tracer.read_register(s_309_8 as isize, value);
            value
        };
        // C s_309_10: const #11032u : u32
        let s_309_10: u32 = 11032;
        // N s_309_11: write-reg s_309_10 <= s_309_9
        let s_309_11: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_309_10 as isize, s_309_9);
            tracer.write_register(s_309_10 as isize, s_309_9);
        };
        // C s_309_12: const #90392u : u32
        let s_309_12: u32 = 90392;
        // D s_309_13: read-reg s_309_12:u8
        let s_309_13: bool = {
            let value = state.read_register::<bool>(s_309_12 as isize);
            tracer.read_register(s_309_12 as isize, value);
            value
        };
        // N s_309_14: branch s_309_13 b327 b310
        if s_309_13 {
            return block_327(state, tracer, fn_state);
        } else {
            return block_310(state, tracer, fn_state);
        };
    }
    fn block_310<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_310_0: jump b311
        return block_311(state, tracer, fn_state);
    }
    fn block_311<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_311_0: const #22552u : u32
        let s_311_0: u32 = 22552;
        // D s_311_1: read-reg s_311_0:struct
        let s_311_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_311_0 as isize);
            tracer.read_register(s_311_0 as isize, value);
            value
        };
        // C s_311_2: const #22552u : u32
        let s_311_2: u32 = 22552;
        // N s_311_3: write-reg s_311_2 <= s_311_1
        let s_311_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_311_2 as isize, s_311_1);
            tracer.write_register(s_311_2 as isize, s_311_1);
        };
        // C s_311_4: const #101824u : u32
        let s_311_4: u32 = 101824;
        // D s_311_5: read-reg s_311_4:struct
        let s_311_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_311_4 as isize);
            tracer.read_register(s_311_4 as isize, value);
            value
        };
        // D s_311_6: call _get_ID_AA64PFR0_EL1_Type_MPAM(s_311_5)
        let s_311_6: u8 = u_get_ID_AA64PFR0_EL1_Type_MPAM(state, tracer, s_311_5);
        // D s_311_7: cast zx s_311_6 -> bv
        let s_311_7: Bits = Bits::new(s_311_6 as u128, 4u16);
        // D s_311_8: cast zx s_311_7 -> i
        let s_311_8: i128 = (s_311_7.value() as i128);
        // D s_311_9: cast reint s_311_8 -> i64
        let s_311_9: i64 = (s_311_8 as i64);
        // C s_311_10: const #1s : i
        let s_311_10: i128 = 1;
        // D s_311_11: cast zx s_311_9 -> i
        let s_311_11: i128 = (i128::try_from(s_311_9).unwrap());
        // D s_311_12: cmp-gt s_311_11 s_311_10
        let s_311_12: bool = ((s_311_11) > (s_311_10));
        // N s_311_13: branch s_311_12 b326 b312
        if s_311_12 {
            return block_326(state, tracer, fn_state);
        } else {
            return block_312(state, tracer, fn_state);
        };
    }
    fn block_312<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_312_0: const #101824u : u32
        let s_312_0: u32 = 101824;
        // D s_312_1: read-reg s_312_0:struct
        let s_312_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_312_0 as isize);
            tracer.read_register(s_312_0 as isize, value);
            value
        };
        // D s_312_2: call _get_ID_AA64PFR0_EL1_Type_MPAM(s_312_1)
        let s_312_2: u8 = u_get_ID_AA64PFR0_EL1_Type_MPAM(state, tracer, s_312_1);
        // C s_312_3: const #1s : i
        let s_312_3: i128 = 1;
        // D s_312_4: cast zx s_312_2 -> bv
        let s_312_4: Bits = Bits::new(s_312_2 as u128, 4u16);
        // C s_312_5: const #1s : i64
        let s_312_5: i64 = 1;
        // C s_312_6: cast zx s_312_5 -> i
        let s_312_6: i128 = (i128::try_from(s_312_5).unwrap());
        // C s_312_7: const #2s : i
        let s_312_7: i128 = 2;
        // C s_312_8: add s_312_7 s_312_6
        let s_312_8: i128 = (s_312_7 + s_312_6);
        // D s_312_9: bit-extract s_312_4 s_312_3 s_312_8
        let s_312_9: Bits = (Bits::new(
            ((s_312_4) >> (s_312_3)).value(),
            u16::try_from(s_312_8).unwrap(),
        ));
        // D s_312_10: cast reint s_312_9 -> u8
        let s_312_10: u8 = (s_312_9.value() as u8);
        // D s_312_11: cast zx s_312_10 -> bv
        let s_312_11: Bits = Bits::new(s_312_10 as u128, 3u16);
        // C s_312_12: const #0u : u8
        let s_312_12: u8 = 0;
        // C s_312_13: cast zx s_312_12 -> bv
        let s_312_13: Bits = Bits::new(s_312_12 as u128, 3u16);
        // D s_312_14: cmp-eq s_312_11 s_312_13
        let s_312_14: bool = ((s_312_11) == (s_312_13));
        // D s_312_15: not s_312_14
        let s_312_15: bool = !s_312_14;
        // N s_312_16: branch s_312_15 b325 b313
        if s_312_15 {
            return block_325(state, tracer, fn_state);
        } else {
            return block_313(state, tracer, fn_state);
        };
    }
    fn block_313<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_313_0: const #1u : u8
        let s_313_0: bool = true;
        // D s_313_1: write-var gs#330412 <= s_313_0
        fn_state.gs_330412 = s_313_0;
        // N s_313_2: jump b314
        return block_314(state, tracer, fn_state);
    }
    fn block_314<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_314_0: read-var gs#330412:u8
        let s_314_0: bool = fn_state.gs_330412;
        // N s_314_1: branch s_314_0 b324 b315
        if s_314_0 {
            return block_324(state, tracer, fn_state);
        } else {
            return block_315(state, tracer, fn_state);
        };
    }
    fn block_315<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_315_0: const #0u : u8
        let s_315_0: bool = false;
        // D s_315_1: write-var gs#330417 <= s_315_0
        fn_state.gs_330417 = s_315_0;
        // N s_315_2: jump b316
        return block_316(state, tracer, fn_state);
    }
    fn block_316<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_316_0: read-var gs#330417:u8
        let s_316_0: bool = fn_state.gs_330417;
        // D s_316_1: write-var gs#330418 <= s_316_0
        fn_state.gs_330418 = s_316_0;
        // N s_316_2: jump b317
        return block_317(state, tracer, fn_state);
    }
    fn block_317<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_317_0: read-var gs#330418:u8
        let s_317_0: bool = fn_state.gs_330418;
        // N s_317_1: assert s_317_0
        let s_317_1: () = assert!(s_317_0);
        // C s_317_2: const #21056u : u32
        let s_317_2: u32 = 21056;
        // D s_317_3: read-reg s_317_2:u8
        let s_317_3: bool = {
            let value = state.read_register::<bool>(s_317_2 as isize);
            tracer.read_register(s_317_2 as isize, value);
            value
        };
        // N s_317_4: branch s_317_3 b323 b318
        if s_317_3 {
            return block_323(state, tracer, fn_state);
        } else {
            return block_318(state, tracer, fn_state);
        };
    }
    fn block_318<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_318_0: jump b319
        return block_319(state, tracer, fn_state);
    }
    fn block_319<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_319_0: const #15224u : u32
        let s_319_0: u32 = 15224;
        // D s_319_1: read-reg s_319_0:u8
        let s_319_1: bool = {
            let value = state.read_register::<bool>(s_319_0 as isize);
            tracer.read_register(s_319_0 as isize, value);
            value
        };
        // N s_319_2: branch s_319_1 b322 b320
        if s_319_1 {
            return block_322(state, tracer, fn_state);
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
        // N s_321_0: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_322<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_322_0: const #11032u : u32
        let s_322_0: u32 = 11032;
        // D s_322_1: read-reg s_322_0:struct
        let s_322_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_322_0 as isize);
            tracer.read_register(s_322_0 as isize, value);
            value
        };
        // C s_322_2: const #11032u : u32
        let s_322_2: u32 = 11032;
        // N s_322_3: write-reg s_322_2 <= s_322_1
        let s_322_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_322_2 as isize, s_322_1);
            tracer.write_register(s_322_2 as isize, s_322_1);
        };
        // N s_322_4: jump b321
        return block_321(state, tracer, fn_state);
    }
    fn block_323<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_323_0: const #11032u : u32
        let s_323_0: u32 = 11032;
        // D s_323_1: read-reg s_323_0:struct
        let s_323_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_323_0 as isize);
            tracer.read_register(s_323_0 as isize, value);
            value
        };
        // C s_323_2: const #11032u : u32
        let s_323_2: u32 = 11032;
        // N s_323_3: write-reg s_323_2 <= s_323_1
        let s_323_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_323_2 as isize, s_323_1);
            tracer.write_register(s_323_2 as isize, s_323_1);
        };
        // N s_323_4: jump b319
        return block_319(state, tracer, fn_state);
    }
    fn block_324<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_324_0: const #22552u : u32
        let s_324_0: u32 = 22552;
        // D s_324_1: read-reg s_324_0:struct
        let s_324_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_324_0 as isize);
            tracer.read_register(s_324_0 as isize, value);
            value
        };
        // D s_324_2: call _get_ID_AA64PFR1_EL1_Type_MPAM_frac(s_324_1)
        let s_324_2: u8 = u_get_ID_AA64PFR1_EL1_Type_MPAM_frac(state, tracer, s_324_1);
        // D s_324_3: cast zx s_324_2 -> bv
        let s_324_3: Bits = Bits::new(s_324_2 as u128, 4u16);
        // C s_324_4: const #1u : u8
        let s_324_4: u8 = 1;
        // C s_324_5: cast zx s_324_4 -> bv
        let s_324_5: Bits = Bits::new(s_324_4 as u128, 4u16);
        // D s_324_6: cmp-eq s_324_3 s_324_5
        let s_324_6: bool = ((s_324_3) == (s_324_5));
        // D s_324_7: write-var gs#330417 <= s_324_6
        fn_state.gs_330417 = s_324_6;
        // N s_324_8: jump b316
        return block_316(state, tracer, fn_state);
    }
    fn block_325<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_325_0: const #0u : u8
        let s_325_0: bool = false;
        // D s_325_1: write-var gs#330412 <= s_325_0
        fn_state.gs_330412 = s_325_0;
        // N s_325_2: jump b314
        return block_314(state, tracer, fn_state);
    }
    fn block_326<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_326_0: const #1u : u8
        let s_326_0: bool = true;
        // D s_326_1: write-var gs#330418 <= s_326_0
        fn_state.gs_330418 = s_326_0;
        // N s_326_2: jump b317
        return block_317(state, tracer, fn_state);
    }
    fn block_327<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_327_0: const #11032u : u32
        let s_327_0: u32 = 11032;
        // D s_327_1: read-reg s_327_0:struct
        let s_327_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_327_0 as isize);
            tracer.read_register(s_327_0 as isize, value);
            value
        };
        // C s_327_2: const #11032u : u32
        let s_327_2: u32 = 11032;
        // N s_327_3: write-reg s_327_2 <= s_327_1
        let s_327_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_327_2 as isize, s_327_1);
            tracer.write_register(s_327_2 as isize, s_327_1);
        };
        // C s_327_4: const #11032u : u32
        let s_327_4: u32 = 11032;
        // D s_327_5: read-reg s_327_4:struct
        let s_327_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_327_4 as isize);
            tracer.read_register(s_327_4 as isize, value);
            value
        };
        // C s_327_6: const #11032u : u32
        let s_327_6: u32 = 11032;
        // N s_327_7: write-reg s_327_6 <= s_327_5
        let s_327_7: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_327_6 as isize, s_327_5);
            tracer.write_register(s_327_6 as isize, s_327_5);
        };
        // N s_327_8: jump b311
        return block_311(state, tracer, fn_state);
    }
    fn block_328<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_328_0: const #1272u : u32
        let s_328_0: u32 = 1272;
        // D s_328_1: read-reg s_328_0:u8
        let s_328_1: u8 = {
            let value = state.read_register::<u8>(s_328_0 as isize);
            tracer.read_register(s_328_0 as isize, value);
            value
        };
        // C s_328_2: const #90144u : u32
        let s_328_2: u32 = 90144;
        // N s_328_3: write-reg s_328_2 <= s_328_1
        let s_328_3: () = {
            state.write_register::<u8>(s_328_2 as isize, s_328_1);
            tracer.write_register(s_328_2 as isize, s_328_1);
        };
        // C s_328_4: const #1296u : u32
        let s_328_4: u32 = 1296;
        // D s_328_5: read-reg s_328_4:u8
        let s_328_5: u8 = {
            let value = state.read_register::<u8>(s_328_4 as isize);
            tracer.read_register(s_328_4 as isize, value);
            value
        };
        // C s_328_6: const #14256u : u32
        let s_328_6: u32 = 14256;
        // N s_328_7: write-reg s_328_6 <= s_328_5
        let s_328_7: () = {
            state.write_register::<u8>(s_328_6 as isize, s_328_5);
            tracer.write_register(s_328_6 as isize, s_328_5);
        };
        // C s_328_8: const #11032u : u32
        let s_328_8: u32 = 11032;
        // D s_328_9: read-reg s_328_8:struct
        let s_328_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_328_8 as isize);
            tracer.read_register(s_328_8 as isize, value);
            value
        };
        // C s_328_10: const #11032u : u32
        let s_328_10: u32 = 11032;
        // N s_328_11: write-reg s_328_10 <= s_328_9
        let s_328_11: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_328_10 as isize, s_328_9);
            tracer.write_register(s_328_10 as isize, s_328_9);
        };
        // N s_328_12: jump b309
        return block_309(state, tracer, fn_state);
    }
    fn block_329<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_329_0: const #103144u : u32
        let s_329_0: u32 = 103144;
        // D s_329_1: read-reg s_329_0:struct
        let s_329_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_329_0 as isize);
            tracer.read_register(s_329_0 as isize, value);
            value
        };
        // C s_329_2: const #103144u : u32
        let s_329_2: u32 = 103144;
        // N s_329_3: write-reg s_329_2 <= s_329_1
        let s_329_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_329_2 as isize, s_329_1);
            tracer.write_register(s_329_2 as isize, s_329_1);
        };
        // C s_329_4: const #90312u : u32
        let s_329_4: u32 = 90312;
        // D s_329_5: read-reg s_329_4:struct
        let s_329_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_329_4 as isize);
            tracer.read_register(s_329_4 as isize, value);
            value
        };
        // C s_329_6: const #90312u : u32
        let s_329_6: u32 = 90312;
        // N s_329_7: write-reg s_329_6 <= s_329_5
        let s_329_7: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_329_6 as isize, s_329_5);
            tracer.write_register(s_329_6 as isize, s_329_5);
        };
        // N s_329_8: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_330<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_330_0: const #101824u : u32
        let s_330_0: u32 = 101824;
        // D s_330_1: read-reg s_330_0:struct
        let s_330_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_330_0 as isize);
            tracer.read_register(s_330_0 as isize, value);
            value
        };
        // C s_330_2: const #101824u : u32
        let s_330_2: u32 = 101824;
        // N s_330_3: write-reg s_330_2 <= s_330_1
        let s_330_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_330_2 as isize, s_330_1);
            tracer.write_register(s_330_2 as isize, s_330_1);
        };
        // C s_330_4: const #101824u : u32
        let s_330_4: u32 = 101824;
        // D s_330_5: read-reg s_330_4:struct
        let s_330_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_330_4 as isize);
            tracer.read_register(s_330_4 as isize, value);
            value
        };
        // C s_330_6: const #101824u : u32
        let s_330_6: u32 = 101824;
        // N s_330_7: write-reg s_330_6 <= s_330_5
        let s_330_7: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_330_6 as isize, s_330_5);
            tracer.write_register(s_330_6 as isize, s_330_5);
        };
        // N s_330_8: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_331<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_331_0: const #90608u : u32
        let s_331_0: u32 = 90608;
        // D s_331_1: read-reg s_331_0:struct
        let s_331_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_331_0 as isize);
            tracer.read_register(s_331_0 as isize, value);
            value
        };
        // C s_331_2: const #90608u : u32
        let s_331_2: u32 = 90608;
        // N s_331_3: write-reg s_331_2 <= s_331_1
        let s_331_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_331_2 as isize, s_331_1);
            tracer.write_register(s_331_2 as isize, s_331_1);
        };
        // N s_331_4: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_332<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_332_0: const #101824u : u32
        let s_332_0: u32 = 101824;
        // D s_332_1: read-reg s_332_0:struct
        let s_332_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_332_0 as isize);
            tracer.read_register(s_332_0 as isize, value);
            value
        };
        // C s_332_2: const #101824u : u32
        let s_332_2: u32 = 101824;
        // N s_332_3: write-reg s_332_2 <= s_332_1
        let s_332_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_332_2 as isize, s_332_1);
            tracer.write_register(s_332_2 as isize, s_332_1);
        };
        // C s_332_4: const #16288u : u32
        let s_332_4: u32 = 16288;
        // D s_332_5: read-reg s_332_4:struct
        let s_332_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_332_4 as isize);
            tracer.read_register(s_332_4 as isize, value);
            value
        };
        // C s_332_6: const #16288u : u32
        let s_332_6: u32 = 16288;
        // N s_332_7: write-reg s_332_6 <= s_332_5
        let s_332_7: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_332_6 as isize, s_332_5);
            tracer.write_register(s_332_6 as isize, s_332_5);
        };
        // C s_332_8: const #104664u : u32
        let s_332_8: u32 = 104664;
        // D s_332_9: read-reg s_332_8:struct
        let s_332_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_332_8 as isize);
            tracer.read_register(s_332_8 as isize, value);
            value
        };
        // C s_332_10: const #104664u : u32
        let s_332_10: u32 = 104664;
        // N s_332_11: write-reg s_332_10 <= s_332_9
        let s_332_11: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_332_10 as isize, s_332_9);
            tracer.write_register(s_332_10 as isize, s_332_9);
        };
        // C s_332_12: const #104664u : u32
        let s_332_12: u32 = 104664;
        // D s_332_13: read-reg s_332_12:struct
        let s_332_13: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_332_12 as isize);
            tracer.read_register(s_332_12 as isize, value);
            value
        };
        // C s_332_14: const #104664u : u32
        let s_332_14: u32 = 104664;
        // N s_332_15: write-reg s_332_14 <= s_332_13
        let s_332_15: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_332_14 as isize, s_332_13);
            tracer.write_register(s_332_14 as isize, s_332_13);
        };
        // N s_332_16: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_333<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_333_0: const #103144u : u32
        let s_333_0: u32 = 103144;
        // D s_333_1: read-reg s_333_0:struct
        let s_333_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_333_0 as isize);
            tracer.read_register(s_333_0 as isize, value);
            value
        };
        // C s_333_2: const #103144u : u32
        let s_333_2: u32 = 103144;
        // N s_333_3: write-reg s_333_2 <= s_333_1
        let s_333_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_333_2 as isize, s_333_1);
            tracer.write_register(s_333_2 as isize, s_333_1);
        };
        // C s_333_4: const #103144u : u32
        let s_333_4: u32 = 103144;
        // D s_333_5: read-reg s_333_4:struct
        let s_333_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_333_4 as isize);
            tracer.read_register(s_333_4 as isize, value);
            value
        };
        // C s_333_6: const #103144u : u32
        let s_333_6: u32 = 103144;
        // N s_333_7: write-reg s_333_6 <= s_333_5
        let s_333_7: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_333_6 as isize, s_333_5);
            tracer.write_register(s_333_6 as isize, s_333_5);
        };
        // C s_333_8: const #90600u : u32
        let s_333_8: u32 = 90600;
        // D s_333_9: read-reg s_333_8:struct
        let s_333_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_333_8 as isize);
            tracer.read_register(s_333_8 as isize, value);
            value
        };
        // C s_333_10: const #90600u : u32
        let s_333_10: u32 = 90600;
        // N s_333_11: write-reg s_333_10 <= s_333_9
        let s_333_11: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_333_10 as isize, s_333_9);
            tracer.write_register(s_333_10 as isize, s_333_9);
        };
        // C s_333_12: const #90600u : u32
        let s_333_12: u32 = 90600;
        // D s_333_13: read-reg s_333_12:struct
        let s_333_13: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_333_12 as isize);
            tracer.read_register(s_333_12 as isize, value);
            value
        };
        // C s_333_14: const #90600u : u32
        let s_333_14: u32 = 90600;
        // N s_333_15: write-reg s_333_14 <= s_333_13
        let s_333_15: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_333_14 as isize, s_333_13);
            tracer.write_register(s_333_14 as isize, s_333_13);
        };
        // C s_333_16: const #90600u : u32
        let s_333_16: u32 = 90600;
        // D s_333_17: read-reg s_333_16:struct
        let s_333_17: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_333_16 as isize);
            tracer.read_register(s_333_16 as isize, value);
            value
        };
        // C s_333_18: const #90600u : u32
        let s_333_18: u32 = 90600;
        // N s_333_19: write-reg s_333_18 <= s_333_17
        let s_333_19: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_333_18 as isize, s_333_17);
            tracer.write_register(s_333_18 as isize, s_333_17);
        };
        // C s_333_20: const #90600u : u32
        let s_333_20: u32 = 90600;
        // D s_333_21: read-reg s_333_20:struct
        let s_333_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_333_20 as isize);
            tracer.read_register(s_333_20 as isize, value);
            value
        };
        // C s_333_22: const #90600u : u32
        let s_333_22: u32 = 90600;
        // N s_333_23: write-reg s_333_22 <= s_333_21
        let s_333_23: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_333_22 as isize, s_333_21);
            tracer.write_register(s_333_22 as isize, s_333_21);
        };
        // C s_333_24: const #90728u : u32
        let s_333_24: u32 = 90728;
        // D s_333_25: read-reg s_333_24:struct
        let s_333_25: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_333_24 as isize);
            tracer.read_register(s_333_24 as isize, value);
            value
        };
        // C s_333_26: const #90728u : u32
        let s_333_26: u32 = 90728;
        // N s_333_27: write-reg s_333_26 <= s_333_25
        let s_333_27: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_333_26 as isize, s_333_25);
            tracer.write_register(s_333_26 as isize, s_333_25);
        };
        // C s_333_28: const #90600u : u32
        let s_333_28: u32 = 90600;
        // D s_333_29: read-reg s_333_28:struct
        let s_333_29: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_333_28 as isize);
            tracer.read_register(s_333_28 as isize, value);
            value
        };
        // C s_333_30: const #90600u : u32
        let s_333_30: u32 = 90600;
        // N s_333_31: write-reg s_333_30 <= s_333_29
        let s_333_31: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_333_30 as isize, s_333_29);
            tracer.write_register(s_333_30 as isize, s_333_29);
        };
        // C s_333_32: const #90312u : u32
        let s_333_32: u32 = 90312;
        // D s_333_33: read-reg s_333_32:struct
        let s_333_33: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_333_32 as isize);
            tracer.read_register(s_333_32 as isize, value);
            value
        };
        // C s_333_34: const #90312u : u32
        let s_333_34: u32 = 90312;
        // N s_333_35: write-reg s_333_34 <= s_333_33
        let s_333_35: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_333_34 as isize, s_333_33);
            tracer.write_register(s_333_34 as isize, s_333_33);
        };
        // C s_333_36: const #90312u : u32
        let s_333_36: u32 = 90312;
        // D s_333_37: read-reg s_333_36:struct
        let s_333_37: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_333_36 as isize);
            tracer.read_register(s_333_36 as isize, value);
            value
        };
        // C s_333_38: const #90312u : u32
        let s_333_38: u32 = 90312;
        // N s_333_39: write-reg s_333_38 <= s_333_37
        let s_333_39: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_333_38 as isize, s_333_37);
            tracer.write_register(s_333_38 as isize, s_333_37);
        };
        // C s_333_40: const #90312u : u32
        let s_333_40: u32 = 90312;
        // D s_333_41: read-reg s_333_40:struct
        let s_333_41: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_333_40 as isize);
            tracer.read_register(s_333_40 as isize, value);
            value
        };
        // C s_333_42: const #90312u : u32
        let s_333_42: u32 = 90312;
        // N s_333_43: write-reg s_333_42 <= s_333_41
        let s_333_43: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_333_42 as isize, s_333_41);
            tracer.write_register(s_333_42 as isize, s_333_41);
        };
        // C s_333_44: const #90312u : u32
        let s_333_44: u32 = 90312;
        // D s_333_45: read-reg s_333_44:struct
        let s_333_45: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_333_44 as isize);
            tracer.read_register(s_333_44 as isize, value);
            value
        };
        // C s_333_46: const #90312u : u32
        let s_333_46: u32 = 90312;
        // N s_333_47: write-reg s_333_46 <= s_333_45
        let s_333_47: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_333_46 as isize, s_333_45);
            tracer.write_register(s_333_46 as isize, s_333_45);
        };
        // C s_333_48: const #432u : u32
        let s_333_48: u32 = 432;
        // D s_333_49: read-reg s_333_48:u8
        let s_333_49: u8 = {
            let value = state.read_register::<u8>(s_333_48 as isize);
            tracer.read_register(s_333_48 as isize, value);
            value
        };
        // C s_333_50: const #2u : u8
        let s_333_50: u8 = 2;
        // D s_333_51: cmp-lt s_333_49 s_333_50
        let s_333_51: bool = ((s_333_49) < (s_333_50));
        // N s_333_52: branch s_333_51 b339 b334
        if s_333_51 {
            return block_339(state, tracer, fn_state);
        } else {
            return block_334(state, tracer, fn_state);
        };
    }
    fn block_334<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_334_0: const #90312u : u32
        let s_334_0: u32 = 90312;
        // D s_334_1: read-reg s_334_0:struct
        let s_334_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_334_0 as isize);
            tracer.read_register(s_334_0 as isize, value);
            value
        };
        // C s_334_2: const #90312u : u32
        let s_334_2: u32 = 90312;
        // N s_334_3: write-reg s_334_2 <= s_334_1
        let s_334_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_334_2 as isize, s_334_1);
            tracer.write_register(s_334_2 as isize, s_334_1);
        };
        // C s_334_4: const #90600u : u32
        let s_334_4: u32 = 90600;
        // D s_334_5: read-reg s_334_4:struct
        let s_334_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_334_4 as isize);
            tracer.read_register(s_334_4 as isize, value);
            value
        };
        // C s_334_6: const #90600u : u32
        let s_334_6: u32 = 90600;
        // N s_334_7: write-reg s_334_6 <= s_334_5
        let s_334_7: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_334_6 as isize, s_334_5);
            tracer.write_register(s_334_6 as isize, s_334_5);
        };
        // N s_334_8: jump b335
        return block_335(state, tracer, fn_state);
    }
    fn block_335<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_335_0: const #90312u : u32
        let s_335_0: u32 = 90312;
        // D s_335_1: read-reg s_335_0:struct
        let s_335_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_335_0 as isize);
            tracer.read_register(s_335_0 as isize, value);
            value
        };
        // C s_335_2: const #90312u : u32
        let s_335_2: u32 = 90312;
        // N s_335_3: write-reg s_335_2 <= s_335_1
        let s_335_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_335_2 as isize, s_335_1);
            tracer.write_register(s_335_2 as isize, s_335_1);
        };
        // C s_335_4: const #100256u : u32
        let s_335_4: u32 = 100256;
        // D s_335_5: read-reg s_335_4:struct
        let s_335_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_335_4 as isize);
            tracer.read_register(s_335_4 as isize, value);
            value
        };
        // C s_335_6: const #100256u : u32
        let s_335_6: u32 = 100256;
        // N s_335_7: write-reg s_335_6 <= s_335_5
        let s_335_7: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_335_6 as isize, s_335_5);
            tracer.write_register(s_335_6 as isize, s_335_5);
        };
        // C s_335_8: const #100256u : u32
        let s_335_8: u32 = 100256;
        // D s_335_9: read-reg s_335_8:struct
        let s_335_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_335_8 as isize);
            tracer.read_register(s_335_8 as isize, value);
            value
        };
        // C s_335_10: const #100256u : u32
        let s_335_10: u32 = 100256;
        // N s_335_11: write-reg s_335_10 <= s_335_9
        let s_335_11: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_335_10 as isize, s_335_9);
            tracer.write_register(s_335_10 as isize, s_335_9);
        };
        // C s_335_12: const #100256u : u32
        let s_335_12: u32 = 100256;
        // D s_335_13: read-reg s_335_12:struct
        let s_335_13: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_335_12 as isize);
            tracer.read_register(s_335_12 as isize, value);
            value
        };
        // C s_335_14: const #100256u : u32
        let s_335_14: u32 = 100256;
        // N s_335_15: write-reg s_335_14 <= s_335_13
        let s_335_15: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_335_14 as isize, s_335_13);
            tracer.write_register(s_335_14 as isize, s_335_13);
        };
        // C s_335_16: const #100256u : u32
        let s_335_16: u32 = 100256;
        // D s_335_17: read-reg s_335_16:struct
        let s_335_17: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_335_16 as isize);
            tracer.read_register(s_335_16 as isize, value);
            value
        };
        // C s_335_18: const #100256u : u32
        let s_335_18: u32 = 100256;
        // N s_335_19: write-reg s_335_18 <= s_335_17
        let s_335_19: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_335_18 as isize, s_335_17);
            tracer.write_register(s_335_18 as isize, s_335_17);
        };
        // C s_335_20: const #100256u : u32
        let s_335_20: u32 = 100256;
        // D s_335_21: read-reg s_335_20:struct
        let s_335_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_335_20 as isize);
            tracer.read_register(s_335_20 as isize, value);
            value
        };
        // C s_335_22: const #100256u : u32
        let s_335_22: u32 = 100256;
        // N s_335_23: write-reg s_335_22 <= s_335_21
        let s_335_23: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_335_22 as isize, s_335_21);
            tracer.write_register(s_335_22 as isize, s_335_21);
        };
        // C s_335_24: const #100256u : u32
        let s_335_24: u32 = 100256;
        // D s_335_25: read-reg s_335_24:struct
        let s_335_25: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_335_24 as isize);
            tracer.read_register(s_335_24 as isize, value);
            value
        };
        // C s_335_26: const #100256u : u32
        let s_335_26: u32 = 100256;
        // N s_335_27: write-reg s_335_26 <= s_335_25
        let s_335_27: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_335_26 as isize, s_335_25);
            tracer.write_register(s_335_26 as isize, s_335_25);
        };
        // C s_335_28: const #100256u : u32
        let s_335_28: u32 = 100256;
        // D s_335_29: read-reg s_335_28:struct
        let s_335_29: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_335_28 as isize);
            tracer.read_register(s_335_28 as isize, value);
            value
        };
        // C s_335_30: const #100256u : u32
        let s_335_30: u32 = 100256;
        // N s_335_31: write-reg s_335_30 <= s_335_29
        let s_335_31: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_335_30 as isize, s_335_29);
            tracer.write_register(s_335_30 as isize, s_335_29);
        };
        // C s_335_32: const #14600u : u32
        let s_335_32: u32 = 14600;
        // D s_335_33: read-reg s_335_32:struct
        let s_335_33: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_335_32 as isize);
            tracer.read_register(s_335_32 as isize, value);
            value
        };
        // C s_335_34: const #14600u : u32
        let s_335_34: u32 = 14600;
        // N s_335_35: write-reg s_335_34 <= s_335_33
        let s_335_35: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_335_34 as isize, s_335_33);
            tracer.write_register(s_335_34 as isize, s_335_33);
        };
        // C s_335_36: const #14600u : u32
        let s_335_36: u32 = 14600;
        // D s_335_37: read-reg s_335_36:struct
        let s_335_37: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_335_36 as isize);
            tracer.read_register(s_335_36 as isize, value);
            value
        };
        // C s_335_38: const #14600u : u32
        let s_335_38: u32 = 14600;
        // N s_335_39: write-reg s_335_38 <= s_335_37
        let s_335_39: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_335_38 as isize, s_335_37);
            tracer.write_register(s_335_38 as isize, s_335_37);
        };
        // C s_335_40: const #14600u : u32
        let s_335_40: u32 = 14600;
        // D s_335_41: read-reg s_335_40:struct
        let s_335_41: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_335_40 as isize);
            tracer.read_register(s_335_40 as isize, value);
            value
        };
        // C s_335_42: const #14600u : u32
        let s_335_42: u32 = 14600;
        // N s_335_43: write-reg s_335_42 <= s_335_41
        let s_335_43: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_335_42 as isize, s_335_41);
            tracer.write_register(s_335_42 as isize, s_335_41);
        };
        // C s_335_44: const #90408u : u32
        let s_335_44: u32 = 90408;
        // D s_335_45: read-reg s_335_44:struct
        let s_335_45: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_335_44 as isize);
            tracer.read_register(s_335_44 as isize, value);
            value
        };
        // C s_335_46: const #90408u : u32
        let s_335_46: u32 = 90408;
        // N s_335_47: write-reg s_335_46 <= s_335_45
        let s_335_47: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_335_46 as isize, s_335_45);
            tracer.write_register(s_335_46 as isize, s_335_45);
        };
        // C s_335_48: const #() : ()
        let s_335_48: () = ();
        // S s_335_49: call HaveDoPD(s_335_48)
        let s_335_49: bool = HaveDoPD(state, tracer, s_335_48);
        // N s_335_50: branch s_335_49 b338 b336
        if s_335_49 {
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
        // C s_336_0: const #23432u : u32
        let s_336_0: u32 = 23432;
        // D s_336_1: read-reg s_336_0:struct
        let s_336_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_336_0 as isize);
            tracer.read_register(s_336_0 as isize, value);
            value
        };
        // C s_336_2: const #23432u : u32
        let s_336_2: u32 = 23432;
        // N s_336_3: write-reg s_336_2 <= s_336_1
        let s_336_3: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_336_2 as isize, s_336_1);
            tracer.write_register(s_336_2 as isize, s_336_1);
        };
        // C s_336_4: const #101032u : u32
        let s_336_4: u32 = 101032;
        // D s_336_5: read-reg s_336_4:struct
        let s_336_5: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_336_4 as isize);
            tracer.read_register(s_336_4 as isize, value);
            value
        };
        // C s_336_6: const #101032u : u32
        let s_336_6: u32 = 101032;
        // N s_336_7: write-reg s_336_6 <= s_336_5
        let s_336_7: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_336_6 as isize, s_336_5);
            tracer.write_register(s_336_6 as isize, s_336_5);
        };
        // N s_336_8: jump b337
        return block_337(state, tracer, fn_state);
    }
    fn block_337<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_337_0: const #104664u : u32
        let s_337_0: u32 = 104664;
        // D s_337_1: read-reg s_337_0:struct
        let s_337_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_337_0 as isize);
            tracer.read_register(s_337_0 as isize, value);
            value
        };
        // C s_337_2: const #104664u : u32
        let s_337_2: u32 = 104664;
        // N s_337_3: write-reg s_337_2 <= s_337_1
        let s_337_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_337_2 as isize, s_337_1);
            tracer.write_register(s_337_2 as isize, s_337_1);
        };
        // C s_337_4: const #104664u : u32
        let s_337_4: u32 = 104664;
        // D s_337_5: read-reg s_337_4:struct
        let s_337_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_337_4 as isize);
            tracer.read_register(s_337_4 as isize, value);
            value
        };
        // C s_337_6: const #104664u : u32
        let s_337_6: u32 = 104664;
        // N s_337_7: write-reg s_337_6 <= s_337_5
        let s_337_7: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_337_6 as isize, s_337_5);
            tracer.write_register(s_337_6 as isize, s_337_5);
        };
        // C s_337_8: const #104664u : u32
        let s_337_8: u32 = 104664;
        // D s_337_9: read-reg s_337_8:struct
        let s_337_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_337_8 as isize);
            tracer.read_register(s_337_8 as isize, value);
            value
        };
        // C s_337_10: const #104664u : u32
        let s_337_10: u32 = 104664;
        // N s_337_11: write-reg s_337_10 <= s_337_9
        let s_337_11: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_337_10 as isize, s_337_9);
            tracer.write_register(s_337_10 as isize, s_337_9);
        };
        // C s_337_12: const #104664u : u32
        let s_337_12: u32 = 104664;
        // D s_337_13: read-reg s_337_12:struct
        let s_337_13: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_337_12 as isize);
            tracer.read_register(s_337_12 as isize, value);
            value
        };
        // C s_337_14: const #104664u : u32
        let s_337_14: u32 = 104664;
        // N s_337_15: write-reg s_337_14 <= s_337_13
        let s_337_15: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_337_14 as isize, s_337_13);
            tracer.write_register(s_337_14 as isize, s_337_13);
        };
        // C s_337_16: const #104664u : u32
        let s_337_16: u32 = 104664;
        // D s_337_17: read-reg s_337_16:struct
        let s_337_17: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_337_16 as isize);
            tracer.read_register(s_337_16 as isize, value);
            value
        };
        // C s_337_18: const #104664u : u32
        let s_337_18: u32 = 104664;
        // N s_337_19: write-reg s_337_18 <= s_337_17
        let s_337_19: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_337_18 as isize, s_337_17);
            tracer.write_register(s_337_18 as isize, s_337_17);
        };
        // C s_337_20: const #104664u : u32
        let s_337_20: u32 = 104664;
        // D s_337_21: read-reg s_337_20:struct
        let s_337_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_337_20 as isize);
            tracer.read_register(s_337_20 as isize, value);
            value
        };
        // C s_337_22: const #104664u : u32
        let s_337_22: u32 = 104664;
        // N s_337_23: write-reg s_337_22 <= s_337_21
        let s_337_23: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_337_22 as isize, s_337_21);
            tracer.write_register(s_337_22 as isize, s_337_21);
        };
        // C s_337_24: const #14600u : u32
        let s_337_24: u32 = 14600;
        // D s_337_25: read-reg s_337_24:struct
        let s_337_25: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_337_24 as isize);
            tracer.read_register(s_337_24 as isize, value);
            value
        };
        // C s_337_26: const #14600u : u32
        let s_337_26: u32 = 14600;
        // N s_337_27: write-reg s_337_26 <= s_337_25
        let s_337_27: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_337_26 as isize, s_337_25);
            tracer.write_register(s_337_26 as isize, s_337_25);
        };
        // C s_337_28: const #104664u : u32
        let s_337_28: u32 = 104664;
        // D s_337_29: read-reg s_337_28:struct
        let s_337_29: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_337_28 as isize);
            tracer.read_register(s_337_28 as isize, value);
            value
        };
        // C s_337_30: const #104664u : u32
        let s_337_30: u32 = 104664;
        // N s_337_31: write-reg s_337_30 <= s_337_29
        let s_337_31: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_337_30 as isize, s_337_29);
            tracer.write_register(s_337_30 as isize, s_337_29);
        };
        // C s_337_32: const #14600u : u32
        let s_337_32: u32 = 14600;
        // D s_337_33: read-reg s_337_32:struct
        let s_337_33: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_337_32 as isize);
            tracer.read_register(s_337_32 as isize, value);
            value
        };
        // C s_337_34: const #14600u : u32
        let s_337_34: u32 = 14600;
        // N s_337_35: write-reg s_337_34 <= s_337_33
        let s_337_35: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_337_34 as isize, s_337_33);
            tracer.write_register(s_337_34 as isize, s_337_33);
        };
        // C s_337_36: const #14600u : u32
        let s_337_36: u32 = 14600;
        // D s_337_37: read-reg s_337_36:struct
        let s_337_37: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_337_36 as isize);
            tracer.read_register(s_337_36 as isize, value);
            value
        };
        // C s_337_38: const #14600u : u32
        let s_337_38: u32 = 14600;
        // N s_337_39: write-reg s_337_38 <= s_337_37
        let s_337_39: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_337_38 as isize, s_337_37);
            tracer.write_register(s_337_38 as isize, s_337_37);
        };
        // C s_337_40: const #90408u : u32
        let s_337_40: u32 = 90408;
        // D s_337_41: read-reg s_337_40:struct
        let s_337_41: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_337_40 as isize);
            tracer.read_register(s_337_40 as isize, value);
            value
        };
        // C s_337_42: const #90408u : u32
        let s_337_42: u32 = 90408;
        // N s_337_43: write-reg s_337_42 <= s_337_41
        let s_337_43: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_337_42 as isize, s_337_41);
            tracer.write_register(s_337_42 as isize, s_337_41);
        };
        // C s_337_44: const #104664u : u32
        let s_337_44: u32 = 104664;
        // D s_337_45: read-reg s_337_44:struct
        let s_337_45: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_337_44 as isize);
            tracer.read_register(s_337_44 as isize, value);
            value
        };
        // C s_337_46: const #104664u : u32
        let s_337_46: u32 = 104664;
        // N s_337_47: write-reg s_337_46 <= s_337_45
        let s_337_47: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_337_46 as isize, s_337_45);
            tracer.write_register(s_337_46 as isize, s_337_45);
        };
        // C s_337_48: const #14600u : u32
        let s_337_48: u32 = 14600;
        // D s_337_49: read-reg s_337_48:struct
        let s_337_49: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_337_48 as isize);
            tracer.read_register(s_337_48 as isize, value);
            value
        };
        // C s_337_50: const #14600u : u32
        let s_337_50: u32 = 14600;
        // N s_337_51: write-reg s_337_50 <= s_337_49
        let s_337_51: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_337_50 as isize, s_337_49);
            tracer.write_register(s_337_50 as isize, s_337_49);
        };
        // C s_337_52: const #90408u : u32
        let s_337_52: u32 = 90408;
        // D s_337_53: read-reg s_337_52:struct
        let s_337_53: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_337_52 as isize);
            tracer.read_register(s_337_52 as isize, value);
            value
        };
        // C s_337_54: const #90408u : u32
        let s_337_54: u32 = 90408;
        // N s_337_55: write-reg s_337_54 <= s_337_53
        let s_337_55: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_337_54 as isize, s_337_53);
            tracer.write_register(s_337_54 as isize, s_337_53);
        };
        // N s_337_56: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_338<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_338_0: const #23432u : u32
        let s_338_0: u32 = 23432;
        // D s_338_1: read-reg s_338_0:struct
        let s_338_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_338_0 as isize);
            tracer.read_register(s_338_0 as isize, value);
            value
        };
        // C s_338_2: const #23432u : u32
        let s_338_2: u32 = 23432;
        // N s_338_3: write-reg s_338_2 <= s_338_1
        let s_338_3: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_338_2 as isize, s_338_1);
            tracer.write_register(s_338_2 as isize, s_338_1);
        };
        // C s_338_4: const #101032u : u32
        let s_338_4: u32 = 101032;
        // D s_338_5: read-reg s_338_4:struct
        let s_338_5: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_338_4 as isize);
            tracer.read_register(s_338_4 as isize, value);
            value
        };
        // C s_338_6: const #101032u : u32
        let s_338_6: u32 = 101032;
        // N s_338_7: write-reg s_338_6 <= s_338_5
        let s_338_7: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_338_6 as isize, s_338_5);
            tracer.write_register(s_338_6 as isize, s_338_5);
        };
        // N s_338_8: jump b337
        return block_337(state, tracer, fn_state);
    }
    fn block_339<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_339_0: const #90312u : u32
        let s_339_0: u32 = 90312;
        // D s_339_1: read-reg s_339_0:struct
        let s_339_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_339_0 as isize);
            tracer.read_register(s_339_0 as isize, value);
            value
        };
        // C s_339_2: const #90312u : u32
        let s_339_2: u32 = 90312;
        // N s_339_3: write-reg s_339_2 <= s_339_1
        let s_339_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_339_2 as isize, s_339_1);
            tracer.write_register(s_339_2 as isize, s_339_1);
        };
        // C s_339_4: const #90600u : u32
        let s_339_4: u32 = 90600;
        // D s_339_5: read-reg s_339_4:struct
        let s_339_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_339_4 as isize);
            tracer.read_register(s_339_4 as isize, value);
            value
        };
        // C s_339_6: const #90600u : u32
        let s_339_6: u32 = 90600;
        // N s_339_7: write-reg s_339_6 <= s_339_5
        let s_339_7: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_339_6 as isize, s_339_5);
            tracer.write_register(s_339_6 as isize, s_339_5);
        };
        // N s_339_8: jump b335
        return block_335(state, tracer, fn_state);
    }
    fn block_340<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_340_0: const #90608u : u32
        let s_340_0: u32 = 90608;
        // D s_340_1: read-reg s_340_0:struct
        let s_340_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_340_0 as isize);
            tracer.read_register(s_340_0 as isize, value);
            value
        };
        // C s_340_2: const #90608u : u32
        let s_340_2: u32 = 90608;
        // N s_340_3: write-reg s_340_2 <= s_340_1
        let s_340_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_340_2 as isize, s_340_1);
            tracer.write_register(s_340_2 as isize, s_340_1);
        };
        // N s_340_4: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_341<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_341_0: const #90608u : u32
        let s_341_0: u32 = 90608;
        // D s_341_1: read-reg s_341_0:struct
        let s_341_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_341_0 as isize);
            tracer.read_register(s_341_0 as isize, value);
            value
        };
        // C s_341_2: const #90608u : u32
        let s_341_2: u32 = 90608;
        // N s_341_3: write-reg s_341_2 <= s_341_1
        let s_341_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_341_2 as isize, s_341_1);
            tracer.write_register(s_341_2 as isize, s_341_1);
        };
        // N s_341_4: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_342<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_342_0: const #90608u : u32
        let s_342_0: u32 = 90608;
        // D s_342_1: read-reg s_342_0:struct
        let s_342_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_342_0 as isize);
            tracer.read_register(s_342_0 as isize, value);
            value
        };
        // C s_342_2: const #90608u : u32
        let s_342_2: u32 = 90608;
        // N s_342_3: write-reg s_342_2 <= s_342_1
        let s_342_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_342_2 as isize, s_342_1);
            tracer.write_register(s_342_2 as isize, s_342_1);
        };
        // N s_342_4: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_343<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_343_0: const #36u : u32
        let s_343_0: u32 = 36;
        // S s_343_1: call IsFeatureImplemented(s_343_0)
        let s_343_1: bool = IsFeatureImplemented(state, tracer, s_343_0);
        // D s_343_2: write-var gs#328666 <= s_343_1
        fn_state.gs_328666 = s_343_1;
        // N s_343_3: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_344<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_344_0: const #90608u : u32
        let s_344_0: u32 = 90608;
        // D s_344_1: read-reg s_344_0:struct
        let s_344_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_344_0 as isize);
            tracer.read_register(s_344_0 as isize, value);
            value
        };
        // C s_344_2: const #90608u : u32
        let s_344_2: u32 = 90608;
        // N s_344_3: write-reg s_344_2 <= s_344_1
        let s_344_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_344_2 as isize, s_344_1);
            tracer.write_register(s_344_2 as isize, s_344_1);
        };
        // N s_344_4: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_345<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_345_0: const #90608u : u32
        let s_345_0: u32 = 90608;
        // D s_345_1: read-reg s_345_0:struct
        let s_345_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_345_0 as isize);
            tracer.read_register(s_345_0 as isize, value);
            value
        };
        // C s_345_2: const #90608u : u32
        let s_345_2: u32 = 90608;
        // N s_345_3: write-reg s_345_2 <= s_345_1
        let s_345_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_345_2 as isize, s_345_1);
            tracer.write_register(s_345_2 as isize, s_345_1);
        };
        // N s_345_4: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_346<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_346_0: const #103144u : u32
        let s_346_0: u32 = 103144;
        // D s_346_1: read-reg s_346_0:struct
        let s_346_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_346_0 as isize);
            tracer.read_register(s_346_0 as isize, value);
            value
        };
        // C s_346_2: const #103144u : u32
        let s_346_2: u32 = 103144;
        // N s_346_3: write-reg s_346_2 <= s_346_1
        let s_346_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_346_2 as isize, s_346_1);
            tracer.write_register(s_346_2 as isize, s_346_1);
        };
        // C s_346_4: const #90728u : u32
        let s_346_4: u32 = 90728;
        // D s_346_5: read-reg s_346_4:struct
        let s_346_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_346_4 as isize);
            tracer.read_register(s_346_4 as isize, value);
            value
        };
        // C s_346_6: const #90728u : u32
        let s_346_6: u32 = 90728;
        // N s_346_7: write-reg s_346_6 <= s_346_5
        let s_346_7: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_346_6 as isize, s_346_5);
            tracer.write_register(s_346_6 as isize, s_346_5);
        };
        // N s_346_8: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_347<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_347_0: const #103144u : u32
        let s_347_0: u32 = 103144;
        // D s_347_1: read-reg s_347_0:struct
        let s_347_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_347_0 as isize);
            tracer.read_register(s_347_0 as isize, value);
            value
        };
        // C s_347_2: const #103144u : u32
        let s_347_2: u32 = 103144;
        // N s_347_3: write-reg s_347_2 <= s_347_1
        let s_347_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_347_2 as isize, s_347_1);
            tracer.write_register(s_347_2 as isize, s_347_1);
        };
        // N s_347_4: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_348<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_348_0: const #1480u : u32
        let s_348_0: u32 = 1480;
        // D s_348_1: read-reg s_348_0:struct
        let s_348_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_348_0 as isize);
            tracer.read_register(s_348_0 as isize, value);
            value
        };
        // C s_348_2: const #1480u : u32
        let s_348_2: u32 = 1480;
        // N s_348_3: write-reg s_348_2 <= s_348_1
        let s_348_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_348_2 as isize, s_348_1);
            tracer.write_register(s_348_2 as isize, s_348_1);
        };
        // C s_348_4: const #90608u : u32
        let s_348_4: u32 = 90608;
        // D s_348_5: read-reg s_348_4:struct
        let s_348_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_348_4 as isize);
            tracer.read_register(s_348_4 as isize, value);
            value
        };
        // C s_348_6: const #90608u : u32
        let s_348_6: u32 = 90608;
        // N s_348_7: write-reg s_348_6 <= s_348_5
        let s_348_7: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_348_6 as isize, s_348_5);
            tracer.write_register(s_348_6 as isize, s_348_5);
        };
        // N s_348_8: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_349<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_349_0: const #1480u : u32
        let s_349_0: u32 = 1480;
        // D s_349_1: read-reg s_349_0:struct
        let s_349_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_349_0 as isize);
            tracer.read_register(s_349_0 as isize, value);
            value
        };
        // C s_349_2: const #1480u : u32
        let s_349_2: u32 = 1480;
        // N s_349_3: write-reg s_349_2 <= s_349_1
        let s_349_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_349_2 as isize, s_349_1);
            tracer.write_register(s_349_2 as isize, s_349_1);
        };
        // C s_349_4: const #90608u : u32
        let s_349_4: u32 = 90608;
        // D s_349_5: read-reg s_349_4:struct
        let s_349_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_349_4 as isize);
            tracer.read_register(s_349_4 as isize, value);
            value
        };
        // C s_349_6: const #90608u : u32
        let s_349_6: u32 = 90608;
        // N s_349_7: write-reg s_349_6 <= s_349_5
        let s_349_7: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_349_6 as isize, s_349_5);
            tracer.write_register(s_349_6 as isize, s_349_5);
        };
        // C s_349_8: const #90608u : u32
        let s_349_8: u32 = 90608;
        // D s_349_9: read-reg s_349_8:struct
        let s_349_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_349_8 as isize);
            tracer.read_register(s_349_8 as isize, value);
            value
        };
        // C s_349_10: const #90608u : u32
        let s_349_10: u32 = 90608;
        // N s_349_11: write-reg s_349_10 <= s_349_9
        let s_349_11: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_349_10 as isize, s_349_9);
            tracer.write_register(s_349_10 as isize, s_349_9);
        };
        // C s_349_12: const #103144u : u32
        let s_349_12: u32 = 103144;
        // D s_349_13: read-reg s_349_12:struct
        let s_349_13: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_349_12 as isize);
            tracer.read_register(s_349_12 as isize, value);
            value
        };
        // C s_349_14: const #103144u : u32
        let s_349_14: u32 = 103144;
        // N s_349_15: write-reg s_349_14 <= s_349_13
        let s_349_15: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_349_14 as isize, s_349_13);
            tracer.write_register(s_349_14 as isize, s_349_13);
        };
        // C s_349_16: const #103144u : u32
        let s_349_16: u32 = 103144;
        // D s_349_17: read-reg s_349_16:struct
        let s_349_17: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_349_16 as isize);
            tracer.read_register(s_349_16 as isize, value);
            value
        };
        // C s_349_18: const #103144u : u32
        let s_349_18: u32 = 103144;
        // N s_349_19: write-reg s_349_18 <= s_349_17
        let s_349_19: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_349_18 as isize, s_349_17);
            tracer.write_register(s_349_18 as isize, s_349_17);
        };
        // C s_349_20: const #103144u : u32
        let s_349_20: u32 = 103144;
        // D s_349_21: read-reg s_349_20:struct
        let s_349_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_349_20 as isize);
            tracer.read_register(s_349_20 as isize, value);
            value
        };
        // C s_349_22: const #103144u : u32
        let s_349_22: u32 = 103144;
        // N s_349_23: write-reg s_349_22 <= s_349_21
        let s_349_23: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_349_22 as isize, s_349_21);
            tracer.write_register(s_349_22 as isize, s_349_21);
        };
        // C s_349_24: const #103144u : u32
        let s_349_24: u32 = 103144;
        // D s_349_25: read-reg s_349_24:struct
        let s_349_25: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_349_24 as isize);
            tracer.read_register(s_349_24 as isize, value);
            value
        };
        // C s_349_26: const #103144u : u32
        let s_349_26: u32 = 103144;
        // N s_349_27: write-reg s_349_26 <= s_349_25
        let s_349_27: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_349_26 as isize, s_349_25);
            tracer.write_register(s_349_26 as isize, s_349_25);
        };
        // N s_349_28: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_350<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_350_0: const #15896u : u32
        let s_350_0: u32 = 15896;
        // D s_350_1: read-reg s_350_0:struct
        let s_350_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_350_0 as isize);
            tracer.read_register(s_350_0 as isize, value);
            value
        };
        // C s_350_2: const #15896u : u32
        let s_350_2: u32 = 15896;
        // N s_350_3: write-reg s_350_2 <= s_350_1
        let s_350_3: () = {
            state
                .write_register::<
                    ProductType700c18a878c5601b,
                >(s_350_2 as isize, s_350_1);
            tracer.write_register(s_350_2 as isize, s_350_1);
        };
        // N s_350_4: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_351<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_351_0: const #20112u : u32
        let s_351_0: u32 = 20112;
        // D s_351_1: read-reg s_351_0:struct
        let s_351_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_351_0 as isize);
            tracer.read_register(s_351_0 as isize, value);
            value
        };
        // C s_351_2: const #20112u : u32
        let s_351_2: u32 = 20112;
        // N s_351_3: write-reg s_351_2 <= s_351_1
        let s_351_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_351_2 as isize, s_351_1);
            tracer.write_register(s_351_2 as isize, s_351_1);
        };
        // C s_351_4: const #13496u : u32
        let s_351_4: u32 = 13496;
        // D s_351_5: read-reg s_351_4:struct
        let s_351_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_351_4 as isize);
            tracer.read_register(s_351_4 as isize, value);
            value
        };
        // C s_351_6: const #13496u : u32
        let s_351_6: u32 = 13496;
        // N s_351_7: write-reg s_351_6 <= s_351_5
        let s_351_7: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_351_6 as isize, s_351_5);
            tracer.write_register(s_351_6 as isize, s_351_5);
        };
        // C s_351_8: const #13496u : u32
        let s_351_8: u32 = 13496;
        // D s_351_9: read-reg s_351_8:struct
        let s_351_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_351_8 as isize);
            tracer.read_register(s_351_8 as isize, value);
            value
        };
        // C s_351_10: const #13496u : u32
        let s_351_10: u32 = 13496;
        // N s_351_11: write-reg s_351_10 <= s_351_9
        let s_351_11: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_351_10 as isize, s_351_9);
            tracer.write_register(s_351_10 as isize, s_351_9);
        };
        // C s_351_12: const #7s : i
        let s_351_12: i128 = 7;
        // C s_351_13: const #0s : i
        let s_351_13: i128 = 0;
        // C s_351_14: const #20824u : u32
        let s_351_14: u32 = 20824;
        // D s_351_15: read-reg s_351_14:i
        let s_351_15: i128 = {
            let value = state.read_register::<i128>(s_351_14 as isize);
            tracer.read_register(s_351_14 as isize, value);
            value
        };
        // D s_351_16: call integer_subrange(s_351_15, s_351_12, s_351_13)
        let s_351_16: Bits = integer_subrange(
            state,
            tracer,
            s_351_15,
            s_351_12,
            s_351_13,
        );
        // C s_351_17: const #13496u : u32
        let s_351_17: u32 = 13496;
        // D s_351_18: read-reg s_351_17:struct
        let s_351_18: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_351_17 as isize);
            tracer.read_register(s_351_17 as isize, value);
            value
        };
        // C s_351_19: const #13496u : u32
        let s_351_19: u32 = 13496;
        // N s_351_20: write-reg s_351_19 <= s_351_18
        let s_351_20: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_351_19 as isize, s_351_18);
            tracer.write_register(s_351_19 as isize, s_351_18);
        };
        // N s_351_21: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_352<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_352_0: const #101824u : u32
        let s_352_0: u32 = 101824;
        // D s_352_1: read-reg s_352_0:struct
        let s_352_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_352_0 as isize);
            tracer.read_register(s_352_0 as isize, value);
            value
        };
        // C s_352_2: const #101824u : u32
        let s_352_2: u32 = 101824;
        // N s_352_3: write-reg s_352_2 <= s_352_1
        let s_352_3: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_352_2 as isize, s_352_1);
            tracer.write_register(s_352_2 as isize, s_352_1);
        };
        // C s_352_4: const #16648u : u32
        let s_352_4: u32 = 16648;
        // D s_352_5: read-reg s_352_4:struct
        let s_352_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_352_4 as isize);
            tracer.read_register(s_352_4 as isize, value);
            value
        };
        // C s_352_6: const #16648u : u32
        let s_352_6: u32 = 16648;
        // N s_352_7: write-reg s_352_6 <= s_352_5
        let s_352_7: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_352_6 as isize, s_352_5);
            tracer.write_register(s_352_6 as isize, s_352_5);
        };
        // N s_352_8: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_353<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_353_0: const #8s : i
        let s_353_0: i128 = 8;
        // S s_353_1: call Zeros(s_353_0)
        let s_353_1: Bits = Zeros(state, tracer, s_353_0);
        // N s_353_2: jump b8
        return block_8(state, tracer, fn_state);
    }
}
