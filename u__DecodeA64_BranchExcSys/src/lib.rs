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
use decode_csdb_aarch64_instrs_system_hints::*;
use decode_pacib_aarch64_instrs_integer_pac_pacib_hint::*;
use decode_ttest_aarch64_instrs_system_tme_ttest::*;
use decode_eret_aarch64_instrs_branch_unconditional_eret::*;
use decode_esb_aarch64_instrs_system_hints::*;
use decode_tbnz_aarch64_instrs_branch_conditional_test::*;
use decode_dsb_aarch64_instrs_system_barriers_dsb_nxs::*;
use decode_msrr_aarch64_instrs_system_register_system_128::*;
use decode_msr_imm_aarch64_instrs_system_register_cpsr::*;
use decode_xpac_aarch64_instrs_integer_pac_strip_hint::*;
use decode_blra_aarch64_instrs_branch_unconditional_register::*;
use decode_brk_aarch64_instrs_system_exceptions_debug_breakpoint::*;
use decode_isb_aarch64_instrs_system_barriers_isb::*;
use decode_smc_aarch64_instrs_system_exceptions_runtime_smc::*;
use decode_sysp_aarch64_instrs_system_sysops_128::*;
use decode_nop_aarch64_instrs_system_hints::*;
use decode_ereta_aarch64_instrs_branch_unconditional_eret::*;
use decode_wfit_aarch64_instrs_system_sysinstwithreg_wfit::*;
use decode_b_cond_aarch64_instrs_branch_conditional_cond::*;
use decode_mrrs_aarch64_instrs_system_register_system_128::*;
use decode_dgh_aarch64_instrs_system_hints::*;
use decode_cbnz_aarch64_instrs_branch_conditional_compare::*;
use decode_psb_aarch64_instrs_system_hints::*;
use decode_dmb_aarch64_instrs_system_barriers_dmb::*;
use decode_tsb_aarch64_instrs_system_hints::*;
use decode_autia_aarch64_instrs_integer_pac_autia_hint::*;
use decode_b_uncond_aarch64_instrs_branch_unconditional_immediate::*;
use decode_bl_aarch64_instrs_branch_unconditional_immediate::*;
use decode_clrbhb_aarch64_instrs_system_hints::*;
use decode_dcps1_aarch64_instrs_system_exceptions_debug_exception::*;
use decode_yield_aarch64_instrs_system_hints::*;
use decode_pacia_aarch64_instrs_integer_pac_pacia_hint::*;
use decode_drps_aarch64_instrs_branch_unconditional_dret::*;
use decode_gcsb_aarch64_instrs_system_hints::*;
use decode_dcps2_aarch64_instrs_system_exceptions_debug_exception::*;
use decode_hvc_aarch64_instrs_system_exceptions_runtime_hvc::*;
use decode_tcancel_aarch64_instrs_system_tme_tcancel::*;
use decode_sys_aarch64_instrs_system_sysops::*;
use decode_cbz_aarch64_instrs_branch_conditional_compare::*;
use decode_tcommit_aarch64_instrs_system_tme_tcommit::*;
use decode_xaflag_aarch64_instrs_integer_flags_xaflag::*;
use decode_ret_aarch64_instrs_branch_unconditional_register::*;
use decode_br_aarch64_instrs_branch_unconditional_register::*;
use decode_cfinv_aarch64_instrs_integer_flags_cfinv::*;
use decode_hlt_aarch64_instrs_system_exceptions_debug_halt::*;
use decode_clrex_aarch64_instrs_system_monitors::*;
use decode_sevl_aarch64_instrs_system_hints::*;
use decode_dcps3_aarch64_instrs_system_exceptions_debug_exception::*;
use decode_wfet_aarch64_instrs_system_sysinstwithreg_wfet::*;
use decode_dsb_aarch64_instrs_system_barriers_dsb::*;
use decode_svc_aarch64_instrs_system_exceptions_runtime_svc::*;
use decode_chkfeat_aarch64_instrs_system_hints::*;
use decode_autib_aarch64_instrs_integer_pac_autib_hint::*;
use decode_sev_aarch64_instrs_system_hints::*;
use decode_tstart_aarch64_instrs_system_tme_tstart::*;
use decode_bti_aarch64_instrs_system_hints::*;
use decode_bc_cond_aarch64_instrs_branch_conditional_hinted::*;
use decode_hint_aarch64_instrs_system_hints::*;
use decode_wfi_aarch64_instrs_system_hints::*;
use decode_mrs_aarch64_instrs_system_register_system::*;
use decode_sysl_aarch64_instrs_system_sysops::*;
use decode_msr_reg_aarch64_instrs_system_register_system::*;
use decode_reta_aarch64_instrs_branch_unconditional_register::*;
use decode_wfe_aarch64_instrs_system_hints::*;
use decode_sb_aarch64_instrs_system_barriers_sb::*;
use decode_tbz_aarch64_instrs_branch_conditional_test::*;
use decode_blr_aarch64_instrs_branch_unconditional_register::*;
use decode_axflag_aarch64_instrs_integer_flags_axflag::*;
use decode_bra_aarch64_instrs_branch_unconditional_register::*;
use common::*;
pub fn u__DecodeA64_BranchExcSys<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_373616: i128,
    gs_373617: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        u_23230: u32,
        gs_374361: bool,
        u_23283: u8,
        u_23104: u32,
        u_23289: u32,
        gs_374554: bool,
        gs_373966: bool,
        gs_374396: bool,
        gs_373663: bool,
        gs_374037: bool,
        gs_373793: bool,
        gs_374097: bool,
        u_23338: u32,
        u_23300: u32,
        gs_373948: bool,
        u_23174: u32,
        u_23072: u32,
        u_23214: u32,
        u_23126: u32,
        u_23063: u32,
        gs_373925: bool,
        u_23284: u8,
        gs_374476: bool,
        gs_374113: bool,
        gs_374259: bool,
        u_23057: u32,
        u_23059: u32,
        gs_373771: bool,
        gs_373674: bool,
        u_23170: u32,
        gs_373939: bool,
        gs_374387: bool,
        gs_374158: bool,
        gs_373747: bool,
        gs_374073: bool,
        gs_373643: bool,
        gs_374384: bool,
        u_23269: u32,
        gs_373628: bool,
        gs_373957: bool,
        gs_374390: bool,
        u_23178: u32,
        gs_373699: bool,
        u_23050: u32,
        gs_374325: bool,
        gs_374493: bool,
        u_23278: u32,
        gs_374457: bool,
        gs_374576: bool,
        gs_373791: bool,
        gs_374185: bool,
        gs_373880: bool,
        gs_374511: bool,
        u_23274: u32,
        gs_374545: bool,
        gs_373645: bool,
        gs_373871: bool,
        u_23340: u32,
        gs_373993: bool,
        gs_374011: bool,
        u_23332: u32,
        gs_374085: bool,
        gs_374328: bool,
        u_23344: u8,
        gs_373898: bool,
        gs_374344: bool,
        gs_374573: bool,
        u_23134: u32,
        gs_374301: bool,
        gs_373725: bool,
        u_23069: u32,
        gs_374160: bool,
        u_23335: u32,
        gs_374022: bool,
        u_23282: u32,
        u_23150: u32,
        gs_374326: bool,
        gs_374213: bool,
        gs_374398: bool,
        gs_374345: bool,
        gs_374579: bool,
        u_23343: u32,
        u_23242: u32,
        gs_374438: bool,
        gs_374410: bool,
        gs_374111: bool,
        gs_374280: bool,
        u_23158: u32,
        gs_373625: bool,
        gs_373975: bool,
        gs_374127: bool,
        u_23233: u32,
        u_23154: u32,
        u_23237: u32,
        u_23320: u32,
        u_23118: u32,
        gs_373907: bool,
        u_23189: u8,
        gs_374362: bool,
        u_23080: u32,
        gs_373626: bool,
        gs_373713: bool,
        u_23122: u32,
        gs_374062: bool,
        gs_374373: bool,
        gs_374050: bool,
        gs_374136: bool,
        u_23221: u32,
        gs_374167: bool,
        u_23065: u32,
        gs_374125: bool,
        gs_374527: bool,
        gs_373916: bool,
        u_23201: u32,
        gs_374364: bool,
        u_23054: u32,
        u_23194: u32,
        u_23327: u32,
        gs_373727: bool,
        gs_373769: bool,
        u_23188: u32,
        u_23138: u32,
        gs_373669: bool,
        u_23205: u32,
        u_23308: u32,
        gs_373654: bool,
        gs_373749: bool,
        gs_374201: bool,
        u_23292: u32,
        u_23166: u32,
        u_23182: u32,
        u_23146: u32,
        gs_373889: bool,
        u_23162: u32,
        gs_374561: bool,
        gs_373711: bool,
        gs_374215: bool,
        gs_374099: bool,
        gs_374347: bool,
        gs_373859: bool,
        gs_373937: bool,
        gs_373642: bool,
        u_23210: u32,
        gs_374419: bool,
        u_23130: u32,
        gs_374408: bool,
        u_23114: u32,
        gs_374065: bool,
        gs_374513: bool,
        gs_374146: bool,
        u_23316: u32,
        u_23088: u32,
        gs_374059: bool,
        gs_374570: bool,
        gs_373815: bool,
        u_23216: u32,
        u_23251: u32,
        gs_374203: bool,
        u__opcode: u32,
        merge_var: ProductType7b8639ca40b2f578,
        u_23197: u32,
        u_23191: u32,
        gs_374238: bool,
        u_23260: u32,
        gs_373813: bool,
        gs_373837: bool,
        gs_373835: bool,
        gs_373666: bool,
        gs_374144: bool,
        u_23112: u32,
        u_23142: u32,
        gs_373685: bool,
        gs_373984: bool,
        gs_373869: bool,
        gs_374520: bool,
        gs_373697: bool,
        gs_374536: bool,
        u_23055: u8,
        u_23096: u32,
        gs_374227: bool,
        gs_374002: bool,
        gs_374225: bool,
        gs_373857: bool,
        u_23227: u32,
        gs_374083: bool,
        gs_374071: bool,
        gs_374183: bool,
        u_23286: u32,
        gs_373616: i128,
        gs_373617: u32,
    }
    let fn_state = FunctionState {
        gs_373616,
        gs_373617,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var gs#373616:i
        let s_0_0: i128 = fn_state.gs_373616;
        // D s_0_1: write-var merge#var.0 <= s_0_0
        fn_state.merge_var._0 = s_0_0;
        // D s_0_2: read-var gs#373617:u32
        let s_0_2: u32 = fn_state.gs_373617;
        // D s_0_3: write-var merge#var.1 <= s_0_2
        fn_state.merge_var._1 = s_0_2;
        // D s_0_4: read-var merge#var.1:struct
        let s_0_4: u32 = fn_state.merge_var._1;
        // D s_0_5: write-var __opcode <= s_0_4
        fn_state.u__opcode = s_0_4;
        // C s_0_6: const #10s : i
        let s_0_6: i128 = 10;
        // D s_0_7: read-var __opcode:u32
        let s_0_7: u32 = fn_state.u__opcode;
        // D s_0_8: cast zx s_0_7 -> bv
        let s_0_8: Bits = Bits::new(s_0_7 as u128, 32u16);
        // C s_0_9: const #1s : i64
        let s_0_9: i64 = 1;
        // C s_0_10: cast zx s_0_9 -> i
        let s_0_10: i128 = (i128::try_from(s_0_9).unwrap());
        // C s_0_11: const #21s : i
        let s_0_11: i128 = 21;
        // C s_0_12: add s_0_11 s_0_10
        let s_0_12: i128 = (s_0_11 + s_0_10);
        // D s_0_13: bit-extract s_0_8 s_0_6 s_0_12
        let s_0_13: Bits = (Bits::new(
            ((s_0_8) >> (s_0_6)).value(),
            u16::try_from(s_0_12).unwrap(),
        ));
        // D s_0_14: cast reint s_0_13 -> u22
        let s_0_14: u32 = (s_0_13.value() as u32);
        // D s_0_15: cast zx s_0_14 -> bv
        let s_0_15: Bits = Bits::new(s_0_14 as u128, 22u16);
        // C s_0_16: const #3489992u : u22
        let s_0_16: u32 = 3489992;
        // C s_0_17: cast zx s_0_16 -> bv
        let s_0_17: Bits = Bits::new(s_0_16 as u128, 22u16);
        // D s_0_18: cmp-eq s_0_15 s_0_17
        let s_0_18: bool = ((s_0_15) == (s_0_17));
        // N s_0_19: branch s_0_18 b491 b1
        if s_0_18 {
            return block_491(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#373626 <= s_1_0
        fn_state.gs_373626 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#373626:u8
        let s_2_0: bool = fn_state.gs_373626;
        // N s_2_1: branch s_2_0 b490 b3
        if s_2_0 {
            return block_490(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#373628 <= s_3_0
        fn_state.gs_373628 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#373628:u8
        let s_4_0: bool = fn_state.gs_373628;
        // D s_4_1: not s_4_0
        let s_4_1: bool = !s_4_0;
        // N s_4_2: branch s_4_1 b6 b5
        if s_4_1 {
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
        // C s_5_0: const #63s : i
        let s_5_0: i128 = 63;
        // C s_5_1: const #14696u : u32
        let s_5_1: u32 = 14696;
        // N s_5_2: write-reg s_5_1 <= s_5_0
        let s_5_2: () = {
            state.write_register::<i128>(s_5_1 as isize, s_5_0);
            tracer.write_register(s_5_1 as isize, s_5_0);
        };
        // C s_5_3: const #5s : i
        let s_5_3: i128 = 5;
        // C s_5_4: const #3s : i
        let s_5_4: i128 = 3;
        // D s_5_5: read-var __opcode:u32
        let s_5_5: u32 = fn_state.u__opcode;
        // D s_5_6: cast zx s_5_5 -> bv
        let s_5_6: Bits = Bits::new(s_5_5 as u128, 32u16);
        // D s_5_7: bit-extract s_5_6 s_5_3 s_5_4
        let s_5_7: Bits = (Bits::new(
            ((s_5_6) >> (s_5_3)).value(),
            u16::try_from(s_5_4).unwrap(),
        ));
        // D s_5_8: cast reint s_5_7 -> u8
        let s_5_8: u8 = (s_5_7.value() as u8);
        // C s_5_9: const #8s : i
        let s_5_9: i128 = 8;
        // C s_5_10: const #4s : i
        let s_5_10: i128 = 4;
        // D s_5_11: read-var __opcode:u32
        let s_5_11: u32 = fn_state.u__opcode;
        // D s_5_12: cast zx s_5_11 -> bv
        let s_5_12: Bits = Bits::new(s_5_11 as u128, 32u16);
        // D s_5_13: bit-extract s_5_12 s_5_9 s_5_10
        let s_5_13: Bits = (Bits::new(
            ((s_5_12) >> (s_5_9)).value(),
            u16::try_from(s_5_10).unwrap(),
        ));
        // D s_5_14: cast reint s_5_13 -> u8
        let s_5_14: u8 = (s_5_13.value() as u8);
        // D s_5_15: call decode_autia_aarch64_instrs_integer_pac_autia_hint(s_5_8, s_5_14)
        let s_5_15: () = decode_autia_aarch64_instrs_integer_pac_autia_hint(
            state,
            tracer,
            s_5_8,
            s_5_14,
        );
        // N s_5_16: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var merge#var.1:struct
        let s_6_0: u32 = fn_state.merge_var._1;
        // D s_6_1: write-var u#23050 <= s_6_0
        fn_state.u_23050 = s_6_0;
        // C s_6_2: const #10s : i
        let s_6_2: i128 = 10;
        // D s_6_3: read-var u#23050:u32
        let s_6_3: u32 = fn_state.u_23050;
        // D s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 32u16);
        // C s_6_5: const #1s : i64
        let s_6_5: i64 = 1;
        // C s_6_6: cast zx s_6_5 -> i
        let s_6_6: i128 = (i128::try_from(s_6_5).unwrap());
        // C s_6_7: const #21s : i
        let s_6_7: i128 = 21;
        // C s_6_8: add s_6_7 s_6_6
        let s_6_8: i128 = (s_6_7 + s_6_6);
        // D s_6_9: bit-extract s_6_4 s_6_2 s_6_8
        let s_6_9: Bits = (Bits::new(
            ((s_6_4) >> (s_6_2)).value(),
            u16::try_from(s_6_8).unwrap(),
        ));
        // D s_6_10: cast reint s_6_9 -> u22
        let s_6_10: u32 = (s_6_9.value() as u32);
        // D s_6_11: cast zx s_6_10 -> bv
        let s_6_11: Bits = Bits::new(s_6_10 as u128, 22u16);
        // C s_6_12: const #3489992u : u22
        let s_6_12: u32 = 3489992;
        // C s_6_13: cast zx s_6_12 -> bv
        let s_6_13: Bits = Bits::new(s_6_12 as u128, 22u16);
        // D s_6_14: cmp-eq s_6_11 s_6_13
        let s_6_14: bool = ((s_6_11) == (s_6_13));
        // N s_6_15: branch s_6_14 b486 b7
        if s_6_14 {
            return block_486(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#373643 <= s_7_0
        fn_state.gs_373643 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#373643:u8
        let s_8_0: bool = fn_state.gs_373643;
        // N s_8_1: branch s_8_0 b485 b9
        if s_8_0 {
            return block_485(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#373645 <= s_9_0
        fn_state.gs_373645 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#373645:u8
        let s_10_0: bool = fn_state.gs_373645;
        // D s_10_1: not s_10_0
        let s_10_1: bool = !s_10_0;
        // N s_10_2: branch s_10_1 b12 b11
        if s_10_1 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #65s : i
        let s_11_0: i128 = 65;
        // C s_11_1: const #14696u : u32
        let s_11_1: u32 = 14696;
        // N s_11_2: write-reg s_11_1 <= s_11_0
        let s_11_2: () = {
            state.write_register::<i128>(s_11_1 as isize, s_11_0);
            tracer.write_register(s_11_1 as isize, s_11_0);
        };
        // C s_11_3: const #5s : i
        let s_11_3: i128 = 5;
        // C s_11_4: const #3s : i
        let s_11_4: i128 = 3;
        // D s_11_5: read-var u#23050:u32
        let s_11_5: u32 = fn_state.u_23050;
        // D s_11_6: cast zx s_11_5 -> bv
        let s_11_6: Bits = Bits::new(s_11_5 as u128, 32u16);
        // D s_11_7: bit-extract s_11_6 s_11_3 s_11_4
        let s_11_7: Bits = (Bits::new(
            ((s_11_6) >> (s_11_3)).value(),
            u16::try_from(s_11_4).unwrap(),
        ));
        // D s_11_8: cast reint s_11_7 -> u8
        let s_11_8: u8 = (s_11_7.value() as u8);
        // C s_11_9: const #8s : i
        let s_11_9: i128 = 8;
        // C s_11_10: const #4s : i
        let s_11_10: i128 = 4;
        // D s_11_11: read-var u#23050:u32
        let s_11_11: u32 = fn_state.u_23050;
        // D s_11_12: cast zx s_11_11 -> bv
        let s_11_12: Bits = Bits::new(s_11_11 as u128, 32u16);
        // D s_11_13: bit-extract s_11_12 s_11_9 s_11_10
        let s_11_13: Bits = (Bits::new(
            ((s_11_12) >> (s_11_9)).value(),
            u16::try_from(s_11_10).unwrap(),
        ));
        // D s_11_14: cast reint s_11_13 -> u8
        let s_11_14: u8 = (s_11_13.value() as u8);
        // D s_11_15: call decode_autib_aarch64_instrs_integer_pac_autib_hint(s_11_8, s_11_14)
        let s_11_15: () = decode_autib_aarch64_instrs_integer_pac_autib_hint(
            state,
            tracer,
            s_11_8,
            s_11_14,
        );
        // N s_11_16: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var merge#var.1:struct
        let s_12_0: u32 = fn_state.merge_var._1;
        // D s_12_1: write-var u#23054 <= s_12_0
        fn_state.u_23054 = s_12_0;
        // D s_12_2: read-var u#23054:u32
        let s_12_2: u32 = fn_state.u_23054;
        // D s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 32u16);
        // C s_12_4: const #3573563487u : u32
        let s_12_4: u32 = 3573563487;
        // C s_12_5: cast zx s_12_4 -> bv
        let s_12_5: Bits = Bits::new(s_12_4 as u128, 32u16);
        // D s_12_6: cmp-eq s_12_3 s_12_5
        let s_12_6: bool = ((s_12_3) == (s_12_5));
        // N s_12_7: branch s_12_6 b484 b13
        if s_12_6 {
            return block_484(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#373654 <= s_13_0
        fn_state.gs_373654 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#373654:u8
        let s_14_0: bool = fn_state.gs_373654;
        // D s_14_1: not s_14_0
        let s_14_1: bool = !s_14_0;
        // N s_14_2: branch s_14_1 b27 b15
        if s_14_1 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #66s : i
        let s_15_0: i128 = 66;
        // C s_15_1: const #14696u : u32
        let s_15_1: u32 = 14696;
        // N s_15_2: write-reg s_15_1 <= s_15_0
        let s_15_2: () = {
            state.write_register::<i128>(s_15_1 as isize, s_15_0);
            tracer.write_register(s_15_1 as isize, s_15_0);
        };
        // C s_15_3: const #8s : i
        let s_15_3: i128 = 8;
        // C s_15_4: const #4s : i
        let s_15_4: i128 = 4;
        // D s_15_5: read-var u#23054:u32
        let s_15_5: u32 = fn_state.u_23054;
        // D s_15_6: cast zx s_15_5 -> bv
        let s_15_6: Bits = Bits::new(s_15_5 as u128, 32u16);
        // D s_15_7: bit-extract s_15_6 s_15_3 s_15_4
        let s_15_7: Bits = (Bits::new(
            ((s_15_6) >> (s_15_3)).value(),
            u16::try_from(s_15_4).unwrap(),
        ));
        // D s_15_8: cast reint s_15_7 -> u8
        let s_15_8: u8 = (s_15_7.value() as u8);
        // D s_15_9: write-var u#23055 <= s_15_8
        fn_state.u_23055 = s_15_8;
        // C s_15_10: const #8s : i
        let s_15_10: i128 = 8;
        // D s_15_11: read-var u#23054:u32
        let s_15_11: u32 = fn_state.u_23054;
        // D s_15_12: cast zx s_15_11 -> bv
        let s_15_12: Bits = Bits::new(s_15_11 as u128, 32u16);
        // C s_15_13: const #1u : u64
        let s_15_13: u64 = 1;
        // D s_15_14: bit-extract s_15_12 s_15_10 s_15_13
        let s_15_14: Bits = (Bits::new(
            ((s_15_12) >> (s_15_10)).value(),
            u16::try_from(s_15_13).unwrap(),
        ));
        // D s_15_15: cast reint s_15_14 -> u8
        let s_15_15: bool = ((s_15_14.value()) != 0);
        // C s_15_16: const #0s : i
        let s_15_16: i128 = 0;
        // C s_15_17: const #0u : u64
        let s_15_17: u64 = 0;
        // D s_15_18: cast zx s_15_15 -> u64
        let s_15_18: u64 = (s_15_15 as u64);
        // C s_15_19: const #1u : u64
        let s_15_19: u64 = 1;
        // D s_15_20: and s_15_18 s_15_19
        let s_15_20: u64 = ((s_15_18) & (s_15_19));
        // D s_15_21: cmp-eq s_15_20 s_15_19
        let s_15_21: bool = ((s_15_20) == (s_15_19));
        // D s_15_22: lsl s_15_18 s_15_16
        let s_15_22: u64 = s_15_18 << s_15_16;
        // D s_15_23: or s_15_17 s_15_22
        let s_15_23: u64 = ((s_15_17) | (s_15_22));
        // D s_15_24: cmpl s_15_22
        let s_15_24: u64 = !s_15_22;
        // D s_15_25: and s_15_17 s_15_24
        let s_15_25: u64 = ((s_15_17) & (s_15_24));
        // D s_15_26: select s_15_21 s_15_23 s_15_25
        let s_15_26: u64 = if s_15_21 { s_15_23 } else { s_15_25 };
        // D s_15_27: cast trunc s_15_26 -> u8
        let s_15_27: bool = ((s_15_26) != 0);
        // D s_15_28: cast zx s_15_27 -> bv
        let s_15_28: Bits = Bits::new(s_15_27 as u128, 1u16);
        // C s_15_29: const #0u : u8
        let s_15_29: bool = false;
        // C s_15_30: cast zx s_15_29 -> bv
        let s_15_30: Bits = Bits::new(s_15_29 as u128, 1u16);
        // D s_15_31: cmp-ne s_15_28 s_15_30
        let s_15_31: bool = ((s_15_28) != (s_15_30));
        // N s_15_32: branch s_15_31 b26 b16
        if s_15_31 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #9s : i
        let s_16_0: i128 = 9;
        // D s_16_1: read-var u#23054:u32
        let s_16_1: u32 = fn_state.u_23054;
        // D s_16_2: cast zx s_16_1 -> bv
        let s_16_2: Bits = Bits::new(s_16_1 as u128, 32u16);
        // C s_16_3: const #1u : u64
        let s_16_3: u64 = 1;
        // D s_16_4: bit-extract s_16_2 s_16_0 s_16_3
        let s_16_4: Bits = (Bits::new(
            ((s_16_2) >> (s_16_0)).value(),
            u16::try_from(s_16_3).unwrap(),
        ));
        // D s_16_5: cast reint s_16_4 -> u8
        let s_16_5: bool = ((s_16_4.value()) != 0);
        // C s_16_6: const #0s : i
        let s_16_6: i128 = 0;
        // C s_16_7: const #0u : u64
        let s_16_7: u64 = 0;
        // D s_16_8: cast zx s_16_5 -> u64
        let s_16_8: u64 = (s_16_5 as u64);
        // C s_16_9: const #1u : u64
        let s_16_9: u64 = 1;
        // D s_16_10: and s_16_8 s_16_9
        let s_16_10: u64 = ((s_16_8) & (s_16_9));
        // D s_16_11: cmp-eq s_16_10 s_16_9
        let s_16_11: bool = ((s_16_10) == (s_16_9));
        // D s_16_12: lsl s_16_8 s_16_6
        let s_16_12: u64 = s_16_8 << s_16_6;
        // D s_16_13: or s_16_7 s_16_12
        let s_16_13: u64 = ((s_16_7) | (s_16_12));
        // D s_16_14: cmpl s_16_12
        let s_16_14: u64 = !s_16_12;
        // D s_16_15: and s_16_7 s_16_14
        let s_16_15: u64 = ((s_16_7) & (s_16_14));
        // D s_16_16: select s_16_11 s_16_13 s_16_15
        let s_16_16: u64 = if s_16_11 { s_16_13 } else { s_16_15 };
        // D s_16_17: cast trunc s_16_16 -> u8
        let s_16_17: bool = ((s_16_16) != 0);
        // D s_16_18: cast zx s_16_17 -> bv
        let s_16_18: Bits = Bits::new(s_16_17 as u128, 1u16);
        // C s_16_19: const #0u : u8
        let s_16_19: bool = false;
        // C s_16_20: cast zx s_16_19 -> bv
        let s_16_20: Bits = Bits::new(s_16_19 as u128, 1u16);
        // D s_16_21: cmp-ne s_16_18 s_16_20
        let s_16_21: bool = ((s_16_18) != (s_16_20));
        // D s_16_22: write-var gs#373663 <= s_16_21
        fn_state.gs_373663 = s_16_21;
        // N s_16_23: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#373663:u8
        let s_17_0: bool = fn_state.gs_373663;
        // N s_17_1: branch s_17_0 b25 b18
        if s_17_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #10s : i
        let s_18_0: i128 = 10;
        // D s_18_1: read-var u#23054:u32
        let s_18_1: u32 = fn_state.u_23054;
        // D s_18_2: cast zx s_18_1 -> bv
        let s_18_2: Bits = Bits::new(s_18_1 as u128, 32u16);
        // C s_18_3: const #1u : u64
        let s_18_3: u64 = 1;
        // D s_18_4: bit-extract s_18_2 s_18_0 s_18_3
        let s_18_4: Bits = (Bits::new(
            ((s_18_2) >> (s_18_0)).value(),
            u16::try_from(s_18_3).unwrap(),
        ));
        // D s_18_5: cast reint s_18_4 -> u8
        let s_18_5: bool = ((s_18_4.value()) != 0);
        // C s_18_6: const #0s : i
        let s_18_6: i128 = 0;
        // C s_18_7: const #0u : u64
        let s_18_7: u64 = 0;
        // D s_18_8: cast zx s_18_5 -> u64
        let s_18_8: u64 = (s_18_5 as u64);
        // C s_18_9: const #1u : u64
        let s_18_9: u64 = 1;
        // D s_18_10: and s_18_8 s_18_9
        let s_18_10: u64 = ((s_18_8) & (s_18_9));
        // D s_18_11: cmp-eq s_18_10 s_18_9
        let s_18_11: bool = ((s_18_10) == (s_18_9));
        // D s_18_12: lsl s_18_8 s_18_6
        let s_18_12: u64 = s_18_8 << s_18_6;
        // D s_18_13: or s_18_7 s_18_12
        let s_18_13: u64 = ((s_18_7) | (s_18_12));
        // D s_18_14: cmpl s_18_12
        let s_18_14: u64 = !s_18_12;
        // D s_18_15: and s_18_7 s_18_14
        let s_18_15: u64 = ((s_18_7) & (s_18_14));
        // D s_18_16: select s_18_11 s_18_13 s_18_15
        let s_18_16: u64 = if s_18_11 { s_18_13 } else { s_18_15 };
        // D s_18_17: cast trunc s_18_16 -> u8
        let s_18_17: bool = ((s_18_16) != 0);
        // D s_18_18: cast zx s_18_17 -> bv
        let s_18_18: Bits = Bits::new(s_18_17 as u128, 1u16);
        // C s_18_19: const #0u : u8
        let s_18_19: bool = false;
        // C s_18_20: cast zx s_18_19 -> bv
        let s_18_20: Bits = Bits::new(s_18_19 as u128, 1u16);
        // D s_18_21: cmp-ne s_18_18 s_18_20
        let s_18_21: bool = ((s_18_18) != (s_18_20));
        // D s_18_22: write-var gs#373666 <= s_18_21
        fn_state.gs_373666 = s_18_21;
        // N s_18_23: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#373666:u8
        let s_19_0: bool = fn_state.gs_373666;
        // N s_19_1: branch s_19_0 b24 b20
        if s_19_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #11s : i
        let s_20_0: i128 = 11;
        // D s_20_1: read-var u#23054:u32
        let s_20_1: u32 = fn_state.u_23054;
        // D s_20_2: cast zx s_20_1 -> bv
        let s_20_2: Bits = Bits::new(s_20_1 as u128, 32u16);
        // C s_20_3: const #1u : u64
        let s_20_3: u64 = 1;
        // D s_20_4: bit-extract s_20_2 s_20_0 s_20_3
        let s_20_4: Bits = (Bits::new(
            ((s_20_2) >> (s_20_0)).value(),
            u16::try_from(s_20_3).unwrap(),
        ));
        // D s_20_5: cast reint s_20_4 -> u8
        let s_20_5: bool = ((s_20_4.value()) != 0);
        // C s_20_6: const #0s : i
        let s_20_6: i128 = 0;
        // C s_20_7: const #0u : u64
        let s_20_7: u64 = 0;
        // D s_20_8: cast zx s_20_5 -> u64
        let s_20_8: u64 = (s_20_5 as u64);
        // C s_20_9: const #1u : u64
        let s_20_9: u64 = 1;
        // D s_20_10: and s_20_8 s_20_9
        let s_20_10: u64 = ((s_20_8) & (s_20_9));
        // D s_20_11: cmp-eq s_20_10 s_20_9
        let s_20_11: bool = ((s_20_10) == (s_20_9));
        // D s_20_12: lsl s_20_8 s_20_6
        let s_20_12: u64 = s_20_8 << s_20_6;
        // D s_20_13: or s_20_7 s_20_12
        let s_20_13: u64 = ((s_20_7) | (s_20_12));
        // D s_20_14: cmpl s_20_12
        let s_20_14: u64 = !s_20_12;
        // D s_20_15: and s_20_7 s_20_14
        let s_20_15: u64 = ((s_20_7) & (s_20_14));
        // D s_20_16: select s_20_11 s_20_13 s_20_15
        let s_20_16: u64 = if s_20_11 { s_20_13 } else { s_20_15 };
        // D s_20_17: cast trunc s_20_16 -> u8
        let s_20_17: bool = ((s_20_16) != 0);
        // D s_20_18: cast zx s_20_17 -> bv
        let s_20_18: Bits = Bits::new(s_20_17 as u128, 1u16);
        // C s_20_19: const #0u : u8
        let s_20_19: bool = false;
        // C s_20_20: cast zx s_20_19 -> bv
        let s_20_20: Bits = Bits::new(s_20_19 as u128, 1u16);
        // D s_20_21: cmp-ne s_20_18 s_20_20
        let s_20_21: bool = ((s_20_18) != (s_20_20));
        // D s_20_22: write-var gs#373669 <= s_20_21
        fn_state.gs_373669 = s_20_21;
        // N s_20_23: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#373669:u8
        let s_21_0: bool = fn_state.gs_373669;
        // N s_21_1: branch s_21_0 b23 b22
        if s_21_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var u#23055:u8
        let s_22_0: u8 = fn_state.u_23055;
        // D s_22_1: call decode_axflag_aarch64_instrs_integer_flags_axflag(s_22_0)
        let s_22_1: () = decode_axflag_aarch64_instrs_integer_flags_axflag(
            state,
            tracer,
            s_22_0,
        );
        // N s_22_2: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: panic
        panic!("{:?}", ());
        // N s_23_1: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #1u : u8
        let s_24_0: bool = true;
        // D s_24_1: write-var gs#373669 <= s_24_0
        fn_state.gs_373669 = s_24_0;
        // N s_24_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #1u : u8
        let s_25_0: bool = true;
        // D s_25_1: write-var gs#373666 <= s_25_0
        fn_state.gs_373666 = s_25_0;
        // N s_25_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #1u : u8
        let s_26_0: bool = true;
        // D s_26_1: write-var gs#373663 <= s_26_0
        fn_state.gs_373663 = s_26_0;
        // N s_26_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var merge#var.1:struct
        let s_27_0: u32 = fn_state.merge_var._1;
        // D s_27_1: write-var u#23057 <= s_27_0
        fn_state.u_23057 = s_27_0;
        // C s_27_2: const #26s : i
        let s_27_2: i128 = 26;
        // D s_27_3: read-var u#23057:u32
        let s_27_3: u32 = fn_state.u_23057;
        // D s_27_4: cast zx s_27_3 -> bv
        let s_27_4: Bits = Bits::new(s_27_3 as u128, 32u16);
        // C s_27_5: const #1s : i64
        let s_27_5: i64 = 1;
        // C s_27_6: cast zx s_27_5 -> i
        let s_27_6: i128 = (i128::try_from(s_27_5).unwrap());
        // C s_27_7: const #5s : i
        let s_27_7: i128 = 5;
        // C s_27_8: add s_27_7 s_27_6
        let s_27_8: i128 = (s_27_7 + s_27_6);
        // D s_27_9: bit-extract s_27_4 s_27_2 s_27_8
        let s_27_9: Bits = (Bits::new(
            ((s_27_4) >> (s_27_2)).value(),
            u16::try_from(s_27_8).unwrap(),
        ));
        // D s_27_10: cast reint s_27_9 -> u8
        let s_27_10: u8 = (s_27_9.value() as u8);
        // D s_27_11: cast zx s_27_10 -> bv
        let s_27_11: Bits = Bits::new(s_27_10 as u128, 6u16);
        // C s_27_12: const #5u : u8
        let s_27_12: u8 = 5;
        // C s_27_13: cast zx s_27_12 -> bv
        let s_27_13: Bits = Bits::new(s_27_12 as u128, 6u16);
        // D s_27_14: cmp-eq s_27_11 s_27_13
        let s_27_14: bool = ((s_27_11) == (s_27_13));
        // N s_27_15: branch s_27_14 b483 b28
        if s_27_14 {
            return block_483(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #0u : u8
        let s_28_0: bool = false;
        // D s_28_1: write-var gs#373674 <= s_28_0
        fn_state.gs_373674 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#373674:u8
        let s_29_0: bool = fn_state.gs_373674;
        // D s_29_1: not s_29_0
        let s_29_1: bool = !s_29_0;
        // N s_29_2: branch s_29_1 b31 b30
        if s_29_1 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #67s : i
        let s_30_0: i128 = 67;
        // C s_30_1: const #14696u : u32
        let s_30_1: u32 = 14696;
        // N s_30_2: write-reg s_30_1 <= s_30_0
        let s_30_2: () = {
            state.write_register::<i128>(s_30_1 as isize, s_30_0);
            tracer.write_register(s_30_1 as isize, s_30_0);
        };
        // C s_30_3: const #0s : i
        let s_30_3: i128 = 0;
        // C s_30_4: const #26s : i
        let s_30_4: i128 = 26;
        // D s_30_5: read-var u#23057:u32
        let s_30_5: u32 = fn_state.u_23057;
        // D s_30_6: cast zx s_30_5 -> bv
        let s_30_6: Bits = Bits::new(s_30_5 as u128, 32u16);
        // D s_30_7: bit-extract s_30_6 s_30_3 s_30_4
        let s_30_7: Bits = (Bits::new(
            ((s_30_6) >> (s_30_3)).value(),
            u16::try_from(s_30_4).unwrap(),
        ));
        // D s_30_8: cast reint s_30_7 -> u26
        let s_30_8: u32 = (s_30_7.value() as u32);
        // C s_30_9: const #31s : i
        let s_30_9: i128 = 31;
        // C s_30_10: const #1s : i
        let s_30_10: i128 = 1;
        // D s_30_11: read-var u#23057:u32
        let s_30_11: u32 = fn_state.u_23057;
        // D s_30_12: cast zx s_30_11 -> bv
        let s_30_12: Bits = Bits::new(s_30_11 as u128, 32u16);
        // D s_30_13: bit-extract s_30_12 s_30_9 s_30_10
        let s_30_13: Bits = (Bits::new(
            ((s_30_12) >> (s_30_9)).value(),
            u16::try_from(s_30_10).unwrap(),
        ));
        // D s_30_14: cast reint s_30_13 -> u8
        let s_30_14: bool = ((s_30_13.value()) != 0);
        // D s_30_15: call decode_b_uncond_aarch64_instrs_branch_unconditional_immediate(s_30_8, s_30_14)
        let s_30_15: () = decode_b_uncond_aarch64_instrs_branch_unconditional_immediate(
            state,
            tracer,
            s_30_8,
            s_30_14,
        );
        // N s_30_16: return
        return;
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var merge#var.1:struct
        let s_31_0: u32 = fn_state.merge_var._1;
        // D s_31_1: write-var u#23059 <= s_31_0
        fn_state.u_23059 = s_31_0;
        // C s_31_2: const #26s : i
        let s_31_2: i128 = 26;
        // D s_31_3: read-var u#23059:u32
        let s_31_3: u32 = fn_state.u_23059;
        // D s_31_4: cast zx s_31_3 -> bv
        let s_31_4: Bits = Bits::new(s_31_3 as u128, 32u16);
        // C s_31_5: const #1s : i64
        let s_31_5: i64 = 1;
        // C s_31_6: cast zx s_31_5 -> i
        let s_31_6: i128 = (i128::try_from(s_31_5).unwrap());
        // C s_31_7: const #5s : i
        let s_31_7: i128 = 5;
        // C s_31_8: add s_31_7 s_31_6
        let s_31_8: i128 = (s_31_7 + s_31_6);
        // D s_31_9: bit-extract s_31_4 s_31_2 s_31_8
        let s_31_9: Bits = (Bits::new(
            ((s_31_4) >> (s_31_2)).value(),
            u16::try_from(s_31_8).unwrap(),
        ));
        // D s_31_10: cast reint s_31_9 -> u8
        let s_31_10: u8 = (s_31_9.value() as u8);
        // D s_31_11: cast zx s_31_10 -> bv
        let s_31_11: Bits = Bits::new(s_31_10 as u128, 6u16);
        // C s_31_12: const #37u : u8
        let s_31_12: u8 = 37;
        // C s_31_13: cast zx s_31_12 -> bv
        let s_31_13: Bits = Bits::new(s_31_12 as u128, 6u16);
        // D s_31_14: cmp-eq s_31_11 s_31_13
        let s_31_14: bool = ((s_31_11) == (s_31_13));
        // N s_31_15: branch s_31_14 b482 b32
        if s_31_14 {
            return block_482(state, tracer, fn_state);
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
        // D s_32_1: write-var gs#373685 <= s_32_0
        fn_state.gs_373685 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#373685:u8
        let s_33_0: bool = fn_state.gs_373685;
        // D s_33_1: not s_33_0
        let s_33_1: bool = !s_33_0;
        // N s_33_2: branch s_33_1 b35 b34
        if s_33_1 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #68s : i
        let s_34_0: i128 = 68;
        // C s_34_1: const #14696u : u32
        let s_34_1: u32 = 14696;
        // N s_34_2: write-reg s_34_1 <= s_34_0
        let s_34_2: () = {
            state.write_register::<i128>(s_34_1 as isize, s_34_0);
            tracer.write_register(s_34_1 as isize, s_34_0);
        };
        // C s_34_3: const #0s : i
        let s_34_3: i128 = 0;
        // C s_34_4: const #26s : i
        let s_34_4: i128 = 26;
        // D s_34_5: read-var u#23059:u32
        let s_34_5: u32 = fn_state.u_23059;
        // D s_34_6: cast zx s_34_5 -> bv
        let s_34_6: Bits = Bits::new(s_34_5 as u128, 32u16);
        // D s_34_7: bit-extract s_34_6 s_34_3 s_34_4
        let s_34_7: Bits = (Bits::new(
            ((s_34_6) >> (s_34_3)).value(),
            u16::try_from(s_34_4).unwrap(),
        ));
        // D s_34_8: cast reint s_34_7 -> u26
        let s_34_8: u32 = (s_34_7.value() as u32);
        // C s_34_9: const #31s : i
        let s_34_9: i128 = 31;
        // C s_34_10: const #1s : i
        let s_34_10: i128 = 1;
        // D s_34_11: read-var u#23059:u32
        let s_34_11: u32 = fn_state.u_23059;
        // D s_34_12: cast zx s_34_11 -> bv
        let s_34_12: Bits = Bits::new(s_34_11 as u128, 32u16);
        // D s_34_13: bit-extract s_34_12 s_34_9 s_34_10
        let s_34_13: Bits = (Bits::new(
            ((s_34_12) >> (s_34_9)).value(),
            u16::try_from(s_34_10).unwrap(),
        ));
        // D s_34_14: cast reint s_34_13 -> u8
        let s_34_14: bool = ((s_34_13.value()) != 0);
        // D s_34_15: call decode_bl_aarch64_instrs_branch_unconditional_immediate(s_34_8, s_34_14)
        let s_34_15: () = decode_bl_aarch64_instrs_branch_unconditional_immediate(
            state,
            tracer,
            s_34_8,
            s_34_14,
        );
        // N s_34_16: return
        return;
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var merge#var.1:struct
        let s_35_0: u32 = fn_state.merge_var._1;
        // D s_35_1: write-var u#23063 <= s_35_0
        fn_state.u_23063 = s_35_0;
        // C s_35_2: const #24s : i
        let s_35_2: i128 = 24;
        // D s_35_3: read-var u#23063:u32
        let s_35_3: u32 = fn_state.u_23063;
        // D s_35_4: cast zx s_35_3 -> bv
        let s_35_4: Bits = Bits::new(s_35_3 as u128, 32u16);
        // C s_35_5: const #1s : i64
        let s_35_5: i64 = 1;
        // C s_35_6: cast zx s_35_5 -> i
        let s_35_6: i128 = (i128::try_from(s_35_5).unwrap());
        // C s_35_7: const #7s : i
        let s_35_7: i128 = 7;
        // C s_35_8: add s_35_7 s_35_6
        let s_35_8: i128 = (s_35_7 + s_35_6);
        // D s_35_9: bit-extract s_35_4 s_35_2 s_35_8
        let s_35_9: Bits = (Bits::new(
            ((s_35_4) >> (s_35_2)).value(),
            u16::try_from(s_35_8).unwrap(),
        ));
        // D s_35_10: cast reint s_35_9 -> u8
        let s_35_10: u8 = (s_35_9.value() as u8);
        // D s_35_11: cast zx s_35_10 -> bv
        let s_35_11: Bits = Bits::new(s_35_10 as u128, 8u16);
        // C s_35_12: const #84u : u8
        let s_35_12: u8 = 84;
        // C s_35_13: cast zx s_35_12 -> bv
        let s_35_13: Bits = Bits::new(s_35_12 as u128, 8u16);
        // D s_35_14: cmp-eq s_35_11 s_35_13
        let s_35_14: bool = ((s_35_11) == (s_35_13));
        // N s_35_15: branch s_35_14 b481 b36
        if s_35_14 {
            return block_481(state, tracer, fn_state);
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
        // D s_36_1: write-var gs#373697 <= s_36_0
        fn_state.gs_373697 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#373697:u8
        let s_37_0: bool = fn_state.gs_373697;
        // N s_37_1: branch s_37_0 b480 b38
        if s_37_0 {
            return block_480(state, tracer, fn_state);
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
        // D s_38_1: write-var gs#373699 <= s_38_0
        fn_state.gs_373699 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#373699:u8
        let s_39_0: bool = fn_state.gs_373699;
        // D s_39_1: not s_39_0
        let s_39_1: bool = !s_39_0;
        // N s_39_2: branch s_39_1 b41 b40
        if s_39_1 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #69s : i
        let s_40_0: i128 = 69;
        // C s_40_1: const #14696u : u32
        let s_40_1: u32 = 14696;
        // N s_40_2: write-reg s_40_1 <= s_40_0
        let s_40_2: () = {
            state.write_register::<i128>(s_40_1 as isize, s_40_0);
            tracer.write_register(s_40_1 as isize, s_40_0);
        };
        // C s_40_3: const #0s : i
        let s_40_3: i128 = 0;
        // C s_40_4: const #4s : i
        let s_40_4: i128 = 4;
        // D s_40_5: read-var u#23063:u32
        let s_40_5: u32 = fn_state.u_23063;
        // D s_40_6: cast zx s_40_5 -> bv
        let s_40_6: Bits = Bits::new(s_40_5 as u128, 32u16);
        // D s_40_7: bit-extract s_40_6 s_40_3 s_40_4
        let s_40_7: Bits = (Bits::new(
            ((s_40_6) >> (s_40_3)).value(),
            u16::try_from(s_40_4).unwrap(),
        ));
        // D s_40_8: cast reint s_40_7 -> u8
        let s_40_8: u8 = (s_40_7.value() as u8);
        // C s_40_9: const #5s : i
        let s_40_9: i128 = 5;
        // C s_40_10: const #19s : i
        let s_40_10: i128 = 19;
        // D s_40_11: read-var u#23063:u32
        let s_40_11: u32 = fn_state.u_23063;
        // D s_40_12: cast zx s_40_11 -> bv
        let s_40_12: Bits = Bits::new(s_40_11 as u128, 32u16);
        // D s_40_13: bit-extract s_40_12 s_40_9 s_40_10
        let s_40_13: Bits = (Bits::new(
            ((s_40_12) >> (s_40_9)).value(),
            u16::try_from(s_40_10).unwrap(),
        ));
        // D s_40_14: cast reint s_40_13 -> u19
        let s_40_14: u32 = (s_40_13.value() as u32);
        // D s_40_15: call decode_b_cond_aarch64_instrs_branch_conditional_cond(s_40_8, s_40_14)
        let s_40_15: () = decode_b_cond_aarch64_instrs_branch_conditional_cond(
            state,
            tracer,
            s_40_8,
            s_40_14,
        );
        // N s_40_16: return
        return;
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var merge#var.1:struct
        let s_41_0: u32 = fn_state.merge_var._1;
        // D s_41_1: write-var u#23065 <= s_41_0
        fn_state.u_23065 = s_41_0;
        // C s_41_2: const #24s : i
        let s_41_2: i128 = 24;
        // D s_41_3: read-var u#23065:u32
        let s_41_3: u32 = fn_state.u_23065;
        // D s_41_4: cast zx s_41_3 -> bv
        let s_41_4: Bits = Bits::new(s_41_3 as u128, 32u16);
        // C s_41_5: const #1s : i64
        let s_41_5: i64 = 1;
        // C s_41_6: cast zx s_41_5 -> i
        let s_41_6: i128 = (i128::try_from(s_41_5).unwrap());
        // C s_41_7: const #7s : i
        let s_41_7: i128 = 7;
        // C s_41_8: add s_41_7 s_41_6
        let s_41_8: i128 = (s_41_7 + s_41_6);
        // D s_41_9: bit-extract s_41_4 s_41_2 s_41_8
        let s_41_9: Bits = (Bits::new(
            ((s_41_4) >> (s_41_2)).value(),
            u16::try_from(s_41_8).unwrap(),
        ));
        // D s_41_10: cast reint s_41_9 -> u8
        let s_41_10: u8 = (s_41_9.value() as u8);
        // D s_41_11: cast zx s_41_10 -> bv
        let s_41_11: Bits = Bits::new(s_41_10 as u128, 8u16);
        // C s_41_12: const #84u : u8
        let s_41_12: u8 = 84;
        // C s_41_13: cast zx s_41_12 -> bv
        let s_41_13: Bits = Bits::new(s_41_12 as u128, 8u16);
        // D s_41_14: cmp-eq s_41_11 s_41_13
        let s_41_14: bool = ((s_41_11) == (s_41_13));
        // N s_41_15: branch s_41_14 b479 b42
        if s_41_14 {
            return block_479(state, tracer, fn_state);
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
        // D s_42_1: write-var gs#373711 <= s_42_0
        fn_state.gs_373711 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#373711:u8
        let s_43_0: bool = fn_state.gs_373711;
        // N s_43_1: branch s_43_0 b478 b44
        if s_43_0 {
            return block_478(state, tracer, fn_state);
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
        // D s_44_1: write-var gs#373713 <= s_44_0
        fn_state.gs_373713 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#373713:u8
        let s_45_0: bool = fn_state.gs_373713;
        // D s_45_1: not s_45_0
        let s_45_1: bool = !s_45_0;
        // N s_45_2: branch s_45_1 b47 b46
        if s_45_1 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #70s : i
        let s_46_0: i128 = 70;
        // C s_46_1: const #14696u : u32
        let s_46_1: u32 = 14696;
        // N s_46_2: write-reg s_46_1 <= s_46_0
        let s_46_2: () = {
            state.write_register::<i128>(s_46_1 as isize, s_46_0);
            tracer.write_register(s_46_1 as isize, s_46_0);
        };
        // C s_46_3: const #0s : i
        let s_46_3: i128 = 0;
        // C s_46_4: const #4s : i
        let s_46_4: i128 = 4;
        // D s_46_5: read-var u#23065:u32
        let s_46_5: u32 = fn_state.u_23065;
        // D s_46_6: cast zx s_46_5 -> bv
        let s_46_6: Bits = Bits::new(s_46_5 as u128, 32u16);
        // D s_46_7: bit-extract s_46_6 s_46_3 s_46_4
        let s_46_7: Bits = (Bits::new(
            ((s_46_6) >> (s_46_3)).value(),
            u16::try_from(s_46_4).unwrap(),
        ));
        // D s_46_8: cast reint s_46_7 -> u8
        let s_46_8: u8 = (s_46_7.value() as u8);
        // C s_46_9: const #5s : i
        let s_46_9: i128 = 5;
        // C s_46_10: const #19s : i
        let s_46_10: i128 = 19;
        // D s_46_11: read-var u#23065:u32
        let s_46_11: u32 = fn_state.u_23065;
        // D s_46_12: cast zx s_46_11 -> bv
        let s_46_12: Bits = Bits::new(s_46_11 as u128, 32u16);
        // D s_46_13: bit-extract s_46_12 s_46_9 s_46_10
        let s_46_13: Bits = (Bits::new(
            ((s_46_12) >> (s_46_9)).value(),
            u16::try_from(s_46_10).unwrap(),
        ));
        // D s_46_14: cast reint s_46_13 -> u19
        let s_46_14: u32 = (s_46_13.value() as u32);
        // D s_46_15: call decode_bc_cond_aarch64_instrs_branch_conditional_hinted(s_46_8, s_46_14)
        let s_46_15: () = decode_bc_cond_aarch64_instrs_branch_conditional_hinted(
            state,
            tracer,
            s_46_8,
            s_46_14,
        );
        // N s_46_16: return
        return;
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var merge#var.1:struct
        let s_47_0: u32 = fn_state.merge_var._1;
        // D s_47_1: write-var u#23069 <= s_47_0
        fn_state.u_23069 = s_47_0;
        // C s_47_2: const #10s : i
        let s_47_2: i128 = 10;
        // D s_47_3: read-var u#23069:u32
        let s_47_3: u32 = fn_state.u_23069;
        // D s_47_4: cast zx s_47_3 -> bv
        let s_47_4: Bits = Bits::new(s_47_3 as u128, 32u16);
        // C s_47_5: const #1s : i64
        let s_47_5: i64 = 1;
        // C s_47_6: cast zx s_47_5 -> i
        let s_47_6: i128 = (i128::try_from(s_47_5).unwrap());
        // C s_47_7: const #21s : i
        let s_47_7: i128 = 21;
        // C s_47_8: add s_47_7 s_47_6
        let s_47_8: i128 = (s_47_7 + s_47_6);
        // D s_47_9: bit-extract s_47_4 s_47_2 s_47_8
        let s_47_9: Bits = (Bits::new(
            ((s_47_4) >> (s_47_2)).value(),
            u16::try_from(s_47_8).unwrap(),
        ));
        // D s_47_10: cast reint s_47_9 -> u22
        let s_47_10: u32 = (s_47_9.value() as u32);
        // D s_47_11: cast zx s_47_10 -> bv
        let s_47_11: Bits = Bits::new(s_47_10 as u128, 22u16);
        // C s_47_12: const #3510208u : u22
        let s_47_12: u32 = 3510208;
        // C s_47_13: cast zx s_47_12 -> bv
        let s_47_13: Bits = Bits::new(s_47_12 as u128, 22u16);
        // D s_47_14: cmp-eq s_47_11 s_47_13
        let s_47_14: bool = ((s_47_11) == (s_47_13));
        // N s_47_15: branch s_47_14 b477 b48
        if s_47_14 {
            return block_477(state, tracer, fn_state);
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
        // D s_48_1: write-var gs#373725 <= s_48_0
        fn_state.gs_373725 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#373725:u8
        let s_49_0: bool = fn_state.gs_373725;
        // N s_49_1: branch s_49_0 b476 b50
        if s_49_0 {
            return block_476(state, tracer, fn_state);
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
        // D s_50_1: write-var gs#373727 <= s_50_0
        fn_state.gs_373727 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#373727:u8
        let s_51_0: bool = fn_state.gs_373727;
        // D s_51_1: not s_51_0
        let s_51_1: bool = !s_51_0;
        // N s_51_2: branch s_51_1 b53 b52
        if s_51_1 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #90s : i
        let s_52_0: i128 = 90;
        // C s_52_1: const #14696u : u32
        let s_52_1: u32 = 14696;
        // N s_52_2: write-reg s_52_1 <= s_52_0
        let s_52_2: () = {
            state.write_register::<i128>(s_52_1 as isize, s_52_0);
            tracer.write_register(s_52_1 as isize, s_52_0);
        };
        // C s_52_3: const #0s : i
        let s_52_3: i128 = 0;
        // C s_52_4: const #5s : i
        let s_52_4: i128 = 5;
        // D s_52_5: read-var u#23069:u32
        let s_52_5: u32 = fn_state.u_23069;
        // D s_52_6: cast zx s_52_5 -> bv
        let s_52_6: Bits = Bits::new(s_52_5 as u128, 32u16);
        // D s_52_7: bit-extract s_52_6 s_52_3 s_52_4
        let s_52_7: Bits = (Bits::new(
            ((s_52_6) >> (s_52_3)).value(),
            u16::try_from(s_52_4).unwrap(),
        ));
        // D s_52_8: cast reint s_52_7 -> u8
        let s_52_8: u8 = (s_52_7.value() as u8);
        // C s_52_9: const #5s : i
        let s_52_9: i128 = 5;
        // C s_52_10: const #5s : i
        let s_52_10: i128 = 5;
        // D s_52_11: read-var u#23069:u32
        let s_52_11: u32 = fn_state.u_23069;
        // D s_52_12: cast zx s_52_11 -> bv
        let s_52_12: Bits = Bits::new(s_52_11 as u128, 32u16);
        // D s_52_13: bit-extract s_52_12 s_52_9 s_52_10
        let s_52_13: Bits = (Bits::new(
            ((s_52_12) >> (s_52_9)).value(),
            u16::try_from(s_52_10).unwrap(),
        ));
        // D s_52_14: cast reint s_52_13 -> u8
        let s_52_14: u8 = (s_52_13.value() as u8);
        // C s_52_15: const #10s : i
        let s_52_15: i128 = 10;
        // C s_52_16: const #1s : i
        let s_52_16: i128 = 1;
        // D s_52_17: read-var u#23069:u32
        let s_52_17: u32 = fn_state.u_23069;
        // D s_52_18: cast zx s_52_17 -> bv
        let s_52_18: Bits = Bits::new(s_52_17 as u128, 32u16);
        // D s_52_19: bit-extract s_52_18 s_52_15 s_52_16
        let s_52_19: Bits = (Bits::new(
            ((s_52_18) >> (s_52_15)).value(),
            u16::try_from(s_52_16).unwrap(),
        ));
        // D s_52_20: cast reint s_52_19 -> u8
        let s_52_20: bool = ((s_52_19.value()) != 0);
        // C s_52_21: const #11s : i
        let s_52_21: i128 = 11;
        // C s_52_22: const #1s : i
        let s_52_22: i128 = 1;
        // D s_52_23: read-var u#23069:u32
        let s_52_23: u32 = fn_state.u_23069;
        // D s_52_24: cast zx s_52_23 -> bv
        let s_52_24: Bits = Bits::new(s_52_23 as u128, 32u16);
        // D s_52_25: bit-extract s_52_24 s_52_21 s_52_22
        let s_52_25: Bits = (Bits::new(
            ((s_52_24) >> (s_52_21)).value(),
            u16::try_from(s_52_22).unwrap(),
        ));
        // D s_52_26: cast reint s_52_25 -> u8
        let s_52_26: bool = ((s_52_25.value()) != 0);
        // C s_52_27: const #21s : i
        let s_52_27: i128 = 21;
        // C s_52_28: const #2s : i
        let s_52_28: i128 = 2;
        // D s_52_29: read-var u#23069:u32
        let s_52_29: u32 = fn_state.u_23069;
        // D s_52_30: cast zx s_52_29 -> bv
        let s_52_30: Bits = Bits::new(s_52_29 as u128, 32u16);
        // D s_52_31: bit-extract s_52_30 s_52_27 s_52_28
        let s_52_31: Bits = (Bits::new(
            ((s_52_30) >> (s_52_27)).value(),
            u16::try_from(s_52_28).unwrap(),
        ));
        // D s_52_32: cast reint s_52_31 -> u8
        let s_52_32: u8 = (s_52_31.value() as u8);
        // C s_52_33: const #24s : i
        let s_52_33: i128 = 24;
        // C s_52_34: const #1s : i
        let s_52_34: i128 = 1;
        // D s_52_35: read-var u#23069:u32
        let s_52_35: u32 = fn_state.u_23069;
        // D s_52_36: cast zx s_52_35 -> bv
        let s_52_36: Bits = Bits::new(s_52_35 as u128, 32u16);
        // D s_52_37: bit-extract s_52_36 s_52_33 s_52_34
        let s_52_37: Bits = (Bits::new(
            ((s_52_36) >> (s_52_33)).value(),
            u16::try_from(s_52_34).unwrap(),
        ));
        // D s_52_38: cast reint s_52_37 -> u8
        let s_52_38: bool = ((s_52_37.value()) != 0);
        // D s_52_39: call decode_blr_aarch64_instrs_branch_unconditional_register(s_52_8, s_52_14, s_52_20, s_52_26, s_52_32, s_52_38)
        let s_52_39: () = decode_blr_aarch64_instrs_branch_unconditional_register(
            state,
            tracer,
            s_52_8,
            s_52_14,
            s_52_20,
            s_52_26,
            s_52_32,
            s_52_38,
        );
        // N s_52_40: return
        return;
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var merge#var.1:struct
        let s_53_0: u32 = fn_state.merge_var._1;
        // D s_53_1: write-var u#23072 <= s_53_0
        fn_state.u_23072 = s_53_0;
        // C s_53_2: const #25s : i
        let s_53_2: i128 = 25;
        // D s_53_3: read-var u#23072:u32
        let s_53_3: u32 = fn_state.u_23072;
        // D s_53_4: cast zx s_53_3 -> bv
        let s_53_4: Bits = Bits::new(s_53_3 as u128, 32u16);
        // C s_53_5: const #1s : i64
        let s_53_5: i64 = 1;
        // C s_53_6: cast zx s_53_5 -> i
        let s_53_6: i128 = (i128::try_from(s_53_5).unwrap());
        // C s_53_7: const #6s : i
        let s_53_7: i128 = 6;
        // C s_53_8: add s_53_7 s_53_6
        let s_53_8: i128 = (s_53_7 + s_53_6);
        // D s_53_9: bit-extract s_53_4 s_53_2 s_53_8
        let s_53_9: Bits = (Bits::new(
            ((s_53_4) >> (s_53_2)).value(),
            u16::try_from(s_53_8).unwrap(),
        ));
        // D s_53_10: cast reint s_53_9 -> u8
        let s_53_10: u8 = (s_53_9.value() as u8);
        // D s_53_11: cast zx s_53_10 -> bv
        let s_53_11: Bits = Bits::new(s_53_10 as u128, 7u16);
        // C s_53_12: const #107u : u8
        let s_53_12: u8 = 107;
        // C s_53_13: cast zx s_53_12 -> bv
        let s_53_13: Bits = Bits::new(s_53_12 as u128, 7u16);
        // D s_53_14: cmp-eq s_53_11 s_53_13
        let s_53_14: bool = ((s_53_11) == (s_53_13));
        // N s_53_15: branch s_53_14 b475 b54
        if s_53_14 {
            return block_475(state, tracer, fn_state);
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
        // D s_54_1: write-var gs#373747 <= s_54_0
        fn_state.gs_373747 = s_54_0;
        // N s_54_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var gs#373747:u8
        let s_55_0: bool = fn_state.gs_373747;
        // N s_55_1: branch s_55_0 b474 b56
        if s_55_0 {
            return block_474(state, tracer, fn_state);
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
        // D s_56_1: write-var gs#373749 <= s_56_0
        fn_state.gs_373749 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#373749:u8
        let s_57_0: bool = fn_state.gs_373749;
        // D s_57_1: not s_57_0
        let s_57_1: bool = !s_57_0;
        // N s_57_2: branch s_57_1 b59 b58
        if s_57_1 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #91s : i
        let s_58_0: i128 = 91;
        // C s_58_1: const #14696u : u32
        let s_58_1: u32 = 14696;
        // N s_58_2: write-reg s_58_1 <= s_58_0
        let s_58_2: () = {
            state.write_register::<i128>(s_58_1 as isize, s_58_0);
            tracer.write_register(s_58_1 as isize, s_58_0);
        };
        // C s_58_3: const #0s : i
        let s_58_3: i128 = 0;
        // C s_58_4: const #5s : i
        let s_58_4: i128 = 5;
        // D s_58_5: read-var u#23072:u32
        let s_58_5: u32 = fn_state.u_23072;
        // D s_58_6: cast zx s_58_5 -> bv
        let s_58_6: Bits = Bits::new(s_58_5 as u128, 32u16);
        // D s_58_7: bit-extract s_58_6 s_58_3 s_58_4
        let s_58_7: Bits = (Bits::new(
            ((s_58_6) >> (s_58_3)).value(),
            u16::try_from(s_58_4).unwrap(),
        ));
        // D s_58_8: cast reint s_58_7 -> u8
        let s_58_8: u8 = (s_58_7.value() as u8);
        // C s_58_9: const #5s : i
        let s_58_9: i128 = 5;
        // C s_58_10: const #5s : i
        let s_58_10: i128 = 5;
        // D s_58_11: read-var u#23072:u32
        let s_58_11: u32 = fn_state.u_23072;
        // D s_58_12: cast zx s_58_11 -> bv
        let s_58_12: Bits = Bits::new(s_58_11 as u128, 32u16);
        // D s_58_13: bit-extract s_58_12 s_58_9 s_58_10
        let s_58_13: Bits = (Bits::new(
            ((s_58_12) >> (s_58_9)).value(),
            u16::try_from(s_58_10).unwrap(),
        ));
        // D s_58_14: cast reint s_58_13 -> u8
        let s_58_14: u8 = (s_58_13.value() as u8);
        // C s_58_15: const #10s : i
        let s_58_15: i128 = 10;
        // C s_58_16: const #1s : i
        let s_58_16: i128 = 1;
        // D s_58_17: read-var u#23072:u32
        let s_58_17: u32 = fn_state.u_23072;
        // D s_58_18: cast zx s_58_17 -> bv
        let s_58_18: Bits = Bits::new(s_58_17 as u128, 32u16);
        // D s_58_19: bit-extract s_58_18 s_58_15 s_58_16
        let s_58_19: Bits = (Bits::new(
            ((s_58_18) >> (s_58_15)).value(),
            u16::try_from(s_58_16).unwrap(),
        ));
        // D s_58_20: cast reint s_58_19 -> u8
        let s_58_20: bool = ((s_58_19.value()) != 0);
        // C s_58_21: const #11s : i
        let s_58_21: i128 = 11;
        // C s_58_22: const #1s : i
        let s_58_22: i128 = 1;
        // D s_58_23: read-var u#23072:u32
        let s_58_23: u32 = fn_state.u_23072;
        // D s_58_24: cast zx s_58_23 -> bv
        let s_58_24: Bits = Bits::new(s_58_23 as u128, 32u16);
        // D s_58_25: bit-extract s_58_24 s_58_21 s_58_22
        let s_58_25: Bits = (Bits::new(
            ((s_58_24) >> (s_58_21)).value(),
            u16::try_from(s_58_22).unwrap(),
        ));
        // D s_58_26: cast reint s_58_25 -> u8
        let s_58_26: bool = ((s_58_25.value()) != 0);
        // C s_58_27: const #21s : i
        let s_58_27: i128 = 21;
        // C s_58_28: const #2s : i
        let s_58_28: i128 = 2;
        // D s_58_29: read-var u#23072:u32
        let s_58_29: u32 = fn_state.u_23072;
        // D s_58_30: cast zx s_58_29 -> bv
        let s_58_30: Bits = Bits::new(s_58_29 as u128, 32u16);
        // D s_58_31: bit-extract s_58_30 s_58_27 s_58_28
        let s_58_31: Bits = (Bits::new(
            ((s_58_30) >> (s_58_27)).value(),
            u16::try_from(s_58_28).unwrap(),
        ));
        // D s_58_32: cast reint s_58_31 -> u8
        let s_58_32: u8 = (s_58_31.value() as u8);
        // C s_58_33: const #24s : i
        let s_58_33: i128 = 24;
        // C s_58_34: const #1s : i
        let s_58_34: i128 = 1;
        // D s_58_35: read-var u#23072:u32
        let s_58_35: u32 = fn_state.u_23072;
        // D s_58_36: cast zx s_58_35 -> bv
        let s_58_36: Bits = Bits::new(s_58_35 as u128, 32u16);
        // D s_58_37: bit-extract s_58_36 s_58_33 s_58_34
        let s_58_37: Bits = (Bits::new(
            ((s_58_36) >> (s_58_33)).value(),
            u16::try_from(s_58_34).unwrap(),
        ));
        // D s_58_38: cast reint s_58_37 -> u8
        let s_58_38: bool = ((s_58_37.value()) != 0);
        // D s_58_39: call decode_blra_aarch64_instrs_branch_unconditional_register(s_58_8, s_58_14, s_58_20, s_58_26, s_58_32, s_58_38)
        let s_58_39: () = decode_blra_aarch64_instrs_branch_unconditional_register(
            state,
            tracer,
            s_58_8,
            s_58_14,
            s_58_20,
            s_58_26,
            s_58_32,
            s_58_38,
        );
        // N s_58_40: return
        return;
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var merge#var.1:struct
        let s_59_0: u32 = fn_state.merge_var._1;
        // D s_59_1: write-var u#23080 <= s_59_0
        fn_state.u_23080 = s_59_0;
        // C s_59_2: const #10s : i
        let s_59_2: i128 = 10;
        // D s_59_3: read-var u#23080:u32
        let s_59_3: u32 = fn_state.u_23080;
        // D s_59_4: cast zx s_59_3 -> bv
        let s_59_4: Bits = Bits::new(s_59_3 as u128, 32u16);
        // C s_59_5: const #1s : i64
        let s_59_5: i64 = 1;
        // C s_59_6: cast zx s_59_5 -> i
        let s_59_6: i128 = (i128::try_from(s_59_5).unwrap());
        // C s_59_7: const #21s : i
        let s_59_7: i128 = 21;
        // C s_59_8: add s_59_7 s_59_6
        let s_59_8: i128 = (s_59_7 + s_59_6);
        // D s_59_9: bit-extract s_59_4 s_59_2 s_59_8
        let s_59_9: Bits = (Bits::new(
            ((s_59_4) >> (s_59_2)).value(),
            u16::try_from(s_59_8).unwrap(),
        ));
        // D s_59_10: cast reint s_59_9 -> u22
        let s_59_10: u32 = (s_59_9.value() as u32);
        // D s_59_11: cast zx s_59_10 -> bv
        let s_59_11: Bits = Bits::new(s_59_10 as u128, 22u16);
        // C s_59_12: const #3508160u : u22
        let s_59_12: u32 = 3508160;
        // C s_59_13: cast zx s_59_12 -> bv
        let s_59_13: Bits = Bits::new(s_59_12 as u128, 22u16);
        // D s_59_14: cmp-eq s_59_11 s_59_13
        let s_59_14: bool = ((s_59_11) == (s_59_13));
        // N s_59_15: branch s_59_14 b473 b60
        if s_59_14 {
            return block_473(state, tracer, fn_state);
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
        // D s_60_1: write-var gs#373769 <= s_60_0
        fn_state.gs_373769 = s_60_0;
        // N s_60_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var gs#373769:u8
        let s_61_0: bool = fn_state.gs_373769;
        // N s_61_1: branch s_61_0 b472 b62
        if s_61_0 {
            return block_472(state, tracer, fn_state);
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
        // D s_62_1: write-var gs#373771 <= s_62_0
        fn_state.gs_373771 = s_62_0;
        // N s_62_2: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var gs#373771:u8
        let s_63_0: bool = fn_state.gs_373771;
        // D s_63_1: not s_63_0
        let s_63_1: bool = !s_63_0;
        // N s_63_2: branch s_63_1 b65 b64
        if s_63_1 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_64(state, tracer, fn_state);
        };
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #92s : i
        let s_64_0: i128 = 92;
        // C s_64_1: const #14696u : u32
        let s_64_1: u32 = 14696;
        // N s_64_2: write-reg s_64_1 <= s_64_0
        let s_64_2: () = {
            state.write_register::<i128>(s_64_1 as isize, s_64_0);
            tracer.write_register(s_64_1 as isize, s_64_0);
        };
        // C s_64_3: const #0s : i
        let s_64_3: i128 = 0;
        // C s_64_4: const #5s : i
        let s_64_4: i128 = 5;
        // D s_64_5: read-var u#23080:u32
        let s_64_5: u32 = fn_state.u_23080;
        // D s_64_6: cast zx s_64_5 -> bv
        let s_64_6: Bits = Bits::new(s_64_5 as u128, 32u16);
        // D s_64_7: bit-extract s_64_6 s_64_3 s_64_4
        let s_64_7: Bits = (Bits::new(
            ((s_64_6) >> (s_64_3)).value(),
            u16::try_from(s_64_4).unwrap(),
        ));
        // D s_64_8: cast reint s_64_7 -> u8
        let s_64_8: u8 = (s_64_7.value() as u8);
        // C s_64_9: const #5s : i
        let s_64_9: i128 = 5;
        // C s_64_10: const #5s : i
        let s_64_10: i128 = 5;
        // D s_64_11: read-var u#23080:u32
        let s_64_11: u32 = fn_state.u_23080;
        // D s_64_12: cast zx s_64_11 -> bv
        let s_64_12: Bits = Bits::new(s_64_11 as u128, 32u16);
        // D s_64_13: bit-extract s_64_12 s_64_9 s_64_10
        let s_64_13: Bits = (Bits::new(
            ((s_64_12) >> (s_64_9)).value(),
            u16::try_from(s_64_10).unwrap(),
        ));
        // D s_64_14: cast reint s_64_13 -> u8
        let s_64_14: u8 = (s_64_13.value() as u8);
        // C s_64_15: const #10s : i
        let s_64_15: i128 = 10;
        // C s_64_16: const #1s : i
        let s_64_16: i128 = 1;
        // D s_64_17: read-var u#23080:u32
        let s_64_17: u32 = fn_state.u_23080;
        // D s_64_18: cast zx s_64_17 -> bv
        let s_64_18: Bits = Bits::new(s_64_17 as u128, 32u16);
        // D s_64_19: bit-extract s_64_18 s_64_15 s_64_16
        let s_64_19: Bits = (Bits::new(
            ((s_64_18) >> (s_64_15)).value(),
            u16::try_from(s_64_16).unwrap(),
        ));
        // D s_64_20: cast reint s_64_19 -> u8
        let s_64_20: bool = ((s_64_19.value()) != 0);
        // C s_64_21: const #11s : i
        let s_64_21: i128 = 11;
        // C s_64_22: const #1s : i
        let s_64_22: i128 = 1;
        // D s_64_23: read-var u#23080:u32
        let s_64_23: u32 = fn_state.u_23080;
        // D s_64_24: cast zx s_64_23 -> bv
        let s_64_24: Bits = Bits::new(s_64_23 as u128, 32u16);
        // D s_64_25: bit-extract s_64_24 s_64_21 s_64_22
        let s_64_25: Bits = (Bits::new(
            ((s_64_24) >> (s_64_21)).value(),
            u16::try_from(s_64_22).unwrap(),
        ));
        // D s_64_26: cast reint s_64_25 -> u8
        let s_64_26: bool = ((s_64_25.value()) != 0);
        // C s_64_27: const #21s : i
        let s_64_27: i128 = 21;
        // C s_64_28: const #2s : i
        let s_64_28: i128 = 2;
        // D s_64_29: read-var u#23080:u32
        let s_64_29: u32 = fn_state.u_23080;
        // D s_64_30: cast zx s_64_29 -> bv
        let s_64_30: Bits = Bits::new(s_64_29 as u128, 32u16);
        // D s_64_31: bit-extract s_64_30 s_64_27 s_64_28
        let s_64_31: Bits = (Bits::new(
            ((s_64_30) >> (s_64_27)).value(),
            u16::try_from(s_64_28).unwrap(),
        ));
        // D s_64_32: cast reint s_64_31 -> u8
        let s_64_32: u8 = (s_64_31.value() as u8);
        // C s_64_33: const #24s : i
        let s_64_33: i128 = 24;
        // C s_64_34: const #1s : i
        let s_64_34: i128 = 1;
        // D s_64_35: read-var u#23080:u32
        let s_64_35: u32 = fn_state.u_23080;
        // D s_64_36: cast zx s_64_35 -> bv
        let s_64_36: Bits = Bits::new(s_64_35 as u128, 32u16);
        // D s_64_37: bit-extract s_64_36 s_64_33 s_64_34
        let s_64_37: Bits = (Bits::new(
            ((s_64_36) >> (s_64_33)).value(),
            u16::try_from(s_64_34).unwrap(),
        ));
        // D s_64_38: cast reint s_64_37 -> u8
        let s_64_38: bool = ((s_64_37.value()) != 0);
        // D s_64_39: call decode_br_aarch64_instrs_branch_unconditional_register(s_64_8, s_64_14, s_64_20, s_64_26, s_64_32, s_64_38)
        let s_64_39: () = decode_br_aarch64_instrs_branch_unconditional_register(
            state,
            tracer,
            s_64_8,
            s_64_14,
            s_64_20,
            s_64_26,
            s_64_32,
            s_64_38,
        );
        // N s_64_40: return
        return;
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var merge#var.1:struct
        let s_65_0: u32 = fn_state.merge_var._1;
        // D s_65_1: write-var u#23088 <= s_65_0
        fn_state.u_23088 = s_65_0;
        // C s_65_2: const #25s : i
        let s_65_2: i128 = 25;
        // D s_65_3: read-var u#23088:u32
        let s_65_3: u32 = fn_state.u_23088;
        // D s_65_4: cast zx s_65_3 -> bv
        let s_65_4: Bits = Bits::new(s_65_3 as u128, 32u16);
        // C s_65_5: const #1s : i64
        let s_65_5: i64 = 1;
        // C s_65_6: cast zx s_65_5 -> i
        let s_65_6: i128 = (i128::try_from(s_65_5).unwrap());
        // C s_65_7: const #6s : i
        let s_65_7: i128 = 6;
        // C s_65_8: add s_65_7 s_65_6
        let s_65_8: i128 = (s_65_7 + s_65_6);
        // D s_65_9: bit-extract s_65_4 s_65_2 s_65_8
        let s_65_9: Bits = (Bits::new(
            ((s_65_4) >> (s_65_2)).value(),
            u16::try_from(s_65_8).unwrap(),
        ));
        // D s_65_10: cast reint s_65_9 -> u8
        let s_65_10: u8 = (s_65_9.value() as u8);
        // D s_65_11: cast zx s_65_10 -> bv
        let s_65_11: Bits = Bits::new(s_65_10 as u128, 7u16);
        // C s_65_12: const #107u : u8
        let s_65_12: u8 = 107;
        // C s_65_13: cast zx s_65_12 -> bv
        let s_65_13: Bits = Bits::new(s_65_12 as u128, 7u16);
        // D s_65_14: cmp-eq s_65_11 s_65_13
        let s_65_14: bool = ((s_65_11) == (s_65_13));
        // N s_65_15: branch s_65_14 b471 b66
        if s_65_14 {
            return block_471(state, tracer, fn_state);
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
        // D s_66_1: write-var gs#373791 <= s_66_0
        fn_state.gs_373791 = s_66_0;
        // N s_66_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var gs#373791:u8
        let s_67_0: bool = fn_state.gs_373791;
        // N s_67_1: branch s_67_0 b470 b68
        if s_67_0 {
            return block_470(state, tracer, fn_state);
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
        // D s_68_1: write-var gs#373793 <= s_68_0
        fn_state.gs_373793 = s_68_0;
        // N s_68_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var gs#373793:u8
        let s_69_0: bool = fn_state.gs_373793;
        // D s_69_1: not s_69_0
        let s_69_1: bool = !s_69_0;
        // N s_69_2: branch s_69_1 b71 b70
        if s_69_1 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_70(state, tracer, fn_state);
        };
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #93s : i
        let s_70_0: i128 = 93;
        // C s_70_1: const #14696u : u32
        let s_70_1: u32 = 14696;
        // N s_70_2: write-reg s_70_1 <= s_70_0
        let s_70_2: () = {
            state.write_register::<i128>(s_70_1 as isize, s_70_0);
            tracer.write_register(s_70_1 as isize, s_70_0);
        };
        // C s_70_3: const #0s : i
        let s_70_3: i128 = 0;
        // C s_70_4: const #5s : i
        let s_70_4: i128 = 5;
        // D s_70_5: read-var u#23088:u32
        let s_70_5: u32 = fn_state.u_23088;
        // D s_70_6: cast zx s_70_5 -> bv
        let s_70_6: Bits = Bits::new(s_70_5 as u128, 32u16);
        // D s_70_7: bit-extract s_70_6 s_70_3 s_70_4
        let s_70_7: Bits = (Bits::new(
            ((s_70_6) >> (s_70_3)).value(),
            u16::try_from(s_70_4).unwrap(),
        ));
        // D s_70_8: cast reint s_70_7 -> u8
        let s_70_8: u8 = (s_70_7.value() as u8);
        // C s_70_9: const #5s : i
        let s_70_9: i128 = 5;
        // C s_70_10: const #5s : i
        let s_70_10: i128 = 5;
        // D s_70_11: read-var u#23088:u32
        let s_70_11: u32 = fn_state.u_23088;
        // D s_70_12: cast zx s_70_11 -> bv
        let s_70_12: Bits = Bits::new(s_70_11 as u128, 32u16);
        // D s_70_13: bit-extract s_70_12 s_70_9 s_70_10
        let s_70_13: Bits = (Bits::new(
            ((s_70_12) >> (s_70_9)).value(),
            u16::try_from(s_70_10).unwrap(),
        ));
        // D s_70_14: cast reint s_70_13 -> u8
        let s_70_14: u8 = (s_70_13.value() as u8);
        // C s_70_15: const #10s : i
        let s_70_15: i128 = 10;
        // C s_70_16: const #1s : i
        let s_70_16: i128 = 1;
        // D s_70_17: read-var u#23088:u32
        let s_70_17: u32 = fn_state.u_23088;
        // D s_70_18: cast zx s_70_17 -> bv
        let s_70_18: Bits = Bits::new(s_70_17 as u128, 32u16);
        // D s_70_19: bit-extract s_70_18 s_70_15 s_70_16
        let s_70_19: Bits = (Bits::new(
            ((s_70_18) >> (s_70_15)).value(),
            u16::try_from(s_70_16).unwrap(),
        ));
        // D s_70_20: cast reint s_70_19 -> u8
        let s_70_20: bool = ((s_70_19.value()) != 0);
        // C s_70_21: const #11s : i
        let s_70_21: i128 = 11;
        // C s_70_22: const #1s : i
        let s_70_22: i128 = 1;
        // D s_70_23: read-var u#23088:u32
        let s_70_23: u32 = fn_state.u_23088;
        // D s_70_24: cast zx s_70_23 -> bv
        let s_70_24: Bits = Bits::new(s_70_23 as u128, 32u16);
        // D s_70_25: bit-extract s_70_24 s_70_21 s_70_22
        let s_70_25: Bits = (Bits::new(
            ((s_70_24) >> (s_70_21)).value(),
            u16::try_from(s_70_22).unwrap(),
        ));
        // D s_70_26: cast reint s_70_25 -> u8
        let s_70_26: bool = ((s_70_25.value()) != 0);
        // C s_70_27: const #21s : i
        let s_70_27: i128 = 21;
        // C s_70_28: const #2s : i
        let s_70_28: i128 = 2;
        // D s_70_29: read-var u#23088:u32
        let s_70_29: u32 = fn_state.u_23088;
        // D s_70_30: cast zx s_70_29 -> bv
        let s_70_30: Bits = Bits::new(s_70_29 as u128, 32u16);
        // D s_70_31: bit-extract s_70_30 s_70_27 s_70_28
        let s_70_31: Bits = (Bits::new(
            ((s_70_30) >> (s_70_27)).value(),
            u16::try_from(s_70_28).unwrap(),
        ));
        // D s_70_32: cast reint s_70_31 -> u8
        let s_70_32: u8 = (s_70_31.value() as u8);
        // C s_70_33: const #24s : i
        let s_70_33: i128 = 24;
        // C s_70_34: const #1s : i
        let s_70_34: i128 = 1;
        // D s_70_35: read-var u#23088:u32
        let s_70_35: u32 = fn_state.u_23088;
        // D s_70_36: cast zx s_70_35 -> bv
        let s_70_36: Bits = Bits::new(s_70_35 as u128, 32u16);
        // D s_70_37: bit-extract s_70_36 s_70_33 s_70_34
        let s_70_37: Bits = (Bits::new(
            ((s_70_36) >> (s_70_33)).value(),
            u16::try_from(s_70_34).unwrap(),
        ));
        // D s_70_38: cast reint s_70_37 -> u8
        let s_70_38: bool = ((s_70_37.value()) != 0);
        // D s_70_39: call decode_bra_aarch64_instrs_branch_unconditional_register(s_70_8, s_70_14, s_70_20, s_70_26, s_70_32, s_70_38)
        let s_70_39: () = decode_bra_aarch64_instrs_branch_unconditional_register(
            state,
            tracer,
            s_70_8,
            s_70_14,
            s_70_20,
            s_70_26,
            s_70_32,
            s_70_38,
        );
        // N s_70_40: return
        return;
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var merge#var.1:struct
        let s_71_0: u32 = fn_state.merge_var._1;
        // D s_71_1: write-var u#23096 <= s_71_0
        fn_state.u_23096 = s_71_0;
        // C s_71_2: const #10s : i
        let s_71_2: i128 = 10;
        // D s_71_3: read-var u#23096:u32
        let s_71_3: u32 = fn_state.u_23096;
        // D s_71_4: cast zx s_71_3 -> bv
        let s_71_4: Bits = Bits::new(s_71_3 as u128, 32u16);
        // C s_71_5: const #1s : i64
        let s_71_5: i64 = 1;
        // C s_71_6: cast zx s_71_5 -> i
        let s_71_6: i128 = (i128::try_from(s_71_5).unwrap());
        // C s_71_7: const #21s : i
        let s_71_7: i128 = 21;
        // C s_71_8: add s_71_7 s_71_6
        let s_71_8: i128 = (s_71_7 + s_71_6);
        // D s_71_9: bit-extract s_71_4 s_71_2 s_71_8
        let s_71_9: Bits = (Bits::new(
            ((s_71_4) >> (s_71_2)).value(),
            u16::try_from(s_71_8).unwrap(),
        ));
        // D s_71_10: cast reint s_71_9 -> u22
        let s_71_10: u32 = (s_71_9.value() as u32);
        // D s_71_11: cast zx s_71_10 -> bv
        let s_71_11: Bits = Bits::new(s_71_10 as u128, 22u16);
        // C s_71_12: const #3512256u : u22
        let s_71_12: u32 = 3512256;
        // C s_71_13: cast zx s_71_12 -> bv
        let s_71_13: Bits = Bits::new(s_71_12 as u128, 22u16);
        // D s_71_14: cmp-eq s_71_11 s_71_13
        let s_71_14: bool = ((s_71_11) == (s_71_13));
        // N s_71_15: branch s_71_14 b469 b72
        if s_71_14 {
            return block_469(state, tracer, fn_state);
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
        // D s_72_1: write-var gs#373813 <= s_72_0
        fn_state.gs_373813 = s_72_0;
        // N s_72_2: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var gs#373813:u8
        let s_73_0: bool = fn_state.gs_373813;
        // N s_73_1: branch s_73_0 b468 b74
        if s_73_0 {
            return block_468(state, tracer, fn_state);
        } else {
            return block_74(state, tracer, fn_state);
        };
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #0u : u8
        let s_74_0: bool = false;
        // D s_74_1: write-var gs#373815 <= s_74_0
        fn_state.gs_373815 = s_74_0;
        // N s_74_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var gs#373815:u8
        let s_75_0: bool = fn_state.gs_373815;
        // D s_75_1: not s_75_0
        let s_75_1: bool = !s_75_0;
        // N s_75_2: branch s_75_1 b77 b76
        if s_75_1 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_76(state, tracer, fn_state);
        };
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #94s : i
        let s_76_0: i128 = 94;
        // C s_76_1: const #14696u : u32
        let s_76_1: u32 = 14696;
        // N s_76_2: write-reg s_76_1 <= s_76_0
        let s_76_2: () = {
            state.write_register::<i128>(s_76_1 as isize, s_76_0);
            tracer.write_register(s_76_1 as isize, s_76_0);
        };
        // C s_76_3: const #0s : i
        let s_76_3: i128 = 0;
        // C s_76_4: const #5s : i
        let s_76_4: i128 = 5;
        // D s_76_5: read-var u#23096:u32
        let s_76_5: u32 = fn_state.u_23096;
        // D s_76_6: cast zx s_76_5 -> bv
        let s_76_6: Bits = Bits::new(s_76_5 as u128, 32u16);
        // D s_76_7: bit-extract s_76_6 s_76_3 s_76_4
        let s_76_7: Bits = (Bits::new(
            ((s_76_6) >> (s_76_3)).value(),
            u16::try_from(s_76_4).unwrap(),
        ));
        // D s_76_8: cast reint s_76_7 -> u8
        let s_76_8: u8 = (s_76_7.value() as u8);
        // C s_76_9: const #5s : i
        let s_76_9: i128 = 5;
        // C s_76_10: const #5s : i
        let s_76_10: i128 = 5;
        // D s_76_11: read-var u#23096:u32
        let s_76_11: u32 = fn_state.u_23096;
        // D s_76_12: cast zx s_76_11 -> bv
        let s_76_12: Bits = Bits::new(s_76_11 as u128, 32u16);
        // D s_76_13: bit-extract s_76_12 s_76_9 s_76_10
        let s_76_13: Bits = (Bits::new(
            ((s_76_12) >> (s_76_9)).value(),
            u16::try_from(s_76_10).unwrap(),
        ));
        // D s_76_14: cast reint s_76_13 -> u8
        let s_76_14: u8 = (s_76_13.value() as u8);
        // C s_76_15: const #10s : i
        let s_76_15: i128 = 10;
        // C s_76_16: const #1s : i
        let s_76_16: i128 = 1;
        // D s_76_17: read-var u#23096:u32
        let s_76_17: u32 = fn_state.u_23096;
        // D s_76_18: cast zx s_76_17 -> bv
        let s_76_18: Bits = Bits::new(s_76_17 as u128, 32u16);
        // D s_76_19: bit-extract s_76_18 s_76_15 s_76_16
        let s_76_19: Bits = (Bits::new(
            ((s_76_18) >> (s_76_15)).value(),
            u16::try_from(s_76_16).unwrap(),
        ));
        // D s_76_20: cast reint s_76_19 -> u8
        let s_76_20: bool = ((s_76_19.value()) != 0);
        // C s_76_21: const #11s : i
        let s_76_21: i128 = 11;
        // C s_76_22: const #1s : i
        let s_76_22: i128 = 1;
        // D s_76_23: read-var u#23096:u32
        let s_76_23: u32 = fn_state.u_23096;
        // D s_76_24: cast zx s_76_23 -> bv
        let s_76_24: Bits = Bits::new(s_76_23 as u128, 32u16);
        // D s_76_25: bit-extract s_76_24 s_76_21 s_76_22
        let s_76_25: Bits = (Bits::new(
            ((s_76_24) >> (s_76_21)).value(),
            u16::try_from(s_76_22).unwrap(),
        ));
        // D s_76_26: cast reint s_76_25 -> u8
        let s_76_26: bool = ((s_76_25.value()) != 0);
        // C s_76_27: const #21s : i
        let s_76_27: i128 = 21;
        // C s_76_28: const #2s : i
        let s_76_28: i128 = 2;
        // D s_76_29: read-var u#23096:u32
        let s_76_29: u32 = fn_state.u_23096;
        // D s_76_30: cast zx s_76_29 -> bv
        let s_76_30: Bits = Bits::new(s_76_29 as u128, 32u16);
        // D s_76_31: bit-extract s_76_30 s_76_27 s_76_28
        let s_76_31: Bits = (Bits::new(
            ((s_76_30) >> (s_76_27)).value(),
            u16::try_from(s_76_28).unwrap(),
        ));
        // D s_76_32: cast reint s_76_31 -> u8
        let s_76_32: u8 = (s_76_31.value() as u8);
        // C s_76_33: const #24s : i
        let s_76_33: i128 = 24;
        // C s_76_34: const #1s : i
        let s_76_34: i128 = 1;
        // D s_76_35: read-var u#23096:u32
        let s_76_35: u32 = fn_state.u_23096;
        // D s_76_36: cast zx s_76_35 -> bv
        let s_76_36: Bits = Bits::new(s_76_35 as u128, 32u16);
        // D s_76_37: bit-extract s_76_36 s_76_33 s_76_34
        let s_76_37: Bits = (Bits::new(
            ((s_76_36) >> (s_76_33)).value(),
            u16::try_from(s_76_34).unwrap(),
        ));
        // D s_76_38: cast reint s_76_37 -> u8
        let s_76_38: bool = ((s_76_37.value()) != 0);
        // D s_76_39: call decode_ret_aarch64_instrs_branch_unconditional_register(s_76_8, s_76_14, s_76_20, s_76_26, s_76_32, s_76_38)
        let s_76_39: () = decode_ret_aarch64_instrs_branch_unconditional_register(
            state,
            tracer,
            s_76_8,
            s_76_14,
            s_76_20,
            s_76_26,
            s_76_32,
            s_76_38,
        );
        // N s_76_40: return
        return;
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var merge#var.1:struct
        let s_77_0: u32 = fn_state.merge_var._1;
        // D s_77_1: write-var u#23104 <= s_77_0
        fn_state.u_23104 = s_77_0;
        // C s_77_2: const #11s : i
        let s_77_2: i128 = 11;
        // D s_77_3: read-var u#23104:u32
        let s_77_3: u32 = fn_state.u_23104;
        // D s_77_4: cast zx s_77_3 -> bv
        let s_77_4: Bits = Bits::new(s_77_3 as u128, 32u16);
        // C s_77_5: const #1s : i64
        let s_77_5: i64 = 1;
        // C s_77_6: cast zx s_77_5 -> i
        let s_77_6: i128 = (i128::try_from(s_77_5).unwrap());
        // C s_77_7: const #20s : i
        let s_77_7: i128 = 20;
        // C s_77_8: add s_77_7 s_77_6
        let s_77_8: i128 = (s_77_7 + s_77_6);
        // D s_77_9: bit-extract s_77_4 s_77_2 s_77_8
        let s_77_9: Bits = (Bits::new(
            ((s_77_4) >> (s_77_2)).value(),
            u16::try_from(s_77_8).unwrap(),
        ));
        // D s_77_10: cast reint s_77_9 -> u21
        let s_77_10: u32 = (s_77_9.value() as u32);
        // D s_77_11: cast zx s_77_10 -> bv
        let s_77_11: Bits = Bits::new(s_77_10 as u128, 21u16);
        // C s_77_12: const #1756129u : u21
        let s_77_12: u32 = 1756129;
        // C s_77_13: cast zx s_77_12 -> bv
        let s_77_13: Bits = Bits::new(s_77_12 as u128, 21u16);
        // D s_77_14: cmp-eq s_77_11 s_77_13
        let s_77_14: bool = ((s_77_11) == (s_77_13));
        // N s_77_15: branch s_77_14 b467 b78
        if s_77_14 {
            return block_467(state, tracer, fn_state);
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
        // D s_78_1: write-var gs#373835 <= s_78_0
        fn_state.gs_373835 = s_78_0;
        // N s_78_2: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var gs#373835:u8
        let s_79_0: bool = fn_state.gs_373835;
        // N s_79_1: branch s_79_0 b466 b80
        if s_79_0 {
            return block_466(state, tracer, fn_state);
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
        // D s_80_1: write-var gs#373837 <= s_80_0
        fn_state.gs_373837 = s_80_0;
        // N s_80_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var gs#373837:u8
        let s_81_0: bool = fn_state.gs_373837;
        // D s_81_1: not s_81_0
        let s_81_1: bool = !s_81_0;
        // N s_81_2: branch s_81_1 b83 b82
        if s_81_1 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_82(state, tracer, fn_state);
        };
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #95s : i
        let s_82_0: i128 = 95;
        // C s_82_1: const #14696u : u32
        let s_82_1: u32 = 14696;
        // N s_82_2: write-reg s_82_1 <= s_82_0
        let s_82_2: () = {
            state.write_register::<i128>(s_82_1 as isize, s_82_0);
            tracer.write_register(s_82_1 as isize, s_82_0);
        };
        // C s_82_3: const #0s : i
        let s_82_3: i128 = 0;
        // C s_82_4: const #5s : i
        let s_82_4: i128 = 5;
        // D s_82_5: read-var u#23104:u32
        let s_82_5: u32 = fn_state.u_23104;
        // D s_82_6: cast zx s_82_5 -> bv
        let s_82_6: Bits = Bits::new(s_82_5 as u128, 32u16);
        // D s_82_7: bit-extract s_82_6 s_82_3 s_82_4
        let s_82_7: Bits = (Bits::new(
            ((s_82_6) >> (s_82_3)).value(),
            u16::try_from(s_82_4).unwrap(),
        ));
        // D s_82_8: cast reint s_82_7 -> u8
        let s_82_8: u8 = (s_82_7.value() as u8);
        // C s_82_9: const #5s : i
        let s_82_9: i128 = 5;
        // C s_82_10: const #5s : i
        let s_82_10: i128 = 5;
        // D s_82_11: read-var u#23104:u32
        let s_82_11: u32 = fn_state.u_23104;
        // D s_82_12: cast zx s_82_11 -> bv
        let s_82_12: Bits = Bits::new(s_82_11 as u128, 32u16);
        // D s_82_13: bit-extract s_82_12 s_82_9 s_82_10
        let s_82_13: Bits = (Bits::new(
            ((s_82_12) >> (s_82_9)).value(),
            u16::try_from(s_82_10).unwrap(),
        ));
        // D s_82_14: cast reint s_82_13 -> u8
        let s_82_14: u8 = (s_82_13.value() as u8);
        // C s_82_15: const #10s : i
        let s_82_15: i128 = 10;
        // C s_82_16: const #1s : i
        let s_82_16: i128 = 1;
        // D s_82_17: read-var u#23104:u32
        let s_82_17: u32 = fn_state.u_23104;
        // D s_82_18: cast zx s_82_17 -> bv
        let s_82_18: Bits = Bits::new(s_82_17 as u128, 32u16);
        // D s_82_19: bit-extract s_82_18 s_82_15 s_82_16
        let s_82_19: Bits = (Bits::new(
            ((s_82_18) >> (s_82_15)).value(),
            u16::try_from(s_82_16).unwrap(),
        ));
        // D s_82_20: cast reint s_82_19 -> u8
        let s_82_20: bool = ((s_82_19.value()) != 0);
        // C s_82_21: const #11s : i
        let s_82_21: i128 = 11;
        // C s_82_22: const #1s : i
        let s_82_22: i128 = 1;
        // D s_82_23: read-var u#23104:u32
        let s_82_23: u32 = fn_state.u_23104;
        // D s_82_24: cast zx s_82_23 -> bv
        let s_82_24: Bits = Bits::new(s_82_23 as u128, 32u16);
        // D s_82_25: bit-extract s_82_24 s_82_21 s_82_22
        let s_82_25: Bits = (Bits::new(
            ((s_82_24) >> (s_82_21)).value(),
            u16::try_from(s_82_22).unwrap(),
        ));
        // D s_82_26: cast reint s_82_25 -> u8
        let s_82_26: bool = ((s_82_25.value()) != 0);
        // C s_82_27: const #21s : i
        let s_82_27: i128 = 21;
        // C s_82_28: const #2s : i
        let s_82_28: i128 = 2;
        // D s_82_29: read-var u#23104:u32
        let s_82_29: u32 = fn_state.u_23104;
        // D s_82_30: cast zx s_82_29 -> bv
        let s_82_30: Bits = Bits::new(s_82_29 as u128, 32u16);
        // D s_82_31: bit-extract s_82_30 s_82_27 s_82_28
        let s_82_31: Bits = (Bits::new(
            ((s_82_30) >> (s_82_27)).value(),
            u16::try_from(s_82_28).unwrap(),
        ));
        // D s_82_32: cast reint s_82_31 -> u8
        let s_82_32: u8 = (s_82_31.value() as u8);
        // C s_82_33: const #24s : i
        let s_82_33: i128 = 24;
        // C s_82_34: const #1s : i
        let s_82_34: i128 = 1;
        // D s_82_35: read-var u#23104:u32
        let s_82_35: u32 = fn_state.u_23104;
        // D s_82_36: cast zx s_82_35 -> bv
        let s_82_36: Bits = Bits::new(s_82_35 as u128, 32u16);
        // D s_82_37: bit-extract s_82_36 s_82_33 s_82_34
        let s_82_37: Bits = (Bits::new(
            ((s_82_36) >> (s_82_33)).value(),
            u16::try_from(s_82_34).unwrap(),
        ));
        // D s_82_38: cast reint s_82_37 -> u8
        let s_82_38: bool = ((s_82_37.value()) != 0);
        // D s_82_39: call decode_reta_aarch64_instrs_branch_unconditional_register(s_82_8, s_82_14, s_82_20, s_82_26, s_82_32, s_82_38)
        let s_82_39: () = decode_reta_aarch64_instrs_branch_unconditional_register(
            state,
            tracer,
            s_82_8,
            s_82_14,
            s_82_20,
            s_82_26,
            s_82_32,
            s_82_38,
        );
        // N s_82_40: return
        return;
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_83_0: read-var merge#var.1:struct
        let s_83_0: u32 = fn_state.merge_var._1;
        // D s_83_1: write-var u#23112 <= s_83_0
        fn_state.u_23112 = s_83_0;
        // C s_83_2: const #21s : i
        let s_83_2: i128 = 21;
        // D s_83_3: read-var u#23112:u32
        let s_83_3: u32 = fn_state.u_23112;
        // D s_83_4: cast zx s_83_3 -> bv
        let s_83_4: Bits = Bits::new(s_83_3 as u128, 32u16);
        // C s_83_5: const #1s : i64
        let s_83_5: i64 = 1;
        // C s_83_6: cast zx s_83_5 -> i
        let s_83_6: i128 = (i128::try_from(s_83_5).unwrap());
        // C s_83_7: const #10s : i
        let s_83_7: i128 = 10;
        // C s_83_8: add s_83_7 s_83_6
        let s_83_8: i128 = (s_83_7 + s_83_6);
        // D s_83_9: bit-extract s_83_4 s_83_2 s_83_8
        let s_83_9: Bits = (Bits::new(
            ((s_83_4) >> (s_83_2)).value(),
            u16::try_from(s_83_8).unwrap(),
        ));
        // D s_83_10: cast reint s_83_9 -> u11
        let s_83_10: u16 = (s_83_9.value() as u16);
        // D s_83_11: cast zx s_83_10 -> bv
        let s_83_11: Bits = Bits::new(s_83_10 as u128, 11u16);
        // C s_83_12: const #1697u : u11
        let s_83_12: u16 = 1697;
        // C s_83_13: cast zx s_83_12 -> bv
        let s_83_13: Bits = Bits::new(s_83_12 as u128, 11u16);
        // D s_83_14: cmp-eq s_83_11 s_83_13
        let s_83_14: bool = ((s_83_11) == (s_83_13));
        // N s_83_15: branch s_83_14 b465 b84
        if s_83_14 {
            return block_465(state, tracer, fn_state);
        } else {
            return block_84(state, tracer, fn_state);
        };
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #0u : u8
        let s_84_0: bool = false;
        // D s_84_1: write-var gs#373857 <= s_84_0
        fn_state.gs_373857 = s_84_0;
        // N s_84_2: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_85_0: read-var gs#373857:u8
        let s_85_0: bool = fn_state.gs_373857;
        // N s_85_1: branch s_85_0 b464 b86
        if s_85_0 {
            return block_464(state, tracer, fn_state);
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
        // D s_86_1: write-var gs#373859 <= s_86_0
        fn_state.gs_373859 = s_86_0;
        // N s_86_2: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_87_0: read-var gs#373859:u8
        let s_87_0: bool = fn_state.gs_373859;
        // D s_87_1: not s_87_0
        let s_87_1: bool = !s_87_0;
        // N s_87_2: branch s_87_1 b89 b88
        if s_87_1 {
            return block_89(state, tracer, fn_state);
        } else {
            return block_88(state, tracer, fn_state);
        };
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #96s : i
        let s_88_0: i128 = 96;
        // C s_88_1: const #14696u : u32
        let s_88_1: u32 = 14696;
        // N s_88_2: write-reg s_88_1 <= s_88_0
        let s_88_2: () = {
            state.write_register::<i128>(s_88_1 as isize, s_88_0);
            tracer.write_register(s_88_1 as isize, s_88_0);
        };
        // C s_88_3: const #5s : i
        let s_88_3: i128 = 5;
        // C s_88_4: const #16s : i
        let s_88_4: i128 = 16;
        // D s_88_5: read-var u#23112:u32
        let s_88_5: u32 = fn_state.u_23112;
        // D s_88_6: cast zx s_88_5 -> bv
        let s_88_6: Bits = Bits::new(s_88_5 as u128, 32u16);
        // D s_88_7: bit-extract s_88_6 s_88_3 s_88_4
        let s_88_7: Bits = (Bits::new(
            ((s_88_6) >> (s_88_3)).value(),
            u16::try_from(s_88_4).unwrap(),
        ));
        // D s_88_8: cast reint s_88_7 -> u16
        let s_88_8: u16 = (s_88_7.value() as u16);
        // D s_88_9: call decode_brk_aarch64_instrs_system_exceptions_debug_breakpoint(s_88_8)
        let s_88_9: () = decode_brk_aarch64_instrs_system_exceptions_debug_breakpoint(
            state,
            tracer,
            s_88_8,
        );
        // N s_88_10: return
        return;
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_89_0: read-var merge#var.1:struct
        let s_89_0: u32 = fn_state.merge_var._1;
        // D s_89_1: write-var u#23114 <= s_89_0
        fn_state.u_23114 = s_89_0;
        // C s_89_2: const #8s : i
        let s_89_2: i128 = 8;
        // D s_89_3: read-var u#23114:u32
        let s_89_3: u32 = fn_state.u_23114;
        // D s_89_4: cast zx s_89_3 -> bv
        let s_89_4: Bits = Bits::new(s_89_3 as u128, 32u16);
        // C s_89_5: const #1s : i64
        let s_89_5: i64 = 1;
        // C s_89_6: cast zx s_89_5 -> i
        let s_89_6: i128 = (i128::try_from(s_89_5).unwrap());
        // C s_89_7: const #23s : i
        let s_89_7: i128 = 23;
        // C s_89_8: add s_89_7 s_89_6
        let s_89_8: i128 = (s_89_7 + s_89_6);
        // D s_89_9: bit-extract s_89_4 s_89_2 s_89_8
        let s_89_9: Bits = (Bits::new(
            ((s_89_4) >> (s_89_2)).value(),
            u16::try_from(s_89_8).unwrap(),
        ));
        // D s_89_10: cast reint s_89_9 -> u24
        let s_89_10: u32 = (s_89_9.value() as u32);
        // D s_89_11: cast zx s_89_10 -> bv
        let s_89_11: Bits = Bits::new(s_89_10 as u128, 24u16);
        // C s_89_12: const #13959972u : u24
        let s_89_12: u32 = 13959972;
        // C s_89_13: cast zx s_89_12 -> bv
        let s_89_13: Bits = Bits::new(s_89_12 as u128, 24u16);
        // D s_89_14: cmp-eq s_89_11 s_89_13
        let s_89_14: bool = ((s_89_11) == (s_89_13));
        // N s_89_15: branch s_89_14 b463 b90
        if s_89_14 {
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
        // D s_90_1: write-var gs#373869 <= s_90_0
        fn_state.gs_373869 = s_90_0;
        // N s_90_2: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_91_0: read-var gs#373869:u8
        let s_91_0: bool = fn_state.gs_373869;
        // N s_91_1: branch s_91_0 b462 b92
        if s_91_0 {
            return block_462(state, tracer, fn_state);
        } else {
            return block_92(state, tracer, fn_state);
        };
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #0u : u8
        let s_92_0: bool = false;
        // D s_92_1: write-var gs#373871 <= s_92_0
        fn_state.gs_373871 = s_92_0;
        // N s_92_2: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_93_0: read-var gs#373871:u8
        let s_93_0: bool = fn_state.gs_373871;
        // D s_93_1: not s_93_0
        let s_93_1: bool = !s_93_0;
        // N s_93_2: branch s_93_1 b95 b94
        if s_93_1 {
            return block_95(state, tracer, fn_state);
        } else {
            return block_94(state, tracer, fn_state);
        };
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #97s : i
        let s_94_0: i128 = 97;
        // C s_94_1: const #14696u : u32
        let s_94_1: u32 = 14696;
        // N s_94_2: write-reg s_94_1 <= s_94_0
        let s_94_2: () = {
            state.write_register::<i128>(s_94_1 as isize, s_94_0);
            tracer.write_register(s_94_1 as isize, s_94_0);
        };
        // C s_94_3: const #5s : i
        let s_94_3: i128 = 5;
        // C s_94_4: const #3s : i
        let s_94_4: i128 = 3;
        // D s_94_5: read-var u#23114:u32
        let s_94_5: u32 = fn_state.u_23114;
        // D s_94_6: cast zx s_94_5 -> bv
        let s_94_6: Bits = Bits::new(s_94_5 as u128, 32u16);
        // D s_94_7: bit-extract s_94_6 s_94_3 s_94_4
        let s_94_7: Bits = (Bits::new(
            ((s_94_6) >> (s_94_3)).value(),
            u16::try_from(s_94_4).unwrap(),
        ));
        // D s_94_8: cast reint s_94_7 -> u8
        let s_94_8: u8 = (s_94_7.value() as u8);
        // C s_94_9: const #8s : i
        let s_94_9: i128 = 8;
        // C s_94_10: const #4s : i
        let s_94_10: i128 = 4;
        // D s_94_11: read-var u#23114:u32
        let s_94_11: u32 = fn_state.u_23114;
        // D s_94_12: cast zx s_94_11 -> bv
        let s_94_12: Bits = Bits::new(s_94_11 as u128, 32u16);
        // D s_94_13: bit-extract s_94_12 s_94_9 s_94_10
        let s_94_13: Bits = (Bits::new(
            ((s_94_12) >> (s_94_9)).value(),
            u16::try_from(s_94_10).unwrap(),
        ));
        // D s_94_14: cast reint s_94_13 -> u8
        let s_94_14: u8 = (s_94_13.value() as u8);
        // D s_94_15: call decode_bti_aarch64_instrs_system_hints(s_94_8, s_94_14)
        let s_94_15: () = decode_bti_aarch64_instrs_system_hints(
            state,
            tracer,
            s_94_8,
            s_94_14,
        );
        // N s_94_16: return
        return;
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_95_0: read-var merge#var.1:struct
        let s_95_0: u32 = fn_state.merge_var._1;
        // D s_95_1: write-var u#23118 <= s_95_0
        fn_state.u_23118 = s_95_0;
        // D s_95_2: read-var u#23118:u32
        let s_95_2: u32 = fn_state.u_23118;
        // D s_95_3: cast zx s_95_2 -> bv
        let s_95_3: Bits = Bits::new(s_95_2 as u128, 32u16);
        // C s_95_4: const #3573753119u : u32
        let s_95_4: u32 = 3573753119;
        // C s_95_5: cast zx s_95_4 -> bv
        let s_95_5: Bits = Bits::new(s_95_4 as u128, 32u16);
        // D s_95_6: cmp-eq s_95_3 s_95_5
        let s_95_6: bool = ((s_95_3) == (s_95_5));
        // N s_95_7: branch s_95_6 b461 b96
        if s_95_6 {
            return block_461(state, tracer, fn_state);
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
        // D s_96_1: write-var gs#373880 <= s_96_0
        fn_state.gs_373880 = s_96_0;
        // N s_96_2: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_97_0: read-var gs#373880:u8
        let s_97_0: bool = fn_state.gs_373880;
        // D s_97_1: not s_97_0
        let s_97_1: bool = !s_97_0;
        // N s_97_2: branch s_97_1 b99 b98
        if s_97_1 {
            return block_99(state, tracer, fn_state);
        } else {
            return block_98(state, tracer, fn_state);
        };
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #98s : i
        let s_98_0: i128 = 98;
        // C s_98_1: const #14696u : u32
        let s_98_1: u32 = 14696;
        // N s_98_2: write-reg s_98_1 <= s_98_0
        let s_98_2: () = {
            state.write_register::<i128>(s_98_1 as isize, s_98_0);
            tracer.write_register(s_98_1 as isize, s_98_0);
        };
        // C s_98_3: const #5s : i
        let s_98_3: i128 = 5;
        // C s_98_4: const #3s : i
        let s_98_4: i128 = 3;
        // D s_98_5: read-var u#23118:u32
        let s_98_5: u32 = fn_state.u_23118;
        // D s_98_6: cast zx s_98_5 -> bv
        let s_98_6: Bits = Bits::new(s_98_5 as u128, 32u16);
        // D s_98_7: bit-extract s_98_6 s_98_3 s_98_4
        let s_98_7: Bits = (Bits::new(
            ((s_98_6) >> (s_98_3)).value(),
            u16::try_from(s_98_4).unwrap(),
        ));
        // D s_98_8: cast reint s_98_7 -> u8
        let s_98_8: u8 = (s_98_7.value() as u8);
        // C s_98_9: const #8s : i
        let s_98_9: i128 = 8;
        // C s_98_10: const #4s : i
        let s_98_10: i128 = 4;
        // D s_98_11: read-var u#23118:u32
        let s_98_11: u32 = fn_state.u_23118;
        // D s_98_12: cast zx s_98_11 -> bv
        let s_98_12: Bits = Bits::new(s_98_11 as u128, 32u16);
        // D s_98_13: bit-extract s_98_12 s_98_9 s_98_10
        let s_98_13: Bits = (Bits::new(
            ((s_98_12) >> (s_98_9)).value(),
            u16::try_from(s_98_10).unwrap(),
        ));
        // D s_98_14: cast reint s_98_13 -> u8
        let s_98_14: u8 = (s_98_13.value() as u8);
        // D s_98_15: call decode_chkfeat_aarch64_instrs_system_hints(s_98_8, s_98_14)
        let s_98_15: () = decode_chkfeat_aarch64_instrs_system_hints(
            state,
            tracer,
            s_98_8,
            s_98_14,
        );
        // N s_98_16: return
        return;
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_99_0: read-var merge#var.1:struct
        let s_99_0: u32 = fn_state.merge_var._1;
        // D s_99_1: write-var u#23122 <= s_99_0
        fn_state.u_23122 = s_99_0;
        // D s_99_2: read-var u#23122:u32
        let s_99_2: u32 = fn_state.u_23122;
        // D s_99_3: cast zx s_99_2 -> bv
        let s_99_3: Bits = Bits::new(s_99_2 as u128, 32u16);
        // C s_99_4: const #3573752543u : u32
        let s_99_4: u32 = 3573752543;
        // C s_99_5: cast zx s_99_4 -> bv
        let s_99_5: Bits = Bits::new(s_99_4 as u128, 32u16);
        // D s_99_6: cmp-eq s_99_3 s_99_5
        let s_99_6: bool = ((s_99_3) == (s_99_5));
        // N s_99_7: branch s_99_6 b460 b100
        if s_99_6 {
            return block_460(state, tracer, fn_state);
        } else {
            return block_100(state, tracer, fn_state);
        };
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #0u : u8
        let s_100_0: bool = false;
        // D s_100_1: write-var gs#373889 <= s_100_0
        fn_state.gs_373889 = s_100_0;
        // N s_100_2: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_101_0: read-var gs#373889:u8
        let s_101_0: bool = fn_state.gs_373889;
        // D s_101_1: not s_101_0
        let s_101_1: bool = !s_101_0;
        // N s_101_2: branch s_101_1 b103 b102
        if s_101_1 {
            return block_103(state, tracer, fn_state);
        } else {
            return block_102(state, tracer, fn_state);
        };
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_102_0: const #99s : i
        let s_102_0: i128 = 99;
        // C s_102_1: const #14696u : u32
        let s_102_1: u32 = 14696;
        // N s_102_2: write-reg s_102_1 <= s_102_0
        let s_102_2: () = {
            state.write_register::<i128>(s_102_1 as isize, s_102_0);
            tracer.write_register(s_102_1 as isize, s_102_0);
        };
        // C s_102_3: const #5s : i
        let s_102_3: i128 = 5;
        // C s_102_4: const #3s : i
        let s_102_4: i128 = 3;
        // D s_102_5: read-var u#23122:u32
        let s_102_5: u32 = fn_state.u_23122;
        // D s_102_6: cast zx s_102_5 -> bv
        let s_102_6: Bits = Bits::new(s_102_5 as u128, 32u16);
        // D s_102_7: bit-extract s_102_6 s_102_3 s_102_4
        let s_102_7: Bits = (Bits::new(
            ((s_102_6) >> (s_102_3)).value(),
            u16::try_from(s_102_4).unwrap(),
        ));
        // D s_102_8: cast reint s_102_7 -> u8
        let s_102_8: u8 = (s_102_7.value() as u8);
        // C s_102_9: const #8s : i
        let s_102_9: i128 = 8;
        // C s_102_10: const #4s : i
        let s_102_10: i128 = 4;
        // D s_102_11: read-var u#23122:u32
        let s_102_11: u32 = fn_state.u_23122;
        // D s_102_12: cast zx s_102_11 -> bv
        let s_102_12: Bits = Bits::new(s_102_11 as u128, 32u16);
        // D s_102_13: bit-extract s_102_12 s_102_9 s_102_10
        let s_102_13: Bits = (Bits::new(
            ((s_102_12) >> (s_102_9)).value(),
            u16::try_from(s_102_10).unwrap(),
        ));
        // D s_102_14: cast reint s_102_13 -> u8
        let s_102_14: u8 = (s_102_13.value() as u8);
        // D s_102_15: call decode_clrbhb_aarch64_instrs_system_hints(s_102_8, s_102_14)
        let s_102_15: () = decode_clrbhb_aarch64_instrs_system_hints(
            state,
            tracer,
            s_102_8,
            s_102_14,
        );
        // N s_102_16: return
        return;
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_103_0: read-var merge#var.1:struct
        let s_103_0: u32 = fn_state.merge_var._1;
        // D s_103_1: write-var u#23126 <= s_103_0
        fn_state.u_23126 = s_103_0;
        // D s_103_2: read-var u#23126:u32
        let s_103_2: u32 = fn_state.u_23126;
        // D s_103_3: cast zx s_103_2 -> bv
        let s_103_3: Bits = Bits::new(s_103_2 as u128, 32u16);
        // C s_103_4: const #3573752479u : u32
        let s_103_4: u32 = 3573752479;
        // C s_103_5: cast zx s_103_4 -> bv
        let s_103_5: Bits = Bits::new(s_103_4 as u128, 32u16);
        // D s_103_6: cmp-eq s_103_3 s_103_5
        let s_103_6: bool = ((s_103_3) == (s_103_5));
        // N s_103_7: branch s_103_6 b459 b104
        if s_103_6 {
            return block_459(state, tracer, fn_state);
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
        // D s_104_1: write-var gs#373898 <= s_104_0
        fn_state.gs_373898 = s_104_0;
        // N s_104_2: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_105_0: read-var gs#373898:u8
        let s_105_0: bool = fn_state.gs_373898;
        // D s_105_1: not s_105_0
        let s_105_1: bool = !s_105_0;
        // N s_105_2: branch s_105_1 b107 b106
        if s_105_1 {
            return block_107(state, tracer, fn_state);
        } else {
            return block_106(state, tracer, fn_state);
        };
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #100s : i
        let s_106_0: i128 = 100;
        // C s_106_1: const #14696u : u32
        let s_106_1: u32 = 14696;
        // N s_106_2: write-reg s_106_1 <= s_106_0
        let s_106_2: () = {
            state.write_register::<i128>(s_106_1 as isize, s_106_0);
            tracer.write_register(s_106_1 as isize, s_106_0);
        };
        // C s_106_3: const #5s : i
        let s_106_3: i128 = 5;
        // C s_106_4: const #3s : i
        let s_106_4: i128 = 3;
        // D s_106_5: read-var u#23126:u32
        let s_106_5: u32 = fn_state.u_23126;
        // D s_106_6: cast zx s_106_5 -> bv
        let s_106_6: Bits = Bits::new(s_106_5 as u128, 32u16);
        // D s_106_7: bit-extract s_106_6 s_106_3 s_106_4
        let s_106_7: Bits = (Bits::new(
            ((s_106_6) >> (s_106_3)).value(),
            u16::try_from(s_106_4).unwrap(),
        ));
        // D s_106_8: cast reint s_106_7 -> u8
        let s_106_8: u8 = (s_106_7.value() as u8);
        // C s_106_9: const #8s : i
        let s_106_9: i128 = 8;
        // C s_106_10: const #4s : i
        let s_106_10: i128 = 4;
        // D s_106_11: read-var u#23126:u32
        let s_106_11: u32 = fn_state.u_23126;
        // D s_106_12: cast zx s_106_11 -> bv
        let s_106_12: Bits = Bits::new(s_106_11 as u128, 32u16);
        // D s_106_13: bit-extract s_106_12 s_106_9 s_106_10
        let s_106_13: Bits = (Bits::new(
            ((s_106_12) >> (s_106_9)).value(),
            u16::try_from(s_106_10).unwrap(),
        ));
        // D s_106_14: cast reint s_106_13 -> u8
        let s_106_14: u8 = (s_106_13.value() as u8);
        // D s_106_15: call decode_csdb_aarch64_instrs_system_hints(s_106_8, s_106_14)
        let s_106_15: () = decode_csdb_aarch64_instrs_system_hints(
            state,
            tracer,
            s_106_8,
            s_106_14,
        );
        // N s_106_16: return
        return;
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_107_0: read-var merge#var.1:struct
        let s_107_0: u32 = fn_state.merge_var._1;
        // D s_107_1: write-var u#23130 <= s_107_0
        fn_state.u_23130 = s_107_0;
        // D s_107_2: read-var u#23130:u32
        let s_107_2: u32 = fn_state.u_23130;
        // D s_107_3: cast zx s_107_2 -> bv
        let s_107_3: Bits = Bits::new(s_107_2 as u128, 32u16);
        // C s_107_4: const #3573752031u : u32
        let s_107_4: u32 = 3573752031;
        // C s_107_5: cast zx s_107_4 -> bv
        let s_107_5: Bits = Bits::new(s_107_4 as u128, 32u16);
        // D s_107_6: cmp-eq s_107_3 s_107_5
        let s_107_6: bool = ((s_107_3) == (s_107_5));
        // N s_107_7: branch s_107_6 b458 b108
        if s_107_6 {
            return block_458(state, tracer, fn_state);
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
        // D s_108_1: write-var gs#373907 <= s_108_0
        fn_state.gs_373907 = s_108_0;
        // N s_108_2: jump b109
        return block_109(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_109_0: read-var gs#373907:u8
        let s_109_0: bool = fn_state.gs_373907;
        // D s_109_1: not s_109_0
        let s_109_1: bool = !s_109_0;
        // N s_109_2: branch s_109_1 b111 b110
        if s_109_1 {
            return block_111(state, tracer, fn_state);
        } else {
            return block_110(state, tracer, fn_state);
        };
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_110_0: const #101s : i
        let s_110_0: i128 = 101;
        // C s_110_1: const #14696u : u32
        let s_110_1: u32 = 14696;
        // N s_110_2: write-reg s_110_1 <= s_110_0
        let s_110_2: () = {
            state.write_register::<i128>(s_110_1 as isize, s_110_0);
            tracer.write_register(s_110_1 as isize, s_110_0);
        };
        // C s_110_3: const #5s : i
        let s_110_3: i128 = 5;
        // C s_110_4: const #3s : i
        let s_110_4: i128 = 3;
        // D s_110_5: read-var u#23130:u32
        let s_110_5: u32 = fn_state.u_23130;
        // D s_110_6: cast zx s_110_5 -> bv
        let s_110_6: Bits = Bits::new(s_110_5 as u128, 32u16);
        // D s_110_7: bit-extract s_110_6 s_110_3 s_110_4
        let s_110_7: Bits = (Bits::new(
            ((s_110_6) >> (s_110_3)).value(),
            u16::try_from(s_110_4).unwrap(),
        ));
        // D s_110_8: cast reint s_110_7 -> u8
        let s_110_8: u8 = (s_110_7.value() as u8);
        // C s_110_9: const #8s : i
        let s_110_9: i128 = 8;
        // C s_110_10: const #4s : i
        let s_110_10: i128 = 4;
        // D s_110_11: read-var u#23130:u32
        let s_110_11: u32 = fn_state.u_23130;
        // D s_110_12: cast zx s_110_11 -> bv
        let s_110_12: Bits = Bits::new(s_110_11 as u128, 32u16);
        // D s_110_13: bit-extract s_110_12 s_110_9 s_110_10
        let s_110_13: Bits = (Bits::new(
            ((s_110_12) >> (s_110_9)).value(),
            u16::try_from(s_110_10).unwrap(),
        ));
        // D s_110_14: cast reint s_110_13 -> u8
        let s_110_14: u8 = (s_110_13.value() as u8);
        // D s_110_15: call decode_dgh_aarch64_instrs_system_hints(s_110_8, s_110_14)
        let s_110_15: () = decode_dgh_aarch64_instrs_system_hints(
            state,
            tracer,
            s_110_8,
            s_110_14,
        );
        // N s_110_16: return
        return;
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_111_0: read-var merge#var.1:struct
        let s_111_0: u32 = fn_state.merge_var._1;
        // D s_111_1: write-var u#23134 <= s_111_0
        fn_state.u_23134 = s_111_0;
        // D s_111_2: read-var u#23134:u32
        let s_111_2: u32 = fn_state.u_23134;
        // D s_111_3: cast zx s_111_2 -> bv
        let s_111_3: Bits = Bits::new(s_111_2 as u128, 32u16);
        // C s_111_4: const #3573752351u : u32
        let s_111_4: u32 = 3573752351;
        // C s_111_5: cast zx s_111_4 -> bv
        let s_111_5: Bits = Bits::new(s_111_4 as u128, 32u16);
        // D s_111_6: cmp-eq s_111_3 s_111_5
        let s_111_6: bool = ((s_111_3) == (s_111_5));
        // N s_111_7: branch s_111_6 b457 b112
        if s_111_6 {
            return block_457(state, tracer, fn_state);
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
        // D s_112_1: write-var gs#373916 <= s_112_0
        fn_state.gs_373916 = s_112_0;
        // N s_112_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_113_0: read-var gs#373916:u8
        let s_113_0: bool = fn_state.gs_373916;
        // D s_113_1: not s_113_0
        let s_113_1: bool = !s_113_0;
        // N s_113_2: branch s_113_1 b115 b114
        if s_113_1 {
            return block_115(state, tracer, fn_state);
        } else {
            return block_114(state, tracer, fn_state);
        };
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #102s : i
        let s_114_0: i128 = 102;
        // C s_114_1: const #14696u : u32
        let s_114_1: u32 = 14696;
        // N s_114_2: write-reg s_114_1 <= s_114_0
        let s_114_2: () = {
            state.write_register::<i128>(s_114_1 as isize, s_114_0);
            tracer.write_register(s_114_1 as isize, s_114_0);
        };
        // C s_114_3: const #5s : i
        let s_114_3: i128 = 5;
        // C s_114_4: const #3s : i
        let s_114_4: i128 = 3;
        // D s_114_5: read-var u#23134:u32
        let s_114_5: u32 = fn_state.u_23134;
        // D s_114_6: cast zx s_114_5 -> bv
        let s_114_6: Bits = Bits::new(s_114_5 as u128, 32u16);
        // D s_114_7: bit-extract s_114_6 s_114_3 s_114_4
        let s_114_7: Bits = (Bits::new(
            ((s_114_6) >> (s_114_3)).value(),
            u16::try_from(s_114_4).unwrap(),
        ));
        // D s_114_8: cast reint s_114_7 -> u8
        let s_114_8: u8 = (s_114_7.value() as u8);
        // C s_114_9: const #8s : i
        let s_114_9: i128 = 8;
        // C s_114_10: const #4s : i
        let s_114_10: i128 = 4;
        // D s_114_11: read-var u#23134:u32
        let s_114_11: u32 = fn_state.u_23134;
        // D s_114_12: cast zx s_114_11 -> bv
        let s_114_12: Bits = Bits::new(s_114_11 as u128, 32u16);
        // D s_114_13: bit-extract s_114_12 s_114_9 s_114_10
        let s_114_13: Bits = (Bits::new(
            ((s_114_12) >> (s_114_9)).value(),
            u16::try_from(s_114_10).unwrap(),
        ));
        // D s_114_14: cast reint s_114_13 -> u8
        let s_114_14: u8 = (s_114_13.value() as u8);
        // D s_114_15: call decode_esb_aarch64_instrs_system_hints(s_114_8, s_114_14)
        let s_114_15: () = decode_esb_aarch64_instrs_system_hints(
            state,
            tracer,
            s_114_8,
            s_114_14,
        );
        // N s_114_16: return
        return;
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_115_0: read-var merge#var.1:struct
        let s_115_0: u32 = fn_state.merge_var._1;
        // D s_115_1: write-var u#23138 <= s_115_0
        fn_state.u_23138 = s_115_0;
        // D s_115_2: read-var u#23138:u32
        let s_115_2: u32 = fn_state.u_23138;
        // D s_115_3: cast zx s_115_2 -> bv
        let s_115_3: Bits = Bits::new(s_115_2 as u128, 32u16);
        // C s_115_4: const #3573752447u : u32
        let s_115_4: u32 = 3573752447;
        // C s_115_5: cast zx s_115_4 -> bv
        let s_115_5: Bits = Bits::new(s_115_4 as u128, 32u16);
        // D s_115_6: cmp-eq s_115_3 s_115_5
        let s_115_6: bool = ((s_115_3) == (s_115_5));
        // N s_115_7: branch s_115_6 b456 b116
        if s_115_6 {
            return block_456(state, tracer, fn_state);
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
        // D s_116_1: write-var gs#373925 <= s_116_0
        fn_state.gs_373925 = s_116_0;
        // N s_116_2: jump b117
        return block_117(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_117_0: read-var gs#373925:u8
        let s_117_0: bool = fn_state.gs_373925;
        // D s_117_1: not s_117_0
        let s_117_1: bool = !s_117_0;
        // N s_117_2: branch s_117_1 b119 b118
        if s_117_1 {
            return block_119(state, tracer, fn_state);
        } else {
            return block_118(state, tracer, fn_state);
        };
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #103s : i
        let s_118_0: i128 = 103;
        // C s_118_1: const #14696u : u32
        let s_118_1: u32 = 14696;
        // N s_118_2: write-reg s_118_1 <= s_118_0
        let s_118_2: () = {
            state.write_register::<i128>(s_118_1 as isize, s_118_0);
            tracer.write_register(s_118_1 as isize, s_118_0);
        };
        // C s_118_3: const #5s : i
        let s_118_3: i128 = 5;
        // C s_118_4: const #3s : i
        let s_118_4: i128 = 3;
        // D s_118_5: read-var u#23138:u32
        let s_118_5: u32 = fn_state.u_23138;
        // D s_118_6: cast zx s_118_5 -> bv
        let s_118_6: Bits = Bits::new(s_118_5 as u128, 32u16);
        // D s_118_7: bit-extract s_118_6 s_118_3 s_118_4
        let s_118_7: Bits = (Bits::new(
            ((s_118_6) >> (s_118_3)).value(),
            u16::try_from(s_118_4).unwrap(),
        ));
        // D s_118_8: cast reint s_118_7 -> u8
        let s_118_8: u8 = (s_118_7.value() as u8);
        // C s_118_9: const #8s : i
        let s_118_9: i128 = 8;
        // C s_118_10: const #4s : i
        let s_118_10: i128 = 4;
        // D s_118_11: read-var u#23138:u32
        let s_118_11: u32 = fn_state.u_23138;
        // D s_118_12: cast zx s_118_11 -> bv
        let s_118_12: Bits = Bits::new(s_118_11 as u128, 32u16);
        // D s_118_13: bit-extract s_118_12 s_118_9 s_118_10
        let s_118_13: Bits = (Bits::new(
            ((s_118_12) >> (s_118_9)).value(),
            u16::try_from(s_118_10).unwrap(),
        ));
        // D s_118_14: cast reint s_118_13 -> u8
        let s_118_14: u8 = (s_118_13.value() as u8);
        // D s_118_15: call decode_gcsb_aarch64_instrs_system_hints(s_118_8, s_118_14)
        let s_118_15: () = decode_gcsb_aarch64_instrs_system_hints(
            state,
            tracer,
            s_118_8,
            s_118_14,
        );
        // N s_118_16: return
        return;
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_119_0: read-var merge#var.1:struct
        let s_119_0: u32 = fn_state.merge_var._1;
        // D s_119_1: write-var u#23142 <= s_119_0
        fn_state.u_23142 = s_119_0;
        // C s_119_2: const #12s : i
        let s_119_2: i128 = 12;
        // D s_119_3: read-var u#23142:u32
        let s_119_3: u32 = fn_state.u_23142;
        // D s_119_4: cast zx s_119_3 -> bv
        let s_119_4: Bits = Bits::new(s_119_3 as u128, 32u16);
        // C s_119_5: const #1s : i64
        let s_119_5: i64 = 1;
        // C s_119_6: cast zx s_119_5 -> i
        let s_119_6: i128 = (i128::try_from(s_119_5).unwrap());
        // C s_119_7: const #19s : i
        let s_119_7: i128 = 19;
        // C s_119_8: add s_119_7 s_119_6
        let s_119_8: i128 = (s_119_7 + s_119_6);
        // D s_119_9: bit-extract s_119_4 s_119_2 s_119_8
        let s_119_9: Bits = (Bits::new(
            ((s_119_4) >> (s_119_2)).value(),
            u16::try_from(s_119_8).unwrap(),
        ));
        // D s_119_10: cast reint s_119_9 -> u20
        let s_119_10: u32 = (s_119_9.value() as u32);
        // D s_119_11: cast zx s_119_10 -> bv
        let s_119_11: Bits = Bits::new(s_119_10 as u128, 20u16);
        // C s_119_12: const #872498u : u20
        let s_119_12: u32 = 872498;
        // C s_119_13: cast zx s_119_12 -> bv
        let s_119_13: Bits = Bits::new(s_119_12 as u128, 20u16);
        // D s_119_14: cmp-eq s_119_11 s_119_13
        let s_119_14: bool = ((s_119_11) == (s_119_13));
        // N s_119_15: branch s_119_14 b455 b120
        if s_119_14 {
            return block_455(state, tracer, fn_state);
        } else {
            return block_120(state, tracer, fn_state);
        };
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_120_0: const #0u : u8
        let s_120_0: bool = false;
        // D s_120_1: write-var gs#373937 <= s_120_0
        fn_state.gs_373937 = s_120_0;
        // N s_120_2: jump b121
        return block_121(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_121_0: read-var gs#373937:u8
        let s_121_0: bool = fn_state.gs_373937;
        // N s_121_1: branch s_121_0 b454 b122
        if s_121_0 {
            return block_454(state, tracer, fn_state);
        } else {
            return block_122(state, tracer, fn_state);
        };
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_122_0: const #0u : u8
        let s_122_0: bool = false;
        // D s_122_1: write-var gs#373939 <= s_122_0
        fn_state.gs_373939 = s_122_0;
        // N s_122_2: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_123_0: read-var gs#373939:u8
        let s_123_0: bool = fn_state.gs_373939;
        // D s_123_1: not s_123_0
        let s_123_1: bool = !s_123_0;
        // N s_123_2: branch s_123_1 b125 b124
        if s_123_1 {
            return block_125(state, tracer, fn_state);
        } else {
            return block_124(state, tracer, fn_state);
        };
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_124_0: const #104s : i
        let s_124_0: i128 = 104;
        // C s_124_1: const #14696u : u32
        let s_124_1: u32 = 14696;
        // N s_124_2: write-reg s_124_1 <= s_124_0
        let s_124_2: () = {
            state.write_register::<i128>(s_124_1 as isize, s_124_0);
            tracer.write_register(s_124_1 as isize, s_124_0);
        };
        // C s_124_3: const #5s : i
        let s_124_3: i128 = 5;
        // C s_124_4: const #3s : i
        let s_124_4: i128 = 3;
        // D s_124_5: read-var u#23142:u32
        let s_124_5: u32 = fn_state.u_23142;
        // D s_124_6: cast zx s_124_5 -> bv
        let s_124_6: Bits = Bits::new(s_124_5 as u128, 32u16);
        // D s_124_7: bit-extract s_124_6 s_124_3 s_124_4
        let s_124_7: Bits = (Bits::new(
            ((s_124_6) >> (s_124_3)).value(),
            u16::try_from(s_124_4).unwrap(),
        ));
        // D s_124_8: cast reint s_124_7 -> u8
        let s_124_8: u8 = (s_124_7.value() as u8);
        // C s_124_9: const #8s : i
        let s_124_9: i128 = 8;
        // C s_124_10: const #4s : i
        let s_124_10: i128 = 4;
        // D s_124_11: read-var u#23142:u32
        let s_124_11: u32 = fn_state.u_23142;
        // D s_124_12: cast zx s_124_11 -> bv
        let s_124_12: Bits = Bits::new(s_124_11 as u128, 32u16);
        // D s_124_13: bit-extract s_124_12 s_124_9 s_124_10
        let s_124_13: Bits = (Bits::new(
            ((s_124_12) >> (s_124_9)).value(),
            u16::try_from(s_124_10).unwrap(),
        ));
        // D s_124_14: cast reint s_124_13 -> u8
        let s_124_14: u8 = (s_124_13.value() as u8);
        // D s_124_15: call decode_hint_aarch64_instrs_system_hints(s_124_8, s_124_14)
        let s_124_15: () = decode_hint_aarch64_instrs_system_hints(
            state,
            tracer,
            s_124_8,
            s_124_14,
        );
        // N s_124_16: return
        return;
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_125_0: read-var merge#var.1:struct
        let s_125_0: u32 = fn_state.merge_var._1;
        // D s_125_1: write-var u#23146 <= s_125_0
        fn_state.u_23146 = s_125_0;
        // D s_125_2: read-var u#23146:u32
        let s_125_2: u32 = fn_state.u_23146;
        // D s_125_3: cast zx s_125_2 -> bv
        let s_125_3: Bits = Bits::new(s_125_2 as u128, 32u16);
        // C s_125_4: const #3573751839u : u32
        let s_125_4: u32 = 3573751839;
        // C s_125_5: cast zx s_125_4 -> bv
        let s_125_5: Bits = Bits::new(s_125_4 as u128, 32u16);
        // D s_125_6: cmp-eq s_125_3 s_125_5
        let s_125_6: bool = ((s_125_3) == (s_125_5));
        // N s_125_7: branch s_125_6 b453 b126
        if s_125_6 {
            return block_453(state, tracer, fn_state);
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
        // D s_126_1: write-var gs#373948 <= s_126_0
        fn_state.gs_373948 = s_126_0;
        // N s_126_2: jump b127
        return block_127(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_127_0: read-var gs#373948:u8
        let s_127_0: bool = fn_state.gs_373948;
        // D s_127_1: not s_127_0
        let s_127_1: bool = !s_127_0;
        // N s_127_2: branch s_127_1 b129 b128
        if s_127_1 {
            return block_129(state, tracer, fn_state);
        } else {
            return block_128(state, tracer, fn_state);
        };
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_128_0: const #105s : i
        let s_128_0: i128 = 105;
        // C s_128_1: const #14696u : u32
        let s_128_1: u32 = 14696;
        // N s_128_2: write-reg s_128_1 <= s_128_0
        let s_128_2: () = {
            state.write_register::<i128>(s_128_1 as isize, s_128_0);
            tracer.write_register(s_128_1 as isize, s_128_0);
        };
        // C s_128_3: const #5s : i
        let s_128_3: i128 = 5;
        // C s_128_4: const #3s : i
        let s_128_4: i128 = 3;
        // D s_128_5: read-var u#23146:u32
        let s_128_5: u32 = fn_state.u_23146;
        // D s_128_6: cast zx s_128_5 -> bv
        let s_128_6: Bits = Bits::new(s_128_5 as u128, 32u16);
        // D s_128_7: bit-extract s_128_6 s_128_3 s_128_4
        let s_128_7: Bits = (Bits::new(
            ((s_128_6) >> (s_128_3)).value(),
            u16::try_from(s_128_4).unwrap(),
        ));
        // D s_128_8: cast reint s_128_7 -> u8
        let s_128_8: u8 = (s_128_7.value() as u8);
        // C s_128_9: const #8s : i
        let s_128_9: i128 = 8;
        // C s_128_10: const #4s : i
        let s_128_10: i128 = 4;
        // D s_128_11: read-var u#23146:u32
        let s_128_11: u32 = fn_state.u_23146;
        // D s_128_12: cast zx s_128_11 -> bv
        let s_128_12: Bits = Bits::new(s_128_11 as u128, 32u16);
        // D s_128_13: bit-extract s_128_12 s_128_9 s_128_10
        let s_128_13: Bits = (Bits::new(
            ((s_128_12) >> (s_128_9)).value(),
            u16::try_from(s_128_10).unwrap(),
        ));
        // D s_128_14: cast reint s_128_13 -> u8
        let s_128_14: u8 = (s_128_13.value() as u8);
        // D s_128_15: call decode_nop_aarch64_instrs_system_hints(s_128_8, s_128_14)
        let s_128_15: () = decode_nop_aarch64_instrs_system_hints(
            state,
            tracer,
            s_128_8,
            s_128_14,
        );
        // N s_128_16: return
        return;
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_129_0: read-var merge#var.1:struct
        let s_129_0: u32 = fn_state.merge_var._1;
        // D s_129_1: write-var u#23150 <= s_129_0
        fn_state.u_23150 = s_129_0;
        // D s_129_2: read-var u#23150:u32
        let s_129_2: u32 = fn_state.u_23150;
        // D s_129_3: cast zx s_129_2 -> bv
        let s_129_3: Bits = Bits::new(s_129_2 as u128, 32u16);
        // C s_129_4: const #3573752383u : u32
        let s_129_4: u32 = 3573752383;
        // C s_129_5: cast zx s_129_4 -> bv
        let s_129_5: Bits = Bits::new(s_129_4 as u128, 32u16);
        // D s_129_6: cmp-eq s_129_3 s_129_5
        let s_129_6: bool = ((s_129_3) == (s_129_5));
        // N s_129_7: branch s_129_6 b452 b130
        if s_129_6 {
            return block_452(state, tracer, fn_state);
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
        // D s_130_1: write-var gs#373957 <= s_130_0
        fn_state.gs_373957 = s_130_0;
        // N s_130_2: jump b131
        return block_131(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_131_0: read-var gs#373957:u8
        let s_131_0: bool = fn_state.gs_373957;
        // D s_131_1: not s_131_0
        let s_131_1: bool = !s_131_0;
        // N s_131_2: branch s_131_1 b133 b132
        if s_131_1 {
            return block_133(state, tracer, fn_state);
        } else {
            return block_132(state, tracer, fn_state);
        };
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_132_0: const #106s : i
        let s_132_0: i128 = 106;
        // C s_132_1: const #14696u : u32
        let s_132_1: u32 = 14696;
        // N s_132_2: write-reg s_132_1 <= s_132_0
        let s_132_2: () = {
            state.write_register::<i128>(s_132_1 as isize, s_132_0);
            tracer.write_register(s_132_1 as isize, s_132_0);
        };
        // C s_132_3: const #5s : i
        let s_132_3: i128 = 5;
        // C s_132_4: const #3s : i
        let s_132_4: i128 = 3;
        // D s_132_5: read-var u#23150:u32
        let s_132_5: u32 = fn_state.u_23150;
        // D s_132_6: cast zx s_132_5 -> bv
        let s_132_6: Bits = Bits::new(s_132_5 as u128, 32u16);
        // D s_132_7: bit-extract s_132_6 s_132_3 s_132_4
        let s_132_7: Bits = (Bits::new(
            ((s_132_6) >> (s_132_3)).value(),
            u16::try_from(s_132_4).unwrap(),
        ));
        // D s_132_8: cast reint s_132_7 -> u8
        let s_132_8: u8 = (s_132_7.value() as u8);
        // C s_132_9: const #8s : i
        let s_132_9: i128 = 8;
        // C s_132_10: const #4s : i
        let s_132_10: i128 = 4;
        // D s_132_11: read-var u#23150:u32
        let s_132_11: u32 = fn_state.u_23150;
        // D s_132_12: cast zx s_132_11 -> bv
        let s_132_12: Bits = Bits::new(s_132_11 as u128, 32u16);
        // D s_132_13: bit-extract s_132_12 s_132_9 s_132_10
        let s_132_13: Bits = (Bits::new(
            ((s_132_12) >> (s_132_9)).value(),
            u16::try_from(s_132_10).unwrap(),
        ));
        // D s_132_14: cast reint s_132_13 -> u8
        let s_132_14: u8 = (s_132_13.value() as u8);
        // D s_132_15: call decode_psb_aarch64_instrs_system_hints(s_132_8, s_132_14)
        let s_132_15: () = decode_psb_aarch64_instrs_system_hints(
            state,
            tracer,
            s_132_8,
            s_132_14,
        );
        // N s_132_16: return
        return;
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_133_0: read-var merge#var.1:struct
        let s_133_0: u32 = fn_state.merge_var._1;
        // D s_133_1: write-var u#23154 <= s_133_0
        fn_state.u_23154 = s_133_0;
        // D s_133_2: read-var u#23154:u32
        let s_133_2: u32 = fn_state.u_23154;
        // D s_133_3: cast zx s_133_2 -> bv
        let s_133_3: Bits = Bits::new(s_133_2 as u128, 32u16);
        // C s_133_4: const #3573751967u : u32
        let s_133_4: u32 = 3573751967;
        // C s_133_5: cast zx s_133_4 -> bv
        let s_133_5: Bits = Bits::new(s_133_4 as u128, 32u16);
        // D s_133_6: cmp-eq s_133_3 s_133_5
        let s_133_6: bool = ((s_133_3) == (s_133_5));
        // N s_133_7: branch s_133_6 b451 b134
        if s_133_6 {
            return block_451(state, tracer, fn_state);
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
        // D s_134_1: write-var gs#373966 <= s_134_0
        fn_state.gs_373966 = s_134_0;
        // N s_134_2: jump b135
        return block_135(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_135_0: read-var gs#373966:u8
        let s_135_0: bool = fn_state.gs_373966;
        // D s_135_1: not s_135_0
        let s_135_1: bool = !s_135_0;
        // N s_135_2: branch s_135_1 b137 b136
        if s_135_1 {
            return block_137(state, tracer, fn_state);
        } else {
            return block_136(state, tracer, fn_state);
        };
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_136_0: const #107s : i
        let s_136_0: i128 = 107;
        // C s_136_1: const #14696u : u32
        let s_136_1: u32 = 14696;
        // N s_136_2: write-reg s_136_1 <= s_136_0
        let s_136_2: () = {
            state.write_register::<i128>(s_136_1 as isize, s_136_0);
            tracer.write_register(s_136_1 as isize, s_136_0);
        };
        // C s_136_3: const #5s : i
        let s_136_3: i128 = 5;
        // C s_136_4: const #3s : i
        let s_136_4: i128 = 3;
        // D s_136_5: read-var u#23154:u32
        let s_136_5: u32 = fn_state.u_23154;
        // D s_136_6: cast zx s_136_5 -> bv
        let s_136_6: Bits = Bits::new(s_136_5 as u128, 32u16);
        // D s_136_7: bit-extract s_136_6 s_136_3 s_136_4
        let s_136_7: Bits = (Bits::new(
            ((s_136_6) >> (s_136_3)).value(),
            u16::try_from(s_136_4).unwrap(),
        ));
        // D s_136_8: cast reint s_136_7 -> u8
        let s_136_8: u8 = (s_136_7.value() as u8);
        // C s_136_9: const #8s : i
        let s_136_9: i128 = 8;
        // C s_136_10: const #4s : i
        let s_136_10: i128 = 4;
        // D s_136_11: read-var u#23154:u32
        let s_136_11: u32 = fn_state.u_23154;
        // D s_136_12: cast zx s_136_11 -> bv
        let s_136_12: Bits = Bits::new(s_136_11 as u128, 32u16);
        // D s_136_13: bit-extract s_136_12 s_136_9 s_136_10
        let s_136_13: Bits = (Bits::new(
            ((s_136_12) >> (s_136_9)).value(),
            u16::try_from(s_136_10).unwrap(),
        ));
        // D s_136_14: cast reint s_136_13 -> u8
        let s_136_14: u8 = (s_136_13.value() as u8);
        // D s_136_15: call decode_sev_aarch64_instrs_system_hints(s_136_8, s_136_14)
        let s_136_15: () = decode_sev_aarch64_instrs_system_hints(
            state,
            tracer,
            s_136_8,
            s_136_14,
        );
        // N s_136_16: return
        return;
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_137_0: read-var merge#var.1:struct
        let s_137_0: u32 = fn_state.merge_var._1;
        // D s_137_1: write-var u#23158 <= s_137_0
        fn_state.u_23158 = s_137_0;
        // D s_137_2: read-var u#23158:u32
        let s_137_2: u32 = fn_state.u_23158;
        // D s_137_3: cast zx s_137_2 -> bv
        let s_137_3: Bits = Bits::new(s_137_2 as u128, 32u16);
        // C s_137_4: const #3573751999u : u32
        let s_137_4: u32 = 3573751999;
        // C s_137_5: cast zx s_137_4 -> bv
        let s_137_5: Bits = Bits::new(s_137_4 as u128, 32u16);
        // D s_137_6: cmp-eq s_137_3 s_137_5
        let s_137_6: bool = ((s_137_3) == (s_137_5));
        // N s_137_7: branch s_137_6 b450 b138
        if s_137_6 {
            return block_450(state, tracer, fn_state);
        } else {
            return block_138(state, tracer, fn_state);
        };
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_138_0: const #0u : u8
        let s_138_0: bool = false;
        // D s_138_1: write-var gs#373975 <= s_138_0
        fn_state.gs_373975 = s_138_0;
        // N s_138_2: jump b139
        return block_139(state, tracer, fn_state);
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_139_0: read-var gs#373975:u8
        let s_139_0: bool = fn_state.gs_373975;
        // D s_139_1: not s_139_0
        let s_139_1: bool = !s_139_0;
        // N s_139_2: branch s_139_1 b141 b140
        if s_139_1 {
            return block_141(state, tracer, fn_state);
        } else {
            return block_140(state, tracer, fn_state);
        };
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_140_0: const #108s : i
        let s_140_0: i128 = 108;
        // C s_140_1: const #14696u : u32
        let s_140_1: u32 = 14696;
        // N s_140_2: write-reg s_140_1 <= s_140_0
        let s_140_2: () = {
            state.write_register::<i128>(s_140_1 as isize, s_140_0);
            tracer.write_register(s_140_1 as isize, s_140_0);
        };
        // C s_140_3: const #5s : i
        let s_140_3: i128 = 5;
        // C s_140_4: const #3s : i
        let s_140_4: i128 = 3;
        // D s_140_5: read-var u#23158:u32
        let s_140_5: u32 = fn_state.u_23158;
        // D s_140_6: cast zx s_140_5 -> bv
        let s_140_6: Bits = Bits::new(s_140_5 as u128, 32u16);
        // D s_140_7: bit-extract s_140_6 s_140_3 s_140_4
        let s_140_7: Bits = (Bits::new(
            ((s_140_6) >> (s_140_3)).value(),
            u16::try_from(s_140_4).unwrap(),
        ));
        // D s_140_8: cast reint s_140_7 -> u8
        let s_140_8: u8 = (s_140_7.value() as u8);
        // C s_140_9: const #8s : i
        let s_140_9: i128 = 8;
        // C s_140_10: const #4s : i
        let s_140_10: i128 = 4;
        // D s_140_11: read-var u#23158:u32
        let s_140_11: u32 = fn_state.u_23158;
        // D s_140_12: cast zx s_140_11 -> bv
        let s_140_12: Bits = Bits::new(s_140_11 as u128, 32u16);
        // D s_140_13: bit-extract s_140_12 s_140_9 s_140_10
        let s_140_13: Bits = (Bits::new(
            ((s_140_12) >> (s_140_9)).value(),
            u16::try_from(s_140_10).unwrap(),
        ));
        // D s_140_14: cast reint s_140_13 -> u8
        let s_140_14: u8 = (s_140_13.value() as u8);
        // D s_140_15: call decode_sevl_aarch64_instrs_system_hints(s_140_8, s_140_14)
        let s_140_15: () = decode_sevl_aarch64_instrs_system_hints(
            state,
            tracer,
            s_140_8,
            s_140_14,
        );
        // N s_140_16: return
        return;
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_141_0: read-var merge#var.1:struct
        let s_141_0: u32 = fn_state.merge_var._1;
        // D s_141_1: write-var u#23162 <= s_141_0
        fn_state.u_23162 = s_141_0;
        // D s_141_2: read-var u#23162:u32
        let s_141_2: u32 = fn_state.u_23162;
        // D s_141_3: cast zx s_141_2 -> bv
        let s_141_3: Bits = Bits::new(s_141_2 as u128, 32u16);
        // C s_141_4: const #3573752415u : u32
        let s_141_4: u32 = 3573752415;
        // C s_141_5: cast zx s_141_4 -> bv
        let s_141_5: Bits = Bits::new(s_141_4 as u128, 32u16);
        // D s_141_6: cmp-eq s_141_3 s_141_5
        let s_141_6: bool = ((s_141_3) == (s_141_5));
        // N s_141_7: branch s_141_6 b449 b142
        if s_141_6 {
            return block_449(state, tracer, fn_state);
        } else {
            return block_142(state, tracer, fn_state);
        };
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_142_0: const #0u : u8
        let s_142_0: bool = false;
        // D s_142_1: write-var gs#373984 <= s_142_0
        fn_state.gs_373984 = s_142_0;
        // N s_142_2: jump b143
        return block_143(state, tracer, fn_state);
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_143_0: read-var gs#373984:u8
        let s_143_0: bool = fn_state.gs_373984;
        // D s_143_1: not s_143_0
        let s_143_1: bool = !s_143_0;
        // N s_143_2: branch s_143_1 b145 b144
        if s_143_1 {
            return block_145(state, tracer, fn_state);
        } else {
            return block_144(state, tracer, fn_state);
        };
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_144_0: const #109s : i
        let s_144_0: i128 = 109;
        // C s_144_1: const #14696u : u32
        let s_144_1: u32 = 14696;
        // N s_144_2: write-reg s_144_1 <= s_144_0
        let s_144_2: () = {
            state.write_register::<i128>(s_144_1 as isize, s_144_0);
            tracer.write_register(s_144_1 as isize, s_144_0);
        };
        // C s_144_3: const #5s : i
        let s_144_3: i128 = 5;
        // C s_144_4: const #3s : i
        let s_144_4: i128 = 3;
        // D s_144_5: read-var u#23162:u32
        let s_144_5: u32 = fn_state.u_23162;
        // D s_144_6: cast zx s_144_5 -> bv
        let s_144_6: Bits = Bits::new(s_144_5 as u128, 32u16);
        // D s_144_7: bit-extract s_144_6 s_144_3 s_144_4
        let s_144_7: Bits = (Bits::new(
            ((s_144_6) >> (s_144_3)).value(),
            u16::try_from(s_144_4).unwrap(),
        ));
        // D s_144_8: cast reint s_144_7 -> u8
        let s_144_8: u8 = (s_144_7.value() as u8);
        // C s_144_9: const #8s : i
        let s_144_9: i128 = 8;
        // C s_144_10: const #4s : i
        let s_144_10: i128 = 4;
        // D s_144_11: read-var u#23162:u32
        let s_144_11: u32 = fn_state.u_23162;
        // D s_144_12: cast zx s_144_11 -> bv
        let s_144_12: Bits = Bits::new(s_144_11 as u128, 32u16);
        // D s_144_13: bit-extract s_144_12 s_144_9 s_144_10
        let s_144_13: Bits = (Bits::new(
            ((s_144_12) >> (s_144_9)).value(),
            u16::try_from(s_144_10).unwrap(),
        ));
        // D s_144_14: cast reint s_144_13 -> u8
        let s_144_14: u8 = (s_144_13.value() as u8);
        // D s_144_15: call decode_tsb_aarch64_instrs_system_hints(s_144_8, s_144_14)
        let s_144_15: () = decode_tsb_aarch64_instrs_system_hints(
            state,
            tracer,
            s_144_8,
            s_144_14,
        );
        // N s_144_16: return
        return;
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_145_0: read-var merge#var.1:struct
        let s_145_0: u32 = fn_state.merge_var._1;
        // D s_145_1: write-var u#23166 <= s_145_0
        fn_state.u_23166 = s_145_0;
        // D s_145_2: read-var u#23166:u32
        let s_145_2: u32 = fn_state.u_23166;
        // D s_145_3: cast zx s_145_2 -> bv
        let s_145_3: Bits = Bits::new(s_145_2 as u128, 32u16);
        // C s_145_4: const #3573751903u : u32
        let s_145_4: u32 = 3573751903;
        // C s_145_5: cast zx s_145_4 -> bv
        let s_145_5: Bits = Bits::new(s_145_4 as u128, 32u16);
        // D s_145_6: cmp-eq s_145_3 s_145_5
        let s_145_6: bool = ((s_145_3) == (s_145_5));
        // N s_145_7: branch s_145_6 b448 b146
        if s_145_6 {
            return block_448(state, tracer, fn_state);
        } else {
            return block_146(state, tracer, fn_state);
        };
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_146_0: const #0u : u8
        let s_146_0: bool = false;
        // D s_146_1: write-var gs#373993 <= s_146_0
        fn_state.gs_373993 = s_146_0;
        // N s_146_2: jump b147
        return block_147(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_147_0: read-var gs#373993:u8
        let s_147_0: bool = fn_state.gs_373993;
        // D s_147_1: not s_147_0
        let s_147_1: bool = !s_147_0;
        // N s_147_2: branch s_147_1 b149 b148
        if s_147_1 {
            return block_149(state, tracer, fn_state);
        } else {
            return block_148(state, tracer, fn_state);
        };
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_148_0: const #110s : i
        let s_148_0: i128 = 110;
        // C s_148_1: const #14696u : u32
        let s_148_1: u32 = 14696;
        // N s_148_2: write-reg s_148_1 <= s_148_0
        let s_148_2: () = {
            state.write_register::<i128>(s_148_1 as isize, s_148_0);
            tracer.write_register(s_148_1 as isize, s_148_0);
        };
        // C s_148_3: const #5s : i
        let s_148_3: i128 = 5;
        // C s_148_4: const #3s : i
        let s_148_4: i128 = 3;
        // D s_148_5: read-var u#23166:u32
        let s_148_5: u32 = fn_state.u_23166;
        // D s_148_6: cast zx s_148_5 -> bv
        let s_148_6: Bits = Bits::new(s_148_5 as u128, 32u16);
        // D s_148_7: bit-extract s_148_6 s_148_3 s_148_4
        let s_148_7: Bits = (Bits::new(
            ((s_148_6) >> (s_148_3)).value(),
            u16::try_from(s_148_4).unwrap(),
        ));
        // D s_148_8: cast reint s_148_7 -> u8
        let s_148_8: u8 = (s_148_7.value() as u8);
        // C s_148_9: const #8s : i
        let s_148_9: i128 = 8;
        // C s_148_10: const #4s : i
        let s_148_10: i128 = 4;
        // D s_148_11: read-var u#23166:u32
        let s_148_11: u32 = fn_state.u_23166;
        // D s_148_12: cast zx s_148_11 -> bv
        let s_148_12: Bits = Bits::new(s_148_11 as u128, 32u16);
        // D s_148_13: bit-extract s_148_12 s_148_9 s_148_10
        let s_148_13: Bits = (Bits::new(
            ((s_148_12) >> (s_148_9)).value(),
            u16::try_from(s_148_10).unwrap(),
        ));
        // D s_148_14: cast reint s_148_13 -> u8
        let s_148_14: u8 = (s_148_13.value() as u8);
        // D s_148_15: call decode_wfe_aarch64_instrs_system_hints(s_148_8, s_148_14)
        let s_148_15: () = decode_wfe_aarch64_instrs_system_hints(
            state,
            tracer,
            s_148_8,
            s_148_14,
        );
        // N s_148_16: return
        return;
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_149_0: read-var merge#var.1:struct
        let s_149_0: u32 = fn_state.merge_var._1;
        // D s_149_1: write-var u#23170 <= s_149_0
        fn_state.u_23170 = s_149_0;
        // D s_149_2: read-var u#23170:u32
        let s_149_2: u32 = fn_state.u_23170;
        // D s_149_3: cast zx s_149_2 -> bv
        let s_149_3: Bits = Bits::new(s_149_2 as u128, 32u16);
        // C s_149_4: const #3573751935u : u32
        let s_149_4: u32 = 3573751935;
        // C s_149_5: cast zx s_149_4 -> bv
        let s_149_5: Bits = Bits::new(s_149_4 as u128, 32u16);
        // D s_149_6: cmp-eq s_149_3 s_149_5
        let s_149_6: bool = ((s_149_3) == (s_149_5));
        // N s_149_7: branch s_149_6 b447 b150
        if s_149_6 {
            return block_447(state, tracer, fn_state);
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
        // D s_150_1: write-var gs#374002 <= s_150_0
        fn_state.gs_374002 = s_150_0;
        // N s_150_2: jump b151
        return block_151(state, tracer, fn_state);
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_151_0: read-var gs#374002:u8
        let s_151_0: bool = fn_state.gs_374002;
        // D s_151_1: not s_151_0
        let s_151_1: bool = !s_151_0;
        // N s_151_2: branch s_151_1 b153 b152
        if s_151_1 {
            return block_153(state, tracer, fn_state);
        } else {
            return block_152(state, tracer, fn_state);
        };
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_152_0: const #111s : i
        let s_152_0: i128 = 111;
        // C s_152_1: const #14696u : u32
        let s_152_1: u32 = 14696;
        // N s_152_2: write-reg s_152_1 <= s_152_0
        let s_152_2: () = {
            state.write_register::<i128>(s_152_1 as isize, s_152_0);
            tracer.write_register(s_152_1 as isize, s_152_0);
        };
        // C s_152_3: const #5s : i
        let s_152_3: i128 = 5;
        // C s_152_4: const #3s : i
        let s_152_4: i128 = 3;
        // D s_152_5: read-var u#23170:u32
        let s_152_5: u32 = fn_state.u_23170;
        // D s_152_6: cast zx s_152_5 -> bv
        let s_152_6: Bits = Bits::new(s_152_5 as u128, 32u16);
        // D s_152_7: bit-extract s_152_6 s_152_3 s_152_4
        let s_152_7: Bits = (Bits::new(
            ((s_152_6) >> (s_152_3)).value(),
            u16::try_from(s_152_4).unwrap(),
        ));
        // D s_152_8: cast reint s_152_7 -> u8
        let s_152_8: u8 = (s_152_7.value() as u8);
        // C s_152_9: const #8s : i
        let s_152_9: i128 = 8;
        // C s_152_10: const #4s : i
        let s_152_10: i128 = 4;
        // D s_152_11: read-var u#23170:u32
        let s_152_11: u32 = fn_state.u_23170;
        // D s_152_12: cast zx s_152_11 -> bv
        let s_152_12: Bits = Bits::new(s_152_11 as u128, 32u16);
        // D s_152_13: bit-extract s_152_12 s_152_9 s_152_10
        let s_152_13: Bits = (Bits::new(
            ((s_152_12) >> (s_152_9)).value(),
            u16::try_from(s_152_10).unwrap(),
        ));
        // D s_152_14: cast reint s_152_13 -> u8
        let s_152_14: u8 = (s_152_13.value() as u8);
        // D s_152_15: call decode_wfi_aarch64_instrs_system_hints(s_152_8, s_152_14)
        let s_152_15: () = decode_wfi_aarch64_instrs_system_hints(
            state,
            tracer,
            s_152_8,
            s_152_14,
        );
        // N s_152_16: return
        return;
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_153_0: read-var merge#var.1:struct
        let s_153_0: u32 = fn_state.merge_var._1;
        // D s_153_1: write-var u#23174 <= s_153_0
        fn_state.u_23174 = s_153_0;
        // D s_153_2: read-var u#23174:u32
        let s_153_2: u32 = fn_state.u_23174;
        // D s_153_3: cast zx s_153_2 -> bv
        let s_153_3: Bits = Bits::new(s_153_2 as u128, 32u16);
        // C s_153_4: const #3573751871u : u32
        let s_153_4: u32 = 3573751871;
        // C s_153_5: cast zx s_153_4 -> bv
        let s_153_5: Bits = Bits::new(s_153_4 as u128, 32u16);
        // D s_153_6: cmp-eq s_153_3 s_153_5
        let s_153_6: bool = ((s_153_3) == (s_153_5));
        // N s_153_7: branch s_153_6 b446 b154
        if s_153_6 {
            return block_446(state, tracer, fn_state);
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
        // D s_154_1: write-var gs#374011 <= s_154_0
        fn_state.gs_374011 = s_154_0;
        // N s_154_2: jump b155
        return block_155(state, tracer, fn_state);
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_155_0: read-var gs#374011:u8
        let s_155_0: bool = fn_state.gs_374011;
        // D s_155_1: not s_155_0
        let s_155_1: bool = !s_155_0;
        // N s_155_2: branch s_155_1 b157 b156
        if s_155_1 {
            return block_157(state, tracer, fn_state);
        } else {
            return block_156(state, tracer, fn_state);
        };
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_156_0: const #112s : i
        let s_156_0: i128 = 112;
        // C s_156_1: const #14696u : u32
        let s_156_1: u32 = 14696;
        // N s_156_2: write-reg s_156_1 <= s_156_0
        let s_156_2: () = {
            state.write_register::<i128>(s_156_1 as isize, s_156_0);
            tracer.write_register(s_156_1 as isize, s_156_0);
        };
        // C s_156_3: const #5s : i
        let s_156_3: i128 = 5;
        // C s_156_4: const #3s : i
        let s_156_4: i128 = 3;
        // D s_156_5: read-var u#23174:u32
        let s_156_5: u32 = fn_state.u_23174;
        // D s_156_6: cast zx s_156_5 -> bv
        let s_156_6: Bits = Bits::new(s_156_5 as u128, 32u16);
        // D s_156_7: bit-extract s_156_6 s_156_3 s_156_4
        let s_156_7: Bits = (Bits::new(
            ((s_156_6) >> (s_156_3)).value(),
            u16::try_from(s_156_4).unwrap(),
        ));
        // D s_156_8: cast reint s_156_7 -> u8
        let s_156_8: u8 = (s_156_7.value() as u8);
        // C s_156_9: const #8s : i
        let s_156_9: i128 = 8;
        // C s_156_10: const #4s : i
        let s_156_10: i128 = 4;
        // D s_156_11: read-var u#23174:u32
        let s_156_11: u32 = fn_state.u_23174;
        // D s_156_12: cast zx s_156_11 -> bv
        let s_156_12: Bits = Bits::new(s_156_11 as u128, 32u16);
        // D s_156_13: bit-extract s_156_12 s_156_9 s_156_10
        let s_156_13: Bits = (Bits::new(
            ((s_156_12) >> (s_156_9)).value(),
            u16::try_from(s_156_10).unwrap(),
        ));
        // D s_156_14: cast reint s_156_13 -> u8
        let s_156_14: u8 = (s_156_13.value() as u8);
        // D s_156_15: call decode_yield_aarch64_instrs_system_hints(s_156_8, s_156_14)
        let s_156_15: () = decode_yield_aarch64_instrs_system_hints(
            state,
            tracer,
            s_156_8,
            s_156_14,
        );
        // N s_156_16: return
        return;
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_157_0: read-var merge#var.1:struct
        let s_157_0: u32 = fn_state.merge_var._1;
        // D s_157_1: write-var u#23178 <= s_157_0
        fn_state.u_23178 = s_157_0;
        // C s_157_2: const #24s : i
        let s_157_2: i128 = 24;
        // D s_157_3: read-var u#23178:u32
        let s_157_3: u32 = fn_state.u_23178;
        // D s_157_4: cast zx s_157_3 -> bv
        let s_157_4: Bits = Bits::new(s_157_3 as u128, 32u16);
        // C s_157_5: const #1s : i64
        let s_157_5: i64 = 1;
        // C s_157_6: cast zx s_157_5 -> i
        let s_157_6: i128 = (i128::try_from(s_157_5).unwrap());
        // C s_157_7: const #6s : i
        let s_157_7: i128 = 6;
        // C s_157_8: add s_157_7 s_157_6
        let s_157_8: i128 = (s_157_7 + s_157_6);
        // D s_157_9: bit-extract s_157_4 s_157_2 s_157_8
        let s_157_9: Bits = (Bits::new(
            ((s_157_4) >> (s_157_2)).value(),
            u16::try_from(s_157_8).unwrap(),
        ));
        // D s_157_10: cast reint s_157_9 -> u8
        let s_157_10: u8 = (s_157_9.value() as u8);
        // D s_157_11: cast zx s_157_10 -> bv
        let s_157_11: Bits = Bits::new(s_157_10 as u128, 7u16);
        // C s_157_12: const #53u : u8
        let s_157_12: u8 = 53;
        // C s_157_13: cast zx s_157_12 -> bv
        let s_157_13: Bits = Bits::new(s_157_12 as u128, 7u16);
        // D s_157_14: cmp-eq s_157_11 s_157_13
        let s_157_14: bool = ((s_157_11) == (s_157_13));
        // N s_157_15: branch s_157_14 b445 b158
        if s_157_14 {
            return block_445(state, tracer, fn_state);
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
        // D s_158_1: write-var gs#374022 <= s_158_0
        fn_state.gs_374022 = s_158_0;
        // N s_158_2: jump b159
        return block_159(state, tracer, fn_state);
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_159_0: read-var gs#374022:u8
        let s_159_0: bool = fn_state.gs_374022;
        // D s_159_1: not s_159_0
        let s_159_1: bool = !s_159_0;
        // N s_159_2: branch s_159_1 b161 b160
        if s_159_1 {
            return block_161(state, tracer, fn_state);
        } else {
            return block_160(state, tracer, fn_state);
        };
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_160_0: const #117s : i
        let s_160_0: i128 = 117;
        // C s_160_1: const #14696u : u32
        let s_160_1: u32 = 14696;
        // N s_160_2: write-reg s_160_1 <= s_160_0
        let s_160_2: () = {
            state.write_register::<i128>(s_160_1 as isize, s_160_0);
            tracer.write_register(s_160_1 as isize, s_160_0);
        };
        // C s_160_3: const #0s : i
        let s_160_3: i128 = 0;
        // C s_160_4: const #5s : i
        let s_160_4: i128 = 5;
        // D s_160_5: read-var u#23178:u32
        let s_160_5: u32 = fn_state.u_23178;
        // D s_160_6: cast zx s_160_5 -> bv
        let s_160_6: Bits = Bits::new(s_160_5 as u128, 32u16);
        // D s_160_7: bit-extract s_160_6 s_160_3 s_160_4
        let s_160_7: Bits = (Bits::new(
            ((s_160_6) >> (s_160_3)).value(),
            u16::try_from(s_160_4).unwrap(),
        ));
        // D s_160_8: cast reint s_160_7 -> u8
        let s_160_8: u8 = (s_160_7.value() as u8);
        // C s_160_9: const #5s : i
        let s_160_9: i128 = 5;
        // C s_160_10: const #19s : i
        let s_160_10: i128 = 19;
        // D s_160_11: read-var u#23178:u32
        let s_160_11: u32 = fn_state.u_23178;
        // D s_160_12: cast zx s_160_11 -> bv
        let s_160_12: Bits = Bits::new(s_160_11 as u128, 32u16);
        // D s_160_13: bit-extract s_160_12 s_160_9 s_160_10
        let s_160_13: Bits = (Bits::new(
            ((s_160_12) >> (s_160_9)).value(),
            u16::try_from(s_160_10).unwrap(),
        ));
        // D s_160_14: cast reint s_160_13 -> u19
        let s_160_14: u32 = (s_160_13.value() as u32);
        // C s_160_15: const #24s : i
        let s_160_15: i128 = 24;
        // C s_160_16: const #1s : i
        let s_160_16: i128 = 1;
        // D s_160_17: read-var u#23178:u32
        let s_160_17: u32 = fn_state.u_23178;
        // D s_160_18: cast zx s_160_17 -> bv
        let s_160_18: Bits = Bits::new(s_160_17 as u128, 32u16);
        // D s_160_19: bit-extract s_160_18 s_160_15 s_160_16
        let s_160_19: Bits = (Bits::new(
            ((s_160_18) >> (s_160_15)).value(),
            u16::try_from(s_160_16).unwrap(),
        ));
        // D s_160_20: cast reint s_160_19 -> u8
        let s_160_20: bool = ((s_160_19.value()) != 0);
        // C s_160_21: const #31s : i
        let s_160_21: i128 = 31;
        // C s_160_22: const #1s : i
        let s_160_22: i128 = 1;
        // D s_160_23: read-var u#23178:u32
        let s_160_23: u32 = fn_state.u_23178;
        // D s_160_24: cast zx s_160_23 -> bv
        let s_160_24: Bits = Bits::new(s_160_23 as u128, 32u16);
        // D s_160_25: bit-extract s_160_24 s_160_21 s_160_22
        let s_160_25: Bits = (Bits::new(
            ((s_160_24) >> (s_160_21)).value(),
            u16::try_from(s_160_22).unwrap(),
        ));
        // D s_160_26: cast reint s_160_25 -> u8
        let s_160_26: bool = ((s_160_25.value()) != 0);
        // D s_160_27: call decode_cbnz_aarch64_instrs_branch_conditional_compare(s_160_8, s_160_14, s_160_20, s_160_26)
        let s_160_27: () = decode_cbnz_aarch64_instrs_branch_conditional_compare(
            state,
            tracer,
            s_160_8,
            s_160_14,
            s_160_20,
            s_160_26,
        );
        // N s_160_28: return
        return;
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_161_0: read-var merge#var.1:struct
        let s_161_0: u32 = fn_state.merge_var._1;
        // D s_161_1: write-var u#23182 <= s_161_0
        fn_state.u_23182 = s_161_0;
        // C s_161_2: const #24s : i
        let s_161_2: i128 = 24;
        // D s_161_3: read-var u#23182:u32
        let s_161_3: u32 = fn_state.u_23182;
        // D s_161_4: cast zx s_161_3 -> bv
        let s_161_4: Bits = Bits::new(s_161_3 as u128, 32u16);
        // C s_161_5: const #1s : i64
        let s_161_5: i64 = 1;
        // C s_161_6: cast zx s_161_5 -> i
        let s_161_6: i128 = (i128::try_from(s_161_5).unwrap());
        // C s_161_7: const #6s : i
        let s_161_7: i128 = 6;
        // C s_161_8: add s_161_7 s_161_6
        let s_161_8: i128 = (s_161_7 + s_161_6);
        // D s_161_9: bit-extract s_161_4 s_161_2 s_161_8
        let s_161_9: Bits = (Bits::new(
            ((s_161_4) >> (s_161_2)).value(),
            u16::try_from(s_161_8).unwrap(),
        ));
        // D s_161_10: cast reint s_161_9 -> u8
        let s_161_10: u8 = (s_161_9.value() as u8);
        // D s_161_11: cast zx s_161_10 -> bv
        let s_161_11: Bits = Bits::new(s_161_10 as u128, 7u16);
        // C s_161_12: const #52u : u8
        let s_161_12: u8 = 52;
        // C s_161_13: cast zx s_161_12 -> bv
        let s_161_13: Bits = Bits::new(s_161_12 as u128, 7u16);
        // D s_161_14: cmp-eq s_161_11 s_161_13
        let s_161_14: bool = ((s_161_11) == (s_161_13));
        // N s_161_15: branch s_161_14 b444 b162
        if s_161_14 {
            return block_444(state, tracer, fn_state);
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
        // D s_162_1: write-var gs#374037 <= s_162_0
        fn_state.gs_374037 = s_162_0;
        // N s_162_2: jump b163
        return block_163(state, tracer, fn_state);
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_163_0: read-var gs#374037:u8
        let s_163_0: bool = fn_state.gs_374037;
        // D s_163_1: not s_163_0
        let s_163_1: bool = !s_163_0;
        // N s_163_2: branch s_163_1 b165 b164
        if s_163_1 {
            return block_165(state, tracer, fn_state);
        } else {
            return block_164(state, tracer, fn_state);
        };
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_164_0: const #118s : i
        let s_164_0: i128 = 118;
        // C s_164_1: const #14696u : u32
        let s_164_1: u32 = 14696;
        // N s_164_2: write-reg s_164_1 <= s_164_0
        let s_164_2: () = {
            state.write_register::<i128>(s_164_1 as isize, s_164_0);
            tracer.write_register(s_164_1 as isize, s_164_0);
        };
        // C s_164_3: const #0s : i
        let s_164_3: i128 = 0;
        // C s_164_4: const #5s : i
        let s_164_4: i128 = 5;
        // D s_164_5: read-var u#23182:u32
        let s_164_5: u32 = fn_state.u_23182;
        // D s_164_6: cast zx s_164_5 -> bv
        let s_164_6: Bits = Bits::new(s_164_5 as u128, 32u16);
        // D s_164_7: bit-extract s_164_6 s_164_3 s_164_4
        let s_164_7: Bits = (Bits::new(
            ((s_164_6) >> (s_164_3)).value(),
            u16::try_from(s_164_4).unwrap(),
        ));
        // D s_164_8: cast reint s_164_7 -> u8
        let s_164_8: u8 = (s_164_7.value() as u8);
        // C s_164_9: const #5s : i
        let s_164_9: i128 = 5;
        // C s_164_10: const #19s : i
        let s_164_10: i128 = 19;
        // D s_164_11: read-var u#23182:u32
        let s_164_11: u32 = fn_state.u_23182;
        // D s_164_12: cast zx s_164_11 -> bv
        let s_164_12: Bits = Bits::new(s_164_11 as u128, 32u16);
        // D s_164_13: bit-extract s_164_12 s_164_9 s_164_10
        let s_164_13: Bits = (Bits::new(
            ((s_164_12) >> (s_164_9)).value(),
            u16::try_from(s_164_10).unwrap(),
        ));
        // D s_164_14: cast reint s_164_13 -> u19
        let s_164_14: u32 = (s_164_13.value() as u32);
        // C s_164_15: const #24s : i
        let s_164_15: i128 = 24;
        // C s_164_16: const #1s : i
        let s_164_16: i128 = 1;
        // D s_164_17: read-var u#23182:u32
        let s_164_17: u32 = fn_state.u_23182;
        // D s_164_18: cast zx s_164_17 -> bv
        let s_164_18: Bits = Bits::new(s_164_17 as u128, 32u16);
        // D s_164_19: bit-extract s_164_18 s_164_15 s_164_16
        let s_164_19: Bits = (Bits::new(
            ((s_164_18) >> (s_164_15)).value(),
            u16::try_from(s_164_16).unwrap(),
        ));
        // D s_164_20: cast reint s_164_19 -> u8
        let s_164_20: bool = ((s_164_19.value()) != 0);
        // C s_164_21: const #31s : i
        let s_164_21: i128 = 31;
        // C s_164_22: const #1s : i
        let s_164_22: i128 = 1;
        // D s_164_23: read-var u#23182:u32
        let s_164_23: u32 = fn_state.u_23182;
        // D s_164_24: cast zx s_164_23 -> bv
        let s_164_24: Bits = Bits::new(s_164_23 as u128, 32u16);
        // D s_164_25: bit-extract s_164_24 s_164_21 s_164_22
        let s_164_25: Bits = (Bits::new(
            ((s_164_24) >> (s_164_21)).value(),
            u16::try_from(s_164_22).unwrap(),
        ));
        // D s_164_26: cast reint s_164_25 -> u8
        let s_164_26: bool = ((s_164_25.value()) != 0);
        // D s_164_27: call decode_cbz_aarch64_instrs_branch_conditional_compare(s_164_8, s_164_14, s_164_20, s_164_26)
        let s_164_27: () = decode_cbz_aarch64_instrs_branch_conditional_compare(
            state,
            tracer,
            s_164_8,
            s_164_14,
            s_164_20,
            s_164_26,
        );
        // N s_164_28: return
        return;
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_165_0: read-var merge#var.1:struct
        let s_165_0: u32 = fn_state.merge_var._1;
        // D s_165_1: write-var u#23188 <= s_165_0
        fn_state.u_23188 = s_165_0;
        // D s_165_2: read-var u#23188:u32
        let s_165_2: u32 = fn_state.u_23188;
        // D s_165_3: cast zx s_165_2 -> bv
        let s_165_3: Bits = Bits::new(s_165_2 as u128, 32u16);
        // C s_165_4: const #3573563423u : u32
        let s_165_4: u32 = 3573563423;
        // C s_165_5: cast zx s_165_4 -> bv
        let s_165_5: Bits = Bits::new(s_165_4 as u128, 32u16);
        // D s_165_6: cmp-eq s_165_3 s_165_5
        let s_165_6: bool = ((s_165_3) == (s_165_5));
        // N s_165_7: branch s_165_6 b443 b166
        if s_165_6 {
            return block_443(state, tracer, fn_state);
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
        // D s_166_1: write-var gs#374050 <= s_166_0
        fn_state.gs_374050 = s_166_0;
        // N s_166_2: jump b167
        return block_167(state, tracer, fn_state);
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_167_0: read-var gs#374050:u8
        let s_167_0: bool = fn_state.gs_374050;
        // D s_167_1: not s_167_0
        let s_167_1: bool = !s_167_0;
        // N s_167_2: branch s_167_1 b180 b168
        if s_167_1 {
            return block_180(state, tracer, fn_state);
        } else {
            return block_168(state, tracer, fn_state);
        };
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_168_0: const #123s : i
        let s_168_0: i128 = 123;
        // C s_168_1: const #14696u : u32
        let s_168_1: u32 = 14696;
        // N s_168_2: write-reg s_168_1 <= s_168_0
        let s_168_2: () = {
            state.write_register::<i128>(s_168_1 as isize, s_168_0);
            tracer.write_register(s_168_1 as isize, s_168_0);
        };
        // C s_168_3: const #8s : i
        let s_168_3: i128 = 8;
        // C s_168_4: const #4s : i
        let s_168_4: i128 = 4;
        // D s_168_5: read-var u#23188:u32
        let s_168_5: u32 = fn_state.u_23188;
        // D s_168_6: cast zx s_168_5 -> bv
        let s_168_6: Bits = Bits::new(s_168_5 as u128, 32u16);
        // D s_168_7: bit-extract s_168_6 s_168_3 s_168_4
        let s_168_7: Bits = (Bits::new(
            ((s_168_6) >> (s_168_3)).value(),
            u16::try_from(s_168_4).unwrap(),
        ));
        // D s_168_8: cast reint s_168_7 -> u8
        let s_168_8: u8 = (s_168_7.value() as u8);
        // D s_168_9: write-var u#23189 <= s_168_8
        fn_state.u_23189 = s_168_8;
        // C s_168_10: const #8s : i
        let s_168_10: i128 = 8;
        // D s_168_11: read-var u#23188:u32
        let s_168_11: u32 = fn_state.u_23188;
        // D s_168_12: cast zx s_168_11 -> bv
        let s_168_12: Bits = Bits::new(s_168_11 as u128, 32u16);
        // C s_168_13: const #1u : u64
        let s_168_13: u64 = 1;
        // D s_168_14: bit-extract s_168_12 s_168_10 s_168_13
        let s_168_14: Bits = (Bits::new(
            ((s_168_12) >> (s_168_10)).value(),
            u16::try_from(s_168_13).unwrap(),
        ));
        // D s_168_15: cast reint s_168_14 -> u8
        let s_168_15: bool = ((s_168_14.value()) != 0);
        // C s_168_16: const #0s : i
        let s_168_16: i128 = 0;
        // C s_168_17: const #0u : u64
        let s_168_17: u64 = 0;
        // D s_168_18: cast zx s_168_15 -> u64
        let s_168_18: u64 = (s_168_15 as u64);
        // C s_168_19: const #1u : u64
        let s_168_19: u64 = 1;
        // D s_168_20: and s_168_18 s_168_19
        let s_168_20: u64 = ((s_168_18) & (s_168_19));
        // D s_168_21: cmp-eq s_168_20 s_168_19
        let s_168_21: bool = ((s_168_20) == (s_168_19));
        // D s_168_22: lsl s_168_18 s_168_16
        let s_168_22: u64 = s_168_18 << s_168_16;
        // D s_168_23: or s_168_17 s_168_22
        let s_168_23: u64 = ((s_168_17) | (s_168_22));
        // D s_168_24: cmpl s_168_22
        let s_168_24: u64 = !s_168_22;
        // D s_168_25: and s_168_17 s_168_24
        let s_168_25: u64 = ((s_168_17) & (s_168_24));
        // D s_168_26: select s_168_21 s_168_23 s_168_25
        let s_168_26: u64 = if s_168_21 { s_168_23 } else { s_168_25 };
        // D s_168_27: cast trunc s_168_26 -> u8
        let s_168_27: bool = ((s_168_26) != 0);
        // D s_168_28: cast zx s_168_27 -> bv
        let s_168_28: Bits = Bits::new(s_168_27 as u128, 1u16);
        // C s_168_29: const #0u : u8
        let s_168_29: bool = false;
        // C s_168_30: cast zx s_168_29 -> bv
        let s_168_30: Bits = Bits::new(s_168_29 as u128, 1u16);
        // D s_168_31: cmp-ne s_168_28 s_168_30
        let s_168_31: bool = ((s_168_28) != (s_168_30));
        // N s_168_32: branch s_168_31 b179 b169
        if s_168_31 {
            return block_179(state, tracer, fn_state);
        } else {
            return block_169(state, tracer, fn_state);
        };
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_169_0: const #9s : i
        let s_169_0: i128 = 9;
        // D s_169_1: read-var u#23188:u32
        let s_169_1: u32 = fn_state.u_23188;
        // D s_169_2: cast zx s_169_1 -> bv
        let s_169_2: Bits = Bits::new(s_169_1 as u128, 32u16);
        // C s_169_3: const #1u : u64
        let s_169_3: u64 = 1;
        // D s_169_4: bit-extract s_169_2 s_169_0 s_169_3
        let s_169_4: Bits = (Bits::new(
            ((s_169_2) >> (s_169_0)).value(),
            u16::try_from(s_169_3).unwrap(),
        ));
        // D s_169_5: cast reint s_169_4 -> u8
        let s_169_5: bool = ((s_169_4.value()) != 0);
        // C s_169_6: const #0s : i
        let s_169_6: i128 = 0;
        // C s_169_7: const #0u : u64
        let s_169_7: u64 = 0;
        // D s_169_8: cast zx s_169_5 -> u64
        let s_169_8: u64 = (s_169_5 as u64);
        // C s_169_9: const #1u : u64
        let s_169_9: u64 = 1;
        // D s_169_10: and s_169_8 s_169_9
        let s_169_10: u64 = ((s_169_8) & (s_169_9));
        // D s_169_11: cmp-eq s_169_10 s_169_9
        let s_169_11: bool = ((s_169_10) == (s_169_9));
        // D s_169_12: lsl s_169_8 s_169_6
        let s_169_12: u64 = s_169_8 << s_169_6;
        // D s_169_13: or s_169_7 s_169_12
        let s_169_13: u64 = ((s_169_7) | (s_169_12));
        // D s_169_14: cmpl s_169_12
        let s_169_14: u64 = !s_169_12;
        // D s_169_15: and s_169_7 s_169_14
        let s_169_15: u64 = ((s_169_7) & (s_169_14));
        // D s_169_16: select s_169_11 s_169_13 s_169_15
        let s_169_16: u64 = if s_169_11 { s_169_13 } else { s_169_15 };
        // D s_169_17: cast trunc s_169_16 -> u8
        let s_169_17: bool = ((s_169_16) != 0);
        // D s_169_18: cast zx s_169_17 -> bv
        let s_169_18: Bits = Bits::new(s_169_17 as u128, 1u16);
        // C s_169_19: const #0u : u8
        let s_169_19: bool = false;
        // C s_169_20: cast zx s_169_19 -> bv
        let s_169_20: Bits = Bits::new(s_169_19 as u128, 1u16);
        // D s_169_21: cmp-ne s_169_18 s_169_20
        let s_169_21: bool = ((s_169_18) != (s_169_20));
        // D s_169_22: write-var gs#374059 <= s_169_21
        fn_state.gs_374059 = s_169_21;
        // N s_169_23: jump b170
        return block_170(state, tracer, fn_state);
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_170_0: read-var gs#374059:u8
        let s_170_0: bool = fn_state.gs_374059;
        // N s_170_1: branch s_170_0 b178 b171
        if s_170_0 {
            return block_178(state, tracer, fn_state);
        } else {
            return block_171(state, tracer, fn_state);
        };
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_171_0: const #10s : i
        let s_171_0: i128 = 10;
        // D s_171_1: read-var u#23188:u32
        let s_171_1: u32 = fn_state.u_23188;
        // D s_171_2: cast zx s_171_1 -> bv
        let s_171_2: Bits = Bits::new(s_171_1 as u128, 32u16);
        // C s_171_3: const #1u : u64
        let s_171_3: u64 = 1;
        // D s_171_4: bit-extract s_171_2 s_171_0 s_171_3
        let s_171_4: Bits = (Bits::new(
            ((s_171_2) >> (s_171_0)).value(),
            u16::try_from(s_171_3).unwrap(),
        ));
        // D s_171_5: cast reint s_171_4 -> u8
        let s_171_5: bool = ((s_171_4.value()) != 0);
        // C s_171_6: const #0s : i
        let s_171_6: i128 = 0;
        // C s_171_7: const #0u : u64
        let s_171_7: u64 = 0;
        // D s_171_8: cast zx s_171_5 -> u64
        let s_171_8: u64 = (s_171_5 as u64);
        // C s_171_9: const #1u : u64
        let s_171_9: u64 = 1;
        // D s_171_10: and s_171_8 s_171_9
        let s_171_10: u64 = ((s_171_8) & (s_171_9));
        // D s_171_11: cmp-eq s_171_10 s_171_9
        let s_171_11: bool = ((s_171_10) == (s_171_9));
        // D s_171_12: lsl s_171_8 s_171_6
        let s_171_12: u64 = s_171_8 << s_171_6;
        // D s_171_13: or s_171_7 s_171_12
        let s_171_13: u64 = ((s_171_7) | (s_171_12));
        // D s_171_14: cmpl s_171_12
        let s_171_14: u64 = !s_171_12;
        // D s_171_15: and s_171_7 s_171_14
        let s_171_15: u64 = ((s_171_7) & (s_171_14));
        // D s_171_16: select s_171_11 s_171_13 s_171_15
        let s_171_16: u64 = if s_171_11 { s_171_13 } else { s_171_15 };
        // D s_171_17: cast trunc s_171_16 -> u8
        let s_171_17: bool = ((s_171_16) != 0);
        // D s_171_18: cast zx s_171_17 -> bv
        let s_171_18: Bits = Bits::new(s_171_17 as u128, 1u16);
        // C s_171_19: const #0u : u8
        let s_171_19: bool = false;
        // C s_171_20: cast zx s_171_19 -> bv
        let s_171_20: Bits = Bits::new(s_171_19 as u128, 1u16);
        // D s_171_21: cmp-ne s_171_18 s_171_20
        let s_171_21: bool = ((s_171_18) != (s_171_20));
        // D s_171_22: write-var gs#374062 <= s_171_21
        fn_state.gs_374062 = s_171_21;
        // N s_171_23: jump b172
        return block_172(state, tracer, fn_state);
    }
    fn block_172<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_172_0: read-var gs#374062:u8
        let s_172_0: bool = fn_state.gs_374062;
        // N s_172_1: branch s_172_0 b177 b173
        if s_172_0 {
            return block_177(state, tracer, fn_state);
        } else {
            return block_173(state, tracer, fn_state);
        };
    }
    fn block_173<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_173_0: const #11s : i
        let s_173_0: i128 = 11;
        // D s_173_1: read-var u#23188:u32
        let s_173_1: u32 = fn_state.u_23188;
        // D s_173_2: cast zx s_173_1 -> bv
        let s_173_2: Bits = Bits::new(s_173_1 as u128, 32u16);
        // C s_173_3: const #1u : u64
        let s_173_3: u64 = 1;
        // D s_173_4: bit-extract s_173_2 s_173_0 s_173_3
        let s_173_4: Bits = (Bits::new(
            ((s_173_2) >> (s_173_0)).value(),
            u16::try_from(s_173_3).unwrap(),
        ));
        // D s_173_5: cast reint s_173_4 -> u8
        let s_173_5: bool = ((s_173_4.value()) != 0);
        // C s_173_6: const #0s : i
        let s_173_6: i128 = 0;
        // C s_173_7: const #0u : u64
        let s_173_7: u64 = 0;
        // D s_173_8: cast zx s_173_5 -> u64
        let s_173_8: u64 = (s_173_5 as u64);
        // C s_173_9: const #1u : u64
        let s_173_9: u64 = 1;
        // D s_173_10: and s_173_8 s_173_9
        let s_173_10: u64 = ((s_173_8) & (s_173_9));
        // D s_173_11: cmp-eq s_173_10 s_173_9
        let s_173_11: bool = ((s_173_10) == (s_173_9));
        // D s_173_12: lsl s_173_8 s_173_6
        let s_173_12: u64 = s_173_8 << s_173_6;
        // D s_173_13: or s_173_7 s_173_12
        let s_173_13: u64 = ((s_173_7) | (s_173_12));
        // D s_173_14: cmpl s_173_12
        let s_173_14: u64 = !s_173_12;
        // D s_173_15: and s_173_7 s_173_14
        let s_173_15: u64 = ((s_173_7) & (s_173_14));
        // D s_173_16: select s_173_11 s_173_13 s_173_15
        let s_173_16: u64 = if s_173_11 { s_173_13 } else { s_173_15 };
        // D s_173_17: cast trunc s_173_16 -> u8
        let s_173_17: bool = ((s_173_16) != 0);
        // D s_173_18: cast zx s_173_17 -> bv
        let s_173_18: Bits = Bits::new(s_173_17 as u128, 1u16);
        // C s_173_19: const #0u : u8
        let s_173_19: bool = false;
        // C s_173_20: cast zx s_173_19 -> bv
        let s_173_20: Bits = Bits::new(s_173_19 as u128, 1u16);
        // D s_173_21: cmp-ne s_173_18 s_173_20
        let s_173_21: bool = ((s_173_18) != (s_173_20));
        // D s_173_22: write-var gs#374065 <= s_173_21
        fn_state.gs_374065 = s_173_21;
        // N s_173_23: jump b174
        return block_174(state, tracer, fn_state);
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_174_0: read-var gs#374065:u8
        let s_174_0: bool = fn_state.gs_374065;
        // N s_174_1: branch s_174_0 b176 b175
        if s_174_0 {
            return block_176(state, tracer, fn_state);
        } else {
            return block_175(state, tracer, fn_state);
        };
    }
    fn block_175<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_175_0: read-var u#23189:u8
        let s_175_0: u8 = fn_state.u_23189;
        // D s_175_1: call decode_cfinv_aarch64_instrs_integer_flags_cfinv(s_175_0)
        let s_175_1: () = decode_cfinv_aarch64_instrs_integer_flags_cfinv(
            state,
            tracer,
            s_175_0,
        );
        // N s_175_2: return
        return;
    }
    fn block_176<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_176_0: panic
        panic!("{:?}", ());
        // N s_176_1: return
        return;
    }
    fn block_177<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_177_0: const #1u : u8
        let s_177_0: bool = true;
        // D s_177_1: write-var gs#374065 <= s_177_0
        fn_state.gs_374065 = s_177_0;
        // N s_177_2: jump b174
        return block_174(state, tracer, fn_state);
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_178_0: const #1u : u8
        let s_178_0: bool = true;
        // D s_178_1: write-var gs#374062 <= s_178_0
        fn_state.gs_374062 = s_178_0;
        // N s_178_2: jump b172
        return block_172(state, tracer, fn_state);
    }
    fn block_179<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_179_0: const #1u : u8
        let s_179_0: bool = true;
        // D s_179_1: write-var gs#374059 <= s_179_0
        fn_state.gs_374059 = s_179_0;
        // N s_179_2: jump b170
        return block_170(state, tracer, fn_state);
    }
    fn block_180<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_180_0: read-var merge#var.1:struct
        let s_180_0: u32 = fn_state.merge_var._1;
        // D s_180_1: write-var u#23191 <= s_180_0
        fn_state.u_23191 = s_180_0;
        // C s_180_2: const #12s : i
        let s_180_2: i128 = 12;
        // D s_180_3: read-var u#23191:u32
        let s_180_3: u32 = fn_state.u_23191;
        // D s_180_4: cast zx s_180_3 -> bv
        let s_180_4: Bits = Bits::new(s_180_3 as u128, 32u16);
        // C s_180_5: const #1s : i64
        let s_180_5: i64 = 1;
        // C s_180_6: cast zx s_180_5 -> i
        let s_180_6: i128 = (i128::try_from(s_180_5).unwrap());
        // C s_180_7: const #19s : i
        let s_180_7: i128 = 19;
        // C s_180_8: add s_180_7 s_180_6
        let s_180_8: i128 = (s_180_7 + s_180_6);
        // D s_180_9: bit-extract s_180_4 s_180_2 s_180_8
        let s_180_9: Bits = (Bits::new(
            ((s_180_4) >> (s_180_2)).value(),
            u16::try_from(s_180_8).unwrap(),
        ));
        // D s_180_10: cast reint s_180_9 -> u20
        let s_180_10: u32 = (s_180_9.value() as u32);
        // D s_180_11: cast zx s_180_10 -> bv
        let s_180_11: Bits = Bits::new(s_180_10 as u128, 20u16);
        // C s_180_12: const #872499u : u20
        let s_180_12: u32 = 872499;
        // C s_180_13: cast zx s_180_12 -> bv
        let s_180_13: Bits = Bits::new(s_180_12 as u128, 20u16);
        // D s_180_14: cmp-eq s_180_11 s_180_13
        let s_180_14: bool = ((s_180_11) == (s_180_13));
        // N s_180_15: branch s_180_14 b442 b181
        if s_180_14 {
            return block_442(state, tracer, fn_state);
        } else {
            return block_181(state, tracer, fn_state);
        };
    }
    fn block_181<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_181_0: const #0u : u8
        let s_181_0: bool = false;
        // D s_181_1: write-var gs#374071 <= s_181_0
        fn_state.gs_374071 = s_181_0;
        // N s_181_2: jump b182
        return block_182(state, tracer, fn_state);
    }
    fn block_182<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_182_0: read-var gs#374071:u8
        let s_182_0: bool = fn_state.gs_374071;
        // N s_182_1: branch s_182_0 b441 b183
        if s_182_0 {
            return block_441(state, tracer, fn_state);
        } else {
            return block_183(state, tracer, fn_state);
        };
    }
    fn block_183<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_183_0: const #0u : u8
        let s_183_0: bool = false;
        // D s_183_1: write-var gs#374073 <= s_183_0
        fn_state.gs_374073 = s_183_0;
        // N s_183_2: jump b184
        return block_184(state, tracer, fn_state);
    }
    fn block_184<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_184_0: read-var gs#374073:u8
        let s_184_0: bool = fn_state.gs_374073;
        // D s_184_1: not s_184_0
        let s_184_1: bool = !s_184_0;
        // N s_184_2: branch s_184_1 b186 b185
        if s_184_1 {
            return block_186(state, tracer, fn_state);
        } else {
            return block_185(state, tracer, fn_state);
        };
    }
    fn block_185<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_185_0: const #124s : i
        let s_185_0: i128 = 124;
        // C s_185_1: const #14696u : u32
        let s_185_1: u32 = 14696;
        // N s_185_2: write-reg s_185_1 <= s_185_0
        let s_185_2: () = {
            state.write_register::<i128>(s_185_1 as isize, s_185_0);
            tracer.write_register(s_185_1 as isize, s_185_0);
        };
        // C s_185_3: const #8s : i
        let s_185_3: i128 = 8;
        // C s_185_4: const #4s : i
        let s_185_4: i128 = 4;
        // D s_185_5: read-var u#23191:u32
        let s_185_5: u32 = fn_state.u_23191;
        // D s_185_6: cast zx s_185_5 -> bv
        let s_185_6: Bits = Bits::new(s_185_5 as u128, 32u16);
        // D s_185_7: bit-extract s_185_6 s_185_3 s_185_4
        let s_185_7: Bits = (Bits::new(
            ((s_185_6) >> (s_185_3)).value(),
            u16::try_from(s_185_4).unwrap(),
        ));
        // D s_185_8: cast reint s_185_7 -> u8
        let s_185_8: u8 = (s_185_7.value() as u8);
        // D s_185_9: call decode_clrex_aarch64_instrs_system_monitors(s_185_8)
        let s_185_9: () = decode_clrex_aarch64_instrs_system_monitors(
            state,
            tracer,
            s_185_8,
        );
        // N s_185_10: return
        return;
    }
    fn block_186<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_186_0: read-var merge#var.1:struct
        let s_186_0: u32 = fn_state.merge_var._1;
        // D s_186_1: write-var u#23194 <= s_186_0
        fn_state.u_23194 = s_186_0;
        // C s_186_2: const #21s : i
        let s_186_2: i128 = 21;
        // D s_186_3: read-var u#23194:u32
        let s_186_3: u32 = fn_state.u_23194;
        // D s_186_4: cast zx s_186_3 -> bv
        let s_186_4: Bits = Bits::new(s_186_3 as u128, 32u16);
        // C s_186_5: const #1s : i64
        let s_186_5: i64 = 1;
        // C s_186_6: cast zx s_186_5 -> i
        let s_186_6: i128 = (i128::try_from(s_186_5).unwrap());
        // C s_186_7: const #10s : i
        let s_186_7: i128 = 10;
        // C s_186_8: add s_186_7 s_186_6
        let s_186_8: i128 = (s_186_7 + s_186_6);
        // D s_186_9: bit-extract s_186_4 s_186_2 s_186_8
        let s_186_9: Bits = (Bits::new(
            ((s_186_4) >> (s_186_2)).value(),
            u16::try_from(s_186_8).unwrap(),
        ));
        // D s_186_10: cast reint s_186_9 -> u11
        let s_186_10: u16 = (s_186_9.value() as u16);
        // D s_186_11: cast zx s_186_10 -> bv
        let s_186_11: Bits = Bits::new(s_186_10 as u128, 11u16);
        // C s_186_12: const #1701u : u11
        let s_186_12: u16 = 1701;
        // C s_186_13: cast zx s_186_12 -> bv
        let s_186_13: Bits = Bits::new(s_186_12 as u128, 11u16);
        // D s_186_14: cmp-eq s_186_11 s_186_13
        let s_186_14: bool = ((s_186_11) == (s_186_13));
        // N s_186_15: branch s_186_14 b440 b187
        if s_186_14 {
            return block_440(state, tracer, fn_state);
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
        // D s_187_1: write-var gs#374083 <= s_187_0
        fn_state.gs_374083 = s_187_0;
        // N s_187_2: jump b188
        return block_188(state, tracer, fn_state);
    }
    fn block_188<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_188_0: read-var gs#374083:u8
        let s_188_0: bool = fn_state.gs_374083;
        // N s_188_1: branch s_188_0 b439 b189
        if s_188_0 {
            return block_439(state, tracer, fn_state);
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
        // D s_189_1: write-var gs#374085 <= s_189_0
        fn_state.gs_374085 = s_189_0;
        // N s_189_2: jump b190
        return block_190(state, tracer, fn_state);
    }
    fn block_190<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_190_0: read-var gs#374085:u8
        let s_190_0: bool = fn_state.gs_374085;
        // D s_190_1: not s_190_0
        let s_190_1: bool = !s_190_0;
        // N s_190_2: branch s_190_1 b192 b191
        if s_190_1 {
            return block_192(state, tracer, fn_state);
        } else {
            return block_191(state, tracer, fn_state);
        };
    }
    fn block_191<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_191_0: const #192s : i
        let s_191_0: i128 = 192;
        // C s_191_1: const #14696u : u32
        let s_191_1: u32 = 14696;
        // N s_191_2: write-reg s_191_1 <= s_191_0
        let s_191_2: () = {
            state.write_register::<i128>(s_191_1 as isize, s_191_0);
            tracer.write_register(s_191_1 as isize, s_191_0);
        };
        // C s_191_3: const #0s : i
        let s_191_3: i128 = 0;
        // C s_191_4: const #2s : i
        let s_191_4: i128 = 2;
        // D s_191_5: read-var u#23194:u32
        let s_191_5: u32 = fn_state.u_23194;
        // D s_191_6: cast zx s_191_5 -> bv
        let s_191_6: Bits = Bits::new(s_191_5 as u128, 32u16);
        // D s_191_7: bit-extract s_191_6 s_191_3 s_191_4
        let s_191_7: Bits = (Bits::new(
            ((s_191_6) >> (s_191_3)).value(),
            u16::try_from(s_191_4).unwrap(),
        ));
        // D s_191_8: cast reint s_191_7 -> u8
        let s_191_8: u8 = (s_191_7.value() as u8);
        // C s_191_9: const #5s : i
        let s_191_9: i128 = 5;
        // C s_191_10: const #16s : i
        let s_191_10: i128 = 16;
        // D s_191_11: read-var u#23194:u32
        let s_191_11: u32 = fn_state.u_23194;
        // D s_191_12: cast zx s_191_11 -> bv
        let s_191_12: Bits = Bits::new(s_191_11 as u128, 32u16);
        // D s_191_13: bit-extract s_191_12 s_191_9 s_191_10
        let s_191_13: Bits = (Bits::new(
            ((s_191_12) >> (s_191_9)).value(),
            u16::try_from(s_191_10).unwrap(),
        ));
        // D s_191_14: cast reint s_191_13 -> u16
        let s_191_14: u16 = (s_191_13.value() as u16);
        // D s_191_15: call decode_dcps1_aarch64_instrs_system_exceptions_debug_exception(s_191_8, s_191_14)
        let s_191_15: () = decode_dcps1_aarch64_instrs_system_exceptions_debug_exception(
            state,
            tracer,
            s_191_8,
            s_191_14,
        );
        // N s_191_16: return
        return;
    }
    fn block_192<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_192_0: read-var merge#var.1:struct
        let s_192_0: u32 = fn_state.merge_var._1;
        // D s_192_1: write-var u#23197 <= s_192_0
        fn_state.u_23197 = s_192_0;
        // C s_192_2: const #21s : i
        let s_192_2: i128 = 21;
        // D s_192_3: read-var u#23197:u32
        let s_192_3: u32 = fn_state.u_23197;
        // D s_192_4: cast zx s_192_3 -> bv
        let s_192_4: Bits = Bits::new(s_192_3 as u128, 32u16);
        // C s_192_5: const #1s : i64
        let s_192_5: i64 = 1;
        // C s_192_6: cast zx s_192_5 -> i
        let s_192_6: i128 = (i128::try_from(s_192_5).unwrap());
        // C s_192_7: const #10s : i
        let s_192_7: i128 = 10;
        // C s_192_8: add s_192_7 s_192_6
        let s_192_8: i128 = (s_192_7 + s_192_6);
        // D s_192_9: bit-extract s_192_4 s_192_2 s_192_8
        let s_192_9: Bits = (Bits::new(
            ((s_192_4) >> (s_192_2)).value(),
            u16::try_from(s_192_8).unwrap(),
        ));
        // D s_192_10: cast reint s_192_9 -> u11
        let s_192_10: u16 = (s_192_9.value() as u16);
        // D s_192_11: cast zx s_192_10 -> bv
        let s_192_11: Bits = Bits::new(s_192_10 as u128, 11u16);
        // C s_192_12: const #1701u : u11
        let s_192_12: u16 = 1701;
        // C s_192_13: cast zx s_192_12 -> bv
        let s_192_13: Bits = Bits::new(s_192_12 as u128, 11u16);
        // D s_192_14: cmp-eq s_192_11 s_192_13
        let s_192_14: bool = ((s_192_11) == (s_192_13));
        // N s_192_15: branch s_192_14 b438 b193
        if s_192_14 {
            return block_438(state, tracer, fn_state);
        } else {
            return block_193(state, tracer, fn_state);
        };
    }
    fn block_193<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_193_0: const #0u : u8
        let s_193_0: bool = false;
        // D s_193_1: write-var gs#374097 <= s_193_0
        fn_state.gs_374097 = s_193_0;
        // N s_193_2: jump b194
        return block_194(state, tracer, fn_state);
    }
    fn block_194<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_194_0: read-var gs#374097:u8
        let s_194_0: bool = fn_state.gs_374097;
        // N s_194_1: branch s_194_0 b437 b195
        if s_194_0 {
            return block_437(state, tracer, fn_state);
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
        // D s_195_1: write-var gs#374099 <= s_195_0
        fn_state.gs_374099 = s_195_0;
        // N s_195_2: jump b196
        return block_196(state, tracer, fn_state);
    }
    fn block_196<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_196_0: read-var gs#374099:u8
        let s_196_0: bool = fn_state.gs_374099;
        // D s_196_1: not s_196_0
        let s_196_1: bool = !s_196_0;
        // N s_196_2: branch s_196_1 b198 b197
        if s_196_1 {
            return block_198(state, tracer, fn_state);
        } else {
            return block_197(state, tracer, fn_state);
        };
    }
    fn block_197<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_197_0: const #193s : i
        let s_197_0: i128 = 193;
        // C s_197_1: const #14696u : u32
        let s_197_1: u32 = 14696;
        // N s_197_2: write-reg s_197_1 <= s_197_0
        let s_197_2: () = {
            state.write_register::<i128>(s_197_1 as isize, s_197_0);
            tracer.write_register(s_197_1 as isize, s_197_0);
        };
        // C s_197_3: const #0s : i
        let s_197_3: i128 = 0;
        // C s_197_4: const #2s : i
        let s_197_4: i128 = 2;
        // D s_197_5: read-var u#23197:u32
        let s_197_5: u32 = fn_state.u_23197;
        // D s_197_6: cast zx s_197_5 -> bv
        let s_197_6: Bits = Bits::new(s_197_5 as u128, 32u16);
        // D s_197_7: bit-extract s_197_6 s_197_3 s_197_4
        let s_197_7: Bits = (Bits::new(
            ((s_197_6) >> (s_197_3)).value(),
            u16::try_from(s_197_4).unwrap(),
        ));
        // D s_197_8: cast reint s_197_7 -> u8
        let s_197_8: u8 = (s_197_7.value() as u8);
        // C s_197_9: const #5s : i
        let s_197_9: i128 = 5;
        // C s_197_10: const #16s : i
        let s_197_10: i128 = 16;
        // D s_197_11: read-var u#23197:u32
        let s_197_11: u32 = fn_state.u_23197;
        // D s_197_12: cast zx s_197_11 -> bv
        let s_197_12: Bits = Bits::new(s_197_11 as u128, 32u16);
        // D s_197_13: bit-extract s_197_12 s_197_9 s_197_10
        let s_197_13: Bits = (Bits::new(
            ((s_197_12) >> (s_197_9)).value(),
            u16::try_from(s_197_10).unwrap(),
        ));
        // D s_197_14: cast reint s_197_13 -> u16
        let s_197_14: u16 = (s_197_13.value() as u16);
        // D s_197_15: call decode_dcps2_aarch64_instrs_system_exceptions_debug_exception(s_197_8, s_197_14)
        let s_197_15: () = decode_dcps2_aarch64_instrs_system_exceptions_debug_exception(
            state,
            tracer,
            s_197_8,
            s_197_14,
        );
        // N s_197_16: return
        return;
    }
    fn block_198<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_198_0: read-var merge#var.1:struct
        let s_198_0: u32 = fn_state.merge_var._1;
        // D s_198_1: write-var u#23201 <= s_198_0
        fn_state.u_23201 = s_198_0;
        // C s_198_2: const #21s : i
        let s_198_2: i128 = 21;
        // D s_198_3: read-var u#23201:u32
        let s_198_3: u32 = fn_state.u_23201;
        // D s_198_4: cast zx s_198_3 -> bv
        let s_198_4: Bits = Bits::new(s_198_3 as u128, 32u16);
        // C s_198_5: const #1s : i64
        let s_198_5: i64 = 1;
        // C s_198_6: cast zx s_198_5 -> i
        let s_198_6: i128 = (i128::try_from(s_198_5).unwrap());
        // C s_198_7: const #10s : i
        let s_198_7: i128 = 10;
        // C s_198_8: add s_198_7 s_198_6
        let s_198_8: i128 = (s_198_7 + s_198_6);
        // D s_198_9: bit-extract s_198_4 s_198_2 s_198_8
        let s_198_9: Bits = (Bits::new(
            ((s_198_4) >> (s_198_2)).value(),
            u16::try_from(s_198_8).unwrap(),
        ));
        // D s_198_10: cast reint s_198_9 -> u11
        let s_198_10: u16 = (s_198_9.value() as u16);
        // D s_198_11: cast zx s_198_10 -> bv
        let s_198_11: Bits = Bits::new(s_198_10 as u128, 11u16);
        // C s_198_12: const #1701u : u11
        let s_198_12: u16 = 1701;
        // C s_198_13: cast zx s_198_12 -> bv
        let s_198_13: Bits = Bits::new(s_198_12 as u128, 11u16);
        // D s_198_14: cmp-eq s_198_11 s_198_13
        let s_198_14: bool = ((s_198_11) == (s_198_13));
        // N s_198_15: branch s_198_14 b436 b199
        if s_198_14 {
            return block_436(state, tracer, fn_state);
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
        // D s_199_1: write-var gs#374111 <= s_199_0
        fn_state.gs_374111 = s_199_0;
        // N s_199_2: jump b200
        return block_200(state, tracer, fn_state);
    }
    fn block_200<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_200_0: read-var gs#374111:u8
        let s_200_0: bool = fn_state.gs_374111;
        // N s_200_1: branch s_200_0 b435 b201
        if s_200_0 {
            return block_435(state, tracer, fn_state);
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
        // D s_201_1: write-var gs#374113 <= s_201_0
        fn_state.gs_374113 = s_201_0;
        // N s_201_2: jump b202
        return block_202(state, tracer, fn_state);
    }
    fn block_202<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_202_0: read-var gs#374113:u8
        let s_202_0: bool = fn_state.gs_374113;
        // D s_202_1: not s_202_0
        let s_202_1: bool = !s_202_0;
        // N s_202_2: branch s_202_1 b204 b203
        if s_202_1 {
            return block_204(state, tracer, fn_state);
        } else {
            return block_203(state, tracer, fn_state);
        };
    }
    fn block_203<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_203_0: const #194s : i
        let s_203_0: i128 = 194;
        // C s_203_1: const #14696u : u32
        let s_203_1: u32 = 14696;
        // N s_203_2: write-reg s_203_1 <= s_203_0
        let s_203_2: () = {
            state.write_register::<i128>(s_203_1 as isize, s_203_0);
            tracer.write_register(s_203_1 as isize, s_203_0);
        };
        // C s_203_3: const #0s : i
        let s_203_3: i128 = 0;
        // C s_203_4: const #2s : i
        let s_203_4: i128 = 2;
        // D s_203_5: read-var u#23201:u32
        let s_203_5: u32 = fn_state.u_23201;
        // D s_203_6: cast zx s_203_5 -> bv
        let s_203_6: Bits = Bits::new(s_203_5 as u128, 32u16);
        // D s_203_7: bit-extract s_203_6 s_203_3 s_203_4
        let s_203_7: Bits = (Bits::new(
            ((s_203_6) >> (s_203_3)).value(),
            u16::try_from(s_203_4).unwrap(),
        ));
        // D s_203_8: cast reint s_203_7 -> u8
        let s_203_8: u8 = (s_203_7.value() as u8);
        // C s_203_9: const #5s : i
        let s_203_9: i128 = 5;
        // C s_203_10: const #16s : i
        let s_203_10: i128 = 16;
        // D s_203_11: read-var u#23201:u32
        let s_203_11: u32 = fn_state.u_23201;
        // D s_203_12: cast zx s_203_11 -> bv
        let s_203_12: Bits = Bits::new(s_203_11 as u128, 32u16);
        // D s_203_13: bit-extract s_203_12 s_203_9 s_203_10
        let s_203_13: Bits = (Bits::new(
            ((s_203_12) >> (s_203_9)).value(),
            u16::try_from(s_203_10).unwrap(),
        ));
        // D s_203_14: cast reint s_203_13 -> u16
        let s_203_14: u16 = (s_203_13.value() as u16);
        // D s_203_15: call decode_dcps3_aarch64_instrs_system_exceptions_debug_exception(s_203_8, s_203_14)
        let s_203_15: () = decode_dcps3_aarch64_instrs_system_exceptions_debug_exception(
            state,
            tracer,
            s_203_8,
            s_203_14,
        );
        // N s_203_16: return
        return;
    }
    fn block_204<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_204_0: read-var merge#var.1:struct
        let s_204_0: u32 = fn_state.merge_var._1;
        // D s_204_1: write-var u#23205 <= s_204_0
        fn_state.u_23205 = s_204_0;
        // C s_204_2: const #12s : i
        let s_204_2: i128 = 12;
        // D s_204_3: read-var u#23205:u32
        let s_204_3: u32 = fn_state.u_23205;
        // D s_204_4: cast zx s_204_3 -> bv
        let s_204_4: Bits = Bits::new(s_204_3 as u128, 32u16);
        // C s_204_5: const #1s : i64
        let s_204_5: i64 = 1;
        // C s_204_6: cast zx s_204_5 -> i
        let s_204_6: i128 = (i128::try_from(s_204_5).unwrap());
        // C s_204_7: const #19s : i
        let s_204_7: i128 = 19;
        // C s_204_8: add s_204_7 s_204_6
        let s_204_8: i128 = (s_204_7 + s_204_6);
        // D s_204_9: bit-extract s_204_4 s_204_2 s_204_8
        let s_204_9: Bits = (Bits::new(
            ((s_204_4) >> (s_204_2)).value(),
            u16::try_from(s_204_8).unwrap(),
        ));
        // D s_204_10: cast reint s_204_9 -> u20
        let s_204_10: u32 = (s_204_9.value() as u32);
        // D s_204_11: cast zx s_204_10 -> bv
        let s_204_11: Bits = Bits::new(s_204_10 as u128, 20u16);
        // C s_204_12: const #872499u : u20
        let s_204_12: u32 = 872499;
        // C s_204_13: cast zx s_204_12 -> bv
        let s_204_13: Bits = Bits::new(s_204_12 as u128, 20u16);
        // D s_204_14: cmp-eq s_204_11 s_204_13
        let s_204_14: bool = ((s_204_11) == (s_204_13));
        // N s_204_15: branch s_204_14 b434 b205
        if s_204_14 {
            return block_434(state, tracer, fn_state);
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
        // D s_205_1: write-var gs#374125 <= s_205_0
        fn_state.gs_374125 = s_205_0;
        // N s_205_2: jump b206
        return block_206(state, tracer, fn_state);
    }
    fn block_206<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_206_0: read-var gs#374125:u8
        let s_206_0: bool = fn_state.gs_374125;
        // N s_206_1: branch s_206_0 b433 b207
        if s_206_0 {
            return block_433(state, tracer, fn_state);
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
        // D s_207_1: write-var gs#374127 <= s_207_0
        fn_state.gs_374127 = s_207_0;
        // N s_207_2: jump b208
        return block_208(state, tracer, fn_state);
    }
    fn block_208<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_208_0: read-var gs#374127:u8
        let s_208_0: bool = fn_state.gs_374127;
        // D s_208_1: not s_208_0
        let s_208_1: bool = !s_208_0;
        // N s_208_2: branch s_208_1 b210 b209
        if s_208_1 {
            return block_210(state, tracer, fn_state);
        } else {
            return block_209(state, tracer, fn_state);
        };
    }
    fn block_209<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_209_0: const #195s : i
        let s_209_0: i128 = 195;
        // C s_209_1: const #14696u : u32
        let s_209_1: u32 = 14696;
        // N s_209_2: write-reg s_209_1 <= s_209_0
        let s_209_2: () = {
            state.write_register::<i128>(s_209_1 as isize, s_209_0);
            tracer.write_register(s_209_1 as isize, s_209_0);
        };
        // C s_209_3: const #5s : i
        let s_209_3: i128 = 5;
        // C s_209_4: const #2s : i
        let s_209_4: i128 = 2;
        // D s_209_5: read-var u#23205:u32
        let s_209_5: u32 = fn_state.u_23205;
        // D s_209_6: cast zx s_209_5 -> bv
        let s_209_6: Bits = Bits::new(s_209_5 as u128, 32u16);
        // D s_209_7: bit-extract s_209_6 s_209_3 s_209_4
        let s_209_7: Bits = (Bits::new(
            ((s_209_6) >> (s_209_3)).value(),
            u16::try_from(s_209_4).unwrap(),
        ));
        // D s_209_8: cast reint s_209_7 -> u8
        let s_209_8: u8 = (s_209_7.value() as u8);
        // C s_209_9: const #8s : i
        let s_209_9: i128 = 8;
        // C s_209_10: const #4s : i
        let s_209_10: i128 = 4;
        // D s_209_11: read-var u#23205:u32
        let s_209_11: u32 = fn_state.u_23205;
        // D s_209_12: cast zx s_209_11 -> bv
        let s_209_12: Bits = Bits::new(s_209_11 as u128, 32u16);
        // D s_209_13: bit-extract s_209_12 s_209_9 s_209_10
        let s_209_13: Bits = (Bits::new(
            ((s_209_12) >> (s_209_9)).value(),
            u16::try_from(s_209_10).unwrap(),
        ));
        // D s_209_14: cast reint s_209_13 -> u8
        let s_209_14: u8 = (s_209_13.value() as u8);
        // D s_209_15: call decode_dmb_aarch64_instrs_system_barriers_dmb(s_209_8, s_209_14)
        let s_209_15: () = decode_dmb_aarch64_instrs_system_barriers_dmb(
            state,
            tracer,
            s_209_8,
            s_209_14,
        );
        // N s_209_16: return
        return;
    }
    fn block_210<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_210_0: read-var merge#var.1:struct
        let s_210_0: u32 = fn_state.merge_var._1;
        // D s_210_1: cast zx s_210_0 -> bv
        let s_210_1: Bits = Bits::new(s_210_0 as u128, 32u16);
        // C s_210_2: const #3602842592u : u32
        let s_210_2: u32 = 3602842592;
        // C s_210_3: cast zx s_210_2 -> bv
        let s_210_3: Bits = Bits::new(s_210_2 as u128, 32u16);
        // D s_210_4: cmp-eq s_210_1 s_210_3
        let s_210_4: bool = ((s_210_1) == (s_210_3));
        // N s_210_5: branch s_210_4 b432 b211
        if s_210_4 {
            return block_432(state, tracer, fn_state);
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
        // D s_211_1: write-var gs#374136 <= s_211_0
        fn_state.gs_374136 = s_211_0;
        // N s_211_2: jump b212
        return block_212(state, tracer, fn_state);
    }
    fn block_212<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_212_0: read-var gs#374136:u8
        let s_212_0: bool = fn_state.gs_374136;
        // D s_212_1: not s_212_0
        let s_212_1: bool = !s_212_0;
        // N s_212_2: branch s_212_1 b214 b213
        if s_212_1 {
            return block_214(state, tracer, fn_state);
        } else {
            return block_213(state, tracer, fn_state);
        };
    }
    fn block_213<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_213_0: const #196s : i
        let s_213_0: i128 = 196;
        // C s_213_1: const #14696u : u32
        let s_213_1: u32 = 14696;
        // N s_213_2: write-reg s_213_1 <= s_213_0
        let s_213_2: () = {
            state.write_register::<i128>(s_213_1 as isize, s_213_0);
            tracer.write_register(s_213_1 as isize, s_213_0);
        };
        // C s_213_3: const #() : ()
        let s_213_3: () = ();
        // S s_213_4: call decode_drps_aarch64_instrs_branch_unconditional_dret(s_213_3)
        let s_213_4: () = decode_drps_aarch64_instrs_branch_unconditional_dret(
            state,
            tracer,
            s_213_3,
        );
        // N s_213_5: return
        return;
    }
    fn block_214<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_214_0: read-var merge#var.1:struct
        let s_214_0: u32 = fn_state.merge_var._1;
        // D s_214_1: write-var u#23210 <= s_214_0
        fn_state.u_23210 = s_214_0;
        // C s_214_2: const #12s : i
        let s_214_2: i128 = 12;
        // D s_214_3: read-var u#23210:u32
        let s_214_3: u32 = fn_state.u_23210;
        // D s_214_4: cast zx s_214_3 -> bv
        let s_214_4: Bits = Bits::new(s_214_3 as u128, 32u16);
        // C s_214_5: const #1s : i64
        let s_214_5: i64 = 1;
        // C s_214_6: cast zx s_214_5 -> i
        let s_214_6: i128 = (i128::try_from(s_214_5).unwrap());
        // C s_214_7: const #19s : i
        let s_214_7: i128 = 19;
        // C s_214_8: add s_214_7 s_214_6
        let s_214_8: i128 = (s_214_7 + s_214_6);
        // D s_214_9: bit-extract s_214_4 s_214_2 s_214_8
        let s_214_9: Bits = (Bits::new(
            ((s_214_4) >> (s_214_2)).value(),
            u16::try_from(s_214_8).unwrap(),
        ));
        // D s_214_10: cast reint s_214_9 -> u20
        let s_214_10: u32 = (s_214_9.value() as u32);
        // D s_214_11: cast zx s_214_10 -> bv
        let s_214_11: Bits = Bits::new(s_214_10 as u128, 20u16);
        // C s_214_12: const #872499u : u20
        let s_214_12: u32 = 872499;
        // C s_214_13: cast zx s_214_12 -> bv
        let s_214_13: Bits = Bits::new(s_214_12 as u128, 20u16);
        // D s_214_14: cmp-eq s_214_11 s_214_13
        let s_214_14: bool = ((s_214_11) == (s_214_13));
        // N s_214_15: branch s_214_14 b431 b215
        if s_214_14 {
            return block_431(state, tracer, fn_state);
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
        // D s_215_1: write-var gs#374144 <= s_215_0
        fn_state.gs_374144 = s_215_0;
        // N s_215_2: jump b216
        return block_216(state, tracer, fn_state);
    }
    fn block_216<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_216_0: read-var gs#374144:u8
        let s_216_0: bool = fn_state.gs_374144;
        // N s_216_1: branch s_216_0 b430 b217
        if s_216_0 {
            return block_430(state, tracer, fn_state);
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
        // D s_217_1: write-var gs#374146 <= s_217_0
        fn_state.gs_374146 = s_217_0;
        // N s_217_2: jump b218
        return block_218(state, tracer, fn_state);
    }
    fn block_218<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_218_0: read-var gs#374146:u8
        let s_218_0: bool = fn_state.gs_374146;
        // D s_218_1: not s_218_0
        let s_218_1: bool = !s_218_0;
        // N s_218_2: branch s_218_1 b220 b219
        if s_218_1 {
            return block_220(state, tracer, fn_state);
        } else {
            return block_219(state, tracer, fn_state);
        };
    }
    fn block_219<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_219_0: const #197s : i
        let s_219_0: i128 = 197;
        // C s_219_1: const #14696u : u32
        let s_219_1: u32 = 14696;
        // N s_219_2: write-reg s_219_1 <= s_219_0
        let s_219_2: () = {
            state.write_register::<i128>(s_219_1 as isize, s_219_0);
            tracer.write_register(s_219_1 as isize, s_219_0);
        };
        // C s_219_3: const #5s : i
        let s_219_3: i128 = 5;
        // C s_219_4: const #2s : i
        let s_219_4: i128 = 2;
        // D s_219_5: read-var u#23210:u32
        let s_219_5: u32 = fn_state.u_23210;
        // D s_219_6: cast zx s_219_5 -> bv
        let s_219_6: Bits = Bits::new(s_219_5 as u128, 32u16);
        // D s_219_7: bit-extract s_219_6 s_219_3 s_219_4
        let s_219_7: Bits = (Bits::new(
            ((s_219_6) >> (s_219_3)).value(),
            u16::try_from(s_219_4).unwrap(),
        ));
        // D s_219_8: cast reint s_219_7 -> u8
        let s_219_8: u8 = (s_219_7.value() as u8);
        // C s_219_9: const #8s : i
        let s_219_9: i128 = 8;
        // C s_219_10: const #4s : i
        let s_219_10: i128 = 4;
        // D s_219_11: read-var u#23210:u32
        let s_219_11: u32 = fn_state.u_23210;
        // D s_219_12: cast zx s_219_11 -> bv
        let s_219_12: Bits = Bits::new(s_219_11 as u128, 32u16);
        // D s_219_13: bit-extract s_219_12 s_219_9 s_219_10
        let s_219_13: Bits = (Bits::new(
            ((s_219_12) >> (s_219_9)).value(),
            u16::try_from(s_219_10).unwrap(),
        ));
        // D s_219_14: cast reint s_219_13 -> u8
        let s_219_14: u8 = (s_219_13.value() as u8);
        // D s_219_15: call decode_dsb_aarch64_instrs_system_barriers_dsb(s_219_8, s_219_14)
        let s_219_15: () = decode_dsb_aarch64_instrs_system_barriers_dsb(
            state,
            tracer,
            s_219_8,
            s_219_14,
        );
        // N s_219_16: return
        return;
    }
    fn block_220<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_220_0: read-var merge#var.1:struct
        let s_220_0: u32 = fn_state.merge_var._1;
        // D s_220_1: write-var u#23214 <= s_220_0
        fn_state.u_23214 = s_220_0;
        // C s_220_2: const #12s : i
        let s_220_2: i128 = 12;
        // D s_220_3: read-var u#23214:u32
        let s_220_3: u32 = fn_state.u_23214;
        // D s_220_4: cast zx s_220_3 -> bv
        let s_220_4: Bits = Bits::new(s_220_3 as u128, 32u16);
        // C s_220_5: const #1s : i64
        let s_220_5: i64 = 1;
        // C s_220_6: cast zx s_220_5 -> i
        let s_220_6: i128 = (i128::try_from(s_220_5).unwrap());
        // C s_220_7: const #19s : i
        let s_220_7: i128 = 19;
        // C s_220_8: add s_220_7 s_220_6
        let s_220_8: i128 = (s_220_7 + s_220_6);
        // D s_220_9: bit-extract s_220_4 s_220_2 s_220_8
        let s_220_9: Bits = (Bits::new(
            ((s_220_4) >> (s_220_2)).value(),
            u16::try_from(s_220_8).unwrap(),
        ));
        // D s_220_10: cast reint s_220_9 -> u20
        let s_220_10: u32 = (s_220_9.value() as u32);
        // D s_220_11: cast zx s_220_10 -> bv
        let s_220_11: Bits = Bits::new(s_220_10 as u128, 20u16);
        // C s_220_12: const #872499u : u20
        let s_220_12: u32 = 872499;
        // C s_220_13: cast zx s_220_12 -> bv
        let s_220_13: Bits = Bits::new(s_220_12 as u128, 20u16);
        // D s_220_14: cmp-eq s_220_11 s_220_13
        let s_220_14: bool = ((s_220_11) == (s_220_13));
        // N s_220_15: branch s_220_14 b429 b221
        if s_220_14 {
            return block_429(state, tracer, fn_state);
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
        // D s_221_1: write-var gs#374158 <= s_221_0
        fn_state.gs_374158 = s_221_0;
        // N s_221_2: jump b222
        return block_222(state, tracer, fn_state);
    }
    fn block_222<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_222_0: read-var gs#374158:u8
        let s_222_0: bool = fn_state.gs_374158;
        // N s_222_1: branch s_222_0 b428 b223
        if s_222_0 {
            return block_428(state, tracer, fn_state);
        } else {
            return block_223(state, tracer, fn_state);
        };
    }
    fn block_223<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_223_0: const #0u : u8
        let s_223_0: bool = false;
        // D s_223_1: write-var gs#374160 <= s_223_0
        fn_state.gs_374160 = s_223_0;
        // N s_223_2: jump b224
        return block_224(state, tracer, fn_state);
    }
    fn block_224<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_224_0: read-var gs#374160:u8
        let s_224_0: bool = fn_state.gs_374160;
        // D s_224_1: not s_224_0
        let s_224_1: bool = !s_224_0;
        // N s_224_2: branch s_224_1 b226 b225
        if s_224_1 {
            return block_226(state, tracer, fn_state);
        } else {
            return block_225(state, tracer, fn_state);
        };
    }
    fn block_225<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_225_0: const #198s : i
        let s_225_0: i128 = 198;
        // C s_225_1: const #14696u : u32
        let s_225_1: u32 = 14696;
        // N s_225_2: write-reg s_225_1 <= s_225_0
        let s_225_2: () = {
            state.write_register::<i128>(s_225_1 as isize, s_225_0);
            tracer.write_register(s_225_1 as isize, s_225_0);
        };
        // C s_225_3: const #10s : i
        let s_225_3: i128 = 10;
        // C s_225_4: const #2s : i
        let s_225_4: i128 = 2;
        // D s_225_5: read-var u#23214:u32
        let s_225_5: u32 = fn_state.u_23214;
        // D s_225_6: cast zx s_225_5 -> bv
        let s_225_6: Bits = Bits::new(s_225_5 as u128, 32u16);
        // D s_225_7: bit-extract s_225_6 s_225_3 s_225_4
        let s_225_7: Bits = (Bits::new(
            ((s_225_6) >> (s_225_3)).value(),
            u16::try_from(s_225_4).unwrap(),
        ));
        // D s_225_8: cast reint s_225_7 -> u8
        let s_225_8: u8 = (s_225_7.value() as u8);
        // D s_225_9: call decode_dsb_aarch64_instrs_system_barriers_dsb_nxs(s_225_8)
        let s_225_9: () = decode_dsb_aarch64_instrs_system_barriers_dsb_nxs(
            state,
            tracer,
            s_225_8,
        );
        // N s_225_10: return
        return;
    }
    fn block_226<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_226_0: read-var merge#var.1:struct
        let s_226_0: u32 = fn_state.merge_var._1;
        // D s_226_1: write-var u#23216 <= s_226_0
        fn_state.u_23216 = s_226_0;
        // D s_226_2: read-var u#23216:u32
        let s_226_2: u32 = fn_state.u_23216;
        // D s_226_3: cast zx s_226_2 -> bv
        let s_226_3: Bits = Bits::new(s_226_2 as u128, 32u16);
        // C s_226_4: const #3600745440u : u32
        let s_226_4: u32 = 3600745440;
        // C s_226_5: cast zx s_226_4 -> bv
        let s_226_5: Bits = Bits::new(s_226_4 as u128, 32u16);
        // D s_226_6: cmp-eq s_226_3 s_226_5
        let s_226_6: bool = ((s_226_3) == (s_226_5));
        // N s_226_7: branch s_226_6 b427 b227
        if s_226_6 {
            return block_427(state, tracer, fn_state);
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
        // D s_227_1: write-var gs#374167 <= s_227_0
        fn_state.gs_374167 = s_227_0;
        // N s_227_2: jump b228
        return block_228(state, tracer, fn_state);
    }
    fn block_228<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_228_0: read-var gs#374167:u8
        let s_228_0: bool = fn_state.gs_374167;
        // D s_228_1: not s_228_0
        let s_228_1: bool = !s_228_0;
        // N s_228_2: branch s_228_1 b230 b229
        if s_228_1 {
            return block_230(state, tracer, fn_state);
        } else {
            return block_229(state, tracer, fn_state);
        };
    }
    fn block_229<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_229_0: const #203s : i
        let s_229_0: i128 = 203;
        // C s_229_1: const #14696u : u32
        let s_229_1: u32 = 14696;
        // N s_229_2: write-reg s_229_1 <= s_229_0
        let s_229_2: () = {
            state.write_register::<i128>(s_229_1 as isize, s_229_0);
            tracer.write_register(s_229_1 as isize, s_229_0);
        };
        // C s_229_3: const #0s : i
        let s_229_3: i128 = 0;
        // C s_229_4: const #5s : i
        let s_229_4: i128 = 5;
        // D s_229_5: read-var u#23216:u32
        let s_229_5: u32 = fn_state.u_23216;
        // D s_229_6: cast zx s_229_5 -> bv
        let s_229_6: Bits = Bits::new(s_229_5 as u128, 32u16);
        // D s_229_7: bit-extract s_229_6 s_229_3 s_229_4
        let s_229_7: Bits = (Bits::new(
            ((s_229_6) >> (s_229_3)).value(),
            u16::try_from(s_229_4).unwrap(),
        ));
        // D s_229_8: cast reint s_229_7 -> u8
        let s_229_8: u8 = (s_229_7.value() as u8);
        // C s_229_9: const #5s : i
        let s_229_9: i128 = 5;
        // C s_229_10: const #5s : i
        let s_229_10: i128 = 5;
        // D s_229_11: read-var u#23216:u32
        let s_229_11: u32 = fn_state.u_23216;
        // D s_229_12: cast zx s_229_11 -> bv
        let s_229_12: Bits = Bits::new(s_229_11 as u128, 32u16);
        // D s_229_13: bit-extract s_229_12 s_229_9 s_229_10
        let s_229_13: Bits = (Bits::new(
            ((s_229_12) >> (s_229_9)).value(),
            u16::try_from(s_229_10).unwrap(),
        ));
        // D s_229_14: cast reint s_229_13 -> u8
        let s_229_14: u8 = (s_229_13.value() as u8);
        // C s_229_15: const #10s : i
        let s_229_15: i128 = 10;
        // C s_229_16: const #1s : i
        let s_229_16: i128 = 1;
        // D s_229_17: read-var u#23216:u32
        let s_229_17: u32 = fn_state.u_23216;
        // D s_229_18: cast zx s_229_17 -> bv
        let s_229_18: Bits = Bits::new(s_229_17 as u128, 32u16);
        // D s_229_19: bit-extract s_229_18 s_229_15 s_229_16
        let s_229_19: Bits = (Bits::new(
            ((s_229_18) >> (s_229_15)).value(),
            u16::try_from(s_229_16).unwrap(),
        ));
        // D s_229_20: cast reint s_229_19 -> u8
        let s_229_20: bool = ((s_229_19.value()) != 0);
        // C s_229_21: const #11s : i
        let s_229_21: i128 = 11;
        // C s_229_22: const #1s : i
        let s_229_22: i128 = 1;
        // D s_229_23: read-var u#23216:u32
        let s_229_23: u32 = fn_state.u_23216;
        // D s_229_24: cast zx s_229_23 -> bv
        let s_229_24: Bits = Bits::new(s_229_23 as u128, 32u16);
        // D s_229_25: bit-extract s_229_24 s_229_21 s_229_22
        let s_229_25: Bits = (Bits::new(
            ((s_229_24) >> (s_229_21)).value(),
            u16::try_from(s_229_22).unwrap(),
        ));
        // D s_229_26: cast reint s_229_25 -> u8
        let s_229_26: bool = ((s_229_25.value()) != 0);
        // D s_229_27: call decode_eret_aarch64_instrs_branch_unconditional_eret(s_229_8, s_229_14, s_229_20, s_229_26)
        let s_229_27: () = decode_eret_aarch64_instrs_branch_unconditional_eret(
            state,
            tracer,
            s_229_8,
            s_229_14,
            s_229_20,
            s_229_26,
        );
        // N s_229_28: return
        return;
    }
    fn block_230<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_230_0: read-var merge#var.1:struct
        let s_230_0: u32 = fn_state.merge_var._1;
        // D s_230_1: write-var u#23221 <= s_230_0
        fn_state.u_23221 = s_230_0;
        // C s_230_2: const #11s : i
        let s_230_2: i128 = 11;
        // D s_230_3: read-var u#23221:u32
        let s_230_3: u32 = fn_state.u_23221;
        // D s_230_4: cast zx s_230_3 -> bv
        let s_230_4: Bits = Bits::new(s_230_3 as u128, 32u16);
        // C s_230_5: const #1s : i64
        let s_230_5: i64 = 1;
        // C s_230_6: cast zx s_230_5 -> i
        let s_230_6: i128 = (i128::try_from(s_230_5).unwrap());
        // C s_230_7: const #20s : i
        let s_230_7: i128 = 20;
        // C s_230_8: add s_230_7 s_230_6
        let s_230_8: i128 = (s_230_7 + s_230_6);
        // D s_230_9: bit-extract s_230_4 s_230_2 s_230_8
        let s_230_9: Bits = (Bits::new(
            ((s_230_4) >> (s_230_2)).value(),
            u16::try_from(s_230_8).unwrap(),
        ));
        // D s_230_10: cast reint s_230_9 -> u21
        let s_230_10: u32 = (s_230_9.value() as u32);
        // D s_230_11: cast zx s_230_10 -> bv
        let s_230_11: Bits = Bits::new(s_230_10 as u128, 21u16);
        // C s_230_12: const #1758177u : u21
        let s_230_12: u32 = 1758177;
        // C s_230_13: cast zx s_230_12 -> bv
        let s_230_13: Bits = Bits::new(s_230_12 as u128, 21u16);
        // D s_230_14: cmp-eq s_230_11 s_230_13
        let s_230_14: bool = ((s_230_11) == (s_230_13));
        // N s_230_15: branch s_230_14 b426 b231
        if s_230_14 {
            return block_426(state, tracer, fn_state);
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
        // D s_231_1: write-var gs#374183 <= s_231_0
        fn_state.gs_374183 = s_231_0;
        // N s_231_2: jump b232
        return block_232(state, tracer, fn_state);
    }
    fn block_232<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_232_0: read-var gs#374183:u8
        let s_232_0: bool = fn_state.gs_374183;
        // N s_232_1: branch s_232_0 b425 b233
        if s_232_0 {
            return block_425(state, tracer, fn_state);
        } else {
            return block_233(state, tracer, fn_state);
        };
    }
    fn block_233<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_233_0: const #0u : u8
        let s_233_0: bool = false;
        // D s_233_1: write-var gs#374185 <= s_233_0
        fn_state.gs_374185 = s_233_0;
        // N s_233_2: jump b234
        return block_234(state, tracer, fn_state);
    }
    fn block_234<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_234_0: read-var gs#374185:u8
        let s_234_0: bool = fn_state.gs_374185;
        // D s_234_1: not s_234_0
        let s_234_1: bool = !s_234_0;
        // N s_234_2: branch s_234_1 b236 b235
        if s_234_1 {
            return block_236(state, tracer, fn_state);
        } else {
            return block_235(state, tracer, fn_state);
        };
    }
    fn block_235<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_235_0: const #204s : i
        let s_235_0: i128 = 204;
        // C s_235_1: const #14696u : u32
        let s_235_1: u32 = 14696;
        // N s_235_2: write-reg s_235_1 <= s_235_0
        let s_235_2: () = {
            state.write_register::<i128>(s_235_1 as isize, s_235_0);
            tracer.write_register(s_235_1 as isize, s_235_0);
        };
        // C s_235_3: const #0s : i
        let s_235_3: i128 = 0;
        // C s_235_4: const #5s : i
        let s_235_4: i128 = 5;
        // D s_235_5: read-var u#23221:u32
        let s_235_5: u32 = fn_state.u_23221;
        // D s_235_6: cast zx s_235_5 -> bv
        let s_235_6: Bits = Bits::new(s_235_5 as u128, 32u16);
        // D s_235_7: bit-extract s_235_6 s_235_3 s_235_4
        let s_235_7: Bits = (Bits::new(
            ((s_235_6) >> (s_235_3)).value(),
            u16::try_from(s_235_4).unwrap(),
        ));
        // D s_235_8: cast reint s_235_7 -> u8
        let s_235_8: u8 = (s_235_7.value() as u8);
        // C s_235_9: const #5s : i
        let s_235_9: i128 = 5;
        // C s_235_10: const #5s : i
        let s_235_10: i128 = 5;
        // D s_235_11: read-var u#23221:u32
        let s_235_11: u32 = fn_state.u_23221;
        // D s_235_12: cast zx s_235_11 -> bv
        let s_235_12: Bits = Bits::new(s_235_11 as u128, 32u16);
        // D s_235_13: bit-extract s_235_12 s_235_9 s_235_10
        let s_235_13: Bits = (Bits::new(
            ((s_235_12) >> (s_235_9)).value(),
            u16::try_from(s_235_10).unwrap(),
        ));
        // D s_235_14: cast reint s_235_13 -> u8
        let s_235_14: u8 = (s_235_13.value() as u8);
        // C s_235_15: const #10s : i
        let s_235_15: i128 = 10;
        // C s_235_16: const #1s : i
        let s_235_16: i128 = 1;
        // D s_235_17: read-var u#23221:u32
        let s_235_17: u32 = fn_state.u_23221;
        // D s_235_18: cast zx s_235_17 -> bv
        let s_235_18: Bits = Bits::new(s_235_17 as u128, 32u16);
        // D s_235_19: bit-extract s_235_18 s_235_15 s_235_16
        let s_235_19: Bits = (Bits::new(
            ((s_235_18) >> (s_235_15)).value(),
            u16::try_from(s_235_16).unwrap(),
        ));
        // D s_235_20: cast reint s_235_19 -> u8
        let s_235_20: bool = ((s_235_19.value()) != 0);
        // C s_235_21: const #11s : i
        let s_235_21: i128 = 11;
        // C s_235_22: const #1s : i
        let s_235_22: i128 = 1;
        // D s_235_23: read-var u#23221:u32
        let s_235_23: u32 = fn_state.u_23221;
        // D s_235_24: cast zx s_235_23 -> bv
        let s_235_24: Bits = Bits::new(s_235_23 as u128, 32u16);
        // D s_235_25: bit-extract s_235_24 s_235_21 s_235_22
        let s_235_25: Bits = (Bits::new(
            ((s_235_24) >> (s_235_21)).value(),
            u16::try_from(s_235_22).unwrap(),
        ));
        // D s_235_26: cast reint s_235_25 -> u8
        let s_235_26: bool = ((s_235_25.value()) != 0);
        // D s_235_27: call decode_ereta_aarch64_instrs_branch_unconditional_eret(s_235_8, s_235_14, s_235_20, s_235_26)
        let s_235_27: () = decode_ereta_aarch64_instrs_branch_unconditional_eret(
            state,
            tracer,
            s_235_8,
            s_235_14,
            s_235_20,
            s_235_26,
        );
        // N s_235_28: return
        return;
    }
    fn block_236<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_236_0: read-var merge#var.1:struct
        let s_236_0: u32 = fn_state.merge_var._1;
        // D s_236_1: write-var u#23227 <= s_236_0
        fn_state.u_23227 = s_236_0;
        // C s_236_2: const #21s : i
        let s_236_2: i128 = 21;
        // D s_236_3: read-var u#23227:u32
        let s_236_3: u32 = fn_state.u_23227;
        // D s_236_4: cast zx s_236_3 -> bv
        let s_236_4: Bits = Bits::new(s_236_3 as u128, 32u16);
        // C s_236_5: const #1s : i64
        let s_236_5: i64 = 1;
        // C s_236_6: cast zx s_236_5 -> i
        let s_236_6: i128 = (i128::try_from(s_236_5).unwrap());
        // C s_236_7: const #10s : i
        let s_236_7: i128 = 10;
        // C s_236_8: add s_236_7 s_236_6
        let s_236_8: i128 = (s_236_7 + s_236_6);
        // D s_236_9: bit-extract s_236_4 s_236_2 s_236_8
        let s_236_9: Bits = (Bits::new(
            ((s_236_4) >> (s_236_2)).value(),
            u16::try_from(s_236_8).unwrap(),
        ));
        // D s_236_10: cast reint s_236_9 -> u11
        let s_236_10: u16 = (s_236_9.value() as u16);
        // D s_236_11: cast zx s_236_10 -> bv
        let s_236_11: Bits = Bits::new(s_236_10 as u128, 11u16);
        // C s_236_12: const #1698u : u11
        let s_236_12: u16 = 1698;
        // C s_236_13: cast zx s_236_12 -> bv
        let s_236_13: Bits = Bits::new(s_236_12 as u128, 11u16);
        // D s_236_14: cmp-eq s_236_11 s_236_13
        let s_236_14: bool = ((s_236_11) == (s_236_13));
        // N s_236_15: branch s_236_14 b424 b237
        if s_236_14 {
            return block_424(state, tracer, fn_state);
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
        // D s_237_1: write-var gs#374201 <= s_237_0
        fn_state.gs_374201 = s_237_0;
        // N s_237_2: jump b238
        return block_238(state, tracer, fn_state);
    }
    fn block_238<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_238_0: read-var gs#374201:u8
        let s_238_0: bool = fn_state.gs_374201;
        // N s_238_1: branch s_238_0 b423 b239
        if s_238_0 {
            return block_423(state, tracer, fn_state);
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
        // D s_239_1: write-var gs#374203 <= s_239_0
        fn_state.gs_374203 = s_239_0;
        // N s_239_2: jump b240
        return block_240(state, tracer, fn_state);
    }
    fn block_240<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_240_0: read-var gs#374203:u8
        let s_240_0: bool = fn_state.gs_374203;
        // D s_240_1: not s_240_0
        let s_240_1: bool = !s_240_0;
        // N s_240_2: branch s_240_1 b242 b241
        if s_240_1 {
            return block_242(state, tracer, fn_state);
        } else {
            return block_241(state, tracer, fn_state);
        };
    }
    fn block_241<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_241_0: const #478s : i
        let s_241_0: i128 = 478;
        // C s_241_1: const #14696u : u32
        let s_241_1: u32 = 14696;
        // N s_241_2: write-reg s_241_1 <= s_241_0
        let s_241_2: () = {
            state.write_register::<i128>(s_241_1 as isize, s_241_0);
            tracer.write_register(s_241_1 as isize, s_241_0);
        };
        // C s_241_3: const #5s : i
        let s_241_3: i128 = 5;
        // C s_241_4: const #16s : i
        let s_241_4: i128 = 16;
        // D s_241_5: read-var u#23227:u32
        let s_241_5: u32 = fn_state.u_23227;
        // D s_241_6: cast zx s_241_5 -> bv
        let s_241_6: Bits = Bits::new(s_241_5 as u128, 32u16);
        // D s_241_7: bit-extract s_241_6 s_241_3 s_241_4
        let s_241_7: Bits = (Bits::new(
            ((s_241_6) >> (s_241_3)).value(),
            u16::try_from(s_241_4).unwrap(),
        ));
        // D s_241_8: cast reint s_241_7 -> u16
        let s_241_8: u16 = (s_241_7.value() as u16);
        // D s_241_9: call decode_hlt_aarch64_instrs_system_exceptions_debug_halt(s_241_8)
        let s_241_9: () = decode_hlt_aarch64_instrs_system_exceptions_debug_halt(
            state,
            tracer,
            s_241_8,
        );
        // N s_241_10: return
        return;
    }
    fn block_242<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_242_0: read-var merge#var.1:struct
        let s_242_0: u32 = fn_state.merge_var._1;
        // D s_242_1: write-var u#23230 <= s_242_0
        fn_state.u_23230 = s_242_0;
        // C s_242_2: const #21s : i
        let s_242_2: i128 = 21;
        // D s_242_3: read-var u#23230:u32
        let s_242_3: u32 = fn_state.u_23230;
        // D s_242_4: cast zx s_242_3 -> bv
        let s_242_4: Bits = Bits::new(s_242_3 as u128, 32u16);
        // C s_242_5: const #1s : i64
        let s_242_5: i64 = 1;
        // C s_242_6: cast zx s_242_5 -> i
        let s_242_6: i128 = (i128::try_from(s_242_5).unwrap());
        // C s_242_7: const #10s : i
        let s_242_7: i128 = 10;
        // C s_242_8: add s_242_7 s_242_6
        let s_242_8: i128 = (s_242_7 + s_242_6);
        // D s_242_9: bit-extract s_242_4 s_242_2 s_242_8
        let s_242_9: Bits = (Bits::new(
            ((s_242_4) >> (s_242_2)).value(),
            u16::try_from(s_242_8).unwrap(),
        ));
        // D s_242_10: cast reint s_242_9 -> u11
        let s_242_10: u16 = (s_242_9.value() as u16);
        // D s_242_11: cast zx s_242_10 -> bv
        let s_242_11: Bits = Bits::new(s_242_10 as u128, 11u16);
        // C s_242_12: const #1696u : u11
        let s_242_12: u16 = 1696;
        // C s_242_13: cast zx s_242_12 -> bv
        let s_242_13: Bits = Bits::new(s_242_12 as u128, 11u16);
        // D s_242_14: cmp-eq s_242_11 s_242_13
        let s_242_14: bool = ((s_242_11) == (s_242_13));
        // N s_242_15: branch s_242_14 b422 b243
        if s_242_14 {
            return block_422(state, tracer, fn_state);
        } else {
            return block_243(state, tracer, fn_state);
        };
    }
    fn block_243<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_243_0: const #0u : u8
        let s_243_0: bool = false;
        // D s_243_1: write-var gs#374213 <= s_243_0
        fn_state.gs_374213 = s_243_0;
        // N s_243_2: jump b244
        return block_244(state, tracer, fn_state);
    }
    fn block_244<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_244_0: read-var gs#374213:u8
        let s_244_0: bool = fn_state.gs_374213;
        // N s_244_1: branch s_244_0 b421 b245
        if s_244_0 {
            return block_421(state, tracer, fn_state);
        } else {
            return block_245(state, tracer, fn_state);
        };
    }
    fn block_245<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_245_0: const #0u : u8
        let s_245_0: bool = false;
        // D s_245_1: write-var gs#374215 <= s_245_0
        fn_state.gs_374215 = s_245_0;
        // N s_245_2: jump b246
        return block_246(state, tracer, fn_state);
    }
    fn block_246<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_246_0: read-var gs#374215:u8
        let s_246_0: bool = fn_state.gs_374215;
        // D s_246_1: not s_246_0
        let s_246_1: bool = !s_246_0;
        // N s_246_2: branch s_246_1 b248 b247
        if s_246_1 {
            return block_248(state, tracer, fn_state);
        } else {
            return block_247(state, tracer, fn_state);
        };
    }
    fn block_247<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_247_0: const #479s : i
        let s_247_0: i128 = 479;
        // C s_247_1: const #14696u : u32
        let s_247_1: u32 = 14696;
        // N s_247_2: write-reg s_247_1 <= s_247_0
        let s_247_2: () = {
            state.write_register::<i128>(s_247_1 as isize, s_247_0);
            tracer.write_register(s_247_1 as isize, s_247_0);
        };
        // C s_247_3: const #5s : i
        let s_247_3: i128 = 5;
        // C s_247_4: const #16s : i
        let s_247_4: i128 = 16;
        // D s_247_5: read-var u#23230:u32
        let s_247_5: u32 = fn_state.u_23230;
        // D s_247_6: cast zx s_247_5 -> bv
        let s_247_6: Bits = Bits::new(s_247_5 as u128, 32u16);
        // D s_247_7: bit-extract s_247_6 s_247_3 s_247_4
        let s_247_7: Bits = (Bits::new(
            ((s_247_6) >> (s_247_3)).value(),
            u16::try_from(s_247_4).unwrap(),
        ));
        // D s_247_8: cast reint s_247_7 -> u16
        let s_247_8: u16 = (s_247_7.value() as u16);
        // D s_247_9: call decode_hvc_aarch64_instrs_system_exceptions_runtime_hvc(s_247_8)
        let s_247_9: () = decode_hvc_aarch64_instrs_system_exceptions_runtime_hvc(
            state,
            tracer,
            s_247_8,
        );
        // N s_247_10: return
        return;
    }
    fn block_248<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_248_0: read-var merge#var.1:struct
        let s_248_0: u32 = fn_state.merge_var._1;
        // D s_248_1: write-var u#23233 <= s_248_0
        fn_state.u_23233 = s_248_0;
        // C s_248_2: const #12s : i
        let s_248_2: i128 = 12;
        // D s_248_3: read-var u#23233:u32
        let s_248_3: u32 = fn_state.u_23233;
        // D s_248_4: cast zx s_248_3 -> bv
        let s_248_4: Bits = Bits::new(s_248_3 as u128, 32u16);
        // C s_248_5: const #1s : i64
        let s_248_5: i64 = 1;
        // C s_248_6: cast zx s_248_5 -> i
        let s_248_6: i128 = (i128::try_from(s_248_5).unwrap());
        // C s_248_7: const #19s : i
        let s_248_7: i128 = 19;
        // C s_248_8: add s_248_7 s_248_6
        let s_248_8: i128 = (s_248_7 + s_248_6);
        // D s_248_9: bit-extract s_248_4 s_248_2 s_248_8
        let s_248_9: Bits = (Bits::new(
            ((s_248_4) >> (s_248_2)).value(),
            u16::try_from(s_248_8).unwrap(),
        ));
        // D s_248_10: cast reint s_248_9 -> u20
        let s_248_10: u32 = (s_248_9.value() as u32);
        // D s_248_11: cast zx s_248_10 -> bv
        let s_248_11: Bits = Bits::new(s_248_10 as u128, 20u16);
        // C s_248_12: const #872499u : u20
        let s_248_12: u32 = 872499;
        // C s_248_13: cast zx s_248_12 -> bv
        let s_248_13: Bits = Bits::new(s_248_12 as u128, 20u16);
        // D s_248_14: cmp-eq s_248_11 s_248_13
        let s_248_14: bool = ((s_248_11) == (s_248_13));
        // N s_248_15: branch s_248_14 b420 b249
        if s_248_14 {
            return block_420(state, tracer, fn_state);
        } else {
            return block_249(state, tracer, fn_state);
        };
    }
    fn block_249<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_249_0: const #0u : u8
        let s_249_0: bool = false;
        // D s_249_1: write-var gs#374225 <= s_249_0
        fn_state.gs_374225 = s_249_0;
        // N s_249_2: jump b250
        return block_250(state, tracer, fn_state);
    }
    fn block_250<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_250_0: read-var gs#374225:u8
        let s_250_0: bool = fn_state.gs_374225;
        // N s_250_1: branch s_250_0 b419 b251
        if s_250_0 {
            return block_419(state, tracer, fn_state);
        } else {
            return block_251(state, tracer, fn_state);
        };
    }
    fn block_251<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_251_0: const #0u : u8
        let s_251_0: bool = false;
        // D s_251_1: write-var gs#374227 <= s_251_0
        fn_state.gs_374227 = s_251_0;
        // N s_251_2: jump b252
        return block_252(state, tracer, fn_state);
    }
    fn block_252<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_252_0: read-var gs#374227:u8
        let s_252_0: bool = fn_state.gs_374227;
        // D s_252_1: not s_252_0
        let s_252_1: bool = !s_252_0;
        // N s_252_2: branch s_252_1 b254 b253
        if s_252_1 {
            return block_254(state, tracer, fn_state);
        } else {
            return block_253(state, tracer, fn_state);
        };
    }
    fn block_253<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_253_0: const #483s : i
        let s_253_0: i128 = 483;
        // C s_253_1: const #14696u : u32
        let s_253_1: u32 = 14696;
        // N s_253_2: write-reg s_253_1 <= s_253_0
        let s_253_2: () = {
            state.write_register::<i128>(s_253_1 as isize, s_253_0);
            tracer.write_register(s_253_1 as isize, s_253_0);
        };
        // C s_253_3: const #5s : i
        let s_253_3: i128 = 5;
        // C s_253_4: const #2s : i
        let s_253_4: i128 = 2;
        // D s_253_5: read-var u#23233:u32
        let s_253_5: u32 = fn_state.u_23233;
        // D s_253_6: cast zx s_253_5 -> bv
        let s_253_6: Bits = Bits::new(s_253_5 as u128, 32u16);
        // D s_253_7: bit-extract s_253_6 s_253_3 s_253_4
        let s_253_7: Bits = (Bits::new(
            ((s_253_6) >> (s_253_3)).value(),
            u16::try_from(s_253_4).unwrap(),
        ));
        // D s_253_8: cast reint s_253_7 -> u8
        let s_253_8: u8 = (s_253_7.value() as u8);
        // C s_253_9: const #8s : i
        let s_253_9: i128 = 8;
        // C s_253_10: const #4s : i
        let s_253_10: i128 = 4;
        // D s_253_11: read-var u#23233:u32
        let s_253_11: u32 = fn_state.u_23233;
        // D s_253_12: cast zx s_253_11 -> bv
        let s_253_12: Bits = Bits::new(s_253_11 as u128, 32u16);
        // D s_253_13: bit-extract s_253_12 s_253_9 s_253_10
        let s_253_13: Bits = (Bits::new(
            ((s_253_12) >> (s_253_9)).value(),
            u16::try_from(s_253_10).unwrap(),
        ));
        // D s_253_14: cast reint s_253_13 -> u8
        let s_253_14: u8 = (s_253_13.value() as u8);
        // D s_253_15: call decode_isb_aarch64_instrs_system_barriers_isb(s_253_8, s_253_14)
        let s_253_15: () = decode_isb_aarch64_instrs_system_barriers_isb(
            state,
            tracer,
            s_253_8,
            s_253_14,
        );
        // N s_253_16: return
        return;
    }
    fn block_254<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_254_0: read-var merge#var.1:struct
        let s_254_0: u32 = fn_state.merge_var._1;
        // D s_254_1: write-var u#23237 <= s_254_0
        fn_state.u_23237 = s_254_0;
        // C s_254_2: const #20s : i
        let s_254_2: i128 = 20;
        // D s_254_3: read-var u#23237:u32
        let s_254_3: u32 = fn_state.u_23237;
        // D s_254_4: cast zx s_254_3 -> bv
        let s_254_4: Bits = Bits::new(s_254_3 as u128, 32u16);
        // C s_254_5: const #1s : i64
        let s_254_5: i64 = 1;
        // C s_254_6: cast zx s_254_5 -> i
        let s_254_6: i128 = (i128::try_from(s_254_5).unwrap());
        // C s_254_7: const #11s : i
        let s_254_7: i128 = 11;
        // C s_254_8: add s_254_7 s_254_6
        let s_254_8: i128 = (s_254_7 + s_254_6);
        // D s_254_9: bit-extract s_254_4 s_254_2 s_254_8
        let s_254_9: Bits = (Bits::new(
            ((s_254_4) >> (s_254_2)).value(),
            u16::try_from(s_254_8).unwrap(),
        ));
        // D s_254_10: cast reint s_254_9 -> u12
        let s_254_10: u16 = (s_254_9.value() as u16);
        // D s_254_11: cast zx s_254_10 -> bv
        let s_254_11: Bits = Bits::new(s_254_10 as u128, 12u16);
        // C s_254_12: const #3415u : u12
        let s_254_12: u16 = 3415;
        // C s_254_13: cast zx s_254_12 -> bv
        let s_254_13: Bits = Bits::new(s_254_12 as u128, 12u16);
        // D s_254_14: cmp-eq s_254_11 s_254_13
        let s_254_14: bool = ((s_254_11) == (s_254_13));
        // N s_254_15: branch s_254_14 b418 b255
        if s_254_14 {
            return block_418(state, tracer, fn_state);
        } else {
            return block_255(state, tracer, fn_state);
        };
    }
    fn block_255<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_255_0: const #0u : u8
        let s_255_0: bool = false;
        // D s_255_1: write-var gs#374238 <= s_255_0
        fn_state.gs_374238 = s_255_0;
        // N s_255_2: jump b256
        return block_256(state, tracer, fn_state);
    }
    fn block_256<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_256_0: read-var gs#374238:u8
        let s_256_0: bool = fn_state.gs_374238;
        // D s_256_1: not s_256_0
        let s_256_1: bool = !s_256_0;
        // N s_256_2: branch s_256_1 b258 b257
        if s_256_1 {
            return block_258(state, tracer, fn_state);
        } else {
            return block_257(state, tracer, fn_state);
        };
    }
    fn block_257<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_257_0: const #699s : i
        let s_257_0: i128 = 699;
        // C s_257_1: const #14696u : u32
        let s_257_1: u32 = 14696;
        // N s_257_2: write-reg s_257_1 <= s_257_0
        let s_257_2: () = {
            state.write_register::<i128>(s_257_1 as isize, s_257_0);
            tracer.write_register(s_257_1 as isize, s_257_0);
        };
        // C s_257_3: const #0s : i
        let s_257_3: i128 = 0;
        // C s_257_4: const #5s : i
        let s_257_4: i128 = 5;
        // D s_257_5: read-var u#23237:u32
        let s_257_5: u32 = fn_state.u_23237;
        // D s_257_6: cast zx s_257_5 -> bv
        let s_257_6: Bits = Bits::new(s_257_5 as u128, 32u16);
        // D s_257_7: bit-extract s_257_6 s_257_3 s_257_4
        let s_257_7: Bits = (Bits::new(
            ((s_257_6) >> (s_257_3)).value(),
            u16::try_from(s_257_4).unwrap(),
        ));
        // D s_257_8: cast reint s_257_7 -> u8
        let s_257_8: u8 = (s_257_7.value() as u8);
        // C s_257_9: const #5s : i
        let s_257_9: i128 = 5;
        // C s_257_10: const #3s : i
        let s_257_10: i128 = 3;
        // D s_257_11: read-var u#23237:u32
        let s_257_11: u32 = fn_state.u_23237;
        // D s_257_12: cast zx s_257_11 -> bv
        let s_257_12: Bits = Bits::new(s_257_11 as u128, 32u16);
        // D s_257_13: bit-extract s_257_12 s_257_9 s_257_10
        let s_257_13: Bits = (Bits::new(
            ((s_257_12) >> (s_257_9)).value(),
            u16::try_from(s_257_10).unwrap(),
        ));
        // D s_257_14: cast reint s_257_13 -> u8
        let s_257_14: u8 = (s_257_13.value() as u8);
        // C s_257_15: const #8s : i
        let s_257_15: i128 = 8;
        // C s_257_16: const #4s : i
        let s_257_16: i128 = 4;
        // D s_257_17: read-var u#23237:u32
        let s_257_17: u32 = fn_state.u_23237;
        // D s_257_18: cast zx s_257_17 -> bv
        let s_257_18: Bits = Bits::new(s_257_17 as u128, 32u16);
        // D s_257_19: bit-extract s_257_18 s_257_15 s_257_16
        let s_257_19: Bits = (Bits::new(
            ((s_257_18) >> (s_257_15)).value(),
            u16::try_from(s_257_16).unwrap(),
        ));
        // D s_257_20: cast reint s_257_19 -> u8
        let s_257_20: u8 = (s_257_19.value() as u8);
        // C s_257_21: const #12s : i
        let s_257_21: i128 = 12;
        // C s_257_22: const #4s : i
        let s_257_22: i128 = 4;
        // D s_257_23: read-var u#23237:u32
        let s_257_23: u32 = fn_state.u_23237;
        // D s_257_24: cast zx s_257_23 -> bv
        let s_257_24: Bits = Bits::new(s_257_23 as u128, 32u16);
        // D s_257_25: bit-extract s_257_24 s_257_21 s_257_22
        let s_257_25: Bits = (Bits::new(
            ((s_257_24) >> (s_257_21)).value(),
            u16::try_from(s_257_22).unwrap(),
        ));
        // D s_257_26: cast reint s_257_25 -> u8
        let s_257_26: u8 = (s_257_25.value() as u8);
        // C s_257_27: const #16s : i
        let s_257_27: i128 = 16;
        // C s_257_28: const #3s : i
        let s_257_28: i128 = 3;
        // D s_257_29: read-var u#23237:u32
        let s_257_29: u32 = fn_state.u_23237;
        // D s_257_30: cast zx s_257_29 -> bv
        let s_257_30: Bits = Bits::new(s_257_29 as u128, 32u16);
        // D s_257_31: bit-extract s_257_30 s_257_27 s_257_28
        let s_257_31: Bits = (Bits::new(
            ((s_257_30) >> (s_257_27)).value(),
            u16::try_from(s_257_28).unwrap(),
        ));
        // D s_257_32: cast reint s_257_31 -> u8
        let s_257_32: u8 = (s_257_31.value() as u8);
        // C s_257_33: const #19s : i
        let s_257_33: i128 = 19;
        // C s_257_34: const #1s : i
        let s_257_34: i128 = 1;
        // D s_257_35: read-var u#23237:u32
        let s_257_35: u32 = fn_state.u_23237;
        // D s_257_36: cast zx s_257_35 -> bv
        let s_257_36: Bits = Bits::new(s_257_35 as u128, 32u16);
        // D s_257_37: bit-extract s_257_36 s_257_33 s_257_34
        let s_257_37: Bits = (Bits::new(
            ((s_257_36) >> (s_257_33)).value(),
            u16::try_from(s_257_34).unwrap(),
        ));
        // D s_257_38: cast reint s_257_37 -> u8
        let s_257_38: bool = ((s_257_37.value()) != 0);
        // C s_257_39: const #21s : i
        let s_257_39: i128 = 21;
        // C s_257_40: const #1s : i
        let s_257_40: i128 = 1;
        // D s_257_41: read-var u#23237:u32
        let s_257_41: u32 = fn_state.u_23237;
        // D s_257_42: cast zx s_257_41 -> bv
        let s_257_42: Bits = Bits::new(s_257_41 as u128, 32u16);
        // D s_257_43: bit-extract s_257_42 s_257_39 s_257_40
        let s_257_43: Bits = (Bits::new(
            ((s_257_42) >> (s_257_39)).value(),
            u16::try_from(s_257_40).unwrap(),
        ));
        // D s_257_44: cast reint s_257_43 -> u8
        let s_257_44: bool = ((s_257_43.value()) != 0);
        // D s_257_45: call decode_mrrs_aarch64_instrs_system_register_system_128(s_257_8, s_257_14, s_257_20, s_257_26, s_257_32, s_257_38, s_257_44)
        let s_257_45: () = decode_mrrs_aarch64_instrs_system_register_system_128(
            state,
            tracer,
            s_257_8,
            s_257_14,
            s_257_20,
            s_257_26,
            s_257_32,
            s_257_38,
            s_257_44,
        );
        // N s_257_46: return
        return;
    }
    fn block_258<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_258_0: read-var merge#var.1:struct
        let s_258_0: u32 = fn_state.merge_var._1;
        // D s_258_1: write-var u#23242 <= s_258_0
        fn_state.u_23242 = s_258_0;
        // C s_258_2: const #20s : i
        let s_258_2: i128 = 20;
        // D s_258_3: read-var u#23242:u32
        let s_258_3: u32 = fn_state.u_23242;
        // D s_258_4: cast zx s_258_3 -> bv
        let s_258_4: Bits = Bits::new(s_258_3 as u128, 32u16);
        // C s_258_5: const #1s : i64
        let s_258_5: i64 = 1;
        // C s_258_6: cast zx s_258_5 -> i
        let s_258_6: i128 = (i128::try_from(s_258_5).unwrap());
        // C s_258_7: const #11s : i
        let s_258_7: i128 = 11;
        // C s_258_8: add s_258_7 s_258_6
        let s_258_8: i128 = (s_258_7 + s_258_6);
        // D s_258_9: bit-extract s_258_4 s_258_2 s_258_8
        let s_258_9: Bits = (Bits::new(
            ((s_258_4) >> (s_258_2)).value(),
            u16::try_from(s_258_8).unwrap(),
        ));
        // D s_258_10: cast reint s_258_9 -> u12
        let s_258_10: u16 = (s_258_9.value() as u16);
        // D s_258_11: cast zx s_258_10 -> bv
        let s_258_11: Bits = Bits::new(s_258_10 as u128, 12u16);
        // C s_258_12: const #3413u : u12
        let s_258_12: u16 = 3413;
        // C s_258_13: cast zx s_258_12 -> bv
        let s_258_13: Bits = Bits::new(s_258_12 as u128, 12u16);
        // D s_258_14: cmp-eq s_258_11 s_258_13
        let s_258_14: bool = ((s_258_11) == (s_258_13));
        // N s_258_15: branch s_258_14 b417 b259
        if s_258_14 {
            return block_417(state, tracer, fn_state);
        } else {
            return block_259(state, tracer, fn_state);
        };
    }
    fn block_259<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_259_0: const #0u : u8
        let s_259_0: bool = false;
        // D s_259_1: write-var gs#374259 <= s_259_0
        fn_state.gs_374259 = s_259_0;
        // N s_259_2: jump b260
        return block_260(state, tracer, fn_state);
    }
    fn block_260<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_260_0: read-var gs#374259:u8
        let s_260_0: bool = fn_state.gs_374259;
        // D s_260_1: not s_260_0
        let s_260_1: bool = !s_260_0;
        // N s_260_2: branch s_260_1 b262 b261
        if s_260_1 {
            return block_262(state, tracer, fn_state);
        } else {
            return block_261(state, tracer, fn_state);
        };
    }
    fn block_261<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_261_0: const #700s : i
        let s_261_0: i128 = 700;
        // C s_261_1: const #14696u : u32
        let s_261_1: u32 = 14696;
        // N s_261_2: write-reg s_261_1 <= s_261_0
        let s_261_2: () = {
            state.write_register::<i128>(s_261_1 as isize, s_261_0);
            tracer.write_register(s_261_1 as isize, s_261_0);
        };
        // C s_261_3: const #0s : i
        let s_261_3: i128 = 0;
        // C s_261_4: const #5s : i
        let s_261_4: i128 = 5;
        // D s_261_5: read-var u#23242:u32
        let s_261_5: u32 = fn_state.u_23242;
        // D s_261_6: cast zx s_261_5 -> bv
        let s_261_6: Bits = Bits::new(s_261_5 as u128, 32u16);
        // D s_261_7: bit-extract s_261_6 s_261_3 s_261_4
        let s_261_7: Bits = (Bits::new(
            ((s_261_6) >> (s_261_3)).value(),
            u16::try_from(s_261_4).unwrap(),
        ));
        // D s_261_8: cast reint s_261_7 -> u8
        let s_261_8: u8 = (s_261_7.value() as u8);
        // C s_261_9: const #5s : i
        let s_261_9: i128 = 5;
        // C s_261_10: const #3s : i
        let s_261_10: i128 = 3;
        // D s_261_11: read-var u#23242:u32
        let s_261_11: u32 = fn_state.u_23242;
        // D s_261_12: cast zx s_261_11 -> bv
        let s_261_12: Bits = Bits::new(s_261_11 as u128, 32u16);
        // D s_261_13: bit-extract s_261_12 s_261_9 s_261_10
        let s_261_13: Bits = (Bits::new(
            ((s_261_12) >> (s_261_9)).value(),
            u16::try_from(s_261_10).unwrap(),
        ));
        // D s_261_14: cast reint s_261_13 -> u8
        let s_261_14: u8 = (s_261_13.value() as u8);
        // C s_261_15: const #8s : i
        let s_261_15: i128 = 8;
        // C s_261_16: const #4s : i
        let s_261_16: i128 = 4;
        // D s_261_17: read-var u#23242:u32
        let s_261_17: u32 = fn_state.u_23242;
        // D s_261_18: cast zx s_261_17 -> bv
        let s_261_18: Bits = Bits::new(s_261_17 as u128, 32u16);
        // D s_261_19: bit-extract s_261_18 s_261_15 s_261_16
        let s_261_19: Bits = (Bits::new(
            ((s_261_18) >> (s_261_15)).value(),
            u16::try_from(s_261_16).unwrap(),
        ));
        // D s_261_20: cast reint s_261_19 -> u8
        let s_261_20: u8 = (s_261_19.value() as u8);
        // C s_261_21: const #12s : i
        let s_261_21: i128 = 12;
        // C s_261_22: const #4s : i
        let s_261_22: i128 = 4;
        // D s_261_23: read-var u#23242:u32
        let s_261_23: u32 = fn_state.u_23242;
        // D s_261_24: cast zx s_261_23 -> bv
        let s_261_24: Bits = Bits::new(s_261_23 as u128, 32u16);
        // D s_261_25: bit-extract s_261_24 s_261_21 s_261_22
        let s_261_25: Bits = (Bits::new(
            ((s_261_24) >> (s_261_21)).value(),
            u16::try_from(s_261_22).unwrap(),
        ));
        // D s_261_26: cast reint s_261_25 -> u8
        let s_261_26: u8 = (s_261_25.value() as u8);
        // C s_261_27: const #16s : i
        let s_261_27: i128 = 16;
        // C s_261_28: const #3s : i
        let s_261_28: i128 = 3;
        // D s_261_29: read-var u#23242:u32
        let s_261_29: u32 = fn_state.u_23242;
        // D s_261_30: cast zx s_261_29 -> bv
        let s_261_30: Bits = Bits::new(s_261_29 as u128, 32u16);
        // D s_261_31: bit-extract s_261_30 s_261_27 s_261_28
        let s_261_31: Bits = (Bits::new(
            ((s_261_30) >> (s_261_27)).value(),
            u16::try_from(s_261_28).unwrap(),
        ));
        // D s_261_32: cast reint s_261_31 -> u8
        let s_261_32: u8 = (s_261_31.value() as u8);
        // C s_261_33: const #19s : i
        let s_261_33: i128 = 19;
        // C s_261_34: const #1s : i
        let s_261_34: i128 = 1;
        // D s_261_35: read-var u#23242:u32
        let s_261_35: u32 = fn_state.u_23242;
        // D s_261_36: cast zx s_261_35 -> bv
        let s_261_36: Bits = Bits::new(s_261_35 as u128, 32u16);
        // D s_261_37: bit-extract s_261_36 s_261_33 s_261_34
        let s_261_37: Bits = (Bits::new(
            ((s_261_36) >> (s_261_33)).value(),
            u16::try_from(s_261_34).unwrap(),
        ));
        // D s_261_38: cast reint s_261_37 -> u8
        let s_261_38: bool = ((s_261_37.value()) != 0);
        // C s_261_39: const #21s : i
        let s_261_39: i128 = 21;
        // C s_261_40: const #1s : i
        let s_261_40: i128 = 1;
        // D s_261_41: read-var u#23242:u32
        let s_261_41: u32 = fn_state.u_23242;
        // D s_261_42: cast zx s_261_41 -> bv
        let s_261_42: Bits = Bits::new(s_261_41 as u128, 32u16);
        // D s_261_43: bit-extract s_261_42 s_261_39 s_261_40
        let s_261_43: Bits = (Bits::new(
            ((s_261_42) >> (s_261_39)).value(),
            u16::try_from(s_261_40).unwrap(),
        ));
        // D s_261_44: cast reint s_261_43 -> u8
        let s_261_44: bool = ((s_261_43.value()) != 0);
        // D s_261_45: call decode_msrr_aarch64_instrs_system_register_system_128(s_261_8, s_261_14, s_261_20, s_261_26, s_261_32, s_261_38, s_261_44)
        let s_261_45: () = decode_msrr_aarch64_instrs_system_register_system_128(
            state,
            tracer,
            s_261_8,
            s_261_14,
            s_261_20,
            s_261_26,
            s_261_32,
            s_261_38,
            s_261_44,
        );
        // N s_261_46: return
        return;
    }
    fn block_262<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_262_0: read-var merge#var.1:struct
        let s_262_0: u32 = fn_state.merge_var._1;
        // D s_262_1: write-var u#23251 <= s_262_0
        fn_state.u_23251 = s_262_0;
        // C s_262_2: const #20s : i
        let s_262_2: i128 = 20;
        // D s_262_3: read-var u#23251:u32
        let s_262_3: u32 = fn_state.u_23251;
        // D s_262_4: cast zx s_262_3 -> bv
        let s_262_4: Bits = Bits::new(s_262_3 as u128, 32u16);
        // C s_262_5: const #1s : i64
        let s_262_5: i64 = 1;
        // C s_262_6: cast zx s_262_5 -> i
        let s_262_6: i128 = (i128::try_from(s_262_5).unwrap());
        // C s_262_7: const #11s : i
        let s_262_7: i128 = 11;
        // C s_262_8: add s_262_7 s_262_6
        let s_262_8: i128 = (s_262_7 + s_262_6);
        // D s_262_9: bit-extract s_262_4 s_262_2 s_262_8
        let s_262_9: Bits = (Bits::new(
            ((s_262_4) >> (s_262_2)).value(),
            u16::try_from(s_262_8).unwrap(),
        ));
        // D s_262_10: cast reint s_262_9 -> u12
        let s_262_10: u16 = (s_262_9.value() as u16);
        // D s_262_11: cast zx s_262_10 -> bv
        let s_262_11: Bits = Bits::new(s_262_10 as u128, 12u16);
        // C s_262_12: const #3411u : u12
        let s_262_12: u16 = 3411;
        // C s_262_13: cast zx s_262_12 -> bv
        let s_262_13: Bits = Bits::new(s_262_12 as u128, 12u16);
        // D s_262_14: cmp-eq s_262_11 s_262_13
        let s_262_14: bool = ((s_262_11) == (s_262_13));
        // N s_262_15: branch s_262_14 b416 b263
        if s_262_14 {
            return block_416(state, tracer, fn_state);
        } else {
            return block_263(state, tracer, fn_state);
        };
    }
    fn block_263<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_263_0: const #0u : u8
        let s_263_0: bool = false;
        // D s_263_1: write-var gs#374280 <= s_263_0
        fn_state.gs_374280 = s_263_0;
        // N s_263_2: jump b264
        return block_264(state, tracer, fn_state);
    }
    fn block_264<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_264_0: read-var gs#374280:u8
        let s_264_0: bool = fn_state.gs_374280;
        // D s_264_1: not s_264_0
        let s_264_1: bool = !s_264_0;
        // N s_264_2: branch s_264_1 b266 b265
        if s_264_1 {
            return block_266(state, tracer, fn_state);
        } else {
            return block_265(state, tracer, fn_state);
        };
    }
    fn block_265<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_265_0: const #701s : i
        let s_265_0: i128 = 701;
        // C s_265_1: const #14696u : u32
        let s_265_1: u32 = 14696;
        // N s_265_2: write-reg s_265_1 <= s_265_0
        let s_265_2: () = {
            state.write_register::<i128>(s_265_1 as isize, s_265_0);
            tracer.write_register(s_265_1 as isize, s_265_0);
        };
        // C s_265_3: const #0s : i
        let s_265_3: i128 = 0;
        // C s_265_4: const #5s : i
        let s_265_4: i128 = 5;
        // D s_265_5: read-var u#23251:u32
        let s_265_5: u32 = fn_state.u_23251;
        // D s_265_6: cast zx s_265_5 -> bv
        let s_265_6: Bits = Bits::new(s_265_5 as u128, 32u16);
        // D s_265_7: bit-extract s_265_6 s_265_3 s_265_4
        let s_265_7: Bits = (Bits::new(
            ((s_265_6) >> (s_265_3)).value(),
            u16::try_from(s_265_4).unwrap(),
        ));
        // D s_265_8: cast reint s_265_7 -> u8
        let s_265_8: u8 = (s_265_7.value() as u8);
        // C s_265_9: const #5s : i
        let s_265_9: i128 = 5;
        // C s_265_10: const #3s : i
        let s_265_10: i128 = 3;
        // D s_265_11: read-var u#23251:u32
        let s_265_11: u32 = fn_state.u_23251;
        // D s_265_12: cast zx s_265_11 -> bv
        let s_265_12: Bits = Bits::new(s_265_11 as u128, 32u16);
        // D s_265_13: bit-extract s_265_12 s_265_9 s_265_10
        let s_265_13: Bits = (Bits::new(
            ((s_265_12) >> (s_265_9)).value(),
            u16::try_from(s_265_10).unwrap(),
        ));
        // D s_265_14: cast reint s_265_13 -> u8
        let s_265_14: u8 = (s_265_13.value() as u8);
        // C s_265_15: const #8s : i
        let s_265_15: i128 = 8;
        // C s_265_16: const #4s : i
        let s_265_16: i128 = 4;
        // D s_265_17: read-var u#23251:u32
        let s_265_17: u32 = fn_state.u_23251;
        // D s_265_18: cast zx s_265_17 -> bv
        let s_265_18: Bits = Bits::new(s_265_17 as u128, 32u16);
        // D s_265_19: bit-extract s_265_18 s_265_15 s_265_16
        let s_265_19: Bits = (Bits::new(
            ((s_265_18) >> (s_265_15)).value(),
            u16::try_from(s_265_16).unwrap(),
        ));
        // D s_265_20: cast reint s_265_19 -> u8
        let s_265_20: u8 = (s_265_19.value() as u8);
        // C s_265_21: const #12s : i
        let s_265_21: i128 = 12;
        // C s_265_22: const #4s : i
        let s_265_22: i128 = 4;
        // D s_265_23: read-var u#23251:u32
        let s_265_23: u32 = fn_state.u_23251;
        // D s_265_24: cast zx s_265_23 -> bv
        let s_265_24: Bits = Bits::new(s_265_23 as u128, 32u16);
        // D s_265_25: bit-extract s_265_24 s_265_21 s_265_22
        let s_265_25: Bits = (Bits::new(
            ((s_265_24) >> (s_265_21)).value(),
            u16::try_from(s_265_22).unwrap(),
        ));
        // D s_265_26: cast reint s_265_25 -> u8
        let s_265_26: u8 = (s_265_25.value() as u8);
        // C s_265_27: const #16s : i
        let s_265_27: i128 = 16;
        // C s_265_28: const #3s : i
        let s_265_28: i128 = 3;
        // D s_265_29: read-var u#23251:u32
        let s_265_29: u32 = fn_state.u_23251;
        // D s_265_30: cast zx s_265_29 -> bv
        let s_265_30: Bits = Bits::new(s_265_29 as u128, 32u16);
        // D s_265_31: bit-extract s_265_30 s_265_27 s_265_28
        let s_265_31: Bits = (Bits::new(
            ((s_265_30) >> (s_265_27)).value(),
            u16::try_from(s_265_28).unwrap(),
        ));
        // D s_265_32: cast reint s_265_31 -> u8
        let s_265_32: u8 = (s_265_31.value() as u8);
        // C s_265_33: const #19s : i
        let s_265_33: i128 = 19;
        // C s_265_34: const #1s : i
        let s_265_34: i128 = 1;
        // D s_265_35: read-var u#23251:u32
        let s_265_35: u32 = fn_state.u_23251;
        // D s_265_36: cast zx s_265_35 -> bv
        let s_265_36: Bits = Bits::new(s_265_35 as u128, 32u16);
        // D s_265_37: bit-extract s_265_36 s_265_33 s_265_34
        let s_265_37: Bits = (Bits::new(
            ((s_265_36) >> (s_265_33)).value(),
            u16::try_from(s_265_34).unwrap(),
        ));
        // D s_265_38: cast reint s_265_37 -> u8
        let s_265_38: bool = ((s_265_37.value()) != 0);
        // C s_265_39: const #21s : i
        let s_265_39: i128 = 21;
        // C s_265_40: const #1s : i
        let s_265_40: i128 = 1;
        // D s_265_41: read-var u#23251:u32
        let s_265_41: u32 = fn_state.u_23251;
        // D s_265_42: cast zx s_265_41 -> bv
        let s_265_42: Bits = Bits::new(s_265_41 as u128, 32u16);
        // D s_265_43: bit-extract s_265_42 s_265_39 s_265_40
        let s_265_43: Bits = (Bits::new(
            ((s_265_42) >> (s_265_39)).value(),
            u16::try_from(s_265_40).unwrap(),
        ));
        // D s_265_44: cast reint s_265_43 -> u8
        let s_265_44: bool = ((s_265_43.value()) != 0);
        // D s_265_45: call decode_mrs_aarch64_instrs_system_register_system(s_265_8, s_265_14, s_265_20, s_265_26, s_265_32, s_265_38, s_265_44)
        let s_265_45: () = decode_mrs_aarch64_instrs_system_register_system(
            state,
            tracer,
            s_265_8,
            s_265_14,
            s_265_20,
            s_265_26,
            s_265_32,
            s_265_38,
            s_265_44,
        );
        // N s_265_46: return
        return;
    }
    fn block_266<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_266_0: read-var merge#var.1:struct
        let s_266_0: u32 = fn_state.merge_var._1;
        // D s_266_1: write-var u#23260 <= s_266_0
        fn_state.u_23260 = s_266_0;
        // C s_266_2: const #20s : i
        let s_266_2: i128 = 20;
        // D s_266_3: read-var u#23260:u32
        let s_266_3: u32 = fn_state.u_23260;
        // D s_266_4: cast zx s_266_3 -> bv
        let s_266_4: Bits = Bits::new(s_266_3 as u128, 32u16);
        // C s_266_5: const #1s : i64
        let s_266_5: i64 = 1;
        // C s_266_6: cast zx s_266_5 -> i
        let s_266_6: i128 = (i128::try_from(s_266_5).unwrap());
        // C s_266_7: const #11s : i
        let s_266_7: i128 = 11;
        // C s_266_8: add s_266_7 s_266_6
        let s_266_8: i128 = (s_266_7 + s_266_6);
        // D s_266_9: bit-extract s_266_4 s_266_2 s_266_8
        let s_266_9: Bits = (Bits::new(
            ((s_266_4) >> (s_266_2)).value(),
            u16::try_from(s_266_8).unwrap(),
        ));
        // D s_266_10: cast reint s_266_9 -> u12
        let s_266_10: u16 = (s_266_9.value() as u16);
        // D s_266_11: cast zx s_266_10 -> bv
        let s_266_11: Bits = Bits::new(s_266_10 as u128, 12u16);
        // C s_266_12: const #3409u : u12
        let s_266_12: u16 = 3409;
        // C s_266_13: cast zx s_266_12 -> bv
        let s_266_13: Bits = Bits::new(s_266_12 as u128, 12u16);
        // D s_266_14: cmp-eq s_266_11 s_266_13
        let s_266_14: bool = ((s_266_11) == (s_266_13));
        // N s_266_15: branch s_266_14 b415 b267
        if s_266_14 {
            return block_415(state, tracer, fn_state);
        } else {
            return block_267(state, tracer, fn_state);
        };
    }
    fn block_267<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_267_0: const #0u : u8
        let s_267_0: bool = false;
        // D s_267_1: write-var gs#374301 <= s_267_0
        fn_state.gs_374301 = s_267_0;
        // N s_267_2: jump b268
        return block_268(state, tracer, fn_state);
    }
    fn block_268<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_268_0: read-var gs#374301:u8
        let s_268_0: bool = fn_state.gs_374301;
        // D s_268_1: not s_268_0
        let s_268_1: bool = !s_268_0;
        // N s_268_2: branch s_268_1 b270 b269
        if s_268_1 {
            return block_270(state, tracer, fn_state);
        } else {
            return block_269(state, tracer, fn_state);
        };
    }
    fn block_269<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_269_0: const #702s : i
        let s_269_0: i128 = 702;
        // C s_269_1: const #14696u : u32
        let s_269_1: u32 = 14696;
        // N s_269_2: write-reg s_269_1 <= s_269_0
        let s_269_2: () = {
            state.write_register::<i128>(s_269_1 as isize, s_269_0);
            tracer.write_register(s_269_1 as isize, s_269_0);
        };
        // C s_269_3: const #0s : i
        let s_269_3: i128 = 0;
        // C s_269_4: const #5s : i
        let s_269_4: i128 = 5;
        // D s_269_5: read-var u#23260:u32
        let s_269_5: u32 = fn_state.u_23260;
        // D s_269_6: cast zx s_269_5 -> bv
        let s_269_6: Bits = Bits::new(s_269_5 as u128, 32u16);
        // D s_269_7: bit-extract s_269_6 s_269_3 s_269_4
        let s_269_7: Bits = (Bits::new(
            ((s_269_6) >> (s_269_3)).value(),
            u16::try_from(s_269_4).unwrap(),
        ));
        // D s_269_8: cast reint s_269_7 -> u8
        let s_269_8: u8 = (s_269_7.value() as u8);
        // C s_269_9: const #5s : i
        let s_269_9: i128 = 5;
        // C s_269_10: const #3s : i
        let s_269_10: i128 = 3;
        // D s_269_11: read-var u#23260:u32
        let s_269_11: u32 = fn_state.u_23260;
        // D s_269_12: cast zx s_269_11 -> bv
        let s_269_12: Bits = Bits::new(s_269_11 as u128, 32u16);
        // D s_269_13: bit-extract s_269_12 s_269_9 s_269_10
        let s_269_13: Bits = (Bits::new(
            ((s_269_12) >> (s_269_9)).value(),
            u16::try_from(s_269_10).unwrap(),
        ));
        // D s_269_14: cast reint s_269_13 -> u8
        let s_269_14: u8 = (s_269_13.value() as u8);
        // C s_269_15: const #8s : i
        let s_269_15: i128 = 8;
        // C s_269_16: const #4s : i
        let s_269_16: i128 = 4;
        // D s_269_17: read-var u#23260:u32
        let s_269_17: u32 = fn_state.u_23260;
        // D s_269_18: cast zx s_269_17 -> bv
        let s_269_18: Bits = Bits::new(s_269_17 as u128, 32u16);
        // D s_269_19: bit-extract s_269_18 s_269_15 s_269_16
        let s_269_19: Bits = (Bits::new(
            ((s_269_18) >> (s_269_15)).value(),
            u16::try_from(s_269_16).unwrap(),
        ));
        // D s_269_20: cast reint s_269_19 -> u8
        let s_269_20: u8 = (s_269_19.value() as u8);
        // C s_269_21: const #12s : i
        let s_269_21: i128 = 12;
        // C s_269_22: const #4s : i
        let s_269_22: i128 = 4;
        // D s_269_23: read-var u#23260:u32
        let s_269_23: u32 = fn_state.u_23260;
        // D s_269_24: cast zx s_269_23 -> bv
        let s_269_24: Bits = Bits::new(s_269_23 as u128, 32u16);
        // D s_269_25: bit-extract s_269_24 s_269_21 s_269_22
        let s_269_25: Bits = (Bits::new(
            ((s_269_24) >> (s_269_21)).value(),
            u16::try_from(s_269_22).unwrap(),
        ));
        // D s_269_26: cast reint s_269_25 -> u8
        let s_269_26: u8 = (s_269_25.value() as u8);
        // C s_269_27: const #16s : i
        let s_269_27: i128 = 16;
        // C s_269_28: const #3s : i
        let s_269_28: i128 = 3;
        // D s_269_29: read-var u#23260:u32
        let s_269_29: u32 = fn_state.u_23260;
        // D s_269_30: cast zx s_269_29 -> bv
        let s_269_30: Bits = Bits::new(s_269_29 as u128, 32u16);
        // D s_269_31: bit-extract s_269_30 s_269_27 s_269_28
        let s_269_31: Bits = (Bits::new(
            ((s_269_30) >> (s_269_27)).value(),
            u16::try_from(s_269_28).unwrap(),
        ));
        // D s_269_32: cast reint s_269_31 -> u8
        let s_269_32: u8 = (s_269_31.value() as u8);
        // C s_269_33: const #19s : i
        let s_269_33: i128 = 19;
        // C s_269_34: const #1s : i
        let s_269_34: i128 = 1;
        // D s_269_35: read-var u#23260:u32
        let s_269_35: u32 = fn_state.u_23260;
        // D s_269_36: cast zx s_269_35 -> bv
        let s_269_36: Bits = Bits::new(s_269_35 as u128, 32u16);
        // D s_269_37: bit-extract s_269_36 s_269_33 s_269_34
        let s_269_37: Bits = (Bits::new(
            ((s_269_36) >> (s_269_33)).value(),
            u16::try_from(s_269_34).unwrap(),
        ));
        // D s_269_38: cast reint s_269_37 -> u8
        let s_269_38: bool = ((s_269_37.value()) != 0);
        // C s_269_39: const #21s : i
        let s_269_39: i128 = 21;
        // C s_269_40: const #1s : i
        let s_269_40: i128 = 1;
        // D s_269_41: read-var u#23260:u32
        let s_269_41: u32 = fn_state.u_23260;
        // D s_269_42: cast zx s_269_41 -> bv
        let s_269_42: Bits = Bits::new(s_269_41 as u128, 32u16);
        // D s_269_43: bit-extract s_269_42 s_269_39 s_269_40
        let s_269_43: Bits = (Bits::new(
            ((s_269_42) >> (s_269_39)).value(),
            u16::try_from(s_269_40).unwrap(),
        ));
        // D s_269_44: cast reint s_269_43 -> u8
        let s_269_44: bool = ((s_269_43.value()) != 0);
        // D s_269_45: call decode_msr_reg_aarch64_instrs_system_register_system(s_269_8, s_269_14, s_269_20, s_269_26, s_269_32, s_269_38, s_269_44)
        let s_269_45: () = decode_msr_reg_aarch64_instrs_system_register_system(
            state,
            tracer,
            s_269_8,
            s_269_14,
            s_269_20,
            s_269_26,
            s_269_32,
            s_269_38,
            s_269_44,
        );
        // N s_269_46: return
        return;
    }
    fn block_270<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_270_0: read-var merge#var.1:struct
        let s_270_0: u32 = fn_state.merge_var._1;
        // D s_270_1: write-var u#23269 <= s_270_0
        fn_state.u_23269 = s_270_0;
        // C s_270_2: const #19s : i
        let s_270_2: i128 = 19;
        // D s_270_3: read-var u#23269:u32
        let s_270_3: u32 = fn_state.u_23269;
        // D s_270_4: cast zx s_270_3 -> bv
        let s_270_4: Bits = Bits::new(s_270_3 as u128, 32u16);
        // C s_270_5: const #1s : i64
        let s_270_5: i64 = 1;
        // C s_270_6: cast zx s_270_5 -> i
        let s_270_6: i128 = (i128::try_from(s_270_5).unwrap());
        // C s_270_7: const #12s : i
        let s_270_7: i128 = 12;
        // C s_270_8: add s_270_7 s_270_6
        let s_270_8: i128 = (s_270_7 + s_270_6);
        // D s_270_9: bit-extract s_270_4 s_270_2 s_270_8
        let s_270_9: Bits = (Bits::new(
            ((s_270_4) >> (s_270_2)).value(),
            u16::try_from(s_270_8).unwrap(),
        ));
        // D s_270_10: cast reint s_270_9 -> u13
        let s_270_10: u16 = (s_270_9.value() as u16);
        // D s_270_11: cast zx s_270_10 -> bv
        let s_270_11: Bits = Bits::new(s_270_10 as u128, 13u16);
        // C s_270_12: const #6816u : u13
        let s_270_12: u16 = 6816;
        // C s_270_13: cast zx s_270_12 -> bv
        let s_270_13: Bits = Bits::new(s_270_12 as u128, 13u16);
        // D s_270_14: cmp-eq s_270_11 s_270_13
        let s_270_14: bool = ((s_270_11) == (s_270_13));
        // N s_270_15: branch s_270_14 b411 b271
        if s_270_14 {
            return block_411(state, tracer, fn_state);
        } else {
            return block_271(state, tracer, fn_state);
        };
    }
    fn block_271<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_271_0: const #0u : u8
        let s_271_0: bool = false;
        // D s_271_1: write-var gs#374326 <= s_271_0
        fn_state.gs_374326 = s_271_0;
        // N s_271_2: jump b272
        return block_272(state, tracer, fn_state);
    }
    fn block_272<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_272_0: read-var gs#374326:u8
        let s_272_0: bool = fn_state.gs_374326;
        // N s_272_1: branch s_272_0 b410 b273
        if s_272_0 {
            return block_410(state, tracer, fn_state);
        } else {
            return block_273(state, tracer, fn_state);
        };
    }
    fn block_273<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_273_0: const #0u : u8
        let s_273_0: bool = false;
        // D s_273_1: write-var gs#374328 <= s_273_0
        fn_state.gs_374328 = s_273_0;
        // N s_273_2: jump b274
        return block_274(state, tracer, fn_state);
    }
    fn block_274<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_274_0: read-var gs#374328:u8
        let s_274_0: bool = fn_state.gs_374328;
        // D s_274_1: not s_274_0
        let s_274_1: bool = !s_274_0;
        // N s_274_2: branch s_274_1 b276 b275
        if s_274_1 {
            return block_276(state, tracer, fn_state);
        } else {
            return block_275(state, tracer, fn_state);
        };
    }
    fn block_275<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_275_0: const #703s : i
        let s_275_0: i128 = 703;
        // C s_275_1: const #14696u : u32
        let s_275_1: u32 = 14696;
        // N s_275_2: write-reg s_275_1 <= s_275_0
        let s_275_2: () = {
            state.write_register::<i128>(s_275_1 as isize, s_275_0);
            tracer.write_register(s_275_1 as isize, s_275_0);
        };
        // C s_275_3: const #5s : i
        let s_275_3: i128 = 5;
        // C s_275_4: const #3s : i
        let s_275_4: i128 = 3;
        // D s_275_5: read-var u#23269:u32
        let s_275_5: u32 = fn_state.u_23269;
        // D s_275_6: cast zx s_275_5 -> bv
        let s_275_6: Bits = Bits::new(s_275_5 as u128, 32u16);
        // D s_275_7: bit-extract s_275_6 s_275_3 s_275_4
        let s_275_7: Bits = (Bits::new(
            ((s_275_6) >> (s_275_3)).value(),
            u16::try_from(s_275_4).unwrap(),
        ));
        // D s_275_8: cast reint s_275_7 -> u8
        let s_275_8: u8 = (s_275_7.value() as u8);
        // C s_275_9: const #8s : i
        let s_275_9: i128 = 8;
        // C s_275_10: const #4s : i
        let s_275_10: i128 = 4;
        // D s_275_11: read-var u#23269:u32
        let s_275_11: u32 = fn_state.u_23269;
        // D s_275_12: cast zx s_275_11 -> bv
        let s_275_12: Bits = Bits::new(s_275_11 as u128, 32u16);
        // D s_275_13: bit-extract s_275_12 s_275_9 s_275_10
        let s_275_13: Bits = (Bits::new(
            ((s_275_12) >> (s_275_9)).value(),
            u16::try_from(s_275_10).unwrap(),
        ));
        // D s_275_14: cast reint s_275_13 -> u8
        let s_275_14: u8 = (s_275_13.value() as u8);
        // C s_275_15: const #16s : i
        let s_275_15: i128 = 16;
        // C s_275_16: const #3s : i
        let s_275_16: i128 = 3;
        // D s_275_17: read-var u#23269:u32
        let s_275_17: u32 = fn_state.u_23269;
        // D s_275_18: cast zx s_275_17 -> bv
        let s_275_18: Bits = Bits::new(s_275_17 as u128, 32u16);
        // D s_275_19: bit-extract s_275_18 s_275_15 s_275_16
        let s_275_19: Bits = (Bits::new(
            ((s_275_18) >> (s_275_15)).value(),
            u16::try_from(s_275_16).unwrap(),
        ));
        // D s_275_20: cast reint s_275_19 -> u8
        let s_275_20: u8 = (s_275_19.value() as u8);
        // D s_275_21: call decode_msr_imm_aarch64_instrs_system_register_cpsr(s_275_8, s_275_14, s_275_20)
        let s_275_21: () = decode_msr_imm_aarch64_instrs_system_register_cpsr(
            state,
            tracer,
            s_275_8,
            s_275_14,
            s_275_20,
        );
        // N s_275_22: return
        return;
    }
    fn block_276<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_276_0: read-var merge#var.1:struct
        let s_276_0: u32 = fn_state.merge_var._1;
        // D s_276_1: write-var u#23274 <= s_276_0
        fn_state.u_23274 = s_276_0;
        // C s_276_2: const #10s : i
        let s_276_2: i128 = 10;
        // D s_276_3: read-var u#23274:u32
        let s_276_3: u32 = fn_state.u_23274;
        // D s_276_4: cast zx s_276_3 -> bv
        let s_276_4: Bits = Bits::new(s_276_3 as u128, 32u16);
        // C s_276_5: const #1s : i64
        let s_276_5: i64 = 1;
        // C s_276_6: cast zx s_276_5 -> i
        let s_276_6: i128 = (i128::try_from(s_276_5).unwrap());
        // C s_276_7: const #21s : i
        let s_276_7: i128 = 21;
        // C s_276_8: add s_276_7 s_276_6
        let s_276_8: i128 = (s_276_7 + s_276_6);
        // D s_276_9: bit-extract s_276_4 s_276_2 s_276_8
        let s_276_9: Bits = (Bits::new(
            ((s_276_4) >> (s_276_2)).value(),
            u16::try_from(s_276_8).unwrap(),
        ));
        // D s_276_10: cast reint s_276_9 -> u22
        let s_276_10: u32 = (s_276_9.value() as u32);
        // D s_276_11: cast zx s_276_10 -> bv
        let s_276_11: Bits = Bits::new(s_276_10 as u128, 22u16);
        // C s_276_12: const #3489992u : u22
        let s_276_12: u32 = 3489992;
        // C s_276_13: cast zx s_276_12 -> bv
        let s_276_13: Bits = Bits::new(s_276_12 as u128, 22u16);
        // D s_276_14: cmp-eq s_276_11 s_276_13
        let s_276_14: bool = ((s_276_11) == (s_276_13));
        // N s_276_15: branch s_276_14 b406 b277
        if s_276_14 {
            return block_406(state, tracer, fn_state);
        } else {
            return block_277(state, tracer, fn_state);
        };
    }
    fn block_277<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_277_0: const #0u : u8
        let s_277_0: bool = false;
        // D s_277_1: write-var gs#374345 <= s_277_0
        fn_state.gs_374345 = s_277_0;
        // N s_277_2: jump b278
        return block_278(state, tracer, fn_state);
    }
    fn block_278<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_278_0: read-var gs#374345:u8
        let s_278_0: bool = fn_state.gs_374345;
        // N s_278_1: branch s_278_0 b405 b279
        if s_278_0 {
            return block_405(state, tracer, fn_state);
        } else {
            return block_279(state, tracer, fn_state);
        };
    }
    fn block_279<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_279_0: const #0u : u8
        let s_279_0: bool = false;
        // D s_279_1: write-var gs#374347 <= s_279_0
        fn_state.gs_374347 = s_279_0;
        // N s_279_2: jump b280
        return block_280(state, tracer, fn_state);
    }
    fn block_280<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_280_0: read-var gs#374347:u8
        let s_280_0: bool = fn_state.gs_374347;
        // D s_280_1: not s_280_0
        let s_280_1: bool = !s_280_0;
        // N s_280_2: branch s_280_1 b282 b281
        if s_280_1 {
            return block_282(state, tracer, fn_state);
        } else {
            return block_281(state, tracer, fn_state);
        };
    }
    fn block_281<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_281_0: const #712s : i
        let s_281_0: i128 = 712;
        // C s_281_1: const #14696u : u32
        let s_281_1: u32 = 14696;
        // N s_281_2: write-reg s_281_1 <= s_281_0
        let s_281_2: () = {
            state.write_register::<i128>(s_281_1 as isize, s_281_0);
            tracer.write_register(s_281_1 as isize, s_281_0);
        };
        // C s_281_3: const #5s : i
        let s_281_3: i128 = 5;
        // C s_281_4: const #3s : i
        let s_281_4: i128 = 3;
        // D s_281_5: read-var u#23274:u32
        let s_281_5: u32 = fn_state.u_23274;
        // D s_281_6: cast zx s_281_5 -> bv
        let s_281_6: Bits = Bits::new(s_281_5 as u128, 32u16);
        // D s_281_7: bit-extract s_281_6 s_281_3 s_281_4
        let s_281_7: Bits = (Bits::new(
            ((s_281_6) >> (s_281_3)).value(),
            u16::try_from(s_281_4).unwrap(),
        ));
        // D s_281_8: cast reint s_281_7 -> u8
        let s_281_8: u8 = (s_281_7.value() as u8);
        // C s_281_9: const #8s : i
        let s_281_9: i128 = 8;
        // C s_281_10: const #4s : i
        let s_281_10: i128 = 4;
        // D s_281_11: read-var u#23274:u32
        let s_281_11: u32 = fn_state.u_23274;
        // D s_281_12: cast zx s_281_11 -> bv
        let s_281_12: Bits = Bits::new(s_281_11 as u128, 32u16);
        // D s_281_13: bit-extract s_281_12 s_281_9 s_281_10
        let s_281_13: Bits = (Bits::new(
            ((s_281_12) >> (s_281_9)).value(),
            u16::try_from(s_281_10).unwrap(),
        ));
        // D s_281_14: cast reint s_281_13 -> u8
        let s_281_14: u8 = (s_281_13.value() as u8);
        // D s_281_15: call decode_pacia_aarch64_instrs_integer_pac_pacia_hint(s_281_8, s_281_14)
        let s_281_15: () = decode_pacia_aarch64_instrs_integer_pac_pacia_hint(
            state,
            tracer,
            s_281_8,
            s_281_14,
        );
        // N s_281_16: return
        return;
    }
    fn block_282<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_282_0: read-var merge#var.1:struct
        let s_282_0: u32 = fn_state.merge_var._1;
        // D s_282_1: write-var u#23278 <= s_282_0
        fn_state.u_23278 = s_282_0;
        // C s_282_2: const #10s : i
        let s_282_2: i128 = 10;
        // D s_282_3: read-var u#23278:u32
        let s_282_3: u32 = fn_state.u_23278;
        // D s_282_4: cast zx s_282_3 -> bv
        let s_282_4: Bits = Bits::new(s_282_3 as u128, 32u16);
        // C s_282_5: const #1s : i64
        let s_282_5: i64 = 1;
        // C s_282_6: cast zx s_282_5 -> i
        let s_282_6: i128 = (i128::try_from(s_282_5).unwrap());
        // C s_282_7: const #21s : i
        let s_282_7: i128 = 21;
        // C s_282_8: add s_282_7 s_282_6
        let s_282_8: i128 = (s_282_7 + s_282_6);
        // D s_282_9: bit-extract s_282_4 s_282_2 s_282_8
        let s_282_9: Bits = (Bits::new(
            ((s_282_4) >> (s_282_2)).value(),
            u16::try_from(s_282_8).unwrap(),
        ));
        // D s_282_10: cast reint s_282_9 -> u22
        let s_282_10: u32 = (s_282_9.value() as u32);
        // D s_282_11: cast zx s_282_10 -> bv
        let s_282_11: Bits = Bits::new(s_282_10 as u128, 22u16);
        // C s_282_12: const #3489992u : u22
        let s_282_12: u32 = 3489992;
        // C s_282_13: cast zx s_282_12 -> bv
        let s_282_13: Bits = Bits::new(s_282_12 as u128, 22u16);
        // D s_282_14: cmp-eq s_282_11 s_282_13
        let s_282_14: bool = ((s_282_11) == (s_282_13));
        // N s_282_15: branch s_282_14 b401 b283
        if s_282_14 {
            return block_401(state, tracer, fn_state);
        } else {
            return block_283(state, tracer, fn_state);
        };
    }
    fn block_283<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_283_0: const #0u : u8
        let s_283_0: bool = false;
        // D s_283_1: write-var gs#374362 <= s_283_0
        fn_state.gs_374362 = s_283_0;
        // N s_283_2: jump b284
        return block_284(state, tracer, fn_state);
    }
    fn block_284<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_284_0: read-var gs#374362:u8
        let s_284_0: bool = fn_state.gs_374362;
        // N s_284_1: branch s_284_0 b400 b285
        if s_284_0 {
            return block_400(state, tracer, fn_state);
        } else {
            return block_285(state, tracer, fn_state);
        };
    }
    fn block_285<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_285_0: const #0u : u8
        let s_285_0: bool = false;
        // D s_285_1: write-var gs#374364 <= s_285_0
        fn_state.gs_374364 = s_285_0;
        // N s_285_2: jump b286
        return block_286(state, tracer, fn_state);
    }
    fn block_286<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_286_0: read-var gs#374364:u8
        let s_286_0: bool = fn_state.gs_374364;
        // D s_286_1: not s_286_0
        let s_286_1: bool = !s_286_0;
        // N s_286_2: branch s_286_1 b288 b287
        if s_286_1 {
            return block_288(state, tracer, fn_state);
        } else {
            return block_287(state, tracer, fn_state);
        };
    }
    fn block_287<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_287_0: const #714s : i
        let s_287_0: i128 = 714;
        // C s_287_1: const #14696u : u32
        let s_287_1: u32 = 14696;
        // N s_287_2: write-reg s_287_1 <= s_287_0
        let s_287_2: () = {
            state.write_register::<i128>(s_287_1 as isize, s_287_0);
            tracer.write_register(s_287_1 as isize, s_287_0);
        };
        // C s_287_3: const #5s : i
        let s_287_3: i128 = 5;
        // C s_287_4: const #3s : i
        let s_287_4: i128 = 3;
        // D s_287_5: read-var u#23278:u32
        let s_287_5: u32 = fn_state.u_23278;
        // D s_287_6: cast zx s_287_5 -> bv
        let s_287_6: Bits = Bits::new(s_287_5 as u128, 32u16);
        // D s_287_7: bit-extract s_287_6 s_287_3 s_287_4
        let s_287_7: Bits = (Bits::new(
            ((s_287_6) >> (s_287_3)).value(),
            u16::try_from(s_287_4).unwrap(),
        ));
        // D s_287_8: cast reint s_287_7 -> u8
        let s_287_8: u8 = (s_287_7.value() as u8);
        // C s_287_9: const #8s : i
        let s_287_9: i128 = 8;
        // C s_287_10: const #4s : i
        let s_287_10: i128 = 4;
        // D s_287_11: read-var u#23278:u32
        let s_287_11: u32 = fn_state.u_23278;
        // D s_287_12: cast zx s_287_11 -> bv
        let s_287_12: Bits = Bits::new(s_287_11 as u128, 32u16);
        // D s_287_13: bit-extract s_287_12 s_287_9 s_287_10
        let s_287_13: Bits = (Bits::new(
            ((s_287_12) >> (s_287_9)).value(),
            u16::try_from(s_287_10).unwrap(),
        ));
        // D s_287_14: cast reint s_287_13 -> u8
        let s_287_14: u8 = (s_287_13.value() as u8);
        // D s_287_15: call decode_pacib_aarch64_instrs_integer_pac_pacib_hint(s_287_8, s_287_14)
        let s_287_15: () = decode_pacib_aarch64_instrs_integer_pac_pacib_hint(
            state,
            tracer,
            s_287_8,
            s_287_14,
        );
        // N s_287_16: return
        return;
    }
    fn block_288<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_288_0: read-var merge#var.1:struct
        let s_288_0: u32 = fn_state.merge_var._1;
        // D s_288_1: write-var u#23282 <= s_288_0
        fn_state.u_23282 = s_288_0;
        // D s_288_2: read-var u#23282:u32
        let s_288_2: u32 = fn_state.u_23282;
        // D s_288_3: cast zx s_288_2 -> bv
        let s_288_3: Bits = Bits::new(s_288_2 as u128, 32u16);
        // C s_288_4: const #3573756159u : u32
        let s_288_4: u32 = 3573756159;
        // C s_288_5: cast zx s_288_4 -> bv
        let s_288_5: Bits = Bits::new(s_288_4 as u128, 32u16);
        // D s_288_6: cmp-eq s_288_3 s_288_5
        let s_288_6: bool = ((s_288_3) == (s_288_5));
        // N s_288_7: branch s_288_6 b399 b289
        if s_288_6 {
            return block_399(state, tracer, fn_state);
        } else {
            return block_289(state, tracer, fn_state);
        };
    }
    fn block_289<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_289_0: const #0u : u8
        let s_289_0: bool = false;
        // D s_289_1: write-var gs#374373 <= s_289_0
        fn_state.gs_374373 = s_289_0;
        // N s_289_2: jump b290
        return block_290(state, tracer, fn_state);
    }
    fn block_290<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_290_0: read-var gs#374373:u8
        let s_290_0: bool = fn_state.gs_374373;
        // D s_290_1: not s_290_0
        let s_290_1: bool = !s_290_0;
        // N s_290_2: branch s_290_1 b303 b291
        if s_290_1 {
            return block_303(state, tracer, fn_state);
        } else {
            return block_291(state, tracer, fn_state);
        };
    }
    fn block_291<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_291_0: const #768s : i
        let s_291_0: i128 = 768;
        // C s_291_1: const #14696u : u32
        let s_291_1: u32 = 14696;
        // N s_291_2: write-reg s_291_1 <= s_291_0
        let s_291_2: () = {
            state.write_register::<i128>(s_291_1 as isize, s_291_0);
            tracer.write_register(s_291_1 as isize, s_291_0);
        };
        // C s_291_3: const #5s : i
        let s_291_3: i128 = 5;
        // C s_291_4: const #2s : i
        let s_291_4: i128 = 2;
        // D s_291_5: read-var u#23282:u32
        let s_291_5: u32 = fn_state.u_23282;
        // D s_291_6: cast zx s_291_5 -> bv
        let s_291_6: Bits = Bits::new(s_291_5 as u128, 32u16);
        // D s_291_7: bit-extract s_291_6 s_291_3 s_291_4
        let s_291_7: Bits = (Bits::new(
            ((s_291_6) >> (s_291_3)).value(),
            u16::try_from(s_291_4).unwrap(),
        ));
        // D s_291_8: cast reint s_291_7 -> u8
        let s_291_8: u8 = (s_291_7.value() as u8);
        // D s_291_9: write-var u#23283 <= s_291_8
        fn_state.u_23283 = s_291_8;
        // C s_291_10: const #8s : i
        let s_291_10: i128 = 8;
        // C s_291_11: const #4s : i
        let s_291_11: i128 = 4;
        // D s_291_12: read-var u#23282:u32
        let s_291_12: u32 = fn_state.u_23282;
        // D s_291_13: cast zx s_291_12 -> bv
        let s_291_13: Bits = Bits::new(s_291_12 as u128, 32u16);
        // D s_291_14: bit-extract s_291_13 s_291_10 s_291_11
        let s_291_14: Bits = (Bits::new(
            ((s_291_13) >> (s_291_10)).value(),
            u16::try_from(s_291_11).unwrap(),
        ));
        // D s_291_15: cast reint s_291_14 -> u8
        let s_291_15: u8 = (s_291_14.value() as u8);
        // D s_291_16: write-var u#23284 <= s_291_15
        fn_state.u_23284 = s_291_15;
        // C s_291_17: const #8s : i
        let s_291_17: i128 = 8;
        // D s_291_18: read-var u#23282:u32
        let s_291_18: u32 = fn_state.u_23282;
        // D s_291_19: cast zx s_291_18 -> bv
        let s_291_19: Bits = Bits::new(s_291_18 as u128, 32u16);
        // C s_291_20: const #1u : u64
        let s_291_20: u64 = 1;
        // D s_291_21: bit-extract s_291_19 s_291_17 s_291_20
        let s_291_21: Bits = (Bits::new(
            ((s_291_19) >> (s_291_17)).value(),
            u16::try_from(s_291_20).unwrap(),
        ));
        // D s_291_22: cast reint s_291_21 -> u8
        let s_291_22: bool = ((s_291_21.value()) != 0);
        // C s_291_23: const #0s : i
        let s_291_23: i128 = 0;
        // C s_291_24: const #0u : u64
        let s_291_24: u64 = 0;
        // D s_291_25: cast zx s_291_22 -> u64
        let s_291_25: u64 = (s_291_22 as u64);
        // C s_291_26: const #1u : u64
        let s_291_26: u64 = 1;
        // D s_291_27: and s_291_25 s_291_26
        let s_291_27: u64 = ((s_291_25) & (s_291_26));
        // D s_291_28: cmp-eq s_291_27 s_291_26
        let s_291_28: bool = ((s_291_27) == (s_291_26));
        // D s_291_29: lsl s_291_25 s_291_23
        let s_291_29: u64 = s_291_25 << s_291_23;
        // D s_291_30: or s_291_24 s_291_29
        let s_291_30: u64 = ((s_291_24) | (s_291_29));
        // D s_291_31: cmpl s_291_29
        let s_291_31: u64 = !s_291_29;
        // D s_291_32: and s_291_24 s_291_31
        let s_291_32: u64 = ((s_291_24) & (s_291_31));
        // D s_291_33: select s_291_28 s_291_30 s_291_32
        let s_291_33: u64 = if s_291_28 { s_291_30 } else { s_291_32 };
        // D s_291_34: cast trunc s_291_33 -> u8
        let s_291_34: bool = ((s_291_33) != 0);
        // D s_291_35: cast zx s_291_34 -> bv
        let s_291_35: Bits = Bits::new(s_291_34 as u128, 1u16);
        // C s_291_36: const #0u : u8
        let s_291_36: bool = false;
        // C s_291_37: cast zx s_291_36 -> bv
        let s_291_37: Bits = Bits::new(s_291_36 as u128, 1u16);
        // D s_291_38: cmp-ne s_291_35 s_291_37
        let s_291_38: bool = ((s_291_35) != (s_291_37));
        // N s_291_39: branch s_291_38 b302 b292
        if s_291_38 {
            return block_302(state, tracer, fn_state);
        } else {
            return block_292(state, tracer, fn_state);
        };
    }
    fn block_292<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_292_0: const #9s : i
        let s_292_0: i128 = 9;
        // D s_292_1: read-var u#23282:u32
        let s_292_1: u32 = fn_state.u_23282;
        // D s_292_2: cast zx s_292_1 -> bv
        let s_292_2: Bits = Bits::new(s_292_1 as u128, 32u16);
        // C s_292_3: const #1u : u64
        let s_292_3: u64 = 1;
        // D s_292_4: bit-extract s_292_2 s_292_0 s_292_3
        let s_292_4: Bits = (Bits::new(
            ((s_292_2) >> (s_292_0)).value(),
            u16::try_from(s_292_3).unwrap(),
        ));
        // D s_292_5: cast reint s_292_4 -> u8
        let s_292_5: bool = ((s_292_4.value()) != 0);
        // C s_292_6: const #0s : i
        let s_292_6: i128 = 0;
        // C s_292_7: const #0u : u64
        let s_292_7: u64 = 0;
        // D s_292_8: cast zx s_292_5 -> u64
        let s_292_8: u64 = (s_292_5 as u64);
        // C s_292_9: const #1u : u64
        let s_292_9: u64 = 1;
        // D s_292_10: and s_292_8 s_292_9
        let s_292_10: u64 = ((s_292_8) & (s_292_9));
        // D s_292_11: cmp-eq s_292_10 s_292_9
        let s_292_11: bool = ((s_292_10) == (s_292_9));
        // D s_292_12: lsl s_292_8 s_292_6
        let s_292_12: u64 = s_292_8 << s_292_6;
        // D s_292_13: or s_292_7 s_292_12
        let s_292_13: u64 = ((s_292_7) | (s_292_12));
        // D s_292_14: cmpl s_292_12
        let s_292_14: u64 = !s_292_12;
        // D s_292_15: and s_292_7 s_292_14
        let s_292_15: u64 = ((s_292_7) & (s_292_14));
        // D s_292_16: select s_292_11 s_292_13 s_292_15
        let s_292_16: u64 = if s_292_11 { s_292_13 } else { s_292_15 };
        // D s_292_17: cast trunc s_292_16 -> u8
        let s_292_17: bool = ((s_292_16) != 0);
        // D s_292_18: cast zx s_292_17 -> bv
        let s_292_18: Bits = Bits::new(s_292_17 as u128, 1u16);
        // C s_292_19: const #0u : u8
        let s_292_19: bool = false;
        // C s_292_20: cast zx s_292_19 -> bv
        let s_292_20: Bits = Bits::new(s_292_19 as u128, 1u16);
        // D s_292_21: cmp-ne s_292_18 s_292_20
        let s_292_21: bool = ((s_292_18) != (s_292_20));
        // D s_292_22: write-var gs#374384 <= s_292_21
        fn_state.gs_374384 = s_292_21;
        // N s_292_23: jump b293
        return block_293(state, tracer, fn_state);
    }
    fn block_293<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_293_0: read-var gs#374384:u8
        let s_293_0: bool = fn_state.gs_374384;
        // N s_293_1: branch s_293_0 b301 b294
        if s_293_0 {
            return block_301(state, tracer, fn_state);
        } else {
            return block_294(state, tracer, fn_state);
        };
    }
    fn block_294<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_294_0: const #10s : i
        let s_294_0: i128 = 10;
        // D s_294_1: read-var u#23282:u32
        let s_294_1: u32 = fn_state.u_23282;
        // D s_294_2: cast zx s_294_1 -> bv
        let s_294_2: Bits = Bits::new(s_294_1 as u128, 32u16);
        // C s_294_3: const #1u : u64
        let s_294_3: u64 = 1;
        // D s_294_4: bit-extract s_294_2 s_294_0 s_294_3
        let s_294_4: Bits = (Bits::new(
            ((s_294_2) >> (s_294_0)).value(),
            u16::try_from(s_294_3).unwrap(),
        ));
        // D s_294_5: cast reint s_294_4 -> u8
        let s_294_5: bool = ((s_294_4.value()) != 0);
        // C s_294_6: const #0s : i
        let s_294_6: i128 = 0;
        // C s_294_7: const #0u : u64
        let s_294_7: u64 = 0;
        // D s_294_8: cast zx s_294_5 -> u64
        let s_294_8: u64 = (s_294_5 as u64);
        // C s_294_9: const #1u : u64
        let s_294_9: u64 = 1;
        // D s_294_10: and s_294_8 s_294_9
        let s_294_10: u64 = ((s_294_8) & (s_294_9));
        // D s_294_11: cmp-eq s_294_10 s_294_9
        let s_294_11: bool = ((s_294_10) == (s_294_9));
        // D s_294_12: lsl s_294_8 s_294_6
        let s_294_12: u64 = s_294_8 << s_294_6;
        // D s_294_13: or s_294_7 s_294_12
        let s_294_13: u64 = ((s_294_7) | (s_294_12));
        // D s_294_14: cmpl s_294_12
        let s_294_14: u64 = !s_294_12;
        // D s_294_15: and s_294_7 s_294_14
        let s_294_15: u64 = ((s_294_7) & (s_294_14));
        // D s_294_16: select s_294_11 s_294_13 s_294_15
        let s_294_16: u64 = if s_294_11 { s_294_13 } else { s_294_15 };
        // D s_294_17: cast trunc s_294_16 -> u8
        let s_294_17: bool = ((s_294_16) != 0);
        // D s_294_18: cast zx s_294_17 -> bv
        let s_294_18: Bits = Bits::new(s_294_17 as u128, 1u16);
        // C s_294_19: const #0u : u8
        let s_294_19: bool = false;
        // C s_294_20: cast zx s_294_19 -> bv
        let s_294_20: Bits = Bits::new(s_294_19 as u128, 1u16);
        // D s_294_21: cmp-ne s_294_18 s_294_20
        let s_294_21: bool = ((s_294_18) != (s_294_20));
        // D s_294_22: write-var gs#374387 <= s_294_21
        fn_state.gs_374387 = s_294_21;
        // N s_294_23: jump b295
        return block_295(state, tracer, fn_state);
    }
    fn block_295<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_295_0: read-var gs#374387:u8
        let s_295_0: bool = fn_state.gs_374387;
        // N s_295_1: branch s_295_0 b300 b296
        if s_295_0 {
            return block_300(state, tracer, fn_state);
        } else {
            return block_296(state, tracer, fn_state);
        };
    }
    fn block_296<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_296_0: const #11s : i
        let s_296_0: i128 = 11;
        // D s_296_1: read-var u#23282:u32
        let s_296_1: u32 = fn_state.u_23282;
        // D s_296_2: cast zx s_296_1 -> bv
        let s_296_2: Bits = Bits::new(s_296_1 as u128, 32u16);
        // C s_296_3: const #1u : u64
        let s_296_3: u64 = 1;
        // D s_296_4: bit-extract s_296_2 s_296_0 s_296_3
        let s_296_4: Bits = (Bits::new(
            ((s_296_2) >> (s_296_0)).value(),
            u16::try_from(s_296_3).unwrap(),
        ));
        // D s_296_5: cast reint s_296_4 -> u8
        let s_296_5: bool = ((s_296_4.value()) != 0);
        // C s_296_6: const #0s : i
        let s_296_6: i128 = 0;
        // C s_296_7: const #0u : u64
        let s_296_7: u64 = 0;
        // D s_296_8: cast zx s_296_5 -> u64
        let s_296_8: u64 = (s_296_5 as u64);
        // C s_296_9: const #1u : u64
        let s_296_9: u64 = 1;
        // D s_296_10: and s_296_8 s_296_9
        let s_296_10: u64 = ((s_296_8) & (s_296_9));
        // D s_296_11: cmp-eq s_296_10 s_296_9
        let s_296_11: bool = ((s_296_10) == (s_296_9));
        // D s_296_12: lsl s_296_8 s_296_6
        let s_296_12: u64 = s_296_8 << s_296_6;
        // D s_296_13: or s_296_7 s_296_12
        let s_296_13: u64 = ((s_296_7) | (s_296_12));
        // D s_296_14: cmpl s_296_12
        let s_296_14: u64 = !s_296_12;
        // D s_296_15: and s_296_7 s_296_14
        let s_296_15: u64 = ((s_296_7) & (s_296_14));
        // D s_296_16: select s_296_11 s_296_13 s_296_15
        let s_296_16: u64 = if s_296_11 { s_296_13 } else { s_296_15 };
        // D s_296_17: cast trunc s_296_16 -> u8
        let s_296_17: bool = ((s_296_16) != 0);
        // D s_296_18: cast zx s_296_17 -> bv
        let s_296_18: Bits = Bits::new(s_296_17 as u128, 1u16);
        // C s_296_19: const #0u : u8
        let s_296_19: bool = false;
        // C s_296_20: cast zx s_296_19 -> bv
        let s_296_20: Bits = Bits::new(s_296_19 as u128, 1u16);
        // D s_296_21: cmp-ne s_296_18 s_296_20
        let s_296_21: bool = ((s_296_18) != (s_296_20));
        // D s_296_22: write-var gs#374390 <= s_296_21
        fn_state.gs_374390 = s_296_21;
        // N s_296_23: jump b297
        return block_297(state, tracer, fn_state);
    }
    fn block_297<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_297_0: read-var gs#374390:u8
        let s_297_0: bool = fn_state.gs_374390;
        // N s_297_1: branch s_297_0 b299 b298
        if s_297_0 {
            return block_299(state, tracer, fn_state);
        } else {
            return block_298(state, tracer, fn_state);
        };
    }
    fn block_298<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_298_0: read-var u#23283:u8
        let s_298_0: u8 = fn_state.u_23283;
        // D s_298_1: read-var u#23284:u8
        let s_298_1: u8 = fn_state.u_23284;
        // D s_298_2: call decode_sb_aarch64_instrs_system_barriers_sb(s_298_0, s_298_1)
        let s_298_2: () = decode_sb_aarch64_instrs_system_barriers_sb(
            state,
            tracer,
            s_298_0,
            s_298_1,
        );
        // N s_298_3: return
        return;
    }
    fn block_299<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_299_0: panic
        panic!("{:?}", ());
        // N s_299_1: return
        return;
    }
    fn block_300<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_300_0: const #1u : u8
        let s_300_0: bool = true;
        // D s_300_1: write-var gs#374390 <= s_300_0
        fn_state.gs_374390 = s_300_0;
        // N s_300_2: jump b297
        return block_297(state, tracer, fn_state);
    }
    fn block_301<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_301_0: const #1u : u8
        let s_301_0: bool = true;
        // D s_301_1: write-var gs#374387 <= s_301_0
        fn_state.gs_374387 = s_301_0;
        // N s_301_2: jump b295
        return block_295(state, tracer, fn_state);
    }
    fn block_302<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_302_0: const #1u : u8
        let s_302_0: bool = true;
        // D s_302_1: write-var gs#374384 <= s_302_0
        fn_state.gs_374384 = s_302_0;
        // N s_302_2: jump b293
        return block_293(state, tracer, fn_state);
    }
    fn block_303<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_303_0: read-var merge#var.1:struct
        let s_303_0: u32 = fn_state.merge_var._1;
        // D s_303_1: write-var u#23286 <= s_303_0
        fn_state.u_23286 = s_303_0;
        // C s_303_2: const #21s : i
        let s_303_2: i128 = 21;
        // D s_303_3: read-var u#23286:u32
        let s_303_3: u32 = fn_state.u_23286;
        // D s_303_4: cast zx s_303_3 -> bv
        let s_303_4: Bits = Bits::new(s_303_3 as u128, 32u16);
        // C s_303_5: const #1s : i64
        let s_303_5: i64 = 1;
        // C s_303_6: cast zx s_303_5 -> i
        let s_303_6: i128 = (i128::try_from(s_303_5).unwrap());
        // C s_303_7: const #10s : i
        let s_303_7: i128 = 10;
        // C s_303_8: add s_303_7 s_303_6
        let s_303_8: i128 = (s_303_7 + s_303_6);
        // D s_303_9: bit-extract s_303_4 s_303_2 s_303_8
        let s_303_9: Bits = (Bits::new(
            ((s_303_4) >> (s_303_2)).value(),
            u16::try_from(s_303_8).unwrap(),
        ));
        // D s_303_10: cast reint s_303_9 -> u11
        let s_303_10: u16 = (s_303_9.value() as u16);
        // D s_303_11: cast zx s_303_10 -> bv
        let s_303_11: Bits = Bits::new(s_303_10 as u128, 11u16);
        // C s_303_12: const #1696u : u11
        let s_303_12: u16 = 1696;
        // C s_303_13: cast zx s_303_12 -> bv
        let s_303_13: Bits = Bits::new(s_303_12 as u128, 11u16);
        // D s_303_14: cmp-eq s_303_11 s_303_13
        let s_303_14: bool = ((s_303_11) == (s_303_13));
        // N s_303_15: branch s_303_14 b398 b304
        if s_303_14 {
            return block_398(state, tracer, fn_state);
        } else {
            return block_304(state, tracer, fn_state);
        };
    }
    fn block_304<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_304_0: const #0u : u8
        let s_304_0: bool = false;
        // D s_304_1: write-var gs#374396 <= s_304_0
        fn_state.gs_374396 = s_304_0;
        // N s_304_2: jump b305
        return block_305(state, tracer, fn_state);
    }
    fn block_305<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_305_0: read-var gs#374396:u8
        let s_305_0: bool = fn_state.gs_374396;
        // N s_305_1: branch s_305_0 b397 b306
        if s_305_0 {
            return block_397(state, tracer, fn_state);
        } else {
            return block_306(state, tracer, fn_state);
        };
    }
    fn block_306<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_306_0: const #0u : u8
        let s_306_0: bool = false;
        // D s_306_1: write-var gs#374398 <= s_306_0
        fn_state.gs_374398 = s_306_0;
        // N s_306_2: jump b307
        return block_307(state, tracer, fn_state);
    }
    fn block_307<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_307_0: read-var gs#374398:u8
        let s_307_0: bool = fn_state.gs_374398;
        // D s_307_1: not s_307_0
        let s_307_1: bool = !s_307_0;
        // N s_307_2: branch s_307_1 b309 b308
        if s_307_1 {
            return block_309(state, tracer, fn_state);
        } else {
            return block_308(state, tracer, fn_state);
        };
    }
    fn block_308<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_308_0: const #846s : i
        let s_308_0: i128 = 846;
        // C s_308_1: const #14696u : u32
        let s_308_1: u32 = 14696;
        // N s_308_2: write-reg s_308_1 <= s_308_0
        let s_308_2: () = {
            state.write_register::<i128>(s_308_1 as isize, s_308_0);
            tracer.write_register(s_308_1 as isize, s_308_0);
        };
        // C s_308_3: const #5s : i
        let s_308_3: i128 = 5;
        // C s_308_4: const #16s : i
        let s_308_4: i128 = 16;
        // D s_308_5: read-var u#23286:u32
        let s_308_5: u32 = fn_state.u_23286;
        // D s_308_6: cast zx s_308_5 -> bv
        let s_308_6: Bits = Bits::new(s_308_5 as u128, 32u16);
        // D s_308_7: bit-extract s_308_6 s_308_3 s_308_4
        let s_308_7: Bits = (Bits::new(
            ((s_308_6) >> (s_308_3)).value(),
            u16::try_from(s_308_4).unwrap(),
        ));
        // D s_308_8: cast reint s_308_7 -> u16
        let s_308_8: u16 = (s_308_7.value() as u16);
        // D s_308_9: call decode_smc_aarch64_instrs_system_exceptions_runtime_smc(s_308_8)
        let s_308_9: () = decode_smc_aarch64_instrs_system_exceptions_runtime_smc(
            state,
            tracer,
            s_308_8,
        );
        // N s_308_10: return
        return;
    }
    fn block_309<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_309_0: read-var merge#var.1:struct
        let s_309_0: u32 = fn_state.merge_var._1;
        // D s_309_1: write-var u#23289 <= s_309_0
        fn_state.u_23289 = s_309_0;
        // C s_309_2: const #21s : i
        let s_309_2: i128 = 21;
        // D s_309_3: read-var u#23289:u32
        let s_309_3: u32 = fn_state.u_23289;
        // D s_309_4: cast zx s_309_3 -> bv
        let s_309_4: Bits = Bits::new(s_309_3 as u128, 32u16);
        // C s_309_5: const #1s : i64
        let s_309_5: i64 = 1;
        // C s_309_6: cast zx s_309_5 -> i
        let s_309_6: i128 = (i128::try_from(s_309_5).unwrap());
        // C s_309_7: const #10s : i
        let s_309_7: i128 = 10;
        // C s_309_8: add s_309_7 s_309_6
        let s_309_8: i128 = (s_309_7 + s_309_6);
        // D s_309_9: bit-extract s_309_4 s_309_2 s_309_8
        let s_309_9: Bits = (Bits::new(
            ((s_309_4) >> (s_309_2)).value(),
            u16::try_from(s_309_8).unwrap(),
        ));
        // D s_309_10: cast reint s_309_9 -> u11
        let s_309_10: u16 = (s_309_9.value() as u16);
        // D s_309_11: cast zx s_309_10 -> bv
        let s_309_11: Bits = Bits::new(s_309_10 as u128, 11u16);
        // C s_309_12: const #1696u : u11
        let s_309_12: u16 = 1696;
        // C s_309_13: cast zx s_309_12 -> bv
        let s_309_13: Bits = Bits::new(s_309_12 as u128, 11u16);
        // D s_309_14: cmp-eq s_309_11 s_309_13
        let s_309_14: bool = ((s_309_11) == (s_309_13));
        // N s_309_15: branch s_309_14 b396 b310
        if s_309_14 {
            return block_396(state, tracer, fn_state);
        } else {
            return block_310(state, tracer, fn_state);
        };
    }
    fn block_310<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_310_0: const #0u : u8
        let s_310_0: bool = false;
        // D s_310_1: write-var gs#374408 <= s_310_0
        fn_state.gs_374408 = s_310_0;
        // N s_310_2: jump b311
        return block_311(state, tracer, fn_state);
    }
    fn block_311<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_311_0: read-var gs#374408:u8
        let s_311_0: bool = fn_state.gs_374408;
        // N s_311_1: branch s_311_0 b395 b312
        if s_311_0 {
            return block_395(state, tracer, fn_state);
        } else {
            return block_312(state, tracer, fn_state);
        };
    }
    fn block_312<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_312_0: const #0u : u8
        let s_312_0: bool = false;
        // D s_312_1: write-var gs#374410 <= s_312_0
        fn_state.gs_374410 = s_312_0;
        // N s_312_2: jump b313
        return block_313(state, tracer, fn_state);
    }
    fn block_313<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_313_0: read-var gs#374410:u8
        let s_313_0: bool = fn_state.gs_374410;
        // D s_313_1: not s_313_0
        let s_313_1: bool = !s_313_0;
        // N s_313_2: branch s_313_1 b315 b314
        if s_313_1 {
            return block_315(state, tracer, fn_state);
        } else {
            return block_314(state, tracer, fn_state);
        };
    }
    fn block_314<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_314_0: const #999s : i
        let s_314_0: i128 = 999;
        // C s_314_1: const #14696u : u32
        let s_314_1: u32 = 14696;
        // N s_314_2: write-reg s_314_1 <= s_314_0
        let s_314_2: () = {
            state.write_register::<i128>(s_314_1 as isize, s_314_0);
            tracer.write_register(s_314_1 as isize, s_314_0);
        };
        // C s_314_3: const #5s : i
        let s_314_3: i128 = 5;
        // C s_314_4: const #16s : i
        let s_314_4: i128 = 16;
        // D s_314_5: read-var u#23289:u32
        let s_314_5: u32 = fn_state.u_23289;
        // D s_314_6: cast zx s_314_5 -> bv
        let s_314_6: Bits = Bits::new(s_314_5 as u128, 32u16);
        // D s_314_7: bit-extract s_314_6 s_314_3 s_314_4
        let s_314_7: Bits = (Bits::new(
            ((s_314_6) >> (s_314_3)).value(),
            u16::try_from(s_314_4).unwrap(),
        ));
        // D s_314_8: cast reint s_314_7 -> u16
        let s_314_8: u16 = (s_314_7.value() as u16);
        // D s_314_9: call decode_svc_aarch64_instrs_system_exceptions_runtime_svc(s_314_8)
        let s_314_9: () = decode_svc_aarch64_instrs_system_exceptions_runtime_svc(
            state,
            tracer,
            s_314_8,
        );
        // N s_314_10: return
        return;
    }
    fn block_315<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_315_0: read-var merge#var.1:struct
        let s_315_0: u32 = fn_state.merge_var._1;
        // D s_315_1: write-var u#23292 <= s_315_0
        fn_state.u_23292 = s_315_0;
        // C s_315_2: const #19s : i
        let s_315_2: i128 = 19;
        // D s_315_3: read-var u#23292:u32
        let s_315_3: u32 = fn_state.u_23292;
        // D s_315_4: cast zx s_315_3 -> bv
        let s_315_4: Bits = Bits::new(s_315_3 as u128, 32u16);
        // C s_315_5: const #1s : i64
        let s_315_5: i64 = 1;
        // C s_315_6: cast zx s_315_5 -> i
        let s_315_6: i128 = (i128::try_from(s_315_5).unwrap());
        // C s_315_7: const #12s : i
        let s_315_7: i128 = 12;
        // C s_315_8: add s_315_7 s_315_6
        let s_315_8: i128 = (s_315_7 + s_315_6);
        // D s_315_9: bit-extract s_315_4 s_315_2 s_315_8
        let s_315_9: Bits = (Bits::new(
            ((s_315_4) >> (s_315_2)).value(),
            u16::try_from(s_315_8).unwrap(),
        ));
        // D s_315_10: cast reint s_315_9 -> u13
        let s_315_10: u16 = (s_315_9.value() as u16);
        // D s_315_11: cast zx s_315_10 -> bv
        let s_315_11: Bits = Bits::new(s_315_10 as u128, 13u16);
        // C s_315_12: const #6817u : u13
        let s_315_12: u16 = 6817;
        // C s_315_13: cast zx s_315_12 -> bv
        let s_315_13: Bits = Bits::new(s_315_12 as u128, 13u16);
        // D s_315_14: cmp-eq s_315_11 s_315_13
        let s_315_14: bool = ((s_315_11) == (s_315_13));
        // N s_315_15: branch s_315_14 b394 b316
        if s_315_14 {
            return block_394(state, tracer, fn_state);
        } else {
            return block_316(state, tracer, fn_state);
        };
    }
    fn block_316<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_316_0: const #0u : u8
        let s_316_0: bool = false;
        // D s_316_1: write-var gs#374419 <= s_316_0
        fn_state.gs_374419 = s_316_0;
        // N s_316_2: jump b317
        return block_317(state, tracer, fn_state);
    }
    fn block_317<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_317_0: read-var gs#374419:u8
        let s_317_0: bool = fn_state.gs_374419;
        // D s_317_1: not s_317_0
        let s_317_1: bool = !s_317_0;
        // N s_317_2: branch s_317_1 b319 b318
        if s_317_1 {
            return block_319(state, tracer, fn_state);
        } else {
            return block_318(state, tracer, fn_state);
        };
    }
    fn block_318<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_318_0: const #1004s : i
        let s_318_0: i128 = 1004;
        // C s_318_1: const #14696u : u32
        let s_318_1: u32 = 14696;
        // N s_318_2: write-reg s_318_1 <= s_318_0
        let s_318_2: () = {
            state.write_register::<i128>(s_318_1 as isize, s_318_0);
            tracer.write_register(s_318_1 as isize, s_318_0);
        };
        // C s_318_3: const #0s : i
        let s_318_3: i128 = 0;
        // C s_318_4: const #5s : i
        let s_318_4: i128 = 5;
        // D s_318_5: read-var u#23292:u32
        let s_318_5: u32 = fn_state.u_23292;
        // D s_318_6: cast zx s_318_5 -> bv
        let s_318_6: Bits = Bits::new(s_318_5 as u128, 32u16);
        // D s_318_7: bit-extract s_318_6 s_318_3 s_318_4
        let s_318_7: Bits = (Bits::new(
            ((s_318_6) >> (s_318_3)).value(),
            u16::try_from(s_318_4).unwrap(),
        ));
        // D s_318_8: cast reint s_318_7 -> u8
        let s_318_8: u8 = (s_318_7.value() as u8);
        // C s_318_9: const #5s : i
        let s_318_9: i128 = 5;
        // C s_318_10: const #3s : i
        let s_318_10: i128 = 3;
        // D s_318_11: read-var u#23292:u32
        let s_318_11: u32 = fn_state.u_23292;
        // D s_318_12: cast zx s_318_11 -> bv
        let s_318_12: Bits = Bits::new(s_318_11 as u128, 32u16);
        // D s_318_13: bit-extract s_318_12 s_318_9 s_318_10
        let s_318_13: Bits = (Bits::new(
            ((s_318_12) >> (s_318_9)).value(),
            u16::try_from(s_318_10).unwrap(),
        ));
        // D s_318_14: cast reint s_318_13 -> u8
        let s_318_14: u8 = (s_318_13.value() as u8);
        // C s_318_15: const #8s : i
        let s_318_15: i128 = 8;
        // C s_318_16: const #4s : i
        let s_318_16: i128 = 4;
        // D s_318_17: read-var u#23292:u32
        let s_318_17: u32 = fn_state.u_23292;
        // D s_318_18: cast zx s_318_17 -> bv
        let s_318_18: Bits = Bits::new(s_318_17 as u128, 32u16);
        // D s_318_19: bit-extract s_318_18 s_318_15 s_318_16
        let s_318_19: Bits = (Bits::new(
            ((s_318_18) >> (s_318_15)).value(),
            u16::try_from(s_318_16).unwrap(),
        ));
        // D s_318_20: cast reint s_318_19 -> u8
        let s_318_20: u8 = (s_318_19.value() as u8);
        // C s_318_21: const #12s : i
        let s_318_21: i128 = 12;
        // C s_318_22: const #4s : i
        let s_318_22: i128 = 4;
        // D s_318_23: read-var u#23292:u32
        let s_318_23: u32 = fn_state.u_23292;
        // D s_318_24: cast zx s_318_23 -> bv
        let s_318_24: Bits = Bits::new(s_318_23 as u128, 32u16);
        // D s_318_25: bit-extract s_318_24 s_318_21 s_318_22
        let s_318_25: Bits = (Bits::new(
            ((s_318_24) >> (s_318_21)).value(),
            u16::try_from(s_318_22).unwrap(),
        ));
        // D s_318_26: cast reint s_318_25 -> u8
        let s_318_26: u8 = (s_318_25.value() as u8);
        // C s_318_27: const #16s : i
        let s_318_27: i128 = 16;
        // C s_318_28: const #3s : i
        let s_318_28: i128 = 3;
        // D s_318_29: read-var u#23292:u32
        let s_318_29: u32 = fn_state.u_23292;
        // D s_318_30: cast zx s_318_29 -> bv
        let s_318_30: Bits = Bits::new(s_318_29 as u128, 32u16);
        // D s_318_31: bit-extract s_318_30 s_318_27 s_318_28
        let s_318_31: Bits = (Bits::new(
            ((s_318_30) >> (s_318_27)).value(),
            u16::try_from(s_318_28).unwrap(),
        ));
        // D s_318_32: cast reint s_318_31 -> u8
        let s_318_32: u8 = (s_318_31.value() as u8);
        // C s_318_33: const #21s : i
        let s_318_33: i128 = 21;
        // C s_318_34: const #1s : i
        let s_318_34: i128 = 1;
        // D s_318_35: read-var u#23292:u32
        let s_318_35: u32 = fn_state.u_23292;
        // D s_318_36: cast zx s_318_35 -> bv
        let s_318_36: Bits = Bits::new(s_318_35 as u128, 32u16);
        // D s_318_37: bit-extract s_318_36 s_318_33 s_318_34
        let s_318_37: Bits = (Bits::new(
            ((s_318_36) >> (s_318_33)).value(),
            u16::try_from(s_318_34).unwrap(),
        ));
        // D s_318_38: cast reint s_318_37 -> u8
        let s_318_38: bool = ((s_318_37.value()) != 0);
        // D s_318_39: call decode_sys_aarch64_instrs_system_sysops(s_318_8, s_318_14, s_318_20, s_318_26, s_318_32, s_318_38)
        let s_318_39: () = decode_sys_aarch64_instrs_system_sysops(
            state,
            tracer,
            s_318_8,
            s_318_14,
            s_318_20,
            s_318_26,
            s_318_32,
            s_318_38,
        );
        // N s_318_40: return
        return;
    }
    fn block_319<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_319_0: read-var merge#var.1:struct
        let s_319_0: u32 = fn_state.merge_var._1;
        // D s_319_1: write-var u#23300 <= s_319_0
        fn_state.u_23300 = s_319_0;
        // C s_319_2: const #19s : i
        let s_319_2: i128 = 19;
        // D s_319_3: read-var u#23300:u32
        let s_319_3: u32 = fn_state.u_23300;
        // D s_319_4: cast zx s_319_3 -> bv
        let s_319_4: Bits = Bits::new(s_319_3 as u128, 32u16);
        // C s_319_5: const #1s : i64
        let s_319_5: i64 = 1;
        // C s_319_6: cast zx s_319_5 -> i
        let s_319_6: i128 = (i128::try_from(s_319_5).unwrap());
        // C s_319_7: const #12s : i
        let s_319_7: i128 = 12;
        // C s_319_8: add s_319_7 s_319_6
        let s_319_8: i128 = (s_319_7 + s_319_6);
        // D s_319_9: bit-extract s_319_4 s_319_2 s_319_8
        let s_319_9: Bits = (Bits::new(
            ((s_319_4) >> (s_319_2)).value(),
            u16::try_from(s_319_8).unwrap(),
        ));
        // D s_319_10: cast reint s_319_9 -> u13
        let s_319_10: u16 = (s_319_9.value() as u16);
        // D s_319_11: cast zx s_319_10 -> bv
        let s_319_11: Bits = Bits::new(s_319_10 as u128, 13u16);
        // C s_319_12: const #6821u : u13
        let s_319_12: u16 = 6821;
        // C s_319_13: cast zx s_319_12 -> bv
        let s_319_13: Bits = Bits::new(s_319_12 as u128, 13u16);
        // D s_319_14: cmp-eq s_319_11 s_319_13
        let s_319_14: bool = ((s_319_11) == (s_319_13));
        // N s_319_15: branch s_319_14 b393 b320
        if s_319_14 {
            return block_393(state, tracer, fn_state);
        } else {
            return block_320(state, tracer, fn_state);
        };
    }
    fn block_320<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_320_0: const #0u : u8
        let s_320_0: bool = false;
        // D s_320_1: write-var gs#374438 <= s_320_0
        fn_state.gs_374438 = s_320_0;
        // N s_320_2: jump b321
        return block_321(state, tracer, fn_state);
    }
    fn block_321<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_321_0: read-var gs#374438:u8
        let s_321_0: bool = fn_state.gs_374438;
        // D s_321_1: not s_321_0
        let s_321_1: bool = !s_321_0;
        // N s_321_2: branch s_321_1 b323 b322
        if s_321_1 {
            return block_323(state, tracer, fn_state);
        } else {
            return block_322(state, tracer, fn_state);
        };
    }
    fn block_322<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_322_0: const #1005s : i
        let s_322_0: i128 = 1005;
        // C s_322_1: const #14696u : u32
        let s_322_1: u32 = 14696;
        // N s_322_2: write-reg s_322_1 <= s_322_0
        let s_322_2: () = {
            state.write_register::<i128>(s_322_1 as isize, s_322_0);
            tracer.write_register(s_322_1 as isize, s_322_0);
        };
        // C s_322_3: const #0s : i
        let s_322_3: i128 = 0;
        // C s_322_4: const #5s : i
        let s_322_4: i128 = 5;
        // D s_322_5: read-var u#23300:u32
        let s_322_5: u32 = fn_state.u_23300;
        // D s_322_6: cast zx s_322_5 -> bv
        let s_322_6: Bits = Bits::new(s_322_5 as u128, 32u16);
        // D s_322_7: bit-extract s_322_6 s_322_3 s_322_4
        let s_322_7: Bits = (Bits::new(
            ((s_322_6) >> (s_322_3)).value(),
            u16::try_from(s_322_4).unwrap(),
        ));
        // D s_322_8: cast reint s_322_7 -> u8
        let s_322_8: u8 = (s_322_7.value() as u8);
        // C s_322_9: const #5s : i
        let s_322_9: i128 = 5;
        // C s_322_10: const #3s : i
        let s_322_10: i128 = 3;
        // D s_322_11: read-var u#23300:u32
        let s_322_11: u32 = fn_state.u_23300;
        // D s_322_12: cast zx s_322_11 -> bv
        let s_322_12: Bits = Bits::new(s_322_11 as u128, 32u16);
        // D s_322_13: bit-extract s_322_12 s_322_9 s_322_10
        let s_322_13: Bits = (Bits::new(
            ((s_322_12) >> (s_322_9)).value(),
            u16::try_from(s_322_10).unwrap(),
        ));
        // D s_322_14: cast reint s_322_13 -> u8
        let s_322_14: u8 = (s_322_13.value() as u8);
        // C s_322_15: const #8s : i
        let s_322_15: i128 = 8;
        // C s_322_16: const #4s : i
        let s_322_16: i128 = 4;
        // D s_322_17: read-var u#23300:u32
        let s_322_17: u32 = fn_state.u_23300;
        // D s_322_18: cast zx s_322_17 -> bv
        let s_322_18: Bits = Bits::new(s_322_17 as u128, 32u16);
        // D s_322_19: bit-extract s_322_18 s_322_15 s_322_16
        let s_322_19: Bits = (Bits::new(
            ((s_322_18) >> (s_322_15)).value(),
            u16::try_from(s_322_16).unwrap(),
        ));
        // D s_322_20: cast reint s_322_19 -> u8
        let s_322_20: u8 = (s_322_19.value() as u8);
        // C s_322_21: const #12s : i
        let s_322_21: i128 = 12;
        // C s_322_22: const #4s : i
        let s_322_22: i128 = 4;
        // D s_322_23: read-var u#23300:u32
        let s_322_23: u32 = fn_state.u_23300;
        // D s_322_24: cast zx s_322_23 -> bv
        let s_322_24: Bits = Bits::new(s_322_23 as u128, 32u16);
        // D s_322_25: bit-extract s_322_24 s_322_21 s_322_22
        let s_322_25: Bits = (Bits::new(
            ((s_322_24) >> (s_322_21)).value(),
            u16::try_from(s_322_22).unwrap(),
        ));
        // D s_322_26: cast reint s_322_25 -> u8
        let s_322_26: u8 = (s_322_25.value() as u8);
        // C s_322_27: const #16s : i
        let s_322_27: i128 = 16;
        // C s_322_28: const #3s : i
        let s_322_28: i128 = 3;
        // D s_322_29: read-var u#23300:u32
        let s_322_29: u32 = fn_state.u_23300;
        // D s_322_30: cast zx s_322_29 -> bv
        let s_322_30: Bits = Bits::new(s_322_29 as u128, 32u16);
        // D s_322_31: bit-extract s_322_30 s_322_27 s_322_28
        let s_322_31: Bits = (Bits::new(
            ((s_322_30) >> (s_322_27)).value(),
            u16::try_from(s_322_28).unwrap(),
        ));
        // D s_322_32: cast reint s_322_31 -> u8
        let s_322_32: u8 = (s_322_31.value() as u8);
        // C s_322_33: const #21s : i
        let s_322_33: i128 = 21;
        // C s_322_34: const #1s : i
        let s_322_34: i128 = 1;
        // D s_322_35: read-var u#23300:u32
        let s_322_35: u32 = fn_state.u_23300;
        // D s_322_36: cast zx s_322_35 -> bv
        let s_322_36: Bits = Bits::new(s_322_35 as u128, 32u16);
        // D s_322_37: bit-extract s_322_36 s_322_33 s_322_34
        let s_322_37: Bits = (Bits::new(
            ((s_322_36) >> (s_322_33)).value(),
            u16::try_from(s_322_34).unwrap(),
        ));
        // D s_322_38: cast reint s_322_37 -> u8
        let s_322_38: bool = ((s_322_37.value()) != 0);
        // D s_322_39: call decode_sysl_aarch64_instrs_system_sysops(s_322_8, s_322_14, s_322_20, s_322_26, s_322_32, s_322_38)
        let s_322_39: () = decode_sysl_aarch64_instrs_system_sysops(
            state,
            tracer,
            s_322_8,
            s_322_14,
            s_322_20,
            s_322_26,
            s_322_32,
            s_322_38,
        );
        // N s_322_40: return
        return;
    }
    fn block_323<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_323_0: read-var merge#var.1:struct
        let s_323_0: u32 = fn_state.merge_var._1;
        // D s_323_1: write-var u#23308 <= s_323_0
        fn_state.u_23308 = s_323_0;
        // C s_323_2: const #19s : i
        let s_323_2: i128 = 19;
        // D s_323_3: read-var u#23308:u32
        let s_323_3: u32 = fn_state.u_23308;
        // D s_323_4: cast zx s_323_3 -> bv
        let s_323_4: Bits = Bits::new(s_323_3 as u128, 32u16);
        // C s_323_5: const #1s : i64
        let s_323_5: i64 = 1;
        // C s_323_6: cast zx s_323_5 -> i
        let s_323_6: i128 = (i128::try_from(s_323_5).unwrap());
        // C s_323_7: const #12s : i
        let s_323_7: i128 = 12;
        // C s_323_8: add s_323_7 s_323_6
        let s_323_8: i128 = (s_323_7 + s_323_6);
        // D s_323_9: bit-extract s_323_4 s_323_2 s_323_8
        let s_323_9: Bits = (Bits::new(
            ((s_323_4) >> (s_323_2)).value(),
            u16::try_from(s_323_8).unwrap(),
        ));
        // D s_323_10: cast reint s_323_9 -> u13
        let s_323_10: u16 = (s_323_9.value() as u16);
        // D s_323_11: cast zx s_323_10 -> bv
        let s_323_11: Bits = Bits::new(s_323_10 as u128, 13u16);
        // C s_323_12: const #6825u : u13
        let s_323_12: u16 = 6825;
        // C s_323_13: cast zx s_323_12 -> bv
        let s_323_13: Bits = Bits::new(s_323_12 as u128, 13u16);
        // D s_323_14: cmp-eq s_323_11 s_323_13
        let s_323_14: bool = ((s_323_11) == (s_323_13));
        // N s_323_15: branch s_323_14 b392 b324
        if s_323_14 {
            return block_392(state, tracer, fn_state);
        } else {
            return block_324(state, tracer, fn_state);
        };
    }
    fn block_324<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_324_0: const #0u : u8
        let s_324_0: bool = false;
        // D s_324_1: write-var gs#374457 <= s_324_0
        fn_state.gs_374457 = s_324_0;
        // N s_324_2: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_325<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_325_0: read-var gs#374457:u8
        let s_325_0: bool = fn_state.gs_374457;
        // D s_325_1: not s_325_0
        let s_325_1: bool = !s_325_0;
        // N s_325_2: branch s_325_1 b327 b326
        if s_325_1 {
            return block_327(state, tracer, fn_state);
        } else {
            return block_326(state, tracer, fn_state);
        };
    }
    fn block_326<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_326_0: const #1006s : i
        let s_326_0: i128 = 1006;
        // C s_326_1: const #14696u : u32
        let s_326_1: u32 = 14696;
        // N s_326_2: write-reg s_326_1 <= s_326_0
        let s_326_2: () = {
            state.write_register::<i128>(s_326_1 as isize, s_326_0);
            tracer.write_register(s_326_1 as isize, s_326_0);
        };
        // C s_326_3: const #0s : i
        let s_326_3: i128 = 0;
        // C s_326_4: const #5s : i
        let s_326_4: i128 = 5;
        // D s_326_5: read-var u#23308:u32
        let s_326_5: u32 = fn_state.u_23308;
        // D s_326_6: cast zx s_326_5 -> bv
        let s_326_6: Bits = Bits::new(s_326_5 as u128, 32u16);
        // D s_326_7: bit-extract s_326_6 s_326_3 s_326_4
        let s_326_7: Bits = (Bits::new(
            ((s_326_6) >> (s_326_3)).value(),
            u16::try_from(s_326_4).unwrap(),
        ));
        // D s_326_8: cast reint s_326_7 -> u8
        let s_326_8: u8 = (s_326_7.value() as u8);
        // C s_326_9: const #5s : i
        let s_326_9: i128 = 5;
        // C s_326_10: const #3s : i
        let s_326_10: i128 = 3;
        // D s_326_11: read-var u#23308:u32
        let s_326_11: u32 = fn_state.u_23308;
        // D s_326_12: cast zx s_326_11 -> bv
        let s_326_12: Bits = Bits::new(s_326_11 as u128, 32u16);
        // D s_326_13: bit-extract s_326_12 s_326_9 s_326_10
        let s_326_13: Bits = (Bits::new(
            ((s_326_12) >> (s_326_9)).value(),
            u16::try_from(s_326_10).unwrap(),
        ));
        // D s_326_14: cast reint s_326_13 -> u8
        let s_326_14: u8 = (s_326_13.value() as u8);
        // C s_326_15: const #8s : i
        let s_326_15: i128 = 8;
        // C s_326_16: const #4s : i
        let s_326_16: i128 = 4;
        // D s_326_17: read-var u#23308:u32
        let s_326_17: u32 = fn_state.u_23308;
        // D s_326_18: cast zx s_326_17 -> bv
        let s_326_18: Bits = Bits::new(s_326_17 as u128, 32u16);
        // D s_326_19: bit-extract s_326_18 s_326_15 s_326_16
        let s_326_19: Bits = (Bits::new(
            ((s_326_18) >> (s_326_15)).value(),
            u16::try_from(s_326_16).unwrap(),
        ));
        // D s_326_20: cast reint s_326_19 -> u8
        let s_326_20: u8 = (s_326_19.value() as u8);
        // C s_326_21: const #12s : i
        let s_326_21: i128 = 12;
        // C s_326_22: const #4s : i
        let s_326_22: i128 = 4;
        // D s_326_23: read-var u#23308:u32
        let s_326_23: u32 = fn_state.u_23308;
        // D s_326_24: cast zx s_326_23 -> bv
        let s_326_24: Bits = Bits::new(s_326_23 as u128, 32u16);
        // D s_326_25: bit-extract s_326_24 s_326_21 s_326_22
        let s_326_25: Bits = (Bits::new(
            ((s_326_24) >> (s_326_21)).value(),
            u16::try_from(s_326_22).unwrap(),
        ));
        // D s_326_26: cast reint s_326_25 -> u8
        let s_326_26: u8 = (s_326_25.value() as u8);
        // C s_326_27: const #16s : i
        let s_326_27: i128 = 16;
        // C s_326_28: const #3s : i
        let s_326_28: i128 = 3;
        // D s_326_29: read-var u#23308:u32
        let s_326_29: u32 = fn_state.u_23308;
        // D s_326_30: cast zx s_326_29 -> bv
        let s_326_30: Bits = Bits::new(s_326_29 as u128, 32u16);
        // D s_326_31: bit-extract s_326_30 s_326_27 s_326_28
        let s_326_31: Bits = (Bits::new(
            ((s_326_30) >> (s_326_27)).value(),
            u16::try_from(s_326_28).unwrap(),
        ));
        // D s_326_32: cast reint s_326_31 -> u8
        let s_326_32: u8 = (s_326_31.value() as u8);
        // C s_326_33: const #21s : i
        let s_326_33: i128 = 21;
        // C s_326_34: const #1s : i
        let s_326_34: i128 = 1;
        // D s_326_35: read-var u#23308:u32
        let s_326_35: u32 = fn_state.u_23308;
        // D s_326_36: cast zx s_326_35 -> bv
        let s_326_36: Bits = Bits::new(s_326_35 as u128, 32u16);
        // D s_326_37: bit-extract s_326_36 s_326_33 s_326_34
        let s_326_37: Bits = (Bits::new(
            ((s_326_36) >> (s_326_33)).value(),
            u16::try_from(s_326_34).unwrap(),
        ));
        // D s_326_38: cast reint s_326_37 -> u8
        let s_326_38: bool = ((s_326_37.value()) != 0);
        // D s_326_39: call decode_sysp_aarch64_instrs_system_sysops_128(s_326_8, s_326_14, s_326_20, s_326_26, s_326_32, s_326_38)
        let s_326_39: () = decode_sysp_aarch64_instrs_system_sysops_128(
            state,
            tracer,
            s_326_8,
            s_326_14,
            s_326_20,
            s_326_26,
            s_326_32,
            s_326_38,
        );
        // N s_326_40: return
        return;
    }
    fn block_327<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_327_0: read-var merge#var.1:struct
        let s_327_0: u32 = fn_state.merge_var._1;
        // D s_327_1: write-var u#23316 <= s_327_0
        fn_state.u_23316 = s_327_0;
        // C s_327_2: const #24s : i
        let s_327_2: i128 = 24;
        // D s_327_3: read-var u#23316:u32
        let s_327_3: u32 = fn_state.u_23316;
        // D s_327_4: cast zx s_327_3 -> bv
        let s_327_4: Bits = Bits::new(s_327_3 as u128, 32u16);
        // C s_327_5: const #1s : i64
        let s_327_5: i64 = 1;
        // C s_327_6: cast zx s_327_5 -> i
        let s_327_6: i128 = (i128::try_from(s_327_5).unwrap());
        // C s_327_7: const #6s : i
        let s_327_7: i128 = 6;
        // C s_327_8: add s_327_7 s_327_6
        let s_327_8: i128 = (s_327_7 + s_327_6);
        // D s_327_9: bit-extract s_327_4 s_327_2 s_327_8
        let s_327_9: Bits = (Bits::new(
            ((s_327_4) >> (s_327_2)).value(),
            u16::try_from(s_327_8).unwrap(),
        ));
        // D s_327_10: cast reint s_327_9 -> u8
        let s_327_10: u8 = (s_327_9.value() as u8);
        // D s_327_11: cast zx s_327_10 -> bv
        let s_327_11: Bits = Bits::new(s_327_10 as u128, 7u16);
        // C s_327_12: const #55u : u8
        let s_327_12: u8 = 55;
        // C s_327_13: cast zx s_327_12 -> bv
        let s_327_13: Bits = Bits::new(s_327_12 as u128, 7u16);
        // D s_327_14: cmp-eq s_327_11 s_327_13
        let s_327_14: bool = ((s_327_11) == (s_327_13));
        // N s_327_15: branch s_327_14 b391 b328
        if s_327_14 {
            return block_391(state, tracer, fn_state);
        } else {
            return block_328(state, tracer, fn_state);
        };
    }
    fn block_328<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_328_0: const #0u : u8
        let s_328_0: bool = false;
        // D s_328_1: write-var gs#374476 <= s_328_0
        fn_state.gs_374476 = s_328_0;
        // N s_328_2: jump b329
        return block_329(state, tracer, fn_state);
    }
    fn block_329<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_329_0: read-var gs#374476:u8
        let s_329_0: bool = fn_state.gs_374476;
        // D s_329_1: not s_329_0
        let s_329_1: bool = !s_329_0;
        // N s_329_2: branch s_329_1 b331 b330
        if s_329_1 {
            return block_331(state, tracer, fn_state);
        } else {
            return block_330(state, tracer, fn_state);
        };
    }
    fn block_330<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_330_0: const #1009s : i
        let s_330_0: i128 = 1009;
        // C s_330_1: const #14696u : u32
        let s_330_1: u32 = 14696;
        // N s_330_2: write-reg s_330_1 <= s_330_0
        let s_330_2: () = {
            state.write_register::<i128>(s_330_1 as isize, s_330_0);
            tracer.write_register(s_330_1 as isize, s_330_0);
        };
        // C s_330_3: const #0s : i
        let s_330_3: i128 = 0;
        // C s_330_4: const #5s : i
        let s_330_4: i128 = 5;
        // D s_330_5: read-var u#23316:u32
        let s_330_5: u32 = fn_state.u_23316;
        // D s_330_6: cast zx s_330_5 -> bv
        let s_330_6: Bits = Bits::new(s_330_5 as u128, 32u16);
        // D s_330_7: bit-extract s_330_6 s_330_3 s_330_4
        let s_330_7: Bits = (Bits::new(
            ((s_330_6) >> (s_330_3)).value(),
            u16::try_from(s_330_4).unwrap(),
        ));
        // D s_330_8: cast reint s_330_7 -> u8
        let s_330_8: u8 = (s_330_7.value() as u8);
        // C s_330_9: const #5s : i
        let s_330_9: i128 = 5;
        // C s_330_10: const #14s : i
        let s_330_10: i128 = 14;
        // D s_330_11: read-var u#23316:u32
        let s_330_11: u32 = fn_state.u_23316;
        // D s_330_12: cast zx s_330_11 -> bv
        let s_330_12: Bits = Bits::new(s_330_11 as u128, 32u16);
        // D s_330_13: bit-extract s_330_12 s_330_9 s_330_10
        let s_330_13: Bits = (Bits::new(
            ((s_330_12) >> (s_330_9)).value(),
            u16::try_from(s_330_10).unwrap(),
        ));
        // D s_330_14: cast reint s_330_13 -> u14
        let s_330_14: u16 = (s_330_13.value() as u16);
        // C s_330_15: const #19s : i
        let s_330_15: i128 = 19;
        // C s_330_16: const #5s : i
        let s_330_16: i128 = 5;
        // D s_330_17: read-var u#23316:u32
        let s_330_17: u32 = fn_state.u_23316;
        // D s_330_18: cast zx s_330_17 -> bv
        let s_330_18: Bits = Bits::new(s_330_17 as u128, 32u16);
        // D s_330_19: bit-extract s_330_18 s_330_15 s_330_16
        let s_330_19: Bits = (Bits::new(
            ((s_330_18) >> (s_330_15)).value(),
            u16::try_from(s_330_16).unwrap(),
        ));
        // D s_330_20: cast reint s_330_19 -> u8
        let s_330_20: u8 = (s_330_19.value() as u8);
        // C s_330_21: const #24s : i
        let s_330_21: i128 = 24;
        // C s_330_22: const #1s : i
        let s_330_22: i128 = 1;
        // D s_330_23: read-var u#23316:u32
        let s_330_23: u32 = fn_state.u_23316;
        // D s_330_24: cast zx s_330_23 -> bv
        let s_330_24: Bits = Bits::new(s_330_23 as u128, 32u16);
        // D s_330_25: bit-extract s_330_24 s_330_21 s_330_22
        let s_330_25: Bits = (Bits::new(
            ((s_330_24) >> (s_330_21)).value(),
            u16::try_from(s_330_22).unwrap(),
        ));
        // D s_330_26: cast reint s_330_25 -> u8
        let s_330_26: bool = ((s_330_25.value()) != 0);
        // C s_330_27: const #31s : i
        let s_330_27: i128 = 31;
        // C s_330_28: const #1s : i
        let s_330_28: i128 = 1;
        // D s_330_29: read-var u#23316:u32
        let s_330_29: u32 = fn_state.u_23316;
        // D s_330_30: cast zx s_330_29 -> bv
        let s_330_30: Bits = Bits::new(s_330_29 as u128, 32u16);
        // D s_330_31: bit-extract s_330_30 s_330_27 s_330_28
        let s_330_31: Bits = (Bits::new(
            ((s_330_30) >> (s_330_27)).value(),
            u16::try_from(s_330_28).unwrap(),
        ));
        // D s_330_32: cast reint s_330_31 -> u8
        let s_330_32: bool = ((s_330_31.value()) != 0);
        // D s_330_33: call decode_tbnz_aarch64_instrs_branch_conditional_test(s_330_8, s_330_14, s_330_20, s_330_26, s_330_32)
        let s_330_33: () = decode_tbnz_aarch64_instrs_branch_conditional_test(
            state,
            tracer,
            s_330_8,
            s_330_14,
            s_330_20,
            s_330_26,
            s_330_32,
        );
        // N s_330_34: return
        return;
    }
    fn block_331<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_331_0: read-var merge#var.1:struct
        let s_331_0: u32 = fn_state.merge_var._1;
        // D s_331_1: write-var u#23320 <= s_331_0
        fn_state.u_23320 = s_331_0;
        // C s_331_2: const #24s : i
        let s_331_2: i128 = 24;
        // D s_331_3: read-var u#23320:u32
        let s_331_3: u32 = fn_state.u_23320;
        // D s_331_4: cast zx s_331_3 -> bv
        let s_331_4: Bits = Bits::new(s_331_3 as u128, 32u16);
        // C s_331_5: const #1s : i64
        let s_331_5: i64 = 1;
        // C s_331_6: cast zx s_331_5 -> i
        let s_331_6: i128 = (i128::try_from(s_331_5).unwrap());
        // C s_331_7: const #6s : i
        let s_331_7: i128 = 6;
        // C s_331_8: add s_331_7 s_331_6
        let s_331_8: i128 = (s_331_7 + s_331_6);
        // D s_331_9: bit-extract s_331_4 s_331_2 s_331_8
        let s_331_9: Bits = (Bits::new(
            ((s_331_4) >> (s_331_2)).value(),
            u16::try_from(s_331_8).unwrap(),
        ));
        // D s_331_10: cast reint s_331_9 -> u8
        let s_331_10: u8 = (s_331_9.value() as u8);
        // D s_331_11: cast zx s_331_10 -> bv
        let s_331_11: Bits = Bits::new(s_331_10 as u128, 7u16);
        // C s_331_12: const #54u : u8
        let s_331_12: u8 = 54;
        // C s_331_13: cast zx s_331_12 -> bv
        let s_331_13: Bits = Bits::new(s_331_12 as u128, 7u16);
        // D s_331_14: cmp-eq s_331_11 s_331_13
        let s_331_14: bool = ((s_331_11) == (s_331_13));
        // N s_331_15: branch s_331_14 b390 b332
        if s_331_14 {
            return block_390(state, tracer, fn_state);
        } else {
            return block_332(state, tracer, fn_state);
        };
    }
    fn block_332<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_332_0: const #0u : u8
        let s_332_0: bool = false;
        // D s_332_1: write-var gs#374493 <= s_332_0
        fn_state.gs_374493 = s_332_0;
        // N s_332_2: jump b333
        return block_333(state, tracer, fn_state);
    }
    fn block_333<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_333_0: read-var gs#374493:u8
        let s_333_0: bool = fn_state.gs_374493;
        // D s_333_1: not s_333_0
        let s_333_1: bool = !s_333_0;
        // N s_333_2: branch s_333_1 b335 b334
        if s_333_1 {
            return block_335(state, tracer, fn_state);
        } else {
            return block_334(state, tracer, fn_state);
        };
    }
    fn block_334<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_334_0: const #1010s : i
        let s_334_0: i128 = 1010;
        // C s_334_1: const #14696u : u32
        let s_334_1: u32 = 14696;
        // N s_334_2: write-reg s_334_1 <= s_334_0
        let s_334_2: () = {
            state.write_register::<i128>(s_334_1 as isize, s_334_0);
            tracer.write_register(s_334_1 as isize, s_334_0);
        };
        // C s_334_3: const #0s : i
        let s_334_3: i128 = 0;
        // C s_334_4: const #5s : i
        let s_334_4: i128 = 5;
        // D s_334_5: read-var u#23320:u32
        let s_334_5: u32 = fn_state.u_23320;
        // D s_334_6: cast zx s_334_5 -> bv
        let s_334_6: Bits = Bits::new(s_334_5 as u128, 32u16);
        // D s_334_7: bit-extract s_334_6 s_334_3 s_334_4
        let s_334_7: Bits = (Bits::new(
            ((s_334_6) >> (s_334_3)).value(),
            u16::try_from(s_334_4).unwrap(),
        ));
        // D s_334_8: cast reint s_334_7 -> u8
        let s_334_8: u8 = (s_334_7.value() as u8);
        // C s_334_9: const #5s : i
        let s_334_9: i128 = 5;
        // C s_334_10: const #14s : i
        let s_334_10: i128 = 14;
        // D s_334_11: read-var u#23320:u32
        let s_334_11: u32 = fn_state.u_23320;
        // D s_334_12: cast zx s_334_11 -> bv
        let s_334_12: Bits = Bits::new(s_334_11 as u128, 32u16);
        // D s_334_13: bit-extract s_334_12 s_334_9 s_334_10
        let s_334_13: Bits = (Bits::new(
            ((s_334_12) >> (s_334_9)).value(),
            u16::try_from(s_334_10).unwrap(),
        ));
        // D s_334_14: cast reint s_334_13 -> u14
        let s_334_14: u16 = (s_334_13.value() as u16);
        // C s_334_15: const #19s : i
        let s_334_15: i128 = 19;
        // C s_334_16: const #5s : i
        let s_334_16: i128 = 5;
        // D s_334_17: read-var u#23320:u32
        let s_334_17: u32 = fn_state.u_23320;
        // D s_334_18: cast zx s_334_17 -> bv
        let s_334_18: Bits = Bits::new(s_334_17 as u128, 32u16);
        // D s_334_19: bit-extract s_334_18 s_334_15 s_334_16
        let s_334_19: Bits = (Bits::new(
            ((s_334_18) >> (s_334_15)).value(),
            u16::try_from(s_334_16).unwrap(),
        ));
        // D s_334_20: cast reint s_334_19 -> u8
        let s_334_20: u8 = (s_334_19.value() as u8);
        // C s_334_21: const #24s : i
        let s_334_21: i128 = 24;
        // C s_334_22: const #1s : i
        let s_334_22: i128 = 1;
        // D s_334_23: read-var u#23320:u32
        let s_334_23: u32 = fn_state.u_23320;
        // D s_334_24: cast zx s_334_23 -> bv
        let s_334_24: Bits = Bits::new(s_334_23 as u128, 32u16);
        // D s_334_25: bit-extract s_334_24 s_334_21 s_334_22
        let s_334_25: Bits = (Bits::new(
            ((s_334_24) >> (s_334_21)).value(),
            u16::try_from(s_334_22).unwrap(),
        ));
        // D s_334_26: cast reint s_334_25 -> u8
        let s_334_26: bool = ((s_334_25.value()) != 0);
        // C s_334_27: const #31s : i
        let s_334_27: i128 = 31;
        // C s_334_28: const #1s : i
        let s_334_28: i128 = 1;
        // D s_334_29: read-var u#23320:u32
        let s_334_29: u32 = fn_state.u_23320;
        // D s_334_30: cast zx s_334_29 -> bv
        let s_334_30: Bits = Bits::new(s_334_29 as u128, 32u16);
        // D s_334_31: bit-extract s_334_30 s_334_27 s_334_28
        let s_334_31: Bits = (Bits::new(
            ((s_334_30) >> (s_334_27)).value(),
            u16::try_from(s_334_28).unwrap(),
        ));
        // D s_334_32: cast reint s_334_31 -> u8
        let s_334_32: bool = ((s_334_31.value()) != 0);
        // D s_334_33: call decode_tbz_aarch64_instrs_branch_conditional_test(s_334_8, s_334_14, s_334_20, s_334_26, s_334_32)
        let s_334_33: () = decode_tbz_aarch64_instrs_branch_conditional_test(
            state,
            tracer,
            s_334_8,
            s_334_14,
            s_334_20,
            s_334_26,
            s_334_32,
        );
        // N s_334_34: return
        return;
    }
    fn block_335<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_335_0: read-var merge#var.1:struct
        let s_335_0: u32 = fn_state.merge_var._1;
        // D s_335_1: write-var u#23327 <= s_335_0
        fn_state.u_23327 = s_335_0;
        // C s_335_2: const #21s : i
        let s_335_2: i128 = 21;
        // D s_335_3: read-var u#23327:u32
        let s_335_3: u32 = fn_state.u_23327;
        // D s_335_4: cast zx s_335_3 -> bv
        let s_335_4: Bits = Bits::new(s_335_3 as u128, 32u16);
        // C s_335_5: const #1s : i64
        let s_335_5: i64 = 1;
        // C s_335_6: cast zx s_335_5 -> i
        let s_335_6: i128 = (i128::try_from(s_335_5).unwrap());
        // C s_335_7: const #10s : i
        let s_335_7: i128 = 10;
        // C s_335_8: add s_335_7 s_335_6
        let s_335_8: i128 = (s_335_7 + s_335_6);
        // D s_335_9: bit-extract s_335_4 s_335_2 s_335_8
        let s_335_9: Bits = (Bits::new(
            ((s_335_4) >> (s_335_2)).value(),
            u16::try_from(s_335_8).unwrap(),
        ));
        // D s_335_10: cast reint s_335_9 -> u11
        let s_335_10: u16 = (s_335_9.value() as u16);
        // D s_335_11: cast zx s_335_10 -> bv
        let s_335_11: Bits = Bits::new(s_335_10 as u128, 11u16);
        // C s_335_12: const #1699u : u11
        let s_335_12: u16 = 1699;
        // C s_335_13: cast zx s_335_12 -> bv
        let s_335_13: Bits = Bits::new(s_335_12 as u128, 11u16);
        // D s_335_14: cmp-eq s_335_11 s_335_13
        let s_335_14: bool = ((s_335_11) == (s_335_13));
        // N s_335_15: branch s_335_14 b389 b336
        if s_335_14 {
            return block_389(state, tracer, fn_state);
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
        // D s_336_1: write-var gs#374511 <= s_336_0
        fn_state.gs_374511 = s_336_0;
        // N s_336_2: jump b337
        return block_337(state, tracer, fn_state);
    }
    fn block_337<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_337_0: read-var gs#374511:u8
        let s_337_0: bool = fn_state.gs_374511;
        // N s_337_1: branch s_337_0 b388 b338
        if s_337_0 {
            return block_388(state, tracer, fn_state);
        } else {
            return block_338(state, tracer, fn_state);
        };
    }
    fn block_338<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_338_0: const #0u : u8
        let s_338_0: bool = false;
        // D s_338_1: write-var gs#374513 <= s_338_0
        fn_state.gs_374513 = s_338_0;
        // N s_338_2: jump b339
        return block_339(state, tracer, fn_state);
    }
    fn block_339<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_339_0: read-var gs#374513:u8
        let s_339_0: bool = fn_state.gs_374513;
        // D s_339_1: not s_339_0
        let s_339_1: bool = !s_339_0;
        // N s_339_2: branch s_339_1 b341 b340
        if s_339_1 {
            return block_341(state, tracer, fn_state);
        } else {
            return block_340(state, tracer, fn_state);
        };
    }
    fn block_340<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_340_0: const #1011s : i
        let s_340_0: i128 = 1011;
        // C s_340_1: const #14696u : u32
        let s_340_1: u32 = 14696;
        // N s_340_2: write-reg s_340_1 <= s_340_0
        let s_340_2: () = {
            state.write_register::<i128>(s_340_1 as isize, s_340_0);
            tracer.write_register(s_340_1 as isize, s_340_0);
        };
        // C s_340_3: const #5s : i
        let s_340_3: i128 = 5;
        // C s_340_4: const #16s : i
        let s_340_4: i128 = 16;
        // D s_340_5: read-var u#23327:u32
        let s_340_5: u32 = fn_state.u_23327;
        // D s_340_6: cast zx s_340_5 -> bv
        let s_340_6: Bits = Bits::new(s_340_5 as u128, 32u16);
        // D s_340_7: bit-extract s_340_6 s_340_3 s_340_4
        let s_340_7: Bits = (Bits::new(
            ((s_340_6) >> (s_340_3)).value(),
            u16::try_from(s_340_4).unwrap(),
        ));
        // D s_340_8: cast reint s_340_7 -> u16
        let s_340_8: u16 = (s_340_7.value() as u16);
        // D s_340_9: call decode_tcancel_aarch64_instrs_system_tme_tcancel(s_340_8)
        let s_340_9: () = decode_tcancel_aarch64_instrs_system_tme_tcancel(
            state,
            tracer,
            s_340_8,
        );
        // N s_340_10: return
        return;
    }
    fn block_341<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_341_0: read-var merge#var.1:struct
        let s_341_0: u32 = fn_state.merge_var._1;
        // D s_341_1: cast zx s_341_0 -> bv
        let s_341_1: Bits = Bits::new(s_341_0 as u128, 32u16);
        // C s_341_2: const #3573756031u : u32
        let s_341_2: u32 = 3573756031;
        // C s_341_3: cast zx s_341_2 -> bv
        let s_341_3: Bits = Bits::new(s_341_2 as u128, 32u16);
        // D s_341_4: cmp-eq s_341_1 s_341_3
        let s_341_4: bool = ((s_341_1) == (s_341_3));
        // N s_341_5: branch s_341_4 b387 b342
        if s_341_4 {
            return block_387(state, tracer, fn_state);
        } else {
            return block_342(state, tracer, fn_state);
        };
    }
    fn block_342<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_342_0: const #0u : u8
        let s_342_0: bool = false;
        // D s_342_1: write-var gs#374520 <= s_342_0
        fn_state.gs_374520 = s_342_0;
        // N s_342_2: jump b343
        return block_343(state, tracer, fn_state);
    }
    fn block_343<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_343_0: read-var gs#374520:u8
        let s_343_0: bool = fn_state.gs_374520;
        // D s_343_1: not s_343_0
        let s_343_1: bool = !s_343_0;
        // N s_343_2: branch s_343_1 b345 b344
        if s_343_1 {
            return block_345(state, tracer, fn_state);
        } else {
            return block_344(state, tracer, fn_state);
        };
    }
    fn block_344<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_344_0: const #1012s : i
        let s_344_0: i128 = 1012;
        // C s_344_1: const #14696u : u32
        let s_344_1: u32 = 14696;
        // N s_344_2: write-reg s_344_1 <= s_344_0
        let s_344_2: () = {
            state.write_register::<i128>(s_344_1 as isize, s_344_0);
            tracer.write_register(s_344_1 as isize, s_344_0);
        };
        // C s_344_3: const #() : ()
        let s_344_3: () = ();
        // S s_344_4: call decode_tcommit_aarch64_instrs_system_tme_tcommit(s_344_3)
        let s_344_4: () = decode_tcommit_aarch64_instrs_system_tme_tcommit(
            state,
            tracer,
            s_344_3,
        );
        // N s_344_5: return
        return;
    }
    fn block_345<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_345_0: read-var merge#var.1:struct
        let s_345_0: u32 = fn_state.merge_var._1;
        // D s_345_1: write-var u#23332 <= s_345_0
        fn_state.u_23332 = s_345_0;
        // C s_345_2: const #5s : i
        let s_345_2: i128 = 5;
        // D s_345_3: read-var u#23332:u32
        let s_345_3: u32 = fn_state.u_23332;
        // D s_345_4: cast zx s_345_3 -> bv
        let s_345_4: Bits = Bits::new(s_345_3 as u128, 32u16);
        // C s_345_5: const #1s : i64
        let s_345_5: i64 = 1;
        // C s_345_6: cast zx s_345_5 -> i
        let s_345_6: i128 = (i128::try_from(s_345_5).unwrap());
        // C s_345_7: const #26s : i
        let s_345_7: i128 = 26;
        // C s_345_8: add s_345_7 s_345_6
        let s_345_8: i128 = (s_345_7 + s_345_6);
        // D s_345_9: bit-extract s_345_4 s_345_2 s_345_8
        let s_345_9: Bits = (Bits::new(
            ((s_345_4) >> (s_345_2)).value(),
            u16::try_from(s_345_8).unwrap(),
        ));
        // D s_345_10: cast reint s_345_9 -> u27
        let s_345_10: u32 = (s_345_9.value() as u32);
        // D s_345_11: cast zx s_345_10 -> bv
        let s_345_11: Bits = Bits::new(s_345_10 as u128, 27u16);
        // C s_345_12: const #111745411u : u27
        let s_345_12: u32 = 111745411;
        // C s_345_13: cast zx s_345_12 -> bv
        let s_345_13: Bits = Bits::new(s_345_12 as u128, 27u16);
        // D s_345_14: cmp-eq s_345_11 s_345_13
        let s_345_14: bool = ((s_345_11) == (s_345_13));
        // N s_345_15: branch s_345_14 b386 b346
        if s_345_14 {
            return block_386(state, tracer, fn_state);
        } else {
            return block_346(state, tracer, fn_state);
        };
    }
    fn block_346<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_346_0: const #0u : u8
        let s_346_0: bool = false;
        // D s_346_1: write-var gs#374527 <= s_346_0
        fn_state.gs_374527 = s_346_0;
        // N s_346_2: jump b347
        return block_347(state, tracer, fn_state);
    }
    fn block_347<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_347_0: read-var gs#374527:u8
        let s_347_0: bool = fn_state.gs_374527;
        // D s_347_1: not s_347_0
        let s_347_1: bool = !s_347_0;
        // N s_347_2: branch s_347_1 b349 b348
        if s_347_1 {
            return block_349(state, tracer, fn_state);
        } else {
            return block_348(state, tracer, fn_state);
        };
    }
    fn block_348<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_348_0: const #1015s : i
        let s_348_0: i128 = 1015;
        // C s_348_1: const #14696u : u32
        let s_348_1: u32 = 14696;
        // N s_348_2: write-reg s_348_1 <= s_348_0
        let s_348_2: () = {
            state.write_register::<i128>(s_348_1 as isize, s_348_0);
            tracer.write_register(s_348_1 as isize, s_348_0);
        };
        // C s_348_3: const #0s : i
        let s_348_3: i128 = 0;
        // C s_348_4: const #5s : i
        let s_348_4: i128 = 5;
        // D s_348_5: read-var u#23332:u32
        let s_348_5: u32 = fn_state.u_23332;
        // D s_348_6: cast zx s_348_5 -> bv
        let s_348_6: Bits = Bits::new(s_348_5 as u128, 32u16);
        // D s_348_7: bit-extract s_348_6 s_348_3 s_348_4
        let s_348_7: Bits = (Bits::new(
            ((s_348_6) >> (s_348_3)).value(),
            u16::try_from(s_348_4).unwrap(),
        ));
        // D s_348_8: cast reint s_348_7 -> u8
        let s_348_8: u8 = (s_348_7.value() as u8);
        // D s_348_9: call decode_tstart_aarch64_instrs_system_tme_tstart(s_348_8)
        let s_348_9: () = decode_tstart_aarch64_instrs_system_tme_tstart(
            state,
            tracer,
            s_348_8,
        );
        // N s_348_10: return
        return;
    }
    fn block_349<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_349_0: read-var merge#var.1:struct
        let s_349_0: u32 = fn_state.merge_var._1;
        // D s_349_1: write-var u#23335 <= s_349_0
        fn_state.u_23335 = s_349_0;
        // C s_349_2: const #5s : i
        let s_349_2: i128 = 5;
        // D s_349_3: read-var u#23335:u32
        let s_349_3: u32 = fn_state.u_23335;
        // D s_349_4: cast zx s_349_3 -> bv
        let s_349_4: Bits = Bits::new(s_349_3 as u128, 32u16);
        // C s_349_5: const #1s : i64
        let s_349_5: i64 = 1;
        // C s_349_6: cast zx s_349_5 -> i
        let s_349_6: i128 = (i128::try_from(s_349_5).unwrap());
        // C s_349_7: const #26s : i
        let s_349_7: i128 = 26;
        // C s_349_8: add s_349_7 s_349_6
        let s_349_8: i128 = (s_349_7 + s_349_6);
        // D s_349_9: bit-extract s_349_4 s_349_2 s_349_8
        let s_349_9: Bits = (Bits::new(
            ((s_349_4) >> (s_349_2)).value(),
            u16::try_from(s_349_8).unwrap(),
        ));
        // D s_349_10: cast reint s_349_9 -> u27
        let s_349_10: u32 = (s_349_9.value() as u32);
        // D s_349_11: cast zx s_349_10 -> bv
        let s_349_11: Bits = Bits::new(s_349_10 as u128, 27u16);
        // C s_349_12: const #111745419u : u27
        let s_349_12: u32 = 111745419;
        // C s_349_13: cast zx s_349_12 -> bv
        let s_349_13: Bits = Bits::new(s_349_12 as u128, 27u16);
        // D s_349_14: cmp-eq s_349_11 s_349_13
        let s_349_14: bool = ((s_349_11) == (s_349_13));
        // N s_349_15: branch s_349_14 b385 b350
        if s_349_14 {
            return block_385(state, tracer, fn_state);
        } else {
            return block_350(state, tracer, fn_state);
        };
    }
    fn block_350<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_350_0: const #0u : u8
        let s_350_0: bool = false;
        // D s_350_1: write-var gs#374536 <= s_350_0
        fn_state.gs_374536 = s_350_0;
        // N s_350_2: jump b351
        return block_351(state, tracer, fn_state);
    }
    fn block_351<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_351_0: read-var gs#374536:u8
        let s_351_0: bool = fn_state.gs_374536;
        // D s_351_1: not s_351_0
        let s_351_1: bool = !s_351_0;
        // N s_351_2: branch s_351_1 b353 b352
        if s_351_1 {
            return block_353(state, tracer, fn_state);
        } else {
            return block_352(state, tracer, fn_state);
        };
    }
    fn block_352<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_352_0: const #1016s : i
        let s_352_0: i128 = 1016;
        // C s_352_1: const #14696u : u32
        let s_352_1: u32 = 14696;
        // N s_352_2: write-reg s_352_1 <= s_352_0
        let s_352_2: () = {
            state.write_register::<i128>(s_352_1 as isize, s_352_0);
            tracer.write_register(s_352_1 as isize, s_352_0);
        };
        // C s_352_3: const #0s : i
        let s_352_3: i128 = 0;
        // C s_352_4: const #5s : i
        let s_352_4: i128 = 5;
        // D s_352_5: read-var u#23335:u32
        let s_352_5: u32 = fn_state.u_23335;
        // D s_352_6: cast zx s_352_5 -> bv
        let s_352_6: Bits = Bits::new(s_352_5 as u128, 32u16);
        // D s_352_7: bit-extract s_352_6 s_352_3 s_352_4
        let s_352_7: Bits = (Bits::new(
            ((s_352_6) >> (s_352_3)).value(),
            u16::try_from(s_352_4).unwrap(),
        ));
        // D s_352_8: cast reint s_352_7 -> u8
        let s_352_8: u8 = (s_352_7.value() as u8);
        // D s_352_9: call decode_ttest_aarch64_instrs_system_tme_ttest(s_352_8)
        let s_352_9: () = decode_ttest_aarch64_instrs_system_tme_ttest(
            state,
            tracer,
            s_352_8,
        );
        // N s_352_10: return
        return;
    }
    fn block_353<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_353_0: read-var merge#var.1:struct
        let s_353_0: u32 = fn_state.merge_var._1;
        // D s_353_1: write-var u#23338 <= s_353_0
        fn_state.u_23338 = s_353_0;
        // C s_353_2: const #5s : i
        let s_353_2: i128 = 5;
        // D s_353_3: read-var u#23338:u32
        let s_353_3: u32 = fn_state.u_23338;
        // D s_353_4: cast zx s_353_3 -> bv
        let s_353_4: Bits = Bits::new(s_353_3 as u128, 32u16);
        // C s_353_5: const #1s : i64
        let s_353_5: i64 = 1;
        // C s_353_6: cast zx s_353_5 -> i
        let s_353_6: i128 = (i128::try_from(s_353_5).unwrap());
        // C s_353_7: const #26s : i
        let s_353_7: i128 = 26;
        // C s_353_8: add s_353_7 s_353_6
        let s_353_8: i128 = (s_353_7 + s_353_6);
        // D s_353_9: bit-extract s_353_4 s_353_2 s_353_8
        let s_353_9: Bits = (Bits::new(
            ((s_353_4) >> (s_353_2)).value(),
            u16::try_from(s_353_8).unwrap(),
        ));
        // D s_353_10: cast reint s_353_9 -> u27
        let s_353_10: u32 = (s_353_9.value() as u32);
        // D s_353_11: cast zx s_353_10 -> bv
        let s_353_11: Bits = Bits::new(s_353_10 as u128, 27u16);
        // C s_353_12: const #111679616u : u27
        let s_353_12: u32 = 111679616;
        // C s_353_13: cast zx s_353_12 -> bv
        let s_353_13: Bits = Bits::new(s_353_12 as u128, 27u16);
        // D s_353_14: cmp-eq s_353_11 s_353_13
        let s_353_14: bool = ((s_353_11) == (s_353_13));
        // N s_353_15: branch s_353_14 b384 b354
        if s_353_14 {
            return block_384(state, tracer, fn_state);
        } else {
            return block_354(state, tracer, fn_state);
        };
    }
    fn block_354<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_354_0: const #0u : u8
        let s_354_0: bool = false;
        // D s_354_1: write-var gs#374545 <= s_354_0
        fn_state.gs_374545 = s_354_0;
        // N s_354_2: jump b355
        return block_355(state, tracer, fn_state);
    }
    fn block_355<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_355_0: read-var gs#374545:u8
        let s_355_0: bool = fn_state.gs_374545;
        // D s_355_1: not s_355_0
        let s_355_1: bool = !s_355_0;
        // N s_355_2: branch s_355_1 b357 b356
        if s_355_1 {
            return block_357(state, tracer, fn_state);
        } else {
            return block_356(state, tracer, fn_state);
        };
    }
    fn block_356<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_356_0: const #1028s : i
        let s_356_0: i128 = 1028;
        // C s_356_1: const #14696u : u32
        let s_356_1: u32 = 14696;
        // N s_356_2: write-reg s_356_1 <= s_356_0
        let s_356_2: () = {
            state.write_register::<i128>(s_356_1 as isize, s_356_0);
            tracer.write_register(s_356_1 as isize, s_356_0);
        };
        // C s_356_3: const #0s : i
        let s_356_3: i128 = 0;
        // C s_356_4: const #5s : i
        let s_356_4: i128 = 5;
        // D s_356_5: read-var u#23338:u32
        let s_356_5: u32 = fn_state.u_23338;
        // D s_356_6: cast zx s_356_5 -> bv
        let s_356_6: Bits = Bits::new(s_356_5 as u128, 32u16);
        // D s_356_7: bit-extract s_356_6 s_356_3 s_356_4
        let s_356_7: Bits = (Bits::new(
            ((s_356_6) >> (s_356_3)).value(),
            u16::try_from(s_356_4).unwrap(),
        ));
        // D s_356_8: cast reint s_356_7 -> u8
        let s_356_8: u8 = (s_356_7.value() as u8);
        // D s_356_9: call decode_wfet_aarch64_instrs_system_sysinstwithreg_wfet(s_356_8)
        let s_356_9: () = decode_wfet_aarch64_instrs_system_sysinstwithreg_wfet(
            state,
            tracer,
            s_356_8,
        );
        // N s_356_10: return
        return;
    }
    fn block_357<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_357_0: read-var merge#var.1:struct
        let s_357_0: u32 = fn_state.merge_var._1;
        // D s_357_1: write-var u#23340 <= s_357_0
        fn_state.u_23340 = s_357_0;
        // C s_357_2: const #5s : i
        let s_357_2: i128 = 5;
        // D s_357_3: read-var u#23340:u32
        let s_357_3: u32 = fn_state.u_23340;
        // D s_357_4: cast zx s_357_3 -> bv
        let s_357_4: Bits = Bits::new(s_357_3 as u128, 32u16);
        // C s_357_5: const #1s : i64
        let s_357_5: i64 = 1;
        // C s_357_6: cast zx s_357_5 -> i
        let s_357_6: i128 = (i128::try_from(s_357_5).unwrap());
        // C s_357_7: const #26s : i
        let s_357_7: i128 = 26;
        // C s_357_8: add s_357_7 s_357_6
        let s_357_8: i128 = (s_357_7 + s_357_6);
        // D s_357_9: bit-extract s_357_4 s_357_2 s_357_8
        let s_357_9: Bits = (Bits::new(
            ((s_357_4) >> (s_357_2)).value(),
            u16::try_from(s_357_8).unwrap(),
        ));
        // D s_357_10: cast reint s_357_9 -> u27
        let s_357_10: u32 = (s_357_9.value() as u32);
        // D s_357_11: cast zx s_357_10 -> bv
        let s_357_11: Bits = Bits::new(s_357_10 as u128, 27u16);
        // C s_357_12: const #111679617u : u27
        let s_357_12: u32 = 111679617;
        // C s_357_13: cast zx s_357_12 -> bv
        let s_357_13: Bits = Bits::new(s_357_12 as u128, 27u16);
        // D s_357_14: cmp-eq s_357_11 s_357_13
        let s_357_14: bool = ((s_357_11) == (s_357_13));
        // N s_357_15: branch s_357_14 b383 b358
        if s_357_14 {
            return block_383(state, tracer, fn_state);
        } else {
            return block_358(state, tracer, fn_state);
        };
    }
    fn block_358<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_358_0: const #0u : u8
        let s_358_0: bool = false;
        // D s_358_1: write-var gs#374554 <= s_358_0
        fn_state.gs_374554 = s_358_0;
        // N s_358_2: jump b359
        return block_359(state, tracer, fn_state);
    }
    fn block_359<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_359_0: read-var gs#374554:u8
        let s_359_0: bool = fn_state.gs_374554;
        // D s_359_1: not s_359_0
        let s_359_1: bool = !s_359_0;
        // N s_359_2: branch s_359_1 b361 b360
        if s_359_1 {
            return block_361(state, tracer, fn_state);
        } else {
            return block_360(state, tracer, fn_state);
        };
    }
    fn block_360<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_360_0: const #1029s : i
        let s_360_0: i128 = 1029;
        // C s_360_1: const #14696u : u32
        let s_360_1: u32 = 14696;
        // N s_360_2: write-reg s_360_1 <= s_360_0
        let s_360_2: () = {
            state.write_register::<i128>(s_360_1 as isize, s_360_0);
            tracer.write_register(s_360_1 as isize, s_360_0);
        };
        // C s_360_3: const #0s : i
        let s_360_3: i128 = 0;
        // C s_360_4: const #5s : i
        let s_360_4: i128 = 5;
        // D s_360_5: read-var u#23340:u32
        let s_360_5: u32 = fn_state.u_23340;
        // D s_360_6: cast zx s_360_5 -> bv
        let s_360_6: Bits = Bits::new(s_360_5 as u128, 32u16);
        // D s_360_7: bit-extract s_360_6 s_360_3 s_360_4
        let s_360_7: Bits = (Bits::new(
            ((s_360_6) >> (s_360_3)).value(),
            u16::try_from(s_360_4).unwrap(),
        ));
        // D s_360_8: cast reint s_360_7 -> u8
        let s_360_8: u8 = (s_360_7.value() as u8);
        // D s_360_9: call decode_wfit_aarch64_instrs_system_sysinstwithreg_wfit(s_360_8)
        let s_360_9: () = decode_wfit_aarch64_instrs_system_sysinstwithreg_wfit(
            state,
            tracer,
            s_360_8,
        );
        // N s_360_10: return
        return;
    }
    fn block_361<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_361_0: read-var merge#var.1:struct
        let s_361_0: u32 = fn_state.merge_var._1;
        // D s_361_1: write-var u#23343 <= s_361_0
        fn_state.u_23343 = s_361_0;
        // D s_361_2: read-var u#23343:u32
        let s_361_2: u32 = fn_state.u_23343;
        // D s_361_3: cast zx s_361_2 -> bv
        let s_361_3: Bits = Bits::new(s_361_2 as u128, 32u16);
        // C s_361_4: const #3573563455u : u32
        let s_361_4: u32 = 3573563455;
        // C s_361_5: cast zx s_361_4 -> bv
        let s_361_5: Bits = Bits::new(s_361_4 as u128, 32u16);
        // D s_361_6: cmp-eq s_361_3 s_361_5
        let s_361_6: bool = ((s_361_3) == (s_361_5));
        // N s_361_7: branch s_361_6 b382 b362
        if s_361_6 {
            return block_382(state, tracer, fn_state);
        } else {
            return block_362(state, tracer, fn_state);
        };
    }
    fn block_362<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_362_0: const #0u : u8
        let s_362_0: bool = false;
        // D s_362_1: write-var gs#374561 <= s_362_0
        fn_state.gs_374561 = s_362_0;
        // N s_362_2: jump b363
        return block_363(state, tracer, fn_state);
    }
    fn block_363<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_363_0: read-var gs#374561:u8
        let s_363_0: bool = fn_state.gs_374561;
        // D s_363_1: not s_363_0
        let s_363_1: bool = !s_363_0;
        // N s_363_2: branch s_363_1 b376 b364
        if s_363_1 {
            return block_376(state, tracer, fn_state);
        } else {
            return block_364(state, tracer, fn_state);
        };
    }
    fn block_364<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_364_0: const #1030s : i
        let s_364_0: i128 = 1030;
        // C s_364_1: const #14696u : u32
        let s_364_1: u32 = 14696;
        // N s_364_2: write-reg s_364_1 <= s_364_0
        let s_364_2: () = {
            state.write_register::<i128>(s_364_1 as isize, s_364_0);
            tracer.write_register(s_364_1 as isize, s_364_0);
        };
        // C s_364_3: const #8s : i
        let s_364_3: i128 = 8;
        // C s_364_4: const #4s : i
        let s_364_4: i128 = 4;
        // D s_364_5: read-var u#23343:u32
        let s_364_5: u32 = fn_state.u_23343;
        // D s_364_6: cast zx s_364_5 -> bv
        let s_364_6: Bits = Bits::new(s_364_5 as u128, 32u16);
        // D s_364_7: bit-extract s_364_6 s_364_3 s_364_4
        let s_364_7: Bits = (Bits::new(
            ((s_364_6) >> (s_364_3)).value(),
            u16::try_from(s_364_4).unwrap(),
        ));
        // D s_364_8: cast reint s_364_7 -> u8
        let s_364_8: u8 = (s_364_7.value() as u8);
        // D s_364_9: write-var u#23344 <= s_364_8
        fn_state.u_23344 = s_364_8;
        // C s_364_10: const #8s : i
        let s_364_10: i128 = 8;
        // D s_364_11: read-var u#23343:u32
        let s_364_11: u32 = fn_state.u_23343;
        // D s_364_12: cast zx s_364_11 -> bv
        let s_364_12: Bits = Bits::new(s_364_11 as u128, 32u16);
        // C s_364_13: const #1u : u64
        let s_364_13: u64 = 1;
        // D s_364_14: bit-extract s_364_12 s_364_10 s_364_13
        let s_364_14: Bits = (Bits::new(
            ((s_364_12) >> (s_364_10)).value(),
            u16::try_from(s_364_13).unwrap(),
        ));
        // D s_364_15: cast reint s_364_14 -> u8
        let s_364_15: bool = ((s_364_14.value()) != 0);
        // C s_364_16: const #0s : i
        let s_364_16: i128 = 0;
        // C s_364_17: const #0u : u64
        let s_364_17: u64 = 0;
        // D s_364_18: cast zx s_364_15 -> u64
        let s_364_18: u64 = (s_364_15 as u64);
        // C s_364_19: const #1u : u64
        let s_364_19: u64 = 1;
        // D s_364_20: and s_364_18 s_364_19
        let s_364_20: u64 = ((s_364_18) & (s_364_19));
        // D s_364_21: cmp-eq s_364_20 s_364_19
        let s_364_21: bool = ((s_364_20) == (s_364_19));
        // D s_364_22: lsl s_364_18 s_364_16
        let s_364_22: u64 = s_364_18 << s_364_16;
        // D s_364_23: or s_364_17 s_364_22
        let s_364_23: u64 = ((s_364_17) | (s_364_22));
        // D s_364_24: cmpl s_364_22
        let s_364_24: u64 = !s_364_22;
        // D s_364_25: and s_364_17 s_364_24
        let s_364_25: u64 = ((s_364_17) & (s_364_24));
        // D s_364_26: select s_364_21 s_364_23 s_364_25
        let s_364_26: u64 = if s_364_21 { s_364_23 } else { s_364_25 };
        // D s_364_27: cast trunc s_364_26 -> u8
        let s_364_27: bool = ((s_364_26) != 0);
        // D s_364_28: cast zx s_364_27 -> bv
        let s_364_28: Bits = Bits::new(s_364_27 as u128, 1u16);
        // C s_364_29: const #0u : u8
        let s_364_29: bool = false;
        // C s_364_30: cast zx s_364_29 -> bv
        let s_364_30: Bits = Bits::new(s_364_29 as u128, 1u16);
        // D s_364_31: cmp-ne s_364_28 s_364_30
        let s_364_31: bool = ((s_364_28) != (s_364_30));
        // N s_364_32: branch s_364_31 b375 b365
        if s_364_31 {
            return block_375(state, tracer, fn_state);
        } else {
            return block_365(state, tracer, fn_state);
        };
    }
    fn block_365<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_365_0: const #9s : i
        let s_365_0: i128 = 9;
        // D s_365_1: read-var u#23343:u32
        let s_365_1: u32 = fn_state.u_23343;
        // D s_365_2: cast zx s_365_1 -> bv
        let s_365_2: Bits = Bits::new(s_365_1 as u128, 32u16);
        // C s_365_3: const #1u : u64
        let s_365_3: u64 = 1;
        // D s_365_4: bit-extract s_365_2 s_365_0 s_365_3
        let s_365_4: Bits = (Bits::new(
            ((s_365_2) >> (s_365_0)).value(),
            u16::try_from(s_365_3).unwrap(),
        ));
        // D s_365_5: cast reint s_365_4 -> u8
        let s_365_5: bool = ((s_365_4.value()) != 0);
        // C s_365_6: const #0s : i
        let s_365_6: i128 = 0;
        // C s_365_7: const #0u : u64
        let s_365_7: u64 = 0;
        // D s_365_8: cast zx s_365_5 -> u64
        let s_365_8: u64 = (s_365_5 as u64);
        // C s_365_9: const #1u : u64
        let s_365_9: u64 = 1;
        // D s_365_10: and s_365_8 s_365_9
        let s_365_10: u64 = ((s_365_8) & (s_365_9));
        // D s_365_11: cmp-eq s_365_10 s_365_9
        let s_365_11: bool = ((s_365_10) == (s_365_9));
        // D s_365_12: lsl s_365_8 s_365_6
        let s_365_12: u64 = s_365_8 << s_365_6;
        // D s_365_13: or s_365_7 s_365_12
        let s_365_13: u64 = ((s_365_7) | (s_365_12));
        // D s_365_14: cmpl s_365_12
        let s_365_14: u64 = !s_365_12;
        // D s_365_15: and s_365_7 s_365_14
        let s_365_15: u64 = ((s_365_7) & (s_365_14));
        // D s_365_16: select s_365_11 s_365_13 s_365_15
        let s_365_16: u64 = if s_365_11 { s_365_13 } else { s_365_15 };
        // D s_365_17: cast trunc s_365_16 -> u8
        let s_365_17: bool = ((s_365_16) != 0);
        // D s_365_18: cast zx s_365_17 -> bv
        let s_365_18: Bits = Bits::new(s_365_17 as u128, 1u16);
        // C s_365_19: const #0u : u8
        let s_365_19: bool = false;
        // C s_365_20: cast zx s_365_19 -> bv
        let s_365_20: Bits = Bits::new(s_365_19 as u128, 1u16);
        // D s_365_21: cmp-ne s_365_18 s_365_20
        let s_365_21: bool = ((s_365_18) != (s_365_20));
        // D s_365_22: write-var gs#374570 <= s_365_21
        fn_state.gs_374570 = s_365_21;
        // N s_365_23: jump b366
        return block_366(state, tracer, fn_state);
    }
    fn block_366<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_366_0: read-var gs#374570:u8
        let s_366_0: bool = fn_state.gs_374570;
        // N s_366_1: branch s_366_0 b374 b367
        if s_366_0 {
            return block_374(state, tracer, fn_state);
        } else {
            return block_367(state, tracer, fn_state);
        };
    }
    fn block_367<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_367_0: const #10s : i
        let s_367_0: i128 = 10;
        // D s_367_1: read-var u#23343:u32
        let s_367_1: u32 = fn_state.u_23343;
        // D s_367_2: cast zx s_367_1 -> bv
        let s_367_2: Bits = Bits::new(s_367_1 as u128, 32u16);
        // C s_367_3: const #1u : u64
        let s_367_3: u64 = 1;
        // D s_367_4: bit-extract s_367_2 s_367_0 s_367_3
        let s_367_4: Bits = (Bits::new(
            ((s_367_2) >> (s_367_0)).value(),
            u16::try_from(s_367_3).unwrap(),
        ));
        // D s_367_5: cast reint s_367_4 -> u8
        let s_367_5: bool = ((s_367_4.value()) != 0);
        // C s_367_6: const #0s : i
        let s_367_6: i128 = 0;
        // C s_367_7: const #0u : u64
        let s_367_7: u64 = 0;
        // D s_367_8: cast zx s_367_5 -> u64
        let s_367_8: u64 = (s_367_5 as u64);
        // C s_367_9: const #1u : u64
        let s_367_9: u64 = 1;
        // D s_367_10: and s_367_8 s_367_9
        let s_367_10: u64 = ((s_367_8) & (s_367_9));
        // D s_367_11: cmp-eq s_367_10 s_367_9
        let s_367_11: bool = ((s_367_10) == (s_367_9));
        // D s_367_12: lsl s_367_8 s_367_6
        let s_367_12: u64 = s_367_8 << s_367_6;
        // D s_367_13: or s_367_7 s_367_12
        let s_367_13: u64 = ((s_367_7) | (s_367_12));
        // D s_367_14: cmpl s_367_12
        let s_367_14: u64 = !s_367_12;
        // D s_367_15: and s_367_7 s_367_14
        let s_367_15: u64 = ((s_367_7) & (s_367_14));
        // D s_367_16: select s_367_11 s_367_13 s_367_15
        let s_367_16: u64 = if s_367_11 { s_367_13 } else { s_367_15 };
        // D s_367_17: cast trunc s_367_16 -> u8
        let s_367_17: bool = ((s_367_16) != 0);
        // D s_367_18: cast zx s_367_17 -> bv
        let s_367_18: Bits = Bits::new(s_367_17 as u128, 1u16);
        // C s_367_19: const #0u : u8
        let s_367_19: bool = false;
        // C s_367_20: cast zx s_367_19 -> bv
        let s_367_20: Bits = Bits::new(s_367_19 as u128, 1u16);
        // D s_367_21: cmp-ne s_367_18 s_367_20
        let s_367_21: bool = ((s_367_18) != (s_367_20));
        // D s_367_22: write-var gs#374573 <= s_367_21
        fn_state.gs_374573 = s_367_21;
        // N s_367_23: jump b368
        return block_368(state, tracer, fn_state);
    }
    fn block_368<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_368_0: read-var gs#374573:u8
        let s_368_0: bool = fn_state.gs_374573;
        // N s_368_1: branch s_368_0 b373 b369
        if s_368_0 {
            return block_373(state, tracer, fn_state);
        } else {
            return block_369(state, tracer, fn_state);
        };
    }
    fn block_369<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_369_0: const #11s : i
        let s_369_0: i128 = 11;
        // D s_369_1: read-var u#23343:u32
        let s_369_1: u32 = fn_state.u_23343;
        // D s_369_2: cast zx s_369_1 -> bv
        let s_369_2: Bits = Bits::new(s_369_1 as u128, 32u16);
        // C s_369_3: const #1u : u64
        let s_369_3: u64 = 1;
        // D s_369_4: bit-extract s_369_2 s_369_0 s_369_3
        let s_369_4: Bits = (Bits::new(
            ((s_369_2) >> (s_369_0)).value(),
            u16::try_from(s_369_3).unwrap(),
        ));
        // D s_369_5: cast reint s_369_4 -> u8
        let s_369_5: bool = ((s_369_4.value()) != 0);
        // C s_369_6: const #0s : i
        let s_369_6: i128 = 0;
        // C s_369_7: const #0u : u64
        let s_369_7: u64 = 0;
        // D s_369_8: cast zx s_369_5 -> u64
        let s_369_8: u64 = (s_369_5 as u64);
        // C s_369_9: const #1u : u64
        let s_369_9: u64 = 1;
        // D s_369_10: and s_369_8 s_369_9
        let s_369_10: u64 = ((s_369_8) & (s_369_9));
        // D s_369_11: cmp-eq s_369_10 s_369_9
        let s_369_11: bool = ((s_369_10) == (s_369_9));
        // D s_369_12: lsl s_369_8 s_369_6
        let s_369_12: u64 = s_369_8 << s_369_6;
        // D s_369_13: or s_369_7 s_369_12
        let s_369_13: u64 = ((s_369_7) | (s_369_12));
        // D s_369_14: cmpl s_369_12
        let s_369_14: u64 = !s_369_12;
        // D s_369_15: and s_369_7 s_369_14
        let s_369_15: u64 = ((s_369_7) & (s_369_14));
        // D s_369_16: select s_369_11 s_369_13 s_369_15
        let s_369_16: u64 = if s_369_11 { s_369_13 } else { s_369_15 };
        // D s_369_17: cast trunc s_369_16 -> u8
        let s_369_17: bool = ((s_369_16) != 0);
        // D s_369_18: cast zx s_369_17 -> bv
        let s_369_18: Bits = Bits::new(s_369_17 as u128, 1u16);
        // C s_369_19: const #0u : u8
        let s_369_19: bool = false;
        // C s_369_20: cast zx s_369_19 -> bv
        let s_369_20: Bits = Bits::new(s_369_19 as u128, 1u16);
        // D s_369_21: cmp-ne s_369_18 s_369_20
        let s_369_21: bool = ((s_369_18) != (s_369_20));
        // D s_369_22: write-var gs#374576 <= s_369_21
        fn_state.gs_374576 = s_369_21;
        // N s_369_23: jump b370
        return block_370(state, tracer, fn_state);
    }
    fn block_370<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_370_0: read-var gs#374576:u8
        let s_370_0: bool = fn_state.gs_374576;
        // N s_370_1: branch s_370_0 b372 b371
        if s_370_0 {
            return block_372(state, tracer, fn_state);
        } else {
            return block_371(state, tracer, fn_state);
        };
    }
    fn block_371<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_371_0: read-var u#23344:u8
        let s_371_0: u8 = fn_state.u_23344;
        // D s_371_1: call decode_xaflag_aarch64_instrs_integer_flags_xaflag(s_371_0)
        let s_371_1: () = decode_xaflag_aarch64_instrs_integer_flags_xaflag(
            state,
            tracer,
            s_371_0,
        );
        // N s_371_2: return
        return;
    }
    fn block_372<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_372_0: panic
        panic!("{:?}", ());
        // N s_372_1: return
        return;
    }
    fn block_373<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_373_0: const #1u : u8
        let s_373_0: bool = true;
        // D s_373_1: write-var gs#374576 <= s_373_0
        fn_state.gs_374576 = s_373_0;
        // N s_373_2: jump b370
        return block_370(state, tracer, fn_state);
    }
    fn block_374<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_374_0: const #1u : u8
        let s_374_0: bool = true;
        // D s_374_1: write-var gs#374573 <= s_374_0
        fn_state.gs_374573 = s_374_0;
        // N s_374_2: jump b368
        return block_368(state, tracer, fn_state);
    }
    fn block_375<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_375_0: const #1u : u8
        let s_375_0: bool = true;
        // D s_375_1: write-var gs#374570 <= s_375_0
        fn_state.gs_374570 = s_375_0;
        // N s_375_2: jump b366
        return block_366(state, tracer, fn_state);
    }
    fn block_376<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_376_0: read-var merge#var.1:struct
        let s_376_0: u32 = fn_state.merge_var._1;
        // D s_376_1: cast zx s_376_0 -> bv
        let s_376_1: Bits = Bits::new(s_376_0 as u128, 32u16);
        // C s_376_2: const #3573752063u : u32
        let s_376_2: u32 = 3573752063;
        // C s_376_3: cast zx s_376_2 -> bv
        let s_376_3: Bits = Bits::new(s_376_2 as u128, 32u16);
        // D s_376_4: cmp-eq s_376_1 s_376_3
        let s_376_4: bool = ((s_376_1) == (s_376_3));
        // N s_376_5: branch s_376_4 b381 b377
        if s_376_4 {
            return block_381(state, tracer, fn_state);
        } else {
            return block_377(state, tracer, fn_state);
        };
    }
    fn block_377<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_377_0: const #0u : u8
        let s_377_0: bool = false;
        // D s_377_1: write-var gs#374579 <= s_377_0
        fn_state.gs_374579 = s_377_0;
        // N s_377_2: jump b378
        return block_378(state, tracer, fn_state);
    }
    fn block_378<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_378_0: read-var gs#374579:u8
        let s_378_0: bool = fn_state.gs_374579;
        // D s_378_1: not s_378_0
        let s_378_1: bool = !s_378_0;
        // N s_378_2: branch s_378_1 b380 b379
        if s_378_1 {
            return block_380(state, tracer, fn_state);
        } else {
            return block_379(state, tracer, fn_state);
        };
    }
    fn block_379<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_379_0: const #1033s : i
        let s_379_0: i128 = 1033;
        // C s_379_1: const #14696u : u32
        let s_379_1: u32 = 14696;
        // N s_379_2: write-reg s_379_1 <= s_379_0
        let s_379_2: () = {
            state.write_register::<i128>(s_379_1 as isize, s_379_0);
            tracer.write_register(s_379_1 as isize, s_379_0);
        };
        // C s_379_3: const #() : ()
        let s_379_3: () = ();
        // S s_379_4: call decode_xpac_aarch64_instrs_integer_pac_strip_hint(s_379_3)
        let s_379_4: () = decode_xpac_aarch64_instrs_integer_pac_strip_hint(
            state,
            tracer,
            s_379_3,
        );
        // N s_379_5: return
        return;
    }
    fn block_380<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_380_0: panic
        panic!("{:?}", ());
        // N s_380_1: return
        return;
    }
    fn block_381<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_381_0: const #1033s : i
        let s_381_0: i128 = 1033;
        // C s_381_1: const #14696u : u32
        let s_381_1: u32 = 14696;
        // D s_381_2: read-reg s_381_1:i
        let s_381_2: i128 = {
            let value = state.read_register::<i128>(s_381_1 as isize);
            tracer.read_register(s_381_1 as isize, value);
            value
        };
        // D s_381_3: cmp-lt s_381_2 s_381_0
        let s_381_3: bool = ((s_381_2) < (s_381_0));
        // D s_381_4: write-var gs#374579 <= s_381_3
        fn_state.gs_374579 = s_381_3;
        // N s_381_5: jump b378
        return block_378(state, tracer, fn_state);
    }
    fn block_382<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_382_0: const #1030s : i
        let s_382_0: i128 = 1030;
        // C s_382_1: const #14696u : u32
        let s_382_1: u32 = 14696;
        // D s_382_2: read-reg s_382_1:i
        let s_382_2: i128 = {
            let value = state.read_register::<i128>(s_382_1 as isize);
            tracer.read_register(s_382_1 as isize, value);
            value
        };
        // D s_382_3: cmp-lt s_382_2 s_382_0
        let s_382_3: bool = ((s_382_2) < (s_382_0));
        // D s_382_4: write-var gs#374561 <= s_382_3
        fn_state.gs_374561 = s_382_3;
        // N s_382_5: jump b363
        return block_363(state, tracer, fn_state);
    }
    fn block_383<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_383_0: const #1029s : i
        let s_383_0: i128 = 1029;
        // C s_383_1: const #14696u : u32
        let s_383_1: u32 = 14696;
        // D s_383_2: read-reg s_383_1:i
        let s_383_2: i128 = {
            let value = state.read_register::<i128>(s_383_1 as isize);
            tracer.read_register(s_383_1 as isize, value);
            value
        };
        // D s_383_3: cmp-lt s_383_2 s_383_0
        let s_383_3: bool = ((s_383_2) < (s_383_0));
        // D s_383_4: write-var gs#374554 <= s_383_3
        fn_state.gs_374554 = s_383_3;
        // N s_383_5: jump b359
        return block_359(state, tracer, fn_state);
    }
    fn block_384<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_384_0: const #1028s : i
        let s_384_0: i128 = 1028;
        // C s_384_1: const #14696u : u32
        let s_384_1: u32 = 14696;
        // D s_384_2: read-reg s_384_1:i
        let s_384_2: i128 = {
            let value = state.read_register::<i128>(s_384_1 as isize);
            tracer.read_register(s_384_1 as isize, value);
            value
        };
        // D s_384_3: cmp-lt s_384_2 s_384_0
        let s_384_3: bool = ((s_384_2) < (s_384_0));
        // D s_384_4: write-var gs#374545 <= s_384_3
        fn_state.gs_374545 = s_384_3;
        // N s_384_5: jump b355
        return block_355(state, tracer, fn_state);
    }
    fn block_385<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_385_0: const #1016s : i
        let s_385_0: i128 = 1016;
        // C s_385_1: const #14696u : u32
        let s_385_1: u32 = 14696;
        // D s_385_2: read-reg s_385_1:i
        let s_385_2: i128 = {
            let value = state.read_register::<i128>(s_385_1 as isize);
            tracer.read_register(s_385_1 as isize, value);
            value
        };
        // D s_385_3: cmp-lt s_385_2 s_385_0
        let s_385_3: bool = ((s_385_2) < (s_385_0));
        // D s_385_4: write-var gs#374536 <= s_385_3
        fn_state.gs_374536 = s_385_3;
        // N s_385_5: jump b351
        return block_351(state, tracer, fn_state);
    }
    fn block_386<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_386_0: const #1015s : i
        let s_386_0: i128 = 1015;
        // C s_386_1: const #14696u : u32
        let s_386_1: u32 = 14696;
        // D s_386_2: read-reg s_386_1:i
        let s_386_2: i128 = {
            let value = state.read_register::<i128>(s_386_1 as isize);
            tracer.read_register(s_386_1 as isize, value);
            value
        };
        // D s_386_3: cmp-lt s_386_2 s_386_0
        let s_386_3: bool = ((s_386_2) < (s_386_0));
        // D s_386_4: write-var gs#374527 <= s_386_3
        fn_state.gs_374527 = s_386_3;
        // N s_386_5: jump b347
        return block_347(state, tracer, fn_state);
    }
    fn block_387<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_387_0: const #1012s : i
        let s_387_0: i128 = 1012;
        // C s_387_1: const #14696u : u32
        let s_387_1: u32 = 14696;
        // D s_387_2: read-reg s_387_1:i
        let s_387_2: i128 = {
            let value = state.read_register::<i128>(s_387_1 as isize);
            tracer.read_register(s_387_1 as isize, value);
            value
        };
        // D s_387_3: cmp-lt s_387_2 s_387_0
        let s_387_3: bool = ((s_387_2) < (s_387_0));
        // D s_387_4: write-var gs#374520 <= s_387_3
        fn_state.gs_374520 = s_387_3;
        // N s_387_5: jump b343
        return block_343(state, tracer, fn_state);
    }
    fn block_388<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_388_0: const #1011s : i
        let s_388_0: i128 = 1011;
        // C s_388_1: const #14696u : u32
        let s_388_1: u32 = 14696;
        // D s_388_2: read-reg s_388_1:i
        let s_388_2: i128 = {
            let value = state.read_register::<i128>(s_388_1 as isize);
            tracer.read_register(s_388_1 as isize, value);
            value
        };
        // D s_388_3: cmp-lt s_388_2 s_388_0
        let s_388_3: bool = ((s_388_2) < (s_388_0));
        // D s_388_4: write-var gs#374513 <= s_388_3
        fn_state.gs_374513 = s_388_3;
        // N s_388_5: jump b339
        return block_339(state, tracer, fn_state);
    }
    fn block_389<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_389_0: const #0s : i
        let s_389_0: i128 = 0;
        // D s_389_1: read-var u#23327:u32
        let s_389_1: u32 = fn_state.u_23327;
        // D s_389_2: cast zx s_389_1 -> bv
        let s_389_2: Bits = Bits::new(s_389_1 as u128, 32u16);
        // C s_389_3: const #1s : i64
        let s_389_3: i64 = 1;
        // C s_389_4: cast zx s_389_3 -> i
        let s_389_4: i128 = (i128::try_from(s_389_3).unwrap());
        // C s_389_5: const #4s : i
        let s_389_5: i128 = 4;
        // C s_389_6: add s_389_5 s_389_4
        let s_389_6: i128 = (s_389_5 + s_389_4);
        // D s_389_7: bit-extract s_389_2 s_389_0 s_389_6
        let s_389_7: Bits = (Bits::new(
            ((s_389_2) >> (s_389_0)).value(),
            u16::try_from(s_389_6).unwrap(),
        ));
        // D s_389_8: cast reint s_389_7 -> u8
        let s_389_8: u8 = (s_389_7.value() as u8);
        // D s_389_9: cast zx s_389_8 -> bv
        let s_389_9: Bits = Bits::new(s_389_8 as u128, 5u16);
        // C s_389_10: const #0u : u8
        let s_389_10: u8 = 0;
        // C s_389_11: cast zx s_389_10 -> bv
        let s_389_11: Bits = Bits::new(s_389_10 as u128, 5u16);
        // D s_389_12: cmp-eq s_389_9 s_389_11
        let s_389_12: bool = ((s_389_9) == (s_389_11));
        // D s_389_13: write-var gs#374511 <= s_389_12
        fn_state.gs_374511 = s_389_12;
        // N s_389_14: jump b337
        return block_337(state, tracer, fn_state);
    }
    fn block_390<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_390_0: const #1010s : i
        let s_390_0: i128 = 1010;
        // C s_390_1: const #14696u : u32
        let s_390_1: u32 = 14696;
        // D s_390_2: read-reg s_390_1:i
        let s_390_2: i128 = {
            let value = state.read_register::<i128>(s_390_1 as isize);
            tracer.read_register(s_390_1 as isize, value);
            value
        };
        // D s_390_3: cmp-lt s_390_2 s_390_0
        let s_390_3: bool = ((s_390_2) < (s_390_0));
        // D s_390_4: write-var gs#374493 <= s_390_3
        fn_state.gs_374493 = s_390_3;
        // N s_390_5: jump b333
        return block_333(state, tracer, fn_state);
    }
    fn block_391<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_391_0: const #1009s : i
        let s_391_0: i128 = 1009;
        // C s_391_1: const #14696u : u32
        let s_391_1: u32 = 14696;
        // D s_391_2: read-reg s_391_1:i
        let s_391_2: i128 = {
            let value = state.read_register::<i128>(s_391_1 as isize);
            tracer.read_register(s_391_1 as isize, value);
            value
        };
        // D s_391_3: cmp-lt s_391_2 s_391_0
        let s_391_3: bool = ((s_391_2) < (s_391_0));
        // D s_391_4: write-var gs#374476 <= s_391_3
        fn_state.gs_374476 = s_391_3;
        // N s_391_5: jump b329
        return block_329(state, tracer, fn_state);
    }
    fn block_392<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_392_0: const #1006s : i
        let s_392_0: i128 = 1006;
        // C s_392_1: const #14696u : u32
        let s_392_1: u32 = 14696;
        // D s_392_2: read-reg s_392_1:i
        let s_392_2: i128 = {
            let value = state.read_register::<i128>(s_392_1 as isize);
            tracer.read_register(s_392_1 as isize, value);
            value
        };
        // D s_392_3: cmp-lt s_392_2 s_392_0
        let s_392_3: bool = ((s_392_2) < (s_392_0));
        // D s_392_4: write-var gs#374457 <= s_392_3
        fn_state.gs_374457 = s_392_3;
        // N s_392_5: jump b325
        return block_325(state, tracer, fn_state);
    }
    fn block_393<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_393_0: const #1005s : i
        let s_393_0: i128 = 1005;
        // C s_393_1: const #14696u : u32
        let s_393_1: u32 = 14696;
        // D s_393_2: read-reg s_393_1:i
        let s_393_2: i128 = {
            let value = state.read_register::<i128>(s_393_1 as isize);
            tracer.read_register(s_393_1 as isize, value);
            value
        };
        // D s_393_3: cmp-lt s_393_2 s_393_0
        let s_393_3: bool = ((s_393_2) < (s_393_0));
        // D s_393_4: write-var gs#374438 <= s_393_3
        fn_state.gs_374438 = s_393_3;
        // N s_393_5: jump b321
        return block_321(state, tracer, fn_state);
    }
    fn block_394<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_394_0: const #1004s : i
        let s_394_0: i128 = 1004;
        // C s_394_1: const #14696u : u32
        let s_394_1: u32 = 14696;
        // D s_394_2: read-reg s_394_1:i
        let s_394_2: i128 = {
            let value = state.read_register::<i128>(s_394_1 as isize);
            tracer.read_register(s_394_1 as isize, value);
            value
        };
        // D s_394_3: cmp-lt s_394_2 s_394_0
        let s_394_3: bool = ((s_394_2) < (s_394_0));
        // D s_394_4: write-var gs#374419 <= s_394_3
        fn_state.gs_374419 = s_394_3;
        // N s_394_5: jump b317
        return block_317(state, tracer, fn_state);
    }
    fn block_395<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_395_0: const #999s : i
        let s_395_0: i128 = 999;
        // C s_395_1: const #14696u : u32
        let s_395_1: u32 = 14696;
        // D s_395_2: read-reg s_395_1:i
        let s_395_2: i128 = {
            let value = state.read_register::<i128>(s_395_1 as isize);
            tracer.read_register(s_395_1 as isize, value);
            value
        };
        // D s_395_3: cmp-lt s_395_2 s_395_0
        let s_395_3: bool = ((s_395_2) < (s_395_0));
        // D s_395_4: write-var gs#374410 <= s_395_3
        fn_state.gs_374410 = s_395_3;
        // N s_395_5: jump b313
        return block_313(state, tracer, fn_state);
    }
    fn block_396<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_396_0: const #0s : i
        let s_396_0: i128 = 0;
        // D s_396_1: read-var u#23289:u32
        let s_396_1: u32 = fn_state.u_23289;
        // D s_396_2: cast zx s_396_1 -> bv
        let s_396_2: Bits = Bits::new(s_396_1 as u128, 32u16);
        // C s_396_3: const #1s : i64
        let s_396_3: i64 = 1;
        // C s_396_4: cast zx s_396_3 -> i
        let s_396_4: i128 = (i128::try_from(s_396_3).unwrap());
        // C s_396_5: const #4s : i
        let s_396_5: i128 = 4;
        // C s_396_6: add s_396_5 s_396_4
        let s_396_6: i128 = (s_396_5 + s_396_4);
        // D s_396_7: bit-extract s_396_2 s_396_0 s_396_6
        let s_396_7: Bits = (Bits::new(
            ((s_396_2) >> (s_396_0)).value(),
            u16::try_from(s_396_6).unwrap(),
        ));
        // D s_396_8: cast reint s_396_7 -> u8
        let s_396_8: u8 = (s_396_7.value() as u8);
        // D s_396_9: cast zx s_396_8 -> bv
        let s_396_9: Bits = Bits::new(s_396_8 as u128, 5u16);
        // C s_396_10: const #1u : u8
        let s_396_10: u8 = 1;
        // C s_396_11: cast zx s_396_10 -> bv
        let s_396_11: Bits = Bits::new(s_396_10 as u128, 5u16);
        // D s_396_12: cmp-eq s_396_9 s_396_11
        let s_396_12: bool = ((s_396_9) == (s_396_11));
        // D s_396_13: write-var gs#374408 <= s_396_12
        fn_state.gs_374408 = s_396_12;
        // N s_396_14: jump b311
        return block_311(state, tracer, fn_state);
    }
    fn block_397<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_397_0: const #846s : i
        let s_397_0: i128 = 846;
        // C s_397_1: const #14696u : u32
        let s_397_1: u32 = 14696;
        // D s_397_2: read-reg s_397_1:i
        let s_397_2: i128 = {
            let value = state.read_register::<i128>(s_397_1 as isize);
            tracer.read_register(s_397_1 as isize, value);
            value
        };
        // D s_397_3: cmp-lt s_397_2 s_397_0
        let s_397_3: bool = ((s_397_2) < (s_397_0));
        // D s_397_4: write-var gs#374398 <= s_397_3
        fn_state.gs_374398 = s_397_3;
        // N s_397_5: jump b307
        return block_307(state, tracer, fn_state);
    }
    fn block_398<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_398_0: const #0s : i
        let s_398_0: i128 = 0;
        // D s_398_1: read-var u#23286:u32
        let s_398_1: u32 = fn_state.u_23286;
        // D s_398_2: cast zx s_398_1 -> bv
        let s_398_2: Bits = Bits::new(s_398_1 as u128, 32u16);
        // C s_398_3: const #1s : i64
        let s_398_3: i64 = 1;
        // C s_398_4: cast zx s_398_3 -> i
        let s_398_4: i128 = (i128::try_from(s_398_3).unwrap());
        // C s_398_5: const #4s : i
        let s_398_5: i128 = 4;
        // C s_398_6: add s_398_5 s_398_4
        let s_398_6: i128 = (s_398_5 + s_398_4);
        // D s_398_7: bit-extract s_398_2 s_398_0 s_398_6
        let s_398_7: Bits = (Bits::new(
            ((s_398_2) >> (s_398_0)).value(),
            u16::try_from(s_398_6).unwrap(),
        ));
        // D s_398_8: cast reint s_398_7 -> u8
        let s_398_8: u8 = (s_398_7.value() as u8);
        // D s_398_9: cast zx s_398_8 -> bv
        let s_398_9: Bits = Bits::new(s_398_8 as u128, 5u16);
        // C s_398_10: const #3u : u8
        let s_398_10: u8 = 3;
        // C s_398_11: cast zx s_398_10 -> bv
        let s_398_11: Bits = Bits::new(s_398_10 as u128, 5u16);
        // D s_398_12: cmp-eq s_398_9 s_398_11
        let s_398_12: bool = ((s_398_9) == (s_398_11));
        // D s_398_13: write-var gs#374396 <= s_398_12
        fn_state.gs_374396 = s_398_12;
        // N s_398_14: jump b305
        return block_305(state, tracer, fn_state);
    }
    fn block_399<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_399_0: const #768s : i
        let s_399_0: i128 = 768;
        // C s_399_1: const #14696u : u32
        let s_399_1: u32 = 14696;
        // D s_399_2: read-reg s_399_1:i
        let s_399_2: i128 = {
            let value = state.read_register::<i128>(s_399_1 as isize);
            tracer.read_register(s_399_1 as isize, value);
            value
        };
        // D s_399_3: cmp-lt s_399_2 s_399_0
        let s_399_3: bool = ((s_399_2) < (s_399_0));
        // D s_399_4: write-var gs#374373 <= s_399_3
        fn_state.gs_374373 = s_399_3;
        // N s_399_5: jump b290
        return block_290(state, tracer, fn_state);
    }
    fn block_400<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_400_0: const #714s : i
        let s_400_0: i128 = 714;
        // C s_400_1: const #14696u : u32
        let s_400_1: u32 = 14696;
        // D s_400_2: read-reg s_400_1:i
        let s_400_2: i128 = {
            let value = state.read_register::<i128>(s_400_1 as isize);
            tracer.read_register(s_400_1 as isize, value);
            value
        };
        // D s_400_3: cmp-lt s_400_2 s_400_0
        let s_400_3: bool = ((s_400_2) < (s_400_0));
        // D s_400_4: write-var gs#374364 <= s_400_3
        fn_state.gs_374364 = s_400_3;
        // N s_400_5: jump b286
        return block_286(state, tracer, fn_state);
    }
    fn block_401<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_401_0: const #6s : i
        let s_401_0: i128 = 6;
        // D s_401_1: read-var u#23278:u32
        let s_401_1: u32 = fn_state.u_23278;
        // D s_401_2: cast zx s_401_1 -> bv
        let s_401_2: Bits = Bits::new(s_401_1 as u128, 32u16);
        // C s_401_3: const #1s : i64
        let s_401_3: i64 = 1;
        // C s_401_4: cast zx s_401_3 -> i
        let s_401_4: i128 = (i128::try_from(s_401_3).unwrap());
        // C s_401_5: const #2s : i
        let s_401_5: i128 = 2;
        // C s_401_6: add s_401_5 s_401_4
        let s_401_6: i128 = (s_401_5 + s_401_4);
        // D s_401_7: bit-extract s_401_2 s_401_0 s_401_6
        let s_401_7: Bits = (Bits::new(
            ((s_401_2) >> (s_401_0)).value(),
            u16::try_from(s_401_6).unwrap(),
        ));
        // D s_401_8: cast reint s_401_7 -> u8
        let s_401_8: u8 = (s_401_7.value() as u8);
        // D s_401_9: cast zx s_401_8 -> bv
        let s_401_9: Bits = Bits::new(s_401_8 as u128, 3u16);
        // C s_401_10: const #5u : u8
        let s_401_10: u8 = 5;
        // C s_401_11: cast zx s_401_10 -> bv
        let s_401_11: Bits = Bits::new(s_401_10 as u128, 3u16);
        // D s_401_12: cmp-eq s_401_9 s_401_11
        let s_401_12: bool = ((s_401_9) == (s_401_11));
        // N s_401_13: branch s_401_12 b404 b402
        if s_401_12 {
            return block_404(state, tracer, fn_state);
        } else {
            return block_402(state, tracer, fn_state);
        };
    }
    fn block_402<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_402_0: const #0u : u8
        let s_402_0: bool = false;
        // D s_402_1: write-var gs#374361 <= s_402_0
        fn_state.gs_374361 = s_402_0;
        // N s_402_2: jump b403
        return block_403(state, tracer, fn_state);
    }
    fn block_403<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_403_0: read-var gs#374361:u8
        let s_403_0: bool = fn_state.gs_374361;
        // D s_403_1: write-var gs#374362 <= s_403_0
        fn_state.gs_374362 = s_403_0;
        // N s_403_2: jump b284
        return block_284(state, tracer, fn_state);
    }
    fn block_404<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_404_0: const #0s : i
        let s_404_0: i128 = 0;
        // D s_404_1: read-var u#23278:u32
        let s_404_1: u32 = fn_state.u_23278;
        // D s_404_2: cast zx s_404_1 -> bv
        let s_404_2: Bits = Bits::new(s_404_1 as u128, 32u16);
        // C s_404_3: const #1s : i64
        let s_404_3: i64 = 1;
        // C s_404_4: cast zx s_404_3 -> i
        let s_404_4: i128 = (i128::try_from(s_404_3).unwrap());
        // C s_404_5: const #4s : i
        let s_404_5: i128 = 4;
        // C s_404_6: add s_404_5 s_404_4
        let s_404_6: i128 = (s_404_5 + s_404_4);
        // D s_404_7: bit-extract s_404_2 s_404_0 s_404_6
        let s_404_7: Bits = (Bits::new(
            ((s_404_2) >> (s_404_0)).value(),
            u16::try_from(s_404_6).unwrap(),
        ));
        // D s_404_8: cast reint s_404_7 -> u8
        let s_404_8: u8 = (s_404_7.value() as u8);
        // D s_404_9: cast zx s_404_8 -> bv
        let s_404_9: Bits = Bits::new(s_404_8 as u128, 5u16);
        // C s_404_10: const #31u : u8
        let s_404_10: u8 = 31;
        // C s_404_11: cast zx s_404_10 -> bv
        let s_404_11: Bits = Bits::new(s_404_10 as u128, 5u16);
        // D s_404_12: cmp-eq s_404_9 s_404_11
        let s_404_12: bool = ((s_404_9) == (s_404_11));
        // D s_404_13: write-var gs#374361 <= s_404_12
        fn_state.gs_374361 = s_404_12;
        // N s_404_14: jump b403
        return block_403(state, tracer, fn_state);
    }
    fn block_405<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_405_0: const #712s : i
        let s_405_0: i128 = 712;
        // C s_405_1: const #14696u : u32
        let s_405_1: u32 = 14696;
        // D s_405_2: read-reg s_405_1:i
        let s_405_2: i128 = {
            let value = state.read_register::<i128>(s_405_1 as isize);
            tracer.read_register(s_405_1 as isize, value);
            value
        };
        // D s_405_3: cmp-lt s_405_2 s_405_0
        let s_405_3: bool = ((s_405_2) < (s_405_0));
        // D s_405_4: write-var gs#374347 <= s_405_3
        fn_state.gs_374347 = s_405_3;
        // N s_405_5: jump b280
        return block_280(state, tracer, fn_state);
    }
    fn block_406<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_406_0: const #6s : i
        let s_406_0: i128 = 6;
        // D s_406_1: read-var u#23274:u32
        let s_406_1: u32 = fn_state.u_23274;
        // D s_406_2: cast zx s_406_1 -> bv
        let s_406_2: Bits = Bits::new(s_406_1 as u128, 32u16);
        // C s_406_3: const #1s : i64
        let s_406_3: i64 = 1;
        // C s_406_4: cast zx s_406_3 -> i
        let s_406_4: i128 = (i128::try_from(s_406_3).unwrap());
        // C s_406_5: const #2s : i
        let s_406_5: i128 = 2;
        // C s_406_6: add s_406_5 s_406_4
        let s_406_6: i128 = (s_406_5 + s_406_4);
        // D s_406_7: bit-extract s_406_2 s_406_0 s_406_6
        let s_406_7: Bits = (Bits::new(
            ((s_406_2) >> (s_406_0)).value(),
            u16::try_from(s_406_6).unwrap(),
        ));
        // D s_406_8: cast reint s_406_7 -> u8
        let s_406_8: u8 = (s_406_7.value() as u8);
        // D s_406_9: cast zx s_406_8 -> bv
        let s_406_9: Bits = Bits::new(s_406_8 as u128, 3u16);
        // C s_406_10: const #4u : u8
        let s_406_10: u8 = 4;
        // C s_406_11: cast zx s_406_10 -> bv
        let s_406_11: Bits = Bits::new(s_406_10 as u128, 3u16);
        // D s_406_12: cmp-eq s_406_9 s_406_11
        let s_406_12: bool = ((s_406_9) == (s_406_11));
        // N s_406_13: branch s_406_12 b409 b407
        if s_406_12 {
            return block_409(state, tracer, fn_state);
        } else {
            return block_407(state, tracer, fn_state);
        };
    }
    fn block_407<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_407_0: const #0u : u8
        let s_407_0: bool = false;
        // D s_407_1: write-var gs#374344 <= s_407_0
        fn_state.gs_374344 = s_407_0;
        // N s_407_2: jump b408
        return block_408(state, tracer, fn_state);
    }
    fn block_408<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_408_0: read-var gs#374344:u8
        let s_408_0: bool = fn_state.gs_374344;
        // D s_408_1: write-var gs#374345 <= s_408_0
        fn_state.gs_374345 = s_408_0;
        // N s_408_2: jump b278
        return block_278(state, tracer, fn_state);
    }
    fn block_409<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_409_0: const #0s : i
        let s_409_0: i128 = 0;
        // D s_409_1: read-var u#23274:u32
        let s_409_1: u32 = fn_state.u_23274;
        // D s_409_2: cast zx s_409_1 -> bv
        let s_409_2: Bits = Bits::new(s_409_1 as u128, 32u16);
        // C s_409_3: const #1s : i64
        let s_409_3: i64 = 1;
        // C s_409_4: cast zx s_409_3 -> i
        let s_409_4: i128 = (i128::try_from(s_409_3).unwrap());
        // C s_409_5: const #4s : i
        let s_409_5: i128 = 4;
        // C s_409_6: add s_409_5 s_409_4
        let s_409_6: i128 = (s_409_5 + s_409_4);
        // D s_409_7: bit-extract s_409_2 s_409_0 s_409_6
        let s_409_7: Bits = (Bits::new(
            ((s_409_2) >> (s_409_0)).value(),
            u16::try_from(s_409_6).unwrap(),
        ));
        // D s_409_8: cast reint s_409_7 -> u8
        let s_409_8: u8 = (s_409_7.value() as u8);
        // D s_409_9: cast zx s_409_8 -> bv
        let s_409_9: Bits = Bits::new(s_409_8 as u128, 5u16);
        // C s_409_10: const #31u : u8
        let s_409_10: u8 = 31;
        // C s_409_11: cast zx s_409_10 -> bv
        let s_409_11: Bits = Bits::new(s_409_10 as u128, 5u16);
        // D s_409_12: cmp-eq s_409_9 s_409_11
        let s_409_12: bool = ((s_409_9) == (s_409_11));
        // D s_409_13: write-var gs#374344 <= s_409_12
        fn_state.gs_374344 = s_409_12;
        // N s_409_14: jump b408
        return block_408(state, tracer, fn_state);
    }
    fn block_410<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_410_0: const #703s : i
        let s_410_0: i128 = 703;
        // C s_410_1: const #14696u : u32
        let s_410_1: u32 = 14696;
        // D s_410_2: read-reg s_410_1:i
        let s_410_2: i128 = {
            let value = state.read_register::<i128>(s_410_1 as isize);
            tracer.read_register(s_410_1 as isize, value);
            value
        };
        // D s_410_3: cmp-lt s_410_2 s_410_0
        let s_410_3: bool = ((s_410_2) < (s_410_0));
        // D s_410_4: write-var gs#374328 <= s_410_3
        fn_state.gs_374328 = s_410_3;
        // N s_410_5: jump b274
        return block_274(state, tracer, fn_state);
    }
    fn block_411<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_411_0: const #12s : i
        let s_411_0: i128 = 12;
        // D s_411_1: read-var u#23269:u32
        let s_411_1: u32 = fn_state.u_23269;
        // D s_411_2: cast zx s_411_1 -> bv
        let s_411_2: Bits = Bits::new(s_411_1 as u128, 32u16);
        // C s_411_3: const #1s : i64
        let s_411_3: i64 = 1;
        // C s_411_4: cast zx s_411_3 -> i
        let s_411_4: i128 = (i128::try_from(s_411_3).unwrap());
        // C s_411_5: const #3s : i
        let s_411_5: i128 = 3;
        // C s_411_6: add s_411_5 s_411_4
        let s_411_6: i128 = (s_411_5 + s_411_4);
        // D s_411_7: bit-extract s_411_2 s_411_0 s_411_6
        let s_411_7: Bits = (Bits::new(
            ((s_411_2) >> (s_411_0)).value(),
            u16::try_from(s_411_6).unwrap(),
        ));
        // D s_411_8: cast reint s_411_7 -> u8
        let s_411_8: u8 = (s_411_7.value() as u8);
        // D s_411_9: cast zx s_411_8 -> bv
        let s_411_9: Bits = Bits::new(s_411_8 as u128, 4u16);
        // C s_411_10: const #4u : u8
        let s_411_10: u8 = 4;
        // C s_411_11: cast zx s_411_10 -> bv
        let s_411_11: Bits = Bits::new(s_411_10 as u128, 4u16);
        // D s_411_12: cmp-eq s_411_9 s_411_11
        let s_411_12: bool = ((s_411_9) == (s_411_11));
        // N s_411_13: branch s_411_12 b414 b412
        if s_411_12 {
            return block_414(state, tracer, fn_state);
        } else {
            return block_412(state, tracer, fn_state);
        };
    }
    fn block_412<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_412_0: const #0u : u8
        let s_412_0: bool = false;
        // D s_412_1: write-var gs#374325 <= s_412_0
        fn_state.gs_374325 = s_412_0;
        // N s_412_2: jump b413
        return block_413(state, tracer, fn_state);
    }
    fn block_413<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_413_0: read-var gs#374325:u8
        let s_413_0: bool = fn_state.gs_374325;
        // D s_413_1: write-var gs#374326 <= s_413_0
        fn_state.gs_374326 = s_413_0;
        // N s_413_2: jump b272
        return block_272(state, tracer, fn_state);
    }
    fn block_414<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_414_0: const #0s : i
        let s_414_0: i128 = 0;
        // D s_414_1: read-var u#23269:u32
        let s_414_1: u32 = fn_state.u_23269;
        // D s_414_2: cast zx s_414_1 -> bv
        let s_414_2: Bits = Bits::new(s_414_1 as u128, 32u16);
        // C s_414_3: const #1s : i64
        let s_414_3: i64 = 1;
        // C s_414_4: cast zx s_414_3 -> i
        let s_414_4: i128 = (i128::try_from(s_414_3).unwrap());
        // C s_414_5: const #4s : i
        let s_414_5: i128 = 4;
        // C s_414_6: add s_414_5 s_414_4
        let s_414_6: i128 = (s_414_5 + s_414_4);
        // D s_414_7: bit-extract s_414_2 s_414_0 s_414_6
        let s_414_7: Bits = (Bits::new(
            ((s_414_2) >> (s_414_0)).value(),
            u16::try_from(s_414_6).unwrap(),
        ));
        // D s_414_8: cast reint s_414_7 -> u8
        let s_414_8: u8 = (s_414_7.value() as u8);
        // D s_414_9: cast zx s_414_8 -> bv
        let s_414_9: Bits = Bits::new(s_414_8 as u128, 5u16);
        // C s_414_10: const #31u : u8
        let s_414_10: u8 = 31;
        // C s_414_11: cast zx s_414_10 -> bv
        let s_414_11: Bits = Bits::new(s_414_10 as u128, 5u16);
        // D s_414_12: cmp-eq s_414_9 s_414_11
        let s_414_12: bool = ((s_414_9) == (s_414_11));
        // D s_414_13: write-var gs#374325 <= s_414_12
        fn_state.gs_374325 = s_414_12;
        // N s_414_14: jump b413
        return block_413(state, tracer, fn_state);
    }
    fn block_415<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_415_0: const #702s : i
        let s_415_0: i128 = 702;
        // C s_415_1: const #14696u : u32
        let s_415_1: u32 = 14696;
        // D s_415_2: read-reg s_415_1:i
        let s_415_2: i128 = {
            let value = state.read_register::<i128>(s_415_1 as isize);
            tracer.read_register(s_415_1 as isize, value);
            value
        };
        // D s_415_3: cmp-lt s_415_2 s_415_0
        let s_415_3: bool = ((s_415_2) < (s_415_0));
        // D s_415_4: write-var gs#374301 <= s_415_3
        fn_state.gs_374301 = s_415_3;
        // N s_415_5: jump b268
        return block_268(state, tracer, fn_state);
    }
    fn block_416<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_416_0: const #701s : i
        let s_416_0: i128 = 701;
        // C s_416_1: const #14696u : u32
        let s_416_1: u32 = 14696;
        // D s_416_2: read-reg s_416_1:i
        let s_416_2: i128 = {
            let value = state.read_register::<i128>(s_416_1 as isize);
            tracer.read_register(s_416_1 as isize, value);
            value
        };
        // D s_416_3: cmp-lt s_416_2 s_416_0
        let s_416_3: bool = ((s_416_2) < (s_416_0));
        // D s_416_4: write-var gs#374280 <= s_416_3
        fn_state.gs_374280 = s_416_3;
        // N s_416_5: jump b264
        return block_264(state, tracer, fn_state);
    }
    fn block_417<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_417_0: const #700s : i
        let s_417_0: i128 = 700;
        // C s_417_1: const #14696u : u32
        let s_417_1: u32 = 14696;
        // D s_417_2: read-reg s_417_1:i
        let s_417_2: i128 = {
            let value = state.read_register::<i128>(s_417_1 as isize);
            tracer.read_register(s_417_1 as isize, value);
            value
        };
        // D s_417_3: cmp-lt s_417_2 s_417_0
        let s_417_3: bool = ((s_417_2) < (s_417_0));
        // D s_417_4: write-var gs#374259 <= s_417_3
        fn_state.gs_374259 = s_417_3;
        // N s_417_5: jump b260
        return block_260(state, tracer, fn_state);
    }
    fn block_418<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_418_0: const #699s : i
        let s_418_0: i128 = 699;
        // C s_418_1: const #14696u : u32
        let s_418_1: u32 = 14696;
        // D s_418_2: read-reg s_418_1:i
        let s_418_2: i128 = {
            let value = state.read_register::<i128>(s_418_1 as isize);
            tracer.read_register(s_418_1 as isize, value);
            value
        };
        // D s_418_3: cmp-lt s_418_2 s_418_0
        let s_418_3: bool = ((s_418_2) < (s_418_0));
        // D s_418_4: write-var gs#374238 <= s_418_3
        fn_state.gs_374238 = s_418_3;
        // N s_418_5: jump b256
        return block_256(state, tracer, fn_state);
    }
    fn block_419<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_419_0: const #483s : i
        let s_419_0: i128 = 483;
        // C s_419_1: const #14696u : u32
        let s_419_1: u32 = 14696;
        // D s_419_2: read-reg s_419_1:i
        let s_419_2: i128 = {
            let value = state.read_register::<i128>(s_419_1 as isize);
            tracer.read_register(s_419_1 as isize, value);
            value
        };
        // D s_419_3: cmp-lt s_419_2 s_419_0
        let s_419_3: bool = ((s_419_2) < (s_419_0));
        // D s_419_4: write-var gs#374227 <= s_419_3
        fn_state.gs_374227 = s_419_3;
        // N s_419_5: jump b252
        return block_252(state, tracer, fn_state);
    }
    fn block_420<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_420_0: const #0s : i
        let s_420_0: i128 = 0;
        // D s_420_1: read-var u#23233:u32
        let s_420_1: u32 = fn_state.u_23233;
        // D s_420_2: cast zx s_420_1 -> bv
        let s_420_2: Bits = Bits::new(s_420_1 as u128, 32u16);
        // C s_420_3: const #1s : i64
        let s_420_3: i64 = 1;
        // C s_420_4: cast zx s_420_3 -> i
        let s_420_4: i128 = (i128::try_from(s_420_3).unwrap());
        // C s_420_5: const #7s : i
        let s_420_5: i128 = 7;
        // C s_420_6: add s_420_5 s_420_4
        let s_420_6: i128 = (s_420_5 + s_420_4);
        // D s_420_7: bit-extract s_420_2 s_420_0 s_420_6
        let s_420_7: Bits = (Bits::new(
            ((s_420_2) >> (s_420_0)).value(),
            u16::try_from(s_420_6).unwrap(),
        ));
        // D s_420_8: cast reint s_420_7 -> u8
        let s_420_8: u8 = (s_420_7.value() as u8);
        // D s_420_9: cast zx s_420_8 -> bv
        let s_420_9: Bits = Bits::new(s_420_8 as u128, 8u16);
        // C s_420_10: const #223u : u8
        let s_420_10: u8 = 223;
        // C s_420_11: cast zx s_420_10 -> bv
        let s_420_11: Bits = Bits::new(s_420_10 as u128, 8u16);
        // D s_420_12: cmp-eq s_420_9 s_420_11
        let s_420_12: bool = ((s_420_9) == (s_420_11));
        // D s_420_13: write-var gs#374225 <= s_420_12
        fn_state.gs_374225 = s_420_12;
        // N s_420_14: jump b250
        return block_250(state, tracer, fn_state);
    }
    fn block_421<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_421_0: const #479s : i
        let s_421_0: i128 = 479;
        // C s_421_1: const #14696u : u32
        let s_421_1: u32 = 14696;
        // D s_421_2: read-reg s_421_1:i
        let s_421_2: i128 = {
            let value = state.read_register::<i128>(s_421_1 as isize);
            tracer.read_register(s_421_1 as isize, value);
            value
        };
        // D s_421_3: cmp-lt s_421_2 s_421_0
        let s_421_3: bool = ((s_421_2) < (s_421_0));
        // D s_421_4: write-var gs#374215 <= s_421_3
        fn_state.gs_374215 = s_421_3;
        // N s_421_5: jump b246
        return block_246(state, tracer, fn_state);
    }
    fn block_422<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_422_0: const #0s : i
        let s_422_0: i128 = 0;
        // D s_422_1: read-var u#23230:u32
        let s_422_1: u32 = fn_state.u_23230;
        // D s_422_2: cast zx s_422_1 -> bv
        let s_422_2: Bits = Bits::new(s_422_1 as u128, 32u16);
        // C s_422_3: const #1s : i64
        let s_422_3: i64 = 1;
        // C s_422_4: cast zx s_422_3 -> i
        let s_422_4: i128 = (i128::try_from(s_422_3).unwrap());
        // C s_422_5: const #4s : i
        let s_422_5: i128 = 4;
        // C s_422_6: add s_422_5 s_422_4
        let s_422_6: i128 = (s_422_5 + s_422_4);
        // D s_422_7: bit-extract s_422_2 s_422_0 s_422_6
        let s_422_7: Bits = (Bits::new(
            ((s_422_2) >> (s_422_0)).value(),
            u16::try_from(s_422_6).unwrap(),
        ));
        // D s_422_8: cast reint s_422_7 -> u8
        let s_422_8: u8 = (s_422_7.value() as u8);
        // D s_422_9: cast zx s_422_8 -> bv
        let s_422_9: Bits = Bits::new(s_422_8 as u128, 5u16);
        // C s_422_10: const #2u : u8
        let s_422_10: u8 = 2;
        // C s_422_11: cast zx s_422_10 -> bv
        let s_422_11: Bits = Bits::new(s_422_10 as u128, 5u16);
        // D s_422_12: cmp-eq s_422_9 s_422_11
        let s_422_12: bool = ((s_422_9) == (s_422_11));
        // D s_422_13: write-var gs#374213 <= s_422_12
        fn_state.gs_374213 = s_422_12;
        // N s_422_14: jump b244
        return block_244(state, tracer, fn_state);
    }
    fn block_423<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_423_0: const #478s : i
        let s_423_0: i128 = 478;
        // C s_423_1: const #14696u : u32
        let s_423_1: u32 = 14696;
        // D s_423_2: read-reg s_423_1:i
        let s_423_2: i128 = {
            let value = state.read_register::<i128>(s_423_1 as isize);
            tracer.read_register(s_423_1 as isize, value);
            value
        };
        // D s_423_3: cmp-lt s_423_2 s_423_0
        let s_423_3: bool = ((s_423_2) < (s_423_0));
        // D s_423_4: write-var gs#374203 <= s_423_3
        fn_state.gs_374203 = s_423_3;
        // N s_423_5: jump b240
        return block_240(state, tracer, fn_state);
    }
    fn block_424<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_424_0: const #0s : i
        let s_424_0: i128 = 0;
        // D s_424_1: read-var u#23227:u32
        let s_424_1: u32 = fn_state.u_23227;
        // D s_424_2: cast zx s_424_1 -> bv
        let s_424_2: Bits = Bits::new(s_424_1 as u128, 32u16);
        // C s_424_3: const #1s : i64
        let s_424_3: i64 = 1;
        // C s_424_4: cast zx s_424_3 -> i
        let s_424_4: i128 = (i128::try_from(s_424_3).unwrap());
        // C s_424_5: const #4s : i
        let s_424_5: i128 = 4;
        // C s_424_6: add s_424_5 s_424_4
        let s_424_6: i128 = (s_424_5 + s_424_4);
        // D s_424_7: bit-extract s_424_2 s_424_0 s_424_6
        let s_424_7: Bits = (Bits::new(
            ((s_424_2) >> (s_424_0)).value(),
            u16::try_from(s_424_6).unwrap(),
        ));
        // D s_424_8: cast reint s_424_7 -> u8
        let s_424_8: u8 = (s_424_7.value() as u8);
        // D s_424_9: cast zx s_424_8 -> bv
        let s_424_9: Bits = Bits::new(s_424_8 as u128, 5u16);
        // C s_424_10: const #0u : u8
        let s_424_10: u8 = 0;
        // C s_424_11: cast zx s_424_10 -> bv
        let s_424_11: Bits = Bits::new(s_424_10 as u128, 5u16);
        // D s_424_12: cmp-eq s_424_9 s_424_11
        let s_424_12: bool = ((s_424_9) == (s_424_11));
        // D s_424_13: write-var gs#374201 <= s_424_12
        fn_state.gs_374201 = s_424_12;
        // N s_424_14: jump b238
        return block_238(state, tracer, fn_state);
    }
    fn block_425<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_425_0: const #204s : i
        let s_425_0: i128 = 204;
        // C s_425_1: const #14696u : u32
        let s_425_1: u32 = 14696;
        // D s_425_2: read-reg s_425_1:i
        let s_425_2: i128 = {
            let value = state.read_register::<i128>(s_425_1 as isize);
            tracer.read_register(s_425_1 as isize, value);
            value
        };
        // D s_425_3: cmp-lt s_425_2 s_425_0
        let s_425_3: bool = ((s_425_2) < (s_425_0));
        // D s_425_4: write-var gs#374185 <= s_425_3
        fn_state.gs_374185 = s_425_3;
        // N s_425_5: jump b234
        return block_234(state, tracer, fn_state);
    }
    fn block_426<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_426_0: const #0s : i
        let s_426_0: i128 = 0;
        // D s_426_1: read-var u#23221:u32
        let s_426_1: u32 = fn_state.u_23221;
        // D s_426_2: cast zx s_426_1 -> bv
        let s_426_2: Bits = Bits::new(s_426_1 as u128, 32u16);
        // C s_426_3: const #1s : i64
        let s_426_3: i64 = 1;
        // C s_426_4: cast zx s_426_3 -> i
        let s_426_4: i128 = (i128::try_from(s_426_3).unwrap());
        // C s_426_5: const #9s : i
        let s_426_5: i128 = 9;
        // C s_426_6: add s_426_5 s_426_4
        let s_426_6: i128 = (s_426_5 + s_426_4);
        // D s_426_7: bit-extract s_426_2 s_426_0 s_426_6
        let s_426_7: Bits = (Bits::new(
            ((s_426_2) >> (s_426_0)).value(),
            u16::try_from(s_426_6).unwrap(),
        ));
        // D s_426_8: cast reint s_426_7 -> u10
        let s_426_8: u16 = (s_426_7.value() as u16);
        // D s_426_9: cast zx s_426_8 -> bv
        let s_426_9: Bits = Bits::new(s_426_8 as u128, 10u16);
        // C s_426_10: const #1023u : u10
        let s_426_10: u16 = 1023;
        // C s_426_11: cast zx s_426_10 -> bv
        let s_426_11: Bits = Bits::new(s_426_10 as u128, 10u16);
        // D s_426_12: cmp-eq s_426_9 s_426_11
        let s_426_12: bool = ((s_426_9) == (s_426_11));
        // D s_426_13: write-var gs#374183 <= s_426_12
        fn_state.gs_374183 = s_426_12;
        // N s_426_14: jump b232
        return block_232(state, tracer, fn_state);
    }
    fn block_427<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_427_0: const #203s : i
        let s_427_0: i128 = 203;
        // C s_427_1: const #14696u : u32
        let s_427_1: u32 = 14696;
        // D s_427_2: read-reg s_427_1:i
        let s_427_2: i128 = {
            let value = state.read_register::<i128>(s_427_1 as isize);
            tracer.read_register(s_427_1 as isize, value);
            value
        };
        // D s_427_3: cmp-lt s_427_2 s_427_0
        let s_427_3: bool = ((s_427_2) < (s_427_0));
        // D s_427_4: write-var gs#374167 <= s_427_3
        fn_state.gs_374167 = s_427_3;
        // N s_427_5: jump b228
        return block_228(state, tracer, fn_state);
    }
    fn block_428<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_428_0: const #198s : i
        let s_428_0: i128 = 198;
        // C s_428_1: const #14696u : u32
        let s_428_1: u32 = 14696;
        // D s_428_2: read-reg s_428_1:i
        let s_428_2: i128 = {
            let value = state.read_register::<i128>(s_428_1 as isize);
            tracer.read_register(s_428_1 as isize, value);
            value
        };
        // D s_428_3: cmp-lt s_428_2 s_428_0
        let s_428_3: bool = ((s_428_2) < (s_428_0));
        // D s_428_4: write-var gs#374160 <= s_428_3
        fn_state.gs_374160 = s_428_3;
        // N s_428_5: jump b224
        return block_224(state, tracer, fn_state);
    }
    fn block_429<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_429_0: const #0s : i
        let s_429_0: i128 = 0;
        // D s_429_1: read-var u#23214:u32
        let s_429_1: u32 = fn_state.u_23214;
        // D s_429_2: cast zx s_429_1 -> bv
        let s_429_2: Bits = Bits::new(s_429_1 as u128, 32u16);
        // C s_429_3: const #1s : i64
        let s_429_3: i64 = 1;
        // C s_429_4: cast zx s_429_3 -> i
        let s_429_4: i128 = (i128::try_from(s_429_3).unwrap());
        // C s_429_5: const #9s : i
        let s_429_5: i128 = 9;
        // C s_429_6: add s_429_5 s_429_4
        let s_429_6: i128 = (s_429_5 + s_429_4);
        // D s_429_7: bit-extract s_429_2 s_429_0 s_429_6
        let s_429_7: Bits = (Bits::new(
            ((s_429_2) >> (s_429_0)).value(),
            u16::try_from(s_429_6).unwrap(),
        ));
        // D s_429_8: cast reint s_429_7 -> u10
        let s_429_8: u16 = (s_429_7.value() as u16);
        // D s_429_9: cast zx s_429_8 -> bv
        let s_429_9: Bits = Bits::new(s_429_8 as u128, 10u16);
        // C s_429_10: const #575u : u10
        let s_429_10: u16 = 575;
        // C s_429_11: cast zx s_429_10 -> bv
        let s_429_11: Bits = Bits::new(s_429_10 as u128, 10u16);
        // D s_429_12: cmp-eq s_429_9 s_429_11
        let s_429_12: bool = ((s_429_9) == (s_429_11));
        // D s_429_13: write-var gs#374158 <= s_429_12
        fn_state.gs_374158 = s_429_12;
        // N s_429_14: jump b222
        return block_222(state, tracer, fn_state);
    }
    fn block_430<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_430_0: const #197s : i
        let s_430_0: i128 = 197;
        // C s_430_1: const #14696u : u32
        let s_430_1: u32 = 14696;
        // D s_430_2: read-reg s_430_1:i
        let s_430_2: i128 = {
            let value = state.read_register::<i128>(s_430_1 as isize);
            tracer.read_register(s_430_1 as isize, value);
            value
        };
        // D s_430_3: cmp-lt s_430_2 s_430_0
        let s_430_3: bool = ((s_430_2) < (s_430_0));
        // D s_430_4: write-var gs#374146 <= s_430_3
        fn_state.gs_374146 = s_430_3;
        // N s_430_5: jump b218
        return block_218(state, tracer, fn_state);
    }
    fn block_431<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_431_0: const #0s : i
        let s_431_0: i128 = 0;
        // D s_431_1: read-var u#23210:u32
        let s_431_1: u32 = fn_state.u_23210;
        // D s_431_2: cast zx s_431_1 -> bv
        let s_431_2: Bits = Bits::new(s_431_1 as u128, 32u16);
        // C s_431_3: const #1s : i64
        let s_431_3: i64 = 1;
        // C s_431_4: cast zx s_431_3 -> i
        let s_431_4: i128 = (i128::try_from(s_431_3).unwrap());
        // C s_431_5: const #7s : i
        let s_431_5: i128 = 7;
        // C s_431_6: add s_431_5 s_431_4
        let s_431_6: i128 = (s_431_5 + s_431_4);
        // D s_431_7: bit-extract s_431_2 s_431_0 s_431_6
        let s_431_7: Bits = (Bits::new(
            ((s_431_2) >> (s_431_0)).value(),
            u16::try_from(s_431_6).unwrap(),
        ));
        // D s_431_8: cast reint s_431_7 -> u8
        let s_431_8: u8 = (s_431_7.value() as u8);
        // D s_431_9: cast zx s_431_8 -> bv
        let s_431_9: Bits = Bits::new(s_431_8 as u128, 8u16);
        // C s_431_10: const #159u : u8
        let s_431_10: u8 = 159;
        // C s_431_11: cast zx s_431_10 -> bv
        let s_431_11: Bits = Bits::new(s_431_10 as u128, 8u16);
        // D s_431_12: cmp-eq s_431_9 s_431_11
        let s_431_12: bool = ((s_431_9) == (s_431_11));
        // D s_431_13: write-var gs#374144 <= s_431_12
        fn_state.gs_374144 = s_431_12;
        // N s_431_14: jump b216
        return block_216(state, tracer, fn_state);
    }
    fn block_432<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_432_0: const #196s : i
        let s_432_0: i128 = 196;
        // C s_432_1: const #14696u : u32
        let s_432_1: u32 = 14696;
        // D s_432_2: read-reg s_432_1:i
        let s_432_2: i128 = {
            let value = state.read_register::<i128>(s_432_1 as isize);
            tracer.read_register(s_432_1 as isize, value);
            value
        };
        // D s_432_3: cmp-lt s_432_2 s_432_0
        let s_432_3: bool = ((s_432_2) < (s_432_0));
        // D s_432_4: write-var gs#374136 <= s_432_3
        fn_state.gs_374136 = s_432_3;
        // N s_432_5: jump b212
        return block_212(state, tracer, fn_state);
    }
    fn block_433<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_433_0: const #195s : i
        let s_433_0: i128 = 195;
        // C s_433_1: const #14696u : u32
        let s_433_1: u32 = 14696;
        // D s_433_2: read-reg s_433_1:i
        let s_433_2: i128 = {
            let value = state.read_register::<i128>(s_433_1 as isize);
            tracer.read_register(s_433_1 as isize, value);
            value
        };
        // D s_433_3: cmp-lt s_433_2 s_433_0
        let s_433_3: bool = ((s_433_2) < (s_433_0));
        // D s_433_4: write-var gs#374127 <= s_433_3
        fn_state.gs_374127 = s_433_3;
        // N s_433_5: jump b208
        return block_208(state, tracer, fn_state);
    }
    fn block_434<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_434_0: const #0s : i
        let s_434_0: i128 = 0;
        // D s_434_1: read-var u#23205:u32
        let s_434_1: u32 = fn_state.u_23205;
        // D s_434_2: cast zx s_434_1 -> bv
        let s_434_2: Bits = Bits::new(s_434_1 as u128, 32u16);
        // C s_434_3: const #1s : i64
        let s_434_3: i64 = 1;
        // C s_434_4: cast zx s_434_3 -> i
        let s_434_4: i128 = (i128::try_from(s_434_3).unwrap());
        // C s_434_5: const #7s : i
        let s_434_5: i128 = 7;
        // C s_434_6: add s_434_5 s_434_4
        let s_434_6: i128 = (s_434_5 + s_434_4);
        // D s_434_7: bit-extract s_434_2 s_434_0 s_434_6
        let s_434_7: Bits = (Bits::new(
            ((s_434_2) >> (s_434_0)).value(),
            u16::try_from(s_434_6).unwrap(),
        ));
        // D s_434_8: cast reint s_434_7 -> u8
        let s_434_8: u8 = (s_434_7.value() as u8);
        // D s_434_9: cast zx s_434_8 -> bv
        let s_434_9: Bits = Bits::new(s_434_8 as u128, 8u16);
        // C s_434_10: const #191u : u8
        let s_434_10: u8 = 191;
        // C s_434_11: cast zx s_434_10 -> bv
        let s_434_11: Bits = Bits::new(s_434_10 as u128, 8u16);
        // D s_434_12: cmp-eq s_434_9 s_434_11
        let s_434_12: bool = ((s_434_9) == (s_434_11));
        // D s_434_13: write-var gs#374125 <= s_434_12
        fn_state.gs_374125 = s_434_12;
        // N s_434_14: jump b206
        return block_206(state, tracer, fn_state);
    }
    fn block_435<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_435_0: const #194s : i
        let s_435_0: i128 = 194;
        // C s_435_1: const #14696u : u32
        let s_435_1: u32 = 14696;
        // D s_435_2: read-reg s_435_1:i
        let s_435_2: i128 = {
            let value = state.read_register::<i128>(s_435_1 as isize);
            tracer.read_register(s_435_1 as isize, value);
            value
        };
        // D s_435_3: cmp-lt s_435_2 s_435_0
        let s_435_3: bool = ((s_435_2) < (s_435_0));
        // D s_435_4: write-var gs#374113 <= s_435_3
        fn_state.gs_374113 = s_435_3;
        // N s_435_5: jump b202
        return block_202(state, tracer, fn_state);
    }
    fn block_436<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_436_0: const #0s : i
        let s_436_0: i128 = 0;
        // D s_436_1: read-var u#23201:u32
        let s_436_1: u32 = fn_state.u_23201;
        // D s_436_2: cast zx s_436_1 -> bv
        let s_436_2: Bits = Bits::new(s_436_1 as u128, 32u16);
        // C s_436_3: const #1s : i64
        let s_436_3: i64 = 1;
        // C s_436_4: cast zx s_436_3 -> i
        let s_436_4: i128 = (i128::try_from(s_436_3).unwrap());
        // C s_436_5: const #4s : i
        let s_436_5: i128 = 4;
        // C s_436_6: add s_436_5 s_436_4
        let s_436_6: i128 = (s_436_5 + s_436_4);
        // D s_436_7: bit-extract s_436_2 s_436_0 s_436_6
        let s_436_7: Bits = (Bits::new(
            ((s_436_2) >> (s_436_0)).value(),
            u16::try_from(s_436_6).unwrap(),
        ));
        // D s_436_8: cast reint s_436_7 -> u8
        let s_436_8: u8 = (s_436_7.value() as u8);
        // D s_436_9: cast zx s_436_8 -> bv
        let s_436_9: Bits = Bits::new(s_436_8 as u128, 5u16);
        // C s_436_10: const #3u : u8
        let s_436_10: u8 = 3;
        // C s_436_11: cast zx s_436_10 -> bv
        let s_436_11: Bits = Bits::new(s_436_10 as u128, 5u16);
        // D s_436_12: cmp-eq s_436_9 s_436_11
        let s_436_12: bool = ((s_436_9) == (s_436_11));
        // D s_436_13: write-var gs#374111 <= s_436_12
        fn_state.gs_374111 = s_436_12;
        // N s_436_14: jump b200
        return block_200(state, tracer, fn_state);
    }
    fn block_437<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_437_0: const #193s : i
        let s_437_0: i128 = 193;
        // C s_437_1: const #14696u : u32
        let s_437_1: u32 = 14696;
        // D s_437_2: read-reg s_437_1:i
        let s_437_2: i128 = {
            let value = state.read_register::<i128>(s_437_1 as isize);
            tracer.read_register(s_437_1 as isize, value);
            value
        };
        // D s_437_3: cmp-lt s_437_2 s_437_0
        let s_437_3: bool = ((s_437_2) < (s_437_0));
        // D s_437_4: write-var gs#374099 <= s_437_3
        fn_state.gs_374099 = s_437_3;
        // N s_437_5: jump b196
        return block_196(state, tracer, fn_state);
    }
    fn block_438<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_438_0: const #0s : i
        let s_438_0: i128 = 0;
        // D s_438_1: read-var u#23197:u32
        let s_438_1: u32 = fn_state.u_23197;
        // D s_438_2: cast zx s_438_1 -> bv
        let s_438_2: Bits = Bits::new(s_438_1 as u128, 32u16);
        // C s_438_3: const #1s : i64
        let s_438_3: i64 = 1;
        // C s_438_4: cast zx s_438_3 -> i
        let s_438_4: i128 = (i128::try_from(s_438_3).unwrap());
        // C s_438_5: const #4s : i
        let s_438_5: i128 = 4;
        // C s_438_6: add s_438_5 s_438_4
        let s_438_6: i128 = (s_438_5 + s_438_4);
        // D s_438_7: bit-extract s_438_2 s_438_0 s_438_6
        let s_438_7: Bits = (Bits::new(
            ((s_438_2) >> (s_438_0)).value(),
            u16::try_from(s_438_6).unwrap(),
        ));
        // D s_438_8: cast reint s_438_7 -> u8
        let s_438_8: u8 = (s_438_7.value() as u8);
        // D s_438_9: cast zx s_438_8 -> bv
        let s_438_9: Bits = Bits::new(s_438_8 as u128, 5u16);
        // C s_438_10: const #2u : u8
        let s_438_10: u8 = 2;
        // C s_438_11: cast zx s_438_10 -> bv
        let s_438_11: Bits = Bits::new(s_438_10 as u128, 5u16);
        // D s_438_12: cmp-eq s_438_9 s_438_11
        let s_438_12: bool = ((s_438_9) == (s_438_11));
        // D s_438_13: write-var gs#374097 <= s_438_12
        fn_state.gs_374097 = s_438_12;
        // N s_438_14: jump b194
        return block_194(state, tracer, fn_state);
    }
    fn block_439<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_439_0: const #192s : i
        let s_439_0: i128 = 192;
        // C s_439_1: const #14696u : u32
        let s_439_1: u32 = 14696;
        // D s_439_2: read-reg s_439_1:i
        let s_439_2: i128 = {
            let value = state.read_register::<i128>(s_439_1 as isize);
            tracer.read_register(s_439_1 as isize, value);
            value
        };
        // D s_439_3: cmp-lt s_439_2 s_439_0
        let s_439_3: bool = ((s_439_2) < (s_439_0));
        // D s_439_4: write-var gs#374085 <= s_439_3
        fn_state.gs_374085 = s_439_3;
        // N s_439_5: jump b190
        return block_190(state, tracer, fn_state);
    }
    fn block_440<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_440_0: const #0s : i
        let s_440_0: i128 = 0;
        // D s_440_1: read-var u#23194:u32
        let s_440_1: u32 = fn_state.u_23194;
        // D s_440_2: cast zx s_440_1 -> bv
        let s_440_2: Bits = Bits::new(s_440_1 as u128, 32u16);
        // C s_440_3: const #1s : i64
        let s_440_3: i64 = 1;
        // C s_440_4: cast zx s_440_3 -> i
        let s_440_4: i128 = (i128::try_from(s_440_3).unwrap());
        // C s_440_5: const #4s : i
        let s_440_5: i128 = 4;
        // C s_440_6: add s_440_5 s_440_4
        let s_440_6: i128 = (s_440_5 + s_440_4);
        // D s_440_7: bit-extract s_440_2 s_440_0 s_440_6
        let s_440_7: Bits = (Bits::new(
            ((s_440_2) >> (s_440_0)).value(),
            u16::try_from(s_440_6).unwrap(),
        ));
        // D s_440_8: cast reint s_440_7 -> u8
        let s_440_8: u8 = (s_440_7.value() as u8);
        // D s_440_9: cast zx s_440_8 -> bv
        let s_440_9: Bits = Bits::new(s_440_8 as u128, 5u16);
        // C s_440_10: const #1u : u8
        let s_440_10: u8 = 1;
        // C s_440_11: cast zx s_440_10 -> bv
        let s_440_11: Bits = Bits::new(s_440_10 as u128, 5u16);
        // D s_440_12: cmp-eq s_440_9 s_440_11
        let s_440_12: bool = ((s_440_9) == (s_440_11));
        // D s_440_13: write-var gs#374083 <= s_440_12
        fn_state.gs_374083 = s_440_12;
        // N s_440_14: jump b188
        return block_188(state, tracer, fn_state);
    }
    fn block_441<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_441_0: const #124s : i
        let s_441_0: i128 = 124;
        // C s_441_1: const #14696u : u32
        let s_441_1: u32 = 14696;
        // D s_441_2: read-reg s_441_1:i
        let s_441_2: i128 = {
            let value = state.read_register::<i128>(s_441_1 as isize);
            tracer.read_register(s_441_1 as isize, value);
            value
        };
        // D s_441_3: cmp-lt s_441_2 s_441_0
        let s_441_3: bool = ((s_441_2) < (s_441_0));
        // D s_441_4: write-var gs#374073 <= s_441_3
        fn_state.gs_374073 = s_441_3;
        // N s_441_5: jump b184
        return block_184(state, tracer, fn_state);
    }
    fn block_442<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_442_0: const #0s : i
        let s_442_0: i128 = 0;
        // D s_442_1: read-var u#23191:u32
        let s_442_1: u32 = fn_state.u_23191;
        // D s_442_2: cast zx s_442_1 -> bv
        let s_442_2: Bits = Bits::new(s_442_1 as u128, 32u16);
        // C s_442_3: const #1s : i64
        let s_442_3: i64 = 1;
        // C s_442_4: cast zx s_442_3 -> i
        let s_442_4: i128 = (i128::try_from(s_442_3).unwrap());
        // C s_442_5: const #7s : i
        let s_442_5: i128 = 7;
        // C s_442_6: add s_442_5 s_442_4
        let s_442_6: i128 = (s_442_5 + s_442_4);
        // D s_442_7: bit-extract s_442_2 s_442_0 s_442_6
        let s_442_7: Bits = (Bits::new(
            ((s_442_2) >> (s_442_0)).value(),
            u16::try_from(s_442_6).unwrap(),
        ));
        // D s_442_8: cast reint s_442_7 -> u8
        let s_442_8: u8 = (s_442_7.value() as u8);
        // D s_442_9: cast zx s_442_8 -> bv
        let s_442_9: Bits = Bits::new(s_442_8 as u128, 8u16);
        // C s_442_10: const #95u : u8
        let s_442_10: u8 = 95;
        // C s_442_11: cast zx s_442_10 -> bv
        let s_442_11: Bits = Bits::new(s_442_10 as u128, 8u16);
        // D s_442_12: cmp-eq s_442_9 s_442_11
        let s_442_12: bool = ((s_442_9) == (s_442_11));
        // D s_442_13: write-var gs#374071 <= s_442_12
        fn_state.gs_374071 = s_442_12;
        // N s_442_14: jump b182
        return block_182(state, tracer, fn_state);
    }
    fn block_443<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_443_0: const #123s : i
        let s_443_0: i128 = 123;
        // C s_443_1: const #14696u : u32
        let s_443_1: u32 = 14696;
        // D s_443_2: read-reg s_443_1:i
        let s_443_2: i128 = {
            let value = state.read_register::<i128>(s_443_1 as isize);
            tracer.read_register(s_443_1 as isize, value);
            value
        };
        // D s_443_3: cmp-lt s_443_2 s_443_0
        let s_443_3: bool = ((s_443_2) < (s_443_0));
        // D s_443_4: write-var gs#374050 <= s_443_3
        fn_state.gs_374050 = s_443_3;
        // N s_443_5: jump b167
        return block_167(state, tracer, fn_state);
    }
    fn block_444<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_444_0: const #118s : i
        let s_444_0: i128 = 118;
        // C s_444_1: const #14696u : u32
        let s_444_1: u32 = 14696;
        // D s_444_2: read-reg s_444_1:i
        let s_444_2: i128 = {
            let value = state.read_register::<i128>(s_444_1 as isize);
            tracer.read_register(s_444_1 as isize, value);
            value
        };
        // D s_444_3: cmp-lt s_444_2 s_444_0
        let s_444_3: bool = ((s_444_2) < (s_444_0));
        // D s_444_4: write-var gs#374037 <= s_444_3
        fn_state.gs_374037 = s_444_3;
        // N s_444_5: jump b163
        return block_163(state, tracer, fn_state);
    }
    fn block_445<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_445_0: const #117s : i
        let s_445_0: i128 = 117;
        // C s_445_1: const #14696u : u32
        let s_445_1: u32 = 14696;
        // D s_445_2: read-reg s_445_1:i
        let s_445_2: i128 = {
            let value = state.read_register::<i128>(s_445_1 as isize);
            tracer.read_register(s_445_1 as isize, value);
            value
        };
        // D s_445_3: cmp-lt s_445_2 s_445_0
        let s_445_3: bool = ((s_445_2) < (s_445_0));
        // D s_445_4: write-var gs#374022 <= s_445_3
        fn_state.gs_374022 = s_445_3;
        // N s_445_5: jump b159
        return block_159(state, tracer, fn_state);
    }
    fn block_446<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_446_0: const #112s : i
        let s_446_0: i128 = 112;
        // C s_446_1: const #14696u : u32
        let s_446_1: u32 = 14696;
        // D s_446_2: read-reg s_446_1:i
        let s_446_2: i128 = {
            let value = state.read_register::<i128>(s_446_1 as isize);
            tracer.read_register(s_446_1 as isize, value);
            value
        };
        // D s_446_3: cmp-lt s_446_2 s_446_0
        let s_446_3: bool = ((s_446_2) < (s_446_0));
        // D s_446_4: write-var gs#374011 <= s_446_3
        fn_state.gs_374011 = s_446_3;
        // N s_446_5: jump b155
        return block_155(state, tracer, fn_state);
    }
    fn block_447<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_447_0: const #111s : i
        let s_447_0: i128 = 111;
        // C s_447_1: const #14696u : u32
        let s_447_1: u32 = 14696;
        // D s_447_2: read-reg s_447_1:i
        let s_447_2: i128 = {
            let value = state.read_register::<i128>(s_447_1 as isize);
            tracer.read_register(s_447_1 as isize, value);
            value
        };
        // D s_447_3: cmp-lt s_447_2 s_447_0
        let s_447_3: bool = ((s_447_2) < (s_447_0));
        // D s_447_4: write-var gs#374002 <= s_447_3
        fn_state.gs_374002 = s_447_3;
        // N s_447_5: jump b151
        return block_151(state, tracer, fn_state);
    }
    fn block_448<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_448_0: const #110s : i
        let s_448_0: i128 = 110;
        // C s_448_1: const #14696u : u32
        let s_448_1: u32 = 14696;
        // D s_448_2: read-reg s_448_1:i
        let s_448_2: i128 = {
            let value = state.read_register::<i128>(s_448_1 as isize);
            tracer.read_register(s_448_1 as isize, value);
            value
        };
        // D s_448_3: cmp-lt s_448_2 s_448_0
        let s_448_3: bool = ((s_448_2) < (s_448_0));
        // D s_448_4: write-var gs#373993 <= s_448_3
        fn_state.gs_373993 = s_448_3;
        // N s_448_5: jump b147
        return block_147(state, tracer, fn_state);
    }
    fn block_449<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_449_0: const #109s : i
        let s_449_0: i128 = 109;
        // C s_449_1: const #14696u : u32
        let s_449_1: u32 = 14696;
        // D s_449_2: read-reg s_449_1:i
        let s_449_2: i128 = {
            let value = state.read_register::<i128>(s_449_1 as isize);
            tracer.read_register(s_449_1 as isize, value);
            value
        };
        // D s_449_3: cmp-lt s_449_2 s_449_0
        let s_449_3: bool = ((s_449_2) < (s_449_0));
        // D s_449_4: write-var gs#373984 <= s_449_3
        fn_state.gs_373984 = s_449_3;
        // N s_449_5: jump b143
        return block_143(state, tracer, fn_state);
    }
    fn block_450<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_450_0: const #108s : i
        let s_450_0: i128 = 108;
        // C s_450_1: const #14696u : u32
        let s_450_1: u32 = 14696;
        // D s_450_2: read-reg s_450_1:i
        let s_450_2: i128 = {
            let value = state.read_register::<i128>(s_450_1 as isize);
            tracer.read_register(s_450_1 as isize, value);
            value
        };
        // D s_450_3: cmp-lt s_450_2 s_450_0
        let s_450_3: bool = ((s_450_2) < (s_450_0));
        // D s_450_4: write-var gs#373975 <= s_450_3
        fn_state.gs_373975 = s_450_3;
        // N s_450_5: jump b139
        return block_139(state, tracer, fn_state);
    }
    fn block_451<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_451_0: const #107s : i
        let s_451_0: i128 = 107;
        // C s_451_1: const #14696u : u32
        let s_451_1: u32 = 14696;
        // D s_451_2: read-reg s_451_1:i
        let s_451_2: i128 = {
            let value = state.read_register::<i128>(s_451_1 as isize);
            tracer.read_register(s_451_1 as isize, value);
            value
        };
        // D s_451_3: cmp-lt s_451_2 s_451_0
        let s_451_3: bool = ((s_451_2) < (s_451_0));
        // D s_451_4: write-var gs#373966 <= s_451_3
        fn_state.gs_373966 = s_451_3;
        // N s_451_5: jump b135
        return block_135(state, tracer, fn_state);
    }
    fn block_452<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_452_0: const #106s : i
        let s_452_0: i128 = 106;
        // C s_452_1: const #14696u : u32
        let s_452_1: u32 = 14696;
        // D s_452_2: read-reg s_452_1:i
        let s_452_2: i128 = {
            let value = state.read_register::<i128>(s_452_1 as isize);
            tracer.read_register(s_452_1 as isize, value);
            value
        };
        // D s_452_3: cmp-lt s_452_2 s_452_0
        let s_452_3: bool = ((s_452_2) < (s_452_0));
        // D s_452_4: write-var gs#373957 <= s_452_3
        fn_state.gs_373957 = s_452_3;
        // N s_452_5: jump b131
        return block_131(state, tracer, fn_state);
    }
    fn block_453<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_453_0: const #105s : i
        let s_453_0: i128 = 105;
        // C s_453_1: const #14696u : u32
        let s_453_1: u32 = 14696;
        // D s_453_2: read-reg s_453_1:i
        let s_453_2: i128 = {
            let value = state.read_register::<i128>(s_453_1 as isize);
            tracer.read_register(s_453_1 as isize, value);
            value
        };
        // D s_453_3: cmp-lt s_453_2 s_453_0
        let s_453_3: bool = ((s_453_2) < (s_453_0));
        // D s_453_4: write-var gs#373948 <= s_453_3
        fn_state.gs_373948 = s_453_3;
        // N s_453_5: jump b127
        return block_127(state, tracer, fn_state);
    }
    fn block_454<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_454_0: const #104s : i
        let s_454_0: i128 = 104;
        // C s_454_1: const #14696u : u32
        let s_454_1: u32 = 14696;
        // D s_454_2: read-reg s_454_1:i
        let s_454_2: i128 = {
            let value = state.read_register::<i128>(s_454_1 as isize);
            tracer.read_register(s_454_1 as isize, value);
            value
        };
        // D s_454_3: cmp-lt s_454_2 s_454_0
        let s_454_3: bool = ((s_454_2) < (s_454_0));
        // D s_454_4: write-var gs#373939 <= s_454_3
        fn_state.gs_373939 = s_454_3;
        // N s_454_5: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_455<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_455_0: const #0s : i
        let s_455_0: i128 = 0;
        // D s_455_1: read-var u#23142:u32
        let s_455_1: u32 = fn_state.u_23142;
        // D s_455_2: cast zx s_455_1 -> bv
        let s_455_2: Bits = Bits::new(s_455_1 as u128, 32u16);
        // C s_455_3: const #1s : i64
        let s_455_3: i64 = 1;
        // C s_455_4: cast zx s_455_3 -> i
        let s_455_4: i128 = (i128::try_from(s_455_3).unwrap());
        // C s_455_5: const #4s : i
        let s_455_5: i128 = 4;
        // C s_455_6: add s_455_5 s_455_4
        let s_455_6: i128 = (s_455_5 + s_455_4);
        // D s_455_7: bit-extract s_455_2 s_455_0 s_455_6
        let s_455_7: Bits = (Bits::new(
            ((s_455_2) >> (s_455_0)).value(),
            u16::try_from(s_455_6).unwrap(),
        ));
        // D s_455_8: cast reint s_455_7 -> u8
        let s_455_8: u8 = (s_455_7.value() as u8);
        // D s_455_9: cast zx s_455_8 -> bv
        let s_455_9: Bits = Bits::new(s_455_8 as u128, 5u16);
        // C s_455_10: const #31u : u8
        let s_455_10: u8 = 31;
        // C s_455_11: cast zx s_455_10 -> bv
        let s_455_11: Bits = Bits::new(s_455_10 as u128, 5u16);
        // D s_455_12: cmp-eq s_455_9 s_455_11
        let s_455_12: bool = ((s_455_9) == (s_455_11));
        // D s_455_13: write-var gs#373937 <= s_455_12
        fn_state.gs_373937 = s_455_12;
        // N s_455_14: jump b121
        return block_121(state, tracer, fn_state);
    }
    fn block_456<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_456_0: const #103s : i
        let s_456_0: i128 = 103;
        // C s_456_1: const #14696u : u32
        let s_456_1: u32 = 14696;
        // D s_456_2: read-reg s_456_1:i
        let s_456_2: i128 = {
            let value = state.read_register::<i128>(s_456_1 as isize);
            tracer.read_register(s_456_1 as isize, value);
            value
        };
        // D s_456_3: cmp-lt s_456_2 s_456_0
        let s_456_3: bool = ((s_456_2) < (s_456_0));
        // D s_456_4: write-var gs#373925 <= s_456_3
        fn_state.gs_373925 = s_456_3;
        // N s_456_5: jump b117
        return block_117(state, tracer, fn_state);
    }
    fn block_457<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_457_0: const #102s : i
        let s_457_0: i128 = 102;
        // C s_457_1: const #14696u : u32
        let s_457_1: u32 = 14696;
        // D s_457_2: read-reg s_457_1:i
        let s_457_2: i128 = {
            let value = state.read_register::<i128>(s_457_1 as isize);
            tracer.read_register(s_457_1 as isize, value);
            value
        };
        // D s_457_3: cmp-lt s_457_2 s_457_0
        let s_457_3: bool = ((s_457_2) < (s_457_0));
        // D s_457_4: write-var gs#373916 <= s_457_3
        fn_state.gs_373916 = s_457_3;
        // N s_457_5: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_458<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_458_0: const #101s : i
        let s_458_0: i128 = 101;
        // C s_458_1: const #14696u : u32
        let s_458_1: u32 = 14696;
        // D s_458_2: read-reg s_458_1:i
        let s_458_2: i128 = {
            let value = state.read_register::<i128>(s_458_1 as isize);
            tracer.read_register(s_458_1 as isize, value);
            value
        };
        // D s_458_3: cmp-lt s_458_2 s_458_0
        let s_458_3: bool = ((s_458_2) < (s_458_0));
        // D s_458_4: write-var gs#373907 <= s_458_3
        fn_state.gs_373907 = s_458_3;
        // N s_458_5: jump b109
        return block_109(state, tracer, fn_state);
    }
    fn block_459<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_459_0: const #100s : i
        let s_459_0: i128 = 100;
        // C s_459_1: const #14696u : u32
        let s_459_1: u32 = 14696;
        // D s_459_2: read-reg s_459_1:i
        let s_459_2: i128 = {
            let value = state.read_register::<i128>(s_459_1 as isize);
            tracer.read_register(s_459_1 as isize, value);
            value
        };
        // D s_459_3: cmp-lt s_459_2 s_459_0
        let s_459_3: bool = ((s_459_2) < (s_459_0));
        // D s_459_4: write-var gs#373898 <= s_459_3
        fn_state.gs_373898 = s_459_3;
        // N s_459_5: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_460<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_460_0: const #99s : i
        let s_460_0: i128 = 99;
        // C s_460_1: const #14696u : u32
        let s_460_1: u32 = 14696;
        // D s_460_2: read-reg s_460_1:i
        let s_460_2: i128 = {
            let value = state.read_register::<i128>(s_460_1 as isize);
            tracer.read_register(s_460_1 as isize, value);
            value
        };
        // D s_460_3: cmp-lt s_460_2 s_460_0
        let s_460_3: bool = ((s_460_2) < (s_460_0));
        // D s_460_4: write-var gs#373889 <= s_460_3
        fn_state.gs_373889 = s_460_3;
        // N s_460_5: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_461<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_461_0: const #98s : i
        let s_461_0: i128 = 98;
        // C s_461_1: const #14696u : u32
        let s_461_1: u32 = 14696;
        // D s_461_2: read-reg s_461_1:i
        let s_461_2: i128 = {
            let value = state.read_register::<i128>(s_461_1 as isize);
            tracer.read_register(s_461_1 as isize, value);
            value
        };
        // D s_461_3: cmp-lt s_461_2 s_461_0
        let s_461_3: bool = ((s_461_2) < (s_461_0));
        // D s_461_4: write-var gs#373880 <= s_461_3
        fn_state.gs_373880 = s_461_3;
        // N s_461_5: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_462<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_462_0: const #97s : i
        let s_462_0: i128 = 97;
        // C s_462_1: const #14696u : u32
        let s_462_1: u32 = 14696;
        // D s_462_2: read-reg s_462_1:i
        let s_462_2: i128 = {
            let value = state.read_register::<i128>(s_462_1 as isize);
            tracer.read_register(s_462_1 as isize, value);
            value
        };
        // D s_462_3: cmp-lt s_462_2 s_462_0
        let s_462_3: bool = ((s_462_2) < (s_462_0));
        // D s_462_4: write-var gs#373871 <= s_462_3
        fn_state.gs_373871 = s_462_3;
        // N s_462_5: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_463<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_463_0: const #0s : i
        let s_463_0: i128 = 0;
        // D s_463_1: read-var u#23114:u32
        let s_463_1: u32 = fn_state.u_23114;
        // D s_463_2: cast zx s_463_1 -> bv
        let s_463_2: Bits = Bits::new(s_463_1 as u128, 32u16);
        // C s_463_3: const #1s : i64
        let s_463_3: i64 = 1;
        // C s_463_4: cast zx s_463_3 -> i
        let s_463_4: i128 = (i128::try_from(s_463_3).unwrap());
        // C s_463_5: const #5s : i
        let s_463_5: i128 = 5;
        // C s_463_6: add s_463_5 s_463_4
        let s_463_6: i128 = (s_463_5 + s_463_4);
        // D s_463_7: bit-extract s_463_2 s_463_0 s_463_6
        let s_463_7: Bits = (Bits::new(
            ((s_463_2) >> (s_463_0)).value(),
            u16::try_from(s_463_6).unwrap(),
        ));
        // D s_463_8: cast reint s_463_7 -> u8
        let s_463_8: u8 = (s_463_7.value() as u8);
        // D s_463_9: cast zx s_463_8 -> bv
        let s_463_9: Bits = Bits::new(s_463_8 as u128, 6u16);
        // C s_463_10: const #31u : u8
        let s_463_10: u8 = 31;
        // C s_463_11: cast zx s_463_10 -> bv
        let s_463_11: Bits = Bits::new(s_463_10 as u128, 6u16);
        // D s_463_12: cmp-eq s_463_9 s_463_11
        let s_463_12: bool = ((s_463_9) == (s_463_11));
        // D s_463_13: write-var gs#373869 <= s_463_12
        fn_state.gs_373869 = s_463_12;
        // N s_463_14: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_464<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_464_0: const #96s : i
        let s_464_0: i128 = 96;
        // C s_464_1: const #14696u : u32
        let s_464_1: u32 = 14696;
        // D s_464_2: read-reg s_464_1:i
        let s_464_2: i128 = {
            let value = state.read_register::<i128>(s_464_1 as isize);
            tracer.read_register(s_464_1 as isize, value);
            value
        };
        // D s_464_3: cmp-lt s_464_2 s_464_0
        let s_464_3: bool = ((s_464_2) < (s_464_0));
        // D s_464_4: write-var gs#373859 <= s_464_3
        fn_state.gs_373859 = s_464_3;
        // N s_464_5: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_465<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_465_0: const #0s : i
        let s_465_0: i128 = 0;
        // D s_465_1: read-var u#23112:u32
        let s_465_1: u32 = fn_state.u_23112;
        // D s_465_2: cast zx s_465_1 -> bv
        let s_465_2: Bits = Bits::new(s_465_1 as u128, 32u16);
        // C s_465_3: const #1s : i64
        let s_465_3: i64 = 1;
        // C s_465_4: cast zx s_465_3 -> i
        let s_465_4: i128 = (i128::try_from(s_465_3).unwrap());
        // C s_465_5: const #4s : i
        let s_465_5: i128 = 4;
        // C s_465_6: add s_465_5 s_465_4
        let s_465_6: i128 = (s_465_5 + s_465_4);
        // D s_465_7: bit-extract s_465_2 s_465_0 s_465_6
        let s_465_7: Bits = (Bits::new(
            ((s_465_2) >> (s_465_0)).value(),
            u16::try_from(s_465_6).unwrap(),
        ));
        // D s_465_8: cast reint s_465_7 -> u8
        let s_465_8: u8 = (s_465_7.value() as u8);
        // D s_465_9: cast zx s_465_8 -> bv
        let s_465_9: Bits = Bits::new(s_465_8 as u128, 5u16);
        // C s_465_10: const #0u : u8
        let s_465_10: u8 = 0;
        // C s_465_11: cast zx s_465_10 -> bv
        let s_465_11: Bits = Bits::new(s_465_10 as u128, 5u16);
        // D s_465_12: cmp-eq s_465_9 s_465_11
        let s_465_12: bool = ((s_465_9) == (s_465_11));
        // D s_465_13: write-var gs#373857 <= s_465_12
        fn_state.gs_373857 = s_465_12;
        // N s_465_14: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_466<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_466_0: const #95s : i
        let s_466_0: i128 = 95;
        // C s_466_1: const #14696u : u32
        let s_466_1: u32 = 14696;
        // D s_466_2: read-reg s_466_1:i
        let s_466_2: i128 = {
            let value = state.read_register::<i128>(s_466_1 as isize);
            tracer.read_register(s_466_1 as isize, value);
            value
        };
        // D s_466_3: cmp-lt s_466_2 s_466_0
        let s_466_3: bool = ((s_466_2) < (s_466_0));
        // D s_466_4: write-var gs#373837 <= s_466_3
        fn_state.gs_373837 = s_466_3;
        // N s_466_5: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_467<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_467_0: const #0s : i
        let s_467_0: i128 = 0;
        // D s_467_1: read-var u#23104:u32
        let s_467_1: u32 = fn_state.u_23104;
        // D s_467_2: cast zx s_467_1 -> bv
        let s_467_2: Bits = Bits::new(s_467_1 as u128, 32u16);
        // C s_467_3: const #1s : i64
        let s_467_3: i64 = 1;
        // C s_467_4: cast zx s_467_3 -> i
        let s_467_4: i128 = (i128::try_from(s_467_3).unwrap());
        // C s_467_5: const #9s : i
        let s_467_5: i128 = 9;
        // C s_467_6: add s_467_5 s_467_4
        let s_467_6: i128 = (s_467_5 + s_467_4);
        // D s_467_7: bit-extract s_467_2 s_467_0 s_467_6
        let s_467_7: Bits = (Bits::new(
            ((s_467_2) >> (s_467_0)).value(),
            u16::try_from(s_467_6).unwrap(),
        ));
        // D s_467_8: cast reint s_467_7 -> u10
        let s_467_8: u16 = (s_467_7.value() as u16);
        // D s_467_9: cast zx s_467_8 -> bv
        let s_467_9: Bits = Bits::new(s_467_8 as u128, 10u16);
        // C s_467_10: const #1023u : u10
        let s_467_10: u16 = 1023;
        // C s_467_11: cast zx s_467_10 -> bv
        let s_467_11: Bits = Bits::new(s_467_10 as u128, 10u16);
        // D s_467_12: cmp-eq s_467_9 s_467_11
        let s_467_12: bool = ((s_467_9) == (s_467_11));
        // D s_467_13: write-var gs#373835 <= s_467_12
        fn_state.gs_373835 = s_467_12;
        // N s_467_14: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_468<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_468_0: const #94s : i
        let s_468_0: i128 = 94;
        // C s_468_1: const #14696u : u32
        let s_468_1: u32 = 14696;
        // D s_468_2: read-reg s_468_1:i
        let s_468_2: i128 = {
            let value = state.read_register::<i128>(s_468_1 as isize);
            tracer.read_register(s_468_1 as isize, value);
            value
        };
        // D s_468_3: cmp-lt s_468_2 s_468_0
        let s_468_3: bool = ((s_468_2) < (s_468_0));
        // D s_468_4: write-var gs#373815 <= s_468_3
        fn_state.gs_373815 = s_468_3;
        // N s_468_5: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_469<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_469_0: const #0s : i
        let s_469_0: i128 = 0;
        // D s_469_1: read-var u#23096:u32
        let s_469_1: u32 = fn_state.u_23096;
        // D s_469_2: cast zx s_469_1 -> bv
        let s_469_2: Bits = Bits::new(s_469_1 as u128, 32u16);
        // C s_469_3: const #1s : i64
        let s_469_3: i64 = 1;
        // C s_469_4: cast zx s_469_3 -> i
        let s_469_4: i128 = (i128::try_from(s_469_3).unwrap());
        // C s_469_5: const #4s : i
        let s_469_5: i128 = 4;
        // C s_469_6: add s_469_5 s_469_4
        let s_469_6: i128 = (s_469_5 + s_469_4);
        // D s_469_7: bit-extract s_469_2 s_469_0 s_469_6
        let s_469_7: Bits = (Bits::new(
            ((s_469_2) >> (s_469_0)).value(),
            u16::try_from(s_469_6).unwrap(),
        ));
        // D s_469_8: cast reint s_469_7 -> u8
        let s_469_8: u8 = (s_469_7.value() as u8);
        // D s_469_9: cast zx s_469_8 -> bv
        let s_469_9: Bits = Bits::new(s_469_8 as u128, 5u16);
        // C s_469_10: const #0u : u8
        let s_469_10: u8 = 0;
        // C s_469_11: cast zx s_469_10 -> bv
        let s_469_11: Bits = Bits::new(s_469_10 as u128, 5u16);
        // D s_469_12: cmp-eq s_469_9 s_469_11
        let s_469_12: bool = ((s_469_9) == (s_469_11));
        // D s_469_13: write-var gs#373813 <= s_469_12
        fn_state.gs_373813 = s_469_12;
        // N s_469_14: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_470<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_470_0: const #93s : i
        let s_470_0: i128 = 93;
        // C s_470_1: const #14696u : u32
        let s_470_1: u32 = 14696;
        // D s_470_2: read-reg s_470_1:i
        let s_470_2: i128 = {
            let value = state.read_register::<i128>(s_470_1 as isize);
            tracer.read_register(s_470_1 as isize, value);
            value
        };
        // D s_470_3: cmp-lt s_470_2 s_470_0
        let s_470_3: bool = ((s_470_2) < (s_470_0));
        // D s_470_4: write-var gs#373793 <= s_470_3
        fn_state.gs_373793 = s_470_3;
        // N s_470_5: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_471<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_471_0: const #11s : i
        let s_471_0: i128 = 11;
        // D s_471_1: read-var u#23088:u32
        let s_471_1: u32 = fn_state.u_23088;
        // D s_471_2: cast zx s_471_1 -> bv
        let s_471_2: Bits = Bits::new(s_471_1 as u128, 32u16);
        // C s_471_3: const #1s : i64
        let s_471_3: i64 = 1;
        // C s_471_4: cast zx s_471_3 -> i
        let s_471_4: i128 = (i128::try_from(s_471_3).unwrap());
        // C s_471_5: const #12s : i
        let s_471_5: i128 = 12;
        // C s_471_6: add s_471_5 s_471_4
        let s_471_6: i128 = (s_471_5 + s_471_4);
        // D s_471_7: bit-extract s_471_2 s_471_0 s_471_6
        let s_471_7: Bits = (Bits::new(
            ((s_471_2) >> (s_471_0)).value(),
            u16::try_from(s_471_6).unwrap(),
        ));
        // D s_471_8: cast reint s_471_7 -> u13
        let s_471_8: u16 = (s_471_7.value() as u16);
        // D s_471_9: cast zx s_471_8 -> bv
        let s_471_9: Bits = Bits::new(s_471_8 as u128, 13u16);
        // C s_471_10: const #993u : u13
        let s_471_10: u16 = 993;
        // C s_471_11: cast zx s_471_10 -> bv
        let s_471_11: Bits = Bits::new(s_471_10 as u128, 13u16);
        // D s_471_12: cmp-eq s_471_9 s_471_11
        let s_471_12: bool = ((s_471_9) == (s_471_11));
        // D s_471_13: write-var gs#373791 <= s_471_12
        fn_state.gs_373791 = s_471_12;
        // N s_471_14: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_472<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_472_0: const #92s : i
        let s_472_0: i128 = 92;
        // C s_472_1: const #14696u : u32
        let s_472_1: u32 = 14696;
        // D s_472_2: read-reg s_472_1:i
        let s_472_2: i128 = {
            let value = state.read_register::<i128>(s_472_1 as isize);
            tracer.read_register(s_472_1 as isize, value);
            value
        };
        // D s_472_3: cmp-lt s_472_2 s_472_0
        let s_472_3: bool = ((s_472_2) < (s_472_0));
        // D s_472_4: write-var gs#373771 <= s_472_3
        fn_state.gs_373771 = s_472_3;
        // N s_472_5: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_473<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_473_0: const #0s : i
        let s_473_0: i128 = 0;
        // D s_473_1: read-var u#23080:u32
        let s_473_1: u32 = fn_state.u_23080;
        // D s_473_2: cast zx s_473_1 -> bv
        let s_473_2: Bits = Bits::new(s_473_1 as u128, 32u16);
        // C s_473_3: const #1s : i64
        let s_473_3: i64 = 1;
        // C s_473_4: cast zx s_473_3 -> i
        let s_473_4: i128 = (i128::try_from(s_473_3).unwrap());
        // C s_473_5: const #4s : i
        let s_473_5: i128 = 4;
        // C s_473_6: add s_473_5 s_473_4
        let s_473_6: i128 = (s_473_5 + s_473_4);
        // D s_473_7: bit-extract s_473_2 s_473_0 s_473_6
        let s_473_7: Bits = (Bits::new(
            ((s_473_2) >> (s_473_0)).value(),
            u16::try_from(s_473_6).unwrap(),
        ));
        // D s_473_8: cast reint s_473_7 -> u8
        let s_473_8: u8 = (s_473_7.value() as u8);
        // D s_473_9: cast zx s_473_8 -> bv
        let s_473_9: Bits = Bits::new(s_473_8 as u128, 5u16);
        // C s_473_10: const #0u : u8
        let s_473_10: u8 = 0;
        // C s_473_11: cast zx s_473_10 -> bv
        let s_473_11: Bits = Bits::new(s_473_10 as u128, 5u16);
        // D s_473_12: cmp-eq s_473_9 s_473_11
        let s_473_12: bool = ((s_473_9) == (s_473_11));
        // D s_473_13: write-var gs#373769 <= s_473_12
        fn_state.gs_373769 = s_473_12;
        // N s_473_14: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_474<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_474_0: const #91s : i
        let s_474_0: i128 = 91;
        // C s_474_1: const #14696u : u32
        let s_474_1: u32 = 14696;
        // D s_474_2: read-reg s_474_1:i
        let s_474_2: i128 = {
            let value = state.read_register::<i128>(s_474_1 as isize);
            tracer.read_register(s_474_1 as isize, value);
            value
        };
        // D s_474_3: cmp-lt s_474_2 s_474_0
        let s_474_3: bool = ((s_474_2) < (s_474_0));
        // D s_474_4: write-var gs#373749 <= s_474_3
        fn_state.gs_373749 = s_474_3;
        // N s_474_5: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_475<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_475_0: const #11s : i
        let s_475_0: i128 = 11;
        // D s_475_1: read-var u#23072:u32
        let s_475_1: u32 = fn_state.u_23072;
        // D s_475_2: cast zx s_475_1 -> bv
        let s_475_2: Bits = Bits::new(s_475_1 as u128, 32u16);
        // C s_475_3: const #1s : i64
        let s_475_3: i64 = 1;
        // C s_475_4: cast zx s_475_3 -> i
        let s_475_4: i128 = (i128::try_from(s_475_3).unwrap());
        // C s_475_5: const #12s : i
        let s_475_5: i128 = 12;
        // C s_475_6: add s_475_5 s_475_4
        let s_475_6: i128 = (s_475_5 + s_475_4);
        // D s_475_7: bit-extract s_475_2 s_475_0 s_475_6
        let s_475_7: Bits = (Bits::new(
            ((s_475_2) >> (s_475_0)).value(),
            u16::try_from(s_475_6).unwrap(),
        ));
        // D s_475_8: cast reint s_475_7 -> u13
        let s_475_8: u16 = (s_475_7.value() as u16);
        // D s_475_9: cast zx s_475_8 -> bv
        let s_475_9: Bits = Bits::new(s_475_8 as u128, 13u16);
        // C s_475_10: const #2017u : u13
        let s_475_10: u16 = 2017;
        // C s_475_11: cast zx s_475_10 -> bv
        let s_475_11: Bits = Bits::new(s_475_10 as u128, 13u16);
        // D s_475_12: cmp-eq s_475_9 s_475_11
        let s_475_12: bool = ((s_475_9) == (s_475_11));
        // D s_475_13: write-var gs#373747 <= s_475_12
        fn_state.gs_373747 = s_475_12;
        // N s_475_14: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_476<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_476_0: const #90s : i
        let s_476_0: i128 = 90;
        // C s_476_1: const #14696u : u32
        let s_476_1: u32 = 14696;
        // D s_476_2: read-reg s_476_1:i
        let s_476_2: i128 = {
            let value = state.read_register::<i128>(s_476_1 as isize);
            tracer.read_register(s_476_1 as isize, value);
            value
        };
        // D s_476_3: cmp-lt s_476_2 s_476_0
        let s_476_3: bool = ((s_476_2) < (s_476_0));
        // D s_476_4: write-var gs#373727 <= s_476_3
        fn_state.gs_373727 = s_476_3;
        // N s_476_5: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_477<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_477_0: const #0s : i
        let s_477_0: i128 = 0;
        // D s_477_1: read-var u#23069:u32
        let s_477_1: u32 = fn_state.u_23069;
        // D s_477_2: cast zx s_477_1 -> bv
        let s_477_2: Bits = Bits::new(s_477_1 as u128, 32u16);
        // C s_477_3: const #1s : i64
        let s_477_3: i64 = 1;
        // C s_477_4: cast zx s_477_3 -> i
        let s_477_4: i128 = (i128::try_from(s_477_3).unwrap());
        // C s_477_5: const #4s : i
        let s_477_5: i128 = 4;
        // C s_477_6: add s_477_5 s_477_4
        let s_477_6: i128 = (s_477_5 + s_477_4);
        // D s_477_7: bit-extract s_477_2 s_477_0 s_477_6
        let s_477_7: Bits = (Bits::new(
            ((s_477_2) >> (s_477_0)).value(),
            u16::try_from(s_477_6).unwrap(),
        ));
        // D s_477_8: cast reint s_477_7 -> u8
        let s_477_8: u8 = (s_477_7.value() as u8);
        // D s_477_9: cast zx s_477_8 -> bv
        let s_477_9: Bits = Bits::new(s_477_8 as u128, 5u16);
        // C s_477_10: const #0u : u8
        let s_477_10: u8 = 0;
        // C s_477_11: cast zx s_477_10 -> bv
        let s_477_11: Bits = Bits::new(s_477_10 as u128, 5u16);
        // D s_477_12: cmp-eq s_477_9 s_477_11
        let s_477_12: bool = ((s_477_9) == (s_477_11));
        // D s_477_13: write-var gs#373725 <= s_477_12
        fn_state.gs_373725 = s_477_12;
        // N s_477_14: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_478<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_478_0: const #70s : i
        let s_478_0: i128 = 70;
        // C s_478_1: const #14696u : u32
        let s_478_1: u32 = 14696;
        // D s_478_2: read-reg s_478_1:i
        let s_478_2: i128 = {
            let value = state.read_register::<i128>(s_478_1 as isize);
            tracer.read_register(s_478_1 as isize, value);
            value
        };
        // D s_478_3: cmp-lt s_478_2 s_478_0
        let s_478_3: bool = ((s_478_2) < (s_478_0));
        // D s_478_4: write-var gs#373713 <= s_478_3
        fn_state.gs_373713 = s_478_3;
        // N s_478_5: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_479<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_479_0: const #4s : i
        let s_479_0: i128 = 4;
        // D s_479_1: read-var u#23065:u32
        let s_479_1: u32 = fn_state.u_23065;
        // D s_479_2: cast zx s_479_1 -> bv
        let s_479_2: Bits = Bits::new(s_479_1 as u128, 32u16);
        // C s_479_3: const #1s : i64
        let s_479_3: i64 = 1;
        // C s_479_4: cast zx s_479_3 -> i
        let s_479_4: i128 = (i128::try_from(s_479_3).unwrap());
        // C s_479_5: const #0s : i
        let s_479_5: i128 = 0;
        // C s_479_6: add s_479_5 s_479_4
        let s_479_6: i128 = (s_479_5 + s_479_4);
        // D s_479_7: bit-extract s_479_2 s_479_0 s_479_6
        let s_479_7: Bits = (Bits::new(
            ((s_479_2) >> (s_479_0)).value(),
            u16::try_from(s_479_6).unwrap(),
        ));
        // D s_479_8: cast reint s_479_7 -> u8
        let s_479_8: bool = ((s_479_7.value()) != 0);
        // D s_479_9: cast zx s_479_8 -> bv
        let s_479_9: Bits = Bits::new(s_479_8 as u128, 1u16);
        // C s_479_10: const #1u : u8
        let s_479_10: bool = true;
        // C s_479_11: cast zx s_479_10 -> bv
        let s_479_11: Bits = Bits::new(s_479_10 as u128, 1u16);
        // D s_479_12: cmp-eq s_479_9 s_479_11
        let s_479_12: bool = ((s_479_9) == (s_479_11));
        // D s_479_13: write-var gs#373711 <= s_479_12
        fn_state.gs_373711 = s_479_12;
        // N s_479_14: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_480<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_480_0: const #69s : i
        let s_480_0: i128 = 69;
        // C s_480_1: const #14696u : u32
        let s_480_1: u32 = 14696;
        // D s_480_2: read-reg s_480_1:i
        let s_480_2: i128 = {
            let value = state.read_register::<i128>(s_480_1 as isize);
            tracer.read_register(s_480_1 as isize, value);
            value
        };
        // D s_480_3: cmp-lt s_480_2 s_480_0
        let s_480_3: bool = ((s_480_2) < (s_480_0));
        // D s_480_4: write-var gs#373699 <= s_480_3
        fn_state.gs_373699 = s_480_3;
        // N s_480_5: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_481<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_481_0: const #4s : i
        let s_481_0: i128 = 4;
        // D s_481_1: read-var u#23063:u32
        let s_481_1: u32 = fn_state.u_23063;
        // D s_481_2: cast zx s_481_1 -> bv
        let s_481_2: Bits = Bits::new(s_481_1 as u128, 32u16);
        // C s_481_3: const #1s : i64
        let s_481_3: i64 = 1;
        // C s_481_4: cast zx s_481_3 -> i
        let s_481_4: i128 = (i128::try_from(s_481_3).unwrap());
        // C s_481_5: const #0s : i
        let s_481_5: i128 = 0;
        // C s_481_6: add s_481_5 s_481_4
        let s_481_6: i128 = (s_481_5 + s_481_4);
        // D s_481_7: bit-extract s_481_2 s_481_0 s_481_6
        let s_481_7: Bits = (Bits::new(
            ((s_481_2) >> (s_481_0)).value(),
            u16::try_from(s_481_6).unwrap(),
        ));
        // D s_481_8: cast reint s_481_7 -> u8
        let s_481_8: bool = ((s_481_7.value()) != 0);
        // D s_481_9: cast zx s_481_8 -> bv
        let s_481_9: Bits = Bits::new(s_481_8 as u128, 1u16);
        // C s_481_10: const #0u : u8
        let s_481_10: bool = false;
        // C s_481_11: cast zx s_481_10 -> bv
        let s_481_11: Bits = Bits::new(s_481_10 as u128, 1u16);
        // D s_481_12: cmp-eq s_481_9 s_481_11
        let s_481_12: bool = ((s_481_9) == (s_481_11));
        // D s_481_13: write-var gs#373697 <= s_481_12
        fn_state.gs_373697 = s_481_12;
        // N s_481_14: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_482<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_482_0: const #68s : i
        let s_482_0: i128 = 68;
        // C s_482_1: const #14696u : u32
        let s_482_1: u32 = 14696;
        // D s_482_2: read-reg s_482_1:i
        let s_482_2: i128 = {
            let value = state.read_register::<i128>(s_482_1 as isize);
            tracer.read_register(s_482_1 as isize, value);
            value
        };
        // D s_482_3: cmp-lt s_482_2 s_482_0
        let s_482_3: bool = ((s_482_2) < (s_482_0));
        // D s_482_4: write-var gs#373685 <= s_482_3
        fn_state.gs_373685 = s_482_3;
        // N s_482_5: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_483<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_483_0: const #67s : i
        let s_483_0: i128 = 67;
        // C s_483_1: const #14696u : u32
        let s_483_1: u32 = 14696;
        // D s_483_2: read-reg s_483_1:i
        let s_483_2: i128 = {
            let value = state.read_register::<i128>(s_483_1 as isize);
            tracer.read_register(s_483_1 as isize, value);
            value
        };
        // D s_483_3: cmp-lt s_483_2 s_483_0
        let s_483_3: bool = ((s_483_2) < (s_483_0));
        // D s_483_4: write-var gs#373674 <= s_483_3
        fn_state.gs_373674 = s_483_3;
        // N s_483_5: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_484<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_484_0: const #66s : i
        let s_484_0: i128 = 66;
        // C s_484_1: const #14696u : u32
        let s_484_1: u32 = 14696;
        // D s_484_2: read-reg s_484_1:i
        let s_484_2: i128 = {
            let value = state.read_register::<i128>(s_484_1 as isize);
            tracer.read_register(s_484_1 as isize, value);
            value
        };
        // D s_484_3: cmp-lt s_484_2 s_484_0
        let s_484_3: bool = ((s_484_2) < (s_484_0));
        // D s_484_4: write-var gs#373654 <= s_484_3
        fn_state.gs_373654 = s_484_3;
        // N s_484_5: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_485<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_485_0: const #65s : i
        let s_485_0: i128 = 65;
        // C s_485_1: const #14696u : u32
        let s_485_1: u32 = 14696;
        // D s_485_2: read-reg s_485_1:i
        let s_485_2: i128 = {
            let value = state.read_register::<i128>(s_485_1 as isize);
            tracer.read_register(s_485_1 as isize, value);
            value
        };
        // D s_485_3: cmp-lt s_485_2 s_485_0
        let s_485_3: bool = ((s_485_2) < (s_485_0));
        // D s_485_4: write-var gs#373645 <= s_485_3
        fn_state.gs_373645 = s_485_3;
        // N s_485_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_486<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_486_0: const #6s : i
        let s_486_0: i128 = 6;
        // D s_486_1: read-var u#23050:u32
        let s_486_1: u32 = fn_state.u_23050;
        // D s_486_2: cast zx s_486_1 -> bv
        let s_486_2: Bits = Bits::new(s_486_1 as u128, 32u16);
        // C s_486_3: const #1s : i64
        let s_486_3: i64 = 1;
        // C s_486_4: cast zx s_486_3 -> i
        let s_486_4: i128 = (i128::try_from(s_486_3).unwrap());
        // C s_486_5: const #2s : i
        let s_486_5: i128 = 2;
        // C s_486_6: add s_486_5 s_486_4
        let s_486_6: i128 = (s_486_5 + s_486_4);
        // D s_486_7: bit-extract s_486_2 s_486_0 s_486_6
        let s_486_7: Bits = (Bits::new(
            ((s_486_2) >> (s_486_0)).value(),
            u16::try_from(s_486_6).unwrap(),
        ));
        // D s_486_8: cast reint s_486_7 -> u8
        let s_486_8: u8 = (s_486_7.value() as u8);
        // D s_486_9: cast zx s_486_8 -> bv
        let s_486_9: Bits = Bits::new(s_486_8 as u128, 3u16);
        // C s_486_10: const #7u : u8
        let s_486_10: u8 = 7;
        // C s_486_11: cast zx s_486_10 -> bv
        let s_486_11: Bits = Bits::new(s_486_10 as u128, 3u16);
        // D s_486_12: cmp-eq s_486_9 s_486_11
        let s_486_12: bool = ((s_486_9) == (s_486_11));
        // N s_486_13: branch s_486_12 b489 b487
        if s_486_12 {
            return block_489(state, tracer, fn_state);
        } else {
            return block_487(state, tracer, fn_state);
        };
    }
    fn block_487<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_487_0: const #0u : u8
        let s_487_0: bool = false;
        // D s_487_1: write-var gs#373642 <= s_487_0
        fn_state.gs_373642 = s_487_0;
        // N s_487_2: jump b488
        return block_488(state, tracer, fn_state);
    }
    fn block_488<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_488_0: read-var gs#373642:u8
        let s_488_0: bool = fn_state.gs_373642;
        // D s_488_1: write-var gs#373643 <= s_488_0
        fn_state.gs_373643 = s_488_0;
        // N s_488_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_489<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_489_0: const #0s : i
        let s_489_0: i128 = 0;
        // D s_489_1: read-var u#23050:u32
        let s_489_1: u32 = fn_state.u_23050;
        // D s_489_2: cast zx s_489_1 -> bv
        let s_489_2: Bits = Bits::new(s_489_1 as u128, 32u16);
        // C s_489_3: const #1s : i64
        let s_489_3: i64 = 1;
        // C s_489_4: cast zx s_489_3 -> i
        let s_489_4: i128 = (i128::try_from(s_489_3).unwrap());
        // C s_489_5: const #4s : i
        let s_489_5: i128 = 4;
        // C s_489_6: add s_489_5 s_489_4
        let s_489_6: i128 = (s_489_5 + s_489_4);
        // D s_489_7: bit-extract s_489_2 s_489_0 s_489_6
        let s_489_7: Bits = (Bits::new(
            ((s_489_2) >> (s_489_0)).value(),
            u16::try_from(s_489_6).unwrap(),
        ));
        // D s_489_8: cast reint s_489_7 -> u8
        let s_489_8: u8 = (s_489_7.value() as u8);
        // D s_489_9: cast zx s_489_8 -> bv
        let s_489_9: Bits = Bits::new(s_489_8 as u128, 5u16);
        // C s_489_10: const #31u : u8
        let s_489_10: u8 = 31;
        // C s_489_11: cast zx s_489_10 -> bv
        let s_489_11: Bits = Bits::new(s_489_10 as u128, 5u16);
        // D s_489_12: cmp-eq s_489_9 s_489_11
        let s_489_12: bool = ((s_489_9) == (s_489_11));
        // D s_489_13: write-var gs#373642 <= s_489_12
        fn_state.gs_373642 = s_489_12;
        // N s_489_14: jump b488
        return block_488(state, tracer, fn_state);
    }
    fn block_490<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_490_0: const #63s : i
        let s_490_0: i128 = 63;
        // C s_490_1: const #14696u : u32
        let s_490_1: u32 = 14696;
        // D s_490_2: read-reg s_490_1:i
        let s_490_2: i128 = {
            let value = state.read_register::<i128>(s_490_1 as isize);
            tracer.read_register(s_490_1 as isize, value);
            value
        };
        // D s_490_3: cmp-lt s_490_2 s_490_0
        let s_490_3: bool = ((s_490_2) < (s_490_0));
        // D s_490_4: write-var gs#373628 <= s_490_3
        fn_state.gs_373628 = s_490_3;
        // N s_490_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_491<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_491_0: const #6s : i
        let s_491_0: i128 = 6;
        // D s_491_1: read-var __opcode:u32
        let s_491_1: u32 = fn_state.u__opcode;
        // D s_491_2: cast zx s_491_1 -> bv
        let s_491_2: Bits = Bits::new(s_491_1 as u128, 32u16);
        // C s_491_3: const #1s : i64
        let s_491_3: i64 = 1;
        // C s_491_4: cast zx s_491_3 -> i
        let s_491_4: i128 = (i128::try_from(s_491_3).unwrap());
        // C s_491_5: const #2s : i
        let s_491_5: i128 = 2;
        // C s_491_6: add s_491_5 s_491_4
        let s_491_6: i128 = (s_491_5 + s_491_4);
        // D s_491_7: bit-extract s_491_2 s_491_0 s_491_6
        let s_491_7: Bits = (Bits::new(
            ((s_491_2) >> (s_491_0)).value(),
            u16::try_from(s_491_6).unwrap(),
        ));
        // D s_491_8: cast reint s_491_7 -> u8
        let s_491_8: u8 = (s_491_7.value() as u8);
        // D s_491_9: cast zx s_491_8 -> bv
        let s_491_9: Bits = Bits::new(s_491_8 as u128, 3u16);
        // C s_491_10: const #6u : u8
        let s_491_10: u8 = 6;
        // C s_491_11: cast zx s_491_10 -> bv
        let s_491_11: Bits = Bits::new(s_491_10 as u128, 3u16);
        // D s_491_12: cmp-eq s_491_9 s_491_11
        let s_491_12: bool = ((s_491_9) == (s_491_11));
        // N s_491_13: branch s_491_12 b494 b492
        if s_491_12 {
            return block_494(state, tracer, fn_state);
        } else {
            return block_492(state, tracer, fn_state);
        };
    }
    fn block_492<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_492_0: const #0u : u8
        let s_492_0: bool = false;
        // D s_492_1: write-var gs#373625 <= s_492_0
        fn_state.gs_373625 = s_492_0;
        // N s_492_2: jump b493
        return block_493(state, tracer, fn_state);
    }
    fn block_493<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_493_0: read-var gs#373625:u8
        let s_493_0: bool = fn_state.gs_373625;
        // D s_493_1: write-var gs#373626 <= s_493_0
        fn_state.gs_373626 = s_493_0;
        // N s_493_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_494<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_494_0: const #0s : i
        let s_494_0: i128 = 0;
        // D s_494_1: read-var __opcode:u32
        let s_494_1: u32 = fn_state.u__opcode;
        // D s_494_2: cast zx s_494_1 -> bv
        let s_494_2: Bits = Bits::new(s_494_1 as u128, 32u16);
        // C s_494_3: const #1s : i64
        let s_494_3: i64 = 1;
        // C s_494_4: cast zx s_494_3 -> i
        let s_494_4: i128 = (i128::try_from(s_494_3).unwrap());
        // C s_494_5: const #4s : i
        let s_494_5: i128 = 4;
        // C s_494_6: add s_494_5 s_494_4
        let s_494_6: i128 = (s_494_5 + s_494_4);
        // D s_494_7: bit-extract s_494_2 s_494_0 s_494_6
        let s_494_7: Bits = (Bits::new(
            ((s_494_2) >> (s_494_0)).value(),
            u16::try_from(s_494_6).unwrap(),
        ));
        // D s_494_8: cast reint s_494_7 -> u8
        let s_494_8: u8 = (s_494_7.value() as u8);
        // D s_494_9: cast zx s_494_8 -> bv
        let s_494_9: Bits = Bits::new(s_494_8 as u128, 5u16);
        // C s_494_10: const #31u : u8
        let s_494_10: u8 = 31;
        // C s_494_11: cast zx s_494_10 -> bv
        let s_494_11: Bits = Bits::new(s_494_10 as u128, 5u16);
        // D s_494_12: cmp-eq s_494_9 s_494_11
        let s_494_12: bool = ((s_494_9) == (s_494_11));
        // D s_494_13: write-var gs#373625 <= s_494_12
        fn_state.gs_373625 = s_494_12;
        // N s_494_14: jump b493
        return block_493(state, tracer, fn_state);
    }
}
