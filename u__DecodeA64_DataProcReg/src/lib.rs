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
use decode_add_addsub_shift_aarch64_instrs_integer_arithmetic_add_sub_shiftedreg::*;
use decode_subps_aarch64_instrs_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags::*;
use decode_subs_addsub_shift_aarch64_instrs_integer_arithmetic_add_sub_shiftedreg::*;
use decode_crc32c_aarch64_instrs_integer_crc::*;
use decode_eor_log_shift_aarch64_instrs_integer_logical_shiftedreg::*;
use decode_smax_reg_aarch64_instrs_integer_arithmetic_max_min_smax_reg::*;
use decode_lslv_aarch64_instrs_integer_shift_variable::*;
use decode_crc32_aarch64_instrs_integer_crc::*;
use decode_sbcs_aarch64_instrs_integer_arithmetic_add_sub_carry::*;
use decode_ccmp_imm_aarch64_instrs_integer_conditional_compare_immediate::*;
use decode_and_log_shift_aarch64_instrs_integer_logical_shiftedreg::*;
use decode_adds_addsub_ext_aarch64_instrs_integer_arithmetic_add_sub_extendedreg::*;
use decode_irg_aarch64_instrs_integer_tags_mcinsertrandomtag::*;
use decode_ctz_aarch64_instrs_integer_arithmetic_unary_ctz::*;
use decode_rorv_aarch64_instrs_integer_shift_variable::*;
use decode_umaddl_aarch64_instrs_integer_arithmetic_mul_widening_32_64::*;
use decode_lsrv_aarch64_instrs_integer_shift_variable::*;
use decode_umin_reg_aarch64_instrs_integer_arithmetic_max_min_umin_reg::*;
use decode_subs_addsub_ext_aarch64_instrs_integer_arithmetic_add_sub_extendedreg::*;
use decode_add_addsub_ext_aarch64_instrs_integer_arithmetic_add_sub_extendedreg::*;
use decode_csel_aarch64_instrs_integer_conditional_select::*;
use decode_pacdb_aarch64_instrs_integer_pac_pacdb_dp_1src::*;
use decode_sbc_aarch64_instrs_integer_arithmetic_add_sub_carry::*;
use decode_autda_aarch64_instrs_integer_pac_autda_dp_1src::*;
use decode_pacda_aarch64_instrs_integer_pac_pacda_dp_1src::*;
use decode_msub_aarch64_instrs_integer_arithmetic_mul_uniform_add_sub::*;
use decode_smin_reg_aarch64_instrs_integer_arithmetic_max_min_smin_reg::*;
use decode_rbit_int_aarch64_instrs_integer_arithmetic_rbit::*;
use decode_pacga_aarch64_instrs_integer_pac_pacga_dp_2src::*;
use decode_umulh_aarch64_instrs_integer_arithmetic_mul_widening_64_128hi::*;
use decode_xpac_aarch64_instrs_integer_pac_strip_dp_1src::*;
use decode_adc_aarch64_instrs_integer_arithmetic_add_sub_carry::*;
use decode_smsubl_aarch64_instrs_integer_arithmetic_mul_widening_32_64::*;
use decode_sub_addsub_ext_aarch64_instrs_integer_arithmetic_add_sub_extendedreg::*;
use decode_clz_int_aarch64_instrs_integer_arithmetic_cnt::*;
use decode_autia_aarch64_instrs_integer_pac_autia_dp_1src::*;
use decode_autib_aarch64_instrs_integer_pac_autib_dp_1src::*;
use decode_orr_log_shift_aarch64_instrs_integer_logical_shiftedreg::*;
use decode_asrv_aarch64_instrs_integer_shift_variable::*;
use decode_umsubl_aarch64_instrs_integer_arithmetic_mul_widening_32_64::*;
use decode_cls_int_aarch64_instrs_integer_arithmetic_cnt::*;
use decode_csneg_aarch64_instrs_integer_conditional_select::*;
use decode_sdiv_aarch64_instrs_integer_arithmetic_div::*;
use decode_rmif_aarch64_instrs_integer_flags_rmif::*;
use decode_adds_addsub_shift_aarch64_instrs_integer_arithmetic_add_sub_shiftedreg::*;
use decode_rev16_int_aarch64_instrs_integer_arithmetic_rev::*;
use decode_sub_addsub_shift_aarch64_instrs_integer_arithmetic_add_sub_shiftedreg::*;
use decode_autdb_aarch64_instrs_integer_pac_autdb_dp_1src::*;
use decode_cnt_aarch64_instrs_integer_arithmetic_unary_cnt::*;
use decode_setf_aarch64_instrs_integer_flags_setf::*;
use decode_abs_aarch64_instrs_integer_arithmetic_unary_abs::*;
use decode_csinc_aarch64_instrs_integer_conditional_select::*;
use decode_madd_aarch64_instrs_integer_arithmetic_mul_uniform_add_sub::*;
use decode_pacib_aarch64_instrs_integer_pac_pacib_dp_1src::*;
use decode_eon_aarch64_instrs_integer_logical_shiftedreg::*;
use decode_adcs_aarch64_instrs_integer_arithmetic_add_sub_carry::*;
use decode_subp_aarch64_instrs_integer_arithmetic_pointer_mcsubtracttaggedaddress::*;
use decode_ccmp_reg_aarch64_instrs_integer_conditional_compare_register::*;
use decode_smaddl_aarch64_instrs_integer_arithmetic_mul_widening_32_64::*;
use decode_orn_log_shift_aarch64_instrs_integer_logical_shiftedreg::*;
use decode_ccmn_imm_aarch64_instrs_integer_conditional_compare_immediate::*;
use decode_smulh_aarch64_instrs_integer_arithmetic_mul_widening_64_128hi::*;
use decode_ccmn_reg_aarch64_instrs_integer_conditional_compare_register::*;
use decode_pacia_aarch64_instrs_integer_pac_pacia_dp_1src::*;
use decode_rev32_int_aarch64_instrs_integer_arithmetic_rev::*;
use decode_rev_aarch64_instrs_integer_arithmetic_rev::*;
use decode_bics_aarch64_instrs_integer_logical_shiftedreg::*;
use decode_bic_log_shift_aarch64_instrs_integer_logical_shiftedreg::*;
use decode_gmi_aarch64_instrs_integer_tags_mcinserttagmask::*;
use decode_ands_log_shift_aarch64_instrs_integer_logical_shiftedreg::*;
use decode_udiv_aarch64_instrs_integer_arithmetic_div::*;
use decode_csinv_aarch64_instrs_integer_conditional_select::*;
use decode_umax_reg_aarch64_instrs_integer_arithmetic_max_min_umax_reg::*;
use common::*;
pub fn u__DecodeA64_DataProcReg<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_382119: i128,
    gs_382120: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_383473: bool,
        gs_383545: bool,
        gs_382830: bool,
        u_25955: u32,
        gs_383328: bool,
        gs_382942: bool,
        gs_383627: bool,
        u_26083: u32,
        u_25843: u32,
        gs_382477: bool,
        gs_383034: bool,
        gs_382743: bool,
        gs_383351: bool,
        gs_382376: bool,
        u_25813: u32,
        gs_383195: bool,
        gs_382553: bool,
        u_26009: u32,
        gs_382605: bool,
        u_26144: u32,
        gs_383489: bool,
        u_26116: u32,
        gs_382279: bool,
        gs_382161: bool,
        gs_383326: bool,
        gs_382451: bool,
        gs_383306: bool,
        u_26004: u32,
        u_25915: u32,
        u_26096: u32,
        u_25803: u32,
        gs_383508: bool,
        gs_382139: bool,
        gs_383269: bool,
        gs_383211: bool,
        gs_383542: bool,
        gs_383433: bool,
        gs_383127: bool,
        gs_383226: bool,
        gs_382227: bool,
        u_25977: u32,
        u_26155: u8,
        gs_382940: bool,
        gs_382399: bool,
        gs_383012: bool,
        gs_383165: bool,
        gs_382964: bool,
        u_26035: u32,
        u_25863: u32,
        gs_383453: bool,
        gs_383308: bool,
        u_26174: u32,
        u_25727: u32,
        gs_383147: bool,
        u_25680: u32,
        gs_382185: bool,
        gs_382777: bool,
        gs_382607: bool,
        gs_382920: bool,
        gs_382852: bool,
        u_25923: u32,
        gs_383348: bool,
        u_26066: u32,
        gs_382653: bool,
        gs_382802: bool,
        gs_383517: bool,
        gs_382759: bool,
        gs_382579: bool,
        gs_383197: bool,
        u_25755: u32,
        gs_383523: bool,
        u_25890: u32,
        gs_382255: bool,
        gs_382780: bool,
        gs_383387: bool,
        u_26180: u32,
        u_26030: u32,
        gs_383287: bool,
        gs_383548: bool,
        gs_383514: bool,
        u_25685: u32,
        u_25765: u32,
        gs_382651: bool,
        gs_383431: bool,
        u_26108: u32,
        gs_383149: bool,
        u_25961: u32,
        gs_383551: bool,
        u_26149: bool,
        u_26148: u8,
        u_25793: u32,
        gs_382425: bool,
        gs_382501: bool,
        u_25894: u32,
        u_25833: u32,
        u_26151: u32,
        gs_382503: bool,
        gs_382711: bool,
        u__opcode: u32,
        gs_382729: bool,
        merge_var: ProductType7b8639ca40b2f578,
        gs_382423: bool,
        u_25944: u32,
        gs_382805: bool,
        gs_382253: bool,
        gs_383213: bool,
        gs_383103: bool,
        u_26055: u32,
        gs_382986: bool,
        gs_383607: bool,
        u_25950: u32,
        gs_382229: bool,
        gs_382918: bool,
        u_26089: u32,
        u_25709: u32,
        gs_382691: bool,
        gs_383557: bool,
        gs_382874: bool,
        gs_382141: bool,
        gs_382761: bool,
        u_25909: u32,
        u_26022: u32,
        u_25930: u32,
        gs_383491: bool,
        u_26016: u32,
        gs_383411: bool,
        gs_382305: bool,
        gs_383163: bool,
        gs_382449: bool,
        gs_383389: bool,
        u_25747: u32,
        gs_382693: bool,
        gs_383105: bool,
        gs_383288: bool,
        gs_382713: bool,
        u_26158: u32,
        gs_382183: bool,
        gs_382855: bool,
        gs_382962: bool,
        gs_383181: bool,
        u_26168: u32,
        u_25995: u32,
        u_26078: u32,
        gs_383010: bool,
        gs_382527: bool,
        gs_382475: bool,
        gs_382330: bool,
        u_25693: u32,
        u_26101: u32,
        gs_383365: bool,
        gs_382803: bool,
        gs_383349: bool,
        u_25701: u32,
        gs_382745: bool,
        gs_383625: bool,
        gs_383073: bool,
        u_25717: u32,
        u_25883: u32,
        gs_383367: bool,
        u_25986: u32,
        gs_383471: bool,
        gs_382827: bool,
        gs_383089: bool,
        gs_382904: bool,
        u_26145: u8,
        gs_382631: bool,
        gs_383036: bool,
        gs_383589: bool,
        gs_383179: bool,
        gs_383239: bool,
        gs_382889: bool,
        u_26154: u8,
        u_26072: u32,
        gs_383591: bool,
        gs_383525: bool,
        gs_383290: bool,
        gs_382778: bool,
        gs_382727: bool,
        u_25785: u32,
        u_25853: u32,
        gs_382353: bool,
        gs_382281: bool,
        u_26163: u32,
        gs_382125: bool,
        u_25737: u32,
        u_26045: u32,
        u_26138: u32,
        gs_383057: bool,
        gs_382853: bool,
        gs_383071: bool,
        u_26050: u32,
        u_26156: bool,
        u_25899: u32,
        gs_382581: bool,
        gs_382555: bool,
        gs_383575: bool,
        gs_383511: bool,
        gs_382163: bool,
        u_26132: u32,
        gs_382205: bool,
        u_25775: u32,
        gs_382828: bool,
        gs_383609: bool,
        gs_383573: bool,
        gs_382673: bool,
        gs_383455: bool,
        u_25938: u32,
        u_25823: u32,
        u_26147: u8,
        u_26152: u8,
        gs_382988: bool,
        u_26153: u8,
        gs_382529: bool,
        gs_382633: bool,
        u_25969: u32,
        u_25904: u32,
        u_26060: u32,
        gs_382207: bool,
        gs_383125: bool,
        gs_383087: bool,
        u_25876: u32,
        gs_383254: bool,
        gs_383409: bool,
        u_25869: u32,
        u_26040: u32,
        gs_383559: bool,
        u_26124: u32,
        gs_382671: bool,
        gs_382307: bool,
        u_26011: u32,
        u_26146: u8,
        gs_382119: i128,
        gs_382120: u32,
    }
    let fn_state = FunctionState {
        gs_382119,
        gs_382120,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var gs#382119:i
        let s_0_0: i128 = fn_state.gs_382119;
        // D s_0_1: write-var merge#var.0 <= s_0_0
        fn_state.merge_var._0 = s_0_0;
        // D s_0_2: read-var gs#382120:u32
        let s_0_2: u32 = fn_state.gs_382120;
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
        // C s_0_11: const #20s : i
        let s_0_11: i128 = 20;
        // C s_0_12: add s_0_11 s_0_10
        let s_0_12: i128 = (s_0_11 + s_0_10);
        // D s_0_13: bit-extract s_0_8 s_0_6 s_0_12
        let s_0_13: Bits = (Bits::new(
            ((s_0_8) >> (s_0_6)).value(),
            u16::try_from(s_0_12).unwrap(),
        ));
        // D s_0_14: cast reint s_0_13 -> u21
        let s_0_14: u32 = (s_0_13.value() as u32);
        // D s_0_15: cast zx s_0_14 -> bv
        let s_0_15: Bits = Bits::new(s_0_14 as u128, 21u16);
        // C s_0_16: const #1486856u : u21
        let s_0_16: u32 = 1486856;
        // C s_0_17: cast zx s_0_16 -> bv
        let s_0_17: Bits = Bits::new(s_0_16 as u128, 21u16);
        // D s_0_18: cmp-eq s_0_15 s_0_17
        let s_0_18: bool = ((s_0_15) == (s_0_17));
        // N s_0_19: branch s_0_18 b591 b1
        if s_0_18 {
            return block_591(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#382125 <= s_1_0
        fn_state.gs_382125 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#382125:u8
        let s_2_0: bool = fn_state.gs_382125;
        // D s_2_1: not s_2_0
        let s_2_1: bool = !s_2_0;
        // N s_2_2: branch s_2_1 b4 b3
        if s_2_1 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #1s : i
        let s_3_0: i128 = 1;
        // C s_3_1: const #14696u : u32
        let s_3_1: u32 = 14696;
        // N s_3_2: write-reg s_3_1 <= s_3_0
        let s_3_2: () = {
            state.write_register::<i128>(s_3_1 as isize, s_3_0);
            tracer.write_register(s_3_1 as isize, s_3_0);
        };
        // C s_3_3: const #0s : i
        let s_3_3: i128 = 0;
        // C s_3_4: const #5s : i
        let s_3_4: i128 = 5;
        // D s_3_5: read-var __opcode:u32
        let s_3_5: u32 = fn_state.u__opcode;
        // D s_3_6: cast zx s_3_5 -> bv
        let s_3_6: Bits = Bits::new(s_3_5 as u128, 32u16);
        // D s_3_7: bit-extract s_3_6 s_3_3 s_3_4
        let s_3_7: Bits = (Bits::new(
            ((s_3_6) >> (s_3_3)).value(),
            u16::try_from(s_3_4).unwrap(),
        ));
        // D s_3_8: cast reint s_3_7 -> u8
        let s_3_8: u8 = (s_3_7.value() as u8);
        // C s_3_9: const #5s : i
        let s_3_9: i128 = 5;
        // C s_3_10: const #5s : i
        let s_3_10: i128 = 5;
        // D s_3_11: read-var __opcode:u32
        let s_3_11: u32 = fn_state.u__opcode;
        // D s_3_12: cast zx s_3_11 -> bv
        let s_3_12: Bits = Bits::new(s_3_11 as u128, 32u16);
        // D s_3_13: bit-extract s_3_12 s_3_9 s_3_10
        let s_3_13: Bits = (Bits::new(
            ((s_3_12) >> (s_3_9)).value(),
            u16::try_from(s_3_10).unwrap(),
        ));
        // D s_3_14: cast reint s_3_13 -> u8
        let s_3_14: u8 = (s_3_13.value() as u8);
        // C s_3_15: const #31s : i
        let s_3_15: i128 = 31;
        // C s_3_16: const #1s : i
        let s_3_16: i128 = 1;
        // D s_3_17: read-var __opcode:u32
        let s_3_17: u32 = fn_state.u__opcode;
        // D s_3_18: cast zx s_3_17 -> bv
        let s_3_18: Bits = Bits::new(s_3_17 as u128, 32u16);
        // D s_3_19: bit-extract s_3_18 s_3_15 s_3_16
        let s_3_19: Bits = (Bits::new(
            ((s_3_18) >> (s_3_15)).value(),
            u16::try_from(s_3_16).unwrap(),
        ));
        // D s_3_20: cast reint s_3_19 -> u8
        let s_3_20: bool = ((s_3_19.value()) != 0);
        // D s_3_21: call decode_abs_aarch64_instrs_integer_arithmetic_unary_abs(s_3_8, s_3_14, s_3_20)
        let s_3_21: () = decode_abs_aarch64_instrs_integer_arithmetic_unary_abs(
            state,
            tracer,
            s_3_8,
            s_3_14,
            s_3_20,
        );
        // N s_3_22: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var merge#var.1:struct
        let s_4_0: u32 = fn_state.merge_var._1;
        // D s_4_1: write-var u#25680 <= s_4_0
        fn_state.u_25680 = s_4_0;
        // C s_4_2: const #21s : i
        let s_4_2: i128 = 21;
        // D s_4_3: read-var u#25680:u32
        let s_4_3: u32 = fn_state.u_25680;
        // D s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 32u16);
        // C s_4_5: const #1s : i64
        let s_4_5: i64 = 1;
        // C s_4_6: cast zx s_4_5 -> i
        let s_4_6: i128 = (i128::try_from(s_4_5).unwrap());
        // C s_4_7: const #9s : i
        let s_4_7: i128 = 9;
        // C s_4_8: add s_4_7 s_4_6
        let s_4_8: i128 = (s_4_7 + s_4_6);
        // D s_4_9: bit-extract s_4_4 s_4_2 s_4_8
        let s_4_9: Bits = (Bits::new(
            ((s_4_4) >> (s_4_2)).value(),
            u16::try_from(s_4_8).unwrap(),
        ));
        // D s_4_10: cast reint s_4_9 -> u10
        let s_4_10: u16 = (s_4_9.value() as u16);
        // D s_4_11: cast zx s_4_10 -> bv
        let s_4_11: Bits = Bits::new(s_4_10 as u128, 10u16);
        // C s_4_12: const #208u : u10
        let s_4_12: u16 = 208;
        // C s_4_13: cast zx s_4_12 -> bv
        let s_4_13: Bits = Bits::new(s_4_12 as u128, 10u16);
        // D s_4_14: cmp-eq s_4_11 s_4_13
        let s_4_14: bool = ((s_4_11) == (s_4_13));
        // N s_4_15: branch s_4_14 b590 b5
        if s_4_14 {
            return block_590(state, tracer, fn_state);
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
        // D s_5_1: write-var gs#382139 <= s_5_0
        fn_state.gs_382139 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#382139:u8
        let s_6_0: bool = fn_state.gs_382139;
        // N s_6_1: branch s_6_0 b589 b7
        if s_6_0 {
            return block_589(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#382141 <= s_7_0
        fn_state.gs_382141 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#382141:u8
        let s_8_0: bool = fn_state.gs_382141;
        // D s_8_1: not s_8_0
        let s_8_1: bool = !s_8_0;
        // N s_8_2: branch s_8_1 b10 b9
        if s_8_1 {
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
        // C s_9_0: const #6s : i
        let s_9_0: i128 = 6;
        // C s_9_1: const #14696u : u32
        let s_9_1: u32 = 14696;
        // N s_9_2: write-reg s_9_1 <= s_9_0
        let s_9_2: () = {
            state.write_register::<i128>(s_9_1 as isize, s_9_0);
            tracer.write_register(s_9_1 as isize, s_9_0);
        };
        // C s_9_3: const #0s : i
        let s_9_3: i128 = 0;
        // C s_9_4: const #5s : i
        let s_9_4: i128 = 5;
        // D s_9_5: read-var u#25680:u32
        let s_9_5: u32 = fn_state.u_25680;
        // D s_9_6: cast zx s_9_5 -> bv
        let s_9_6: Bits = Bits::new(s_9_5 as u128, 32u16);
        // D s_9_7: bit-extract s_9_6 s_9_3 s_9_4
        let s_9_7: Bits = (Bits::new(
            ((s_9_6) >> (s_9_3)).value(),
            u16::try_from(s_9_4).unwrap(),
        ));
        // D s_9_8: cast reint s_9_7 -> u8
        let s_9_8: u8 = (s_9_7.value() as u8);
        // C s_9_9: const #5s : i
        let s_9_9: i128 = 5;
        // C s_9_10: const #5s : i
        let s_9_10: i128 = 5;
        // D s_9_11: read-var u#25680:u32
        let s_9_11: u32 = fn_state.u_25680;
        // D s_9_12: cast zx s_9_11 -> bv
        let s_9_12: Bits = Bits::new(s_9_11 as u128, 32u16);
        // D s_9_13: bit-extract s_9_12 s_9_9 s_9_10
        let s_9_13: Bits = (Bits::new(
            ((s_9_12) >> (s_9_9)).value(),
            u16::try_from(s_9_10).unwrap(),
        ));
        // D s_9_14: cast reint s_9_13 -> u8
        let s_9_14: u8 = (s_9_13.value() as u8);
        // C s_9_15: const #16s : i
        let s_9_15: i128 = 16;
        // C s_9_16: const #5s : i
        let s_9_16: i128 = 5;
        // D s_9_17: read-var u#25680:u32
        let s_9_17: u32 = fn_state.u_25680;
        // D s_9_18: cast zx s_9_17 -> bv
        let s_9_18: Bits = Bits::new(s_9_17 as u128, 32u16);
        // D s_9_19: bit-extract s_9_18 s_9_15 s_9_16
        let s_9_19: Bits = (Bits::new(
            ((s_9_18) >> (s_9_15)).value(),
            u16::try_from(s_9_16).unwrap(),
        ));
        // D s_9_20: cast reint s_9_19 -> u8
        let s_9_20: u8 = (s_9_19.value() as u8);
        // C s_9_21: const #29s : i
        let s_9_21: i128 = 29;
        // C s_9_22: const #1s : i
        let s_9_22: i128 = 1;
        // D s_9_23: read-var u#25680:u32
        let s_9_23: u32 = fn_state.u_25680;
        // D s_9_24: cast zx s_9_23 -> bv
        let s_9_24: Bits = Bits::new(s_9_23 as u128, 32u16);
        // D s_9_25: bit-extract s_9_24 s_9_21 s_9_22
        let s_9_25: Bits = (Bits::new(
            ((s_9_24) >> (s_9_21)).value(),
            u16::try_from(s_9_22).unwrap(),
        ));
        // D s_9_26: cast reint s_9_25 -> u8
        let s_9_26: bool = ((s_9_25.value()) != 0);
        // C s_9_27: const #30s : i
        let s_9_27: i128 = 30;
        // C s_9_28: const #1s : i
        let s_9_28: i128 = 1;
        // D s_9_29: read-var u#25680:u32
        let s_9_29: u32 = fn_state.u_25680;
        // D s_9_30: cast zx s_9_29 -> bv
        let s_9_30: Bits = Bits::new(s_9_29 as u128, 32u16);
        // D s_9_31: bit-extract s_9_30 s_9_27 s_9_28
        let s_9_31: Bits = (Bits::new(
            ((s_9_30) >> (s_9_27)).value(),
            u16::try_from(s_9_28).unwrap(),
        ));
        // D s_9_32: cast reint s_9_31 -> u8
        let s_9_32: bool = ((s_9_31.value()) != 0);
        // C s_9_33: const #31s : i
        let s_9_33: i128 = 31;
        // C s_9_34: const #1s : i
        let s_9_34: i128 = 1;
        // D s_9_35: read-var u#25680:u32
        let s_9_35: u32 = fn_state.u_25680;
        // D s_9_36: cast zx s_9_35 -> bv
        let s_9_36: Bits = Bits::new(s_9_35 as u128, 32u16);
        // D s_9_37: bit-extract s_9_36 s_9_33 s_9_34
        let s_9_37: Bits = (Bits::new(
            ((s_9_36) >> (s_9_33)).value(),
            u16::try_from(s_9_34).unwrap(),
        ));
        // D s_9_38: cast reint s_9_37 -> u8
        let s_9_38: bool = ((s_9_37.value()) != 0);
        // D s_9_39: call decode_adc_aarch64_instrs_integer_arithmetic_add_sub_carry(s_9_8, s_9_14, s_9_20, s_9_26, s_9_32, s_9_38)
        let s_9_39: () = decode_adc_aarch64_instrs_integer_arithmetic_add_sub_carry(
            state,
            tracer,
            s_9_8,
            s_9_14,
            s_9_20,
            s_9_26,
            s_9_32,
            s_9_38,
        );
        // N s_9_40: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var merge#var.1:struct
        let s_10_0: u32 = fn_state.merge_var._1;
        // D s_10_1: write-var u#25685 <= s_10_0
        fn_state.u_25685 = s_10_0;
        // C s_10_2: const #21s : i
        let s_10_2: i128 = 21;
        // D s_10_3: read-var u#25685:u32
        let s_10_3: u32 = fn_state.u_25685;
        // D s_10_4: cast zx s_10_3 -> bv
        let s_10_4: Bits = Bits::new(s_10_3 as u128, 32u16);
        // C s_10_5: const #1s : i64
        let s_10_5: i64 = 1;
        // C s_10_6: cast zx s_10_5 -> i
        let s_10_6: i128 = (i128::try_from(s_10_5).unwrap());
        // C s_10_7: const #9s : i
        let s_10_7: i128 = 9;
        // C s_10_8: add s_10_7 s_10_6
        let s_10_8: i128 = (s_10_7 + s_10_6);
        // D s_10_9: bit-extract s_10_4 s_10_2 s_10_8
        let s_10_9: Bits = (Bits::new(
            ((s_10_4) >> (s_10_2)).value(),
            u16::try_from(s_10_8).unwrap(),
        ));
        // D s_10_10: cast reint s_10_9 -> u10
        let s_10_10: u16 = (s_10_9.value() as u16);
        // D s_10_11: cast zx s_10_10 -> bv
        let s_10_11: Bits = Bits::new(s_10_10 as u128, 10u16);
        // C s_10_12: const #464u : u10
        let s_10_12: u16 = 464;
        // C s_10_13: cast zx s_10_12 -> bv
        let s_10_13: Bits = Bits::new(s_10_12 as u128, 10u16);
        // D s_10_14: cmp-eq s_10_11 s_10_13
        let s_10_14: bool = ((s_10_11) == (s_10_13));
        // N s_10_15: branch s_10_14 b588 b11
        if s_10_14 {
            return block_588(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#382161 <= s_11_0
        fn_state.gs_382161 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#382161:u8
        let s_12_0: bool = fn_state.gs_382161;
        // N s_12_1: branch s_12_0 b587 b13
        if s_12_0 {
            return block_587(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#382163 <= s_13_0
        fn_state.gs_382163 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#382163:u8
        let s_14_0: bool = fn_state.gs_382163;
        // D s_14_1: not s_14_0
        let s_14_1: bool = !s_14_0;
        // N s_14_2: branch s_14_1 b16 b15
        if s_14_1 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #7s : i
        let s_15_0: i128 = 7;
        // C s_15_1: const #14696u : u32
        let s_15_1: u32 = 14696;
        // N s_15_2: write-reg s_15_1 <= s_15_0
        let s_15_2: () = {
            state.write_register::<i128>(s_15_1 as isize, s_15_0);
            tracer.write_register(s_15_1 as isize, s_15_0);
        };
        // C s_15_3: const #0s : i
        let s_15_3: i128 = 0;
        // C s_15_4: const #5s : i
        let s_15_4: i128 = 5;
        // D s_15_5: read-var u#25685:u32
        let s_15_5: u32 = fn_state.u_25685;
        // D s_15_6: cast zx s_15_5 -> bv
        let s_15_6: Bits = Bits::new(s_15_5 as u128, 32u16);
        // D s_15_7: bit-extract s_15_6 s_15_3 s_15_4
        let s_15_7: Bits = (Bits::new(
            ((s_15_6) >> (s_15_3)).value(),
            u16::try_from(s_15_4).unwrap(),
        ));
        // D s_15_8: cast reint s_15_7 -> u8
        let s_15_8: u8 = (s_15_7.value() as u8);
        // C s_15_9: const #5s : i
        let s_15_9: i128 = 5;
        // C s_15_10: const #5s : i
        let s_15_10: i128 = 5;
        // D s_15_11: read-var u#25685:u32
        let s_15_11: u32 = fn_state.u_25685;
        // D s_15_12: cast zx s_15_11 -> bv
        let s_15_12: Bits = Bits::new(s_15_11 as u128, 32u16);
        // D s_15_13: bit-extract s_15_12 s_15_9 s_15_10
        let s_15_13: Bits = (Bits::new(
            ((s_15_12) >> (s_15_9)).value(),
            u16::try_from(s_15_10).unwrap(),
        ));
        // D s_15_14: cast reint s_15_13 -> u8
        let s_15_14: u8 = (s_15_13.value() as u8);
        // C s_15_15: const #16s : i
        let s_15_15: i128 = 16;
        // C s_15_16: const #5s : i
        let s_15_16: i128 = 5;
        // D s_15_17: read-var u#25685:u32
        let s_15_17: u32 = fn_state.u_25685;
        // D s_15_18: cast zx s_15_17 -> bv
        let s_15_18: Bits = Bits::new(s_15_17 as u128, 32u16);
        // D s_15_19: bit-extract s_15_18 s_15_15 s_15_16
        let s_15_19: Bits = (Bits::new(
            ((s_15_18) >> (s_15_15)).value(),
            u16::try_from(s_15_16).unwrap(),
        ));
        // D s_15_20: cast reint s_15_19 -> u8
        let s_15_20: u8 = (s_15_19.value() as u8);
        // C s_15_21: const #29s : i
        let s_15_21: i128 = 29;
        // C s_15_22: const #1s : i
        let s_15_22: i128 = 1;
        // D s_15_23: read-var u#25685:u32
        let s_15_23: u32 = fn_state.u_25685;
        // D s_15_24: cast zx s_15_23 -> bv
        let s_15_24: Bits = Bits::new(s_15_23 as u128, 32u16);
        // D s_15_25: bit-extract s_15_24 s_15_21 s_15_22
        let s_15_25: Bits = (Bits::new(
            ((s_15_24) >> (s_15_21)).value(),
            u16::try_from(s_15_22).unwrap(),
        ));
        // D s_15_26: cast reint s_15_25 -> u8
        let s_15_26: bool = ((s_15_25.value()) != 0);
        // C s_15_27: const #30s : i
        let s_15_27: i128 = 30;
        // C s_15_28: const #1s : i
        let s_15_28: i128 = 1;
        // D s_15_29: read-var u#25685:u32
        let s_15_29: u32 = fn_state.u_25685;
        // D s_15_30: cast zx s_15_29 -> bv
        let s_15_30: Bits = Bits::new(s_15_29 as u128, 32u16);
        // D s_15_31: bit-extract s_15_30 s_15_27 s_15_28
        let s_15_31: Bits = (Bits::new(
            ((s_15_30) >> (s_15_27)).value(),
            u16::try_from(s_15_28).unwrap(),
        ));
        // D s_15_32: cast reint s_15_31 -> u8
        let s_15_32: bool = ((s_15_31.value()) != 0);
        // C s_15_33: const #31s : i
        let s_15_33: i128 = 31;
        // C s_15_34: const #1s : i
        let s_15_34: i128 = 1;
        // D s_15_35: read-var u#25685:u32
        let s_15_35: u32 = fn_state.u_25685;
        // D s_15_36: cast zx s_15_35 -> bv
        let s_15_36: Bits = Bits::new(s_15_35 as u128, 32u16);
        // D s_15_37: bit-extract s_15_36 s_15_33 s_15_34
        let s_15_37: Bits = (Bits::new(
            ((s_15_36) >> (s_15_33)).value(),
            u16::try_from(s_15_34).unwrap(),
        ));
        // D s_15_38: cast reint s_15_37 -> u8
        let s_15_38: bool = ((s_15_37.value()) != 0);
        // D s_15_39: call decode_adcs_aarch64_instrs_integer_arithmetic_add_sub_carry(s_15_8, s_15_14, s_15_20, s_15_26, s_15_32, s_15_38)
        let s_15_39: () = decode_adcs_aarch64_instrs_integer_arithmetic_add_sub_carry(
            state,
            tracer,
            s_15_8,
            s_15_14,
            s_15_20,
            s_15_26,
            s_15_32,
            s_15_38,
        );
        // N s_15_40: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var merge#var.1:struct
        let s_16_0: u32 = fn_state.merge_var._1;
        // D s_16_1: write-var u#25693 <= s_16_0
        fn_state.u_25693 = s_16_0;
        // C s_16_2: const #21s : i
        let s_16_2: i128 = 21;
        // D s_16_3: read-var u#25693:u32
        let s_16_3: u32 = fn_state.u_25693;
        // D s_16_4: cast zx s_16_3 -> bv
        let s_16_4: Bits = Bits::new(s_16_3 as u128, 32u16);
        // C s_16_5: const #1s : i64
        let s_16_5: i64 = 1;
        // C s_16_6: cast zx s_16_5 -> i
        let s_16_6: i128 = (i128::try_from(s_16_5).unwrap());
        // C s_16_7: const #9s : i
        let s_16_7: i128 = 9;
        // C s_16_8: add s_16_7 s_16_6
        let s_16_8: i128 = (s_16_7 + s_16_6);
        // D s_16_9: bit-extract s_16_4 s_16_2 s_16_8
        let s_16_9: Bits = (Bits::new(
            ((s_16_4) >> (s_16_2)).value(),
            u16::try_from(s_16_8).unwrap(),
        ));
        // D s_16_10: cast reint s_16_9 -> u10
        let s_16_10: u16 = (s_16_9.value() as u16);
        // D s_16_11: cast zx s_16_10 -> bv
        let s_16_11: Bits = Bits::new(s_16_10 as u128, 10u16);
        // C s_16_12: const #720u : u10
        let s_16_12: u16 = 720;
        // C s_16_13: cast zx s_16_12 -> bv
        let s_16_13: Bits = Bits::new(s_16_12 as u128, 10u16);
        // D s_16_14: cmp-eq s_16_11 s_16_13
        let s_16_14: bool = ((s_16_11) == (s_16_13));
        // N s_16_15: branch s_16_14 b586 b17
        if s_16_14 {
            return block_586(state, tracer, fn_state);
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
        // D s_17_1: write-var gs#382183 <= s_17_0
        fn_state.gs_382183 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#382183:u8
        let s_18_0: bool = fn_state.gs_382183;
        // N s_18_1: branch s_18_0 b585 b19
        if s_18_0 {
            return block_585(state, tracer, fn_state);
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
        // D s_19_1: write-var gs#382185 <= s_19_0
        fn_state.gs_382185 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#382185:u8
        let s_20_0: bool = fn_state.gs_382185;
        // D s_20_1: not s_20_0
        let s_20_1: bool = !s_20_0;
        // N s_20_2: branch s_20_1 b22 b21
        if s_20_1 {
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
        // C s_21_0: const #8s : i
        let s_21_0: i128 = 8;
        // C s_21_1: const #14696u : u32
        let s_21_1: u32 = 14696;
        // N s_21_2: write-reg s_21_1 <= s_21_0
        let s_21_2: () = {
            state.write_register::<i128>(s_21_1 as isize, s_21_0);
            tracer.write_register(s_21_1 as isize, s_21_0);
        };
        // C s_21_3: const #0s : i
        let s_21_3: i128 = 0;
        // C s_21_4: const #5s : i
        let s_21_4: i128 = 5;
        // D s_21_5: read-var u#25693:u32
        let s_21_5: u32 = fn_state.u_25693;
        // D s_21_6: cast zx s_21_5 -> bv
        let s_21_6: Bits = Bits::new(s_21_5 as u128, 32u16);
        // D s_21_7: bit-extract s_21_6 s_21_3 s_21_4
        let s_21_7: Bits = (Bits::new(
            ((s_21_6) >> (s_21_3)).value(),
            u16::try_from(s_21_4).unwrap(),
        ));
        // D s_21_8: cast reint s_21_7 -> u8
        let s_21_8: u8 = (s_21_7.value() as u8);
        // C s_21_9: const #5s : i
        let s_21_9: i128 = 5;
        // C s_21_10: const #5s : i
        let s_21_10: i128 = 5;
        // D s_21_11: read-var u#25693:u32
        let s_21_11: u32 = fn_state.u_25693;
        // D s_21_12: cast zx s_21_11 -> bv
        let s_21_12: Bits = Bits::new(s_21_11 as u128, 32u16);
        // D s_21_13: bit-extract s_21_12 s_21_9 s_21_10
        let s_21_13: Bits = (Bits::new(
            ((s_21_12) >> (s_21_9)).value(),
            u16::try_from(s_21_10).unwrap(),
        ));
        // D s_21_14: cast reint s_21_13 -> u8
        let s_21_14: u8 = (s_21_13.value() as u8);
        // C s_21_15: const #16s : i
        let s_21_15: i128 = 16;
        // C s_21_16: const #5s : i
        let s_21_16: i128 = 5;
        // D s_21_17: read-var u#25693:u32
        let s_21_17: u32 = fn_state.u_25693;
        // D s_21_18: cast zx s_21_17 -> bv
        let s_21_18: Bits = Bits::new(s_21_17 as u128, 32u16);
        // D s_21_19: bit-extract s_21_18 s_21_15 s_21_16
        let s_21_19: Bits = (Bits::new(
            ((s_21_18) >> (s_21_15)).value(),
            u16::try_from(s_21_16).unwrap(),
        ));
        // D s_21_20: cast reint s_21_19 -> u8
        let s_21_20: u8 = (s_21_19.value() as u8);
        // C s_21_21: const #29s : i
        let s_21_21: i128 = 29;
        // C s_21_22: const #1s : i
        let s_21_22: i128 = 1;
        // D s_21_23: read-var u#25693:u32
        let s_21_23: u32 = fn_state.u_25693;
        // D s_21_24: cast zx s_21_23 -> bv
        let s_21_24: Bits = Bits::new(s_21_23 as u128, 32u16);
        // D s_21_25: bit-extract s_21_24 s_21_21 s_21_22
        let s_21_25: Bits = (Bits::new(
            ((s_21_24) >> (s_21_21)).value(),
            u16::try_from(s_21_22).unwrap(),
        ));
        // D s_21_26: cast reint s_21_25 -> u8
        let s_21_26: bool = ((s_21_25.value()) != 0);
        // C s_21_27: const #30s : i
        let s_21_27: i128 = 30;
        // C s_21_28: const #1s : i
        let s_21_28: i128 = 1;
        // D s_21_29: read-var u#25693:u32
        let s_21_29: u32 = fn_state.u_25693;
        // D s_21_30: cast zx s_21_29 -> bv
        let s_21_30: Bits = Bits::new(s_21_29 as u128, 32u16);
        // D s_21_31: bit-extract s_21_30 s_21_27 s_21_28
        let s_21_31: Bits = (Bits::new(
            ((s_21_30) >> (s_21_27)).value(),
            u16::try_from(s_21_28).unwrap(),
        ));
        // D s_21_32: cast reint s_21_31 -> u8
        let s_21_32: bool = ((s_21_31.value()) != 0);
        // C s_21_33: const #31s : i
        let s_21_33: i128 = 31;
        // C s_21_34: const #1s : i
        let s_21_34: i128 = 1;
        // D s_21_35: read-var u#25693:u32
        let s_21_35: u32 = fn_state.u_25693;
        // D s_21_36: cast zx s_21_35 -> bv
        let s_21_36: Bits = Bits::new(s_21_35 as u128, 32u16);
        // D s_21_37: bit-extract s_21_36 s_21_33 s_21_34
        let s_21_37: Bits = (Bits::new(
            ((s_21_36) >> (s_21_33)).value(),
            u16::try_from(s_21_34).unwrap(),
        ));
        // D s_21_38: cast reint s_21_37 -> u8
        let s_21_38: bool = ((s_21_37.value()) != 0);
        // D s_21_39: call decode_sbc_aarch64_instrs_integer_arithmetic_add_sub_carry(s_21_8, s_21_14, s_21_20, s_21_26, s_21_32, s_21_38)
        let s_21_39: () = decode_sbc_aarch64_instrs_integer_arithmetic_add_sub_carry(
            state,
            tracer,
            s_21_8,
            s_21_14,
            s_21_20,
            s_21_26,
            s_21_32,
            s_21_38,
        );
        // N s_21_40: return
        return;
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var merge#var.1:struct
        let s_22_0: u32 = fn_state.merge_var._1;
        // D s_22_1: write-var u#25701 <= s_22_0
        fn_state.u_25701 = s_22_0;
        // C s_22_2: const #21s : i
        let s_22_2: i128 = 21;
        // D s_22_3: read-var u#25701:u32
        let s_22_3: u32 = fn_state.u_25701;
        // D s_22_4: cast zx s_22_3 -> bv
        let s_22_4: Bits = Bits::new(s_22_3 as u128, 32u16);
        // C s_22_5: const #1s : i64
        let s_22_5: i64 = 1;
        // C s_22_6: cast zx s_22_5 -> i
        let s_22_6: i128 = (i128::try_from(s_22_5).unwrap());
        // C s_22_7: const #9s : i
        let s_22_7: i128 = 9;
        // C s_22_8: add s_22_7 s_22_6
        let s_22_8: i128 = (s_22_7 + s_22_6);
        // D s_22_9: bit-extract s_22_4 s_22_2 s_22_8
        let s_22_9: Bits = (Bits::new(
            ((s_22_4) >> (s_22_2)).value(),
            u16::try_from(s_22_8).unwrap(),
        ));
        // D s_22_10: cast reint s_22_9 -> u10
        let s_22_10: u16 = (s_22_9.value() as u16);
        // D s_22_11: cast zx s_22_10 -> bv
        let s_22_11: Bits = Bits::new(s_22_10 as u128, 10u16);
        // C s_22_12: const #976u : u10
        let s_22_12: u16 = 976;
        // C s_22_13: cast zx s_22_12 -> bv
        let s_22_13: Bits = Bits::new(s_22_12 as u128, 10u16);
        // D s_22_14: cmp-eq s_22_11 s_22_13
        let s_22_14: bool = ((s_22_11) == (s_22_13));
        // N s_22_15: branch s_22_14 b584 b23
        if s_22_14 {
            return block_584(state, tracer, fn_state);
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
        // D s_23_1: write-var gs#382205 <= s_23_0
        fn_state.gs_382205 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#382205:u8
        let s_24_0: bool = fn_state.gs_382205;
        // N s_24_1: branch s_24_0 b583 b25
        if s_24_0 {
            return block_583(state, tracer, fn_state);
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
        // D s_25_1: write-var gs#382207 <= s_25_0
        fn_state.gs_382207 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#382207:u8
        let s_26_0: bool = fn_state.gs_382207;
        // D s_26_1: not s_26_0
        let s_26_1: bool = !s_26_0;
        // N s_26_2: branch s_26_1 b28 b27
        if s_26_1 {
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
        // C s_27_0: const #9s : i
        let s_27_0: i128 = 9;
        // C s_27_1: const #14696u : u32
        let s_27_1: u32 = 14696;
        // N s_27_2: write-reg s_27_1 <= s_27_0
        let s_27_2: () = {
            state.write_register::<i128>(s_27_1 as isize, s_27_0);
            tracer.write_register(s_27_1 as isize, s_27_0);
        };
        // C s_27_3: const #0s : i
        let s_27_3: i128 = 0;
        // C s_27_4: const #5s : i
        let s_27_4: i128 = 5;
        // D s_27_5: read-var u#25701:u32
        let s_27_5: u32 = fn_state.u_25701;
        // D s_27_6: cast zx s_27_5 -> bv
        let s_27_6: Bits = Bits::new(s_27_5 as u128, 32u16);
        // D s_27_7: bit-extract s_27_6 s_27_3 s_27_4
        let s_27_7: Bits = (Bits::new(
            ((s_27_6) >> (s_27_3)).value(),
            u16::try_from(s_27_4).unwrap(),
        ));
        // D s_27_8: cast reint s_27_7 -> u8
        let s_27_8: u8 = (s_27_7.value() as u8);
        // C s_27_9: const #5s : i
        let s_27_9: i128 = 5;
        // C s_27_10: const #5s : i
        let s_27_10: i128 = 5;
        // D s_27_11: read-var u#25701:u32
        let s_27_11: u32 = fn_state.u_25701;
        // D s_27_12: cast zx s_27_11 -> bv
        let s_27_12: Bits = Bits::new(s_27_11 as u128, 32u16);
        // D s_27_13: bit-extract s_27_12 s_27_9 s_27_10
        let s_27_13: Bits = (Bits::new(
            ((s_27_12) >> (s_27_9)).value(),
            u16::try_from(s_27_10).unwrap(),
        ));
        // D s_27_14: cast reint s_27_13 -> u8
        let s_27_14: u8 = (s_27_13.value() as u8);
        // C s_27_15: const #16s : i
        let s_27_15: i128 = 16;
        // C s_27_16: const #5s : i
        let s_27_16: i128 = 5;
        // D s_27_17: read-var u#25701:u32
        let s_27_17: u32 = fn_state.u_25701;
        // D s_27_18: cast zx s_27_17 -> bv
        let s_27_18: Bits = Bits::new(s_27_17 as u128, 32u16);
        // D s_27_19: bit-extract s_27_18 s_27_15 s_27_16
        let s_27_19: Bits = (Bits::new(
            ((s_27_18) >> (s_27_15)).value(),
            u16::try_from(s_27_16).unwrap(),
        ));
        // D s_27_20: cast reint s_27_19 -> u8
        let s_27_20: u8 = (s_27_19.value() as u8);
        // C s_27_21: const #29s : i
        let s_27_21: i128 = 29;
        // C s_27_22: const #1s : i
        let s_27_22: i128 = 1;
        // D s_27_23: read-var u#25701:u32
        let s_27_23: u32 = fn_state.u_25701;
        // D s_27_24: cast zx s_27_23 -> bv
        let s_27_24: Bits = Bits::new(s_27_23 as u128, 32u16);
        // D s_27_25: bit-extract s_27_24 s_27_21 s_27_22
        let s_27_25: Bits = (Bits::new(
            ((s_27_24) >> (s_27_21)).value(),
            u16::try_from(s_27_22).unwrap(),
        ));
        // D s_27_26: cast reint s_27_25 -> u8
        let s_27_26: bool = ((s_27_25.value()) != 0);
        // C s_27_27: const #30s : i
        let s_27_27: i128 = 30;
        // C s_27_28: const #1s : i
        let s_27_28: i128 = 1;
        // D s_27_29: read-var u#25701:u32
        let s_27_29: u32 = fn_state.u_25701;
        // D s_27_30: cast zx s_27_29 -> bv
        let s_27_30: Bits = Bits::new(s_27_29 as u128, 32u16);
        // D s_27_31: bit-extract s_27_30 s_27_27 s_27_28
        let s_27_31: Bits = (Bits::new(
            ((s_27_30) >> (s_27_27)).value(),
            u16::try_from(s_27_28).unwrap(),
        ));
        // D s_27_32: cast reint s_27_31 -> u8
        let s_27_32: bool = ((s_27_31.value()) != 0);
        // C s_27_33: const #31s : i
        let s_27_33: i128 = 31;
        // C s_27_34: const #1s : i
        let s_27_34: i128 = 1;
        // D s_27_35: read-var u#25701:u32
        let s_27_35: u32 = fn_state.u_25701;
        // D s_27_36: cast zx s_27_35 -> bv
        let s_27_36: Bits = Bits::new(s_27_35 as u128, 32u16);
        // D s_27_37: bit-extract s_27_36 s_27_33 s_27_34
        let s_27_37: Bits = (Bits::new(
            ((s_27_36) >> (s_27_33)).value(),
            u16::try_from(s_27_34).unwrap(),
        ));
        // D s_27_38: cast reint s_27_37 -> u8
        let s_27_38: bool = ((s_27_37.value()) != 0);
        // D s_27_39: call decode_sbcs_aarch64_instrs_integer_arithmetic_add_sub_carry(s_27_8, s_27_14, s_27_20, s_27_26, s_27_32, s_27_38)
        let s_27_39: () = decode_sbcs_aarch64_instrs_integer_arithmetic_add_sub_carry(
            state,
            tracer,
            s_27_8,
            s_27_14,
            s_27_20,
            s_27_26,
            s_27_32,
            s_27_38,
        );
        // N s_27_40: return
        return;
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var merge#var.1:struct
        let s_28_0: u32 = fn_state.merge_var._1;
        // D s_28_1: write-var u#25709 <= s_28_0
        fn_state.u_25709 = s_28_0;
        // C s_28_2: const #24s : i
        let s_28_2: i128 = 24;
        // D s_28_3: read-var u#25709:u32
        let s_28_3: u32 = fn_state.u_25709;
        // D s_28_4: cast zx s_28_3 -> bv
        let s_28_4: Bits = Bits::new(s_28_3 as u128, 32u16);
        // C s_28_5: const #1s : i64
        let s_28_5: i64 = 1;
        // C s_28_6: cast zx s_28_5 -> i
        let s_28_6: i128 = (i128::try_from(s_28_5).unwrap());
        // C s_28_7: const #6s : i
        let s_28_7: i128 = 6;
        // C s_28_8: add s_28_7 s_28_6
        let s_28_8: i128 = (s_28_7 + s_28_6);
        // D s_28_9: bit-extract s_28_4 s_28_2 s_28_8
        let s_28_9: Bits = (Bits::new(
            ((s_28_4) >> (s_28_2)).value(),
            u16::try_from(s_28_8).unwrap(),
        ));
        // D s_28_10: cast reint s_28_9 -> u8
        let s_28_10: u8 = (s_28_9.value() as u8);
        // D s_28_11: cast zx s_28_10 -> bv
        let s_28_11: Bits = Bits::new(s_28_10 as u128, 7u16);
        // C s_28_12: const #11u : u8
        let s_28_12: u8 = 11;
        // C s_28_13: cast zx s_28_12 -> bv
        let s_28_13: Bits = Bits::new(s_28_12 as u128, 7u16);
        // D s_28_14: cmp-eq s_28_11 s_28_13
        let s_28_14: bool = ((s_28_11) == (s_28_13));
        // N s_28_15: branch s_28_14 b582 b29
        if s_28_14 {
            return block_582(state, tracer, fn_state);
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
        // D s_29_1: write-var gs#382227 <= s_29_0
        fn_state.gs_382227 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#382227:u8
        let s_30_0: bool = fn_state.gs_382227;
        // N s_30_1: branch s_30_0 b581 b31
        if s_30_0 {
            return block_581(state, tracer, fn_state);
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
        // D s_31_1: write-var gs#382229 <= s_31_0
        fn_state.gs_382229 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#382229:u8
        let s_32_0: bool = fn_state.gs_382229;
        // D s_32_1: not s_32_0
        let s_32_1: bool = !s_32_0;
        // N s_32_2: branch s_32_1 b34 b33
        if s_32_1 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #14s : i
        let s_33_0: i128 = 14;
        // C s_33_1: const #14696u : u32
        let s_33_1: u32 = 14696;
        // N s_33_2: write-reg s_33_1 <= s_33_0
        let s_33_2: () = {
            state.write_register::<i128>(s_33_1 as isize, s_33_0);
            tracer.write_register(s_33_1 as isize, s_33_0);
        };
        // C s_33_3: const #0s : i
        let s_33_3: i128 = 0;
        // C s_33_4: const #5s : i
        let s_33_4: i128 = 5;
        // D s_33_5: read-var u#25709:u32
        let s_33_5: u32 = fn_state.u_25709;
        // D s_33_6: cast zx s_33_5 -> bv
        let s_33_6: Bits = Bits::new(s_33_5 as u128, 32u16);
        // D s_33_7: bit-extract s_33_6 s_33_3 s_33_4
        let s_33_7: Bits = (Bits::new(
            ((s_33_6) >> (s_33_3)).value(),
            u16::try_from(s_33_4).unwrap(),
        ));
        // D s_33_8: cast reint s_33_7 -> u8
        let s_33_8: u8 = (s_33_7.value() as u8);
        // C s_33_9: const #5s : i
        let s_33_9: i128 = 5;
        // C s_33_10: const #5s : i
        let s_33_10: i128 = 5;
        // D s_33_11: read-var u#25709:u32
        let s_33_11: u32 = fn_state.u_25709;
        // D s_33_12: cast zx s_33_11 -> bv
        let s_33_12: Bits = Bits::new(s_33_11 as u128, 32u16);
        // D s_33_13: bit-extract s_33_12 s_33_9 s_33_10
        let s_33_13: Bits = (Bits::new(
            ((s_33_12) >> (s_33_9)).value(),
            u16::try_from(s_33_10).unwrap(),
        ));
        // D s_33_14: cast reint s_33_13 -> u8
        let s_33_14: u8 = (s_33_13.value() as u8);
        // C s_33_15: const #10s : i
        let s_33_15: i128 = 10;
        // C s_33_16: const #6s : i
        let s_33_16: i128 = 6;
        // D s_33_17: read-var u#25709:u32
        let s_33_17: u32 = fn_state.u_25709;
        // D s_33_18: cast zx s_33_17 -> bv
        let s_33_18: Bits = Bits::new(s_33_17 as u128, 32u16);
        // D s_33_19: bit-extract s_33_18 s_33_15 s_33_16
        let s_33_19: Bits = (Bits::new(
            ((s_33_18) >> (s_33_15)).value(),
            u16::try_from(s_33_16).unwrap(),
        ));
        // D s_33_20: cast reint s_33_19 -> u8
        let s_33_20: u8 = (s_33_19.value() as u8);
        // C s_33_21: const #16s : i
        let s_33_21: i128 = 16;
        // C s_33_22: const #5s : i
        let s_33_22: i128 = 5;
        // D s_33_23: read-var u#25709:u32
        let s_33_23: u32 = fn_state.u_25709;
        // D s_33_24: cast zx s_33_23 -> bv
        let s_33_24: Bits = Bits::new(s_33_23 as u128, 32u16);
        // D s_33_25: bit-extract s_33_24 s_33_21 s_33_22
        let s_33_25: Bits = (Bits::new(
            ((s_33_24) >> (s_33_21)).value(),
            u16::try_from(s_33_22).unwrap(),
        ));
        // D s_33_26: cast reint s_33_25 -> u8
        let s_33_26: u8 = (s_33_25.value() as u8);
        // C s_33_27: const #22s : i
        let s_33_27: i128 = 22;
        // C s_33_28: const #2s : i
        let s_33_28: i128 = 2;
        // D s_33_29: read-var u#25709:u32
        let s_33_29: u32 = fn_state.u_25709;
        // D s_33_30: cast zx s_33_29 -> bv
        let s_33_30: Bits = Bits::new(s_33_29 as u128, 32u16);
        // D s_33_31: bit-extract s_33_30 s_33_27 s_33_28
        let s_33_31: Bits = (Bits::new(
            ((s_33_30) >> (s_33_27)).value(),
            u16::try_from(s_33_28).unwrap(),
        ));
        // D s_33_32: cast reint s_33_31 -> u8
        let s_33_32: u8 = (s_33_31.value() as u8);
        // C s_33_33: const #29s : i
        let s_33_33: i128 = 29;
        // C s_33_34: const #1s : i
        let s_33_34: i128 = 1;
        // D s_33_35: read-var u#25709:u32
        let s_33_35: u32 = fn_state.u_25709;
        // D s_33_36: cast zx s_33_35 -> bv
        let s_33_36: Bits = Bits::new(s_33_35 as u128, 32u16);
        // D s_33_37: bit-extract s_33_36 s_33_33 s_33_34
        let s_33_37: Bits = (Bits::new(
            ((s_33_36) >> (s_33_33)).value(),
            u16::try_from(s_33_34).unwrap(),
        ));
        // D s_33_38: cast reint s_33_37 -> u8
        let s_33_38: bool = ((s_33_37.value()) != 0);
        // C s_33_39: const #30s : i
        let s_33_39: i128 = 30;
        // C s_33_40: const #1s : i
        let s_33_40: i128 = 1;
        // D s_33_41: read-var u#25709:u32
        let s_33_41: u32 = fn_state.u_25709;
        // D s_33_42: cast zx s_33_41 -> bv
        let s_33_42: Bits = Bits::new(s_33_41 as u128, 32u16);
        // D s_33_43: bit-extract s_33_42 s_33_39 s_33_40
        let s_33_43: Bits = (Bits::new(
            ((s_33_42) >> (s_33_39)).value(),
            u16::try_from(s_33_40).unwrap(),
        ));
        // D s_33_44: cast reint s_33_43 -> u8
        let s_33_44: bool = ((s_33_43.value()) != 0);
        // C s_33_45: const #31s : i
        let s_33_45: i128 = 31;
        // C s_33_46: const #1s : i
        let s_33_46: i128 = 1;
        // D s_33_47: read-var u#25709:u32
        let s_33_47: u32 = fn_state.u_25709;
        // D s_33_48: cast zx s_33_47 -> bv
        let s_33_48: Bits = Bits::new(s_33_47 as u128, 32u16);
        // D s_33_49: bit-extract s_33_48 s_33_45 s_33_46
        let s_33_49: Bits = (Bits::new(
            ((s_33_48) >> (s_33_45)).value(),
            u16::try_from(s_33_46).unwrap(),
        ));
        // D s_33_50: cast reint s_33_49 -> u8
        let s_33_50: bool = ((s_33_49.value()) != 0);
        // D s_33_51: call decode_add_addsub_shift_aarch64_instrs_integer_arithmetic_add_sub_shiftedreg(s_33_8, s_33_14, s_33_20, s_33_26, s_33_32, s_33_38, s_33_44, s_33_50)
        let s_33_51: () = decode_add_addsub_shift_aarch64_instrs_integer_arithmetic_add_sub_shiftedreg(
            state,
            tracer,
            s_33_8,
            s_33_14,
            s_33_20,
            s_33_26,
            s_33_32,
            s_33_38,
            s_33_44,
            s_33_50,
        );
        // N s_33_52: return
        return;
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var merge#var.1:struct
        let s_34_0: u32 = fn_state.merge_var._1;
        // D s_34_1: write-var u#25717 <= s_34_0
        fn_state.u_25717 = s_34_0;
        // C s_34_2: const #24s : i
        let s_34_2: i128 = 24;
        // D s_34_3: read-var u#25717:u32
        let s_34_3: u32 = fn_state.u_25717;
        // D s_34_4: cast zx s_34_3 -> bv
        let s_34_4: Bits = Bits::new(s_34_3 as u128, 32u16);
        // C s_34_5: const #1s : i64
        let s_34_5: i64 = 1;
        // C s_34_6: cast zx s_34_5 -> i
        let s_34_6: i128 = (i128::try_from(s_34_5).unwrap());
        // C s_34_7: const #6s : i
        let s_34_7: i128 = 6;
        // C s_34_8: add s_34_7 s_34_6
        let s_34_8: i128 = (s_34_7 + s_34_6);
        // D s_34_9: bit-extract s_34_4 s_34_2 s_34_8
        let s_34_9: Bits = (Bits::new(
            ((s_34_4) >> (s_34_2)).value(),
            u16::try_from(s_34_8).unwrap(),
        ));
        // D s_34_10: cast reint s_34_9 -> u8
        let s_34_10: u8 = (s_34_9.value() as u8);
        // D s_34_11: cast zx s_34_10 -> bv
        let s_34_11: Bits = Bits::new(s_34_10 as u128, 7u16);
        // C s_34_12: const #43u : u8
        let s_34_12: u8 = 43;
        // C s_34_13: cast zx s_34_12 -> bv
        let s_34_13: Bits = Bits::new(s_34_12 as u128, 7u16);
        // D s_34_14: cmp-eq s_34_11 s_34_13
        let s_34_14: bool = ((s_34_11) == (s_34_13));
        // N s_34_15: branch s_34_14 b580 b35
        if s_34_14 {
            return block_580(state, tracer, fn_state);
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
        // D s_35_1: write-var gs#382253 <= s_35_0
        fn_state.gs_382253 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#382253:u8
        let s_36_0: bool = fn_state.gs_382253;
        // N s_36_1: branch s_36_0 b579 b37
        if s_36_0 {
            return block_579(state, tracer, fn_state);
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
        // D s_37_1: write-var gs#382255 <= s_37_0
        fn_state.gs_382255 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#382255:u8
        let s_38_0: bool = fn_state.gs_382255;
        // D s_38_1: not s_38_0
        let s_38_1: bool = !s_38_0;
        // N s_38_2: branch s_38_1 b40 b39
        if s_38_1 {
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
        // C s_39_0: const #15s : i
        let s_39_0: i128 = 15;
        // C s_39_1: const #14696u : u32
        let s_39_1: u32 = 14696;
        // N s_39_2: write-reg s_39_1 <= s_39_0
        let s_39_2: () = {
            state.write_register::<i128>(s_39_1 as isize, s_39_0);
            tracer.write_register(s_39_1 as isize, s_39_0);
        };
        // C s_39_3: const #0s : i
        let s_39_3: i128 = 0;
        // C s_39_4: const #5s : i
        let s_39_4: i128 = 5;
        // D s_39_5: read-var u#25717:u32
        let s_39_5: u32 = fn_state.u_25717;
        // D s_39_6: cast zx s_39_5 -> bv
        let s_39_6: Bits = Bits::new(s_39_5 as u128, 32u16);
        // D s_39_7: bit-extract s_39_6 s_39_3 s_39_4
        let s_39_7: Bits = (Bits::new(
            ((s_39_6) >> (s_39_3)).value(),
            u16::try_from(s_39_4).unwrap(),
        ));
        // D s_39_8: cast reint s_39_7 -> u8
        let s_39_8: u8 = (s_39_7.value() as u8);
        // C s_39_9: const #5s : i
        let s_39_9: i128 = 5;
        // C s_39_10: const #5s : i
        let s_39_10: i128 = 5;
        // D s_39_11: read-var u#25717:u32
        let s_39_11: u32 = fn_state.u_25717;
        // D s_39_12: cast zx s_39_11 -> bv
        let s_39_12: Bits = Bits::new(s_39_11 as u128, 32u16);
        // D s_39_13: bit-extract s_39_12 s_39_9 s_39_10
        let s_39_13: Bits = (Bits::new(
            ((s_39_12) >> (s_39_9)).value(),
            u16::try_from(s_39_10).unwrap(),
        ));
        // D s_39_14: cast reint s_39_13 -> u8
        let s_39_14: u8 = (s_39_13.value() as u8);
        // C s_39_15: const #10s : i
        let s_39_15: i128 = 10;
        // C s_39_16: const #6s : i
        let s_39_16: i128 = 6;
        // D s_39_17: read-var u#25717:u32
        let s_39_17: u32 = fn_state.u_25717;
        // D s_39_18: cast zx s_39_17 -> bv
        let s_39_18: Bits = Bits::new(s_39_17 as u128, 32u16);
        // D s_39_19: bit-extract s_39_18 s_39_15 s_39_16
        let s_39_19: Bits = (Bits::new(
            ((s_39_18) >> (s_39_15)).value(),
            u16::try_from(s_39_16).unwrap(),
        ));
        // D s_39_20: cast reint s_39_19 -> u8
        let s_39_20: u8 = (s_39_19.value() as u8);
        // C s_39_21: const #16s : i
        let s_39_21: i128 = 16;
        // C s_39_22: const #5s : i
        let s_39_22: i128 = 5;
        // D s_39_23: read-var u#25717:u32
        let s_39_23: u32 = fn_state.u_25717;
        // D s_39_24: cast zx s_39_23 -> bv
        let s_39_24: Bits = Bits::new(s_39_23 as u128, 32u16);
        // D s_39_25: bit-extract s_39_24 s_39_21 s_39_22
        let s_39_25: Bits = (Bits::new(
            ((s_39_24) >> (s_39_21)).value(),
            u16::try_from(s_39_22).unwrap(),
        ));
        // D s_39_26: cast reint s_39_25 -> u8
        let s_39_26: u8 = (s_39_25.value() as u8);
        // C s_39_27: const #22s : i
        let s_39_27: i128 = 22;
        // C s_39_28: const #2s : i
        let s_39_28: i128 = 2;
        // D s_39_29: read-var u#25717:u32
        let s_39_29: u32 = fn_state.u_25717;
        // D s_39_30: cast zx s_39_29 -> bv
        let s_39_30: Bits = Bits::new(s_39_29 as u128, 32u16);
        // D s_39_31: bit-extract s_39_30 s_39_27 s_39_28
        let s_39_31: Bits = (Bits::new(
            ((s_39_30) >> (s_39_27)).value(),
            u16::try_from(s_39_28).unwrap(),
        ));
        // D s_39_32: cast reint s_39_31 -> u8
        let s_39_32: u8 = (s_39_31.value() as u8);
        // C s_39_33: const #29s : i
        let s_39_33: i128 = 29;
        // C s_39_34: const #1s : i
        let s_39_34: i128 = 1;
        // D s_39_35: read-var u#25717:u32
        let s_39_35: u32 = fn_state.u_25717;
        // D s_39_36: cast zx s_39_35 -> bv
        let s_39_36: Bits = Bits::new(s_39_35 as u128, 32u16);
        // D s_39_37: bit-extract s_39_36 s_39_33 s_39_34
        let s_39_37: Bits = (Bits::new(
            ((s_39_36) >> (s_39_33)).value(),
            u16::try_from(s_39_34).unwrap(),
        ));
        // D s_39_38: cast reint s_39_37 -> u8
        let s_39_38: bool = ((s_39_37.value()) != 0);
        // C s_39_39: const #30s : i
        let s_39_39: i128 = 30;
        // C s_39_40: const #1s : i
        let s_39_40: i128 = 1;
        // D s_39_41: read-var u#25717:u32
        let s_39_41: u32 = fn_state.u_25717;
        // D s_39_42: cast zx s_39_41 -> bv
        let s_39_42: Bits = Bits::new(s_39_41 as u128, 32u16);
        // D s_39_43: bit-extract s_39_42 s_39_39 s_39_40
        let s_39_43: Bits = (Bits::new(
            ((s_39_42) >> (s_39_39)).value(),
            u16::try_from(s_39_40).unwrap(),
        ));
        // D s_39_44: cast reint s_39_43 -> u8
        let s_39_44: bool = ((s_39_43.value()) != 0);
        // C s_39_45: const #31s : i
        let s_39_45: i128 = 31;
        // C s_39_46: const #1s : i
        let s_39_46: i128 = 1;
        // D s_39_47: read-var u#25717:u32
        let s_39_47: u32 = fn_state.u_25717;
        // D s_39_48: cast zx s_39_47 -> bv
        let s_39_48: Bits = Bits::new(s_39_47 as u128, 32u16);
        // D s_39_49: bit-extract s_39_48 s_39_45 s_39_46
        let s_39_49: Bits = (Bits::new(
            ((s_39_48) >> (s_39_45)).value(),
            u16::try_from(s_39_46).unwrap(),
        ));
        // D s_39_50: cast reint s_39_49 -> u8
        let s_39_50: bool = ((s_39_49.value()) != 0);
        // D s_39_51: call decode_adds_addsub_shift_aarch64_instrs_integer_arithmetic_add_sub_shiftedreg(s_39_8, s_39_14, s_39_20, s_39_26, s_39_32, s_39_38, s_39_44, s_39_50)
        let s_39_51: () = decode_adds_addsub_shift_aarch64_instrs_integer_arithmetic_add_sub_shiftedreg(
            state,
            tracer,
            s_39_8,
            s_39_14,
            s_39_20,
            s_39_26,
            s_39_32,
            s_39_38,
            s_39_44,
            s_39_50,
        );
        // N s_39_52: return
        return;
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var merge#var.1:struct
        let s_40_0: u32 = fn_state.merge_var._1;
        // D s_40_1: write-var u#25727 <= s_40_0
        fn_state.u_25727 = s_40_0;
        // C s_40_2: const #24s : i
        let s_40_2: i128 = 24;
        // D s_40_3: read-var u#25727:u32
        let s_40_3: u32 = fn_state.u_25727;
        // D s_40_4: cast zx s_40_3 -> bv
        let s_40_4: Bits = Bits::new(s_40_3 as u128, 32u16);
        // C s_40_5: const #1s : i64
        let s_40_5: i64 = 1;
        // C s_40_6: cast zx s_40_5 -> i
        let s_40_6: i128 = (i128::try_from(s_40_5).unwrap());
        // C s_40_7: const #6s : i
        let s_40_7: i128 = 6;
        // C s_40_8: add s_40_7 s_40_6
        let s_40_8: i128 = (s_40_7 + s_40_6);
        // D s_40_9: bit-extract s_40_4 s_40_2 s_40_8
        let s_40_9: Bits = (Bits::new(
            ((s_40_4) >> (s_40_2)).value(),
            u16::try_from(s_40_8).unwrap(),
        ));
        // D s_40_10: cast reint s_40_9 -> u8
        let s_40_10: u8 = (s_40_9.value() as u8);
        // D s_40_11: cast zx s_40_10 -> bv
        let s_40_11: Bits = Bits::new(s_40_10 as u128, 7u16);
        // C s_40_12: const #75u : u8
        let s_40_12: u8 = 75;
        // C s_40_13: cast zx s_40_12 -> bv
        let s_40_13: Bits = Bits::new(s_40_12 as u128, 7u16);
        // D s_40_14: cmp-eq s_40_11 s_40_13
        let s_40_14: bool = ((s_40_11) == (s_40_13));
        // N s_40_15: branch s_40_14 b578 b41
        if s_40_14 {
            return block_578(state, tracer, fn_state);
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
        // D s_41_1: write-var gs#382279 <= s_41_0
        fn_state.gs_382279 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#382279:u8
        let s_42_0: bool = fn_state.gs_382279;
        // N s_42_1: branch s_42_0 b577 b43
        if s_42_0 {
            return block_577(state, tracer, fn_state);
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
        // D s_43_1: write-var gs#382281 <= s_43_0
        fn_state.gs_382281 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#382281:u8
        let s_44_0: bool = fn_state.gs_382281;
        // D s_44_1: not s_44_0
        let s_44_1: bool = !s_44_0;
        // N s_44_2: branch s_44_1 b46 b45
        if s_44_1 {
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
        // C s_45_0: const #16s : i
        let s_45_0: i128 = 16;
        // C s_45_1: const #14696u : u32
        let s_45_1: u32 = 14696;
        // N s_45_2: write-reg s_45_1 <= s_45_0
        let s_45_2: () = {
            state.write_register::<i128>(s_45_1 as isize, s_45_0);
            tracer.write_register(s_45_1 as isize, s_45_0);
        };
        // C s_45_3: const #0s : i
        let s_45_3: i128 = 0;
        // C s_45_4: const #5s : i
        let s_45_4: i128 = 5;
        // D s_45_5: read-var u#25727:u32
        let s_45_5: u32 = fn_state.u_25727;
        // D s_45_6: cast zx s_45_5 -> bv
        let s_45_6: Bits = Bits::new(s_45_5 as u128, 32u16);
        // D s_45_7: bit-extract s_45_6 s_45_3 s_45_4
        let s_45_7: Bits = (Bits::new(
            ((s_45_6) >> (s_45_3)).value(),
            u16::try_from(s_45_4).unwrap(),
        ));
        // D s_45_8: cast reint s_45_7 -> u8
        let s_45_8: u8 = (s_45_7.value() as u8);
        // C s_45_9: const #5s : i
        let s_45_9: i128 = 5;
        // C s_45_10: const #5s : i
        let s_45_10: i128 = 5;
        // D s_45_11: read-var u#25727:u32
        let s_45_11: u32 = fn_state.u_25727;
        // D s_45_12: cast zx s_45_11 -> bv
        let s_45_12: Bits = Bits::new(s_45_11 as u128, 32u16);
        // D s_45_13: bit-extract s_45_12 s_45_9 s_45_10
        let s_45_13: Bits = (Bits::new(
            ((s_45_12) >> (s_45_9)).value(),
            u16::try_from(s_45_10).unwrap(),
        ));
        // D s_45_14: cast reint s_45_13 -> u8
        let s_45_14: u8 = (s_45_13.value() as u8);
        // C s_45_15: const #10s : i
        let s_45_15: i128 = 10;
        // C s_45_16: const #6s : i
        let s_45_16: i128 = 6;
        // D s_45_17: read-var u#25727:u32
        let s_45_17: u32 = fn_state.u_25727;
        // D s_45_18: cast zx s_45_17 -> bv
        let s_45_18: Bits = Bits::new(s_45_17 as u128, 32u16);
        // D s_45_19: bit-extract s_45_18 s_45_15 s_45_16
        let s_45_19: Bits = (Bits::new(
            ((s_45_18) >> (s_45_15)).value(),
            u16::try_from(s_45_16).unwrap(),
        ));
        // D s_45_20: cast reint s_45_19 -> u8
        let s_45_20: u8 = (s_45_19.value() as u8);
        // C s_45_21: const #16s : i
        let s_45_21: i128 = 16;
        // C s_45_22: const #5s : i
        let s_45_22: i128 = 5;
        // D s_45_23: read-var u#25727:u32
        let s_45_23: u32 = fn_state.u_25727;
        // D s_45_24: cast zx s_45_23 -> bv
        let s_45_24: Bits = Bits::new(s_45_23 as u128, 32u16);
        // D s_45_25: bit-extract s_45_24 s_45_21 s_45_22
        let s_45_25: Bits = (Bits::new(
            ((s_45_24) >> (s_45_21)).value(),
            u16::try_from(s_45_22).unwrap(),
        ));
        // D s_45_26: cast reint s_45_25 -> u8
        let s_45_26: u8 = (s_45_25.value() as u8);
        // C s_45_27: const #22s : i
        let s_45_27: i128 = 22;
        // C s_45_28: const #2s : i
        let s_45_28: i128 = 2;
        // D s_45_29: read-var u#25727:u32
        let s_45_29: u32 = fn_state.u_25727;
        // D s_45_30: cast zx s_45_29 -> bv
        let s_45_30: Bits = Bits::new(s_45_29 as u128, 32u16);
        // D s_45_31: bit-extract s_45_30 s_45_27 s_45_28
        let s_45_31: Bits = (Bits::new(
            ((s_45_30) >> (s_45_27)).value(),
            u16::try_from(s_45_28).unwrap(),
        ));
        // D s_45_32: cast reint s_45_31 -> u8
        let s_45_32: u8 = (s_45_31.value() as u8);
        // C s_45_33: const #29s : i
        let s_45_33: i128 = 29;
        // C s_45_34: const #1s : i
        let s_45_34: i128 = 1;
        // D s_45_35: read-var u#25727:u32
        let s_45_35: u32 = fn_state.u_25727;
        // D s_45_36: cast zx s_45_35 -> bv
        let s_45_36: Bits = Bits::new(s_45_35 as u128, 32u16);
        // D s_45_37: bit-extract s_45_36 s_45_33 s_45_34
        let s_45_37: Bits = (Bits::new(
            ((s_45_36) >> (s_45_33)).value(),
            u16::try_from(s_45_34).unwrap(),
        ));
        // D s_45_38: cast reint s_45_37 -> u8
        let s_45_38: bool = ((s_45_37.value()) != 0);
        // C s_45_39: const #30s : i
        let s_45_39: i128 = 30;
        // C s_45_40: const #1s : i
        let s_45_40: i128 = 1;
        // D s_45_41: read-var u#25727:u32
        let s_45_41: u32 = fn_state.u_25727;
        // D s_45_42: cast zx s_45_41 -> bv
        let s_45_42: Bits = Bits::new(s_45_41 as u128, 32u16);
        // D s_45_43: bit-extract s_45_42 s_45_39 s_45_40
        let s_45_43: Bits = (Bits::new(
            ((s_45_42) >> (s_45_39)).value(),
            u16::try_from(s_45_40).unwrap(),
        ));
        // D s_45_44: cast reint s_45_43 -> u8
        let s_45_44: bool = ((s_45_43.value()) != 0);
        // C s_45_45: const #31s : i
        let s_45_45: i128 = 31;
        // C s_45_46: const #1s : i
        let s_45_46: i128 = 1;
        // D s_45_47: read-var u#25727:u32
        let s_45_47: u32 = fn_state.u_25727;
        // D s_45_48: cast zx s_45_47 -> bv
        let s_45_48: Bits = Bits::new(s_45_47 as u128, 32u16);
        // D s_45_49: bit-extract s_45_48 s_45_45 s_45_46
        let s_45_49: Bits = (Bits::new(
            ((s_45_48) >> (s_45_45)).value(),
            u16::try_from(s_45_46).unwrap(),
        ));
        // D s_45_50: cast reint s_45_49 -> u8
        let s_45_50: bool = ((s_45_49.value()) != 0);
        // D s_45_51: call decode_sub_addsub_shift_aarch64_instrs_integer_arithmetic_add_sub_shiftedreg(s_45_8, s_45_14, s_45_20, s_45_26, s_45_32, s_45_38, s_45_44, s_45_50)
        let s_45_51: () = decode_sub_addsub_shift_aarch64_instrs_integer_arithmetic_add_sub_shiftedreg(
            state,
            tracer,
            s_45_8,
            s_45_14,
            s_45_20,
            s_45_26,
            s_45_32,
            s_45_38,
            s_45_44,
            s_45_50,
        );
        // N s_45_52: return
        return;
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var merge#var.1:struct
        let s_46_0: u32 = fn_state.merge_var._1;
        // D s_46_1: write-var u#25737 <= s_46_0
        fn_state.u_25737 = s_46_0;
        // C s_46_2: const #24s : i
        let s_46_2: i128 = 24;
        // D s_46_3: read-var u#25737:u32
        let s_46_3: u32 = fn_state.u_25737;
        // D s_46_4: cast zx s_46_3 -> bv
        let s_46_4: Bits = Bits::new(s_46_3 as u128, 32u16);
        // C s_46_5: const #1s : i64
        let s_46_5: i64 = 1;
        // C s_46_6: cast zx s_46_5 -> i
        let s_46_6: i128 = (i128::try_from(s_46_5).unwrap());
        // C s_46_7: const #6s : i
        let s_46_7: i128 = 6;
        // C s_46_8: add s_46_7 s_46_6
        let s_46_8: i128 = (s_46_7 + s_46_6);
        // D s_46_9: bit-extract s_46_4 s_46_2 s_46_8
        let s_46_9: Bits = (Bits::new(
            ((s_46_4) >> (s_46_2)).value(),
            u16::try_from(s_46_8).unwrap(),
        ));
        // D s_46_10: cast reint s_46_9 -> u8
        let s_46_10: u8 = (s_46_9.value() as u8);
        // D s_46_11: cast zx s_46_10 -> bv
        let s_46_11: Bits = Bits::new(s_46_10 as u128, 7u16);
        // C s_46_12: const #107u : u8
        let s_46_12: u8 = 107;
        // C s_46_13: cast zx s_46_12 -> bv
        let s_46_13: Bits = Bits::new(s_46_12 as u128, 7u16);
        // D s_46_14: cmp-eq s_46_11 s_46_13
        let s_46_14: bool = ((s_46_11) == (s_46_13));
        // N s_46_15: branch s_46_14 b576 b47
        if s_46_14 {
            return block_576(state, tracer, fn_state);
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
        // D s_47_1: write-var gs#382305 <= s_47_0
        fn_state.gs_382305 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#382305:u8
        let s_48_0: bool = fn_state.gs_382305;
        // N s_48_1: branch s_48_0 b575 b49
        if s_48_0 {
            return block_575(state, tracer, fn_state);
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
        // D s_49_1: write-var gs#382307 <= s_49_0
        fn_state.gs_382307 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#382307:u8
        let s_50_0: bool = fn_state.gs_382307;
        // D s_50_1: not s_50_0
        let s_50_1: bool = !s_50_0;
        // N s_50_2: branch s_50_1 b52 b51
        if s_50_1 {
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
        // C s_51_0: const #17s : i
        let s_51_0: i128 = 17;
        // C s_51_1: const #14696u : u32
        let s_51_1: u32 = 14696;
        // N s_51_2: write-reg s_51_1 <= s_51_0
        let s_51_2: () = {
            state.write_register::<i128>(s_51_1 as isize, s_51_0);
            tracer.write_register(s_51_1 as isize, s_51_0);
        };
        // C s_51_3: const #0s : i
        let s_51_3: i128 = 0;
        // C s_51_4: const #5s : i
        let s_51_4: i128 = 5;
        // D s_51_5: read-var u#25737:u32
        let s_51_5: u32 = fn_state.u_25737;
        // D s_51_6: cast zx s_51_5 -> bv
        let s_51_6: Bits = Bits::new(s_51_5 as u128, 32u16);
        // D s_51_7: bit-extract s_51_6 s_51_3 s_51_4
        let s_51_7: Bits = (Bits::new(
            ((s_51_6) >> (s_51_3)).value(),
            u16::try_from(s_51_4).unwrap(),
        ));
        // D s_51_8: cast reint s_51_7 -> u8
        let s_51_8: u8 = (s_51_7.value() as u8);
        // C s_51_9: const #5s : i
        let s_51_9: i128 = 5;
        // C s_51_10: const #5s : i
        let s_51_10: i128 = 5;
        // D s_51_11: read-var u#25737:u32
        let s_51_11: u32 = fn_state.u_25737;
        // D s_51_12: cast zx s_51_11 -> bv
        let s_51_12: Bits = Bits::new(s_51_11 as u128, 32u16);
        // D s_51_13: bit-extract s_51_12 s_51_9 s_51_10
        let s_51_13: Bits = (Bits::new(
            ((s_51_12) >> (s_51_9)).value(),
            u16::try_from(s_51_10).unwrap(),
        ));
        // D s_51_14: cast reint s_51_13 -> u8
        let s_51_14: u8 = (s_51_13.value() as u8);
        // C s_51_15: const #10s : i
        let s_51_15: i128 = 10;
        // C s_51_16: const #6s : i
        let s_51_16: i128 = 6;
        // D s_51_17: read-var u#25737:u32
        let s_51_17: u32 = fn_state.u_25737;
        // D s_51_18: cast zx s_51_17 -> bv
        let s_51_18: Bits = Bits::new(s_51_17 as u128, 32u16);
        // D s_51_19: bit-extract s_51_18 s_51_15 s_51_16
        let s_51_19: Bits = (Bits::new(
            ((s_51_18) >> (s_51_15)).value(),
            u16::try_from(s_51_16).unwrap(),
        ));
        // D s_51_20: cast reint s_51_19 -> u8
        let s_51_20: u8 = (s_51_19.value() as u8);
        // C s_51_21: const #16s : i
        let s_51_21: i128 = 16;
        // C s_51_22: const #5s : i
        let s_51_22: i128 = 5;
        // D s_51_23: read-var u#25737:u32
        let s_51_23: u32 = fn_state.u_25737;
        // D s_51_24: cast zx s_51_23 -> bv
        let s_51_24: Bits = Bits::new(s_51_23 as u128, 32u16);
        // D s_51_25: bit-extract s_51_24 s_51_21 s_51_22
        let s_51_25: Bits = (Bits::new(
            ((s_51_24) >> (s_51_21)).value(),
            u16::try_from(s_51_22).unwrap(),
        ));
        // D s_51_26: cast reint s_51_25 -> u8
        let s_51_26: u8 = (s_51_25.value() as u8);
        // C s_51_27: const #22s : i
        let s_51_27: i128 = 22;
        // C s_51_28: const #2s : i
        let s_51_28: i128 = 2;
        // D s_51_29: read-var u#25737:u32
        let s_51_29: u32 = fn_state.u_25737;
        // D s_51_30: cast zx s_51_29 -> bv
        let s_51_30: Bits = Bits::new(s_51_29 as u128, 32u16);
        // D s_51_31: bit-extract s_51_30 s_51_27 s_51_28
        let s_51_31: Bits = (Bits::new(
            ((s_51_30) >> (s_51_27)).value(),
            u16::try_from(s_51_28).unwrap(),
        ));
        // D s_51_32: cast reint s_51_31 -> u8
        let s_51_32: u8 = (s_51_31.value() as u8);
        // C s_51_33: const #29s : i
        let s_51_33: i128 = 29;
        // C s_51_34: const #1s : i
        let s_51_34: i128 = 1;
        // D s_51_35: read-var u#25737:u32
        let s_51_35: u32 = fn_state.u_25737;
        // D s_51_36: cast zx s_51_35 -> bv
        let s_51_36: Bits = Bits::new(s_51_35 as u128, 32u16);
        // D s_51_37: bit-extract s_51_36 s_51_33 s_51_34
        let s_51_37: Bits = (Bits::new(
            ((s_51_36) >> (s_51_33)).value(),
            u16::try_from(s_51_34).unwrap(),
        ));
        // D s_51_38: cast reint s_51_37 -> u8
        let s_51_38: bool = ((s_51_37.value()) != 0);
        // C s_51_39: const #30s : i
        let s_51_39: i128 = 30;
        // C s_51_40: const #1s : i
        let s_51_40: i128 = 1;
        // D s_51_41: read-var u#25737:u32
        let s_51_41: u32 = fn_state.u_25737;
        // D s_51_42: cast zx s_51_41 -> bv
        let s_51_42: Bits = Bits::new(s_51_41 as u128, 32u16);
        // D s_51_43: bit-extract s_51_42 s_51_39 s_51_40
        let s_51_43: Bits = (Bits::new(
            ((s_51_42) >> (s_51_39)).value(),
            u16::try_from(s_51_40).unwrap(),
        ));
        // D s_51_44: cast reint s_51_43 -> u8
        let s_51_44: bool = ((s_51_43.value()) != 0);
        // C s_51_45: const #31s : i
        let s_51_45: i128 = 31;
        // C s_51_46: const #1s : i
        let s_51_46: i128 = 1;
        // D s_51_47: read-var u#25737:u32
        let s_51_47: u32 = fn_state.u_25737;
        // D s_51_48: cast zx s_51_47 -> bv
        let s_51_48: Bits = Bits::new(s_51_47 as u128, 32u16);
        // D s_51_49: bit-extract s_51_48 s_51_45 s_51_46
        let s_51_49: Bits = (Bits::new(
            ((s_51_48) >> (s_51_45)).value(),
            u16::try_from(s_51_46).unwrap(),
        ));
        // D s_51_50: cast reint s_51_49 -> u8
        let s_51_50: bool = ((s_51_49.value()) != 0);
        // D s_51_51: call decode_subs_addsub_shift_aarch64_instrs_integer_arithmetic_add_sub_shiftedreg(s_51_8, s_51_14, s_51_20, s_51_26, s_51_32, s_51_38, s_51_44, s_51_50)
        let s_51_51: () = decode_subs_addsub_shift_aarch64_instrs_integer_arithmetic_add_sub_shiftedreg(
            state,
            tracer,
            s_51_8,
            s_51_14,
            s_51_20,
            s_51_26,
            s_51_32,
            s_51_38,
            s_51_44,
            s_51_50,
        );
        // N s_51_52: return
        return;
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var merge#var.1:struct
        let s_52_0: u32 = fn_state.merge_var._1;
        // D s_52_1: write-var u#25747 <= s_52_0
        fn_state.u_25747 = s_52_0;
        // C s_52_2: const #21s : i
        let s_52_2: i128 = 21;
        // D s_52_3: read-var u#25747:u32
        let s_52_3: u32 = fn_state.u_25747;
        // D s_52_4: cast zx s_52_3 -> bv
        let s_52_4: Bits = Bits::new(s_52_3 as u128, 32u16);
        // C s_52_5: const #1s : i64
        let s_52_5: i64 = 1;
        // C s_52_6: cast zx s_52_5 -> i
        let s_52_6: i128 = (i128::try_from(s_52_5).unwrap());
        // C s_52_7: const #9s : i
        let s_52_7: i128 = 9;
        // C s_52_8: add s_52_7 s_52_6
        let s_52_8: i128 = (s_52_7 + s_52_6);
        // D s_52_9: bit-extract s_52_4 s_52_2 s_52_8
        let s_52_9: Bits = (Bits::new(
            ((s_52_4) >> (s_52_2)).value(),
            u16::try_from(s_52_8).unwrap(),
        ));
        // D s_52_10: cast reint s_52_9 -> u10
        let s_52_10: u16 = (s_52_9.value() as u16);
        // D s_52_11: cast zx s_52_10 -> bv
        let s_52_11: Bits = Bits::new(s_52_10 as u128, 10u16);
        // C s_52_12: const #89u : u10
        let s_52_12: u16 = 89;
        // C s_52_13: cast zx s_52_12 -> bv
        let s_52_13: Bits = Bits::new(s_52_12 as u128, 10u16);
        // D s_52_14: cmp-eq s_52_11 s_52_13
        let s_52_14: bool = ((s_52_11) == (s_52_13));
        // N s_52_15: branch s_52_14 b574 b53
        if s_52_14 {
            return block_574(state, tracer, fn_state);
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
        // D s_53_1: write-var gs#382330 <= s_53_0
        fn_state.gs_382330 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var gs#382330:u8
        let s_54_0: bool = fn_state.gs_382330;
        // D s_54_1: not s_54_0
        let s_54_1: bool = !s_54_0;
        // N s_54_2: branch s_54_1 b56 b55
        if s_54_1 {
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
        // C s_55_0: const #22s : i
        let s_55_0: i128 = 22;
        // C s_55_1: const #14696u : u32
        let s_55_1: u32 = 14696;
        // N s_55_2: write-reg s_55_1 <= s_55_0
        let s_55_2: () = {
            state.write_register::<i128>(s_55_1 as isize, s_55_0);
            tracer.write_register(s_55_1 as isize, s_55_0);
        };
        // C s_55_3: const #0s : i
        let s_55_3: i128 = 0;
        // C s_55_4: const #5s : i
        let s_55_4: i128 = 5;
        // D s_55_5: read-var u#25747:u32
        let s_55_5: u32 = fn_state.u_25747;
        // D s_55_6: cast zx s_55_5 -> bv
        let s_55_6: Bits = Bits::new(s_55_5 as u128, 32u16);
        // D s_55_7: bit-extract s_55_6 s_55_3 s_55_4
        let s_55_7: Bits = (Bits::new(
            ((s_55_6) >> (s_55_3)).value(),
            u16::try_from(s_55_4).unwrap(),
        ));
        // D s_55_8: cast reint s_55_7 -> u8
        let s_55_8: u8 = (s_55_7.value() as u8);
        // C s_55_9: const #5s : i
        let s_55_9: i128 = 5;
        // C s_55_10: const #5s : i
        let s_55_10: i128 = 5;
        // D s_55_11: read-var u#25747:u32
        let s_55_11: u32 = fn_state.u_25747;
        // D s_55_12: cast zx s_55_11 -> bv
        let s_55_12: Bits = Bits::new(s_55_11 as u128, 32u16);
        // D s_55_13: bit-extract s_55_12 s_55_9 s_55_10
        let s_55_13: Bits = (Bits::new(
            ((s_55_12) >> (s_55_9)).value(),
            u16::try_from(s_55_10).unwrap(),
        ));
        // D s_55_14: cast reint s_55_13 -> u8
        let s_55_14: u8 = (s_55_13.value() as u8);
        // C s_55_15: const #10s : i
        let s_55_15: i128 = 10;
        // C s_55_16: const #3s : i
        let s_55_16: i128 = 3;
        // D s_55_17: read-var u#25747:u32
        let s_55_17: u32 = fn_state.u_25747;
        // D s_55_18: cast zx s_55_17 -> bv
        let s_55_18: Bits = Bits::new(s_55_17 as u128, 32u16);
        // D s_55_19: bit-extract s_55_18 s_55_15 s_55_16
        let s_55_19: Bits = (Bits::new(
            ((s_55_18) >> (s_55_15)).value(),
            u16::try_from(s_55_16).unwrap(),
        ));
        // D s_55_20: cast reint s_55_19 -> u8
        let s_55_20: u8 = (s_55_19.value() as u8);
        // C s_55_21: const #13s : i
        let s_55_21: i128 = 13;
        // C s_55_22: const #3s : i
        let s_55_22: i128 = 3;
        // D s_55_23: read-var u#25747:u32
        let s_55_23: u32 = fn_state.u_25747;
        // D s_55_24: cast zx s_55_23 -> bv
        let s_55_24: Bits = Bits::new(s_55_23 as u128, 32u16);
        // D s_55_25: bit-extract s_55_24 s_55_21 s_55_22
        let s_55_25: Bits = (Bits::new(
            ((s_55_24) >> (s_55_21)).value(),
            u16::try_from(s_55_22).unwrap(),
        ));
        // D s_55_26: cast reint s_55_25 -> u8
        let s_55_26: u8 = (s_55_25.value() as u8);
        // C s_55_27: const #16s : i
        let s_55_27: i128 = 16;
        // C s_55_28: const #5s : i
        let s_55_28: i128 = 5;
        // D s_55_29: read-var u#25747:u32
        let s_55_29: u32 = fn_state.u_25747;
        // D s_55_30: cast zx s_55_29 -> bv
        let s_55_30: Bits = Bits::new(s_55_29 as u128, 32u16);
        // D s_55_31: bit-extract s_55_30 s_55_27 s_55_28
        let s_55_31: Bits = (Bits::new(
            ((s_55_30) >> (s_55_27)).value(),
            u16::try_from(s_55_28).unwrap(),
        ));
        // D s_55_32: cast reint s_55_31 -> u8
        let s_55_32: u8 = (s_55_31.value() as u8);
        // C s_55_33: const #29s : i
        let s_55_33: i128 = 29;
        // C s_55_34: const #1s : i
        let s_55_34: i128 = 1;
        // D s_55_35: read-var u#25747:u32
        let s_55_35: u32 = fn_state.u_25747;
        // D s_55_36: cast zx s_55_35 -> bv
        let s_55_36: Bits = Bits::new(s_55_35 as u128, 32u16);
        // D s_55_37: bit-extract s_55_36 s_55_33 s_55_34
        let s_55_37: Bits = (Bits::new(
            ((s_55_36) >> (s_55_33)).value(),
            u16::try_from(s_55_34).unwrap(),
        ));
        // D s_55_38: cast reint s_55_37 -> u8
        let s_55_38: bool = ((s_55_37.value()) != 0);
        // C s_55_39: const #30s : i
        let s_55_39: i128 = 30;
        // C s_55_40: const #1s : i
        let s_55_40: i128 = 1;
        // D s_55_41: read-var u#25747:u32
        let s_55_41: u32 = fn_state.u_25747;
        // D s_55_42: cast zx s_55_41 -> bv
        let s_55_42: Bits = Bits::new(s_55_41 as u128, 32u16);
        // D s_55_43: bit-extract s_55_42 s_55_39 s_55_40
        let s_55_43: Bits = (Bits::new(
            ((s_55_42) >> (s_55_39)).value(),
            u16::try_from(s_55_40).unwrap(),
        ));
        // D s_55_44: cast reint s_55_43 -> u8
        let s_55_44: bool = ((s_55_43.value()) != 0);
        // C s_55_45: const #31s : i
        let s_55_45: i128 = 31;
        // C s_55_46: const #1s : i
        let s_55_46: i128 = 1;
        // D s_55_47: read-var u#25747:u32
        let s_55_47: u32 = fn_state.u_25747;
        // D s_55_48: cast zx s_55_47 -> bv
        let s_55_48: Bits = Bits::new(s_55_47 as u128, 32u16);
        // D s_55_49: bit-extract s_55_48 s_55_45 s_55_46
        let s_55_49: Bits = (Bits::new(
            ((s_55_48) >> (s_55_45)).value(),
            u16::try_from(s_55_46).unwrap(),
        ));
        // D s_55_50: cast reint s_55_49 -> u8
        let s_55_50: bool = ((s_55_49.value()) != 0);
        // D s_55_51: call decode_add_addsub_ext_aarch64_instrs_integer_arithmetic_add_sub_extendedreg(s_55_8, s_55_14, s_55_20, s_55_26, s_55_32, s_55_38, s_55_44, s_55_50)
        let s_55_51: () = decode_add_addsub_ext_aarch64_instrs_integer_arithmetic_add_sub_extendedreg(
            state,
            tracer,
            s_55_8,
            s_55_14,
            s_55_20,
            s_55_26,
            s_55_32,
            s_55_38,
            s_55_44,
            s_55_50,
        );
        // N s_55_52: return
        return;
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var merge#var.1:struct
        let s_56_0: u32 = fn_state.merge_var._1;
        // D s_56_1: write-var u#25755 <= s_56_0
        fn_state.u_25755 = s_56_0;
        // C s_56_2: const #21s : i
        let s_56_2: i128 = 21;
        // D s_56_3: read-var u#25755:u32
        let s_56_3: u32 = fn_state.u_25755;
        // D s_56_4: cast zx s_56_3 -> bv
        let s_56_4: Bits = Bits::new(s_56_3 as u128, 32u16);
        // C s_56_5: const #1s : i64
        let s_56_5: i64 = 1;
        // C s_56_6: cast zx s_56_5 -> i
        let s_56_6: i128 = (i128::try_from(s_56_5).unwrap());
        // C s_56_7: const #9s : i
        let s_56_7: i128 = 9;
        // C s_56_8: add s_56_7 s_56_6
        let s_56_8: i128 = (s_56_7 + s_56_6);
        // D s_56_9: bit-extract s_56_4 s_56_2 s_56_8
        let s_56_9: Bits = (Bits::new(
            ((s_56_4) >> (s_56_2)).value(),
            u16::try_from(s_56_8).unwrap(),
        ));
        // D s_56_10: cast reint s_56_9 -> u10
        let s_56_10: u16 = (s_56_9.value() as u16);
        // D s_56_11: cast zx s_56_10 -> bv
        let s_56_11: Bits = Bits::new(s_56_10 as u128, 10u16);
        // C s_56_12: const #345u : u10
        let s_56_12: u16 = 345;
        // C s_56_13: cast zx s_56_12 -> bv
        let s_56_13: Bits = Bits::new(s_56_12 as u128, 10u16);
        // D s_56_14: cmp-eq s_56_11 s_56_13
        let s_56_14: bool = ((s_56_11) == (s_56_13));
        // N s_56_15: branch s_56_14 b573 b57
        if s_56_14 {
            return block_573(state, tracer, fn_state);
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
        // D s_57_1: write-var gs#382353 <= s_57_0
        fn_state.gs_382353 = s_57_0;
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var gs#382353:u8
        let s_58_0: bool = fn_state.gs_382353;
        // D s_58_1: not s_58_0
        let s_58_1: bool = !s_58_0;
        // N s_58_2: branch s_58_1 b60 b59
        if s_58_1 {
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
        // C s_59_0: const #23s : i
        let s_59_0: i128 = 23;
        // C s_59_1: const #14696u : u32
        let s_59_1: u32 = 14696;
        // N s_59_2: write-reg s_59_1 <= s_59_0
        let s_59_2: () = {
            state.write_register::<i128>(s_59_1 as isize, s_59_0);
            tracer.write_register(s_59_1 as isize, s_59_0);
        };
        // C s_59_3: const #0s : i
        let s_59_3: i128 = 0;
        // C s_59_4: const #5s : i
        let s_59_4: i128 = 5;
        // D s_59_5: read-var u#25755:u32
        let s_59_5: u32 = fn_state.u_25755;
        // D s_59_6: cast zx s_59_5 -> bv
        let s_59_6: Bits = Bits::new(s_59_5 as u128, 32u16);
        // D s_59_7: bit-extract s_59_6 s_59_3 s_59_4
        let s_59_7: Bits = (Bits::new(
            ((s_59_6) >> (s_59_3)).value(),
            u16::try_from(s_59_4).unwrap(),
        ));
        // D s_59_8: cast reint s_59_7 -> u8
        let s_59_8: u8 = (s_59_7.value() as u8);
        // C s_59_9: const #5s : i
        let s_59_9: i128 = 5;
        // C s_59_10: const #5s : i
        let s_59_10: i128 = 5;
        // D s_59_11: read-var u#25755:u32
        let s_59_11: u32 = fn_state.u_25755;
        // D s_59_12: cast zx s_59_11 -> bv
        let s_59_12: Bits = Bits::new(s_59_11 as u128, 32u16);
        // D s_59_13: bit-extract s_59_12 s_59_9 s_59_10
        let s_59_13: Bits = (Bits::new(
            ((s_59_12) >> (s_59_9)).value(),
            u16::try_from(s_59_10).unwrap(),
        ));
        // D s_59_14: cast reint s_59_13 -> u8
        let s_59_14: u8 = (s_59_13.value() as u8);
        // C s_59_15: const #10s : i
        let s_59_15: i128 = 10;
        // C s_59_16: const #3s : i
        let s_59_16: i128 = 3;
        // D s_59_17: read-var u#25755:u32
        let s_59_17: u32 = fn_state.u_25755;
        // D s_59_18: cast zx s_59_17 -> bv
        let s_59_18: Bits = Bits::new(s_59_17 as u128, 32u16);
        // D s_59_19: bit-extract s_59_18 s_59_15 s_59_16
        let s_59_19: Bits = (Bits::new(
            ((s_59_18) >> (s_59_15)).value(),
            u16::try_from(s_59_16).unwrap(),
        ));
        // D s_59_20: cast reint s_59_19 -> u8
        let s_59_20: u8 = (s_59_19.value() as u8);
        // C s_59_21: const #13s : i
        let s_59_21: i128 = 13;
        // C s_59_22: const #3s : i
        let s_59_22: i128 = 3;
        // D s_59_23: read-var u#25755:u32
        let s_59_23: u32 = fn_state.u_25755;
        // D s_59_24: cast zx s_59_23 -> bv
        let s_59_24: Bits = Bits::new(s_59_23 as u128, 32u16);
        // D s_59_25: bit-extract s_59_24 s_59_21 s_59_22
        let s_59_25: Bits = (Bits::new(
            ((s_59_24) >> (s_59_21)).value(),
            u16::try_from(s_59_22).unwrap(),
        ));
        // D s_59_26: cast reint s_59_25 -> u8
        let s_59_26: u8 = (s_59_25.value() as u8);
        // C s_59_27: const #16s : i
        let s_59_27: i128 = 16;
        // C s_59_28: const #5s : i
        let s_59_28: i128 = 5;
        // D s_59_29: read-var u#25755:u32
        let s_59_29: u32 = fn_state.u_25755;
        // D s_59_30: cast zx s_59_29 -> bv
        let s_59_30: Bits = Bits::new(s_59_29 as u128, 32u16);
        // D s_59_31: bit-extract s_59_30 s_59_27 s_59_28
        let s_59_31: Bits = (Bits::new(
            ((s_59_30) >> (s_59_27)).value(),
            u16::try_from(s_59_28).unwrap(),
        ));
        // D s_59_32: cast reint s_59_31 -> u8
        let s_59_32: u8 = (s_59_31.value() as u8);
        // C s_59_33: const #29s : i
        let s_59_33: i128 = 29;
        // C s_59_34: const #1s : i
        let s_59_34: i128 = 1;
        // D s_59_35: read-var u#25755:u32
        let s_59_35: u32 = fn_state.u_25755;
        // D s_59_36: cast zx s_59_35 -> bv
        let s_59_36: Bits = Bits::new(s_59_35 as u128, 32u16);
        // D s_59_37: bit-extract s_59_36 s_59_33 s_59_34
        let s_59_37: Bits = (Bits::new(
            ((s_59_36) >> (s_59_33)).value(),
            u16::try_from(s_59_34).unwrap(),
        ));
        // D s_59_38: cast reint s_59_37 -> u8
        let s_59_38: bool = ((s_59_37.value()) != 0);
        // C s_59_39: const #30s : i
        let s_59_39: i128 = 30;
        // C s_59_40: const #1s : i
        let s_59_40: i128 = 1;
        // D s_59_41: read-var u#25755:u32
        let s_59_41: u32 = fn_state.u_25755;
        // D s_59_42: cast zx s_59_41 -> bv
        let s_59_42: Bits = Bits::new(s_59_41 as u128, 32u16);
        // D s_59_43: bit-extract s_59_42 s_59_39 s_59_40
        let s_59_43: Bits = (Bits::new(
            ((s_59_42) >> (s_59_39)).value(),
            u16::try_from(s_59_40).unwrap(),
        ));
        // D s_59_44: cast reint s_59_43 -> u8
        let s_59_44: bool = ((s_59_43.value()) != 0);
        // C s_59_45: const #31s : i
        let s_59_45: i128 = 31;
        // C s_59_46: const #1s : i
        let s_59_46: i128 = 1;
        // D s_59_47: read-var u#25755:u32
        let s_59_47: u32 = fn_state.u_25755;
        // D s_59_48: cast zx s_59_47 -> bv
        let s_59_48: Bits = Bits::new(s_59_47 as u128, 32u16);
        // D s_59_49: bit-extract s_59_48 s_59_45 s_59_46
        let s_59_49: Bits = (Bits::new(
            ((s_59_48) >> (s_59_45)).value(),
            u16::try_from(s_59_46).unwrap(),
        ));
        // D s_59_50: cast reint s_59_49 -> u8
        let s_59_50: bool = ((s_59_49.value()) != 0);
        // D s_59_51: call decode_adds_addsub_ext_aarch64_instrs_integer_arithmetic_add_sub_extendedreg(s_59_8, s_59_14, s_59_20, s_59_26, s_59_32, s_59_38, s_59_44, s_59_50)
        let s_59_51: () = decode_adds_addsub_ext_aarch64_instrs_integer_arithmetic_add_sub_extendedreg(
            state,
            tracer,
            s_59_8,
            s_59_14,
            s_59_20,
            s_59_26,
            s_59_32,
            s_59_38,
            s_59_44,
            s_59_50,
        );
        // N s_59_52: return
        return;
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var merge#var.1:struct
        let s_60_0: u32 = fn_state.merge_var._1;
        // D s_60_1: write-var u#25765 <= s_60_0
        fn_state.u_25765 = s_60_0;
        // C s_60_2: const #21s : i
        let s_60_2: i128 = 21;
        // D s_60_3: read-var u#25765:u32
        let s_60_3: u32 = fn_state.u_25765;
        // D s_60_4: cast zx s_60_3 -> bv
        let s_60_4: Bits = Bits::new(s_60_3 as u128, 32u16);
        // C s_60_5: const #1s : i64
        let s_60_5: i64 = 1;
        // C s_60_6: cast zx s_60_5 -> i
        let s_60_6: i128 = (i128::try_from(s_60_5).unwrap());
        // C s_60_7: const #9s : i
        let s_60_7: i128 = 9;
        // C s_60_8: add s_60_7 s_60_6
        let s_60_8: i128 = (s_60_7 + s_60_6);
        // D s_60_9: bit-extract s_60_4 s_60_2 s_60_8
        let s_60_9: Bits = (Bits::new(
            ((s_60_4) >> (s_60_2)).value(),
            u16::try_from(s_60_8).unwrap(),
        ));
        // D s_60_10: cast reint s_60_9 -> u10
        let s_60_10: u16 = (s_60_9.value() as u16);
        // D s_60_11: cast zx s_60_10 -> bv
        let s_60_11: Bits = Bits::new(s_60_10 as u128, 10u16);
        // C s_60_12: const #601u : u10
        let s_60_12: u16 = 601;
        // C s_60_13: cast zx s_60_12 -> bv
        let s_60_13: Bits = Bits::new(s_60_12 as u128, 10u16);
        // D s_60_14: cmp-eq s_60_11 s_60_13
        let s_60_14: bool = ((s_60_11) == (s_60_13));
        // N s_60_15: branch s_60_14 b572 b61
        if s_60_14 {
            return block_572(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #0u : u8
        let s_61_0: bool = false;
        // D s_61_1: write-var gs#382376 <= s_61_0
        fn_state.gs_382376 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#382376:u8
        let s_62_0: bool = fn_state.gs_382376;
        // D s_62_1: not s_62_0
        let s_62_1: bool = !s_62_0;
        // N s_62_2: branch s_62_1 b64 b63
        if s_62_1 {
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
        // C s_63_0: const #24s : i
        let s_63_0: i128 = 24;
        // C s_63_1: const #14696u : u32
        let s_63_1: u32 = 14696;
        // N s_63_2: write-reg s_63_1 <= s_63_0
        let s_63_2: () = {
            state.write_register::<i128>(s_63_1 as isize, s_63_0);
            tracer.write_register(s_63_1 as isize, s_63_0);
        };
        // C s_63_3: const #0s : i
        let s_63_3: i128 = 0;
        // C s_63_4: const #5s : i
        let s_63_4: i128 = 5;
        // D s_63_5: read-var u#25765:u32
        let s_63_5: u32 = fn_state.u_25765;
        // D s_63_6: cast zx s_63_5 -> bv
        let s_63_6: Bits = Bits::new(s_63_5 as u128, 32u16);
        // D s_63_7: bit-extract s_63_6 s_63_3 s_63_4
        let s_63_7: Bits = (Bits::new(
            ((s_63_6) >> (s_63_3)).value(),
            u16::try_from(s_63_4).unwrap(),
        ));
        // D s_63_8: cast reint s_63_7 -> u8
        let s_63_8: u8 = (s_63_7.value() as u8);
        // C s_63_9: const #5s : i
        let s_63_9: i128 = 5;
        // C s_63_10: const #5s : i
        let s_63_10: i128 = 5;
        // D s_63_11: read-var u#25765:u32
        let s_63_11: u32 = fn_state.u_25765;
        // D s_63_12: cast zx s_63_11 -> bv
        let s_63_12: Bits = Bits::new(s_63_11 as u128, 32u16);
        // D s_63_13: bit-extract s_63_12 s_63_9 s_63_10
        let s_63_13: Bits = (Bits::new(
            ((s_63_12) >> (s_63_9)).value(),
            u16::try_from(s_63_10).unwrap(),
        ));
        // D s_63_14: cast reint s_63_13 -> u8
        let s_63_14: u8 = (s_63_13.value() as u8);
        // C s_63_15: const #10s : i
        let s_63_15: i128 = 10;
        // C s_63_16: const #3s : i
        let s_63_16: i128 = 3;
        // D s_63_17: read-var u#25765:u32
        let s_63_17: u32 = fn_state.u_25765;
        // D s_63_18: cast zx s_63_17 -> bv
        let s_63_18: Bits = Bits::new(s_63_17 as u128, 32u16);
        // D s_63_19: bit-extract s_63_18 s_63_15 s_63_16
        let s_63_19: Bits = (Bits::new(
            ((s_63_18) >> (s_63_15)).value(),
            u16::try_from(s_63_16).unwrap(),
        ));
        // D s_63_20: cast reint s_63_19 -> u8
        let s_63_20: u8 = (s_63_19.value() as u8);
        // C s_63_21: const #13s : i
        let s_63_21: i128 = 13;
        // C s_63_22: const #3s : i
        let s_63_22: i128 = 3;
        // D s_63_23: read-var u#25765:u32
        let s_63_23: u32 = fn_state.u_25765;
        // D s_63_24: cast zx s_63_23 -> bv
        let s_63_24: Bits = Bits::new(s_63_23 as u128, 32u16);
        // D s_63_25: bit-extract s_63_24 s_63_21 s_63_22
        let s_63_25: Bits = (Bits::new(
            ((s_63_24) >> (s_63_21)).value(),
            u16::try_from(s_63_22).unwrap(),
        ));
        // D s_63_26: cast reint s_63_25 -> u8
        let s_63_26: u8 = (s_63_25.value() as u8);
        // C s_63_27: const #16s : i
        let s_63_27: i128 = 16;
        // C s_63_28: const #5s : i
        let s_63_28: i128 = 5;
        // D s_63_29: read-var u#25765:u32
        let s_63_29: u32 = fn_state.u_25765;
        // D s_63_30: cast zx s_63_29 -> bv
        let s_63_30: Bits = Bits::new(s_63_29 as u128, 32u16);
        // D s_63_31: bit-extract s_63_30 s_63_27 s_63_28
        let s_63_31: Bits = (Bits::new(
            ((s_63_30) >> (s_63_27)).value(),
            u16::try_from(s_63_28).unwrap(),
        ));
        // D s_63_32: cast reint s_63_31 -> u8
        let s_63_32: u8 = (s_63_31.value() as u8);
        // C s_63_33: const #29s : i
        let s_63_33: i128 = 29;
        // C s_63_34: const #1s : i
        let s_63_34: i128 = 1;
        // D s_63_35: read-var u#25765:u32
        let s_63_35: u32 = fn_state.u_25765;
        // D s_63_36: cast zx s_63_35 -> bv
        let s_63_36: Bits = Bits::new(s_63_35 as u128, 32u16);
        // D s_63_37: bit-extract s_63_36 s_63_33 s_63_34
        let s_63_37: Bits = (Bits::new(
            ((s_63_36) >> (s_63_33)).value(),
            u16::try_from(s_63_34).unwrap(),
        ));
        // D s_63_38: cast reint s_63_37 -> u8
        let s_63_38: bool = ((s_63_37.value()) != 0);
        // C s_63_39: const #30s : i
        let s_63_39: i128 = 30;
        // C s_63_40: const #1s : i
        let s_63_40: i128 = 1;
        // D s_63_41: read-var u#25765:u32
        let s_63_41: u32 = fn_state.u_25765;
        // D s_63_42: cast zx s_63_41 -> bv
        let s_63_42: Bits = Bits::new(s_63_41 as u128, 32u16);
        // D s_63_43: bit-extract s_63_42 s_63_39 s_63_40
        let s_63_43: Bits = (Bits::new(
            ((s_63_42) >> (s_63_39)).value(),
            u16::try_from(s_63_40).unwrap(),
        ));
        // D s_63_44: cast reint s_63_43 -> u8
        let s_63_44: bool = ((s_63_43.value()) != 0);
        // C s_63_45: const #31s : i
        let s_63_45: i128 = 31;
        // C s_63_46: const #1s : i
        let s_63_46: i128 = 1;
        // D s_63_47: read-var u#25765:u32
        let s_63_47: u32 = fn_state.u_25765;
        // D s_63_48: cast zx s_63_47 -> bv
        let s_63_48: Bits = Bits::new(s_63_47 as u128, 32u16);
        // D s_63_49: bit-extract s_63_48 s_63_45 s_63_46
        let s_63_49: Bits = (Bits::new(
            ((s_63_48) >> (s_63_45)).value(),
            u16::try_from(s_63_46).unwrap(),
        ));
        // D s_63_50: cast reint s_63_49 -> u8
        let s_63_50: bool = ((s_63_49.value()) != 0);
        // D s_63_51: call decode_sub_addsub_ext_aarch64_instrs_integer_arithmetic_add_sub_extendedreg(s_63_8, s_63_14, s_63_20, s_63_26, s_63_32, s_63_38, s_63_44, s_63_50)
        let s_63_51: () = decode_sub_addsub_ext_aarch64_instrs_integer_arithmetic_add_sub_extendedreg(
            state,
            tracer,
            s_63_8,
            s_63_14,
            s_63_20,
            s_63_26,
            s_63_32,
            s_63_38,
            s_63_44,
            s_63_50,
        );
        // N s_63_52: return
        return;
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var merge#var.1:struct
        let s_64_0: u32 = fn_state.merge_var._1;
        // D s_64_1: write-var u#25775 <= s_64_0
        fn_state.u_25775 = s_64_0;
        // C s_64_2: const #21s : i
        let s_64_2: i128 = 21;
        // D s_64_3: read-var u#25775:u32
        let s_64_3: u32 = fn_state.u_25775;
        // D s_64_4: cast zx s_64_3 -> bv
        let s_64_4: Bits = Bits::new(s_64_3 as u128, 32u16);
        // C s_64_5: const #1s : i64
        let s_64_5: i64 = 1;
        // C s_64_6: cast zx s_64_5 -> i
        let s_64_6: i128 = (i128::try_from(s_64_5).unwrap());
        // C s_64_7: const #9s : i
        let s_64_7: i128 = 9;
        // C s_64_8: add s_64_7 s_64_6
        let s_64_8: i128 = (s_64_7 + s_64_6);
        // D s_64_9: bit-extract s_64_4 s_64_2 s_64_8
        let s_64_9: Bits = (Bits::new(
            ((s_64_4) >> (s_64_2)).value(),
            u16::try_from(s_64_8).unwrap(),
        ));
        // D s_64_10: cast reint s_64_9 -> u10
        let s_64_10: u16 = (s_64_9.value() as u16);
        // D s_64_11: cast zx s_64_10 -> bv
        let s_64_11: Bits = Bits::new(s_64_10 as u128, 10u16);
        // C s_64_12: const #857u : u10
        let s_64_12: u16 = 857;
        // C s_64_13: cast zx s_64_12 -> bv
        let s_64_13: Bits = Bits::new(s_64_12 as u128, 10u16);
        // D s_64_14: cmp-eq s_64_11 s_64_13
        let s_64_14: bool = ((s_64_11) == (s_64_13));
        // N s_64_15: branch s_64_14 b571 b65
        if s_64_14 {
            return block_571(state, tracer, fn_state);
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
        // D s_65_1: write-var gs#382399 <= s_65_0
        fn_state.gs_382399 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var gs#382399:u8
        let s_66_0: bool = fn_state.gs_382399;
        // D s_66_1: not s_66_0
        let s_66_1: bool = !s_66_0;
        // N s_66_2: branch s_66_1 b68 b67
        if s_66_1 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #25s : i
        let s_67_0: i128 = 25;
        // C s_67_1: const #14696u : u32
        let s_67_1: u32 = 14696;
        // N s_67_2: write-reg s_67_1 <= s_67_0
        let s_67_2: () = {
            state.write_register::<i128>(s_67_1 as isize, s_67_0);
            tracer.write_register(s_67_1 as isize, s_67_0);
        };
        // C s_67_3: const #0s : i
        let s_67_3: i128 = 0;
        // C s_67_4: const #5s : i
        let s_67_4: i128 = 5;
        // D s_67_5: read-var u#25775:u32
        let s_67_5: u32 = fn_state.u_25775;
        // D s_67_6: cast zx s_67_5 -> bv
        let s_67_6: Bits = Bits::new(s_67_5 as u128, 32u16);
        // D s_67_7: bit-extract s_67_6 s_67_3 s_67_4
        let s_67_7: Bits = (Bits::new(
            ((s_67_6) >> (s_67_3)).value(),
            u16::try_from(s_67_4).unwrap(),
        ));
        // D s_67_8: cast reint s_67_7 -> u8
        let s_67_8: u8 = (s_67_7.value() as u8);
        // C s_67_9: const #5s : i
        let s_67_9: i128 = 5;
        // C s_67_10: const #5s : i
        let s_67_10: i128 = 5;
        // D s_67_11: read-var u#25775:u32
        let s_67_11: u32 = fn_state.u_25775;
        // D s_67_12: cast zx s_67_11 -> bv
        let s_67_12: Bits = Bits::new(s_67_11 as u128, 32u16);
        // D s_67_13: bit-extract s_67_12 s_67_9 s_67_10
        let s_67_13: Bits = (Bits::new(
            ((s_67_12) >> (s_67_9)).value(),
            u16::try_from(s_67_10).unwrap(),
        ));
        // D s_67_14: cast reint s_67_13 -> u8
        let s_67_14: u8 = (s_67_13.value() as u8);
        // C s_67_15: const #10s : i
        let s_67_15: i128 = 10;
        // C s_67_16: const #3s : i
        let s_67_16: i128 = 3;
        // D s_67_17: read-var u#25775:u32
        let s_67_17: u32 = fn_state.u_25775;
        // D s_67_18: cast zx s_67_17 -> bv
        let s_67_18: Bits = Bits::new(s_67_17 as u128, 32u16);
        // D s_67_19: bit-extract s_67_18 s_67_15 s_67_16
        let s_67_19: Bits = (Bits::new(
            ((s_67_18) >> (s_67_15)).value(),
            u16::try_from(s_67_16).unwrap(),
        ));
        // D s_67_20: cast reint s_67_19 -> u8
        let s_67_20: u8 = (s_67_19.value() as u8);
        // C s_67_21: const #13s : i
        let s_67_21: i128 = 13;
        // C s_67_22: const #3s : i
        let s_67_22: i128 = 3;
        // D s_67_23: read-var u#25775:u32
        let s_67_23: u32 = fn_state.u_25775;
        // D s_67_24: cast zx s_67_23 -> bv
        let s_67_24: Bits = Bits::new(s_67_23 as u128, 32u16);
        // D s_67_25: bit-extract s_67_24 s_67_21 s_67_22
        let s_67_25: Bits = (Bits::new(
            ((s_67_24) >> (s_67_21)).value(),
            u16::try_from(s_67_22).unwrap(),
        ));
        // D s_67_26: cast reint s_67_25 -> u8
        let s_67_26: u8 = (s_67_25.value() as u8);
        // C s_67_27: const #16s : i
        let s_67_27: i128 = 16;
        // C s_67_28: const #5s : i
        let s_67_28: i128 = 5;
        // D s_67_29: read-var u#25775:u32
        let s_67_29: u32 = fn_state.u_25775;
        // D s_67_30: cast zx s_67_29 -> bv
        let s_67_30: Bits = Bits::new(s_67_29 as u128, 32u16);
        // D s_67_31: bit-extract s_67_30 s_67_27 s_67_28
        let s_67_31: Bits = (Bits::new(
            ((s_67_30) >> (s_67_27)).value(),
            u16::try_from(s_67_28).unwrap(),
        ));
        // D s_67_32: cast reint s_67_31 -> u8
        let s_67_32: u8 = (s_67_31.value() as u8);
        // C s_67_33: const #29s : i
        let s_67_33: i128 = 29;
        // C s_67_34: const #1s : i
        let s_67_34: i128 = 1;
        // D s_67_35: read-var u#25775:u32
        let s_67_35: u32 = fn_state.u_25775;
        // D s_67_36: cast zx s_67_35 -> bv
        let s_67_36: Bits = Bits::new(s_67_35 as u128, 32u16);
        // D s_67_37: bit-extract s_67_36 s_67_33 s_67_34
        let s_67_37: Bits = (Bits::new(
            ((s_67_36) >> (s_67_33)).value(),
            u16::try_from(s_67_34).unwrap(),
        ));
        // D s_67_38: cast reint s_67_37 -> u8
        let s_67_38: bool = ((s_67_37.value()) != 0);
        // C s_67_39: const #30s : i
        let s_67_39: i128 = 30;
        // C s_67_40: const #1s : i
        let s_67_40: i128 = 1;
        // D s_67_41: read-var u#25775:u32
        let s_67_41: u32 = fn_state.u_25775;
        // D s_67_42: cast zx s_67_41 -> bv
        let s_67_42: Bits = Bits::new(s_67_41 as u128, 32u16);
        // D s_67_43: bit-extract s_67_42 s_67_39 s_67_40
        let s_67_43: Bits = (Bits::new(
            ((s_67_42) >> (s_67_39)).value(),
            u16::try_from(s_67_40).unwrap(),
        ));
        // D s_67_44: cast reint s_67_43 -> u8
        let s_67_44: bool = ((s_67_43.value()) != 0);
        // C s_67_45: const #31s : i
        let s_67_45: i128 = 31;
        // C s_67_46: const #1s : i
        let s_67_46: i128 = 1;
        // D s_67_47: read-var u#25775:u32
        let s_67_47: u32 = fn_state.u_25775;
        // D s_67_48: cast zx s_67_47 -> bv
        let s_67_48: Bits = Bits::new(s_67_47 as u128, 32u16);
        // D s_67_49: bit-extract s_67_48 s_67_45 s_67_46
        let s_67_49: Bits = (Bits::new(
            ((s_67_48) >> (s_67_45)).value(),
            u16::try_from(s_67_46).unwrap(),
        ));
        // D s_67_50: cast reint s_67_49 -> u8
        let s_67_50: bool = ((s_67_49.value()) != 0);
        // D s_67_51: call decode_subs_addsub_ext_aarch64_instrs_integer_arithmetic_add_sub_extendedreg(s_67_8, s_67_14, s_67_20, s_67_26, s_67_32, s_67_38, s_67_44, s_67_50)
        let s_67_51: () = decode_subs_addsub_ext_aarch64_instrs_integer_arithmetic_add_sub_extendedreg(
            state,
            tracer,
            s_67_8,
            s_67_14,
            s_67_20,
            s_67_26,
            s_67_32,
            s_67_38,
            s_67_44,
            s_67_50,
        );
        // N s_67_52: return
        return;
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var merge#var.1:struct
        let s_68_0: u32 = fn_state.merge_var._1;
        // D s_68_1: write-var u#25785 <= s_68_0
        fn_state.u_25785 = s_68_0;
        // C s_68_2: const #24s : i
        let s_68_2: i128 = 24;
        // D s_68_3: read-var u#25785:u32
        let s_68_3: u32 = fn_state.u_25785;
        // D s_68_4: cast zx s_68_3 -> bv
        let s_68_4: Bits = Bits::new(s_68_3 as u128, 32u16);
        // C s_68_5: const #1s : i64
        let s_68_5: i64 = 1;
        // C s_68_6: cast zx s_68_5 -> i
        let s_68_6: i128 = (i128::try_from(s_68_5).unwrap());
        // C s_68_7: const #6s : i
        let s_68_7: i128 = 6;
        // C s_68_8: add s_68_7 s_68_6
        let s_68_8: i128 = (s_68_7 + s_68_6);
        // D s_68_9: bit-extract s_68_4 s_68_2 s_68_8
        let s_68_9: Bits = (Bits::new(
            ((s_68_4) >> (s_68_2)).value(),
            u16::try_from(s_68_8).unwrap(),
        ));
        // D s_68_10: cast reint s_68_9 -> u8
        let s_68_10: u8 = (s_68_9.value() as u8);
        // D s_68_11: cast zx s_68_10 -> bv
        let s_68_11: Bits = Bits::new(s_68_10 as u128, 7u16);
        // C s_68_12: const #10u : u8
        let s_68_12: u8 = 10;
        // C s_68_13: cast zx s_68_12 -> bv
        let s_68_13: Bits = Bits::new(s_68_12 as u128, 7u16);
        // D s_68_14: cmp-eq s_68_11 s_68_13
        let s_68_14: bool = ((s_68_11) == (s_68_13));
        // N s_68_15: branch s_68_14 b570 b69
        if s_68_14 {
            return block_570(state, tracer, fn_state);
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
        // D s_69_1: write-var gs#382423 <= s_69_0
        fn_state.gs_382423 = s_69_0;
        // N s_69_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var gs#382423:u8
        let s_70_0: bool = fn_state.gs_382423;
        // N s_70_1: branch s_70_0 b569 b71
        if s_70_0 {
            return block_569(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #0u : u8
        let s_71_0: bool = false;
        // D s_71_1: write-var gs#382425 <= s_71_0
        fn_state.gs_382425 = s_71_0;
        // N s_71_2: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var gs#382425:u8
        let s_72_0: bool = fn_state.gs_382425;
        // D s_72_1: not s_72_0
        let s_72_1: bool = !s_72_0;
        // N s_72_2: branch s_72_1 b74 b73
        if s_72_1 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #48s : i
        let s_73_0: i128 = 48;
        // C s_73_1: const #14696u : u32
        let s_73_1: u32 = 14696;
        // N s_73_2: write-reg s_73_1 <= s_73_0
        let s_73_2: () = {
            state.write_register::<i128>(s_73_1 as isize, s_73_0);
            tracer.write_register(s_73_1 as isize, s_73_0);
        };
        // C s_73_3: const #0s : i
        let s_73_3: i128 = 0;
        // C s_73_4: const #5s : i
        let s_73_4: i128 = 5;
        // D s_73_5: read-var u#25785:u32
        let s_73_5: u32 = fn_state.u_25785;
        // D s_73_6: cast zx s_73_5 -> bv
        let s_73_6: Bits = Bits::new(s_73_5 as u128, 32u16);
        // D s_73_7: bit-extract s_73_6 s_73_3 s_73_4
        let s_73_7: Bits = (Bits::new(
            ((s_73_6) >> (s_73_3)).value(),
            u16::try_from(s_73_4).unwrap(),
        ));
        // D s_73_8: cast reint s_73_7 -> u8
        let s_73_8: u8 = (s_73_7.value() as u8);
        // C s_73_9: const #5s : i
        let s_73_9: i128 = 5;
        // C s_73_10: const #5s : i
        let s_73_10: i128 = 5;
        // D s_73_11: read-var u#25785:u32
        let s_73_11: u32 = fn_state.u_25785;
        // D s_73_12: cast zx s_73_11 -> bv
        let s_73_12: Bits = Bits::new(s_73_11 as u128, 32u16);
        // D s_73_13: bit-extract s_73_12 s_73_9 s_73_10
        let s_73_13: Bits = (Bits::new(
            ((s_73_12) >> (s_73_9)).value(),
            u16::try_from(s_73_10).unwrap(),
        ));
        // D s_73_14: cast reint s_73_13 -> u8
        let s_73_14: u8 = (s_73_13.value() as u8);
        // C s_73_15: const #10s : i
        let s_73_15: i128 = 10;
        // C s_73_16: const #6s : i
        let s_73_16: i128 = 6;
        // D s_73_17: read-var u#25785:u32
        let s_73_17: u32 = fn_state.u_25785;
        // D s_73_18: cast zx s_73_17 -> bv
        let s_73_18: Bits = Bits::new(s_73_17 as u128, 32u16);
        // D s_73_19: bit-extract s_73_18 s_73_15 s_73_16
        let s_73_19: Bits = (Bits::new(
            ((s_73_18) >> (s_73_15)).value(),
            u16::try_from(s_73_16).unwrap(),
        ));
        // D s_73_20: cast reint s_73_19 -> u8
        let s_73_20: u8 = (s_73_19.value() as u8);
        // C s_73_21: const #16s : i
        let s_73_21: i128 = 16;
        // C s_73_22: const #5s : i
        let s_73_22: i128 = 5;
        // D s_73_23: read-var u#25785:u32
        let s_73_23: u32 = fn_state.u_25785;
        // D s_73_24: cast zx s_73_23 -> bv
        let s_73_24: Bits = Bits::new(s_73_23 as u128, 32u16);
        // D s_73_25: bit-extract s_73_24 s_73_21 s_73_22
        let s_73_25: Bits = (Bits::new(
            ((s_73_24) >> (s_73_21)).value(),
            u16::try_from(s_73_22).unwrap(),
        ));
        // D s_73_26: cast reint s_73_25 -> u8
        let s_73_26: u8 = (s_73_25.value() as u8);
        // C s_73_27: const #21s : i
        let s_73_27: i128 = 21;
        // C s_73_28: const #1s : i
        let s_73_28: i128 = 1;
        // D s_73_29: read-var u#25785:u32
        let s_73_29: u32 = fn_state.u_25785;
        // D s_73_30: cast zx s_73_29 -> bv
        let s_73_30: Bits = Bits::new(s_73_29 as u128, 32u16);
        // D s_73_31: bit-extract s_73_30 s_73_27 s_73_28
        let s_73_31: Bits = (Bits::new(
            ((s_73_30) >> (s_73_27)).value(),
            u16::try_from(s_73_28).unwrap(),
        ));
        // D s_73_32: cast reint s_73_31 -> u8
        let s_73_32: bool = ((s_73_31.value()) != 0);
        // C s_73_33: const #22s : i
        let s_73_33: i128 = 22;
        // C s_73_34: const #2s : i
        let s_73_34: i128 = 2;
        // D s_73_35: read-var u#25785:u32
        let s_73_35: u32 = fn_state.u_25785;
        // D s_73_36: cast zx s_73_35 -> bv
        let s_73_36: Bits = Bits::new(s_73_35 as u128, 32u16);
        // D s_73_37: bit-extract s_73_36 s_73_33 s_73_34
        let s_73_37: Bits = (Bits::new(
            ((s_73_36) >> (s_73_33)).value(),
            u16::try_from(s_73_34).unwrap(),
        ));
        // D s_73_38: cast reint s_73_37 -> u8
        let s_73_38: u8 = (s_73_37.value() as u8);
        // C s_73_39: const #29s : i
        let s_73_39: i128 = 29;
        // C s_73_40: const #2s : i
        let s_73_40: i128 = 2;
        // D s_73_41: read-var u#25785:u32
        let s_73_41: u32 = fn_state.u_25785;
        // D s_73_42: cast zx s_73_41 -> bv
        let s_73_42: Bits = Bits::new(s_73_41 as u128, 32u16);
        // D s_73_43: bit-extract s_73_42 s_73_39 s_73_40
        let s_73_43: Bits = (Bits::new(
            ((s_73_42) >> (s_73_39)).value(),
            u16::try_from(s_73_40).unwrap(),
        ));
        // D s_73_44: cast reint s_73_43 -> u8
        let s_73_44: u8 = (s_73_43.value() as u8);
        // C s_73_45: const #31s : i
        let s_73_45: i128 = 31;
        // C s_73_46: const #1s : i
        let s_73_46: i128 = 1;
        // D s_73_47: read-var u#25785:u32
        let s_73_47: u32 = fn_state.u_25785;
        // D s_73_48: cast zx s_73_47 -> bv
        let s_73_48: Bits = Bits::new(s_73_47 as u128, 32u16);
        // D s_73_49: bit-extract s_73_48 s_73_45 s_73_46
        let s_73_49: Bits = (Bits::new(
            ((s_73_48) >> (s_73_45)).value(),
            u16::try_from(s_73_46).unwrap(),
        ));
        // D s_73_50: cast reint s_73_49 -> u8
        let s_73_50: bool = ((s_73_49.value()) != 0);
        // D s_73_51: call decode_and_log_shift_aarch64_instrs_integer_logical_shiftedreg(s_73_8, s_73_14, s_73_20, s_73_26, s_73_32, s_73_38, s_73_44, s_73_50)
        let s_73_51: () = decode_and_log_shift_aarch64_instrs_integer_logical_shiftedreg(
            state,
            tracer,
            s_73_8,
            s_73_14,
            s_73_20,
            s_73_26,
            s_73_32,
            s_73_38,
            s_73_44,
            s_73_50,
        );
        // N s_73_52: return
        return;
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_74_0: read-var merge#var.1:struct
        let s_74_0: u32 = fn_state.merge_var._1;
        // D s_74_1: write-var u#25793 <= s_74_0
        fn_state.u_25793 = s_74_0;
        // C s_74_2: const #24s : i
        let s_74_2: i128 = 24;
        // D s_74_3: read-var u#25793:u32
        let s_74_3: u32 = fn_state.u_25793;
        // D s_74_4: cast zx s_74_3 -> bv
        let s_74_4: Bits = Bits::new(s_74_3 as u128, 32u16);
        // C s_74_5: const #1s : i64
        let s_74_5: i64 = 1;
        // C s_74_6: cast zx s_74_5 -> i
        let s_74_6: i128 = (i128::try_from(s_74_5).unwrap());
        // C s_74_7: const #6s : i
        let s_74_7: i128 = 6;
        // C s_74_8: add s_74_7 s_74_6
        let s_74_8: i128 = (s_74_7 + s_74_6);
        // D s_74_9: bit-extract s_74_4 s_74_2 s_74_8
        let s_74_9: Bits = (Bits::new(
            ((s_74_4) >> (s_74_2)).value(),
            u16::try_from(s_74_8).unwrap(),
        ));
        // D s_74_10: cast reint s_74_9 -> u8
        let s_74_10: u8 = (s_74_9.value() as u8);
        // D s_74_11: cast zx s_74_10 -> bv
        let s_74_11: Bits = Bits::new(s_74_10 as u128, 7u16);
        // C s_74_12: const #106u : u8
        let s_74_12: u8 = 106;
        // C s_74_13: cast zx s_74_12 -> bv
        let s_74_13: Bits = Bits::new(s_74_12 as u128, 7u16);
        // D s_74_14: cmp-eq s_74_11 s_74_13
        let s_74_14: bool = ((s_74_11) == (s_74_13));
        // N s_74_15: branch s_74_14 b568 b75
        if s_74_14 {
            return block_568(state, tracer, fn_state);
        } else {
            return block_75(state, tracer, fn_state);
        };
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #0u : u8
        let s_75_0: bool = false;
        // D s_75_1: write-var gs#382449 <= s_75_0
        fn_state.gs_382449 = s_75_0;
        // N s_75_2: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_76_0: read-var gs#382449:u8
        let s_76_0: bool = fn_state.gs_382449;
        // N s_76_1: branch s_76_0 b567 b77
        if s_76_0 {
            return block_567(state, tracer, fn_state);
        } else {
            return block_77(state, tracer, fn_state);
        };
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #0u : u8
        let s_77_0: bool = false;
        // D s_77_1: write-var gs#382451 <= s_77_0
        fn_state.gs_382451 = s_77_0;
        // N s_77_2: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var gs#382451:u8
        let s_78_0: bool = fn_state.gs_382451;
        // D s_78_1: not s_78_0
        let s_78_1: bool = !s_78_0;
        // N s_78_2: branch s_78_1 b80 b79
        if s_78_1 {
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
        // C s_79_0: const #49s : i
        let s_79_0: i128 = 49;
        // C s_79_1: const #14696u : u32
        let s_79_1: u32 = 14696;
        // N s_79_2: write-reg s_79_1 <= s_79_0
        let s_79_2: () = {
            state.write_register::<i128>(s_79_1 as isize, s_79_0);
            tracer.write_register(s_79_1 as isize, s_79_0);
        };
        // C s_79_3: const #0s : i
        let s_79_3: i128 = 0;
        // C s_79_4: const #5s : i
        let s_79_4: i128 = 5;
        // D s_79_5: read-var u#25793:u32
        let s_79_5: u32 = fn_state.u_25793;
        // D s_79_6: cast zx s_79_5 -> bv
        let s_79_6: Bits = Bits::new(s_79_5 as u128, 32u16);
        // D s_79_7: bit-extract s_79_6 s_79_3 s_79_4
        let s_79_7: Bits = (Bits::new(
            ((s_79_6) >> (s_79_3)).value(),
            u16::try_from(s_79_4).unwrap(),
        ));
        // D s_79_8: cast reint s_79_7 -> u8
        let s_79_8: u8 = (s_79_7.value() as u8);
        // C s_79_9: const #5s : i
        let s_79_9: i128 = 5;
        // C s_79_10: const #5s : i
        let s_79_10: i128 = 5;
        // D s_79_11: read-var u#25793:u32
        let s_79_11: u32 = fn_state.u_25793;
        // D s_79_12: cast zx s_79_11 -> bv
        let s_79_12: Bits = Bits::new(s_79_11 as u128, 32u16);
        // D s_79_13: bit-extract s_79_12 s_79_9 s_79_10
        let s_79_13: Bits = (Bits::new(
            ((s_79_12) >> (s_79_9)).value(),
            u16::try_from(s_79_10).unwrap(),
        ));
        // D s_79_14: cast reint s_79_13 -> u8
        let s_79_14: u8 = (s_79_13.value() as u8);
        // C s_79_15: const #10s : i
        let s_79_15: i128 = 10;
        // C s_79_16: const #6s : i
        let s_79_16: i128 = 6;
        // D s_79_17: read-var u#25793:u32
        let s_79_17: u32 = fn_state.u_25793;
        // D s_79_18: cast zx s_79_17 -> bv
        let s_79_18: Bits = Bits::new(s_79_17 as u128, 32u16);
        // D s_79_19: bit-extract s_79_18 s_79_15 s_79_16
        let s_79_19: Bits = (Bits::new(
            ((s_79_18) >> (s_79_15)).value(),
            u16::try_from(s_79_16).unwrap(),
        ));
        // D s_79_20: cast reint s_79_19 -> u8
        let s_79_20: u8 = (s_79_19.value() as u8);
        // C s_79_21: const #16s : i
        let s_79_21: i128 = 16;
        // C s_79_22: const #5s : i
        let s_79_22: i128 = 5;
        // D s_79_23: read-var u#25793:u32
        let s_79_23: u32 = fn_state.u_25793;
        // D s_79_24: cast zx s_79_23 -> bv
        let s_79_24: Bits = Bits::new(s_79_23 as u128, 32u16);
        // D s_79_25: bit-extract s_79_24 s_79_21 s_79_22
        let s_79_25: Bits = (Bits::new(
            ((s_79_24) >> (s_79_21)).value(),
            u16::try_from(s_79_22).unwrap(),
        ));
        // D s_79_26: cast reint s_79_25 -> u8
        let s_79_26: u8 = (s_79_25.value() as u8);
        // C s_79_27: const #21s : i
        let s_79_27: i128 = 21;
        // C s_79_28: const #1s : i
        let s_79_28: i128 = 1;
        // D s_79_29: read-var u#25793:u32
        let s_79_29: u32 = fn_state.u_25793;
        // D s_79_30: cast zx s_79_29 -> bv
        let s_79_30: Bits = Bits::new(s_79_29 as u128, 32u16);
        // D s_79_31: bit-extract s_79_30 s_79_27 s_79_28
        let s_79_31: Bits = (Bits::new(
            ((s_79_30) >> (s_79_27)).value(),
            u16::try_from(s_79_28).unwrap(),
        ));
        // D s_79_32: cast reint s_79_31 -> u8
        let s_79_32: bool = ((s_79_31.value()) != 0);
        // C s_79_33: const #22s : i
        let s_79_33: i128 = 22;
        // C s_79_34: const #2s : i
        let s_79_34: i128 = 2;
        // D s_79_35: read-var u#25793:u32
        let s_79_35: u32 = fn_state.u_25793;
        // D s_79_36: cast zx s_79_35 -> bv
        let s_79_36: Bits = Bits::new(s_79_35 as u128, 32u16);
        // D s_79_37: bit-extract s_79_36 s_79_33 s_79_34
        let s_79_37: Bits = (Bits::new(
            ((s_79_36) >> (s_79_33)).value(),
            u16::try_from(s_79_34).unwrap(),
        ));
        // D s_79_38: cast reint s_79_37 -> u8
        let s_79_38: u8 = (s_79_37.value() as u8);
        // C s_79_39: const #29s : i
        let s_79_39: i128 = 29;
        // C s_79_40: const #2s : i
        let s_79_40: i128 = 2;
        // D s_79_41: read-var u#25793:u32
        let s_79_41: u32 = fn_state.u_25793;
        // D s_79_42: cast zx s_79_41 -> bv
        let s_79_42: Bits = Bits::new(s_79_41 as u128, 32u16);
        // D s_79_43: bit-extract s_79_42 s_79_39 s_79_40
        let s_79_43: Bits = (Bits::new(
            ((s_79_42) >> (s_79_39)).value(),
            u16::try_from(s_79_40).unwrap(),
        ));
        // D s_79_44: cast reint s_79_43 -> u8
        let s_79_44: u8 = (s_79_43.value() as u8);
        // C s_79_45: const #31s : i
        let s_79_45: i128 = 31;
        // C s_79_46: const #1s : i
        let s_79_46: i128 = 1;
        // D s_79_47: read-var u#25793:u32
        let s_79_47: u32 = fn_state.u_25793;
        // D s_79_48: cast zx s_79_47 -> bv
        let s_79_48: Bits = Bits::new(s_79_47 as u128, 32u16);
        // D s_79_49: bit-extract s_79_48 s_79_45 s_79_46
        let s_79_49: Bits = (Bits::new(
            ((s_79_48) >> (s_79_45)).value(),
            u16::try_from(s_79_46).unwrap(),
        ));
        // D s_79_50: cast reint s_79_49 -> u8
        let s_79_50: bool = ((s_79_49.value()) != 0);
        // D s_79_51: call decode_ands_log_shift_aarch64_instrs_integer_logical_shiftedreg(s_79_8, s_79_14, s_79_20, s_79_26, s_79_32, s_79_38, s_79_44, s_79_50)
        let s_79_51: () = decode_ands_log_shift_aarch64_instrs_integer_logical_shiftedreg(
            state,
            tracer,
            s_79_8,
            s_79_14,
            s_79_20,
            s_79_26,
            s_79_32,
            s_79_38,
            s_79_44,
            s_79_50,
        );
        // N s_79_52: return
        return;
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_80_0: read-var merge#var.1:struct
        let s_80_0: u32 = fn_state.merge_var._1;
        // D s_80_1: write-var u#25803 <= s_80_0
        fn_state.u_25803 = s_80_0;
        // C s_80_2: const #24s : i
        let s_80_2: i128 = 24;
        // D s_80_3: read-var u#25803:u32
        let s_80_3: u32 = fn_state.u_25803;
        // D s_80_4: cast zx s_80_3 -> bv
        let s_80_4: Bits = Bits::new(s_80_3 as u128, 32u16);
        // C s_80_5: const #1s : i64
        let s_80_5: i64 = 1;
        // C s_80_6: cast zx s_80_5 -> i
        let s_80_6: i128 = (i128::try_from(s_80_5).unwrap());
        // C s_80_7: const #6s : i
        let s_80_7: i128 = 6;
        // C s_80_8: add s_80_7 s_80_6
        let s_80_8: i128 = (s_80_7 + s_80_6);
        // D s_80_9: bit-extract s_80_4 s_80_2 s_80_8
        let s_80_9: Bits = (Bits::new(
            ((s_80_4) >> (s_80_2)).value(),
            u16::try_from(s_80_8).unwrap(),
        ));
        // D s_80_10: cast reint s_80_9 -> u8
        let s_80_10: u8 = (s_80_9.value() as u8);
        // D s_80_11: cast zx s_80_10 -> bv
        let s_80_11: Bits = Bits::new(s_80_10 as u128, 7u16);
        // C s_80_12: const #10u : u8
        let s_80_12: u8 = 10;
        // C s_80_13: cast zx s_80_12 -> bv
        let s_80_13: Bits = Bits::new(s_80_12 as u128, 7u16);
        // D s_80_14: cmp-eq s_80_11 s_80_13
        let s_80_14: bool = ((s_80_11) == (s_80_13));
        // N s_80_15: branch s_80_14 b566 b81
        if s_80_14 {
            return block_566(state, tracer, fn_state);
        } else {
            return block_81(state, tracer, fn_state);
        };
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #0u : u8
        let s_81_0: bool = false;
        // D s_81_1: write-var gs#382475 <= s_81_0
        fn_state.gs_382475 = s_81_0;
        // N s_81_2: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var gs#382475:u8
        let s_82_0: bool = fn_state.gs_382475;
        // N s_82_1: branch s_82_0 b565 b83
        if s_82_0 {
            return block_565(state, tracer, fn_state);
        } else {
            return block_83(state, tracer, fn_state);
        };
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #0u : u8
        let s_83_0: bool = false;
        // D s_83_1: write-var gs#382477 <= s_83_0
        fn_state.gs_382477 = s_83_0;
        // N s_83_2: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var gs#382477:u8
        let s_84_0: bool = fn_state.gs_382477;
        // D s_84_1: not s_84_0
        let s_84_1: bool = !s_84_0;
        // N s_84_2: branch s_84_1 b86 b85
        if s_84_1 {
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
        // C s_85_0: const #50s : i
        let s_85_0: i128 = 50;
        // C s_85_1: const #14696u : u32
        let s_85_1: u32 = 14696;
        // N s_85_2: write-reg s_85_1 <= s_85_0
        let s_85_2: () = {
            state.write_register::<i128>(s_85_1 as isize, s_85_0);
            tracer.write_register(s_85_1 as isize, s_85_0);
        };
        // C s_85_3: const #0s : i
        let s_85_3: i128 = 0;
        // C s_85_4: const #5s : i
        let s_85_4: i128 = 5;
        // D s_85_5: read-var u#25803:u32
        let s_85_5: u32 = fn_state.u_25803;
        // D s_85_6: cast zx s_85_5 -> bv
        let s_85_6: Bits = Bits::new(s_85_5 as u128, 32u16);
        // D s_85_7: bit-extract s_85_6 s_85_3 s_85_4
        let s_85_7: Bits = (Bits::new(
            ((s_85_6) >> (s_85_3)).value(),
            u16::try_from(s_85_4).unwrap(),
        ));
        // D s_85_8: cast reint s_85_7 -> u8
        let s_85_8: u8 = (s_85_7.value() as u8);
        // C s_85_9: const #5s : i
        let s_85_9: i128 = 5;
        // C s_85_10: const #5s : i
        let s_85_10: i128 = 5;
        // D s_85_11: read-var u#25803:u32
        let s_85_11: u32 = fn_state.u_25803;
        // D s_85_12: cast zx s_85_11 -> bv
        let s_85_12: Bits = Bits::new(s_85_11 as u128, 32u16);
        // D s_85_13: bit-extract s_85_12 s_85_9 s_85_10
        let s_85_13: Bits = (Bits::new(
            ((s_85_12) >> (s_85_9)).value(),
            u16::try_from(s_85_10).unwrap(),
        ));
        // D s_85_14: cast reint s_85_13 -> u8
        let s_85_14: u8 = (s_85_13.value() as u8);
        // C s_85_15: const #10s : i
        let s_85_15: i128 = 10;
        // C s_85_16: const #6s : i
        let s_85_16: i128 = 6;
        // D s_85_17: read-var u#25803:u32
        let s_85_17: u32 = fn_state.u_25803;
        // D s_85_18: cast zx s_85_17 -> bv
        let s_85_18: Bits = Bits::new(s_85_17 as u128, 32u16);
        // D s_85_19: bit-extract s_85_18 s_85_15 s_85_16
        let s_85_19: Bits = (Bits::new(
            ((s_85_18) >> (s_85_15)).value(),
            u16::try_from(s_85_16).unwrap(),
        ));
        // D s_85_20: cast reint s_85_19 -> u8
        let s_85_20: u8 = (s_85_19.value() as u8);
        // C s_85_21: const #16s : i
        let s_85_21: i128 = 16;
        // C s_85_22: const #5s : i
        let s_85_22: i128 = 5;
        // D s_85_23: read-var u#25803:u32
        let s_85_23: u32 = fn_state.u_25803;
        // D s_85_24: cast zx s_85_23 -> bv
        let s_85_24: Bits = Bits::new(s_85_23 as u128, 32u16);
        // D s_85_25: bit-extract s_85_24 s_85_21 s_85_22
        let s_85_25: Bits = (Bits::new(
            ((s_85_24) >> (s_85_21)).value(),
            u16::try_from(s_85_22).unwrap(),
        ));
        // D s_85_26: cast reint s_85_25 -> u8
        let s_85_26: u8 = (s_85_25.value() as u8);
        // C s_85_27: const #21s : i
        let s_85_27: i128 = 21;
        // C s_85_28: const #1s : i
        let s_85_28: i128 = 1;
        // D s_85_29: read-var u#25803:u32
        let s_85_29: u32 = fn_state.u_25803;
        // D s_85_30: cast zx s_85_29 -> bv
        let s_85_30: Bits = Bits::new(s_85_29 as u128, 32u16);
        // D s_85_31: bit-extract s_85_30 s_85_27 s_85_28
        let s_85_31: Bits = (Bits::new(
            ((s_85_30) >> (s_85_27)).value(),
            u16::try_from(s_85_28).unwrap(),
        ));
        // D s_85_32: cast reint s_85_31 -> u8
        let s_85_32: bool = ((s_85_31.value()) != 0);
        // C s_85_33: const #22s : i
        let s_85_33: i128 = 22;
        // C s_85_34: const #2s : i
        let s_85_34: i128 = 2;
        // D s_85_35: read-var u#25803:u32
        let s_85_35: u32 = fn_state.u_25803;
        // D s_85_36: cast zx s_85_35 -> bv
        let s_85_36: Bits = Bits::new(s_85_35 as u128, 32u16);
        // D s_85_37: bit-extract s_85_36 s_85_33 s_85_34
        let s_85_37: Bits = (Bits::new(
            ((s_85_36) >> (s_85_33)).value(),
            u16::try_from(s_85_34).unwrap(),
        ));
        // D s_85_38: cast reint s_85_37 -> u8
        let s_85_38: u8 = (s_85_37.value() as u8);
        // C s_85_39: const #29s : i
        let s_85_39: i128 = 29;
        // C s_85_40: const #2s : i
        let s_85_40: i128 = 2;
        // D s_85_41: read-var u#25803:u32
        let s_85_41: u32 = fn_state.u_25803;
        // D s_85_42: cast zx s_85_41 -> bv
        let s_85_42: Bits = Bits::new(s_85_41 as u128, 32u16);
        // D s_85_43: bit-extract s_85_42 s_85_39 s_85_40
        let s_85_43: Bits = (Bits::new(
            ((s_85_42) >> (s_85_39)).value(),
            u16::try_from(s_85_40).unwrap(),
        ));
        // D s_85_44: cast reint s_85_43 -> u8
        let s_85_44: u8 = (s_85_43.value() as u8);
        // C s_85_45: const #31s : i
        let s_85_45: i128 = 31;
        // C s_85_46: const #1s : i
        let s_85_46: i128 = 1;
        // D s_85_47: read-var u#25803:u32
        let s_85_47: u32 = fn_state.u_25803;
        // D s_85_48: cast zx s_85_47 -> bv
        let s_85_48: Bits = Bits::new(s_85_47 as u128, 32u16);
        // D s_85_49: bit-extract s_85_48 s_85_45 s_85_46
        let s_85_49: Bits = (Bits::new(
            ((s_85_48) >> (s_85_45)).value(),
            u16::try_from(s_85_46).unwrap(),
        ));
        // D s_85_50: cast reint s_85_49 -> u8
        let s_85_50: bool = ((s_85_49.value()) != 0);
        // D s_85_51: call decode_bic_log_shift_aarch64_instrs_integer_logical_shiftedreg(s_85_8, s_85_14, s_85_20, s_85_26, s_85_32, s_85_38, s_85_44, s_85_50)
        let s_85_51: () = decode_bic_log_shift_aarch64_instrs_integer_logical_shiftedreg(
            state,
            tracer,
            s_85_8,
            s_85_14,
            s_85_20,
            s_85_26,
            s_85_32,
            s_85_38,
            s_85_44,
            s_85_50,
        );
        // N s_85_52: return
        return;
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var merge#var.1:struct
        let s_86_0: u32 = fn_state.merge_var._1;
        // D s_86_1: write-var u#25813 <= s_86_0
        fn_state.u_25813 = s_86_0;
        // C s_86_2: const #24s : i
        let s_86_2: i128 = 24;
        // D s_86_3: read-var u#25813:u32
        let s_86_3: u32 = fn_state.u_25813;
        // D s_86_4: cast zx s_86_3 -> bv
        let s_86_4: Bits = Bits::new(s_86_3 as u128, 32u16);
        // C s_86_5: const #1s : i64
        let s_86_5: i64 = 1;
        // C s_86_6: cast zx s_86_5 -> i
        let s_86_6: i128 = (i128::try_from(s_86_5).unwrap());
        // C s_86_7: const #6s : i
        let s_86_7: i128 = 6;
        // C s_86_8: add s_86_7 s_86_6
        let s_86_8: i128 = (s_86_7 + s_86_6);
        // D s_86_9: bit-extract s_86_4 s_86_2 s_86_8
        let s_86_9: Bits = (Bits::new(
            ((s_86_4) >> (s_86_2)).value(),
            u16::try_from(s_86_8).unwrap(),
        ));
        // D s_86_10: cast reint s_86_9 -> u8
        let s_86_10: u8 = (s_86_9.value() as u8);
        // D s_86_11: cast zx s_86_10 -> bv
        let s_86_11: Bits = Bits::new(s_86_10 as u128, 7u16);
        // C s_86_12: const #106u : u8
        let s_86_12: u8 = 106;
        // C s_86_13: cast zx s_86_12 -> bv
        let s_86_13: Bits = Bits::new(s_86_12 as u128, 7u16);
        // D s_86_14: cmp-eq s_86_11 s_86_13
        let s_86_14: bool = ((s_86_11) == (s_86_13));
        // N s_86_15: branch s_86_14 b564 b87
        if s_86_14 {
            return block_564(state, tracer, fn_state);
        } else {
            return block_87(state, tracer, fn_state);
        };
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #0u : u8
        let s_87_0: bool = false;
        // D s_87_1: write-var gs#382501 <= s_87_0
        fn_state.gs_382501 = s_87_0;
        // N s_87_2: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var gs#382501:u8
        let s_88_0: bool = fn_state.gs_382501;
        // N s_88_1: branch s_88_0 b563 b89
        if s_88_0 {
            return block_563(state, tracer, fn_state);
        } else {
            return block_89(state, tracer, fn_state);
        };
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #0u : u8
        let s_89_0: bool = false;
        // D s_89_1: write-var gs#382503 <= s_89_0
        fn_state.gs_382503 = s_89_0;
        // N s_89_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var gs#382503:u8
        let s_90_0: bool = fn_state.gs_382503;
        // D s_90_1: not s_90_0
        let s_90_1: bool = !s_90_0;
        // N s_90_2: branch s_90_1 b92 b91
        if s_90_1 {
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
        // C s_91_0: const #51s : i
        let s_91_0: i128 = 51;
        // C s_91_1: const #14696u : u32
        let s_91_1: u32 = 14696;
        // N s_91_2: write-reg s_91_1 <= s_91_0
        let s_91_2: () = {
            state.write_register::<i128>(s_91_1 as isize, s_91_0);
            tracer.write_register(s_91_1 as isize, s_91_0);
        };
        // C s_91_3: const #0s : i
        let s_91_3: i128 = 0;
        // C s_91_4: const #5s : i
        let s_91_4: i128 = 5;
        // D s_91_5: read-var u#25813:u32
        let s_91_5: u32 = fn_state.u_25813;
        // D s_91_6: cast zx s_91_5 -> bv
        let s_91_6: Bits = Bits::new(s_91_5 as u128, 32u16);
        // D s_91_7: bit-extract s_91_6 s_91_3 s_91_4
        let s_91_7: Bits = (Bits::new(
            ((s_91_6) >> (s_91_3)).value(),
            u16::try_from(s_91_4).unwrap(),
        ));
        // D s_91_8: cast reint s_91_7 -> u8
        let s_91_8: u8 = (s_91_7.value() as u8);
        // C s_91_9: const #5s : i
        let s_91_9: i128 = 5;
        // C s_91_10: const #5s : i
        let s_91_10: i128 = 5;
        // D s_91_11: read-var u#25813:u32
        let s_91_11: u32 = fn_state.u_25813;
        // D s_91_12: cast zx s_91_11 -> bv
        let s_91_12: Bits = Bits::new(s_91_11 as u128, 32u16);
        // D s_91_13: bit-extract s_91_12 s_91_9 s_91_10
        let s_91_13: Bits = (Bits::new(
            ((s_91_12) >> (s_91_9)).value(),
            u16::try_from(s_91_10).unwrap(),
        ));
        // D s_91_14: cast reint s_91_13 -> u8
        let s_91_14: u8 = (s_91_13.value() as u8);
        // C s_91_15: const #10s : i
        let s_91_15: i128 = 10;
        // C s_91_16: const #6s : i
        let s_91_16: i128 = 6;
        // D s_91_17: read-var u#25813:u32
        let s_91_17: u32 = fn_state.u_25813;
        // D s_91_18: cast zx s_91_17 -> bv
        let s_91_18: Bits = Bits::new(s_91_17 as u128, 32u16);
        // D s_91_19: bit-extract s_91_18 s_91_15 s_91_16
        let s_91_19: Bits = (Bits::new(
            ((s_91_18) >> (s_91_15)).value(),
            u16::try_from(s_91_16).unwrap(),
        ));
        // D s_91_20: cast reint s_91_19 -> u8
        let s_91_20: u8 = (s_91_19.value() as u8);
        // C s_91_21: const #16s : i
        let s_91_21: i128 = 16;
        // C s_91_22: const #5s : i
        let s_91_22: i128 = 5;
        // D s_91_23: read-var u#25813:u32
        let s_91_23: u32 = fn_state.u_25813;
        // D s_91_24: cast zx s_91_23 -> bv
        let s_91_24: Bits = Bits::new(s_91_23 as u128, 32u16);
        // D s_91_25: bit-extract s_91_24 s_91_21 s_91_22
        let s_91_25: Bits = (Bits::new(
            ((s_91_24) >> (s_91_21)).value(),
            u16::try_from(s_91_22).unwrap(),
        ));
        // D s_91_26: cast reint s_91_25 -> u8
        let s_91_26: u8 = (s_91_25.value() as u8);
        // C s_91_27: const #21s : i
        let s_91_27: i128 = 21;
        // C s_91_28: const #1s : i
        let s_91_28: i128 = 1;
        // D s_91_29: read-var u#25813:u32
        let s_91_29: u32 = fn_state.u_25813;
        // D s_91_30: cast zx s_91_29 -> bv
        let s_91_30: Bits = Bits::new(s_91_29 as u128, 32u16);
        // D s_91_31: bit-extract s_91_30 s_91_27 s_91_28
        let s_91_31: Bits = (Bits::new(
            ((s_91_30) >> (s_91_27)).value(),
            u16::try_from(s_91_28).unwrap(),
        ));
        // D s_91_32: cast reint s_91_31 -> u8
        let s_91_32: bool = ((s_91_31.value()) != 0);
        // C s_91_33: const #22s : i
        let s_91_33: i128 = 22;
        // C s_91_34: const #2s : i
        let s_91_34: i128 = 2;
        // D s_91_35: read-var u#25813:u32
        let s_91_35: u32 = fn_state.u_25813;
        // D s_91_36: cast zx s_91_35 -> bv
        let s_91_36: Bits = Bits::new(s_91_35 as u128, 32u16);
        // D s_91_37: bit-extract s_91_36 s_91_33 s_91_34
        let s_91_37: Bits = (Bits::new(
            ((s_91_36) >> (s_91_33)).value(),
            u16::try_from(s_91_34).unwrap(),
        ));
        // D s_91_38: cast reint s_91_37 -> u8
        let s_91_38: u8 = (s_91_37.value() as u8);
        // C s_91_39: const #29s : i
        let s_91_39: i128 = 29;
        // C s_91_40: const #2s : i
        let s_91_40: i128 = 2;
        // D s_91_41: read-var u#25813:u32
        let s_91_41: u32 = fn_state.u_25813;
        // D s_91_42: cast zx s_91_41 -> bv
        let s_91_42: Bits = Bits::new(s_91_41 as u128, 32u16);
        // D s_91_43: bit-extract s_91_42 s_91_39 s_91_40
        let s_91_43: Bits = (Bits::new(
            ((s_91_42) >> (s_91_39)).value(),
            u16::try_from(s_91_40).unwrap(),
        ));
        // D s_91_44: cast reint s_91_43 -> u8
        let s_91_44: u8 = (s_91_43.value() as u8);
        // C s_91_45: const #31s : i
        let s_91_45: i128 = 31;
        // C s_91_46: const #1s : i
        let s_91_46: i128 = 1;
        // D s_91_47: read-var u#25813:u32
        let s_91_47: u32 = fn_state.u_25813;
        // D s_91_48: cast zx s_91_47 -> bv
        let s_91_48: Bits = Bits::new(s_91_47 as u128, 32u16);
        // D s_91_49: bit-extract s_91_48 s_91_45 s_91_46
        let s_91_49: Bits = (Bits::new(
            ((s_91_48) >> (s_91_45)).value(),
            u16::try_from(s_91_46).unwrap(),
        ));
        // D s_91_50: cast reint s_91_49 -> u8
        let s_91_50: bool = ((s_91_49.value()) != 0);
        // D s_91_51: call decode_bics_aarch64_instrs_integer_logical_shiftedreg(s_91_8, s_91_14, s_91_20, s_91_26, s_91_32, s_91_38, s_91_44, s_91_50)
        let s_91_51: () = decode_bics_aarch64_instrs_integer_logical_shiftedreg(
            state,
            tracer,
            s_91_8,
            s_91_14,
            s_91_20,
            s_91_26,
            s_91_32,
            s_91_38,
            s_91_44,
            s_91_50,
        );
        // N s_91_52: return
        return;
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var merge#var.1:struct
        let s_92_0: u32 = fn_state.merge_var._1;
        // D s_92_1: write-var u#25823 <= s_92_0
        fn_state.u_25823 = s_92_0;
        // C s_92_2: const #24s : i
        let s_92_2: i128 = 24;
        // D s_92_3: read-var u#25823:u32
        let s_92_3: u32 = fn_state.u_25823;
        // D s_92_4: cast zx s_92_3 -> bv
        let s_92_4: Bits = Bits::new(s_92_3 as u128, 32u16);
        // C s_92_5: const #1s : i64
        let s_92_5: i64 = 1;
        // C s_92_6: cast zx s_92_5 -> i
        let s_92_6: i128 = (i128::try_from(s_92_5).unwrap());
        // C s_92_7: const #6s : i
        let s_92_7: i128 = 6;
        // C s_92_8: add s_92_7 s_92_6
        let s_92_8: i128 = (s_92_7 + s_92_6);
        // D s_92_9: bit-extract s_92_4 s_92_2 s_92_8
        let s_92_9: Bits = (Bits::new(
            ((s_92_4) >> (s_92_2)).value(),
            u16::try_from(s_92_8).unwrap(),
        ));
        // D s_92_10: cast reint s_92_9 -> u8
        let s_92_10: u8 = (s_92_9.value() as u8);
        // D s_92_11: cast zx s_92_10 -> bv
        let s_92_11: Bits = Bits::new(s_92_10 as u128, 7u16);
        // C s_92_12: const #74u : u8
        let s_92_12: u8 = 74;
        // C s_92_13: cast zx s_92_12 -> bv
        let s_92_13: Bits = Bits::new(s_92_12 as u128, 7u16);
        // D s_92_14: cmp-eq s_92_11 s_92_13
        let s_92_14: bool = ((s_92_11) == (s_92_13));
        // N s_92_15: branch s_92_14 b562 b93
        if s_92_14 {
            return block_562(state, tracer, fn_state);
        } else {
            return block_93(state, tracer, fn_state);
        };
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #0u : u8
        let s_93_0: bool = false;
        // D s_93_1: write-var gs#382527 <= s_93_0
        fn_state.gs_382527 = s_93_0;
        // N s_93_2: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_94_0: read-var gs#382527:u8
        let s_94_0: bool = fn_state.gs_382527;
        // N s_94_1: branch s_94_0 b561 b95
        if s_94_0 {
            return block_561(state, tracer, fn_state);
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
        // D s_95_1: write-var gs#382529 <= s_95_0
        fn_state.gs_382529 = s_95_0;
        // N s_95_2: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_96_0: read-var gs#382529:u8
        let s_96_0: bool = fn_state.gs_382529;
        // D s_96_1: not s_96_0
        let s_96_1: bool = !s_96_0;
        // N s_96_2: branch s_96_1 b98 b97
        if s_96_1 {
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
        // C s_97_0: const #52s : i
        let s_97_0: i128 = 52;
        // C s_97_1: const #14696u : u32
        let s_97_1: u32 = 14696;
        // N s_97_2: write-reg s_97_1 <= s_97_0
        let s_97_2: () = {
            state.write_register::<i128>(s_97_1 as isize, s_97_0);
            tracer.write_register(s_97_1 as isize, s_97_0);
        };
        // C s_97_3: const #0s : i
        let s_97_3: i128 = 0;
        // C s_97_4: const #5s : i
        let s_97_4: i128 = 5;
        // D s_97_5: read-var u#25823:u32
        let s_97_5: u32 = fn_state.u_25823;
        // D s_97_6: cast zx s_97_5 -> bv
        let s_97_6: Bits = Bits::new(s_97_5 as u128, 32u16);
        // D s_97_7: bit-extract s_97_6 s_97_3 s_97_4
        let s_97_7: Bits = (Bits::new(
            ((s_97_6) >> (s_97_3)).value(),
            u16::try_from(s_97_4).unwrap(),
        ));
        // D s_97_8: cast reint s_97_7 -> u8
        let s_97_8: u8 = (s_97_7.value() as u8);
        // C s_97_9: const #5s : i
        let s_97_9: i128 = 5;
        // C s_97_10: const #5s : i
        let s_97_10: i128 = 5;
        // D s_97_11: read-var u#25823:u32
        let s_97_11: u32 = fn_state.u_25823;
        // D s_97_12: cast zx s_97_11 -> bv
        let s_97_12: Bits = Bits::new(s_97_11 as u128, 32u16);
        // D s_97_13: bit-extract s_97_12 s_97_9 s_97_10
        let s_97_13: Bits = (Bits::new(
            ((s_97_12) >> (s_97_9)).value(),
            u16::try_from(s_97_10).unwrap(),
        ));
        // D s_97_14: cast reint s_97_13 -> u8
        let s_97_14: u8 = (s_97_13.value() as u8);
        // C s_97_15: const #10s : i
        let s_97_15: i128 = 10;
        // C s_97_16: const #6s : i
        let s_97_16: i128 = 6;
        // D s_97_17: read-var u#25823:u32
        let s_97_17: u32 = fn_state.u_25823;
        // D s_97_18: cast zx s_97_17 -> bv
        let s_97_18: Bits = Bits::new(s_97_17 as u128, 32u16);
        // D s_97_19: bit-extract s_97_18 s_97_15 s_97_16
        let s_97_19: Bits = (Bits::new(
            ((s_97_18) >> (s_97_15)).value(),
            u16::try_from(s_97_16).unwrap(),
        ));
        // D s_97_20: cast reint s_97_19 -> u8
        let s_97_20: u8 = (s_97_19.value() as u8);
        // C s_97_21: const #16s : i
        let s_97_21: i128 = 16;
        // C s_97_22: const #5s : i
        let s_97_22: i128 = 5;
        // D s_97_23: read-var u#25823:u32
        let s_97_23: u32 = fn_state.u_25823;
        // D s_97_24: cast zx s_97_23 -> bv
        let s_97_24: Bits = Bits::new(s_97_23 as u128, 32u16);
        // D s_97_25: bit-extract s_97_24 s_97_21 s_97_22
        let s_97_25: Bits = (Bits::new(
            ((s_97_24) >> (s_97_21)).value(),
            u16::try_from(s_97_22).unwrap(),
        ));
        // D s_97_26: cast reint s_97_25 -> u8
        let s_97_26: u8 = (s_97_25.value() as u8);
        // C s_97_27: const #21s : i
        let s_97_27: i128 = 21;
        // C s_97_28: const #1s : i
        let s_97_28: i128 = 1;
        // D s_97_29: read-var u#25823:u32
        let s_97_29: u32 = fn_state.u_25823;
        // D s_97_30: cast zx s_97_29 -> bv
        let s_97_30: Bits = Bits::new(s_97_29 as u128, 32u16);
        // D s_97_31: bit-extract s_97_30 s_97_27 s_97_28
        let s_97_31: Bits = (Bits::new(
            ((s_97_30) >> (s_97_27)).value(),
            u16::try_from(s_97_28).unwrap(),
        ));
        // D s_97_32: cast reint s_97_31 -> u8
        let s_97_32: bool = ((s_97_31.value()) != 0);
        // C s_97_33: const #22s : i
        let s_97_33: i128 = 22;
        // C s_97_34: const #2s : i
        let s_97_34: i128 = 2;
        // D s_97_35: read-var u#25823:u32
        let s_97_35: u32 = fn_state.u_25823;
        // D s_97_36: cast zx s_97_35 -> bv
        let s_97_36: Bits = Bits::new(s_97_35 as u128, 32u16);
        // D s_97_37: bit-extract s_97_36 s_97_33 s_97_34
        let s_97_37: Bits = (Bits::new(
            ((s_97_36) >> (s_97_33)).value(),
            u16::try_from(s_97_34).unwrap(),
        ));
        // D s_97_38: cast reint s_97_37 -> u8
        let s_97_38: u8 = (s_97_37.value() as u8);
        // C s_97_39: const #29s : i
        let s_97_39: i128 = 29;
        // C s_97_40: const #2s : i
        let s_97_40: i128 = 2;
        // D s_97_41: read-var u#25823:u32
        let s_97_41: u32 = fn_state.u_25823;
        // D s_97_42: cast zx s_97_41 -> bv
        let s_97_42: Bits = Bits::new(s_97_41 as u128, 32u16);
        // D s_97_43: bit-extract s_97_42 s_97_39 s_97_40
        let s_97_43: Bits = (Bits::new(
            ((s_97_42) >> (s_97_39)).value(),
            u16::try_from(s_97_40).unwrap(),
        ));
        // D s_97_44: cast reint s_97_43 -> u8
        let s_97_44: u8 = (s_97_43.value() as u8);
        // C s_97_45: const #31s : i
        let s_97_45: i128 = 31;
        // C s_97_46: const #1s : i
        let s_97_46: i128 = 1;
        // D s_97_47: read-var u#25823:u32
        let s_97_47: u32 = fn_state.u_25823;
        // D s_97_48: cast zx s_97_47 -> bv
        let s_97_48: Bits = Bits::new(s_97_47 as u128, 32u16);
        // D s_97_49: bit-extract s_97_48 s_97_45 s_97_46
        let s_97_49: Bits = (Bits::new(
            ((s_97_48) >> (s_97_45)).value(),
            u16::try_from(s_97_46).unwrap(),
        ));
        // D s_97_50: cast reint s_97_49 -> u8
        let s_97_50: bool = ((s_97_49.value()) != 0);
        // D s_97_51: call decode_eon_aarch64_instrs_integer_logical_shiftedreg(s_97_8, s_97_14, s_97_20, s_97_26, s_97_32, s_97_38, s_97_44, s_97_50)
        let s_97_51: () = decode_eon_aarch64_instrs_integer_logical_shiftedreg(
            state,
            tracer,
            s_97_8,
            s_97_14,
            s_97_20,
            s_97_26,
            s_97_32,
            s_97_38,
            s_97_44,
            s_97_50,
        );
        // N s_97_52: return
        return;
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_98_0: read-var merge#var.1:struct
        let s_98_0: u32 = fn_state.merge_var._1;
        // D s_98_1: write-var u#25833 <= s_98_0
        fn_state.u_25833 = s_98_0;
        // C s_98_2: const #24s : i
        let s_98_2: i128 = 24;
        // D s_98_3: read-var u#25833:u32
        let s_98_3: u32 = fn_state.u_25833;
        // D s_98_4: cast zx s_98_3 -> bv
        let s_98_4: Bits = Bits::new(s_98_3 as u128, 32u16);
        // C s_98_5: const #1s : i64
        let s_98_5: i64 = 1;
        // C s_98_6: cast zx s_98_5 -> i
        let s_98_6: i128 = (i128::try_from(s_98_5).unwrap());
        // C s_98_7: const #6s : i
        let s_98_7: i128 = 6;
        // C s_98_8: add s_98_7 s_98_6
        let s_98_8: i128 = (s_98_7 + s_98_6);
        // D s_98_9: bit-extract s_98_4 s_98_2 s_98_8
        let s_98_9: Bits = (Bits::new(
            ((s_98_4) >> (s_98_2)).value(),
            u16::try_from(s_98_8).unwrap(),
        ));
        // D s_98_10: cast reint s_98_9 -> u8
        let s_98_10: u8 = (s_98_9.value() as u8);
        // D s_98_11: cast zx s_98_10 -> bv
        let s_98_11: Bits = Bits::new(s_98_10 as u128, 7u16);
        // C s_98_12: const #74u : u8
        let s_98_12: u8 = 74;
        // C s_98_13: cast zx s_98_12 -> bv
        let s_98_13: Bits = Bits::new(s_98_12 as u128, 7u16);
        // D s_98_14: cmp-eq s_98_11 s_98_13
        let s_98_14: bool = ((s_98_11) == (s_98_13));
        // N s_98_15: branch s_98_14 b560 b99
        if s_98_14 {
            return block_560(state, tracer, fn_state);
        } else {
            return block_99(state, tracer, fn_state);
        };
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #0u : u8
        let s_99_0: bool = false;
        // D s_99_1: write-var gs#382553 <= s_99_0
        fn_state.gs_382553 = s_99_0;
        // N s_99_2: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_100_0: read-var gs#382553:u8
        let s_100_0: bool = fn_state.gs_382553;
        // N s_100_1: branch s_100_0 b559 b101
        if s_100_0 {
            return block_559(state, tracer, fn_state);
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
        // D s_101_1: write-var gs#382555 <= s_101_0
        fn_state.gs_382555 = s_101_0;
        // N s_101_2: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_102_0: read-var gs#382555:u8
        let s_102_0: bool = fn_state.gs_382555;
        // D s_102_1: not s_102_0
        let s_102_1: bool = !s_102_0;
        // N s_102_2: branch s_102_1 b104 b103
        if s_102_1 {
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
        // C s_103_0: const #53s : i
        let s_103_0: i128 = 53;
        // C s_103_1: const #14696u : u32
        let s_103_1: u32 = 14696;
        // N s_103_2: write-reg s_103_1 <= s_103_0
        let s_103_2: () = {
            state.write_register::<i128>(s_103_1 as isize, s_103_0);
            tracer.write_register(s_103_1 as isize, s_103_0);
        };
        // C s_103_3: const #0s : i
        let s_103_3: i128 = 0;
        // C s_103_4: const #5s : i
        let s_103_4: i128 = 5;
        // D s_103_5: read-var u#25833:u32
        let s_103_5: u32 = fn_state.u_25833;
        // D s_103_6: cast zx s_103_5 -> bv
        let s_103_6: Bits = Bits::new(s_103_5 as u128, 32u16);
        // D s_103_7: bit-extract s_103_6 s_103_3 s_103_4
        let s_103_7: Bits = (Bits::new(
            ((s_103_6) >> (s_103_3)).value(),
            u16::try_from(s_103_4).unwrap(),
        ));
        // D s_103_8: cast reint s_103_7 -> u8
        let s_103_8: u8 = (s_103_7.value() as u8);
        // C s_103_9: const #5s : i
        let s_103_9: i128 = 5;
        // C s_103_10: const #5s : i
        let s_103_10: i128 = 5;
        // D s_103_11: read-var u#25833:u32
        let s_103_11: u32 = fn_state.u_25833;
        // D s_103_12: cast zx s_103_11 -> bv
        let s_103_12: Bits = Bits::new(s_103_11 as u128, 32u16);
        // D s_103_13: bit-extract s_103_12 s_103_9 s_103_10
        let s_103_13: Bits = (Bits::new(
            ((s_103_12) >> (s_103_9)).value(),
            u16::try_from(s_103_10).unwrap(),
        ));
        // D s_103_14: cast reint s_103_13 -> u8
        let s_103_14: u8 = (s_103_13.value() as u8);
        // C s_103_15: const #10s : i
        let s_103_15: i128 = 10;
        // C s_103_16: const #6s : i
        let s_103_16: i128 = 6;
        // D s_103_17: read-var u#25833:u32
        let s_103_17: u32 = fn_state.u_25833;
        // D s_103_18: cast zx s_103_17 -> bv
        let s_103_18: Bits = Bits::new(s_103_17 as u128, 32u16);
        // D s_103_19: bit-extract s_103_18 s_103_15 s_103_16
        let s_103_19: Bits = (Bits::new(
            ((s_103_18) >> (s_103_15)).value(),
            u16::try_from(s_103_16).unwrap(),
        ));
        // D s_103_20: cast reint s_103_19 -> u8
        let s_103_20: u8 = (s_103_19.value() as u8);
        // C s_103_21: const #16s : i
        let s_103_21: i128 = 16;
        // C s_103_22: const #5s : i
        let s_103_22: i128 = 5;
        // D s_103_23: read-var u#25833:u32
        let s_103_23: u32 = fn_state.u_25833;
        // D s_103_24: cast zx s_103_23 -> bv
        let s_103_24: Bits = Bits::new(s_103_23 as u128, 32u16);
        // D s_103_25: bit-extract s_103_24 s_103_21 s_103_22
        let s_103_25: Bits = (Bits::new(
            ((s_103_24) >> (s_103_21)).value(),
            u16::try_from(s_103_22).unwrap(),
        ));
        // D s_103_26: cast reint s_103_25 -> u8
        let s_103_26: u8 = (s_103_25.value() as u8);
        // C s_103_27: const #21s : i
        let s_103_27: i128 = 21;
        // C s_103_28: const #1s : i
        let s_103_28: i128 = 1;
        // D s_103_29: read-var u#25833:u32
        let s_103_29: u32 = fn_state.u_25833;
        // D s_103_30: cast zx s_103_29 -> bv
        let s_103_30: Bits = Bits::new(s_103_29 as u128, 32u16);
        // D s_103_31: bit-extract s_103_30 s_103_27 s_103_28
        let s_103_31: Bits = (Bits::new(
            ((s_103_30) >> (s_103_27)).value(),
            u16::try_from(s_103_28).unwrap(),
        ));
        // D s_103_32: cast reint s_103_31 -> u8
        let s_103_32: bool = ((s_103_31.value()) != 0);
        // C s_103_33: const #22s : i
        let s_103_33: i128 = 22;
        // C s_103_34: const #2s : i
        let s_103_34: i128 = 2;
        // D s_103_35: read-var u#25833:u32
        let s_103_35: u32 = fn_state.u_25833;
        // D s_103_36: cast zx s_103_35 -> bv
        let s_103_36: Bits = Bits::new(s_103_35 as u128, 32u16);
        // D s_103_37: bit-extract s_103_36 s_103_33 s_103_34
        let s_103_37: Bits = (Bits::new(
            ((s_103_36) >> (s_103_33)).value(),
            u16::try_from(s_103_34).unwrap(),
        ));
        // D s_103_38: cast reint s_103_37 -> u8
        let s_103_38: u8 = (s_103_37.value() as u8);
        // C s_103_39: const #29s : i
        let s_103_39: i128 = 29;
        // C s_103_40: const #2s : i
        let s_103_40: i128 = 2;
        // D s_103_41: read-var u#25833:u32
        let s_103_41: u32 = fn_state.u_25833;
        // D s_103_42: cast zx s_103_41 -> bv
        let s_103_42: Bits = Bits::new(s_103_41 as u128, 32u16);
        // D s_103_43: bit-extract s_103_42 s_103_39 s_103_40
        let s_103_43: Bits = (Bits::new(
            ((s_103_42) >> (s_103_39)).value(),
            u16::try_from(s_103_40).unwrap(),
        ));
        // D s_103_44: cast reint s_103_43 -> u8
        let s_103_44: u8 = (s_103_43.value() as u8);
        // C s_103_45: const #31s : i
        let s_103_45: i128 = 31;
        // C s_103_46: const #1s : i
        let s_103_46: i128 = 1;
        // D s_103_47: read-var u#25833:u32
        let s_103_47: u32 = fn_state.u_25833;
        // D s_103_48: cast zx s_103_47 -> bv
        let s_103_48: Bits = Bits::new(s_103_47 as u128, 32u16);
        // D s_103_49: bit-extract s_103_48 s_103_45 s_103_46
        let s_103_49: Bits = (Bits::new(
            ((s_103_48) >> (s_103_45)).value(),
            u16::try_from(s_103_46).unwrap(),
        ));
        // D s_103_50: cast reint s_103_49 -> u8
        let s_103_50: bool = ((s_103_49.value()) != 0);
        // D s_103_51: call decode_eor_log_shift_aarch64_instrs_integer_logical_shiftedreg(s_103_8, s_103_14, s_103_20, s_103_26, s_103_32, s_103_38, s_103_44, s_103_50)
        let s_103_51: () = decode_eor_log_shift_aarch64_instrs_integer_logical_shiftedreg(
            state,
            tracer,
            s_103_8,
            s_103_14,
            s_103_20,
            s_103_26,
            s_103_32,
            s_103_38,
            s_103_44,
            s_103_50,
        );
        // N s_103_52: return
        return;
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_104_0: read-var merge#var.1:struct
        let s_104_0: u32 = fn_state.merge_var._1;
        // D s_104_1: write-var u#25843 <= s_104_0
        fn_state.u_25843 = s_104_0;
        // C s_104_2: const #24s : i
        let s_104_2: i128 = 24;
        // D s_104_3: read-var u#25843:u32
        let s_104_3: u32 = fn_state.u_25843;
        // D s_104_4: cast zx s_104_3 -> bv
        let s_104_4: Bits = Bits::new(s_104_3 as u128, 32u16);
        // C s_104_5: const #1s : i64
        let s_104_5: i64 = 1;
        // C s_104_6: cast zx s_104_5 -> i
        let s_104_6: i128 = (i128::try_from(s_104_5).unwrap());
        // C s_104_7: const #6s : i
        let s_104_7: i128 = 6;
        // C s_104_8: add s_104_7 s_104_6
        let s_104_8: i128 = (s_104_7 + s_104_6);
        // D s_104_9: bit-extract s_104_4 s_104_2 s_104_8
        let s_104_9: Bits = (Bits::new(
            ((s_104_4) >> (s_104_2)).value(),
            u16::try_from(s_104_8).unwrap(),
        ));
        // D s_104_10: cast reint s_104_9 -> u8
        let s_104_10: u8 = (s_104_9.value() as u8);
        // D s_104_11: cast zx s_104_10 -> bv
        let s_104_11: Bits = Bits::new(s_104_10 as u128, 7u16);
        // C s_104_12: const #42u : u8
        let s_104_12: u8 = 42;
        // C s_104_13: cast zx s_104_12 -> bv
        let s_104_13: Bits = Bits::new(s_104_12 as u128, 7u16);
        // D s_104_14: cmp-eq s_104_11 s_104_13
        let s_104_14: bool = ((s_104_11) == (s_104_13));
        // N s_104_15: branch s_104_14 b558 b105
        if s_104_14 {
            return block_558(state, tracer, fn_state);
        } else {
            return block_105(state, tracer, fn_state);
        };
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #0u : u8
        let s_105_0: bool = false;
        // D s_105_1: write-var gs#382579 <= s_105_0
        fn_state.gs_382579 = s_105_0;
        // N s_105_2: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_106_0: read-var gs#382579:u8
        let s_106_0: bool = fn_state.gs_382579;
        // N s_106_1: branch s_106_0 b557 b107
        if s_106_0 {
            return block_557(state, tracer, fn_state);
        } else {
            return block_107(state, tracer, fn_state);
        };
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #0u : u8
        let s_107_0: bool = false;
        // D s_107_1: write-var gs#382581 <= s_107_0
        fn_state.gs_382581 = s_107_0;
        // N s_107_2: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_108_0: read-var gs#382581:u8
        let s_108_0: bool = fn_state.gs_382581;
        // D s_108_1: not s_108_0
        let s_108_1: bool = !s_108_0;
        // N s_108_2: branch s_108_1 b110 b109
        if s_108_1 {
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
        // C s_109_0: const #54s : i
        let s_109_0: i128 = 54;
        // C s_109_1: const #14696u : u32
        let s_109_1: u32 = 14696;
        // N s_109_2: write-reg s_109_1 <= s_109_0
        let s_109_2: () = {
            state.write_register::<i128>(s_109_1 as isize, s_109_0);
            tracer.write_register(s_109_1 as isize, s_109_0);
        };
        // C s_109_3: const #0s : i
        let s_109_3: i128 = 0;
        // C s_109_4: const #5s : i
        let s_109_4: i128 = 5;
        // D s_109_5: read-var u#25843:u32
        let s_109_5: u32 = fn_state.u_25843;
        // D s_109_6: cast zx s_109_5 -> bv
        let s_109_6: Bits = Bits::new(s_109_5 as u128, 32u16);
        // D s_109_7: bit-extract s_109_6 s_109_3 s_109_4
        let s_109_7: Bits = (Bits::new(
            ((s_109_6) >> (s_109_3)).value(),
            u16::try_from(s_109_4).unwrap(),
        ));
        // D s_109_8: cast reint s_109_7 -> u8
        let s_109_8: u8 = (s_109_7.value() as u8);
        // C s_109_9: const #5s : i
        let s_109_9: i128 = 5;
        // C s_109_10: const #5s : i
        let s_109_10: i128 = 5;
        // D s_109_11: read-var u#25843:u32
        let s_109_11: u32 = fn_state.u_25843;
        // D s_109_12: cast zx s_109_11 -> bv
        let s_109_12: Bits = Bits::new(s_109_11 as u128, 32u16);
        // D s_109_13: bit-extract s_109_12 s_109_9 s_109_10
        let s_109_13: Bits = (Bits::new(
            ((s_109_12) >> (s_109_9)).value(),
            u16::try_from(s_109_10).unwrap(),
        ));
        // D s_109_14: cast reint s_109_13 -> u8
        let s_109_14: u8 = (s_109_13.value() as u8);
        // C s_109_15: const #10s : i
        let s_109_15: i128 = 10;
        // C s_109_16: const #6s : i
        let s_109_16: i128 = 6;
        // D s_109_17: read-var u#25843:u32
        let s_109_17: u32 = fn_state.u_25843;
        // D s_109_18: cast zx s_109_17 -> bv
        let s_109_18: Bits = Bits::new(s_109_17 as u128, 32u16);
        // D s_109_19: bit-extract s_109_18 s_109_15 s_109_16
        let s_109_19: Bits = (Bits::new(
            ((s_109_18) >> (s_109_15)).value(),
            u16::try_from(s_109_16).unwrap(),
        ));
        // D s_109_20: cast reint s_109_19 -> u8
        let s_109_20: u8 = (s_109_19.value() as u8);
        // C s_109_21: const #16s : i
        let s_109_21: i128 = 16;
        // C s_109_22: const #5s : i
        let s_109_22: i128 = 5;
        // D s_109_23: read-var u#25843:u32
        let s_109_23: u32 = fn_state.u_25843;
        // D s_109_24: cast zx s_109_23 -> bv
        let s_109_24: Bits = Bits::new(s_109_23 as u128, 32u16);
        // D s_109_25: bit-extract s_109_24 s_109_21 s_109_22
        let s_109_25: Bits = (Bits::new(
            ((s_109_24) >> (s_109_21)).value(),
            u16::try_from(s_109_22).unwrap(),
        ));
        // D s_109_26: cast reint s_109_25 -> u8
        let s_109_26: u8 = (s_109_25.value() as u8);
        // C s_109_27: const #21s : i
        let s_109_27: i128 = 21;
        // C s_109_28: const #1s : i
        let s_109_28: i128 = 1;
        // D s_109_29: read-var u#25843:u32
        let s_109_29: u32 = fn_state.u_25843;
        // D s_109_30: cast zx s_109_29 -> bv
        let s_109_30: Bits = Bits::new(s_109_29 as u128, 32u16);
        // D s_109_31: bit-extract s_109_30 s_109_27 s_109_28
        let s_109_31: Bits = (Bits::new(
            ((s_109_30) >> (s_109_27)).value(),
            u16::try_from(s_109_28).unwrap(),
        ));
        // D s_109_32: cast reint s_109_31 -> u8
        let s_109_32: bool = ((s_109_31.value()) != 0);
        // C s_109_33: const #22s : i
        let s_109_33: i128 = 22;
        // C s_109_34: const #2s : i
        let s_109_34: i128 = 2;
        // D s_109_35: read-var u#25843:u32
        let s_109_35: u32 = fn_state.u_25843;
        // D s_109_36: cast zx s_109_35 -> bv
        let s_109_36: Bits = Bits::new(s_109_35 as u128, 32u16);
        // D s_109_37: bit-extract s_109_36 s_109_33 s_109_34
        let s_109_37: Bits = (Bits::new(
            ((s_109_36) >> (s_109_33)).value(),
            u16::try_from(s_109_34).unwrap(),
        ));
        // D s_109_38: cast reint s_109_37 -> u8
        let s_109_38: u8 = (s_109_37.value() as u8);
        // C s_109_39: const #29s : i
        let s_109_39: i128 = 29;
        // C s_109_40: const #2s : i
        let s_109_40: i128 = 2;
        // D s_109_41: read-var u#25843:u32
        let s_109_41: u32 = fn_state.u_25843;
        // D s_109_42: cast zx s_109_41 -> bv
        let s_109_42: Bits = Bits::new(s_109_41 as u128, 32u16);
        // D s_109_43: bit-extract s_109_42 s_109_39 s_109_40
        let s_109_43: Bits = (Bits::new(
            ((s_109_42) >> (s_109_39)).value(),
            u16::try_from(s_109_40).unwrap(),
        ));
        // D s_109_44: cast reint s_109_43 -> u8
        let s_109_44: u8 = (s_109_43.value() as u8);
        // C s_109_45: const #31s : i
        let s_109_45: i128 = 31;
        // C s_109_46: const #1s : i
        let s_109_46: i128 = 1;
        // D s_109_47: read-var u#25843:u32
        let s_109_47: u32 = fn_state.u_25843;
        // D s_109_48: cast zx s_109_47 -> bv
        let s_109_48: Bits = Bits::new(s_109_47 as u128, 32u16);
        // D s_109_49: bit-extract s_109_48 s_109_45 s_109_46
        let s_109_49: Bits = (Bits::new(
            ((s_109_48) >> (s_109_45)).value(),
            u16::try_from(s_109_46).unwrap(),
        ));
        // D s_109_50: cast reint s_109_49 -> u8
        let s_109_50: bool = ((s_109_49.value()) != 0);
        // D s_109_51: call decode_orn_log_shift_aarch64_instrs_integer_logical_shiftedreg(s_109_8, s_109_14, s_109_20, s_109_26, s_109_32, s_109_38, s_109_44, s_109_50)
        let s_109_51: () = decode_orn_log_shift_aarch64_instrs_integer_logical_shiftedreg(
            state,
            tracer,
            s_109_8,
            s_109_14,
            s_109_20,
            s_109_26,
            s_109_32,
            s_109_38,
            s_109_44,
            s_109_50,
        );
        // N s_109_52: return
        return;
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_110_0: read-var merge#var.1:struct
        let s_110_0: u32 = fn_state.merge_var._1;
        // D s_110_1: write-var u#25853 <= s_110_0
        fn_state.u_25853 = s_110_0;
        // C s_110_2: const #24s : i
        let s_110_2: i128 = 24;
        // D s_110_3: read-var u#25853:u32
        let s_110_3: u32 = fn_state.u_25853;
        // D s_110_4: cast zx s_110_3 -> bv
        let s_110_4: Bits = Bits::new(s_110_3 as u128, 32u16);
        // C s_110_5: const #1s : i64
        let s_110_5: i64 = 1;
        // C s_110_6: cast zx s_110_5 -> i
        let s_110_6: i128 = (i128::try_from(s_110_5).unwrap());
        // C s_110_7: const #6s : i
        let s_110_7: i128 = 6;
        // C s_110_8: add s_110_7 s_110_6
        let s_110_8: i128 = (s_110_7 + s_110_6);
        // D s_110_9: bit-extract s_110_4 s_110_2 s_110_8
        let s_110_9: Bits = (Bits::new(
            ((s_110_4) >> (s_110_2)).value(),
            u16::try_from(s_110_8).unwrap(),
        ));
        // D s_110_10: cast reint s_110_9 -> u8
        let s_110_10: u8 = (s_110_9.value() as u8);
        // D s_110_11: cast zx s_110_10 -> bv
        let s_110_11: Bits = Bits::new(s_110_10 as u128, 7u16);
        // C s_110_12: const #42u : u8
        let s_110_12: u8 = 42;
        // C s_110_13: cast zx s_110_12 -> bv
        let s_110_13: Bits = Bits::new(s_110_12 as u128, 7u16);
        // D s_110_14: cmp-eq s_110_11 s_110_13
        let s_110_14: bool = ((s_110_11) == (s_110_13));
        // N s_110_15: branch s_110_14 b556 b111
        if s_110_14 {
            return block_556(state, tracer, fn_state);
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
        // D s_111_1: write-var gs#382605 <= s_111_0
        fn_state.gs_382605 = s_111_0;
        // N s_111_2: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_112_0: read-var gs#382605:u8
        let s_112_0: bool = fn_state.gs_382605;
        // N s_112_1: branch s_112_0 b555 b113
        if s_112_0 {
            return block_555(state, tracer, fn_state);
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
        // D s_113_1: write-var gs#382607 <= s_113_0
        fn_state.gs_382607 = s_113_0;
        // N s_113_2: jump b114
        return block_114(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_114_0: read-var gs#382607:u8
        let s_114_0: bool = fn_state.gs_382607;
        // D s_114_1: not s_114_0
        let s_114_1: bool = !s_114_0;
        // N s_114_2: branch s_114_1 b116 b115
        if s_114_1 {
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
        // C s_115_0: const #55s : i
        let s_115_0: i128 = 55;
        // C s_115_1: const #14696u : u32
        let s_115_1: u32 = 14696;
        // N s_115_2: write-reg s_115_1 <= s_115_0
        let s_115_2: () = {
            state.write_register::<i128>(s_115_1 as isize, s_115_0);
            tracer.write_register(s_115_1 as isize, s_115_0);
        };
        // C s_115_3: const #0s : i
        let s_115_3: i128 = 0;
        // C s_115_4: const #5s : i
        let s_115_4: i128 = 5;
        // D s_115_5: read-var u#25853:u32
        let s_115_5: u32 = fn_state.u_25853;
        // D s_115_6: cast zx s_115_5 -> bv
        let s_115_6: Bits = Bits::new(s_115_5 as u128, 32u16);
        // D s_115_7: bit-extract s_115_6 s_115_3 s_115_4
        let s_115_7: Bits = (Bits::new(
            ((s_115_6) >> (s_115_3)).value(),
            u16::try_from(s_115_4).unwrap(),
        ));
        // D s_115_8: cast reint s_115_7 -> u8
        let s_115_8: u8 = (s_115_7.value() as u8);
        // C s_115_9: const #5s : i
        let s_115_9: i128 = 5;
        // C s_115_10: const #5s : i
        let s_115_10: i128 = 5;
        // D s_115_11: read-var u#25853:u32
        let s_115_11: u32 = fn_state.u_25853;
        // D s_115_12: cast zx s_115_11 -> bv
        let s_115_12: Bits = Bits::new(s_115_11 as u128, 32u16);
        // D s_115_13: bit-extract s_115_12 s_115_9 s_115_10
        let s_115_13: Bits = (Bits::new(
            ((s_115_12) >> (s_115_9)).value(),
            u16::try_from(s_115_10).unwrap(),
        ));
        // D s_115_14: cast reint s_115_13 -> u8
        let s_115_14: u8 = (s_115_13.value() as u8);
        // C s_115_15: const #10s : i
        let s_115_15: i128 = 10;
        // C s_115_16: const #6s : i
        let s_115_16: i128 = 6;
        // D s_115_17: read-var u#25853:u32
        let s_115_17: u32 = fn_state.u_25853;
        // D s_115_18: cast zx s_115_17 -> bv
        let s_115_18: Bits = Bits::new(s_115_17 as u128, 32u16);
        // D s_115_19: bit-extract s_115_18 s_115_15 s_115_16
        let s_115_19: Bits = (Bits::new(
            ((s_115_18) >> (s_115_15)).value(),
            u16::try_from(s_115_16).unwrap(),
        ));
        // D s_115_20: cast reint s_115_19 -> u8
        let s_115_20: u8 = (s_115_19.value() as u8);
        // C s_115_21: const #16s : i
        let s_115_21: i128 = 16;
        // C s_115_22: const #5s : i
        let s_115_22: i128 = 5;
        // D s_115_23: read-var u#25853:u32
        let s_115_23: u32 = fn_state.u_25853;
        // D s_115_24: cast zx s_115_23 -> bv
        let s_115_24: Bits = Bits::new(s_115_23 as u128, 32u16);
        // D s_115_25: bit-extract s_115_24 s_115_21 s_115_22
        let s_115_25: Bits = (Bits::new(
            ((s_115_24) >> (s_115_21)).value(),
            u16::try_from(s_115_22).unwrap(),
        ));
        // D s_115_26: cast reint s_115_25 -> u8
        let s_115_26: u8 = (s_115_25.value() as u8);
        // C s_115_27: const #21s : i
        let s_115_27: i128 = 21;
        // C s_115_28: const #1s : i
        let s_115_28: i128 = 1;
        // D s_115_29: read-var u#25853:u32
        let s_115_29: u32 = fn_state.u_25853;
        // D s_115_30: cast zx s_115_29 -> bv
        let s_115_30: Bits = Bits::new(s_115_29 as u128, 32u16);
        // D s_115_31: bit-extract s_115_30 s_115_27 s_115_28
        let s_115_31: Bits = (Bits::new(
            ((s_115_30) >> (s_115_27)).value(),
            u16::try_from(s_115_28).unwrap(),
        ));
        // D s_115_32: cast reint s_115_31 -> u8
        let s_115_32: bool = ((s_115_31.value()) != 0);
        // C s_115_33: const #22s : i
        let s_115_33: i128 = 22;
        // C s_115_34: const #2s : i
        let s_115_34: i128 = 2;
        // D s_115_35: read-var u#25853:u32
        let s_115_35: u32 = fn_state.u_25853;
        // D s_115_36: cast zx s_115_35 -> bv
        let s_115_36: Bits = Bits::new(s_115_35 as u128, 32u16);
        // D s_115_37: bit-extract s_115_36 s_115_33 s_115_34
        let s_115_37: Bits = (Bits::new(
            ((s_115_36) >> (s_115_33)).value(),
            u16::try_from(s_115_34).unwrap(),
        ));
        // D s_115_38: cast reint s_115_37 -> u8
        let s_115_38: u8 = (s_115_37.value() as u8);
        // C s_115_39: const #29s : i
        let s_115_39: i128 = 29;
        // C s_115_40: const #2s : i
        let s_115_40: i128 = 2;
        // D s_115_41: read-var u#25853:u32
        let s_115_41: u32 = fn_state.u_25853;
        // D s_115_42: cast zx s_115_41 -> bv
        let s_115_42: Bits = Bits::new(s_115_41 as u128, 32u16);
        // D s_115_43: bit-extract s_115_42 s_115_39 s_115_40
        let s_115_43: Bits = (Bits::new(
            ((s_115_42) >> (s_115_39)).value(),
            u16::try_from(s_115_40).unwrap(),
        ));
        // D s_115_44: cast reint s_115_43 -> u8
        let s_115_44: u8 = (s_115_43.value() as u8);
        // C s_115_45: const #31s : i
        let s_115_45: i128 = 31;
        // C s_115_46: const #1s : i
        let s_115_46: i128 = 1;
        // D s_115_47: read-var u#25853:u32
        let s_115_47: u32 = fn_state.u_25853;
        // D s_115_48: cast zx s_115_47 -> bv
        let s_115_48: Bits = Bits::new(s_115_47 as u128, 32u16);
        // D s_115_49: bit-extract s_115_48 s_115_45 s_115_46
        let s_115_49: Bits = (Bits::new(
            ((s_115_48) >> (s_115_45)).value(),
            u16::try_from(s_115_46).unwrap(),
        ));
        // D s_115_50: cast reint s_115_49 -> u8
        let s_115_50: bool = ((s_115_49.value()) != 0);
        // D s_115_51: call decode_orr_log_shift_aarch64_instrs_integer_logical_shiftedreg(s_115_8, s_115_14, s_115_20, s_115_26, s_115_32, s_115_38, s_115_44, s_115_50)
        let s_115_51: () = decode_orr_log_shift_aarch64_instrs_integer_logical_shiftedreg(
            state,
            tracer,
            s_115_8,
            s_115_14,
            s_115_20,
            s_115_26,
            s_115_32,
            s_115_38,
            s_115_44,
            s_115_50,
        );
        // N s_115_52: return
        return;
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_116_0: read-var merge#var.1:struct
        let s_116_0: u32 = fn_state.merge_var._1;
        // D s_116_1: write-var u#25863 <= s_116_0
        fn_state.u_25863 = s_116_0;
        // C s_116_2: const #21s : i
        let s_116_2: i128 = 21;
        // D s_116_3: read-var u#25863:u32
        let s_116_3: u32 = fn_state.u_25863;
        // D s_116_4: cast zx s_116_3 -> bv
        let s_116_4: Bits = Bits::new(s_116_3 as u128, 32u16);
        // C s_116_5: const #1s : i64
        let s_116_5: i64 = 1;
        // C s_116_6: cast zx s_116_5 -> i
        let s_116_6: i128 = (i128::try_from(s_116_5).unwrap());
        // C s_116_7: const #9s : i
        let s_116_7: i128 = 9;
        // C s_116_8: add s_116_7 s_116_6
        let s_116_8: i128 = (s_116_7 + s_116_6);
        // D s_116_9: bit-extract s_116_4 s_116_2 s_116_8
        let s_116_9: Bits = (Bits::new(
            ((s_116_4) >> (s_116_2)).value(),
            u16::try_from(s_116_8).unwrap(),
        ));
        // D s_116_10: cast reint s_116_9 -> u10
        let s_116_10: u16 = (s_116_9.value() as u16);
        // D s_116_11: cast zx s_116_10 -> bv
        let s_116_11: Bits = Bits::new(s_116_10 as u128, 10u16);
        // C s_116_12: const #214u : u10
        let s_116_12: u16 = 214;
        // C s_116_13: cast zx s_116_12 -> bv
        let s_116_13: Bits = Bits::new(s_116_12 as u128, 10u16);
        // D s_116_14: cmp-eq s_116_11 s_116_13
        let s_116_14: bool = ((s_116_11) == (s_116_13));
        // N s_116_15: branch s_116_14 b554 b117
        if s_116_14 {
            return block_554(state, tracer, fn_state);
        } else {
            return block_117(state, tracer, fn_state);
        };
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_117_0: const #0u : u8
        let s_117_0: bool = false;
        // D s_117_1: write-var gs#382631 <= s_117_0
        fn_state.gs_382631 = s_117_0;
        // N s_117_2: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_118_0: read-var gs#382631:u8
        let s_118_0: bool = fn_state.gs_382631;
        // N s_118_1: branch s_118_0 b553 b119
        if s_118_0 {
            return block_553(state, tracer, fn_state);
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
        // D s_119_1: write-var gs#382633 <= s_119_0
        fn_state.gs_382633 = s_119_0;
        // N s_119_2: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_120_0: read-var gs#382633:u8
        let s_120_0: bool = fn_state.gs_382633;
        // D s_120_1: not s_120_0
        let s_120_1: bool = !s_120_0;
        // N s_120_2: branch s_120_1 b122 b121
        if s_120_1 {
            return block_122(state, tracer, fn_state);
        } else {
            return block_121(state, tracer, fn_state);
        };
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_121_0: const #56s : i
        let s_121_0: i128 = 56;
        // C s_121_1: const #14696u : u32
        let s_121_1: u32 = 14696;
        // N s_121_2: write-reg s_121_1 <= s_121_0
        let s_121_2: () = {
            state.write_register::<i128>(s_121_1 as isize, s_121_0);
            tracer.write_register(s_121_1 as isize, s_121_0);
        };
        // C s_121_3: const #0s : i
        let s_121_3: i128 = 0;
        // C s_121_4: const #5s : i
        let s_121_4: i128 = 5;
        // D s_121_5: read-var u#25863:u32
        let s_121_5: u32 = fn_state.u_25863;
        // D s_121_6: cast zx s_121_5 -> bv
        let s_121_6: Bits = Bits::new(s_121_5 as u128, 32u16);
        // D s_121_7: bit-extract s_121_6 s_121_3 s_121_4
        let s_121_7: Bits = (Bits::new(
            ((s_121_6) >> (s_121_3)).value(),
            u16::try_from(s_121_4).unwrap(),
        ));
        // D s_121_8: cast reint s_121_7 -> u8
        let s_121_8: u8 = (s_121_7.value() as u8);
        // C s_121_9: const #5s : i
        let s_121_9: i128 = 5;
        // C s_121_10: const #5s : i
        let s_121_10: i128 = 5;
        // D s_121_11: read-var u#25863:u32
        let s_121_11: u32 = fn_state.u_25863;
        // D s_121_12: cast zx s_121_11 -> bv
        let s_121_12: Bits = Bits::new(s_121_11 as u128, 32u16);
        // D s_121_13: bit-extract s_121_12 s_121_9 s_121_10
        let s_121_13: Bits = (Bits::new(
            ((s_121_12) >> (s_121_9)).value(),
            u16::try_from(s_121_10).unwrap(),
        ));
        // D s_121_14: cast reint s_121_13 -> u8
        let s_121_14: u8 = (s_121_13.value() as u8);
        // C s_121_15: const #10s : i
        let s_121_15: i128 = 10;
        // C s_121_16: const #2s : i
        let s_121_16: i128 = 2;
        // D s_121_17: read-var u#25863:u32
        let s_121_17: u32 = fn_state.u_25863;
        // D s_121_18: cast zx s_121_17 -> bv
        let s_121_18: Bits = Bits::new(s_121_17 as u128, 32u16);
        // D s_121_19: bit-extract s_121_18 s_121_15 s_121_16
        let s_121_19: Bits = (Bits::new(
            ((s_121_18) >> (s_121_15)).value(),
            u16::try_from(s_121_16).unwrap(),
        ));
        // D s_121_20: cast reint s_121_19 -> u8
        let s_121_20: u8 = (s_121_19.value() as u8);
        // C s_121_21: const #16s : i
        let s_121_21: i128 = 16;
        // C s_121_22: const #5s : i
        let s_121_22: i128 = 5;
        // D s_121_23: read-var u#25863:u32
        let s_121_23: u32 = fn_state.u_25863;
        // D s_121_24: cast zx s_121_23 -> bv
        let s_121_24: Bits = Bits::new(s_121_23 as u128, 32u16);
        // D s_121_25: bit-extract s_121_24 s_121_21 s_121_22
        let s_121_25: Bits = (Bits::new(
            ((s_121_24) >> (s_121_21)).value(),
            u16::try_from(s_121_22).unwrap(),
        ));
        // D s_121_26: cast reint s_121_25 -> u8
        let s_121_26: u8 = (s_121_25.value() as u8);
        // C s_121_27: const #31s : i
        let s_121_27: i128 = 31;
        // C s_121_28: const #1s : i
        let s_121_28: i128 = 1;
        // D s_121_29: read-var u#25863:u32
        let s_121_29: u32 = fn_state.u_25863;
        // D s_121_30: cast zx s_121_29 -> bv
        let s_121_30: Bits = Bits::new(s_121_29 as u128, 32u16);
        // D s_121_31: bit-extract s_121_30 s_121_27 s_121_28
        let s_121_31: Bits = (Bits::new(
            ((s_121_30) >> (s_121_27)).value(),
            u16::try_from(s_121_28).unwrap(),
        ));
        // D s_121_32: cast reint s_121_31 -> u8
        let s_121_32: bool = ((s_121_31.value()) != 0);
        // D s_121_33: call decode_asrv_aarch64_instrs_integer_shift_variable(s_121_8, s_121_14, s_121_20, s_121_26, s_121_32)
        let s_121_33: () = decode_asrv_aarch64_instrs_integer_shift_variable(
            state,
            tracer,
            s_121_8,
            s_121_14,
            s_121_20,
            s_121_26,
            s_121_32,
        );
        // N s_121_34: return
        return;
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_122_0: read-var merge#var.1:struct
        let s_122_0: u32 = fn_state.merge_var._1;
        // D s_122_1: write-var u#25869 <= s_122_0
        fn_state.u_25869 = s_122_0;
        // C s_122_2: const #21s : i
        let s_122_2: i128 = 21;
        // D s_122_3: read-var u#25869:u32
        let s_122_3: u32 = fn_state.u_25869;
        // D s_122_4: cast zx s_122_3 -> bv
        let s_122_4: Bits = Bits::new(s_122_3 as u128, 32u16);
        // C s_122_5: const #1s : i64
        let s_122_5: i64 = 1;
        // C s_122_6: cast zx s_122_5 -> i
        let s_122_6: i128 = (i128::try_from(s_122_5).unwrap());
        // C s_122_7: const #9s : i
        let s_122_7: i128 = 9;
        // C s_122_8: add s_122_7 s_122_6
        let s_122_8: i128 = (s_122_7 + s_122_6);
        // D s_122_9: bit-extract s_122_4 s_122_2 s_122_8
        let s_122_9: Bits = (Bits::new(
            ((s_122_4) >> (s_122_2)).value(),
            u16::try_from(s_122_8).unwrap(),
        ));
        // D s_122_10: cast reint s_122_9 -> u10
        let s_122_10: u16 = (s_122_9.value() as u16);
        // D s_122_11: cast zx s_122_10 -> bv
        let s_122_11: Bits = Bits::new(s_122_10 as u128, 10u16);
        // C s_122_12: const #214u : u10
        let s_122_12: u16 = 214;
        // C s_122_13: cast zx s_122_12 -> bv
        let s_122_13: Bits = Bits::new(s_122_12 as u128, 10u16);
        // D s_122_14: cmp-eq s_122_11 s_122_13
        let s_122_14: bool = ((s_122_11) == (s_122_13));
        // N s_122_15: branch s_122_14 b552 b123
        if s_122_14 {
            return block_552(state, tracer, fn_state);
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
        // D s_123_1: write-var gs#382651 <= s_123_0
        fn_state.gs_382651 = s_123_0;
        // N s_123_2: jump b124
        return block_124(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_124_0: read-var gs#382651:u8
        let s_124_0: bool = fn_state.gs_382651;
        // N s_124_1: branch s_124_0 b551 b125
        if s_124_0 {
            return block_551(state, tracer, fn_state);
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
        // D s_125_1: write-var gs#382653 <= s_125_0
        fn_state.gs_382653 = s_125_0;
        // N s_125_2: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_126_0: read-var gs#382653:u8
        let s_126_0: bool = fn_state.gs_382653;
        // D s_126_1: not s_126_0
        let s_126_1: bool = !s_126_0;
        // N s_126_2: branch s_126_1 b128 b127
        if s_126_1 {
            return block_128(state, tracer, fn_state);
        } else {
            return block_127(state, tracer, fn_state);
        };
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_127_0: const #57s : i
        let s_127_0: i128 = 57;
        // C s_127_1: const #14696u : u32
        let s_127_1: u32 = 14696;
        // N s_127_2: write-reg s_127_1 <= s_127_0
        let s_127_2: () = {
            state.write_register::<i128>(s_127_1 as isize, s_127_0);
            tracer.write_register(s_127_1 as isize, s_127_0);
        };
        // C s_127_3: const #0s : i
        let s_127_3: i128 = 0;
        // C s_127_4: const #5s : i
        let s_127_4: i128 = 5;
        // D s_127_5: read-var u#25869:u32
        let s_127_5: u32 = fn_state.u_25869;
        // D s_127_6: cast zx s_127_5 -> bv
        let s_127_6: Bits = Bits::new(s_127_5 as u128, 32u16);
        // D s_127_7: bit-extract s_127_6 s_127_3 s_127_4
        let s_127_7: Bits = (Bits::new(
            ((s_127_6) >> (s_127_3)).value(),
            u16::try_from(s_127_4).unwrap(),
        ));
        // D s_127_8: cast reint s_127_7 -> u8
        let s_127_8: u8 = (s_127_7.value() as u8);
        // C s_127_9: const #5s : i
        let s_127_9: i128 = 5;
        // C s_127_10: const #5s : i
        let s_127_10: i128 = 5;
        // D s_127_11: read-var u#25869:u32
        let s_127_11: u32 = fn_state.u_25869;
        // D s_127_12: cast zx s_127_11 -> bv
        let s_127_12: Bits = Bits::new(s_127_11 as u128, 32u16);
        // D s_127_13: bit-extract s_127_12 s_127_9 s_127_10
        let s_127_13: Bits = (Bits::new(
            ((s_127_12) >> (s_127_9)).value(),
            u16::try_from(s_127_10).unwrap(),
        ));
        // D s_127_14: cast reint s_127_13 -> u8
        let s_127_14: u8 = (s_127_13.value() as u8);
        // C s_127_15: const #10s : i
        let s_127_15: i128 = 10;
        // C s_127_16: const #2s : i
        let s_127_16: i128 = 2;
        // D s_127_17: read-var u#25869:u32
        let s_127_17: u32 = fn_state.u_25869;
        // D s_127_18: cast zx s_127_17 -> bv
        let s_127_18: Bits = Bits::new(s_127_17 as u128, 32u16);
        // D s_127_19: bit-extract s_127_18 s_127_15 s_127_16
        let s_127_19: Bits = (Bits::new(
            ((s_127_18) >> (s_127_15)).value(),
            u16::try_from(s_127_16).unwrap(),
        ));
        // D s_127_20: cast reint s_127_19 -> u8
        let s_127_20: u8 = (s_127_19.value() as u8);
        // C s_127_21: const #16s : i
        let s_127_21: i128 = 16;
        // C s_127_22: const #5s : i
        let s_127_22: i128 = 5;
        // D s_127_23: read-var u#25869:u32
        let s_127_23: u32 = fn_state.u_25869;
        // D s_127_24: cast zx s_127_23 -> bv
        let s_127_24: Bits = Bits::new(s_127_23 as u128, 32u16);
        // D s_127_25: bit-extract s_127_24 s_127_21 s_127_22
        let s_127_25: Bits = (Bits::new(
            ((s_127_24) >> (s_127_21)).value(),
            u16::try_from(s_127_22).unwrap(),
        ));
        // D s_127_26: cast reint s_127_25 -> u8
        let s_127_26: u8 = (s_127_25.value() as u8);
        // C s_127_27: const #31s : i
        let s_127_27: i128 = 31;
        // C s_127_28: const #1s : i
        let s_127_28: i128 = 1;
        // D s_127_29: read-var u#25869:u32
        let s_127_29: u32 = fn_state.u_25869;
        // D s_127_30: cast zx s_127_29 -> bv
        let s_127_30: Bits = Bits::new(s_127_29 as u128, 32u16);
        // D s_127_31: bit-extract s_127_30 s_127_27 s_127_28
        let s_127_31: Bits = (Bits::new(
            ((s_127_30) >> (s_127_27)).value(),
            u16::try_from(s_127_28).unwrap(),
        ));
        // D s_127_32: cast reint s_127_31 -> u8
        let s_127_32: bool = ((s_127_31.value()) != 0);
        // D s_127_33: call decode_lslv_aarch64_instrs_integer_shift_variable(s_127_8, s_127_14, s_127_20, s_127_26, s_127_32)
        let s_127_33: () = decode_lslv_aarch64_instrs_integer_shift_variable(
            state,
            tracer,
            s_127_8,
            s_127_14,
            s_127_20,
            s_127_26,
            s_127_32,
        );
        // N s_127_34: return
        return;
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_128_0: read-var merge#var.1:struct
        let s_128_0: u32 = fn_state.merge_var._1;
        // D s_128_1: write-var u#25876 <= s_128_0
        fn_state.u_25876 = s_128_0;
        // C s_128_2: const #21s : i
        let s_128_2: i128 = 21;
        // D s_128_3: read-var u#25876:u32
        let s_128_3: u32 = fn_state.u_25876;
        // D s_128_4: cast zx s_128_3 -> bv
        let s_128_4: Bits = Bits::new(s_128_3 as u128, 32u16);
        // C s_128_5: const #1s : i64
        let s_128_5: i64 = 1;
        // C s_128_6: cast zx s_128_5 -> i
        let s_128_6: i128 = (i128::try_from(s_128_5).unwrap());
        // C s_128_7: const #9s : i
        let s_128_7: i128 = 9;
        // C s_128_8: add s_128_7 s_128_6
        let s_128_8: i128 = (s_128_7 + s_128_6);
        // D s_128_9: bit-extract s_128_4 s_128_2 s_128_8
        let s_128_9: Bits = (Bits::new(
            ((s_128_4) >> (s_128_2)).value(),
            u16::try_from(s_128_8).unwrap(),
        ));
        // D s_128_10: cast reint s_128_9 -> u10
        let s_128_10: u16 = (s_128_9.value() as u16);
        // D s_128_11: cast zx s_128_10 -> bv
        let s_128_11: Bits = Bits::new(s_128_10 as u128, 10u16);
        // C s_128_12: const #214u : u10
        let s_128_12: u16 = 214;
        // C s_128_13: cast zx s_128_12 -> bv
        let s_128_13: Bits = Bits::new(s_128_12 as u128, 10u16);
        // D s_128_14: cmp-eq s_128_11 s_128_13
        let s_128_14: bool = ((s_128_11) == (s_128_13));
        // N s_128_15: branch s_128_14 b550 b129
        if s_128_14 {
            return block_550(state, tracer, fn_state);
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
        // D s_129_1: write-var gs#382671 <= s_129_0
        fn_state.gs_382671 = s_129_0;
        // N s_129_2: jump b130
        return block_130(state, tracer, fn_state);
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_130_0: read-var gs#382671:u8
        let s_130_0: bool = fn_state.gs_382671;
        // N s_130_1: branch s_130_0 b549 b131
        if s_130_0 {
            return block_549(state, tracer, fn_state);
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
        // D s_131_1: write-var gs#382673 <= s_131_0
        fn_state.gs_382673 = s_131_0;
        // N s_131_2: jump b132
        return block_132(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_132_0: read-var gs#382673:u8
        let s_132_0: bool = fn_state.gs_382673;
        // D s_132_1: not s_132_0
        let s_132_1: bool = !s_132_0;
        // N s_132_2: branch s_132_1 b134 b133
        if s_132_1 {
            return block_134(state, tracer, fn_state);
        } else {
            return block_133(state, tracer, fn_state);
        };
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_133_0: const #58s : i
        let s_133_0: i128 = 58;
        // C s_133_1: const #14696u : u32
        let s_133_1: u32 = 14696;
        // N s_133_2: write-reg s_133_1 <= s_133_0
        let s_133_2: () = {
            state.write_register::<i128>(s_133_1 as isize, s_133_0);
            tracer.write_register(s_133_1 as isize, s_133_0);
        };
        // C s_133_3: const #0s : i
        let s_133_3: i128 = 0;
        // C s_133_4: const #5s : i
        let s_133_4: i128 = 5;
        // D s_133_5: read-var u#25876:u32
        let s_133_5: u32 = fn_state.u_25876;
        // D s_133_6: cast zx s_133_5 -> bv
        let s_133_6: Bits = Bits::new(s_133_5 as u128, 32u16);
        // D s_133_7: bit-extract s_133_6 s_133_3 s_133_4
        let s_133_7: Bits = (Bits::new(
            ((s_133_6) >> (s_133_3)).value(),
            u16::try_from(s_133_4).unwrap(),
        ));
        // D s_133_8: cast reint s_133_7 -> u8
        let s_133_8: u8 = (s_133_7.value() as u8);
        // C s_133_9: const #5s : i
        let s_133_9: i128 = 5;
        // C s_133_10: const #5s : i
        let s_133_10: i128 = 5;
        // D s_133_11: read-var u#25876:u32
        let s_133_11: u32 = fn_state.u_25876;
        // D s_133_12: cast zx s_133_11 -> bv
        let s_133_12: Bits = Bits::new(s_133_11 as u128, 32u16);
        // D s_133_13: bit-extract s_133_12 s_133_9 s_133_10
        let s_133_13: Bits = (Bits::new(
            ((s_133_12) >> (s_133_9)).value(),
            u16::try_from(s_133_10).unwrap(),
        ));
        // D s_133_14: cast reint s_133_13 -> u8
        let s_133_14: u8 = (s_133_13.value() as u8);
        // C s_133_15: const #10s : i
        let s_133_15: i128 = 10;
        // C s_133_16: const #2s : i
        let s_133_16: i128 = 2;
        // D s_133_17: read-var u#25876:u32
        let s_133_17: u32 = fn_state.u_25876;
        // D s_133_18: cast zx s_133_17 -> bv
        let s_133_18: Bits = Bits::new(s_133_17 as u128, 32u16);
        // D s_133_19: bit-extract s_133_18 s_133_15 s_133_16
        let s_133_19: Bits = (Bits::new(
            ((s_133_18) >> (s_133_15)).value(),
            u16::try_from(s_133_16).unwrap(),
        ));
        // D s_133_20: cast reint s_133_19 -> u8
        let s_133_20: u8 = (s_133_19.value() as u8);
        // C s_133_21: const #16s : i
        let s_133_21: i128 = 16;
        // C s_133_22: const #5s : i
        let s_133_22: i128 = 5;
        // D s_133_23: read-var u#25876:u32
        let s_133_23: u32 = fn_state.u_25876;
        // D s_133_24: cast zx s_133_23 -> bv
        let s_133_24: Bits = Bits::new(s_133_23 as u128, 32u16);
        // D s_133_25: bit-extract s_133_24 s_133_21 s_133_22
        let s_133_25: Bits = (Bits::new(
            ((s_133_24) >> (s_133_21)).value(),
            u16::try_from(s_133_22).unwrap(),
        ));
        // D s_133_26: cast reint s_133_25 -> u8
        let s_133_26: u8 = (s_133_25.value() as u8);
        // C s_133_27: const #31s : i
        let s_133_27: i128 = 31;
        // C s_133_28: const #1s : i
        let s_133_28: i128 = 1;
        // D s_133_29: read-var u#25876:u32
        let s_133_29: u32 = fn_state.u_25876;
        // D s_133_30: cast zx s_133_29 -> bv
        let s_133_30: Bits = Bits::new(s_133_29 as u128, 32u16);
        // D s_133_31: bit-extract s_133_30 s_133_27 s_133_28
        let s_133_31: Bits = (Bits::new(
            ((s_133_30) >> (s_133_27)).value(),
            u16::try_from(s_133_28).unwrap(),
        ));
        // D s_133_32: cast reint s_133_31 -> u8
        let s_133_32: bool = ((s_133_31.value()) != 0);
        // D s_133_33: call decode_lsrv_aarch64_instrs_integer_shift_variable(s_133_8, s_133_14, s_133_20, s_133_26, s_133_32)
        let s_133_33: () = decode_lsrv_aarch64_instrs_integer_shift_variable(
            state,
            tracer,
            s_133_8,
            s_133_14,
            s_133_20,
            s_133_26,
            s_133_32,
        );
        // N s_133_34: return
        return;
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_134_0: read-var merge#var.1:struct
        let s_134_0: u32 = fn_state.merge_var._1;
        // D s_134_1: write-var u#25883 <= s_134_0
        fn_state.u_25883 = s_134_0;
        // C s_134_2: const #21s : i
        let s_134_2: i128 = 21;
        // D s_134_3: read-var u#25883:u32
        let s_134_3: u32 = fn_state.u_25883;
        // D s_134_4: cast zx s_134_3 -> bv
        let s_134_4: Bits = Bits::new(s_134_3 as u128, 32u16);
        // C s_134_5: const #1s : i64
        let s_134_5: i64 = 1;
        // C s_134_6: cast zx s_134_5 -> i
        let s_134_6: i128 = (i128::try_from(s_134_5).unwrap());
        // C s_134_7: const #9s : i
        let s_134_7: i128 = 9;
        // C s_134_8: add s_134_7 s_134_6
        let s_134_8: i128 = (s_134_7 + s_134_6);
        // D s_134_9: bit-extract s_134_4 s_134_2 s_134_8
        let s_134_9: Bits = (Bits::new(
            ((s_134_4) >> (s_134_2)).value(),
            u16::try_from(s_134_8).unwrap(),
        ));
        // D s_134_10: cast reint s_134_9 -> u10
        let s_134_10: u16 = (s_134_9.value() as u16);
        // D s_134_11: cast zx s_134_10 -> bv
        let s_134_11: Bits = Bits::new(s_134_10 as u128, 10u16);
        // C s_134_12: const #214u : u10
        let s_134_12: u16 = 214;
        // C s_134_13: cast zx s_134_12 -> bv
        let s_134_13: Bits = Bits::new(s_134_12 as u128, 10u16);
        // D s_134_14: cmp-eq s_134_11 s_134_13
        let s_134_14: bool = ((s_134_11) == (s_134_13));
        // N s_134_15: branch s_134_14 b548 b135
        if s_134_14 {
            return block_548(state, tracer, fn_state);
        } else {
            return block_135(state, tracer, fn_state);
        };
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_135_0: const #0u : u8
        let s_135_0: bool = false;
        // D s_135_1: write-var gs#382691 <= s_135_0
        fn_state.gs_382691 = s_135_0;
        // N s_135_2: jump b136
        return block_136(state, tracer, fn_state);
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_136_0: read-var gs#382691:u8
        let s_136_0: bool = fn_state.gs_382691;
        // N s_136_1: branch s_136_0 b547 b137
        if s_136_0 {
            return block_547(state, tracer, fn_state);
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
        // D s_137_1: write-var gs#382693 <= s_137_0
        fn_state.gs_382693 = s_137_0;
        // N s_137_2: jump b138
        return block_138(state, tracer, fn_state);
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_138_0: read-var gs#382693:u8
        let s_138_0: bool = fn_state.gs_382693;
        // D s_138_1: not s_138_0
        let s_138_1: bool = !s_138_0;
        // N s_138_2: branch s_138_1 b140 b139
        if s_138_1 {
            return block_140(state, tracer, fn_state);
        } else {
            return block_139(state, tracer, fn_state);
        };
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_139_0: const #59s : i
        let s_139_0: i128 = 59;
        // C s_139_1: const #14696u : u32
        let s_139_1: u32 = 14696;
        // N s_139_2: write-reg s_139_1 <= s_139_0
        let s_139_2: () = {
            state.write_register::<i128>(s_139_1 as isize, s_139_0);
            tracer.write_register(s_139_1 as isize, s_139_0);
        };
        // C s_139_3: const #0s : i
        let s_139_3: i128 = 0;
        // C s_139_4: const #5s : i
        let s_139_4: i128 = 5;
        // D s_139_5: read-var u#25883:u32
        let s_139_5: u32 = fn_state.u_25883;
        // D s_139_6: cast zx s_139_5 -> bv
        let s_139_6: Bits = Bits::new(s_139_5 as u128, 32u16);
        // D s_139_7: bit-extract s_139_6 s_139_3 s_139_4
        let s_139_7: Bits = (Bits::new(
            ((s_139_6) >> (s_139_3)).value(),
            u16::try_from(s_139_4).unwrap(),
        ));
        // D s_139_8: cast reint s_139_7 -> u8
        let s_139_8: u8 = (s_139_7.value() as u8);
        // C s_139_9: const #5s : i
        let s_139_9: i128 = 5;
        // C s_139_10: const #5s : i
        let s_139_10: i128 = 5;
        // D s_139_11: read-var u#25883:u32
        let s_139_11: u32 = fn_state.u_25883;
        // D s_139_12: cast zx s_139_11 -> bv
        let s_139_12: Bits = Bits::new(s_139_11 as u128, 32u16);
        // D s_139_13: bit-extract s_139_12 s_139_9 s_139_10
        let s_139_13: Bits = (Bits::new(
            ((s_139_12) >> (s_139_9)).value(),
            u16::try_from(s_139_10).unwrap(),
        ));
        // D s_139_14: cast reint s_139_13 -> u8
        let s_139_14: u8 = (s_139_13.value() as u8);
        // C s_139_15: const #10s : i
        let s_139_15: i128 = 10;
        // C s_139_16: const #2s : i
        let s_139_16: i128 = 2;
        // D s_139_17: read-var u#25883:u32
        let s_139_17: u32 = fn_state.u_25883;
        // D s_139_18: cast zx s_139_17 -> bv
        let s_139_18: Bits = Bits::new(s_139_17 as u128, 32u16);
        // D s_139_19: bit-extract s_139_18 s_139_15 s_139_16
        let s_139_19: Bits = (Bits::new(
            ((s_139_18) >> (s_139_15)).value(),
            u16::try_from(s_139_16).unwrap(),
        ));
        // D s_139_20: cast reint s_139_19 -> u8
        let s_139_20: u8 = (s_139_19.value() as u8);
        // C s_139_21: const #16s : i
        let s_139_21: i128 = 16;
        // C s_139_22: const #5s : i
        let s_139_22: i128 = 5;
        // D s_139_23: read-var u#25883:u32
        let s_139_23: u32 = fn_state.u_25883;
        // D s_139_24: cast zx s_139_23 -> bv
        let s_139_24: Bits = Bits::new(s_139_23 as u128, 32u16);
        // D s_139_25: bit-extract s_139_24 s_139_21 s_139_22
        let s_139_25: Bits = (Bits::new(
            ((s_139_24) >> (s_139_21)).value(),
            u16::try_from(s_139_22).unwrap(),
        ));
        // D s_139_26: cast reint s_139_25 -> u8
        let s_139_26: u8 = (s_139_25.value() as u8);
        // C s_139_27: const #31s : i
        let s_139_27: i128 = 31;
        // C s_139_28: const #1s : i
        let s_139_28: i128 = 1;
        // D s_139_29: read-var u#25883:u32
        let s_139_29: u32 = fn_state.u_25883;
        // D s_139_30: cast zx s_139_29 -> bv
        let s_139_30: Bits = Bits::new(s_139_29 as u128, 32u16);
        // D s_139_31: bit-extract s_139_30 s_139_27 s_139_28
        let s_139_31: Bits = (Bits::new(
            ((s_139_30) >> (s_139_27)).value(),
            u16::try_from(s_139_28).unwrap(),
        ));
        // D s_139_32: cast reint s_139_31 -> u8
        let s_139_32: bool = ((s_139_31.value()) != 0);
        // D s_139_33: call decode_rorv_aarch64_instrs_integer_shift_variable(s_139_8, s_139_14, s_139_20, s_139_26, s_139_32)
        let s_139_33: () = decode_rorv_aarch64_instrs_integer_shift_variable(
            state,
            tracer,
            s_139_8,
            s_139_14,
            s_139_20,
            s_139_26,
            s_139_32,
        );
        // N s_139_34: return
        return;
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_140_0: read-var merge#var.1:struct
        let s_140_0: u32 = fn_state.merge_var._1;
        // D s_140_1: write-var u#25890 <= s_140_0
        fn_state.u_25890 = s_140_0;
        // C s_140_2: const #14s : i
        let s_140_2: i128 = 14;
        // D s_140_3: read-var u#25890:u32
        let s_140_3: u32 = fn_state.u_25890;
        // D s_140_4: cast zx s_140_3 -> bv
        let s_140_4: Bits = Bits::new(s_140_3 as u128, 32u16);
        // C s_140_5: const #1s : i64
        let s_140_5: i64 = 1;
        // C s_140_6: cast zx s_140_5 -> i
        let s_140_6: i128 = (i128::try_from(s_140_5).unwrap());
        // C s_140_7: const #17s : i
        let s_140_7: i128 = 17;
        // C s_140_8: add s_140_7 s_140_6
        let s_140_8: i128 = (s_140_7 + s_140_6);
        // D s_140_9: bit-extract s_140_4 s_140_2 s_140_8
        let s_140_9: Bits = (Bits::new(
            ((s_140_4) >> (s_140_2)).value(),
            u16::try_from(s_140_8).unwrap(),
        ));
        // D s_140_10: cast reint s_140_9 -> u18
        let s_140_10: u32 = (s_140_9.value() as u32);
        // D s_140_11: cast zx s_140_10 -> bv
        let s_140_11: Bits = Bits::new(s_140_10 as u128, 18u16);
        // C s_140_12: const #224004u : u18
        let s_140_12: u32 = 224004;
        // C s_140_13: cast zx s_140_12 -> bv
        let s_140_13: Bits = Bits::new(s_140_12 as u128, 18u16);
        // D s_140_14: cmp-eq s_140_11 s_140_13
        let s_140_14: bool = ((s_140_11) == (s_140_13));
        // N s_140_15: branch s_140_14 b546 b141
        if s_140_14 {
            return block_546(state, tracer, fn_state);
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
        // D s_141_1: write-var gs#382711 <= s_141_0
        fn_state.gs_382711 = s_141_0;
        // N s_141_2: jump b142
        return block_142(state, tracer, fn_state);
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_142_0: read-var gs#382711:u8
        let s_142_0: bool = fn_state.gs_382711;
        // N s_142_1: branch s_142_0 b545 b143
        if s_142_0 {
            return block_545(state, tracer, fn_state);
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
        // D s_143_1: write-var gs#382713 <= s_143_0
        fn_state.gs_382713 = s_143_0;
        // N s_143_2: jump b144
        return block_144(state, tracer, fn_state);
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_144_0: read-var gs#382713:u8
        let s_144_0: bool = fn_state.gs_382713;
        // D s_144_1: not s_144_0
        let s_144_1: bool = !s_144_0;
        // N s_144_2: branch s_144_1 b146 b145
        if s_144_1 {
            return block_146(state, tracer, fn_state);
        } else {
            return block_145(state, tracer, fn_state);
        };
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_145_0: const #60s : i
        let s_145_0: i128 = 60;
        // C s_145_1: const #14696u : u32
        let s_145_1: u32 = 14696;
        // N s_145_2: write-reg s_145_1 <= s_145_0
        let s_145_2: () = {
            state.write_register::<i128>(s_145_1 as isize, s_145_0);
            tracer.write_register(s_145_1 as isize, s_145_0);
        };
        // C s_145_3: const #0s : i
        let s_145_3: i128 = 0;
        // C s_145_4: const #5s : i
        let s_145_4: i128 = 5;
        // D s_145_5: read-var u#25890:u32
        let s_145_5: u32 = fn_state.u_25890;
        // D s_145_6: cast zx s_145_5 -> bv
        let s_145_6: Bits = Bits::new(s_145_5 as u128, 32u16);
        // D s_145_7: bit-extract s_145_6 s_145_3 s_145_4
        let s_145_7: Bits = (Bits::new(
            ((s_145_6) >> (s_145_3)).value(),
            u16::try_from(s_145_4).unwrap(),
        ));
        // D s_145_8: cast reint s_145_7 -> u8
        let s_145_8: u8 = (s_145_7.value() as u8);
        // C s_145_9: const #5s : i
        let s_145_9: i128 = 5;
        // C s_145_10: const #5s : i
        let s_145_10: i128 = 5;
        // D s_145_11: read-var u#25890:u32
        let s_145_11: u32 = fn_state.u_25890;
        // D s_145_12: cast zx s_145_11 -> bv
        let s_145_12: Bits = Bits::new(s_145_11 as u128, 32u16);
        // D s_145_13: bit-extract s_145_12 s_145_9 s_145_10
        let s_145_13: Bits = (Bits::new(
            ((s_145_12) >> (s_145_9)).value(),
            u16::try_from(s_145_10).unwrap(),
        ));
        // D s_145_14: cast reint s_145_13 -> u8
        let s_145_14: u8 = (s_145_13.value() as u8);
        // C s_145_15: const #13s : i
        let s_145_15: i128 = 13;
        // C s_145_16: const #1s : i
        let s_145_16: i128 = 1;
        // D s_145_17: read-var u#25890:u32
        let s_145_17: u32 = fn_state.u_25890;
        // D s_145_18: cast zx s_145_17 -> bv
        let s_145_18: Bits = Bits::new(s_145_17 as u128, 32u16);
        // D s_145_19: bit-extract s_145_18 s_145_15 s_145_16
        let s_145_19: Bits = (Bits::new(
            ((s_145_18) >> (s_145_15)).value(),
            u16::try_from(s_145_16).unwrap(),
        ));
        // D s_145_20: cast reint s_145_19 -> u8
        let s_145_20: bool = ((s_145_19.value()) != 0);
        // D s_145_21: call decode_autda_aarch64_instrs_integer_pac_autda_dp_1src(s_145_8, s_145_14, s_145_20)
        let s_145_21: () = decode_autda_aarch64_instrs_integer_pac_autda_dp_1src(
            state,
            tracer,
            s_145_8,
            s_145_14,
            s_145_20,
        );
        // N s_145_22: return
        return;
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_146_0: read-var merge#var.1:struct
        let s_146_0: u32 = fn_state.merge_var._1;
        // D s_146_1: write-var u#25894 <= s_146_0
        fn_state.u_25894 = s_146_0;
        // C s_146_2: const #14s : i
        let s_146_2: i128 = 14;
        // D s_146_3: read-var u#25894:u32
        let s_146_3: u32 = fn_state.u_25894;
        // D s_146_4: cast zx s_146_3 -> bv
        let s_146_4: Bits = Bits::new(s_146_3 as u128, 32u16);
        // C s_146_5: const #1s : i64
        let s_146_5: i64 = 1;
        // C s_146_6: cast zx s_146_5 -> i
        let s_146_6: i128 = (i128::try_from(s_146_5).unwrap());
        // C s_146_7: const #17s : i
        let s_146_7: i128 = 17;
        // C s_146_8: add s_146_7 s_146_6
        let s_146_8: i128 = (s_146_7 + s_146_6);
        // D s_146_9: bit-extract s_146_4 s_146_2 s_146_8
        let s_146_9: Bits = (Bits::new(
            ((s_146_4) >> (s_146_2)).value(),
            u16::try_from(s_146_8).unwrap(),
        ));
        // D s_146_10: cast reint s_146_9 -> u18
        let s_146_10: u32 = (s_146_9.value() as u32);
        // D s_146_11: cast zx s_146_10 -> bv
        let s_146_11: Bits = Bits::new(s_146_10 as u128, 18u16);
        // C s_146_12: const #224004u : u18
        let s_146_12: u32 = 224004;
        // C s_146_13: cast zx s_146_12 -> bv
        let s_146_13: Bits = Bits::new(s_146_12 as u128, 18u16);
        // D s_146_14: cmp-eq s_146_11 s_146_13
        let s_146_14: bool = ((s_146_11) == (s_146_13));
        // N s_146_15: branch s_146_14 b544 b147
        if s_146_14 {
            return block_544(state, tracer, fn_state);
        } else {
            return block_147(state, tracer, fn_state);
        };
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_147_0: const #0u : u8
        let s_147_0: bool = false;
        // D s_147_1: write-var gs#382727 <= s_147_0
        fn_state.gs_382727 = s_147_0;
        // N s_147_2: jump b148
        return block_148(state, tracer, fn_state);
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_148_0: read-var gs#382727:u8
        let s_148_0: bool = fn_state.gs_382727;
        // N s_148_1: branch s_148_0 b543 b149
        if s_148_0 {
            return block_543(state, tracer, fn_state);
        } else {
            return block_149(state, tracer, fn_state);
        };
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_149_0: const #0u : u8
        let s_149_0: bool = false;
        // D s_149_1: write-var gs#382729 <= s_149_0
        fn_state.gs_382729 = s_149_0;
        // N s_149_2: jump b150
        return block_150(state, tracer, fn_state);
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_150_0: read-var gs#382729:u8
        let s_150_0: bool = fn_state.gs_382729;
        // D s_150_1: not s_150_0
        let s_150_1: bool = !s_150_0;
        // N s_150_2: branch s_150_1 b152 b151
        if s_150_1 {
            return block_152(state, tracer, fn_state);
        } else {
            return block_151(state, tracer, fn_state);
        };
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_151_0: const #61s : i
        let s_151_0: i128 = 61;
        // C s_151_1: const #14696u : u32
        let s_151_1: u32 = 14696;
        // N s_151_2: write-reg s_151_1 <= s_151_0
        let s_151_2: () = {
            state.write_register::<i128>(s_151_1 as isize, s_151_0);
            tracer.write_register(s_151_1 as isize, s_151_0);
        };
        // C s_151_3: const #0s : i
        let s_151_3: i128 = 0;
        // C s_151_4: const #5s : i
        let s_151_4: i128 = 5;
        // D s_151_5: read-var u#25894:u32
        let s_151_5: u32 = fn_state.u_25894;
        // D s_151_6: cast zx s_151_5 -> bv
        let s_151_6: Bits = Bits::new(s_151_5 as u128, 32u16);
        // D s_151_7: bit-extract s_151_6 s_151_3 s_151_4
        let s_151_7: Bits = (Bits::new(
            ((s_151_6) >> (s_151_3)).value(),
            u16::try_from(s_151_4).unwrap(),
        ));
        // D s_151_8: cast reint s_151_7 -> u8
        let s_151_8: u8 = (s_151_7.value() as u8);
        // C s_151_9: const #5s : i
        let s_151_9: i128 = 5;
        // C s_151_10: const #5s : i
        let s_151_10: i128 = 5;
        // D s_151_11: read-var u#25894:u32
        let s_151_11: u32 = fn_state.u_25894;
        // D s_151_12: cast zx s_151_11 -> bv
        let s_151_12: Bits = Bits::new(s_151_11 as u128, 32u16);
        // D s_151_13: bit-extract s_151_12 s_151_9 s_151_10
        let s_151_13: Bits = (Bits::new(
            ((s_151_12) >> (s_151_9)).value(),
            u16::try_from(s_151_10).unwrap(),
        ));
        // D s_151_14: cast reint s_151_13 -> u8
        let s_151_14: u8 = (s_151_13.value() as u8);
        // C s_151_15: const #13s : i
        let s_151_15: i128 = 13;
        // C s_151_16: const #1s : i
        let s_151_16: i128 = 1;
        // D s_151_17: read-var u#25894:u32
        let s_151_17: u32 = fn_state.u_25894;
        // D s_151_18: cast zx s_151_17 -> bv
        let s_151_18: Bits = Bits::new(s_151_17 as u128, 32u16);
        // D s_151_19: bit-extract s_151_18 s_151_15 s_151_16
        let s_151_19: Bits = (Bits::new(
            ((s_151_18) >> (s_151_15)).value(),
            u16::try_from(s_151_16).unwrap(),
        ));
        // D s_151_20: cast reint s_151_19 -> u8
        let s_151_20: bool = ((s_151_19.value()) != 0);
        // D s_151_21: call decode_autdb_aarch64_instrs_integer_pac_autdb_dp_1src(s_151_8, s_151_14, s_151_20)
        let s_151_21: () = decode_autdb_aarch64_instrs_integer_pac_autdb_dp_1src(
            state,
            tracer,
            s_151_8,
            s_151_14,
            s_151_20,
        );
        // N s_151_22: return
        return;
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_152_0: read-var merge#var.1:struct
        let s_152_0: u32 = fn_state.merge_var._1;
        // D s_152_1: write-var u#25899 <= s_152_0
        fn_state.u_25899 = s_152_0;
        // C s_152_2: const #14s : i
        let s_152_2: i128 = 14;
        // D s_152_3: read-var u#25899:u32
        let s_152_3: u32 = fn_state.u_25899;
        // D s_152_4: cast zx s_152_3 -> bv
        let s_152_4: Bits = Bits::new(s_152_3 as u128, 32u16);
        // C s_152_5: const #1s : i64
        let s_152_5: i64 = 1;
        // C s_152_6: cast zx s_152_5 -> i
        let s_152_6: i128 = (i128::try_from(s_152_5).unwrap());
        // C s_152_7: const #17s : i
        let s_152_7: i128 = 17;
        // C s_152_8: add s_152_7 s_152_6
        let s_152_8: i128 = (s_152_7 + s_152_6);
        // D s_152_9: bit-extract s_152_4 s_152_2 s_152_8
        let s_152_9: Bits = (Bits::new(
            ((s_152_4) >> (s_152_2)).value(),
            u16::try_from(s_152_8).unwrap(),
        ));
        // D s_152_10: cast reint s_152_9 -> u18
        let s_152_10: u32 = (s_152_9.value() as u32);
        // D s_152_11: cast zx s_152_10 -> bv
        let s_152_11: Bits = Bits::new(s_152_10 as u128, 18u16);
        // C s_152_12: const #224004u : u18
        let s_152_12: u32 = 224004;
        // C s_152_13: cast zx s_152_12 -> bv
        let s_152_13: Bits = Bits::new(s_152_12 as u128, 18u16);
        // D s_152_14: cmp-eq s_152_11 s_152_13
        let s_152_14: bool = ((s_152_11) == (s_152_13));
        // N s_152_15: branch s_152_14 b542 b153
        if s_152_14 {
            return block_542(state, tracer, fn_state);
        } else {
            return block_153(state, tracer, fn_state);
        };
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_153_0: const #0u : u8
        let s_153_0: bool = false;
        // D s_153_1: write-var gs#382743 <= s_153_0
        fn_state.gs_382743 = s_153_0;
        // N s_153_2: jump b154
        return block_154(state, tracer, fn_state);
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_154_0: read-var gs#382743:u8
        let s_154_0: bool = fn_state.gs_382743;
        // N s_154_1: branch s_154_0 b541 b155
        if s_154_0 {
            return block_541(state, tracer, fn_state);
        } else {
            return block_155(state, tracer, fn_state);
        };
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_155_0: const #0u : u8
        let s_155_0: bool = false;
        // D s_155_1: write-var gs#382745 <= s_155_0
        fn_state.gs_382745 = s_155_0;
        // N s_155_2: jump b156
        return block_156(state, tracer, fn_state);
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_156_0: read-var gs#382745:u8
        let s_156_0: bool = fn_state.gs_382745;
        // D s_156_1: not s_156_0
        let s_156_1: bool = !s_156_0;
        // N s_156_2: branch s_156_1 b158 b157
        if s_156_1 {
            return block_158(state, tracer, fn_state);
        } else {
            return block_157(state, tracer, fn_state);
        };
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_157_0: const #62s : i
        let s_157_0: i128 = 62;
        // C s_157_1: const #14696u : u32
        let s_157_1: u32 = 14696;
        // N s_157_2: write-reg s_157_1 <= s_157_0
        let s_157_2: () = {
            state.write_register::<i128>(s_157_1 as isize, s_157_0);
            tracer.write_register(s_157_1 as isize, s_157_0);
        };
        // C s_157_3: const #0s : i
        let s_157_3: i128 = 0;
        // C s_157_4: const #5s : i
        let s_157_4: i128 = 5;
        // D s_157_5: read-var u#25899:u32
        let s_157_5: u32 = fn_state.u_25899;
        // D s_157_6: cast zx s_157_5 -> bv
        let s_157_6: Bits = Bits::new(s_157_5 as u128, 32u16);
        // D s_157_7: bit-extract s_157_6 s_157_3 s_157_4
        let s_157_7: Bits = (Bits::new(
            ((s_157_6) >> (s_157_3)).value(),
            u16::try_from(s_157_4).unwrap(),
        ));
        // D s_157_8: cast reint s_157_7 -> u8
        let s_157_8: u8 = (s_157_7.value() as u8);
        // C s_157_9: const #5s : i
        let s_157_9: i128 = 5;
        // C s_157_10: const #5s : i
        let s_157_10: i128 = 5;
        // D s_157_11: read-var u#25899:u32
        let s_157_11: u32 = fn_state.u_25899;
        // D s_157_12: cast zx s_157_11 -> bv
        let s_157_12: Bits = Bits::new(s_157_11 as u128, 32u16);
        // D s_157_13: bit-extract s_157_12 s_157_9 s_157_10
        let s_157_13: Bits = (Bits::new(
            ((s_157_12) >> (s_157_9)).value(),
            u16::try_from(s_157_10).unwrap(),
        ));
        // D s_157_14: cast reint s_157_13 -> u8
        let s_157_14: u8 = (s_157_13.value() as u8);
        // C s_157_15: const #13s : i
        let s_157_15: i128 = 13;
        // C s_157_16: const #1s : i
        let s_157_16: i128 = 1;
        // D s_157_17: read-var u#25899:u32
        let s_157_17: u32 = fn_state.u_25899;
        // D s_157_18: cast zx s_157_17 -> bv
        let s_157_18: Bits = Bits::new(s_157_17 as u128, 32u16);
        // D s_157_19: bit-extract s_157_18 s_157_15 s_157_16
        let s_157_19: Bits = (Bits::new(
            ((s_157_18) >> (s_157_15)).value(),
            u16::try_from(s_157_16).unwrap(),
        ));
        // D s_157_20: cast reint s_157_19 -> u8
        let s_157_20: bool = ((s_157_19.value()) != 0);
        // D s_157_21: call decode_autia_aarch64_instrs_integer_pac_autia_dp_1src(s_157_8, s_157_14, s_157_20)
        let s_157_21: () = decode_autia_aarch64_instrs_integer_pac_autia_dp_1src(
            state,
            tracer,
            s_157_8,
            s_157_14,
            s_157_20,
        );
        // N s_157_22: return
        return;
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_158_0: read-var merge#var.1:struct
        let s_158_0: u32 = fn_state.merge_var._1;
        // D s_158_1: write-var u#25904 <= s_158_0
        fn_state.u_25904 = s_158_0;
        // C s_158_2: const #14s : i
        let s_158_2: i128 = 14;
        // D s_158_3: read-var u#25904:u32
        let s_158_3: u32 = fn_state.u_25904;
        // D s_158_4: cast zx s_158_3 -> bv
        let s_158_4: Bits = Bits::new(s_158_3 as u128, 32u16);
        // C s_158_5: const #1s : i64
        let s_158_5: i64 = 1;
        // C s_158_6: cast zx s_158_5 -> i
        let s_158_6: i128 = (i128::try_from(s_158_5).unwrap());
        // C s_158_7: const #17s : i
        let s_158_7: i128 = 17;
        // C s_158_8: add s_158_7 s_158_6
        let s_158_8: i128 = (s_158_7 + s_158_6);
        // D s_158_9: bit-extract s_158_4 s_158_2 s_158_8
        let s_158_9: Bits = (Bits::new(
            ((s_158_4) >> (s_158_2)).value(),
            u16::try_from(s_158_8).unwrap(),
        ));
        // D s_158_10: cast reint s_158_9 -> u18
        let s_158_10: u32 = (s_158_9.value() as u32);
        // D s_158_11: cast zx s_158_10 -> bv
        let s_158_11: Bits = Bits::new(s_158_10 as u128, 18u16);
        // C s_158_12: const #224004u : u18
        let s_158_12: u32 = 224004;
        // C s_158_13: cast zx s_158_12 -> bv
        let s_158_13: Bits = Bits::new(s_158_12 as u128, 18u16);
        // D s_158_14: cmp-eq s_158_11 s_158_13
        let s_158_14: bool = ((s_158_11) == (s_158_13));
        // N s_158_15: branch s_158_14 b540 b159
        if s_158_14 {
            return block_540(state, tracer, fn_state);
        } else {
            return block_159(state, tracer, fn_state);
        };
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_159_0: const #0u : u8
        let s_159_0: bool = false;
        // D s_159_1: write-var gs#382759 <= s_159_0
        fn_state.gs_382759 = s_159_0;
        // N s_159_2: jump b160
        return block_160(state, tracer, fn_state);
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_160_0: read-var gs#382759:u8
        let s_160_0: bool = fn_state.gs_382759;
        // N s_160_1: branch s_160_0 b539 b161
        if s_160_0 {
            return block_539(state, tracer, fn_state);
        } else {
            return block_161(state, tracer, fn_state);
        };
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_161_0: const #0u : u8
        let s_161_0: bool = false;
        // D s_161_1: write-var gs#382761 <= s_161_0
        fn_state.gs_382761 = s_161_0;
        // N s_161_2: jump b162
        return block_162(state, tracer, fn_state);
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_162_0: read-var gs#382761:u8
        let s_162_0: bool = fn_state.gs_382761;
        // D s_162_1: not s_162_0
        let s_162_1: bool = !s_162_0;
        // N s_162_2: branch s_162_1 b164 b163
        if s_162_1 {
            return block_164(state, tracer, fn_state);
        } else {
            return block_163(state, tracer, fn_state);
        };
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_163_0: const #64s : i
        let s_163_0: i128 = 64;
        // C s_163_1: const #14696u : u32
        let s_163_1: u32 = 14696;
        // N s_163_2: write-reg s_163_1 <= s_163_0
        let s_163_2: () = {
            state.write_register::<i128>(s_163_1 as isize, s_163_0);
            tracer.write_register(s_163_1 as isize, s_163_0);
        };
        // C s_163_3: const #0s : i
        let s_163_3: i128 = 0;
        // C s_163_4: const #5s : i
        let s_163_4: i128 = 5;
        // D s_163_5: read-var u#25904:u32
        let s_163_5: u32 = fn_state.u_25904;
        // D s_163_6: cast zx s_163_5 -> bv
        let s_163_6: Bits = Bits::new(s_163_5 as u128, 32u16);
        // D s_163_7: bit-extract s_163_6 s_163_3 s_163_4
        let s_163_7: Bits = (Bits::new(
            ((s_163_6) >> (s_163_3)).value(),
            u16::try_from(s_163_4).unwrap(),
        ));
        // D s_163_8: cast reint s_163_7 -> u8
        let s_163_8: u8 = (s_163_7.value() as u8);
        // C s_163_9: const #5s : i
        let s_163_9: i128 = 5;
        // C s_163_10: const #5s : i
        let s_163_10: i128 = 5;
        // D s_163_11: read-var u#25904:u32
        let s_163_11: u32 = fn_state.u_25904;
        // D s_163_12: cast zx s_163_11 -> bv
        let s_163_12: Bits = Bits::new(s_163_11 as u128, 32u16);
        // D s_163_13: bit-extract s_163_12 s_163_9 s_163_10
        let s_163_13: Bits = (Bits::new(
            ((s_163_12) >> (s_163_9)).value(),
            u16::try_from(s_163_10).unwrap(),
        ));
        // D s_163_14: cast reint s_163_13 -> u8
        let s_163_14: u8 = (s_163_13.value() as u8);
        // C s_163_15: const #13s : i
        let s_163_15: i128 = 13;
        // C s_163_16: const #1s : i
        let s_163_16: i128 = 1;
        // D s_163_17: read-var u#25904:u32
        let s_163_17: u32 = fn_state.u_25904;
        // D s_163_18: cast zx s_163_17 -> bv
        let s_163_18: Bits = Bits::new(s_163_17 as u128, 32u16);
        // D s_163_19: bit-extract s_163_18 s_163_15 s_163_16
        let s_163_19: Bits = (Bits::new(
            ((s_163_18) >> (s_163_15)).value(),
            u16::try_from(s_163_16).unwrap(),
        ));
        // D s_163_20: cast reint s_163_19 -> u8
        let s_163_20: bool = ((s_163_19.value()) != 0);
        // D s_163_21: call decode_autib_aarch64_instrs_integer_pac_autib_dp_1src(s_163_8, s_163_14, s_163_20)
        let s_163_21: () = decode_autib_aarch64_instrs_integer_pac_autib_dp_1src(
            state,
            tracer,
            s_163_8,
            s_163_14,
            s_163_20,
        );
        // N s_163_22: return
        return;
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_164_0: read-var merge#var.1:struct
        let s_164_0: u32 = fn_state.merge_var._1;
        // D s_164_1: write-var u#25909 <= s_164_0
        fn_state.u_25909 = s_164_0;
        // C s_164_2: const #21s : i
        let s_164_2: i128 = 21;
        // D s_164_3: read-var u#25909:u32
        let s_164_3: u32 = fn_state.u_25909;
        // D s_164_4: cast zx s_164_3 -> bv
        let s_164_4: Bits = Bits::new(s_164_3 as u128, 32u16);
        // C s_164_5: const #1s : i64
        let s_164_5: i64 = 1;
        // C s_164_6: cast zx s_164_5 -> i
        let s_164_6: i128 = (i128::try_from(s_164_5).unwrap());
        // C s_164_7: const #9s : i
        let s_164_7: i128 = 9;
        // C s_164_8: add s_164_7 s_164_6
        let s_164_8: i128 = (s_164_7 + s_164_6);
        // D s_164_9: bit-extract s_164_4 s_164_2 s_164_8
        let s_164_9: Bits = (Bits::new(
            ((s_164_4) >> (s_164_2)).value(),
            u16::try_from(s_164_8).unwrap(),
        ));
        // D s_164_10: cast reint s_164_9 -> u10
        let s_164_10: u16 = (s_164_9.value() as u16);
        // D s_164_11: cast zx s_164_10 -> bv
        let s_164_11: Bits = Bits::new(s_164_10 as u128, 10u16);
        // C s_164_12: const #466u : u10
        let s_164_12: u16 = 466;
        // C s_164_13: cast zx s_164_12 -> bv
        let s_164_13: Bits = Bits::new(s_164_12 as u128, 10u16);
        // D s_164_14: cmp-eq s_164_11 s_164_13
        let s_164_14: bool = ((s_164_11) == (s_164_13));
        // N s_164_15: branch s_164_14 b535 b165
        if s_164_14 {
            return block_535(state, tracer, fn_state);
        } else {
            return block_165(state, tracer, fn_state);
        };
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_165_0: const #0u : u8
        let s_165_0: bool = false;
        // D s_165_1: write-var gs#382778 <= s_165_0
        fn_state.gs_382778 = s_165_0;
        // N s_165_2: jump b166
        return block_166(state, tracer, fn_state);
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_166_0: read-var gs#382778:u8
        let s_166_0: bool = fn_state.gs_382778;
        // N s_166_1: branch s_166_0 b534 b167
        if s_166_0 {
            return block_534(state, tracer, fn_state);
        } else {
            return block_167(state, tracer, fn_state);
        };
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_167_0: const #0u : u8
        let s_167_0: bool = false;
        // D s_167_1: write-var gs#382780 <= s_167_0
        fn_state.gs_382780 = s_167_0;
        // N s_167_2: jump b168
        return block_168(state, tracer, fn_state);
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_168_0: read-var gs#382780:u8
        let s_168_0: bool = fn_state.gs_382780;
        // D s_168_1: not s_168_0
        let s_168_1: bool = !s_168_0;
        // N s_168_2: branch s_168_1 b170 b169
        if s_168_1 {
            return block_170(state, tracer, fn_state);
        } else {
            return block_169(state, tracer, fn_state);
        };
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_169_0: const #119s : i
        let s_169_0: i128 = 119;
        // C s_169_1: const #14696u : u32
        let s_169_1: u32 = 14696;
        // N s_169_2: write-reg s_169_1 <= s_169_0
        let s_169_2: () = {
            state.write_register::<i128>(s_169_1 as isize, s_169_0);
            tracer.write_register(s_169_1 as isize, s_169_0);
        };
        // C s_169_3: const #0s : i
        let s_169_3: i128 = 0;
        // C s_169_4: const #4s : i
        let s_169_4: i128 = 4;
        // D s_169_5: read-var u#25909:u32
        let s_169_5: u32 = fn_state.u_25909;
        // D s_169_6: cast zx s_169_5 -> bv
        let s_169_6: Bits = Bits::new(s_169_5 as u128, 32u16);
        // D s_169_7: bit-extract s_169_6 s_169_3 s_169_4
        let s_169_7: Bits = (Bits::new(
            ((s_169_6) >> (s_169_3)).value(),
            u16::try_from(s_169_4).unwrap(),
        ));
        // D s_169_8: cast reint s_169_7 -> u8
        let s_169_8: u8 = (s_169_7.value() as u8);
        // C s_169_9: const #5s : i
        let s_169_9: i128 = 5;
        // C s_169_10: const #5s : i
        let s_169_10: i128 = 5;
        // D s_169_11: read-var u#25909:u32
        let s_169_11: u32 = fn_state.u_25909;
        // D s_169_12: cast zx s_169_11 -> bv
        let s_169_12: Bits = Bits::new(s_169_11 as u128, 32u16);
        // D s_169_13: bit-extract s_169_12 s_169_9 s_169_10
        let s_169_13: Bits = (Bits::new(
            ((s_169_12) >> (s_169_9)).value(),
            u16::try_from(s_169_10).unwrap(),
        ));
        // D s_169_14: cast reint s_169_13 -> u8
        let s_169_14: u8 = (s_169_13.value() as u8);
        // C s_169_15: const #12s : i
        let s_169_15: i128 = 12;
        // C s_169_16: const #4s : i
        let s_169_16: i128 = 4;
        // D s_169_17: read-var u#25909:u32
        let s_169_17: u32 = fn_state.u_25909;
        // D s_169_18: cast zx s_169_17 -> bv
        let s_169_18: Bits = Bits::new(s_169_17 as u128, 32u16);
        // D s_169_19: bit-extract s_169_18 s_169_15 s_169_16
        let s_169_19: Bits = (Bits::new(
            ((s_169_18) >> (s_169_15)).value(),
            u16::try_from(s_169_16).unwrap(),
        ));
        // D s_169_20: cast reint s_169_19 -> u8
        let s_169_20: u8 = (s_169_19.value() as u8);
        // C s_169_21: const #16s : i
        let s_169_21: i128 = 16;
        // C s_169_22: const #5s : i
        let s_169_22: i128 = 5;
        // D s_169_23: read-var u#25909:u32
        let s_169_23: u32 = fn_state.u_25909;
        // D s_169_24: cast zx s_169_23 -> bv
        let s_169_24: Bits = Bits::new(s_169_23 as u128, 32u16);
        // D s_169_25: bit-extract s_169_24 s_169_21 s_169_22
        let s_169_25: Bits = (Bits::new(
            ((s_169_24) >> (s_169_21)).value(),
            u16::try_from(s_169_22).unwrap(),
        ));
        // D s_169_26: cast reint s_169_25 -> u8
        let s_169_26: u8 = (s_169_25.value() as u8);
        // C s_169_27: const #30s : i
        let s_169_27: i128 = 30;
        // C s_169_28: const #1s : i
        let s_169_28: i128 = 1;
        // D s_169_29: read-var u#25909:u32
        let s_169_29: u32 = fn_state.u_25909;
        // D s_169_30: cast zx s_169_29 -> bv
        let s_169_30: Bits = Bits::new(s_169_29 as u128, 32u16);
        // D s_169_31: bit-extract s_169_30 s_169_27 s_169_28
        let s_169_31: Bits = (Bits::new(
            ((s_169_30) >> (s_169_27)).value(),
            u16::try_from(s_169_28).unwrap(),
        ));
        // D s_169_32: cast reint s_169_31 -> u8
        let s_169_32: bool = ((s_169_31.value()) != 0);
        // C s_169_33: const #31s : i
        let s_169_33: i128 = 31;
        // C s_169_34: const #1s : i
        let s_169_34: i128 = 1;
        // D s_169_35: read-var u#25909:u32
        let s_169_35: u32 = fn_state.u_25909;
        // D s_169_36: cast zx s_169_35 -> bv
        let s_169_36: Bits = Bits::new(s_169_35 as u128, 32u16);
        // D s_169_37: bit-extract s_169_36 s_169_33 s_169_34
        let s_169_37: Bits = (Bits::new(
            ((s_169_36) >> (s_169_33)).value(),
            u16::try_from(s_169_34).unwrap(),
        ));
        // D s_169_38: cast reint s_169_37 -> u8
        let s_169_38: bool = ((s_169_37.value()) != 0);
        // D s_169_39: call decode_ccmn_reg_aarch64_instrs_integer_conditional_compare_register(s_169_8, s_169_14, s_169_20, s_169_26, s_169_32, s_169_38)
        let s_169_39: () = decode_ccmn_reg_aarch64_instrs_integer_conditional_compare_register(
            state,
            tracer,
            s_169_8,
            s_169_14,
            s_169_20,
            s_169_26,
            s_169_32,
            s_169_38,
        );
        // N s_169_40: return
        return;
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_170_0: read-var merge#var.1:struct
        let s_170_0: u32 = fn_state.merge_var._1;
        // D s_170_1: write-var u#25915 <= s_170_0
        fn_state.u_25915 = s_170_0;
        // C s_170_2: const #21s : i
        let s_170_2: i128 = 21;
        // D s_170_3: read-var u#25915:u32
        let s_170_3: u32 = fn_state.u_25915;
        // D s_170_4: cast zx s_170_3 -> bv
        let s_170_4: Bits = Bits::new(s_170_3 as u128, 32u16);
        // C s_170_5: const #1s : i64
        let s_170_5: i64 = 1;
        // C s_170_6: cast zx s_170_5 -> i
        let s_170_6: i128 = (i128::try_from(s_170_5).unwrap());
        // C s_170_7: const #9s : i
        let s_170_7: i128 = 9;
        // C s_170_8: add s_170_7 s_170_6
        let s_170_8: i128 = (s_170_7 + s_170_6);
        // D s_170_9: bit-extract s_170_4 s_170_2 s_170_8
        let s_170_9: Bits = (Bits::new(
            ((s_170_4) >> (s_170_2)).value(),
            u16::try_from(s_170_8).unwrap(),
        ));
        // D s_170_10: cast reint s_170_9 -> u10
        let s_170_10: u16 = (s_170_9.value() as u16);
        // D s_170_11: cast zx s_170_10 -> bv
        let s_170_11: Bits = Bits::new(s_170_10 as u128, 10u16);
        // C s_170_12: const #978u : u10
        let s_170_12: u16 = 978;
        // C s_170_13: cast zx s_170_12 -> bv
        let s_170_13: Bits = Bits::new(s_170_12 as u128, 10u16);
        // D s_170_14: cmp-eq s_170_11 s_170_13
        let s_170_14: bool = ((s_170_11) == (s_170_13));
        // N s_170_15: branch s_170_14 b530 b171
        if s_170_14 {
            return block_530(state, tracer, fn_state);
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
        // D s_171_1: write-var gs#382803 <= s_171_0
        fn_state.gs_382803 = s_171_0;
        // N s_171_2: jump b172
        return block_172(state, tracer, fn_state);
    }
    fn block_172<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_172_0: read-var gs#382803:u8
        let s_172_0: bool = fn_state.gs_382803;
        // N s_172_1: branch s_172_0 b529 b173
        if s_172_0 {
            return block_529(state, tracer, fn_state);
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
        // D s_173_1: write-var gs#382805 <= s_173_0
        fn_state.gs_382805 = s_173_0;
        // N s_173_2: jump b174
        return block_174(state, tracer, fn_state);
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_174_0: read-var gs#382805:u8
        let s_174_0: bool = fn_state.gs_382805;
        // D s_174_1: not s_174_0
        let s_174_1: bool = !s_174_0;
        // N s_174_2: branch s_174_1 b176 b175
        if s_174_1 {
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
        // C s_175_0: const #120s : i
        let s_175_0: i128 = 120;
        // C s_175_1: const #14696u : u32
        let s_175_1: u32 = 14696;
        // N s_175_2: write-reg s_175_1 <= s_175_0
        let s_175_2: () = {
            state.write_register::<i128>(s_175_1 as isize, s_175_0);
            tracer.write_register(s_175_1 as isize, s_175_0);
        };
        // C s_175_3: const #0s : i
        let s_175_3: i128 = 0;
        // C s_175_4: const #4s : i
        let s_175_4: i128 = 4;
        // D s_175_5: read-var u#25915:u32
        let s_175_5: u32 = fn_state.u_25915;
        // D s_175_6: cast zx s_175_5 -> bv
        let s_175_6: Bits = Bits::new(s_175_5 as u128, 32u16);
        // D s_175_7: bit-extract s_175_6 s_175_3 s_175_4
        let s_175_7: Bits = (Bits::new(
            ((s_175_6) >> (s_175_3)).value(),
            u16::try_from(s_175_4).unwrap(),
        ));
        // D s_175_8: cast reint s_175_7 -> u8
        let s_175_8: u8 = (s_175_7.value() as u8);
        // C s_175_9: const #5s : i
        let s_175_9: i128 = 5;
        // C s_175_10: const #5s : i
        let s_175_10: i128 = 5;
        // D s_175_11: read-var u#25915:u32
        let s_175_11: u32 = fn_state.u_25915;
        // D s_175_12: cast zx s_175_11 -> bv
        let s_175_12: Bits = Bits::new(s_175_11 as u128, 32u16);
        // D s_175_13: bit-extract s_175_12 s_175_9 s_175_10
        let s_175_13: Bits = (Bits::new(
            ((s_175_12) >> (s_175_9)).value(),
            u16::try_from(s_175_10).unwrap(),
        ));
        // D s_175_14: cast reint s_175_13 -> u8
        let s_175_14: u8 = (s_175_13.value() as u8);
        // C s_175_15: const #12s : i
        let s_175_15: i128 = 12;
        // C s_175_16: const #4s : i
        let s_175_16: i128 = 4;
        // D s_175_17: read-var u#25915:u32
        let s_175_17: u32 = fn_state.u_25915;
        // D s_175_18: cast zx s_175_17 -> bv
        let s_175_18: Bits = Bits::new(s_175_17 as u128, 32u16);
        // D s_175_19: bit-extract s_175_18 s_175_15 s_175_16
        let s_175_19: Bits = (Bits::new(
            ((s_175_18) >> (s_175_15)).value(),
            u16::try_from(s_175_16).unwrap(),
        ));
        // D s_175_20: cast reint s_175_19 -> u8
        let s_175_20: u8 = (s_175_19.value() as u8);
        // C s_175_21: const #16s : i
        let s_175_21: i128 = 16;
        // C s_175_22: const #5s : i
        let s_175_22: i128 = 5;
        // D s_175_23: read-var u#25915:u32
        let s_175_23: u32 = fn_state.u_25915;
        // D s_175_24: cast zx s_175_23 -> bv
        let s_175_24: Bits = Bits::new(s_175_23 as u128, 32u16);
        // D s_175_25: bit-extract s_175_24 s_175_21 s_175_22
        let s_175_25: Bits = (Bits::new(
            ((s_175_24) >> (s_175_21)).value(),
            u16::try_from(s_175_22).unwrap(),
        ));
        // D s_175_26: cast reint s_175_25 -> u8
        let s_175_26: u8 = (s_175_25.value() as u8);
        // C s_175_27: const #30s : i
        let s_175_27: i128 = 30;
        // C s_175_28: const #1s : i
        let s_175_28: i128 = 1;
        // D s_175_29: read-var u#25915:u32
        let s_175_29: u32 = fn_state.u_25915;
        // D s_175_30: cast zx s_175_29 -> bv
        let s_175_30: Bits = Bits::new(s_175_29 as u128, 32u16);
        // D s_175_31: bit-extract s_175_30 s_175_27 s_175_28
        let s_175_31: Bits = (Bits::new(
            ((s_175_30) >> (s_175_27)).value(),
            u16::try_from(s_175_28).unwrap(),
        ));
        // D s_175_32: cast reint s_175_31 -> u8
        let s_175_32: bool = ((s_175_31.value()) != 0);
        // C s_175_33: const #31s : i
        let s_175_33: i128 = 31;
        // C s_175_34: const #1s : i
        let s_175_34: i128 = 1;
        // D s_175_35: read-var u#25915:u32
        let s_175_35: u32 = fn_state.u_25915;
        // D s_175_36: cast zx s_175_35 -> bv
        let s_175_36: Bits = Bits::new(s_175_35 as u128, 32u16);
        // D s_175_37: bit-extract s_175_36 s_175_33 s_175_34
        let s_175_37: Bits = (Bits::new(
            ((s_175_36) >> (s_175_33)).value(),
            u16::try_from(s_175_34).unwrap(),
        ));
        // D s_175_38: cast reint s_175_37 -> u8
        let s_175_38: bool = ((s_175_37.value()) != 0);
        // D s_175_39: call decode_ccmp_reg_aarch64_instrs_integer_conditional_compare_register(s_175_8, s_175_14, s_175_20, s_175_26, s_175_32, s_175_38)
        let s_175_39: () = decode_ccmp_reg_aarch64_instrs_integer_conditional_compare_register(
            state,
            tracer,
            s_175_8,
            s_175_14,
            s_175_20,
            s_175_26,
            s_175_32,
            s_175_38,
        );
        // N s_175_40: return
        return;
    }
    fn block_176<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_176_0: read-var merge#var.1:struct
        let s_176_0: u32 = fn_state.merge_var._1;
        // D s_176_1: write-var u#25923 <= s_176_0
        fn_state.u_25923 = s_176_0;
        // C s_176_2: const #21s : i
        let s_176_2: i128 = 21;
        // D s_176_3: read-var u#25923:u32
        let s_176_3: u32 = fn_state.u_25923;
        // D s_176_4: cast zx s_176_3 -> bv
        let s_176_4: Bits = Bits::new(s_176_3 as u128, 32u16);
        // C s_176_5: const #1s : i64
        let s_176_5: i64 = 1;
        // C s_176_6: cast zx s_176_5 -> i
        let s_176_6: i128 = (i128::try_from(s_176_5).unwrap());
        // C s_176_7: const #9s : i
        let s_176_7: i128 = 9;
        // C s_176_8: add s_176_7 s_176_6
        let s_176_8: i128 = (s_176_7 + s_176_6);
        // D s_176_9: bit-extract s_176_4 s_176_2 s_176_8
        let s_176_9: Bits = (Bits::new(
            ((s_176_4) >> (s_176_2)).value(),
            u16::try_from(s_176_8).unwrap(),
        ));
        // D s_176_10: cast reint s_176_9 -> u10
        let s_176_10: u16 = (s_176_9.value() as u16);
        // D s_176_11: cast zx s_176_10 -> bv
        let s_176_11: Bits = Bits::new(s_176_10 as u128, 10u16);
        // C s_176_12: const #466u : u10
        let s_176_12: u16 = 466;
        // C s_176_13: cast zx s_176_12 -> bv
        let s_176_13: Bits = Bits::new(s_176_12 as u128, 10u16);
        // D s_176_14: cmp-eq s_176_11 s_176_13
        let s_176_14: bool = ((s_176_11) == (s_176_13));
        // N s_176_15: branch s_176_14 b525 b177
        if s_176_14 {
            return block_525(state, tracer, fn_state);
        } else {
            return block_177(state, tracer, fn_state);
        };
    }
    fn block_177<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_177_0: const #0u : u8
        let s_177_0: bool = false;
        // D s_177_1: write-var gs#382828 <= s_177_0
        fn_state.gs_382828 = s_177_0;
        // N s_177_2: jump b178
        return block_178(state, tracer, fn_state);
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_178_0: read-var gs#382828:u8
        let s_178_0: bool = fn_state.gs_382828;
        // N s_178_1: branch s_178_0 b524 b179
        if s_178_0 {
            return block_524(state, tracer, fn_state);
        } else {
            return block_179(state, tracer, fn_state);
        };
    }
    fn block_179<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_179_0: const #0u : u8
        let s_179_0: bool = false;
        // D s_179_1: write-var gs#382830 <= s_179_0
        fn_state.gs_382830 = s_179_0;
        // N s_179_2: jump b180
        return block_180(state, tracer, fn_state);
    }
    fn block_180<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_180_0: read-var gs#382830:u8
        let s_180_0: bool = fn_state.gs_382830;
        // D s_180_1: not s_180_0
        let s_180_1: bool = !s_180_0;
        // N s_180_2: branch s_180_1 b182 b181
        if s_180_1 {
            return block_182(state, tracer, fn_state);
        } else {
            return block_181(state, tracer, fn_state);
        };
    }
    fn block_181<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_181_0: const #121s : i
        let s_181_0: i128 = 121;
        // C s_181_1: const #14696u : u32
        let s_181_1: u32 = 14696;
        // N s_181_2: write-reg s_181_1 <= s_181_0
        let s_181_2: () = {
            state.write_register::<i128>(s_181_1 as isize, s_181_0);
            tracer.write_register(s_181_1 as isize, s_181_0);
        };
        // C s_181_3: const #0s : i
        let s_181_3: i128 = 0;
        // C s_181_4: const #4s : i
        let s_181_4: i128 = 4;
        // D s_181_5: read-var u#25923:u32
        let s_181_5: u32 = fn_state.u_25923;
        // D s_181_6: cast zx s_181_5 -> bv
        let s_181_6: Bits = Bits::new(s_181_5 as u128, 32u16);
        // D s_181_7: bit-extract s_181_6 s_181_3 s_181_4
        let s_181_7: Bits = (Bits::new(
            ((s_181_6) >> (s_181_3)).value(),
            u16::try_from(s_181_4).unwrap(),
        ));
        // D s_181_8: cast reint s_181_7 -> u8
        let s_181_8: u8 = (s_181_7.value() as u8);
        // C s_181_9: const #5s : i
        let s_181_9: i128 = 5;
        // C s_181_10: const #5s : i
        let s_181_10: i128 = 5;
        // D s_181_11: read-var u#25923:u32
        let s_181_11: u32 = fn_state.u_25923;
        // D s_181_12: cast zx s_181_11 -> bv
        let s_181_12: Bits = Bits::new(s_181_11 as u128, 32u16);
        // D s_181_13: bit-extract s_181_12 s_181_9 s_181_10
        let s_181_13: Bits = (Bits::new(
            ((s_181_12) >> (s_181_9)).value(),
            u16::try_from(s_181_10).unwrap(),
        ));
        // D s_181_14: cast reint s_181_13 -> u8
        let s_181_14: u8 = (s_181_13.value() as u8);
        // C s_181_15: const #12s : i
        let s_181_15: i128 = 12;
        // C s_181_16: const #4s : i
        let s_181_16: i128 = 4;
        // D s_181_17: read-var u#25923:u32
        let s_181_17: u32 = fn_state.u_25923;
        // D s_181_18: cast zx s_181_17 -> bv
        let s_181_18: Bits = Bits::new(s_181_17 as u128, 32u16);
        // D s_181_19: bit-extract s_181_18 s_181_15 s_181_16
        let s_181_19: Bits = (Bits::new(
            ((s_181_18) >> (s_181_15)).value(),
            u16::try_from(s_181_16).unwrap(),
        ));
        // D s_181_20: cast reint s_181_19 -> u8
        let s_181_20: u8 = (s_181_19.value() as u8);
        // C s_181_21: const #16s : i
        let s_181_21: i128 = 16;
        // C s_181_22: const #5s : i
        let s_181_22: i128 = 5;
        // D s_181_23: read-var u#25923:u32
        let s_181_23: u32 = fn_state.u_25923;
        // D s_181_24: cast zx s_181_23 -> bv
        let s_181_24: Bits = Bits::new(s_181_23 as u128, 32u16);
        // D s_181_25: bit-extract s_181_24 s_181_21 s_181_22
        let s_181_25: Bits = (Bits::new(
            ((s_181_24) >> (s_181_21)).value(),
            u16::try_from(s_181_22).unwrap(),
        ));
        // D s_181_26: cast reint s_181_25 -> u8
        let s_181_26: u8 = (s_181_25.value() as u8);
        // C s_181_27: const #30s : i
        let s_181_27: i128 = 30;
        // C s_181_28: const #1s : i
        let s_181_28: i128 = 1;
        // D s_181_29: read-var u#25923:u32
        let s_181_29: u32 = fn_state.u_25923;
        // D s_181_30: cast zx s_181_29 -> bv
        let s_181_30: Bits = Bits::new(s_181_29 as u128, 32u16);
        // D s_181_31: bit-extract s_181_30 s_181_27 s_181_28
        let s_181_31: Bits = (Bits::new(
            ((s_181_30) >> (s_181_27)).value(),
            u16::try_from(s_181_28).unwrap(),
        ));
        // D s_181_32: cast reint s_181_31 -> u8
        let s_181_32: bool = ((s_181_31.value()) != 0);
        // C s_181_33: const #31s : i
        let s_181_33: i128 = 31;
        // C s_181_34: const #1s : i
        let s_181_34: i128 = 1;
        // D s_181_35: read-var u#25923:u32
        let s_181_35: u32 = fn_state.u_25923;
        // D s_181_36: cast zx s_181_35 -> bv
        let s_181_36: Bits = Bits::new(s_181_35 as u128, 32u16);
        // D s_181_37: bit-extract s_181_36 s_181_33 s_181_34
        let s_181_37: Bits = (Bits::new(
            ((s_181_36) >> (s_181_33)).value(),
            u16::try_from(s_181_34).unwrap(),
        ));
        // D s_181_38: cast reint s_181_37 -> u8
        let s_181_38: bool = ((s_181_37.value()) != 0);
        // D s_181_39: call decode_ccmn_imm_aarch64_instrs_integer_conditional_compare_immediate(s_181_8, s_181_14, s_181_20, s_181_26, s_181_32, s_181_38)
        let s_181_39: () = decode_ccmn_imm_aarch64_instrs_integer_conditional_compare_immediate(
            state,
            tracer,
            s_181_8,
            s_181_14,
            s_181_20,
            s_181_26,
            s_181_32,
            s_181_38,
        );
        // N s_181_40: return
        return;
    }
    fn block_182<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_182_0: read-var merge#var.1:struct
        let s_182_0: u32 = fn_state.merge_var._1;
        // D s_182_1: write-var u#25930 <= s_182_0
        fn_state.u_25930 = s_182_0;
        // C s_182_2: const #21s : i
        let s_182_2: i128 = 21;
        // D s_182_3: read-var u#25930:u32
        let s_182_3: u32 = fn_state.u_25930;
        // D s_182_4: cast zx s_182_3 -> bv
        let s_182_4: Bits = Bits::new(s_182_3 as u128, 32u16);
        // C s_182_5: const #1s : i64
        let s_182_5: i64 = 1;
        // C s_182_6: cast zx s_182_5 -> i
        let s_182_6: i128 = (i128::try_from(s_182_5).unwrap());
        // C s_182_7: const #9s : i
        let s_182_7: i128 = 9;
        // C s_182_8: add s_182_7 s_182_6
        let s_182_8: i128 = (s_182_7 + s_182_6);
        // D s_182_9: bit-extract s_182_4 s_182_2 s_182_8
        let s_182_9: Bits = (Bits::new(
            ((s_182_4) >> (s_182_2)).value(),
            u16::try_from(s_182_8).unwrap(),
        ));
        // D s_182_10: cast reint s_182_9 -> u10
        let s_182_10: u16 = (s_182_9.value() as u16);
        // D s_182_11: cast zx s_182_10 -> bv
        let s_182_11: Bits = Bits::new(s_182_10 as u128, 10u16);
        // C s_182_12: const #978u : u10
        let s_182_12: u16 = 978;
        // C s_182_13: cast zx s_182_12 -> bv
        let s_182_13: Bits = Bits::new(s_182_12 as u128, 10u16);
        // D s_182_14: cmp-eq s_182_11 s_182_13
        let s_182_14: bool = ((s_182_11) == (s_182_13));
        // N s_182_15: branch s_182_14 b520 b183
        if s_182_14 {
            return block_520(state, tracer, fn_state);
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
        // D s_183_1: write-var gs#382853 <= s_183_0
        fn_state.gs_382853 = s_183_0;
        // N s_183_2: jump b184
        return block_184(state, tracer, fn_state);
    }
    fn block_184<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_184_0: read-var gs#382853:u8
        let s_184_0: bool = fn_state.gs_382853;
        // N s_184_1: branch s_184_0 b519 b185
        if s_184_0 {
            return block_519(state, tracer, fn_state);
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
        // D s_185_1: write-var gs#382855 <= s_185_0
        fn_state.gs_382855 = s_185_0;
        // N s_185_2: jump b186
        return block_186(state, tracer, fn_state);
    }
    fn block_186<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_186_0: read-var gs#382855:u8
        let s_186_0: bool = fn_state.gs_382855;
        // D s_186_1: not s_186_0
        let s_186_1: bool = !s_186_0;
        // N s_186_2: branch s_186_1 b188 b187
        if s_186_1 {
            return block_188(state, tracer, fn_state);
        } else {
            return block_187(state, tracer, fn_state);
        };
    }
    fn block_187<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_187_0: const #122s : i
        let s_187_0: i128 = 122;
        // C s_187_1: const #14696u : u32
        let s_187_1: u32 = 14696;
        // N s_187_2: write-reg s_187_1 <= s_187_0
        let s_187_2: () = {
            state.write_register::<i128>(s_187_1 as isize, s_187_0);
            tracer.write_register(s_187_1 as isize, s_187_0);
        };
        // C s_187_3: const #0s : i
        let s_187_3: i128 = 0;
        // C s_187_4: const #4s : i
        let s_187_4: i128 = 4;
        // D s_187_5: read-var u#25930:u32
        let s_187_5: u32 = fn_state.u_25930;
        // D s_187_6: cast zx s_187_5 -> bv
        let s_187_6: Bits = Bits::new(s_187_5 as u128, 32u16);
        // D s_187_7: bit-extract s_187_6 s_187_3 s_187_4
        let s_187_7: Bits = (Bits::new(
            ((s_187_6) >> (s_187_3)).value(),
            u16::try_from(s_187_4).unwrap(),
        ));
        // D s_187_8: cast reint s_187_7 -> u8
        let s_187_8: u8 = (s_187_7.value() as u8);
        // C s_187_9: const #5s : i
        let s_187_9: i128 = 5;
        // C s_187_10: const #5s : i
        let s_187_10: i128 = 5;
        // D s_187_11: read-var u#25930:u32
        let s_187_11: u32 = fn_state.u_25930;
        // D s_187_12: cast zx s_187_11 -> bv
        let s_187_12: Bits = Bits::new(s_187_11 as u128, 32u16);
        // D s_187_13: bit-extract s_187_12 s_187_9 s_187_10
        let s_187_13: Bits = (Bits::new(
            ((s_187_12) >> (s_187_9)).value(),
            u16::try_from(s_187_10).unwrap(),
        ));
        // D s_187_14: cast reint s_187_13 -> u8
        let s_187_14: u8 = (s_187_13.value() as u8);
        // C s_187_15: const #12s : i
        let s_187_15: i128 = 12;
        // C s_187_16: const #4s : i
        let s_187_16: i128 = 4;
        // D s_187_17: read-var u#25930:u32
        let s_187_17: u32 = fn_state.u_25930;
        // D s_187_18: cast zx s_187_17 -> bv
        let s_187_18: Bits = Bits::new(s_187_17 as u128, 32u16);
        // D s_187_19: bit-extract s_187_18 s_187_15 s_187_16
        let s_187_19: Bits = (Bits::new(
            ((s_187_18) >> (s_187_15)).value(),
            u16::try_from(s_187_16).unwrap(),
        ));
        // D s_187_20: cast reint s_187_19 -> u8
        let s_187_20: u8 = (s_187_19.value() as u8);
        // C s_187_21: const #16s : i
        let s_187_21: i128 = 16;
        // C s_187_22: const #5s : i
        let s_187_22: i128 = 5;
        // D s_187_23: read-var u#25930:u32
        let s_187_23: u32 = fn_state.u_25930;
        // D s_187_24: cast zx s_187_23 -> bv
        let s_187_24: Bits = Bits::new(s_187_23 as u128, 32u16);
        // D s_187_25: bit-extract s_187_24 s_187_21 s_187_22
        let s_187_25: Bits = (Bits::new(
            ((s_187_24) >> (s_187_21)).value(),
            u16::try_from(s_187_22).unwrap(),
        ));
        // D s_187_26: cast reint s_187_25 -> u8
        let s_187_26: u8 = (s_187_25.value() as u8);
        // C s_187_27: const #30s : i
        let s_187_27: i128 = 30;
        // C s_187_28: const #1s : i
        let s_187_28: i128 = 1;
        // D s_187_29: read-var u#25930:u32
        let s_187_29: u32 = fn_state.u_25930;
        // D s_187_30: cast zx s_187_29 -> bv
        let s_187_30: Bits = Bits::new(s_187_29 as u128, 32u16);
        // D s_187_31: bit-extract s_187_30 s_187_27 s_187_28
        let s_187_31: Bits = (Bits::new(
            ((s_187_30) >> (s_187_27)).value(),
            u16::try_from(s_187_28).unwrap(),
        ));
        // D s_187_32: cast reint s_187_31 -> u8
        let s_187_32: bool = ((s_187_31.value()) != 0);
        // C s_187_33: const #31s : i
        let s_187_33: i128 = 31;
        // C s_187_34: const #1s : i
        let s_187_34: i128 = 1;
        // D s_187_35: read-var u#25930:u32
        let s_187_35: u32 = fn_state.u_25930;
        // D s_187_36: cast zx s_187_35 -> bv
        let s_187_36: Bits = Bits::new(s_187_35 as u128, 32u16);
        // D s_187_37: bit-extract s_187_36 s_187_33 s_187_34
        let s_187_37: Bits = (Bits::new(
            ((s_187_36) >> (s_187_33)).value(),
            u16::try_from(s_187_34).unwrap(),
        ));
        // D s_187_38: cast reint s_187_37 -> u8
        let s_187_38: bool = ((s_187_37.value()) != 0);
        // D s_187_39: call decode_ccmp_imm_aarch64_instrs_integer_conditional_compare_immediate(s_187_8, s_187_14, s_187_20, s_187_26, s_187_32, s_187_38)
        let s_187_39: () = decode_ccmp_imm_aarch64_instrs_integer_conditional_compare_immediate(
            state,
            tracer,
            s_187_8,
            s_187_14,
            s_187_20,
            s_187_26,
            s_187_32,
            s_187_38,
        );
        // N s_187_40: return
        return;
    }
    fn block_188<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_188_0: read-var merge#var.1:struct
        let s_188_0: u32 = fn_state.merge_var._1;
        // D s_188_1: write-var u#25938 <= s_188_0
        fn_state.u_25938 = s_188_0;
        // C s_188_2: const #10s : i
        let s_188_2: i128 = 10;
        // D s_188_3: read-var u#25938:u32
        let s_188_3: u32 = fn_state.u_25938;
        // D s_188_4: cast zx s_188_3 -> bv
        let s_188_4: Bits = Bits::new(s_188_3 as u128, 32u16);
        // C s_188_5: const #1s : i64
        let s_188_5: i64 = 1;
        // C s_188_6: cast zx s_188_5 -> i
        let s_188_6: i128 = (i128::try_from(s_188_5).unwrap());
        // C s_188_7: const #20s : i
        let s_188_7: i128 = 20;
        // C s_188_8: add s_188_7 s_188_6
        let s_188_8: i128 = (s_188_7 + s_188_6);
        // D s_188_9: bit-extract s_188_4 s_188_2 s_188_8
        let s_188_9: Bits = (Bits::new(
            ((s_188_4) >> (s_188_2)).value(),
            u16::try_from(s_188_8).unwrap(),
        ));
        // D s_188_10: cast reint s_188_9 -> u21
        let s_188_10: u32 = (s_188_9.value() as u32);
        // D s_188_11: cast zx s_188_10 -> bv
        let s_188_11: Bits = Bits::new(s_188_10 as u128, 21u16);
        // C s_188_12: const #1486853u : u21
        let s_188_12: u32 = 1486853;
        // C s_188_13: cast zx s_188_12 -> bv
        let s_188_13: Bits = Bits::new(s_188_12 as u128, 21u16);
        // D s_188_14: cmp-eq s_188_11 s_188_13
        let s_188_14: bool = ((s_188_11) == (s_188_13));
        // N s_188_15: branch s_188_14 b518 b189
        if s_188_14 {
            return block_518(state, tracer, fn_state);
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
        // D s_189_1: write-var gs#382874 <= s_189_0
        fn_state.gs_382874 = s_189_0;
        // N s_189_2: jump b190
        return block_190(state, tracer, fn_state);
    }
    fn block_190<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_190_0: read-var gs#382874:u8
        let s_190_0: bool = fn_state.gs_382874;
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
        // C s_191_0: const #127s : i
        let s_191_0: i128 = 127;
        // C s_191_1: const #14696u : u32
        let s_191_1: u32 = 14696;
        // N s_191_2: write-reg s_191_1 <= s_191_0
        let s_191_2: () = {
            state.write_register::<i128>(s_191_1 as isize, s_191_0);
            tracer.write_register(s_191_1 as isize, s_191_0);
        };
        // C s_191_3: const #0s : i
        let s_191_3: i128 = 0;
        // C s_191_4: const #5s : i
        let s_191_4: i128 = 5;
        // D s_191_5: read-var u#25938:u32
        let s_191_5: u32 = fn_state.u_25938;
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
        // C s_191_10: const #5s : i
        let s_191_10: i128 = 5;
        // D s_191_11: read-var u#25938:u32
        let s_191_11: u32 = fn_state.u_25938;
        // D s_191_12: cast zx s_191_11 -> bv
        let s_191_12: Bits = Bits::new(s_191_11 as u128, 32u16);
        // D s_191_13: bit-extract s_191_12 s_191_9 s_191_10
        let s_191_13: Bits = (Bits::new(
            ((s_191_12) >> (s_191_9)).value(),
            u16::try_from(s_191_10).unwrap(),
        ));
        // D s_191_14: cast reint s_191_13 -> u8
        let s_191_14: u8 = (s_191_13.value() as u8);
        // C s_191_15: const #10s : i
        let s_191_15: i128 = 10;
        // C s_191_16: const #1s : i
        let s_191_16: i128 = 1;
        // D s_191_17: read-var u#25938:u32
        let s_191_17: u32 = fn_state.u_25938;
        // D s_191_18: cast zx s_191_17 -> bv
        let s_191_18: Bits = Bits::new(s_191_17 as u128, 32u16);
        // D s_191_19: bit-extract s_191_18 s_191_15 s_191_16
        let s_191_19: Bits = (Bits::new(
            ((s_191_18) >> (s_191_15)).value(),
            u16::try_from(s_191_16).unwrap(),
        ));
        // D s_191_20: cast reint s_191_19 -> u8
        let s_191_20: bool = ((s_191_19.value()) != 0);
        // C s_191_21: const #31s : i
        let s_191_21: i128 = 31;
        // C s_191_22: const #1s : i
        let s_191_22: i128 = 1;
        // D s_191_23: read-var u#25938:u32
        let s_191_23: u32 = fn_state.u_25938;
        // D s_191_24: cast zx s_191_23 -> bv
        let s_191_24: Bits = Bits::new(s_191_23 as u128, 32u16);
        // D s_191_25: bit-extract s_191_24 s_191_21 s_191_22
        let s_191_25: Bits = (Bits::new(
            ((s_191_24) >> (s_191_21)).value(),
            u16::try_from(s_191_22).unwrap(),
        ));
        // D s_191_26: cast reint s_191_25 -> u8
        let s_191_26: bool = ((s_191_25.value()) != 0);
        // D s_191_27: call decode_cls_int_aarch64_instrs_integer_arithmetic_cnt(s_191_8, s_191_14, s_191_20, s_191_26)
        let s_191_27: () = decode_cls_int_aarch64_instrs_integer_arithmetic_cnt(
            state,
            tracer,
            s_191_8,
            s_191_14,
            s_191_20,
            s_191_26,
        );
        // N s_191_28: return
        return;
    }
    fn block_192<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_192_0: read-var merge#var.1:struct
        let s_192_0: u32 = fn_state.merge_var._1;
        // D s_192_1: write-var u#25944 <= s_192_0
        fn_state.u_25944 = s_192_0;
        // C s_192_2: const #10s : i
        let s_192_2: i128 = 10;
        // D s_192_3: read-var u#25944:u32
        let s_192_3: u32 = fn_state.u_25944;
        // D s_192_4: cast zx s_192_3 -> bv
        let s_192_4: Bits = Bits::new(s_192_3 as u128, 32u16);
        // C s_192_5: const #1s : i64
        let s_192_5: i64 = 1;
        // C s_192_6: cast zx s_192_5 -> i
        let s_192_6: i128 = (i128::try_from(s_192_5).unwrap());
        // C s_192_7: const #20s : i
        let s_192_7: i128 = 20;
        // C s_192_8: add s_192_7 s_192_6
        let s_192_8: i128 = (s_192_7 + s_192_6);
        // D s_192_9: bit-extract s_192_4 s_192_2 s_192_8
        let s_192_9: Bits = (Bits::new(
            ((s_192_4) >> (s_192_2)).value(),
            u16::try_from(s_192_8).unwrap(),
        ));
        // D s_192_10: cast reint s_192_9 -> u21
        let s_192_10: u32 = (s_192_9.value() as u32);
        // D s_192_11: cast zx s_192_10 -> bv
        let s_192_11: Bits = Bits::new(s_192_10 as u128, 21u16);
        // C s_192_12: const #1486852u : u21
        let s_192_12: u32 = 1486852;
        // C s_192_13: cast zx s_192_12 -> bv
        let s_192_13: Bits = Bits::new(s_192_12 as u128, 21u16);
        // D s_192_14: cmp-eq s_192_11 s_192_13
        let s_192_14: bool = ((s_192_11) == (s_192_13));
        // N s_192_15: branch s_192_14 b517 b193
        if s_192_14 {
            return block_517(state, tracer, fn_state);
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
        // D s_193_1: write-var gs#382889 <= s_193_0
        fn_state.gs_382889 = s_193_0;
        // N s_193_2: jump b194
        return block_194(state, tracer, fn_state);
    }
    fn block_194<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_194_0: read-var gs#382889:u8
        let s_194_0: bool = fn_state.gs_382889;
        // D s_194_1: not s_194_0
        let s_194_1: bool = !s_194_0;
        // N s_194_2: branch s_194_1 b196 b195
        if s_194_1 {
            return block_196(state, tracer, fn_state);
        } else {
            return block_195(state, tracer, fn_state);
        };
    }
    fn block_195<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_195_0: const #128s : i
        let s_195_0: i128 = 128;
        // C s_195_1: const #14696u : u32
        let s_195_1: u32 = 14696;
        // N s_195_2: write-reg s_195_1 <= s_195_0
        let s_195_2: () = {
            state.write_register::<i128>(s_195_1 as isize, s_195_0);
            tracer.write_register(s_195_1 as isize, s_195_0);
        };
        // C s_195_3: const #0s : i
        let s_195_3: i128 = 0;
        // C s_195_4: const #5s : i
        let s_195_4: i128 = 5;
        // D s_195_5: read-var u#25944:u32
        let s_195_5: u32 = fn_state.u_25944;
        // D s_195_6: cast zx s_195_5 -> bv
        let s_195_6: Bits = Bits::new(s_195_5 as u128, 32u16);
        // D s_195_7: bit-extract s_195_6 s_195_3 s_195_4
        let s_195_7: Bits = (Bits::new(
            ((s_195_6) >> (s_195_3)).value(),
            u16::try_from(s_195_4).unwrap(),
        ));
        // D s_195_8: cast reint s_195_7 -> u8
        let s_195_8: u8 = (s_195_7.value() as u8);
        // C s_195_9: const #5s : i
        let s_195_9: i128 = 5;
        // C s_195_10: const #5s : i
        let s_195_10: i128 = 5;
        // D s_195_11: read-var u#25944:u32
        let s_195_11: u32 = fn_state.u_25944;
        // D s_195_12: cast zx s_195_11 -> bv
        let s_195_12: Bits = Bits::new(s_195_11 as u128, 32u16);
        // D s_195_13: bit-extract s_195_12 s_195_9 s_195_10
        let s_195_13: Bits = (Bits::new(
            ((s_195_12) >> (s_195_9)).value(),
            u16::try_from(s_195_10).unwrap(),
        ));
        // D s_195_14: cast reint s_195_13 -> u8
        let s_195_14: u8 = (s_195_13.value() as u8);
        // C s_195_15: const #10s : i
        let s_195_15: i128 = 10;
        // C s_195_16: const #1s : i
        let s_195_16: i128 = 1;
        // D s_195_17: read-var u#25944:u32
        let s_195_17: u32 = fn_state.u_25944;
        // D s_195_18: cast zx s_195_17 -> bv
        let s_195_18: Bits = Bits::new(s_195_17 as u128, 32u16);
        // D s_195_19: bit-extract s_195_18 s_195_15 s_195_16
        let s_195_19: Bits = (Bits::new(
            ((s_195_18) >> (s_195_15)).value(),
            u16::try_from(s_195_16).unwrap(),
        ));
        // D s_195_20: cast reint s_195_19 -> u8
        let s_195_20: bool = ((s_195_19.value()) != 0);
        // C s_195_21: const #31s : i
        let s_195_21: i128 = 31;
        // C s_195_22: const #1s : i
        let s_195_22: i128 = 1;
        // D s_195_23: read-var u#25944:u32
        let s_195_23: u32 = fn_state.u_25944;
        // D s_195_24: cast zx s_195_23 -> bv
        let s_195_24: Bits = Bits::new(s_195_23 as u128, 32u16);
        // D s_195_25: bit-extract s_195_24 s_195_21 s_195_22
        let s_195_25: Bits = (Bits::new(
            ((s_195_24) >> (s_195_21)).value(),
            u16::try_from(s_195_22).unwrap(),
        ));
        // D s_195_26: cast reint s_195_25 -> u8
        let s_195_26: bool = ((s_195_25.value()) != 0);
        // D s_195_27: call decode_clz_int_aarch64_instrs_integer_arithmetic_cnt(s_195_8, s_195_14, s_195_20, s_195_26)
        let s_195_27: () = decode_clz_int_aarch64_instrs_integer_arithmetic_cnt(
            state,
            tracer,
            s_195_8,
            s_195_14,
            s_195_20,
            s_195_26,
        );
        // N s_195_28: return
        return;
    }
    fn block_196<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_196_0: read-var merge#var.1:struct
        let s_196_0: u32 = fn_state.merge_var._1;
        // D s_196_1: write-var u#25950 <= s_196_0
        fn_state.u_25950 = s_196_0;
        // C s_196_2: const #10s : i
        let s_196_2: i128 = 10;
        // D s_196_3: read-var u#25950:u32
        let s_196_3: u32 = fn_state.u_25950;
        // D s_196_4: cast zx s_196_3 -> bv
        let s_196_4: Bits = Bits::new(s_196_3 as u128, 32u16);
        // C s_196_5: const #1s : i64
        let s_196_5: i64 = 1;
        // C s_196_6: cast zx s_196_5 -> i
        let s_196_6: i128 = (i128::try_from(s_196_5).unwrap());
        // C s_196_7: const #20s : i
        let s_196_7: i128 = 20;
        // C s_196_8: add s_196_7 s_196_6
        let s_196_8: i128 = (s_196_7 + s_196_6);
        // D s_196_9: bit-extract s_196_4 s_196_2 s_196_8
        let s_196_9: Bits = (Bits::new(
            ((s_196_4) >> (s_196_2)).value(),
            u16::try_from(s_196_8).unwrap(),
        ));
        // D s_196_10: cast reint s_196_9 -> u21
        let s_196_10: u32 = (s_196_9.value() as u32);
        // D s_196_11: cast zx s_196_10 -> bv
        let s_196_11: Bits = Bits::new(s_196_10 as u128, 21u16);
        // C s_196_12: const #1486855u : u21
        let s_196_12: u32 = 1486855;
        // C s_196_13: cast zx s_196_12 -> bv
        let s_196_13: Bits = Bits::new(s_196_12 as u128, 21u16);
        // D s_196_14: cmp-eq s_196_11 s_196_13
        let s_196_14: bool = ((s_196_11) == (s_196_13));
        // N s_196_15: branch s_196_14 b516 b197
        if s_196_14 {
            return block_516(state, tracer, fn_state);
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
        // D s_197_1: write-var gs#382904 <= s_197_0
        fn_state.gs_382904 = s_197_0;
        // N s_197_2: jump b198
        return block_198(state, tracer, fn_state);
    }
    fn block_198<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_198_0: read-var gs#382904:u8
        let s_198_0: bool = fn_state.gs_382904;
        // D s_198_1: not s_198_0
        let s_198_1: bool = !s_198_0;
        // N s_198_2: branch s_198_1 b200 b199
        if s_198_1 {
            return block_200(state, tracer, fn_state);
        } else {
            return block_199(state, tracer, fn_state);
        };
    }
    fn block_199<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_199_0: const #151s : i
        let s_199_0: i128 = 151;
        // C s_199_1: const #14696u : u32
        let s_199_1: u32 = 14696;
        // N s_199_2: write-reg s_199_1 <= s_199_0
        let s_199_2: () = {
            state.write_register::<i128>(s_199_1 as isize, s_199_0);
            tracer.write_register(s_199_1 as isize, s_199_0);
        };
        // C s_199_3: const #0s : i
        let s_199_3: i128 = 0;
        // C s_199_4: const #5s : i
        let s_199_4: i128 = 5;
        // D s_199_5: read-var u#25950:u32
        let s_199_5: u32 = fn_state.u_25950;
        // D s_199_6: cast zx s_199_5 -> bv
        let s_199_6: Bits = Bits::new(s_199_5 as u128, 32u16);
        // D s_199_7: bit-extract s_199_6 s_199_3 s_199_4
        let s_199_7: Bits = (Bits::new(
            ((s_199_6) >> (s_199_3)).value(),
            u16::try_from(s_199_4).unwrap(),
        ));
        // D s_199_8: cast reint s_199_7 -> u8
        let s_199_8: u8 = (s_199_7.value() as u8);
        // C s_199_9: const #5s : i
        let s_199_9: i128 = 5;
        // C s_199_10: const #5s : i
        let s_199_10: i128 = 5;
        // D s_199_11: read-var u#25950:u32
        let s_199_11: u32 = fn_state.u_25950;
        // D s_199_12: cast zx s_199_11 -> bv
        let s_199_12: Bits = Bits::new(s_199_11 as u128, 32u16);
        // D s_199_13: bit-extract s_199_12 s_199_9 s_199_10
        let s_199_13: Bits = (Bits::new(
            ((s_199_12) >> (s_199_9)).value(),
            u16::try_from(s_199_10).unwrap(),
        ));
        // D s_199_14: cast reint s_199_13 -> u8
        let s_199_14: u8 = (s_199_13.value() as u8);
        // C s_199_15: const #31s : i
        let s_199_15: i128 = 31;
        // C s_199_16: const #1s : i
        let s_199_16: i128 = 1;
        // D s_199_17: read-var u#25950:u32
        let s_199_17: u32 = fn_state.u_25950;
        // D s_199_18: cast zx s_199_17 -> bv
        let s_199_18: Bits = Bits::new(s_199_17 as u128, 32u16);
        // D s_199_19: bit-extract s_199_18 s_199_15 s_199_16
        let s_199_19: Bits = (Bits::new(
            ((s_199_18) >> (s_199_15)).value(),
            u16::try_from(s_199_16).unwrap(),
        ));
        // D s_199_20: cast reint s_199_19 -> u8
        let s_199_20: bool = ((s_199_19.value()) != 0);
        // D s_199_21: call decode_cnt_aarch64_instrs_integer_arithmetic_unary_cnt(s_199_8, s_199_14, s_199_20)
        let s_199_21: () = decode_cnt_aarch64_instrs_integer_arithmetic_unary_cnt(
            state,
            tracer,
            s_199_8,
            s_199_14,
            s_199_20,
        );
        // N s_199_22: return
        return;
    }
    fn block_200<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_200_0: read-var merge#var.1:struct
        let s_200_0: u32 = fn_state.merge_var._1;
        // D s_200_1: write-var u#25955 <= s_200_0
        fn_state.u_25955 = s_200_0;
        // C s_200_2: const #21s : i
        let s_200_2: i128 = 21;
        // D s_200_3: read-var u#25955:u32
        let s_200_3: u32 = fn_state.u_25955;
        // D s_200_4: cast zx s_200_3 -> bv
        let s_200_4: Bits = Bits::new(s_200_3 as u128, 32u16);
        // C s_200_5: const #1s : i64
        let s_200_5: i64 = 1;
        // C s_200_6: cast zx s_200_5 -> i
        let s_200_6: i128 = (i128::try_from(s_200_5).unwrap());
        // C s_200_7: const #9s : i
        let s_200_7: i128 = 9;
        // C s_200_8: add s_200_7 s_200_6
        let s_200_8: i128 = (s_200_7 + s_200_6);
        // D s_200_9: bit-extract s_200_4 s_200_2 s_200_8
        let s_200_9: Bits = (Bits::new(
            ((s_200_4) >> (s_200_2)).value(),
            u16::try_from(s_200_8).unwrap(),
        ));
        // D s_200_10: cast reint s_200_9 -> u10
        let s_200_10: u16 = (s_200_9.value() as u16);
        // D s_200_11: cast zx s_200_10 -> bv
        let s_200_11: Bits = Bits::new(s_200_10 as u128, 10u16);
        // C s_200_12: const #214u : u10
        let s_200_12: u16 = 214;
        // C s_200_13: cast zx s_200_12 -> bv
        let s_200_13: Bits = Bits::new(s_200_12 as u128, 10u16);
        // D s_200_14: cmp-eq s_200_11 s_200_13
        let s_200_14: bool = ((s_200_11) == (s_200_13));
        // N s_200_15: branch s_200_14 b515 b201
        if s_200_14 {
            return block_515(state, tracer, fn_state);
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
        // D s_201_1: write-var gs#382918 <= s_201_0
        fn_state.gs_382918 = s_201_0;
        // N s_201_2: jump b202
        return block_202(state, tracer, fn_state);
    }
    fn block_202<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_202_0: read-var gs#382918:u8
        let s_202_0: bool = fn_state.gs_382918;
        // N s_202_1: branch s_202_0 b514 b203
        if s_202_0 {
            return block_514(state, tracer, fn_state);
        } else {
            return block_203(state, tracer, fn_state);
        };
    }
    fn block_203<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_203_0: const #0u : u8
        let s_203_0: bool = false;
        // D s_203_1: write-var gs#382920 <= s_203_0
        fn_state.gs_382920 = s_203_0;
        // N s_203_2: jump b204
        return block_204(state, tracer, fn_state);
    }
    fn block_204<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_204_0: read-var gs#382920:u8
        let s_204_0: bool = fn_state.gs_382920;
        // D s_204_1: not s_204_0
        let s_204_1: bool = !s_204_0;
        // N s_204_2: branch s_204_1 b206 b205
        if s_204_1 {
            return block_206(state, tracer, fn_state);
        } else {
            return block_205(state, tracer, fn_state);
        };
    }
    fn block_205<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_205_0: const #185s : i
        let s_205_0: i128 = 185;
        // C s_205_1: const #14696u : u32
        let s_205_1: u32 = 14696;
        // N s_205_2: write-reg s_205_1 <= s_205_0
        let s_205_2: () = {
            state.write_register::<i128>(s_205_1 as isize, s_205_0);
            tracer.write_register(s_205_1 as isize, s_205_0);
        };
        // C s_205_3: const #0s : i
        let s_205_3: i128 = 0;
        // C s_205_4: const #5s : i
        let s_205_4: i128 = 5;
        // D s_205_5: read-var u#25955:u32
        let s_205_5: u32 = fn_state.u_25955;
        // D s_205_6: cast zx s_205_5 -> bv
        let s_205_6: Bits = Bits::new(s_205_5 as u128, 32u16);
        // D s_205_7: bit-extract s_205_6 s_205_3 s_205_4
        let s_205_7: Bits = (Bits::new(
            ((s_205_6) >> (s_205_3)).value(),
            u16::try_from(s_205_4).unwrap(),
        ));
        // D s_205_8: cast reint s_205_7 -> u8
        let s_205_8: u8 = (s_205_7.value() as u8);
        // C s_205_9: const #5s : i
        let s_205_9: i128 = 5;
        // C s_205_10: const #5s : i
        let s_205_10: i128 = 5;
        // D s_205_11: read-var u#25955:u32
        let s_205_11: u32 = fn_state.u_25955;
        // D s_205_12: cast zx s_205_11 -> bv
        let s_205_12: Bits = Bits::new(s_205_11 as u128, 32u16);
        // D s_205_13: bit-extract s_205_12 s_205_9 s_205_10
        let s_205_13: Bits = (Bits::new(
            ((s_205_12) >> (s_205_9)).value(),
            u16::try_from(s_205_10).unwrap(),
        ));
        // D s_205_14: cast reint s_205_13 -> u8
        let s_205_14: u8 = (s_205_13.value() as u8);
        // C s_205_15: const #10s : i
        let s_205_15: i128 = 10;
        // C s_205_16: const #2s : i
        let s_205_16: i128 = 2;
        // D s_205_17: read-var u#25955:u32
        let s_205_17: u32 = fn_state.u_25955;
        // D s_205_18: cast zx s_205_17 -> bv
        let s_205_18: Bits = Bits::new(s_205_17 as u128, 32u16);
        // D s_205_19: bit-extract s_205_18 s_205_15 s_205_16
        let s_205_19: Bits = (Bits::new(
            ((s_205_18) >> (s_205_15)).value(),
            u16::try_from(s_205_16).unwrap(),
        ));
        // D s_205_20: cast reint s_205_19 -> u8
        let s_205_20: u8 = (s_205_19.value() as u8);
        // C s_205_21: const #12s : i
        let s_205_21: i128 = 12;
        // C s_205_22: const #1s : i
        let s_205_22: i128 = 1;
        // D s_205_23: read-var u#25955:u32
        let s_205_23: u32 = fn_state.u_25955;
        // D s_205_24: cast zx s_205_23 -> bv
        let s_205_24: Bits = Bits::new(s_205_23 as u128, 32u16);
        // D s_205_25: bit-extract s_205_24 s_205_21 s_205_22
        let s_205_25: Bits = (Bits::new(
            ((s_205_24) >> (s_205_21)).value(),
            u16::try_from(s_205_22).unwrap(),
        ));
        // D s_205_26: cast reint s_205_25 -> u8
        let s_205_26: bool = ((s_205_25.value()) != 0);
        // C s_205_27: const #16s : i
        let s_205_27: i128 = 16;
        // C s_205_28: const #5s : i
        let s_205_28: i128 = 5;
        // D s_205_29: read-var u#25955:u32
        let s_205_29: u32 = fn_state.u_25955;
        // D s_205_30: cast zx s_205_29 -> bv
        let s_205_30: Bits = Bits::new(s_205_29 as u128, 32u16);
        // D s_205_31: bit-extract s_205_30 s_205_27 s_205_28
        let s_205_31: Bits = (Bits::new(
            ((s_205_30) >> (s_205_27)).value(),
            u16::try_from(s_205_28).unwrap(),
        ));
        // D s_205_32: cast reint s_205_31 -> u8
        let s_205_32: u8 = (s_205_31.value() as u8);
        // C s_205_33: const #31s : i
        let s_205_33: i128 = 31;
        // C s_205_34: const #1s : i
        let s_205_34: i128 = 1;
        // D s_205_35: read-var u#25955:u32
        let s_205_35: u32 = fn_state.u_25955;
        // D s_205_36: cast zx s_205_35 -> bv
        let s_205_36: Bits = Bits::new(s_205_35 as u128, 32u16);
        // D s_205_37: bit-extract s_205_36 s_205_33 s_205_34
        let s_205_37: Bits = (Bits::new(
            ((s_205_36) >> (s_205_33)).value(),
            u16::try_from(s_205_34).unwrap(),
        ));
        // D s_205_38: cast reint s_205_37 -> u8
        let s_205_38: bool = ((s_205_37.value()) != 0);
        // D s_205_39: call decode_crc32_aarch64_instrs_integer_crc(s_205_8, s_205_14, s_205_20, s_205_26, s_205_32, s_205_38)
        let s_205_39: () = decode_crc32_aarch64_instrs_integer_crc(
            state,
            tracer,
            s_205_8,
            s_205_14,
            s_205_20,
            s_205_26,
            s_205_32,
            s_205_38,
        );
        // N s_205_40: return
        return;
    }
    fn block_206<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_206_0: read-var merge#var.1:struct
        let s_206_0: u32 = fn_state.merge_var._1;
        // D s_206_1: write-var u#25961 <= s_206_0
        fn_state.u_25961 = s_206_0;
        // C s_206_2: const #21s : i
        let s_206_2: i128 = 21;
        // D s_206_3: read-var u#25961:u32
        let s_206_3: u32 = fn_state.u_25961;
        // D s_206_4: cast zx s_206_3 -> bv
        let s_206_4: Bits = Bits::new(s_206_3 as u128, 32u16);
        // C s_206_5: const #1s : i64
        let s_206_5: i64 = 1;
        // C s_206_6: cast zx s_206_5 -> i
        let s_206_6: i128 = (i128::try_from(s_206_5).unwrap());
        // C s_206_7: const #9s : i
        let s_206_7: i128 = 9;
        // C s_206_8: add s_206_7 s_206_6
        let s_206_8: i128 = (s_206_7 + s_206_6);
        // D s_206_9: bit-extract s_206_4 s_206_2 s_206_8
        let s_206_9: Bits = (Bits::new(
            ((s_206_4) >> (s_206_2)).value(),
            u16::try_from(s_206_8).unwrap(),
        ));
        // D s_206_10: cast reint s_206_9 -> u10
        let s_206_10: u16 = (s_206_9.value() as u16);
        // D s_206_11: cast zx s_206_10 -> bv
        let s_206_11: Bits = Bits::new(s_206_10 as u128, 10u16);
        // C s_206_12: const #214u : u10
        let s_206_12: u16 = 214;
        // C s_206_13: cast zx s_206_12 -> bv
        let s_206_13: Bits = Bits::new(s_206_12 as u128, 10u16);
        // D s_206_14: cmp-eq s_206_11 s_206_13
        let s_206_14: bool = ((s_206_11) == (s_206_13));
        // N s_206_15: branch s_206_14 b513 b207
        if s_206_14 {
            return block_513(state, tracer, fn_state);
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
        // D s_207_1: write-var gs#382940 <= s_207_0
        fn_state.gs_382940 = s_207_0;
        // N s_207_2: jump b208
        return block_208(state, tracer, fn_state);
    }
    fn block_208<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_208_0: read-var gs#382940:u8
        let s_208_0: bool = fn_state.gs_382940;
        // N s_208_1: branch s_208_0 b512 b209
        if s_208_0 {
            return block_512(state, tracer, fn_state);
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
        // D s_209_1: write-var gs#382942 <= s_209_0
        fn_state.gs_382942 = s_209_0;
        // N s_209_2: jump b210
        return block_210(state, tracer, fn_state);
    }
    fn block_210<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_210_0: read-var gs#382942:u8
        let s_210_0: bool = fn_state.gs_382942;
        // D s_210_1: not s_210_0
        let s_210_1: bool = !s_210_0;
        // N s_210_2: branch s_210_1 b212 b211
        if s_210_1 {
            return block_212(state, tracer, fn_state);
        } else {
            return block_211(state, tracer, fn_state);
        };
    }
    fn block_211<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_211_0: const #186s : i
        let s_211_0: i128 = 186;
        // C s_211_1: const #14696u : u32
        let s_211_1: u32 = 14696;
        // N s_211_2: write-reg s_211_1 <= s_211_0
        let s_211_2: () = {
            state.write_register::<i128>(s_211_1 as isize, s_211_0);
            tracer.write_register(s_211_1 as isize, s_211_0);
        };
        // C s_211_3: const #0s : i
        let s_211_3: i128 = 0;
        // C s_211_4: const #5s : i
        let s_211_4: i128 = 5;
        // D s_211_5: read-var u#25961:u32
        let s_211_5: u32 = fn_state.u_25961;
        // D s_211_6: cast zx s_211_5 -> bv
        let s_211_6: Bits = Bits::new(s_211_5 as u128, 32u16);
        // D s_211_7: bit-extract s_211_6 s_211_3 s_211_4
        let s_211_7: Bits = (Bits::new(
            ((s_211_6) >> (s_211_3)).value(),
            u16::try_from(s_211_4).unwrap(),
        ));
        // D s_211_8: cast reint s_211_7 -> u8
        let s_211_8: u8 = (s_211_7.value() as u8);
        // C s_211_9: const #5s : i
        let s_211_9: i128 = 5;
        // C s_211_10: const #5s : i
        let s_211_10: i128 = 5;
        // D s_211_11: read-var u#25961:u32
        let s_211_11: u32 = fn_state.u_25961;
        // D s_211_12: cast zx s_211_11 -> bv
        let s_211_12: Bits = Bits::new(s_211_11 as u128, 32u16);
        // D s_211_13: bit-extract s_211_12 s_211_9 s_211_10
        let s_211_13: Bits = (Bits::new(
            ((s_211_12) >> (s_211_9)).value(),
            u16::try_from(s_211_10).unwrap(),
        ));
        // D s_211_14: cast reint s_211_13 -> u8
        let s_211_14: u8 = (s_211_13.value() as u8);
        // C s_211_15: const #10s : i
        let s_211_15: i128 = 10;
        // C s_211_16: const #2s : i
        let s_211_16: i128 = 2;
        // D s_211_17: read-var u#25961:u32
        let s_211_17: u32 = fn_state.u_25961;
        // D s_211_18: cast zx s_211_17 -> bv
        let s_211_18: Bits = Bits::new(s_211_17 as u128, 32u16);
        // D s_211_19: bit-extract s_211_18 s_211_15 s_211_16
        let s_211_19: Bits = (Bits::new(
            ((s_211_18) >> (s_211_15)).value(),
            u16::try_from(s_211_16).unwrap(),
        ));
        // D s_211_20: cast reint s_211_19 -> u8
        let s_211_20: u8 = (s_211_19.value() as u8);
        // C s_211_21: const #12s : i
        let s_211_21: i128 = 12;
        // C s_211_22: const #1s : i
        let s_211_22: i128 = 1;
        // D s_211_23: read-var u#25961:u32
        let s_211_23: u32 = fn_state.u_25961;
        // D s_211_24: cast zx s_211_23 -> bv
        let s_211_24: Bits = Bits::new(s_211_23 as u128, 32u16);
        // D s_211_25: bit-extract s_211_24 s_211_21 s_211_22
        let s_211_25: Bits = (Bits::new(
            ((s_211_24) >> (s_211_21)).value(),
            u16::try_from(s_211_22).unwrap(),
        ));
        // D s_211_26: cast reint s_211_25 -> u8
        let s_211_26: bool = ((s_211_25.value()) != 0);
        // C s_211_27: const #16s : i
        let s_211_27: i128 = 16;
        // C s_211_28: const #5s : i
        let s_211_28: i128 = 5;
        // D s_211_29: read-var u#25961:u32
        let s_211_29: u32 = fn_state.u_25961;
        // D s_211_30: cast zx s_211_29 -> bv
        let s_211_30: Bits = Bits::new(s_211_29 as u128, 32u16);
        // D s_211_31: bit-extract s_211_30 s_211_27 s_211_28
        let s_211_31: Bits = (Bits::new(
            ((s_211_30) >> (s_211_27)).value(),
            u16::try_from(s_211_28).unwrap(),
        ));
        // D s_211_32: cast reint s_211_31 -> u8
        let s_211_32: u8 = (s_211_31.value() as u8);
        // C s_211_33: const #31s : i
        let s_211_33: i128 = 31;
        // C s_211_34: const #1s : i
        let s_211_34: i128 = 1;
        // D s_211_35: read-var u#25961:u32
        let s_211_35: u32 = fn_state.u_25961;
        // D s_211_36: cast zx s_211_35 -> bv
        let s_211_36: Bits = Bits::new(s_211_35 as u128, 32u16);
        // D s_211_37: bit-extract s_211_36 s_211_33 s_211_34
        let s_211_37: Bits = (Bits::new(
            ((s_211_36) >> (s_211_33)).value(),
            u16::try_from(s_211_34).unwrap(),
        ));
        // D s_211_38: cast reint s_211_37 -> u8
        let s_211_38: bool = ((s_211_37.value()) != 0);
        // D s_211_39: call decode_crc32c_aarch64_instrs_integer_crc(s_211_8, s_211_14, s_211_20, s_211_26, s_211_32, s_211_38)
        let s_211_39: () = decode_crc32c_aarch64_instrs_integer_crc(
            state,
            tracer,
            s_211_8,
            s_211_14,
            s_211_20,
            s_211_26,
            s_211_32,
            s_211_38,
        );
        // N s_211_40: return
        return;
    }
    fn block_212<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_212_0: read-var merge#var.1:struct
        let s_212_0: u32 = fn_state.merge_var._1;
        // D s_212_1: write-var u#25969 <= s_212_0
        fn_state.u_25969 = s_212_0;
        // C s_212_2: const #21s : i
        let s_212_2: i128 = 21;
        // D s_212_3: read-var u#25969:u32
        let s_212_3: u32 = fn_state.u_25969;
        // D s_212_4: cast zx s_212_3 -> bv
        let s_212_4: Bits = Bits::new(s_212_3 as u128, 32u16);
        // C s_212_5: const #1s : i64
        let s_212_5: i64 = 1;
        // C s_212_6: cast zx s_212_5 -> i
        let s_212_6: i128 = (i128::try_from(s_212_5).unwrap());
        // C s_212_7: const #9s : i
        let s_212_7: i128 = 9;
        // C s_212_8: add s_212_7 s_212_6
        let s_212_8: i128 = (s_212_7 + s_212_6);
        // D s_212_9: bit-extract s_212_4 s_212_2 s_212_8
        let s_212_9: Bits = (Bits::new(
            ((s_212_4) >> (s_212_2)).value(),
            u16::try_from(s_212_8).unwrap(),
        ));
        // D s_212_10: cast reint s_212_9 -> u10
        let s_212_10: u16 = (s_212_9.value() as u16);
        // D s_212_11: cast zx s_212_10 -> bv
        let s_212_11: Bits = Bits::new(s_212_10 as u128, 10u16);
        // C s_212_12: const #212u : u10
        let s_212_12: u16 = 212;
        // C s_212_13: cast zx s_212_12 -> bv
        let s_212_13: Bits = Bits::new(s_212_12 as u128, 10u16);
        // D s_212_14: cmp-eq s_212_11 s_212_13
        let s_212_14: bool = ((s_212_11) == (s_212_13));
        // N s_212_15: branch s_212_14 b511 b213
        if s_212_14 {
            return block_511(state, tracer, fn_state);
        } else {
            return block_213(state, tracer, fn_state);
        };
    }
    fn block_213<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_213_0: const #0u : u8
        let s_213_0: bool = false;
        // D s_213_1: write-var gs#382962 <= s_213_0
        fn_state.gs_382962 = s_213_0;
        // N s_213_2: jump b214
        return block_214(state, tracer, fn_state);
    }
    fn block_214<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_214_0: read-var gs#382962:u8
        let s_214_0: bool = fn_state.gs_382962;
        // N s_214_1: branch s_214_0 b510 b215
        if s_214_0 {
            return block_510(state, tracer, fn_state);
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
        // D s_215_1: write-var gs#382964 <= s_215_0
        fn_state.gs_382964 = s_215_0;
        // N s_215_2: jump b216
        return block_216(state, tracer, fn_state);
    }
    fn block_216<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_216_0: read-var gs#382964:u8
        let s_216_0: bool = fn_state.gs_382964;
        // D s_216_1: not s_216_0
        let s_216_1: bool = !s_216_0;
        // N s_216_2: branch s_216_1 b218 b217
        if s_216_1 {
            return block_218(state, tracer, fn_state);
        } else {
            return block_217(state, tracer, fn_state);
        };
    }
    fn block_217<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_217_0: const #187s : i
        let s_217_0: i128 = 187;
        // C s_217_1: const #14696u : u32
        let s_217_1: u32 = 14696;
        // N s_217_2: write-reg s_217_1 <= s_217_0
        let s_217_2: () = {
            state.write_register::<i128>(s_217_1 as isize, s_217_0);
            tracer.write_register(s_217_1 as isize, s_217_0);
        };
        // C s_217_3: const #0s : i
        let s_217_3: i128 = 0;
        // C s_217_4: const #5s : i
        let s_217_4: i128 = 5;
        // D s_217_5: read-var u#25969:u32
        let s_217_5: u32 = fn_state.u_25969;
        // D s_217_6: cast zx s_217_5 -> bv
        let s_217_6: Bits = Bits::new(s_217_5 as u128, 32u16);
        // D s_217_7: bit-extract s_217_6 s_217_3 s_217_4
        let s_217_7: Bits = (Bits::new(
            ((s_217_6) >> (s_217_3)).value(),
            u16::try_from(s_217_4).unwrap(),
        ));
        // D s_217_8: cast reint s_217_7 -> u8
        let s_217_8: u8 = (s_217_7.value() as u8);
        // C s_217_9: const #5s : i
        let s_217_9: i128 = 5;
        // C s_217_10: const #5s : i
        let s_217_10: i128 = 5;
        // D s_217_11: read-var u#25969:u32
        let s_217_11: u32 = fn_state.u_25969;
        // D s_217_12: cast zx s_217_11 -> bv
        let s_217_12: Bits = Bits::new(s_217_11 as u128, 32u16);
        // D s_217_13: bit-extract s_217_12 s_217_9 s_217_10
        let s_217_13: Bits = (Bits::new(
            ((s_217_12) >> (s_217_9)).value(),
            u16::try_from(s_217_10).unwrap(),
        ));
        // D s_217_14: cast reint s_217_13 -> u8
        let s_217_14: u8 = (s_217_13.value() as u8);
        // C s_217_15: const #10s : i
        let s_217_15: i128 = 10;
        // C s_217_16: const #1s : i
        let s_217_16: i128 = 1;
        // D s_217_17: read-var u#25969:u32
        let s_217_17: u32 = fn_state.u_25969;
        // D s_217_18: cast zx s_217_17 -> bv
        let s_217_18: Bits = Bits::new(s_217_17 as u128, 32u16);
        // D s_217_19: bit-extract s_217_18 s_217_15 s_217_16
        let s_217_19: Bits = (Bits::new(
            ((s_217_18) >> (s_217_15)).value(),
            u16::try_from(s_217_16).unwrap(),
        ));
        // D s_217_20: cast reint s_217_19 -> u8
        let s_217_20: bool = ((s_217_19.value()) != 0);
        // C s_217_21: const #12s : i
        let s_217_21: i128 = 12;
        // C s_217_22: const #4s : i
        let s_217_22: i128 = 4;
        // D s_217_23: read-var u#25969:u32
        let s_217_23: u32 = fn_state.u_25969;
        // D s_217_24: cast zx s_217_23 -> bv
        let s_217_24: Bits = Bits::new(s_217_23 as u128, 32u16);
        // D s_217_25: bit-extract s_217_24 s_217_21 s_217_22
        let s_217_25: Bits = (Bits::new(
            ((s_217_24) >> (s_217_21)).value(),
            u16::try_from(s_217_22).unwrap(),
        ));
        // D s_217_26: cast reint s_217_25 -> u8
        let s_217_26: u8 = (s_217_25.value() as u8);
        // C s_217_27: const #16s : i
        let s_217_27: i128 = 16;
        // C s_217_28: const #5s : i
        let s_217_28: i128 = 5;
        // D s_217_29: read-var u#25969:u32
        let s_217_29: u32 = fn_state.u_25969;
        // D s_217_30: cast zx s_217_29 -> bv
        let s_217_30: Bits = Bits::new(s_217_29 as u128, 32u16);
        // D s_217_31: bit-extract s_217_30 s_217_27 s_217_28
        let s_217_31: Bits = (Bits::new(
            ((s_217_30) >> (s_217_27)).value(),
            u16::try_from(s_217_28).unwrap(),
        ));
        // D s_217_32: cast reint s_217_31 -> u8
        let s_217_32: u8 = (s_217_31.value() as u8);
        // C s_217_33: const #30s : i
        let s_217_33: i128 = 30;
        // C s_217_34: const #1s : i
        let s_217_34: i128 = 1;
        // D s_217_35: read-var u#25969:u32
        let s_217_35: u32 = fn_state.u_25969;
        // D s_217_36: cast zx s_217_35 -> bv
        let s_217_36: Bits = Bits::new(s_217_35 as u128, 32u16);
        // D s_217_37: bit-extract s_217_36 s_217_33 s_217_34
        let s_217_37: Bits = (Bits::new(
            ((s_217_36) >> (s_217_33)).value(),
            u16::try_from(s_217_34).unwrap(),
        ));
        // D s_217_38: cast reint s_217_37 -> u8
        let s_217_38: bool = ((s_217_37.value()) != 0);
        // C s_217_39: const #31s : i
        let s_217_39: i128 = 31;
        // C s_217_40: const #1s : i
        let s_217_40: i128 = 1;
        // D s_217_41: read-var u#25969:u32
        let s_217_41: u32 = fn_state.u_25969;
        // D s_217_42: cast zx s_217_41 -> bv
        let s_217_42: Bits = Bits::new(s_217_41 as u128, 32u16);
        // D s_217_43: bit-extract s_217_42 s_217_39 s_217_40
        let s_217_43: Bits = (Bits::new(
            ((s_217_42) >> (s_217_39)).value(),
            u16::try_from(s_217_40).unwrap(),
        ));
        // D s_217_44: cast reint s_217_43 -> u8
        let s_217_44: bool = ((s_217_43.value()) != 0);
        // D s_217_45: call decode_csel_aarch64_instrs_integer_conditional_select(s_217_8, s_217_14, s_217_20, s_217_26, s_217_32, s_217_38, s_217_44)
        let s_217_45: () = decode_csel_aarch64_instrs_integer_conditional_select(
            state,
            tracer,
            s_217_8,
            s_217_14,
            s_217_20,
            s_217_26,
            s_217_32,
            s_217_38,
            s_217_44,
        );
        // N s_217_46: return
        return;
    }
    fn block_218<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_218_0: read-var merge#var.1:struct
        let s_218_0: u32 = fn_state.merge_var._1;
        // D s_218_1: write-var u#25977 <= s_218_0
        fn_state.u_25977 = s_218_0;
        // C s_218_2: const #21s : i
        let s_218_2: i128 = 21;
        // D s_218_3: read-var u#25977:u32
        let s_218_3: u32 = fn_state.u_25977;
        // D s_218_4: cast zx s_218_3 -> bv
        let s_218_4: Bits = Bits::new(s_218_3 as u128, 32u16);
        // C s_218_5: const #1s : i64
        let s_218_5: i64 = 1;
        // C s_218_6: cast zx s_218_5 -> i
        let s_218_6: i128 = (i128::try_from(s_218_5).unwrap());
        // C s_218_7: const #9s : i
        let s_218_7: i128 = 9;
        // C s_218_8: add s_218_7 s_218_6
        let s_218_8: i128 = (s_218_7 + s_218_6);
        // D s_218_9: bit-extract s_218_4 s_218_2 s_218_8
        let s_218_9: Bits = (Bits::new(
            ((s_218_4) >> (s_218_2)).value(),
            u16::try_from(s_218_8).unwrap(),
        ));
        // D s_218_10: cast reint s_218_9 -> u10
        let s_218_10: u16 = (s_218_9.value() as u16);
        // D s_218_11: cast zx s_218_10 -> bv
        let s_218_11: Bits = Bits::new(s_218_10 as u128, 10u16);
        // C s_218_12: const #212u : u10
        let s_218_12: u16 = 212;
        // C s_218_13: cast zx s_218_12 -> bv
        let s_218_13: Bits = Bits::new(s_218_12 as u128, 10u16);
        // D s_218_14: cmp-eq s_218_11 s_218_13
        let s_218_14: bool = ((s_218_11) == (s_218_13));
        // N s_218_15: branch s_218_14 b509 b219
        if s_218_14 {
            return block_509(state, tracer, fn_state);
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
        // D s_219_1: write-var gs#382986 <= s_219_0
        fn_state.gs_382986 = s_219_0;
        // N s_219_2: jump b220
        return block_220(state, tracer, fn_state);
    }
    fn block_220<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_220_0: read-var gs#382986:u8
        let s_220_0: bool = fn_state.gs_382986;
        // N s_220_1: branch s_220_0 b508 b221
        if s_220_0 {
            return block_508(state, tracer, fn_state);
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
        // D s_221_1: write-var gs#382988 <= s_221_0
        fn_state.gs_382988 = s_221_0;
        // N s_221_2: jump b222
        return block_222(state, tracer, fn_state);
    }
    fn block_222<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_222_0: read-var gs#382988:u8
        let s_222_0: bool = fn_state.gs_382988;
        // D s_222_1: not s_222_0
        let s_222_1: bool = !s_222_0;
        // N s_222_2: branch s_222_1 b224 b223
        if s_222_1 {
            return block_224(state, tracer, fn_state);
        } else {
            return block_223(state, tracer, fn_state);
        };
    }
    fn block_223<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_223_0: const #188s : i
        let s_223_0: i128 = 188;
        // C s_223_1: const #14696u : u32
        let s_223_1: u32 = 14696;
        // N s_223_2: write-reg s_223_1 <= s_223_0
        let s_223_2: () = {
            state.write_register::<i128>(s_223_1 as isize, s_223_0);
            tracer.write_register(s_223_1 as isize, s_223_0);
        };
        // C s_223_3: const #0s : i
        let s_223_3: i128 = 0;
        // C s_223_4: const #5s : i
        let s_223_4: i128 = 5;
        // D s_223_5: read-var u#25977:u32
        let s_223_5: u32 = fn_state.u_25977;
        // D s_223_6: cast zx s_223_5 -> bv
        let s_223_6: Bits = Bits::new(s_223_5 as u128, 32u16);
        // D s_223_7: bit-extract s_223_6 s_223_3 s_223_4
        let s_223_7: Bits = (Bits::new(
            ((s_223_6) >> (s_223_3)).value(),
            u16::try_from(s_223_4).unwrap(),
        ));
        // D s_223_8: cast reint s_223_7 -> u8
        let s_223_8: u8 = (s_223_7.value() as u8);
        // C s_223_9: const #5s : i
        let s_223_9: i128 = 5;
        // C s_223_10: const #5s : i
        let s_223_10: i128 = 5;
        // D s_223_11: read-var u#25977:u32
        let s_223_11: u32 = fn_state.u_25977;
        // D s_223_12: cast zx s_223_11 -> bv
        let s_223_12: Bits = Bits::new(s_223_11 as u128, 32u16);
        // D s_223_13: bit-extract s_223_12 s_223_9 s_223_10
        let s_223_13: Bits = (Bits::new(
            ((s_223_12) >> (s_223_9)).value(),
            u16::try_from(s_223_10).unwrap(),
        ));
        // D s_223_14: cast reint s_223_13 -> u8
        let s_223_14: u8 = (s_223_13.value() as u8);
        // C s_223_15: const #10s : i
        let s_223_15: i128 = 10;
        // C s_223_16: const #1s : i
        let s_223_16: i128 = 1;
        // D s_223_17: read-var u#25977:u32
        let s_223_17: u32 = fn_state.u_25977;
        // D s_223_18: cast zx s_223_17 -> bv
        let s_223_18: Bits = Bits::new(s_223_17 as u128, 32u16);
        // D s_223_19: bit-extract s_223_18 s_223_15 s_223_16
        let s_223_19: Bits = (Bits::new(
            ((s_223_18) >> (s_223_15)).value(),
            u16::try_from(s_223_16).unwrap(),
        ));
        // D s_223_20: cast reint s_223_19 -> u8
        let s_223_20: bool = ((s_223_19.value()) != 0);
        // C s_223_21: const #12s : i
        let s_223_21: i128 = 12;
        // C s_223_22: const #4s : i
        let s_223_22: i128 = 4;
        // D s_223_23: read-var u#25977:u32
        let s_223_23: u32 = fn_state.u_25977;
        // D s_223_24: cast zx s_223_23 -> bv
        let s_223_24: Bits = Bits::new(s_223_23 as u128, 32u16);
        // D s_223_25: bit-extract s_223_24 s_223_21 s_223_22
        let s_223_25: Bits = (Bits::new(
            ((s_223_24) >> (s_223_21)).value(),
            u16::try_from(s_223_22).unwrap(),
        ));
        // D s_223_26: cast reint s_223_25 -> u8
        let s_223_26: u8 = (s_223_25.value() as u8);
        // C s_223_27: const #16s : i
        let s_223_27: i128 = 16;
        // C s_223_28: const #5s : i
        let s_223_28: i128 = 5;
        // D s_223_29: read-var u#25977:u32
        let s_223_29: u32 = fn_state.u_25977;
        // D s_223_30: cast zx s_223_29 -> bv
        let s_223_30: Bits = Bits::new(s_223_29 as u128, 32u16);
        // D s_223_31: bit-extract s_223_30 s_223_27 s_223_28
        let s_223_31: Bits = (Bits::new(
            ((s_223_30) >> (s_223_27)).value(),
            u16::try_from(s_223_28).unwrap(),
        ));
        // D s_223_32: cast reint s_223_31 -> u8
        let s_223_32: u8 = (s_223_31.value() as u8);
        // C s_223_33: const #30s : i
        let s_223_33: i128 = 30;
        // C s_223_34: const #1s : i
        let s_223_34: i128 = 1;
        // D s_223_35: read-var u#25977:u32
        let s_223_35: u32 = fn_state.u_25977;
        // D s_223_36: cast zx s_223_35 -> bv
        let s_223_36: Bits = Bits::new(s_223_35 as u128, 32u16);
        // D s_223_37: bit-extract s_223_36 s_223_33 s_223_34
        let s_223_37: Bits = (Bits::new(
            ((s_223_36) >> (s_223_33)).value(),
            u16::try_from(s_223_34).unwrap(),
        ));
        // D s_223_38: cast reint s_223_37 -> u8
        let s_223_38: bool = ((s_223_37.value()) != 0);
        // C s_223_39: const #31s : i
        let s_223_39: i128 = 31;
        // C s_223_40: const #1s : i
        let s_223_40: i128 = 1;
        // D s_223_41: read-var u#25977:u32
        let s_223_41: u32 = fn_state.u_25977;
        // D s_223_42: cast zx s_223_41 -> bv
        let s_223_42: Bits = Bits::new(s_223_41 as u128, 32u16);
        // D s_223_43: bit-extract s_223_42 s_223_39 s_223_40
        let s_223_43: Bits = (Bits::new(
            ((s_223_42) >> (s_223_39)).value(),
            u16::try_from(s_223_40).unwrap(),
        ));
        // D s_223_44: cast reint s_223_43 -> u8
        let s_223_44: bool = ((s_223_43.value()) != 0);
        // D s_223_45: call decode_csinc_aarch64_instrs_integer_conditional_select(s_223_8, s_223_14, s_223_20, s_223_26, s_223_32, s_223_38, s_223_44)
        let s_223_45: () = decode_csinc_aarch64_instrs_integer_conditional_select(
            state,
            tracer,
            s_223_8,
            s_223_14,
            s_223_20,
            s_223_26,
            s_223_32,
            s_223_38,
            s_223_44,
        );
        // N s_223_46: return
        return;
    }
    fn block_224<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_224_0: read-var merge#var.1:struct
        let s_224_0: u32 = fn_state.merge_var._1;
        // D s_224_1: write-var u#25986 <= s_224_0
        fn_state.u_25986 = s_224_0;
        // C s_224_2: const #21s : i
        let s_224_2: i128 = 21;
        // D s_224_3: read-var u#25986:u32
        let s_224_3: u32 = fn_state.u_25986;
        // D s_224_4: cast zx s_224_3 -> bv
        let s_224_4: Bits = Bits::new(s_224_3 as u128, 32u16);
        // C s_224_5: const #1s : i64
        let s_224_5: i64 = 1;
        // C s_224_6: cast zx s_224_5 -> i
        let s_224_6: i128 = (i128::try_from(s_224_5).unwrap());
        // C s_224_7: const #9s : i
        let s_224_7: i128 = 9;
        // C s_224_8: add s_224_7 s_224_6
        let s_224_8: i128 = (s_224_7 + s_224_6);
        // D s_224_9: bit-extract s_224_4 s_224_2 s_224_8
        let s_224_9: Bits = (Bits::new(
            ((s_224_4) >> (s_224_2)).value(),
            u16::try_from(s_224_8).unwrap(),
        ));
        // D s_224_10: cast reint s_224_9 -> u10
        let s_224_10: u16 = (s_224_9.value() as u16);
        // D s_224_11: cast zx s_224_10 -> bv
        let s_224_11: Bits = Bits::new(s_224_10 as u128, 10u16);
        // C s_224_12: const #724u : u10
        let s_224_12: u16 = 724;
        // C s_224_13: cast zx s_224_12 -> bv
        let s_224_13: Bits = Bits::new(s_224_12 as u128, 10u16);
        // D s_224_14: cmp-eq s_224_11 s_224_13
        let s_224_14: bool = ((s_224_11) == (s_224_13));
        // N s_224_15: branch s_224_14 b507 b225
        if s_224_14 {
            return block_507(state, tracer, fn_state);
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
        // D s_225_1: write-var gs#383010 <= s_225_0
        fn_state.gs_383010 = s_225_0;
        // N s_225_2: jump b226
        return block_226(state, tracer, fn_state);
    }
    fn block_226<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_226_0: read-var gs#383010:u8
        let s_226_0: bool = fn_state.gs_383010;
        // N s_226_1: branch s_226_0 b506 b227
        if s_226_0 {
            return block_506(state, tracer, fn_state);
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
        // D s_227_1: write-var gs#383012 <= s_227_0
        fn_state.gs_383012 = s_227_0;
        // N s_227_2: jump b228
        return block_228(state, tracer, fn_state);
    }
    fn block_228<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_228_0: read-var gs#383012:u8
        let s_228_0: bool = fn_state.gs_383012;
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
        // C s_229_0: const #189s : i
        let s_229_0: i128 = 189;
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
        // D s_229_5: read-var u#25986:u32
        let s_229_5: u32 = fn_state.u_25986;
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
        // D s_229_11: read-var u#25986:u32
        let s_229_11: u32 = fn_state.u_25986;
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
        // D s_229_17: read-var u#25986:u32
        let s_229_17: u32 = fn_state.u_25986;
        // D s_229_18: cast zx s_229_17 -> bv
        let s_229_18: Bits = Bits::new(s_229_17 as u128, 32u16);
        // D s_229_19: bit-extract s_229_18 s_229_15 s_229_16
        let s_229_19: Bits = (Bits::new(
            ((s_229_18) >> (s_229_15)).value(),
            u16::try_from(s_229_16).unwrap(),
        ));
        // D s_229_20: cast reint s_229_19 -> u8
        let s_229_20: bool = ((s_229_19.value()) != 0);
        // C s_229_21: const #12s : i
        let s_229_21: i128 = 12;
        // C s_229_22: const #4s : i
        let s_229_22: i128 = 4;
        // D s_229_23: read-var u#25986:u32
        let s_229_23: u32 = fn_state.u_25986;
        // D s_229_24: cast zx s_229_23 -> bv
        let s_229_24: Bits = Bits::new(s_229_23 as u128, 32u16);
        // D s_229_25: bit-extract s_229_24 s_229_21 s_229_22
        let s_229_25: Bits = (Bits::new(
            ((s_229_24) >> (s_229_21)).value(),
            u16::try_from(s_229_22).unwrap(),
        ));
        // D s_229_26: cast reint s_229_25 -> u8
        let s_229_26: u8 = (s_229_25.value() as u8);
        // C s_229_27: const #16s : i
        let s_229_27: i128 = 16;
        // C s_229_28: const #5s : i
        let s_229_28: i128 = 5;
        // D s_229_29: read-var u#25986:u32
        let s_229_29: u32 = fn_state.u_25986;
        // D s_229_30: cast zx s_229_29 -> bv
        let s_229_30: Bits = Bits::new(s_229_29 as u128, 32u16);
        // D s_229_31: bit-extract s_229_30 s_229_27 s_229_28
        let s_229_31: Bits = (Bits::new(
            ((s_229_30) >> (s_229_27)).value(),
            u16::try_from(s_229_28).unwrap(),
        ));
        // D s_229_32: cast reint s_229_31 -> u8
        let s_229_32: u8 = (s_229_31.value() as u8);
        // C s_229_33: const #30s : i
        let s_229_33: i128 = 30;
        // C s_229_34: const #1s : i
        let s_229_34: i128 = 1;
        // D s_229_35: read-var u#25986:u32
        let s_229_35: u32 = fn_state.u_25986;
        // D s_229_36: cast zx s_229_35 -> bv
        let s_229_36: Bits = Bits::new(s_229_35 as u128, 32u16);
        // D s_229_37: bit-extract s_229_36 s_229_33 s_229_34
        let s_229_37: Bits = (Bits::new(
            ((s_229_36) >> (s_229_33)).value(),
            u16::try_from(s_229_34).unwrap(),
        ));
        // D s_229_38: cast reint s_229_37 -> u8
        let s_229_38: bool = ((s_229_37.value()) != 0);
        // C s_229_39: const #31s : i
        let s_229_39: i128 = 31;
        // C s_229_40: const #1s : i
        let s_229_40: i128 = 1;
        // D s_229_41: read-var u#25986:u32
        let s_229_41: u32 = fn_state.u_25986;
        // D s_229_42: cast zx s_229_41 -> bv
        let s_229_42: Bits = Bits::new(s_229_41 as u128, 32u16);
        // D s_229_43: bit-extract s_229_42 s_229_39 s_229_40
        let s_229_43: Bits = (Bits::new(
            ((s_229_42) >> (s_229_39)).value(),
            u16::try_from(s_229_40).unwrap(),
        ));
        // D s_229_44: cast reint s_229_43 -> u8
        let s_229_44: bool = ((s_229_43.value()) != 0);
        // D s_229_45: call decode_csinv_aarch64_instrs_integer_conditional_select(s_229_8, s_229_14, s_229_20, s_229_26, s_229_32, s_229_38, s_229_44)
        let s_229_45: () = decode_csinv_aarch64_instrs_integer_conditional_select(
            state,
            tracer,
            s_229_8,
            s_229_14,
            s_229_20,
            s_229_26,
            s_229_32,
            s_229_38,
            s_229_44,
        );
        // N s_229_46: return
        return;
    }
    fn block_230<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_230_0: read-var merge#var.1:struct
        let s_230_0: u32 = fn_state.merge_var._1;
        // D s_230_1: write-var u#25995 <= s_230_0
        fn_state.u_25995 = s_230_0;
        // C s_230_2: const #21s : i
        let s_230_2: i128 = 21;
        // D s_230_3: read-var u#25995:u32
        let s_230_3: u32 = fn_state.u_25995;
        // D s_230_4: cast zx s_230_3 -> bv
        let s_230_4: Bits = Bits::new(s_230_3 as u128, 32u16);
        // C s_230_5: const #1s : i64
        let s_230_5: i64 = 1;
        // C s_230_6: cast zx s_230_5 -> i
        let s_230_6: i128 = (i128::try_from(s_230_5).unwrap());
        // C s_230_7: const #9s : i
        let s_230_7: i128 = 9;
        // C s_230_8: add s_230_7 s_230_6
        let s_230_8: i128 = (s_230_7 + s_230_6);
        // D s_230_9: bit-extract s_230_4 s_230_2 s_230_8
        let s_230_9: Bits = (Bits::new(
            ((s_230_4) >> (s_230_2)).value(),
            u16::try_from(s_230_8).unwrap(),
        ));
        // D s_230_10: cast reint s_230_9 -> u10
        let s_230_10: u16 = (s_230_9.value() as u16);
        // D s_230_11: cast zx s_230_10 -> bv
        let s_230_11: Bits = Bits::new(s_230_10 as u128, 10u16);
        // C s_230_12: const #724u : u10
        let s_230_12: u16 = 724;
        // C s_230_13: cast zx s_230_12 -> bv
        let s_230_13: Bits = Bits::new(s_230_12 as u128, 10u16);
        // D s_230_14: cmp-eq s_230_11 s_230_13
        let s_230_14: bool = ((s_230_11) == (s_230_13));
        // N s_230_15: branch s_230_14 b505 b231
        if s_230_14 {
            return block_505(state, tracer, fn_state);
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
        // D s_231_1: write-var gs#383034 <= s_231_0
        fn_state.gs_383034 = s_231_0;
        // N s_231_2: jump b232
        return block_232(state, tracer, fn_state);
    }
    fn block_232<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_232_0: read-var gs#383034:u8
        let s_232_0: bool = fn_state.gs_383034;
        // N s_232_1: branch s_232_0 b504 b233
        if s_232_0 {
            return block_504(state, tracer, fn_state);
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
        // D s_233_1: write-var gs#383036 <= s_233_0
        fn_state.gs_383036 = s_233_0;
        // N s_233_2: jump b234
        return block_234(state, tracer, fn_state);
    }
    fn block_234<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_234_0: read-var gs#383036:u8
        let s_234_0: bool = fn_state.gs_383036;
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
        // C s_235_0: const #190s : i
        let s_235_0: i128 = 190;
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
        // D s_235_5: read-var u#25995:u32
        let s_235_5: u32 = fn_state.u_25995;
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
        // D s_235_11: read-var u#25995:u32
        let s_235_11: u32 = fn_state.u_25995;
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
        // D s_235_17: read-var u#25995:u32
        let s_235_17: u32 = fn_state.u_25995;
        // D s_235_18: cast zx s_235_17 -> bv
        let s_235_18: Bits = Bits::new(s_235_17 as u128, 32u16);
        // D s_235_19: bit-extract s_235_18 s_235_15 s_235_16
        let s_235_19: Bits = (Bits::new(
            ((s_235_18) >> (s_235_15)).value(),
            u16::try_from(s_235_16).unwrap(),
        ));
        // D s_235_20: cast reint s_235_19 -> u8
        let s_235_20: bool = ((s_235_19.value()) != 0);
        // C s_235_21: const #12s : i
        let s_235_21: i128 = 12;
        // C s_235_22: const #4s : i
        let s_235_22: i128 = 4;
        // D s_235_23: read-var u#25995:u32
        let s_235_23: u32 = fn_state.u_25995;
        // D s_235_24: cast zx s_235_23 -> bv
        let s_235_24: Bits = Bits::new(s_235_23 as u128, 32u16);
        // D s_235_25: bit-extract s_235_24 s_235_21 s_235_22
        let s_235_25: Bits = (Bits::new(
            ((s_235_24) >> (s_235_21)).value(),
            u16::try_from(s_235_22).unwrap(),
        ));
        // D s_235_26: cast reint s_235_25 -> u8
        let s_235_26: u8 = (s_235_25.value() as u8);
        // C s_235_27: const #16s : i
        let s_235_27: i128 = 16;
        // C s_235_28: const #5s : i
        let s_235_28: i128 = 5;
        // D s_235_29: read-var u#25995:u32
        let s_235_29: u32 = fn_state.u_25995;
        // D s_235_30: cast zx s_235_29 -> bv
        let s_235_30: Bits = Bits::new(s_235_29 as u128, 32u16);
        // D s_235_31: bit-extract s_235_30 s_235_27 s_235_28
        let s_235_31: Bits = (Bits::new(
            ((s_235_30) >> (s_235_27)).value(),
            u16::try_from(s_235_28).unwrap(),
        ));
        // D s_235_32: cast reint s_235_31 -> u8
        let s_235_32: u8 = (s_235_31.value() as u8);
        // C s_235_33: const #30s : i
        let s_235_33: i128 = 30;
        // C s_235_34: const #1s : i
        let s_235_34: i128 = 1;
        // D s_235_35: read-var u#25995:u32
        let s_235_35: u32 = fn_state.u_25995;
        // D s_235_36: cast zx s_235_35 -> bv
        let s_235_36: Bits = Bits::new(s_235_35 as u128, 32u16);
        // D s_235_37: bit-extract s_235_36 s_235_33 s_235_34
        let s_235_37: Bits = (Bits::new(
            ((s_235_36) >> (s_235_33)).value(),
            u16::try_from(s_235_34).unwrap(),
        ));
        // D s_235_38: cast reint s_235_37 -> u8
        let s_235_38: bool = ((s_235_37.value()) != 0);
        // C s_235_39: const #31s : i
        let s_235_39: i128 = 31;
        // C s_235_40: const #1s : i
        let s_235_40: i128 = 1;
        // D s_235_41: read-var u#25995:u32
        let s_235_41: u32 = fn_state.u_25995;
        // D s_235_42: cast zx s_235_41 -> bv
        let s_235_42: Bits = Bits::new(s_235_41 as u128, 32u16);
        // D s_235_43: bit-extract s_235_42 s_235_39 s_235_40
        let s_235_43: Bits = (Bits::new(
            ((s_235_42) >> (s_235_39)).value(),
            u16::try_from(s_235_40).unwrap(),
        ));
        // D s_235_44: cast reint s_235_43 -> u8
        let s_235_44: bool = ((s_235_43.value()) != 0);
        // D s_235_45: call decode_csneg_aarch64_instrs_integer_conditional_select(s_235_8, s_235_14, s_235_20, s_235_26, s_235_32, s_235_38, s_235_44)
        let s_235_45: () = decode_csneg_aarch64_instrs_integer_conditional_select(
            state,
            tracer,
            s_235_8,
            s_235_14,
            s_235_20,
            s_235_26,
            s_235_32,
            s_235_38,
            s_235_44,
        );
        // N s_235_46: return
        return;
    }
    fn block_236<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_236_0: read-var merge#var.1:struct
        let s_236_0: u32 = fn_state.merge_var._1;
        // D s_236_1: write-var u#26004 <= s_236_0
        fn_state.u_26004 = s_236_0;
        // C s_236_2: const #10s : i
        let s_236_2: i128 = 10;
        // D s_236_3: read-var u#26004:u32
        let s_236_3: u32 = fn_state.u_26004;
        // D s_236_4: cast zx s_236_3 -> bv
        let s_236_4: Bits = Bits::new(s_236_3 as u128, 32u16);
        // C s_236_5: const #1s : i64
        let s_236_5: i64 = 1;
        // C s_236_6: cast zx s_236_5 -> i
        let s_236_6: i128 = (i128::try_from(s_236_5).unwrap());
        // C s_236_7: const #20s : i
        let s_236_7: i128 = 20;
        // C s_236_8: add s_236_7 s_236_6
        let s_236_8: i128 = (s_236_7 + s_236_6);
        // D s_236_9: bit-extract s_236_4 s_236_2 s_236_8
        let s_236_9: Bits = (Bits::new(
            ((s_236_4) >> (s_236_2)).value(),
            u16::try_from(s_236_8).unwrap(),
        ));
        // D s_236_10: cast reint s_236_9 -> u21
        let s_236_10: u32 = (s_236_9.value() as u32);
        // D s_236_11: cast zx s_236_10 -> bv
        let s_236_11: Bits = Bits::new(s_236_10 as u128, 21u16);
        // C s_236_12: const #1486854u : u21
        let s_236_12: u32 = 1486854;
        // C s_236_13: cast zx s_236_12 -> bv
        let s_236_13: Bits = Bits::new(s_236_12 as u128, 21u16);
        // D s_236_14: cmp-eq s_236_11 s_236_13
        let s_236_14: bool = ((s_236_11) == (s_236_13));
        // N s_236_15: branch s_236_14 b503 b237
        if s_236_14 {
            return block_503(state, tracer, fn_state);
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
        // D s_237_1: write-var gs#383057 <= s_237_0
        fn_state.gs_383057 = s_237_0;
        // N s_237_2: jump b238
        return block_238(state, tracer, fn_state);
    }
    fn block_238<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_238_0: read-var gs#383057:u8
        let s_238_0: bool = fn_state.gs_383057;
        // D s_238_1: not s_238_0
        let s_238_1: bool = !s_238_0;
        // N s_238_2: branch s_238_1 b240 b239
        if s_238_1 {
            return block_240(state, tracer, fn_state);
        } else {
            return block_239(state, tracer, fn_state);
        };
    }
    fn block_239<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_239_0: const #191s : i
        let s_239_0: i128 = 191;
        // C s_239_1: const #14696u : u32
        let s_239_1: u32 = 14696;
        // N s_239_2: write-reg s_239_1 <= s_239_0
        let s_239_2: () = {
            state.write_register::<i128>(s_239_1 as isize, s_239_0);
            tracer.write_register(s_239_1 as isize, s_239_0);
        };
        // C s_239_3: const #0s : i
        let s_239_3: i128 = 0;
        // C s_239_4: const #5s : i
        let s_239_4: i128 = 5;
        // D s_239_5: read-var u#26004:u32
        let s_239_5: u32 = fn_state.u_26004;
        // D s_239_6: cast zx s_239_5 -> bv
        let s_239_6: Bits = Bits::new(s_239_5 as u128, 32u16);
        // D s_239_7: bit-extract s_239_6 s_239_3 s_239_4
        let s_239_7: Bits = (Bits::new(
            ((s_239_6) >> (s_239_3)).value(),
            u16::try_from(s_239_4).unwrap(),
        ));
        // D s_239_8: cast reint s_239_7 -> u8
        let s_239_8: u8 = (s_239_7.value() as u8);
        // C s_239_9: const #5s : i
        let s_239_9: i128 = 5;
        // C s_239_10: const #5s : i
        let s_239_10: i128 = 5;
        // D s_239_11: read-var u#26004:u32
        let s_239_11: u32 = fn_state.u_26004;
        // D s_239_12: cast zx s_239_11 -> bv
        let s_239_12: Bits = Bits::new(s_239_11 as u128, 32u16);
        // D s_239_13: bit-extract s_239_12 s_239_9 s_239_10
        let s_239_13: Bits = (Bits::new(
            ((s_239_12) >> (s_239_9)).value(),
            u16::try_from(s_239_10).unwrap(),
        ));
        // D s_239_14: cast reint s_239_13 -> u8
        let s_239_14: u8 = (s_239_13.value() as u8);
        // C s_239_15: const #31s : i
        let s_239_15: i128 = 31;
        // C s_239_16: const #1s : i
        let s_239_16: i128 = 1;
        // D s_239_17: read-var u#26004:u32
        let s_239_17: u32 = fn_state.u_26004;
        // D s_239_18: cast zx s_239_17 -> bv
        let s_239_18: Bits = Bits::new(s_239_17 as u128, 32u16);
        // D s_239_19: bit-extract s_239_18 s_239_15 s_239_16
        let s_239_19: Bits = (Bits::new(
            ((s_239_18) >> (s_239_15)).value(),
            u16::try_from(s_239_16).unwrap(),
        ));
        // D s_239_20: cast reint s_239_19 -> u8
        let s_239_20: bool = ((s_239_19.value()) != 0);
        // D s_239_21: call decode_ctz_aarch64_instrs_integer_arithmetic_unary_ctz(s_239_8, s_239_14, s_239_20)
        let s_239_21: () = decode_ctz_aarch64_instrs_integer_arithmetic_unary_ctz(
            state,
            tracer,
            s_239_8,
            s_239_14,
            s_239_20,
        );
        // N s_239_22: return
        return;
    }
    fn block_240<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_240_0: read-var merge#var.1:struct
        let s_240_0: u32 = fn_state.merge_var._1;
        // D s_240_1: write-var u#26009 <= s_240_0
        fn_state.u_26009 = s_240_0;
        // C s_240_2: const #21s : i
        let s_240_2: i128 = 21;
        // D s_240_3: read-var u#26009:u32
        let s_240_3: u32 = fn_state.u_26009;
        // D s_240_4: cast zx s_240_3 -> bv
        let s_240_4: Bits = Bits::new(s_240_3 as u128, 32u16);
        // C s_240_5: const #1s : i64
        let s_240_5: i64 = 1;
        // C s_240_6: cast zx s_240_5 -> i
        let s_240_6: i128 = (i128::try_from(s_240_5).unwrap());
        // C s_240_7: const #10s : i
        let s_240_7: i128 = 10;
        // C s_240_8: add s_240_7 s_240_6
        let s_240_8: i128 = (s_240_7 + s_240_6);
        // D s_240_9: bit-extract s_240_4 s_240_2 s_240_8
        let s_240_9: Bits = (Bits::new(
            ((s_240_4) >> (s_240_2)).value(),
            u16::try_from(s_240_8).unwrap(),
        ));
        // D s_240_10: cast reint s_240_9 -> u11
        let s_240_10: u16 = (s_240_9.value() as u16);
        // D s_240_11: cast zx s_240_10 -> bv
        let s_240_11: Bits = Bits::new(s_240_10 as u128, 11u16);
        // C s_240_12: const #1238u : u11
        let s_240_12: u16 = 1238;
        // C s_240_13: cast zx s_240_12 -> bv
        let s_240_13: Bits = Bits::new(s_240_12 as u128, 11u16);
        // D s_240_14: cmp-eq s_240_11 s_240_13
        let s_240_14: bool = ((s_240_11) == (s_240_13));
        // N s_240_15: branch s_240_14 b502 b241
        if s_240_14 {
            return block_502(state, tracer, fn_state);
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
        // D s_241_1: write-var gs#383071 <= s_241_0
        fn_state.gs_383071 = s_241_0;
        // N s_241_2: jump b242
        return block_242(state, tracer, fn_state);
    }
    fn block_242<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_242_0: read-var gs#383071:u8
        let s_242_0: bool = fn_state.gs_383071;
        // N s_242_1: branch s_242_0 b501 b243
        if s_242_0 {
            return block_501(state, tracer, fn_state);
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
        // D s_243_1: write-var gs#383073 <= s_243_0
        fn_state.gs_383073 = s_243_0;
        // N s_243_2: jump b244
        return block_244(state, tracer, fn_state);
    }
    fn block_244<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_244_0: read-var gs#383073:u8
        let s_244_0: bool = fn_state.gs_383073;
        // D s_244_1: not s_244_0
        let s_244_1: bool = !s_244_0;
        // N s_244_2: branch s_244_1 b246 b245
        if s_244_1 {
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
        // C s_245_0: const #477s : i
        let s_245_0: i128 = 477;
        // C s_245_1: const #14696u : u32
        let s_245_1: u32 = 14696;
        // N s_245_2: write-reg s_245_1 <= s_245_0
        let s_245_2: () = {
            state.write_register::<i128>(s_245_1 as isize, s_245_0);
            tracer.write_register(s_245_1 as isize, s_245_0);
        };
        // C s_245_3: const #0s : i
        let s_245_3: i128 = 0;
        // C s_245_4: const #5s : i
        let s_245_4: i128 = 5;
        // D s_245_5: read-var u#26009:u32
        let s_245_5: u32 = fn_state.u_26009;
        // D s_245_6: cast zx s_245_5 -> bv
        let s_245_6: Bits = Bits::new(s_245_5 as u128, 32u16);
        // D s_245_7: bit-extract s_245_6 s_245_3 s_245_4
        let s_245_7: Bits = (Bits::new(
            ((s_245_6) >> (s_245_3)).value(),
            u16::try_from(s_245_4).unwrap(),
        ));
        // D s_245_8: cast reint s_245_7 -> u8
        let s_245_8: u8 = (s_245_7.value() as u8);
        // C s_245_9: const #5s : i
        let s_245_9: i128 = 5;
        // C s_245_10: const #5s : i
        let s_245_10: i128 = 5;
        // D s_245_11: read-var u#26009:u32
        let s_245_11: u32 = fn_state.u_26009;
        // D s_245_12: cast zx s_245_11 -> bv
        let s_245_12: Bits = Bits::new(s_245_11 as u128, 32u16);
        // D s_245_13: bit-extract s_245_12 s_245_9 s_245_10
        let s_245_13: Bits = (Bits::new(
            ((s_245_12) >> (s_245_9)).value(),
            u16::try_from(s_245_10).unwrap(),
        ));
        // D s_245_14: cast reint s_245_13 -> u8
        let s_245_14: u8 = (s_245_13.value() as u8);
        // C s_245_15: const #16s : i
        let s_245_15: i128 = 16;
        // C s_245_16: const #5s : i
        let s_245_16: i128 = 5;
        // D s_245_17: read-var u#26009:u32
        let s_245_17: u32 = fn_state.u_26009;
        // D s_245_18: cast zx s_245_17 -> bv
        let s_245_18: Bits = Bits::new(s_245_17 as u128, 32u16);
        // D s_245_19: bit-extract s_245_18 s_245_15 s_245_16
        let s_245_19: Bits = (Bits::new(
            ((s_245_18) >> (s_245_15)).value(),
            u16::try_from(s_245_16).unwrap(),
        ));
        // D s_245_20: cast reint s_245_19 -> u8
        let s_245_20: u8 = (s_245_19.value() as u8);
        // D s_245_21: call decode_gmi_aarch64_instrs_integer_tags_mcinserttagmask(s_245_8, s_245_14, s_245_20)
        let s_245_21: () = decode_gmi_aarch64_instrs_integer_tags_mcinserttagmask(
            state,
            tracer,
            s_245_8,
            s_245_14,
            s_245_20,
        );
        // N s_245_22: return
        return;
    }
    fn block_246<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_246_0: read-var merge#var.1:struct
        let s_246_0: u32 = fn_state.merge_var._1;
        // D s_246_1: write-var u#26011 <= s_246_0
        fn_state.u_26011 = s_246_0;
        // C s_246_2: const #21s : i
        let s_246_2: i128 = 21;
        // D s_246_3: read-var u#26011:u32
        let s_246_3: u32 = fn_state.u_26011;
        // D s_246_4: cast zx s_246_3 -> bv
        let s_246_4: Bits = Bits::new(s_246_3 as u128, 32u16);
        // C s_246_5: const #1s : i64
        let s_246_5: i64 = 1;
        // C s_246_6: cast zx s_246_5 -> i
        let s_246_6: i128 = (i128::try_from(s_246_5).unwrap());
        // C s_246_7: const #10s : i
        let s_246_7: i128 = 10;
        // C s_246_8: add s_246_7 s_246_6
        let s_246_8: i128 = (s_246_7 + s_246_6);
        // D s_246_9: bit-extract s_246_4 s_246_2 s_246_8
        let s_246_9: Bits = (Bits::new(
            ((s_246_4) >> (s_246_2)).value(),
            u16::try_from(s_246_8).unwrap(),
        ));
        // D s_246_10: cast reint s_246_9 -> u11
        let s_246_10: u16 = (s_246_9.value() as u16);
        // D s_246_11: cast zx s_246_10 -> bv
        let s_246_11: Bits = Bits::new(s_246_10 as u128, 11u16);
        // C s_246_12: const #1238u : u11
        let s_246_12: u16 = 1238;
        // C s_246_13: cast zx s_246_12 -> bv
        let s_246_13: Bits = Bits::new(s_246_12 as u128, 11u16);
        // D s_246_14: cmp-eq s_246_11 s_246_13
        let s_246_14: bool = ((s_246_11) == (s_246_13));
        // N s_246_15: branch s_246_14 b500 b247
        if s_246_14 {
            return block_500(state, tracer, fn_state);
        } else {
            return block_247(state, tracer, fn_state);
        };
    }
    fn block_247<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_247_0: const #0u : u8
        let s_247_0: bool = false;
        // D s_247_1: write-var gs#383087 <= s_247_0
        fn_state.gs_383087 = s_247_0;
        // N s_247_2: jump b248
        return block_248(state, tracer, fn_state);
    }
    fn block_248<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_248_0: read-var gs#383087:u8
        let s_248_0: bool = fn_state.gs_383087;
        // N s_248_1: branch s_248_0 b499 b249
        if s_248_0 {
            return block_499(state, tracer, fn_state);
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
        // D s_249_1: write-var gs#383089 <= s_249_0
        fn_state.gs_383089 = s_249_0;
        // N s_249_2: jump b250
        return block_250(state, tracer, fn_state);
    }
    fn block_250<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_250_0: read-var gs#383089:u8
        let s_250_0: bool = fn_state.gs_383089;
        // D s_250_1: not s_250_0
        let s_250_1: bool = !s_250_0;
        // N s_250_2: branch s_250_1 b252 b251
        if s_250_1 {
            return block_252(state, tracer, fn_state);
        } else {
            return block_251(state, tracer, fn_state);
        };
    }
    fn block_251<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_251_0: const #482s : i
        let s_251_0: i128 = 482;
        // C s_251_1: const #14696u : u32
        let s_251_1: u32 = 14696;
        // N s_251_2: write-reg s_251_1 <= s_251_0
        let s_251_2: () = {
            state.write_register::<i128>(s_251_1 as isize, s_251_0);
            tracer.write_register(s_251_1 as isize, s_251_0);
        };
        // C s_251_3: const #0s : i
        let s_251_3: i128 = 0;
        // C s_251_4: const #5s : i
        let s_251_4: i128 = 5;
        // D s_251_5: read-var u#26011:u32
        let s_251_5: u32 = fn_state.u_26011;
        // D s_251_6: cast zx s_251_5 -> bv
        let s_251_6: Bits = Bits::new(s_251_5 as u128, 32u16);
        // D s_251_7: bit-extract s_251_6 s_251_3 s_251_4
        let s_251_7: Bits = (Bits::new(
            ((s_251_6) >> (s_251_3)).value(),
            u16::try_from(s_251_4).unwrap(),
        ));
        // D s_251_8: cast reint s_251_7 -> u8
        let s_251_8: u8 = (s_251_7.value() as u8);
        // C s_251_9: const #5s : i
        let s_251_9: i128 = 5;
        // C s_251_10: const #5s : i
        let s_251_10: i128 = 5;
        // D s_251_11: read-var u#26011:u32
        let s_251_11: u32 = fn_state.u_26011;
        // D s_251_12: cast zx s_251_11 -> bv
        let s_251_12: Bits = Bits::new(s_251_11 as u128, 32u16);
        // D s_251_13: bit-extract s_251_12 s_251_9 s_251_10
        let s_251_13: Bits = (Bits::new(
            ((s_251_12) >> (s_251_9)).value(),
            u16::try_from(s_251_10).unwrap(),
        ));
        // D s_251_14: cast reint s_251_13 -> u8
        let s_251_14: u8 = (s_251_13.value() as u8);
        // C s_251_15: const #16s : i
        let s_251_15: i128 = 16;
        // C s_251_16: const #5s : i
        let s_251_16: i128 = 5;
        // D s_251_17: read-var u#26011:u32
        let s_251_17: u32 = fn_state.u_26011;
        // D s_251_18: cast zx s_251_17 -> bv
        let s_251_18: Bits = Bits::new(s_251_17 as u128, 32u16);
        // D s_251_19: bit-extract s_251_18 s_251_15 s_251_16
        let s_251_19: Bits = (Bits::new(
            ((s_251_18) >> (s_251_15)).value(),
            u16::try_from(s_251_16).unwrap(),
        ));
        // D s_251_20: cast reint s_251_19 -> u8
        let s_251_20: u8 = (s_251_19.value() as u8);
        // D s_251_21: call decode_irg_aarch64_instrs_integer_tags_mcinsertrandomtag(s_251_8, s_251_14, s_251_20)
        let s_251_21: () = decode_irg_aarch64_instrs_integer_tags_mcinsertrandomtag(
            state,
            tracer,
            s_251_8,
            s_251_14,
            s_251_20,
        );
        // N s_251_22: return
        return;
    }
    fn block_252<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_252_0: read-var merge#var.1:struct
        let s_252_0: u32 = fn_state.merge_var._1;
        // D s_252_1: write-var u#26016 <= s_252_0
        fn_state.u_26016 = s_252_0;
        // C s_252_2: const #21s : i
        let s_252_2: i128 = 21;
        // D s_252_3: read-var u#26016:u32
        let s_252_3: u32 = fn_state.u_26016;
        // D s_252_4: cast zx s_252_3 -> bv
        let s_252_4: Bits = Bits::new(s_252_3 as u128, 32u16);
        // C s_252_5: const #1s : i64
        let s_252_5: i64 = 1;
        // C s_252_6: cast zx s_252_5 -> i
        let s_252_6: i128 = (i128::try_from(s_252_5).unwrap());
        // C s_252_7: const #9s : i
        let s_252_7: i128 = 9;
        // C s_252_8: add s_252_7 s_252_6
        let s_252_8: i128 = (s_252_7 + s_252_6);
        // D s_252_9: bit-extract s_252_4 s_252_2 s_252_8
        let s_252_9: Bits = (Bits::new(
            ((s_252_4) >> (s_252_2)).value(),
            u16::try_from(s_252_8).unwrap(),
        ));
        // D s_252_10: cast reint s_252_9 -> u10
        let s_252_10: u16 = (s_252_9.value() as u16);
        // D s_252_11: cast zx s_252_10 -> bv
        let s_252_11: Bits = Bits::new(s_252_10 as u128, 10u16);
        // C s_252_12: const #216u : u10
        let s_252_12: u16 = 216;
        // C s_252_13: cast zx s_252_12 -> bv
        let s_252_13: Bits = Bits::new(s_252_12 as u128, 10u16);
        // D s_252_14: cmp-eq s_252_11 s_252_13
        let s_252_14: bool = ((s_252_11) == (s_252_13));
        // N s_252_15: branch s_252_14 b498 b253
        if s_252_14 {
            return block_498(state, tracer, fn_state);
        } else {
            return block_253(state, tracer, fn_state);
        };
    }
    fn block_253<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_253_0: const #0u : u8
        let s_253_0: bool = false;
        // D s_253_1: write-var gs#383103 <= s_253_0
        fn_state.gs_383103 = s_253_0;
        // N s_253_2: jump b254
        return block_254(state, tracer, fn_state);
    }
    fn block_254<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_254_0: read-var gs#383103:u8
        let s_254_0: bool = fn_state.gs_383103;
        // N s_254_1: branch s_254_0 b497 b255
        if s_254_0 {
            return block_497(state, tracer, fn_state);
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
        // D s_255_1: write-var gs#383105 <= s_255_0
        fn_state.gs_383105 = s_255_0;
        // N s_255_2: jump b256
        return block_256(state, tracer, fn_state);
    }
    fn block_256<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_256_0: read-var gs#383105:u8
        let s_256_0: bool = fn_state.gs_383105;
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
        // C s_257_0: const #690s : i
        let s_257_0: i128 = 690;
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
        // D s_257_5: read-var u#26016:u32
        let s_257_5: u32 = fn_state.u_26016;
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
        // C s_257_10: const #5s : i
        let s_257_10: i128 = 5;
        // D s_257_11: read-var u#26016:u32
        let s_257_11: u32 = fn_state.u_26016;
        // D s_257_12: cast zx s_257_11 -> bv
        let s_257_12: Bits = Bits::new(s_257_11 as u128, 32u16);
        // D s_257_13: bit-extract s_257_12 s_257_9 s_257_10
        let s_257_13: Bits = (Bits::new(
            ((s_257_12) >> (s_257_9)).value(),
            u16::try_from(s_257_10).unwrap(),
        ));
        // D s_257_14: cast reint s_257_13 -> u8
        let s_257_14: u8 = (s_257_13.value() as u8);
        // C s_257_15: const #10s : i
        let s_257_15: i128 = 10;
        // C s_257_16: const #5s : i
        let s_257_16: i128 = 5;
        // D s_257_17: read-var u#26016:u32
        let s_257_17: u32 = fn_state.u_26016;
        // D s_257_18: cast zx s_257_17 -> bv
        let s_257_18: Bits = Bits::new(s_257_17 as u128, 32u16);
        // D s_257_19: bit-extract s_257_18 s_257_15 s_257_16
        let s_257_19: Bits = (Bits::new(
            ((s_257_18) >> (s_257_15)).value(),
            u16::try_from(s_257_16).unwrap(),
        ));
        // D s_257_20: cast reint s_257_19 -> u8
        let s_257_20: u8 = (s_257_19.value() as u8);
        // C s_257_21: const #15s : i
        let s_257_21: i128 = 15;
        // C s_257_22: const #1s : i
        let s_257_22: i128 = 1;
        // D s_257_23: read-var u#26016:u32
        let s_257_23: u32 = fn_state.u_26016;
        // D s_257_24: cast zx s_257_23 -> bv
        let s_257_24: Bits = Bits::new(s_257_23 as u128, 32u16);
        // D s_257_25: bit-extract s_257_24 s_257_21 s_257_22
        let s_257_25: Bits = (Bits::new(
            ((s_257_24) >> (s_257_21)).value(),
            u16::try_from(s_257_22).unwrap(),
        ));
        // D s_257_26: cast reint s_257_25 -> u8
        let s_257_26: bool = ((s_257_25.value()) != 0);
        // C s_257_27: const #16s : i
        let s_257_27: i128 = 16;
        // C s_257_28: const #5s : i
        let s_257_28: i128 = 5;
        // D s_257_29: read-var u#26016:u32
        let s_257_29: u32 = fn_state.u_26016;
        // D s_257_30: cast zx s_257_29 -> bv
        let s_257_30: Bits = Bits::new(s_257_29 as u128, 32u16);
        // D s_257_31: bit-extract s_257_30 s_257_27 s_257_28
        let s_257_31: Bits = (Bits::new(
            ((s_257_30) >> (s_257_27)).value(),
            u16::try_from(s_257_28).unwrap(),
        ));
        // D s_257_32: cast reint s_257_31 -> u8
        let s_257_32: u8 = (s_257_31.value() as u8);
        // C s_257_33: const #31s : i
        let s_257_33: i128 = 31;
        // C s_257_34: const #1s : i
        let s_257_34: i128 = 1;
        // D s_257_35: read-var u#26016:u32
        let s_257_35: u32 = fn_state.u_26016;
        // D s_257_36: cast zx s_257_35 -> bv
        let s_257_36: Bits = Bits::new(s_257_35 as u128, 32u16);
        // D s_257_37: bit-extract s_257_36 s_257_33 s_257_34
        let s_257_37: Bits = (Bits::new(
            ((s_257_36) >> (s_257_33)).value(),
            u16::try_from(s_257_34).unwrap(),
        ));
        // D s_257_38: cast reint s_257_37 -> u8
        let s_257_38: bool = ((s_257_37.value()) != 0);
        // D s_257_39: call decode_madd_aarch64_instrs_integer_arithmetic_mul_uniform_add_sub(s_257_8, s_257_14, s_257_20, s_257_26, s_257_32, s_257_38)
        let s_257_39: () = decode_madd_aarch64_instrs_integer_arithmetic_mul_uniform_add_sub(
            state,
            tracer,
            s_257_8,
            s_257_14,
            s_257_20,
            s_257_26,
            s_257_32,
            s_257_38,
        );
        // N s_257_40: return
        return;
    }
    fn block_258<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_258_0: read-var merge#var.1:struct
        let s_258_0: u32 = fn_state.merge_var._1;
        // D s_258_1: write-var u#26022 <= s_258_0
        fn_state.u_26022 = s_258_0;
        // C s_258_2: const #21s : i
        let s_258_2: i128 = 21;
        // D s_258_3: read-var u#26022:u32
        let s_258_3: u32 = fn_state.u_26022;
        // D s_258_4: cast zx s_258_3 -> bv
        let s_258_4: Bits = Bits::new(s_258_3 as u128, 32u16);
        // C s_258_5: const #1s : i64
        let s_258_5: i64 = 1;
        // C s_258_6: cast zx s_258_5 -> i
        let s_258_6: i128 = (i128::try_from(s_258_5).unwrap());
        // C s_258_7: const #9s : i
        let s_258_7: i128 = 9;
        // C s_258_8: add s_258_7 s_258_6
        let s_258_8: i128 = (s_258_7 + s_258_6);
        // D s_258_9: bit-extract s_258_4 s_258_2 s_258_8
        let s_258_9: Bits = (Bits::new(
            ((s_258_4) >> (s_258_2)).value(),
            u16::try_from(s_258_8).unwrap(),
        ));
        // D s_258_10: cast reint s_258_9 -> u10
        let s_258_10: u16 = (s_258_9.value() as u16);
        // D s_258_11: cast zx s_258_10 -> bv
        let s_258_11: Bits = Bits::new(s_258_10 as u128, 10u16);
        // C s_258_12: const #216u : u10
        let s_258_12: u16 = 216;
        // C s_258_13: cast zx s_258_12 -> bv
        let s_258_13: Bits = Bits::new(s_258_12 as u128, 10u16);
        // D s_258_14: cmp-eq s_258_11 s_258_13
        let s_258_14: bool = ((s_258_11) == (s_258_13));
        // N s_258_15: branch s_258_14 b496 b259
        if s_258_14 {
            return block_496(state, tracer, fn_state);
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
        // D s_259_1: write-var gs#383125 <= s_259_0
        fn_state.gs_383125 = s_259_0;
        // N s_259_2: jump b260
        return block_260(state, tracer, fn_state);
    }
    fn block_260<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_260_0: read-var gs#383125:u8
        let s_260_0: bool = fn_state.gs_383125;
        // N s_260_1: branch s_260_0 b495 b261
        if s_260_0 {
            return block_495(state, tracer, fn_state);
        } else {
            return block_261(state, tracer, fn_state);
        };
    }
    fn block_261<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_261_0: const #0u : u8
        let s_261_0: bool = false;
        // D s_261_1: write-var gs#383127 <= s_261_0
        fn_state.gs_383127 = s_261_0;
        // N s_261_2: jump b262
        return block_262(state, tracer, fn_state);
    }
    fn block_262<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_262_0: read-var gs#383127:u8
        let s_262_0: bool = fn_state.gs_383127;
        // D s_262_1: not s_262_0
        let s_262_1: bool = !s_262_0;
        // N s_262_2: branch s_262_1 b264 b263
        if s_262_1 {
            return block_264(state, tracer, fn_state);
        } else {
            return block_263(state, tracer, fn_state);
        };
    }
    fn block_263<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_263_0: const #691s : i
        let s_263_0: i128 = 691;
        // C s_263_1: const #14696u : u32
        let s_263_1: u32 = 14696;
        // N s_263_2: write-reg s_263_1 <= s_263_0
        let s_263_2: () = {
            state.write_register::<i128>(s_263_1 as isize, s_263_0);
            tracer.write_register(s_263_1 as isize, s_263_0);
        };
        // C s_263_3: const #0s : i
        let s_263_3: i128 = 0;
        // C s_263_4: const #5s : i
        let s_263_4: i128 = 5;
        // D s_263_5: read-var u#26022:u32
        let s_263_5: u32 = fn_state.u_26022;
        // D s_263_6: cast zx s_263_5 -> bv
        let s_263_6: Bits = Bits::new(s_263_5 as u128, 32u16);
        // D s_263_7: bit-extract s_263_6 s_263_3 s_263_4
        let s_263_7: Bits = (Bits::new(
            ((s_263_6) >> (s_263_3)).value(),
            u16::try_from(s_263_4).unwrap(),
        ));
        // D s_263_8: cast reint s_263_7 -> u8
        let s_263_8: u8 = (s_263_7.value() as u8);
        // C s_263_9: const #5s : i
        let s_263_9: i128 = 5;
        // C s_263_10: const #5s : i
        let s_263_10: i128 = 5;
        // D s_263_11: read-var u#26022:u32
        let s_263_11: u32 = fn_state.u_26022;
        // D s_263_12: cast zx s_263_11 -> bv
        let s_263_12: Bits = Bits::new(s_263_11 as u128, 32u16);
        // D s_263_13: bit-extract s_263_12 s_263_9 s_263_10
        let s_263_13: Bits = (Bits::new(
            ((s_263_12) >> (s_263_9)).value(),
            u16::try_from(s_263_10).unwrap(),
        ));
        // D s_263_14: cast reint s_263_13 -> u8
        let s_263_14: u8 = (s_263_13.value() as u8);
        // C s_263_15: const #10s : i
        let s_263_15: i128 = 10;
        // C s_263_16: const #5s : i
        let s_263_16: i128 = 5;
        // D s_263_17: read-var u#26022:u32
        let s_263_17: u32 = fn_state.u_26022;
        // D s_263_18: cast zx s_263_17 -> bv
        let s_263_18: Bits = Bits::new(s_263_17 as u128, 32u16);
        // D s_263_19: bit-extract s_263_18 s_263_15 s_263_16
        let s_263_19: Bits = (Bits::new(
            ((s_263_18) >> (s_263_15)).value(),
            u16::try_from(s_263_16).unwrap(),
        ));
        // D s_263_20: cast reint s_263_19 -> u8
        let s_263_20: u8 = (s_263_19.value() as u8);
        // C s_263_21: const #15s : i
        let s_263_21: i128 = 15;
        // C s_263_22: const #1s : i
        let s_263_22: i128 = 1;
        // D s_263_23: read-var u#26022:u32
        let s_263_23: u32 = fn_state.u_26022;
        // D s_263_24: cast zx s_263_23 -> bv
        let s_263_24: Bits = Bits::new(s_263_23 as u128, 32u16);
        // D s_263_25: bit-extract s_263_24 s_263_21 s_263_22
        let s_263_25: Bits = (Bits::new(
            ((s_263_24) >> (s_263_21)).value(),
            u16::try_from(s_263_22).unwrap(),
        ));
        // D s_263_26: cast reint s_263_25 -> u8
        let s_263_26: bool = ((s_263_25.value()) != 0);
        // C s_263_27: const #16s : i
        let s_263_27: i128 = 16;
        // C s_263_28: const #5s : i
        let s_263_28: i128 = 5;
        // D s_263_29: read-var u#26022:u32
        let s_263_29: u32 = fn_state.u_26022;
        // D s_263_30: cast zx s_263_29 -> bv
        let s_263_30: Bits = Bits::new(s_263_29 as u128, 32u16);
        // D s_263_31: bit-extract s_263_30 s_263_27 s_263_28
        let s_263_31: Bits = (Bits::new(
            ((s_263_30) >> (s_263_27)).value(),
            u16::try_from(s_263_28).unwrap(),
        ));
        // D s_263_32: cast reint s_263_31 -> u8
        let s_263_32: u8 = (s_263_31.value() as u8);
        // C s_263_33: const #31s : i
        let s_263_33: i128 = 31;
        // C s_263_34: const #1s : i
        let s_263_34: i128 = 1;
        // D s_263_35: read-var u#26022:u32
        let s_263_35: u32 = fn_state.u_26022;
        // D s_263_36: cast zx s_263_35 -> bv
        let s_263_36: Bits = Bits::new(s_263_35 as u128, 32u16);
        // D s_263_37: bit-extract s_263_36 s_263_33 s_263_34
        let s_263_37: Bits = (Bits::new(
            ((s_263_36) >> (s_263_33)).value(),
            u16::try_from(s_263_34).unwrap(),
        ));
        // D s_263_38: cast reint s_263_37 -> u8
        let s_263_38: bool = ((s_263_37.value()) != 0);
        // D s_263_39: call decode_msub_aarch64_instrs_integer_arithmetic_mul_uniform_add_sub(s_263_8, s_263_14, s_263_20, s_263_26, s_263_32, s_263_38)
        let s_263_39: () = decode_msub_aarch64_instrs_integer_arithmetic_mul_uniform_add_sub(
            state,
            tracer,
            s_263_8,
            s_263_14,
            s_263_20,
            s_263_26,
            s_263_32,
            s_263_38,
        );
        // N s_263_40: return
        return;
    }
    fn block_264<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_264_0: read-var merge#var.1:struct
        let s_264_0: u32 = fn_state.merge_var._1;
        // D s_264_1: write-var u#26030 <= s_264_0
        fn_state.u_26030 = s_264_0;
        // C s_264_2: const #14s : i
        let s_264_2: i128 = 14;
        // D s_264_3: read-var u#26030:u32
        let s_264_3: u32 = fn_state.u_26030;
        // D s_264_4: cast zx s_264_3 -> bv
        let s_264_4: Bits = Bits::new(s_264_3 as u128, 32u16);
        // C s_264_5: const #1s : i64
        let s_264_5: i64 = 1;
        // C s_264_6: cast zx s_264_5 -> i
        let s_264_6: i128 = (i128::try_from(s_264_5).unwrap());
        // C s_264_7: const #17s : i
        let s_264_7: i128 = 17;
        // C s_264_8: add s_264_7 s_264_6
        let s_264_8: i128 = (s_264_7 + s_264_6);
        // D s_264_9: bit-extract s_264_4 s_264_2 s_264_8
        let s_264_9: Bits = (Bits::new(
            ((s_264_4) >> (s_264_2)).value(),
            u16::try_from(s_264_8).unwrap(),
        ));
        // D s_264_10: cast reint s_264_9 -> u18
        let s_264_10: u32 = (s_264_9.value() as u32);
        // D s_264_11: cast zx s_264_10 -> bv
        let s_264_11: Bits = Bits::new(s_264_10 as u128, 18u16);
        // C s_264_12: const #224004u : u18
        let s_264_12: u32 = 224004;
        // C s_264_13: cast zx s_264_12 -> bv
        let s_264_13: Bits = Bits::new(s_264_12 as u128, 18u16);
        // D s_264_14: cmp-eq s_264_11 s_264_13
        let s_264_14: bool = ((s_264_11) == (s_264_13));
        // N s_264_15: branch s_264_14 b494 b265
        if s_264_14 {
            return block_494(state, tracer, fn_state);
        } else {
            return block_265(state, tracer, fn_state);
        };
    }
    fn block_265<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_265_0: const #0u : u8
        let s_265_0: bool = false;
        // D s_265_1: write-var gs#383147 <= s_265_0
        fn_state.gs_383147 = s_265_0;
        // N s_265_2: jump b266
        return block_266(state, tracer, fn_state);
    }
    fn block_266<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_266_0: read-var gs#383147:u8
        let s_266_0: bool = fn_state.gs_383147;
        // N s_266_1: branch s_266_0 b493 b267
        if s_266_0 {
            return block_493(state, tracer, fn_state);
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
        // D s_267_1: write-var gs#383149 <= s_267_0
        fn_state.gs_383149 = s_267_0;
        // N s_267_2: jump b268
        return block_268(state, tracer, fn_state);
    }
    fn block_268<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_268_0: read-var gs#383149:u8
        let s_268_0: bool = fn_state.gs_383149;
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
        // C s_269_0: const #708s : i
        let s_269_0: i128 = 708;
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
        // D s_269_5: read-var u#26030:u32
        let s_269_5: u32 = fn_state.u_26030;
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
        // C s_269_10: const #5s : i
        let s_269_10: i128 = 5;
        // D s_269_11: read-var u#26030:u32
        let s_269_11: u32 = fn_state.u_26030;
        // D s_269_12: cast zx s_269_11 -> bv
        let s_269_12: Bits = Bits::new(s_269_11 as u128, 32u16);
        // D s_269_13: bit-extract s_269_12 s_269_9 s_269_10
        let s_269_13: Bits = (Bits::new(
            ((s_269_12) >> (s_269_9)).value(),
            u16::try_from(s_269_10).unwrap(),
        ));
        // D s_269_14: cast reint s_269_13 -> u8
        let s_269_14: u8 = (s_269_13.value() as u8);
        // C s_269_15: const #13s : i
        let s_269_15: i128 = 13;
        // C s_269_16: const #1s : i
        let s_269_16: i128 = 1;
        // D s_269_17: read-var u#26030:u32
        let s_269_17: u32 = fn_state.u_26030;
        // D s_269_18: cast zx s_269_17 -> bv
        let s_269_18: Bits = Bits::new(s_269_17 as u128, 32u16);
        // D s_269_19: bit-extract s_269_18 s_269_15 s_269_16
        let s_269_19: Bits = (Bits::new(
            ((s_269_18) >> (s_269_15)).value(),
            u16::try_from(s_269_16).unwrap(),
        ));
        // D s_269_20: cast reint s_269_19 -> u8
        let s_269_20: bool = ((s_269_19.value()) != 0);
        // D s_269_21: call decode_pacda_aarch64_instrs_integer_pac_pacda_dp_1src(s_269_8, s_269_14, s_269_20)
        let s_269_21: () = decode_pacda_aarch64_instrs_integer_pac_pacda_dp_1src(
            state,
            tracer,
            s_269_8,
            s_269_14,
            s_269_20,
        );
        // N s_269_22: return
        return;
    }
    fn block_270<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_270_0: read-var merge#var.1:struct
        let s_270_0: u32 = fn_state.merge_var._1;
        // D s_270_1: write-var u#26035 <= s_270_0
        fn_state.u_26035 = s_270_0;
        // C s_270_2: const #14s : i
        let s_270_2: i128 = 14;
        // D s_270_3: read-var u#26035:u32
        let s_270_3: u32 = fn_state.u_26035;
        // D s_270_4: cast zx s_270_3 -> bv
        let s_270_4: Bits = Bits::new(s_270_3 as u128, 32u16);
        // C s_270_5: const #1s : i64
        let s_270_5: i64 = 1;
        // C s_270_6: cast zx s_270_5 -> i
        let s_270_6: i128 = (i128::try_from(s_270_5).unwrap());
        // C s_270_7: const #17s : i
        let s_270_7: i128 = 17;
        // C s_270_8: add s_270_7 s_270_6
        let s_270_8: i128 = (s_270_7 + s_270_6);
        // D s_270_9: bit-extract s_270_4 s_270_2 s_270_8
        let s_270_9: Bits = (Bits::new(
            ((s_270_4) >> (s_270_2)).value(),
            u16::try_from(s_270_8).unwrap(),
        ));
        // D s_270_10: cast reint s_270_9 -> u18
        let s_270_10: u32 = (s_270_9.value() as u32);
        // D s_270_11: cast zx s_270_10 -> bv
        let s_270_11: Bits = Bits::new(s_270_10 as u128, 18u16);
        // C s_270_12: const #224004u : u18
        let s_270_12: u32 = 224004;
        // C s_270_13: cast zx s_270_12 -> bv
        let s_270_13: Bits = Bits::new(s_270_12 as u128, 18u16);
        // D s_270_14: cmp-eq s_270_11 s_270_13
        let s_270_14: bool = ((s_270_11) == (s_270_13));
        // N s_270_15: branch s_270_14 b492 b271
        if s_270_14 {
            return block_492(state, tracer, fn_state);
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
        // D s_271_1: write-var gs#383163 <= s_271_0
        fn_state.gs_383163 = s_271_0;
        // N s_271_2: jump b272
        return block_272(state, tracer, fn_state);
    }
    fn block_272<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_272_0: read-var gs#383163:u8
        let s_272_0: bool = fn_state.gs_383163;
        // N s_272_1: branch s_272_0 b491 b273
        if s_272_0 {
            return block_491(state, tracer, fn_state);
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
        // D s_273_1: write-var gs#383165 <= s_273_0
        fn_state.gs_383165 = s_273_0;
        // N s_273_2: jump b274
        return block_274(state, tracer, fn_state);
    }
    fn block_274<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_274_0: read-var gs#383165:u8
        let s_274_0: bool = fn_state.gs_383165;
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
        // C s_275_0: const #709s : i
        let s_275_0: i128 = 709;
        // C s_275_1: const #14696u : u32
        let s_275_1: u32 = 14696;
        // N s_275_2: write-reg s_275_1 <= s_275_0
        let s_275_2: () = {
            state.write_register::<i128>(s_275_1 as isize, s_275_0);
            tracer.write_register(s_275_1 as isize, s_275_0);
        };
        // C s_275_3: const #0s : i
        let s_275_3: i128 = 0;
        // C s_275_4: const #5s : i
        let s_275_4: i128 = 5;
        // D s_275_5: read-var u#26035:u32
        let s_275_5: u32 = fn_state.u_26035;
        // D s_275_6: cast zx s_275_5 -> bv
        let s_275_6: Bits = Bits::new(s_275_5 as u128, 32u16);
        // D s_275_7: bit-extract s_275_6 s_275_3 s_275_4
        let s_275_7: Bits = (Bits::new(
            ((s_275_6) >> (s_275_3)).value(),
            u16::try_from(s_275_4).unwrap(),
        ));
        // D s_275_8: cast reint s_275_7 -> u8
        let s_275_8: u8 = (s_275_7.value() as u8);
        // C s_275_9: const #5s : i
        let s_275_9: i128 = 5;
        // C s_275_10: const #5s : i
        let s_275_10: i128 = 5;
        // D s_275_11: read-var u#26035:u32
        let s_275_11: u32 = fn_state.u_26035;
        // D s_275_12: cast zx s_275_11 -> bv
        let s_275_12: Bits = Bits::new(s_275_11 as u128, 32u16);
        // D s_275_13: bit-extract s_275_12 s_275_9 s_275_10
        let s_275_13: Bits = (Bits::new(
            ((s_275_12) >> (s_275_9)).value(),
            u16::try_from(s_275_10).unwrap(),
        ));
        // D s_275_14: cast reint s_275_13 -> u8
        let s_275_14: u8 = (s_275_13.value() as u8);
        // C s_275_15: const #13s : i
        let s_275_15: i128 = 13;
        // C s_275_16: const #1s : i
        let s_275_16: i128 = 1;
        // D s_275_17: read-var u#26035:u32
        let s_275_17: u32 = fn_state.u_26035;
        // D s_275_18: cast zx s_275_17 -> bv
        let s_275_18: Bits = Bits::new(s_275_17 as u128, 32u16);
        // D s_275_19: bit-extract s_275_18 s_275_15 s_275_16
        let s_275_19: Bits = (Bits::new(
            ((s_275_18) >> (s_275_15)).value(),
            u16::try_from(s_275_16).unwrap(),
        ));
        // D s_275_20: cast reint s_275_19 -> u8
        let s_275_20: bool = ((s_275_19.value()) != 0);
        // D s_275_21: call decode_pacdb_aarch64_instrs_integer_pac_pacdb_dp_1src(s_275_8, s_275_14, s_275_20)
        let s_275_21: () = decode_pacdb_aarch64_instrs_integer_pac_pacdb_dp_1src(
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
        // D s_276_1: write-var u#26040 <= s_276_0
        fn_state.u_26040 = s_276_0;
        // C s_276_2: const #21s : i
        let s_276_2: i128 = 21;
        // D s_276_3: read-var u#26040:u32
        let s_276_3: u32 = fn_state.u_26040;
        // D s_276_4: cast zx s_276_3 -> bv
        let s_276_4: Bits = Bits::new(s_276_3 as u128, 32u16);
        // C s_276_5: const #1s : i64
        let s_276_5: i64 = 1;
        // C s_276_6: cast zx s_276_5 -> i
        let s_276_6: i128 = (i128::try_from(s_276_5).unwrap());
        // C s_276_7: const #10s : i
        let s_276_7: i128 = 10;
        // C s_276_8: add s_276_7 s_276_6
        let s_276_8: i128 = (s_276_7 + s_276_6);
        // D s_276_9: bit-extract s_276_4 s_276_2 s_276_8
        let s_276_9: Bits = (Bits::new(
            ((s_276_4) >> (s_276_2)).value(),
            u16::try_from(s_276_8).unwrap(),
        ));
        // D s_276_10: cast reint s_276_9 -> u11
        let s_276_10: u16 = (s_276_9.value() as u16);
        // D s_276_11: cast zx s_276_10 -> bv
        let s_276_11: Bits = Bits::new(s_276_10 as u128, 11u16);
        // C s_276_12: const #1238u : u11
        let s_276_12: u16 = 1238;
        // C s_276_13: cast zx s_276_12 -> bv
        let s_276_13: Bits = Bits::new(s_276_12 as u128, 11u16);
        // D s_276_14: cmp-eq s_276_11 s_276_13
        let s_276_14: bool = ((s_276_11) == (s_276_13));
        // N s_276_15: branch s_276_14 b490 b277
        if s_276_14 {
            return block_490(state, tracer, fn_state);
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
        // D s_277_1: write-var gs#383179 <= s_277_0
        fn_state.gs_383179 = s_277_0;
        // N s_277_2: jump b278
        return block_278(state, tracer, fn_state);
    }
    fn block_278<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_278_0: read-var gs#383179:u8
        let s_278_0: bool = fn_state.gs_383179;
        // N s_278_1: branch s_278_0 b489 b279
        if s_278_0 {
            return block_489(state, tracer, fn_state);
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
        // D s_279_1: write-var gs#383181 <= s_279_0
        fn_state.gs_383181 = s_279_0;
        // N s_279_2: jump b280
        return block_280(state, tracer, fn_state);
    }
    fn block_280<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_280_0: read-var gs#383181:u8
        let s_280_0: bool = fn_state.gs_383181;
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
        // C s_281_0: const #710s : i
        let s_281_0: i128 = 710;
        // C s_281_1: const #14696u : u32
        let s_281_1: u32 = 14696;
        // N s_281_2: write-reg s_281_1 <= s_281_0
        let s_281_2: () = {
            state.write_register::<i128>(s_281_1 as isize, s_281_0);
            tracer.write_register(s_281_1 as isize, s_281_0);
        };
        // C s_281_3: const #0s : i
        let s_281_3: i128 = 0;
        // C s_281_4: const #5s : i
        let s_281_4: i128 = 5;
        // D s_281_5: read-var u#26040:u32
        let s_281_5: u32 = fn_state.u_26040;
        // D s_281_6: cast zx s_281_5 -> bv
        let s_281_6: Bits = Bits::new(s_281_5 as u128, 32u16);
        // D s_281_7: bit-extract s_281_6 s_281_3 s_281_4
        let s_281_7: Bits = (Bits::new(
            ((s_281_6) >> (s_281_3)).value(),
            u16::try_from(s_281_4).unwrap(),
        ));
        // D s_281_8: cast reint s_281_7 -> u8
        let s_281_8: u8 = (s_281_7.value() as u8);
        // C s_281_9: const #5s : i
        let s_281_9: i128 = 5;
        // C s_281_10: const #5s : i
        let s_281_10: i128 = 5;
        // D s_281_11: read-var u#26040:u32
        let s_281_11: u32 = fn_state.u_26040;
        // D s_281_12: cast zx s_281_11 -> bv
        let s_281_12: Bits = Bits::new(s_281_11 as u128, 32u16);
        // D s_281_13: bit-extract s_281_12 s_281_9 s_281_10
        let s_281_13: Bits = (Bits::new(
            ((s_281_12) >> (s_281_9)).value(),
            u16::try_from(s_281_10).unwrap(),
        ));
        // D s_281_14: cast reint s_281_13 -> u8
        let s_281_14: u8 = (s_281_13.value() as u8);
        // C s_281_15: const #16s : i
        let s_281_15: i128 = 16;
        // C s_281_16: const #5s : i
        let s_281_16: i128 = 5;
        // D s_281_17: read-var u#26040:u32
        let s_281_17: u32 = fn_state.u_26040;
        // D s_281_18: cast zx s_281_17 -> bv
        let s_281_18: Bits = Bits::new(s_281_17 as u128, 32u16);
        // D s_281_19: bit-extract s_281_18 s_281_15 s_281_16
        let s_281_19: Bits = (Bits::new(
            ((s_281_18) >> (s_281_15)).value(),
            u16::try_from(s_281_16).unwrap(),
        ));
        // D s_281_20: cast reint s_281_19 -> u8
        let s_281_20: u8 = (s_281_19.value() as u8);
        // D s_281_21: call decode_pacga_aarch64_instrs_integer_pac_pacga_dp_2src(s_281_8, s_281_14, s_281_20)
        let s_281_21: () = decode_pacga_aarch64_instrs_integer_pac_pacga_dp_2src(
            state,
            tracer,
            s_281_8,
            s_281_14,
            s_281_20,
        );
        // N s_281_22: return
        return;
    }
    fn block_282<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_282_0: read-var merge#var.1:struct
        let s_282_0: u32 = fn_state.merge_var._1;
        // D s_282_1: write-var u#26045 <= s_282_0
        fn_state.u_26045 = s_282_0;
        // C s_282_2: const #14s : i
        let s_282_2: i128 = 14;
        // D s_282_3: read-var u#26045:u32
        let s_282_3: u32 = fn_state.u_26045;
        // D s_282_4: cast zx s_282_3 -> bv
        let s_282_4: Bits = Bits::new(s_282_3 as u128, 32u16);
        // C s_282_5: const #1s : i64
        let s_282_5: i64 = 1;
        // C s_282_6: cast zx s_282_5 -> i
        let s_282_6: i128 = (i128::try_from(s_282_5).unwrap());
        // C s_282_7: const #17s : i
        let s_282_7: i128 = 17;
        // C s_282_8: add s_282_7 s_282_6
        let s_282_8: i128 = (s_282_7 + s_282_6);
        // D s_282_9: bit-extract s_282_4 s_282_2 s_282_8
        let s_282_9: Bits = (Bits::new(
            ((s_282_4) >> (s_282_2)).value(),
            u16::try_from(s_282_8).unwrap(),
        ));
        // D s_282_10: cast reint s_282_9 -> u18
        let s_282_10: u32 = (s_282_9.value() as u32);
        // D s_282_11: cast zx s_282_10 -> bv
        let s_282_11: Bits = Bits::new(s_282_10 as u128, 18u16);
        // C s_282_12: const #224004u : u18
        let s_282_12: u32 = 224004;
        // C s_282_13: cast zx s_282_12 -> bv
        let s_282_13: Bits = Bits::new(s_282_12 as u128, 18u16);
        // D s_282_14: cmp-eq s_282_11 s_282_13
        let s_282_14: bool = ((s_282_11) == (s_282_13));
        // N s_282_15: branch s_282_14 b488 b283
        if s_282_14 {
            return block_488(state, tracer, fn_state);
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
        // D s_283_1: write-var gs#383195 <= s_283_0
        fn_state.gs_383195 = s_283_0;
        // N s_283_2: jump b284
        return block_284(state, tracer, fn_state);
    }
    fn block_284<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_284_0: read-var gs#383195:u8
        let s_284_0: bool = fn_state.gs_383195;
        // N s_284_1: branch s_284_0 b487 b285
        if s_284_0 {
            return block_487(state, tracer, fn_state);
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
        // D s_285_1: write-var gs#383197 <= s_285_0
        fn_state.gs_383197 = s_285_0;
        // N s_285_2: jump b286
        return block_286(state, tracer, fn_state);
    }
    fn block_286<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_286_0: read-var gs#383197:u8
        let s_286_0: bool = fn_state.gs_383197;
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
        // C s_287_0: const #711s : i
        let s_287_0: i128 = 711;
        // C s_287_1: const #14696u : u32
        let s_287_1: u32 = 14696;
        // N s_287_2: write-reg s_287_1 <= s_287_0
        let s_287_2: () = {
            state.write_register::<i128>(s_287_1 as isize, s_287_0);
            tracer.write_register(s_287_1 as isize, s_287_0);
        };
        // C s_287_3: const #0s : i
        let s_287_3: i128 = 0;
        // C s_287_4: const #5s : i
        let s_287_4: i128 = 5;
        // D s_287_5: read-var u#26045:u32
        let s_287_5: u32 = fn_state.u_26045;
        // D s_287_6: cast zx s_287_5 -> bv
        let s_287_6: Bits = Bits::new(s_287_5 as u128, 32u16);
        // D s_287_7: bit-extract s_287_6 s_287_3 s_287_4
        let s_287_7: Bits = (Bits::new(
            ((s_287_6) >> (s_287_3)).value(),
            u16::try_from(s_287_4).unwrap(),
        ));
        // D s_287_8: cast reint s_287_7 -> u8
        let s_287_8: u8 = (s_287_7.value() as u8);
        // C s_287_9: const #5s : i
        let s_287_9: i128 = 5;
        // C s_287_10: const #5s : i
        let s_287_10: i128 = 5;
        // D s_287_11: read-var u#26045:u32
        let s_287_11: u32 = fn_state.u_26045;
        // D s_287_12: cast zx s_287_11 -> bv
        let s_287_12: Bits = Bits::new(s_287_11 as u128, 32u16);
        // D s_287_13: bit-extract s_287_12 s_287_9 s_287_10
        let s_287_13: Bits = (Bits::new(
            ((s_287_12) >> (s_287_9)).value(),
            u16::try_from(s_287_10).unwrap(),
        ));
        // D s_287_14: cast reint s_287_13 -> u8
        let s_287_14: u8 = (s_287_13.value() as u8);
        // C s_287_15: const #13s : i
        let s_287_15: i128 = 13;
        // C s_287_16: const #1s : i
        let s_287_16: i128 = 1;
        // D s_287_17: read-var u#26045:u32
        let s_287_17: u32 = fn_state.u_26045;
        // D s_287_18: cast zx s_287_17 -> bv
        let s_287_18: Bits = Bits::new(s_287_17 as u128, 32u16);
        // D s_287_19: bit-extract s_287_18 s_287_15 s_287_16
        let s_287_19: Bits = (Bits::new(
            ((s_287_18) >> (s_287_15)).value(),
            u16::try_from(s_287_16).unwrap(),
        ));
        // D s_287_20: cast reint s_287_19 -> u8
        let s_287_20: bool = ((s_287_19.value()) != 0);
        // D s_287_21: call decode_pacia_aarch64_instrs_integer_pac_pacia_dp_1src(s_287_8, s_287_14, s_287_20)
        let s_287_21: () = decode_pacia_aarch64_instrs_integer_pac_pacia_dp_1src(
            state,
            tracer,
            s_287_8,
            s_287_14,
            s_287_20,
        );
        // N s_287_22: return
        return;
    }
    fn block_288<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_288_0: read-var merge#var.1:struct
        let s_288_0: u32 = fn_state.merge_var._1;
        // D s_288_1: write-var u#26050 <= s_288_0
        fn_state.u_26050 = s_288_0;
        // C s_288_2: const #14s : i
        let s_288_2: i128 = 14;
        // D s_288_3: read-var u#26050:u32
        let s_288_3: u32 = fn_state.u_26050;
        // D s_288_4: cast zx s_288_3 -> bv
        let s_288_4: Bits = Bits::new(s_288_3 as u128, 32u16);
        // C s_288_5: const #1s : i64
        let s_288_5: i64 = 1;
        // C s_288_6: cast zx s_288_5 -> i
        let s_288_6: i128 = (i128::try_from(s_288_5).unwrap());
        // C s_288_7: const #17s : i
        let s_288_7: i128 = 17;
        // C s_288_8: add s_288_7 s_288_6
        let s_288_8: i128 = (s_288_7 + s_288_6);
        // D s_288_9: bit-extract s_288_4 s_288_2 s_288_8
        let s_288_9: Bits = (Bits::new(
            ((s_288_4) >> (s_288_2)).value(),
            u16::try_from(s_288_8).unwrap(),
        ));
        // D s_288_10: cast reint s_288_9 -> u18
        let s_288_10: u32 = (s_288_9.value() as u32);
        // D s_288_11: cast zx s_288_10 -> bv
        let s_288_11: Bits = Bits::new(s_288_10 as u128, 18u16);
        // C s_288_12: const #224004u : u18
        let s_288_12: u32 = 224004;
        // C s_288_13: cast zx s_288_12 -> bv
        let s_288_13: Bits = Bits::new(s_288_12 as u128, 18u16);
        // D s_288_14: cmp-eq s_288_11 s_288_13
        let s_288_14: bool = ((s_288_11) == (s_288_13));
        // N s_288_15: branch s_288_14 b486 b289
        if s_288_14 {
            return block_486(state, tracer, fn_state);
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
        // D s_289_1: write-var gs#383211 <= s_289_0
        fn_state.gs_383211 = s_289_0;
        // N s_289_2: jump b290
        return block_290(state, tracer, fn_state);
    }
    fn block_290<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_290_0: read-var gs#383211:u8
        let s_290_0: bool = fn_state.gs_383211;
        // N s_290_1: branch s_290_0 b485 b291
        if s_290_0 {
            return block_485(state, tracer, fn_state);
        } else {
            return block_291(state, tracer, fn_state);
        };
    }
    fn block_291<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_291_0: const #0u : u8
        let s_291_0: bool = false;
        // D s_291_1: write-var gs#383213 <= s_291_0
        fn_state.gs_383213 = s_291_0;
        // N s_291_2: jump b292
        return block_292(state, tracer, fn_state);
    }
    fn block_292<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_292_0: read-var gs#383213:u8
        let s_292_0: bool = fn_state.gs_383213;
        // D s_292_1: not s_292_0
        let s_292_1: bool = !s_292_0;
        // N s_292_2: branch s_292_1 b294 b293
        if s_292_1 {
            return block_294(state, tracer, fn_state);
        } else {
            return block_293(state, tracer, fn_state);
        };
    }
    fn block_293<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_293_0: const #713s : i
        let s_293_0: i128 = 713;
        // C s_293_1: const #14696u : u32
        let s_293_1: u32 = 14696;
        // N s_293_2: write-reg s_293_1 <= s_293_0
        let s_293_2: () = {
            state.write_register::<i128>(s_293_1 as isize, s_293_0);
            tracer.write_register(s_293_1 as isize, s_293_0);
        };
        // C s_293_3: const #0s : i
        let s_293_3: i128 = 0;
        // C s_293_4: const #5s : i
        let s_293_4: i128 = 5;
        // D s_293_5: read-var u#26050:u32
        let s_293_5: u32 = fn_state.u_26050;
        // D s_293_6: cast zx s_293_5 -> bv
        let s_293_6: Bits = Bits::new(s_293_5 as u128, 32u16);
        // D s_293_7: bit-extract s_293_6 s_293_3 s_293_4
        let s_293_7: Bits = (Bits::new(
            ((s_293_6) >> (s_293_3)).value(),
            u16::try_from(s_293_4).unwrap(),
        ));
        // D s_293_8: cast reint s_293_7 -> u8
        let s_293_8: u8 = (s_293_7.value() as u8);
        // C s_293_9: const #5s : i
        let s_293_9: i128 = 5;
        // C s_293_10: const #5s : i
        let s_293_10: i128 = 5;
        // D s_293_11: read-var u#26050:u32
        let s_293_11: u32 = fn_state.u_26050;
        // D s_293_12: cast zx s_293_11 -> bv
        let s_293_12: Bits = Bits::new(s_293_11 as u128, 32u16);
        // D s_293_13: bit-extract s_293_12 s_293_9 s_293_10
        let s_293_13: Bits = (Bits::new(
            ((s_293_12) >> (s_293_9)).value(),
            u16::try_from(s_293_10).unwrap(),
        ));
        // D s_293_14: cast reint s_293_13 -> u8
        let s_293_14: u8 = (s_293_13.value() as u8);
        // C s_293_15: const #13s : i
        let s_293_15: i128 = 13;
        // C s_293_16: const #1s : i
        let s_293_16: i128 = 1;
        // D s_293_17: read-var u#26050:u32
        let s_293_17: u32 = fn_state.u_26050;
        // D s_293_18: cast zx s_293_17 -> bv
        let s_293_18: Bits = Bits::new(s_293_17 as u128, 32u16);
        // D s_293_19: bit-extract s_293_18 s_293_15 s_293_16
        let s_293_19: Bits = (Bits::new(
            ((s_293_18) >> (s_293_15)).value(),
            u16::try_from(s_293_16).unwrap(),
        ));
        // D s_293_20: cast reint s_293_19 -> u8
        let s_293_20: bool = ((s_293_19.value()) != 0);
        // D s_293_21: call decode_pacib_aarch64_instrs_integer_pac_pacib_dp_1src(s_293_8, s_293_14, s_293_20)
        let s_293_21: () = decode_pacib_aarch64_instrs_integer_pac_pacib_dp_1src(
            state,
            tracer,
            s_293_8,
            s_293_14,
            s_293_20,
        );
        // N s_293_22: return
        return;
    }
    fn block_294<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_294_0: read-var merge#var.1:struct
        let s_294_0: u32 = fn_state.merge_var._1;
        // D s_294_1: write-var u#26055 <= s_294_0
        fn_state.u_26055 = s_294_0;
        // C s_294_2: const #10s : i
        let s_294_2: i128 = 10;
        // D s_294_3: read-var u#26055:u32
        let s_294_3: u32 = fn_state.u_26055;
        // D s_294_4: cast zx s_294_3 -> bv
        let s_294_4: Bits = Bits::new(s_294_3 as u128, 32u16);
        // C s_294_5: const #1s : i64
        let s_294_5: i64 = 1;
        // C s_294_6: cast zx s_294_5 -> i
        let s_294_6: i128 = (i128::try_from(s_294_5).unwrap());
        // C s_294_7: const #20s : i
        let s_294_7: i128 = 20;
        // C s_294_8: add s_294_7 s_294_6
        let s_294_8: i128 = (s_294_7 + s_294_6);
        // D s_294_9: bit-extract s_294_4 s_294_2 s_294_8
        let s_294_9: Bits = (Bits::new(
            ((s_294_4) >> (s_294_2)).value(),
            u16::try_from(s_294_8).unwrap(),
        ));
        // D s_294_10: cast reint s_294_9 -> u21
        let s_294_10: u32 = (s_294_9.value() as u32);
        // D s_294_11: cast zx s_294_10 -> bv
        let s_294_11: Bits = Bits::new(s_294_10 as u128, 21u16);
        // C s_294_12: const #1486848u : u21
        let s_294_12: u32 = 1486848;
        // C s_294_13: cast zx s_294_12 -> bv
        let s_294_13: Bits = Bits::new(s_294_12 as u128, 21u16);
        // D s_294_14: cmp-eq s_294_11 s_294_13
        let s_294_14: bool = ((s_294_11) == (s_294_13));
        // N s_294_15: branch s_294_14 b484 b295
        if s_294_14 {
            return block_484(state, tracer, fn_state);
        } else {
            return block_295(state, tracer, fn_state);
        };
    }
    fn block_295<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_295_0: const #0u : u8
        let s_295_0: bool = false;
        // D s_295_1: write-var gs#383226 <= s_295_0
        fn_state.gs_383226 = s_295_0;
        // N s_295_2: jump b296
        return block_296(state, tracer, fn_state);
    }
    fn block_296<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_296_0: read-var gs#383226:u8
        let s_296_0: bool = fn_state.gs_383226;
        // D s_296_1: not s_296_0
        let s_296_1: bool = !s_296_0;
        // N s_296_2: branch s_296_1 b298 b297
        if s_296_1 {
            return block_298(state, tracer, fn_state);
        } else {
            return block_297(state, tracer, fn_state);
        };
    }
    fn block_297<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_297_0: const #719s : i
        let s_297_0: i128 = 719;
        // C s_297_1: const #14696u : u32
        let s_297_1: u32 = 14696;
        // N s_297_2: write-reg s_297_1 <= s_297_0
        let s_297_2: () = {
            state.write_register::<i128>(s_297_1 as isize, s_297_0);
            tracer.write_register(s_297_1 as isize, s_297_0);
        };
        // C s_297_3: const #0s : i
        let s_297_3: i128 = 0;
        // C s_297_4: const #5s : i
        let s_297_4: i128 = 5;
        // D s_297_5: read-var u#26055:u32
        let s_297_5: u32 = fn_state.u_26055;
        // D s_297_6: cast zx s_297_5 -> bv
        let s_297_6: Bits = Bits::new(s_297_5 as u128, 32u16);
        // D s_297_7: bit-extract s_297_6 s_297_3 s_297_4
        let s_297_7: Bits = (Bits::new(
            ((s_297_6) >> (s_297_3)).value(),
            u16::try_from(s_297_4).unwrap(),
        ));
        // D s_297_8: cast reint s_297_7 -> u8
        let s_297_8: u8 = (s_297_7.value() as u8);
        // C s_297_9: const #5s : i
        let s_297_9: i128 = 5;
        // C s_297_10: const #5s : i
        let s_297_10: i128 = 5;
        // D s_297_11: read-var u#26055:u32
        let s_297_11: u32 = fn_state.u_26055;
        // D s_297_12: cast zx s_297_11 -> bv
        let s_297_12: Bits = Bits::new(s_297_11 as u128, 32u16);
        // D s_297_13: bit-extract s_297_12 s_297_9 s_297_10
        let s_297_13: Bits = (Bits::new(
            ((s_297_12) >> (s_297_9)).value(),
            u16::try_from(s_297_10).unwrap(),
        ));
        // D s_297_14: cast reint s_297_13 -> u8
        let s_297_14: u8 = (s_297_13.value() as u8);
        // C s_297_15: const #31s : i
        let s_297_15: i128 = 31;
        // C s_297_16: const #1s : i
        let s_297_16: i128 = 1;
        // D s_297_17: read-var u#26055:u32
        let s_297_17: u32 = fn_state.u_26055;
        // D s_297_18: cast zx s_297_17 -> bv
        let s_297_18: Bits = Bits::new(s_297_17 as u128, 32u16);
        // D s_297_19: bit-extract s_297_18 s_297_15 s_297_16
        let s_297_19: Bits = (Bits::new(
            ((s_297_18) >> (s_297_15)).value(),
            u16::try_from(s_297_16).unwrap(),
        ));
        // D s_297_20: cast reint s_297_19 -> u8
        let s_297_20: bool = ((s_297_19.value()) != 0);
        // D s_297_21: call decode_rbit_int_aarch64_instrs_integer_arithmetic_rbit(s_297_8, s_297_14, s_297_20)
        let s_297_21: () = decode_rbit_int_aarch64_instrs_integer_arithmetic_rbit(
            state,
            tracer,
            s_297_8,
            s_297_14,
            s_297_20,
        );
        // N s_297_22: return
        return;
    }
    fn block_298<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_298_0: read-var merge#var.1:struct
        let s_298_0: u32 = fn_state.merge_var._1;
        // D s_298_1: write-var u#26060 <= s_298_0
        fn_state.u_26060 = s_298_0;
        // C s_298_2: const #11s : i
        let s_298_2: i128 = 11;
        // D s_298_3: read-var u#26060:u32
        let s_298_3: u32 = fn_state.u_26060;
        // D s_298_4: cast zx s_298_3 -> bv
        let s_298_4: Bits = Bits::new(s_298_3 as u128, 32u16);
        // C s_298_5: const #1s : i64
        let s_298_5: i64 = 1;
        // C s_298_6: cast zx s_298_5 -> i
        let s_298_6: i128 = (i128::try_from(s_298_5).unwrap());
        // C s_298_7: const #19s : i
        let s_298_7: i128 = 19;
        // C s_298_8: add s_298_7 s_298_6
        let s_298_8: i128 = (s_298_7 + s_298_6);
        // D s_298_9: bit-extract s_298_4 s_298_2 s_298_8
        let s_298_9: Bits = (Bits::new(
            ((s_298_4) >> (s_298_2)).value(),
            u16::try_from(s_298_8).unwrap(),
        ));
        // D s_298_10: cast reint s_298_9 -> u20
        let s_298_10: u32 = (s_298_9.value() as u32);
        // D s_298_11: cast zx s_298_10 -> bv
        let s_298_11: Bits = Bits::new(s_298_10 as u128, 20u16);
        // C s_298_12: const #743425u : u20
        let s_298_12: u32 = 743425;
        // C s_298_13: cast zx s_298_12 -> bv
        let s_298_13: Bits = Bits::new(s_298_12 as u128, 20u16);
        // D s_298_14: cmp-eq s_298_11 s_298_13
        let s_298_14: bool = ((s_298_11) == (s_298_13));
        // N s_298_15: branch s_298_14 b483 b299
        if s_298_14 {
            return block_483(state, tracer, fn_state);
        } else {
            return block_299(state, tracer, fn_state);
        };
    }
    fn block_299<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_299_0: const #0u : u8
        let s_299_0: bool = false;
        // D s_299_1: write-var gs#383239 <= s_299_0
        fn_state.gs_383239 = s_299_0;
        // N s_299_2: jump b300
        return block_300(state, tracer, fn_state);
    }
    fn block_300<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_300_0: read-var gs#383239:u8
        let s_300_0: bool = fn_state.gs_383239;
        // D s_300_1: not s_300_0
        let s_300_1: bool = !s_300_0;
        // N s_300_2: branch s_300_1 b302 b301
        if s_300_1 {
            return block_302(state, tracer, fn_state);
        } else {
            return block_301(state, tracer, fn_state);
        };
    }
    fn block_301<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_301_0: const #736s : i
        let s_301_0: i128 = 736;
        // C s_301_1: const #14696u : u32
        let s_301_1: u32 = 14696;
        // N s_301_2: write-reg s_301_1 <= s_301_0
        let s_301_2: () = {
            state.write_register::<i128>(s_301_1 as isize, s_301_0);
            tracer.write_register(s_301_1 as isize, s_301_0);
        };
        // C s_301_3: const #0s : i
        let s_301_3: i128 = 0;
        // C s_301_4: const #5s : i
        let s_301_4: i128 = 5;
        // D s_301_5: read-var u#26060:u32
        let s_301_5: u32 = fn_state.u_26060;
        // D s_301_6: cast zx s_301_5 -> bv
        let s_301_6: Bits = Bits::new(s_301_5 as u128, 32u16);
        // D s_301_7: bit-extract s_301_6 s_301_3 s_301_4
        let s_301_7: Bits = (Bits::new(
            ((s_301_6) >> (s_301_3)).value(),
            u16::try_from(s_301_4).unwrap(),
        ));
        // D s_301_8: cast reint s_301_7 -> u8
        let s_301_8: u8 = (s_301_7.value() as u8);
        // C s_301_9: const #5s : i
        let s_301_9: i128 = 5;
        // C s_301_10: const #5s : i
        let s_301_10: i128 = 5;
        // D s_301_11: read-var u#26060:u32
        let s_301_11: u32 = fn_state.u_26060;
        // D s_301_12: cast zx s_301_11 -> bv
        let s_301_12: Bits = Bits::new(s_301_11 as u128, 32u16);
        // D s_301_13: bit-extract s_301_12 s_301_9 s_301_10
        let s_301_13: Bits = (Bits::new(
            ((s_301_12) >> (s_301_9)).value(),
            u16::try_from(s_301_10).unwrap(),
        ));
        // D s_301_14: cast reint s_301_13 -> u8
        let s_301_14: u8 = (s_301_13.value() as u8);
        // C s_301_15: const #10s : i
        let s_301_15: i128 = 10;
        // C s_301_16: const #2s : i
        let s_301_16: i128 = 2;
        // D s_301_17: read-var u#26060:u32
        let s_301_17: u32 = fn_state.u_26060;
        // D s_301_18: cast zx s_301_17 -> bv
        let s_301_18: Bits = Bits::new(s_301_17 as u128, 32u16);
        // D s_301_19: bit-extract s_301_18 s_301_15 s_301_16
        let s_301_19: Bits = (Bits::new(
            ((s_301_18) >> (s_301_15)).value(),
            u16::try_from(s_301_16).unwrap(),
        ));
        // D s_301_20: cast reint s_301_19 -> u8
        let s_301_20: u8 = (s_301_19.value() as u8);
        // C s_301_21: const #31s : i
        let s_301_21: i128 = 31;
        // C s_301_22: const #1s : i
        let s_301_22: i128 = 1;
        // D s_301_23: read-var u#26060:u32
        let s_301_23: u32 = fn_state.u_26060;
        // D s_301_24: cast zx s_301_23 -> bv
        let s_301_24: Bits = Bits::new(s_301_23 as u128, 32u16);
        // D s_301_25: bit-extract s_301_24 s_301_21 s_301_22
        let s_301_25: Bits = (Bits::new(
            ((s_301_24) >> (s_301_21)).value(),
            u16::try_from(s_301_22).unwrap(),
        ));
        // D s_301_26: cast reint s_301_25 -> u8
        let s_301_26: bool = ((s_301_25.value()) != 0);
        // D s_301_27: call decode_rev_aarch64_instrs_integer_arithmetic_rev(s_301_8, s_301_14, s_301_20, s_301_26)
        let s_301_27: () = decode_rev_aarch64_instrs_integer_arithmetic_rev(
            state,
            tracer,
            s_301_8,
            s_301_14,
            s_301_20,
            s_301_26,
        );
        // N s_301_28: return
        return;
    }
    fn block_302<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_302_0: read-var merge#var.1:struct
        let s_302_0: u32 = fn_state.merge_var._1;
        // D s_302_1: write-var u#26066 <= s_302_0
        fn_state.u_26066 = s_302_0;
        // C s_302_2: const #10s : i
        let s_302_2: i128 = 10;
        // D s_302_3: read-var u#26066:u32
        let s_302_3: u32 = fn_state.u_26066;
        // D s_302_4: cast zx s_302_3 -> bv
        let s_302_4: Bits = Bits::new(s_302_3 as u128, 32u16);
        // C s_302_5: const #1s : i64
        let s_302_5: i64 = 1;
        // C s_302_6: cast zx s_302_5 -> i
        let s_302_6: i128 = (i128::try_from(s_302_5).unwrap());
        // C s_302_7: const #20s : i
        let s_302_7: i128 = 20;
        // C s_302_8: add s_302_7 s_302_6
        let s_302_8: i128 = (s_302_7 + s_302_6);
        // D s_302_9: bit-extract s_302_4 s_302_2 s_302_8
        let s_302_9: Bits = (Bits::new(
            ((s_302_4) >> (s_302_2)).value(),
            u16::try_from(s_302_8).unwrap(),
        ));
        // D s_302_10: cast reint s_302_9 -> u21
        let s_302_10: u32 = (s_302_9.value() as u32);
        // D s_302_11: cast zx s_302_10 -> bv
        let s_302_11: Bits = Bits::new(s_302_10 as u128, 21u16);
        // C s_302_12: const #1486849u : u21
        let s_302_12: u32 = 1486849;
        // C s_302_13: cast zx s_302_12 -> bv
        let s_302_13: Bits = Bits::new(s_302_12 as u128, 21u16);
        // D s_302_14: cmp-eq s_302_11 s_302_13
        let s_302_14: bool = ((s_302_11) == (s_302_13));
        // N s_302_15: branch s_302_14 b482 b303
        if s_302_14 {
            return block_482(state, tracer, fn_state);
        } else {
            return block_303(state, tracer, fn_state);
        };
    }
    fn block_303<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_303_0: const #0u : u8
        let s_303_0: bool = false;
        // D s_303_1: write-var gs#383254 <= s_303_0
        fn_state.gs_383254 = s_303_0;
        // N s_303_2: jump b304
        return block_304(state, tracer, fn_state);
    }
    fn block_304<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_304_0: read-var gs#383254:u8
        let s_304_0: bool = fn_state.gs_383254;
        // D s_304_1: not s_304_0
        let s_304_1: bool = !s_304_0;
        // N s_304_2: branch s_304_1 b306 b305
        if s_304_1 {
            return block_306(state, tracer, fn_state);
        } else {
            return block_305(state, tracer, fn_state);
        };
    }
    fn block_305<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_305_0: const #737s : i
        let s_305_0: i128 = 737;
        // C s_305_1: const #14696u : u32
        let s_305_1: u32 = 14696;
        // N s_305_2: write-reg s_305_1 <= s_305_0
        let s_305_2: () = {
            state.write_register::<i128>(s_305_1 as isize, s_305_0);
            tracer.write_register(s_305_1 as isize, s_305_0);
        };
        // C s_305_3: const #0s : i
        let s_305_3: i128 = 0;
        // C s_305_4: const #5s : i
        let s_305_4: i128 = 5;
        // D s_305_5: read-var u#26066:u32
        let s_305_5: u32 = fn_state.u_26066;
        // D s_305_6: cast zx s_305_5 -> bv
        let s_305_6: Bits = Bits::new(s_305_5 as u128, 32u16);
        // D s_305_7: bit-extract s_305_6 s_305_3 s_305_4
        let s_305_7: Bits = (Bits::new(
            ((s_305_6) >> (s_305_3)).value(),
            u16::try_from(s_305_4).unwrap(),
        ));
        // D s_305_8: cast reint s_305_7 -> u8
        let s_305_8: u8 = (s_305_7.value() as u8);
        // C s_305_9: const #5s : i
        let s_305_9: i128 = 5;
        // C s_305_10: const #5s : i
        let s_305_10: i128 = 5;
        // D s_305_11: read-var u#26066:u32
        let s_305_11: u32 = fn_state.u_26066;
        // D s_305_12: cast zx s_305_11 -> bv
        let s_305_12: Bits = Bits::new(s_305_11 as u128, 32u16);
        // D s_305_13: bit-extract s_305_12 s_305_9 s_305_10
        let s_305_13: Bits = (Bits::new(
            ((s_305_12) >> (s_305_9)).value(),
            u16::try_from(s_305_10).unwrap(),
        ));
        // D s_305_14: cast reint s_305_13 -> u8
        let s_305_14: u8 = (s_305_13.value() as u8);
        // C s_305_15: const #10s : i
        let s_305_15: i128 = 10;
        // C s_305_16: const #2s : i
        let s_305_16: i128 = 2;
        // D s_305_17: read-var u#26066:u32
        let s_305_17: u32 = fn_state.u_26066;
        // D s_305_18: cast zx s_305_17 -> bv
        let s_305_18: Bits = Bits::new(s_305_17 as u128, 32u16);
        // D s_305_19: bit-extract s_305_18 s_305_15 s_305_16
        let s_305_19: Bits = (Bits::new(
            ((s_305_18) >> (s_305_15)).value(),
            u16::try_from(s_305_16).unwrap(),
        ));
        // D s_305_20: cast reint s_305_19 -> u8
        let s_305_20: u8 = (s_305_19.value() as u8);
        // C s_305_21: const #31s : i
        let s_305_21: i128 = 31;
        // C s_305_22: const #1s : i
        let s_305_22: i128 = 1;
        // D s_305_23: read-var u#26066:u32
        let s_305_23: u32 = fn_state.u_26066;
        // D s_305_24: cast zx s_305_23 -> bv
        let s_305_24: Bits = Bits::new(s_305_23 as u128, 32u16);
        // D s_305_25: bit-extract s_305_24 s_305_21 s_305_22
        let s_305_25: Bits = (Bits::new(
            ((s_305_24) >> (s_305_21)).value(),
            u16::try_from(s_305_22).unwrap(),
        ));
        // D s_305_26: cast reint s_305_25 -> u8
        let s_305_26: bool = ((s_305_25.value()) != 0);
        // D s_305_27: call decode_rev16_int_aarch64_instrs_integer_arithmetic_rev(s_305_8, s_305_14, s_305_20, s_305_26)
        let s_305_27: () = decode_rev16_int_aarch64_instrs_integer_arithmetic_rev(
            state,
            tracer,
            s_305_8,
            s_305_14,
            s_305_20,
            s_305_26,
        );
        // N s_305_28: return
        return;
    }
    fn block_306<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_306_0: read-var merge#var.1:struct
        let s_306_0: u32 = fn_state.merge_var._1;
        // D s_306_1: write-var u#26072 <= s_306_0
        fn_state.u_26072 = s_306_0;
        // C s_306_2: const #10s : i
        let s_306_2: i128 = 10;
        // D s_306_3: read-var u#26072:u32
        let s_306_3: u32 = fn_state.u_26072;
        // D s_306_4: cast zx s_306_3 -> bv
        let s_306_4: Bits = Bits::new(s_306_3 as u128, 32u16);
        // C s_306_5: const #1s : i64
        let s_306_5: i64 = 1;
        // C s_306_6: cast zx s_306_5 -> i
        let s_306_6: i128 = (i128::try_from(s_306_5).unwrap());
        // C s_306_7: const #21s : i
        let s_306_7: i128 = 21;
        // C s_306_8: add s_306_7 s_306_6
        let s_306_8: i128 = (s_306_7 + s_306_6);
        // D s_306_9: bit-extract s_306_4 s_306_2 s_306_8
        let s_306_9: Bits = (Bits::new(
            ((s_306_4) >> (s_306_2)).value(),
            u16::try_from(s_306_8).unwrap(),
        ));
        // D s_306_10: cast reint s_306_9 -> u22
        let s_306_10: u32 = (s_306_9.value() as u32);
        // D s_306_11: cast zx s_306_10 -> bv
        let s_306_11: Bits = Bits::new(s_306_10 as u128, 22u16);
        // C s_306_12: const #3584002u : u22
        let s_306_12: u32 = 3584002;
        // C s_306_13: cast zx s_306_12 -> bv
        let s_306_13: Bits = Bits::new(s_306_12 as u128, 22u16);
        // D s_306_14: cmp-eq s_306_11 s_306_13
        let s_306_14: bool = ((s_306_11) == (s_306_13));
        // N s_306_15: branch s_306_14 b481 b307
        if s_306_14 {
            return block_481(state, tracer, fn_state);
        } else {
            return block_307(state, tracer, fn_state);
        };
    }
    fn block_307<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_307_0: const #0u : u8
        let s_307_0: bool = false;
        // D s_307_1: write-var gs#383269 <= s_307_0
        fn_state.gs_383269 = s_307_0;
        // N s_307_2: jump b308
        return block_308(state, tracer, fn_state);
    }
    fn block_308<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_308_0: read-var gs#383269:u8
        let s_308_0: bool = fn_state.gs_383269;
        // D s_308_1: not s_308_0
        let s_308_1: bool = !s_308_0;
        // N s_308_2: branch s_308_1 b310 b309
        if s_308_1 {
            return block_310(state, tracer, fn_state);
        } else {
            return block_309(state, tracer, fn_state);
        };
    }
    fn block_309<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_309_0: const #738s : i
        let s_309_0: i128 = 738;
        // C s_309_1: const #14696u : u32
        let s_309_1: u32 = 14696;
        // N s_309_2: write-reg s_309_1 <= s_309_0
        let s_309_2: () = {
            state.write_register::<i128>(s_309_1 as isize, s_309_0);
            tracer.write_register(s_309_1 as isize, s_309_0);
        };
        // C s_309_3: const #0s : i
        let s_309_3: i128 = 0;
        // C s_309_4: const #5s : i
        let s_309_4: i128 = 5;
        // D s_309_5: read-var u#26072:u32
        let s_309_5: u32 = fn_state.u_26072;
        // D s_309_6: cast zx s_309_5 -> bv
        let s_309_6: Bits = Bits::new(s_309_5 as u128, 32u16);
        // D s_309_7: bit-extract s_309_6 s_309_3 s_309_4
        let s_309_7: Bits = (Bits::new(
            ((s_309_6) >> (s_309_3)).value(),
            u16::try_from(s_309_4).unwrap(),
        ));
        // D s_309_8: cast reint s_309_7 -> u8
        let s_309_8: u8 = (s_309_7.value() as u8);
        // C s_309_9: const #5s : i
        let s_309_9: i128 = 5;
        // C s_309_10: const #5s : i
        let s_309_10: i128 = 5;
        // D s_309_11: read-var u#26072:u32
        let s_309_11: u32 = fn_state.u_26072;
        // D s_309_12: cast zx s_309_11 -> bv
        let s_309_12: Bits = Bits::new(s_309_11 as u128, 32u16);
        // D s_309_13: bit-extract s_309_12 s_309_9 s_309_10
        let s_309_13: Bits = (Bits::new(
            ((s_309_12) >> (s_309_9)).value(),
            u16::try_from(s_309_10).unwrap(),
        ));
        // D s_309_14: cast reint s_309_13 -> u8
        let s_309_14: u8 = (s_309_13.value() as u8);
        // C s_309_15: const #10s : i
        let s_309_15: i128 = 10;
        // C s_309_16: const #2s : i
        let s_309_16: i128 = 2;
        // D s_309_17: read-var u#26072:u32
        let s_309_17: u32 = fn_state.u_26072;
        // D s_309_18: cast zx s_309_17 -> bv
        let s_309_18: Bits = Bits::new(s_309_17 as u128, 32u16);
        // D s_309_19: bit-extract s_309_18 s_309_15 s_309_16
        let s_309_19: Bits = (Bits::new(
            ((s_309_18) >> (s_309_15)).value(),
            u16::try_from(s_309_16).unwrap(),
        ));
        // D s_309_20: cast reint s_309_19 -> u8
        let s_309_20: u8 = (s_309_19.value() as u8);
        // C s_309_21: const #31s : i
        let s_309_21: i128 = 31;
        // C s_309_22: const #1s : i
        let s_309_22: i128 = 1;
        // D s_309_23: read-var u#26072:u32
        let s_309_23: u32 = fn_state.u_26072;
        // D s_309_24: cast zx s_309_23 -> bv
        let s_309_24: Bits = Bits::new(s_309_23 as u128, 32u16);
        // D s_309_25: bit-extract s_309_24 s_309_21 s_309_22
        let s_309_25: Bits = (Bits::new(
            ((s_309_24) >> (s_309_21)).value(),
            u16::try_from(s_309_22).unwrap(),
        ));
        // D s_309_26: cast reint s_309_25 -> u8
        let s_309_26: bool = ((s_309_25.value()) != 0);
        // D s_309_27: call decode_rev32_int_aarch64_instrs_integer_arithmetic_rev(s_309_8, s_309_14, s_309_20, s_309_26)
        let s_309_27: () = decode_rev32_int_aarch64_instrs_integer_arithmetic_rev(
            state,
            tracer,
            s_309_8,
            s_309_14,
            s_309_20,
            s_309_26,
        );
        // N s_309_28: return
        return;
    }
    fn block_310<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_310_0: read-var merge#var.1:struct
        let s_310_0: u32 = fn_state.merge_var._1;
        // D s_310_1: write-var u#26078 <= s_310_0
        fn_state.u_26078 = s_310_0;
        // C s_310_2: const #21s : i
        let s_310_2: i128 = 21;
        // D s_310_3: read-var u#26078:u32
        let s_310_3: u32 = fn_state.u_26078;
        // D s_310_4: cast zx s_310_3 -> bv
        let s_310_4: Bits = Bits::new(s_310_3 as u128, 32u16);
        // C s_310_5: const #1s : i64
        let s_310_5: i64 = 1;
        // C s_310_6: cast zx s_310_5 -> i
        let s_310_6: i128 = (i128::try_from(s_310_5).unwrap());
        // C s_310_7: const #10s : i
        let s_310_7: i128 = 10;
        // C s_310_8: add s_310_7 s_310_6
        let s_310_8: i128 = (s_310_7 + s_310_6);
        // D s_310_9: bit-extract s_310_4 s_310_2 s_310_8
        let s_310_9: Bits = (Bits::new(
            ((s_310_4) >> (s_310_2)).value(),
            u16::try_from(s_310_8).unwrap(),
        ));
        // D s_310_10: cast reint s_310_9 -> u11
        let s_310_10: u16 = (s_310_9.value() as u16);
        // D s_310_11: cast zx s_310_10 -> bv
        let s_310_11: Bits = Bits::new(s_310_10 as u128, 11u16);
        // C s_310_12: const #1488u : u11
        let s_310_12: u16 = 1488;
        // C s_310_13: cast zx s_310_12 -> bv
        let s_310_13: Bits = Bits::new(s_310_12 as u128, 11u16);
        // D s_310_14: cmp-eq s_310_11 s_310_13
        let s_310_14: bool = ((s_310_11) == (s_310_13));
        // N s_310_15: branch s_310_14 b477 b311
        if s_310_14 {
            return block_477(state, tracer, fn_state);
        } else {
            return block_311(state, tracer, fn_state);
        };
    }
    fn block_311<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_311_0: const #0u : u8
        let s_311_0: bool = false;
        // D s_311_1: write-var gs#383288 <= s_311_0
        fn_state.gs_383288 = s_311_0;
        // N s_311_2: jump b312
        return block_312(state, tracer, fn_state);
    }
    fn block_312<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_312_0: read-var gs#383288:u8
        let s_312_0: bool = fn_state.gs_383288;
        // N s_312_1: branch s_312_0 b476 b313
        if s_312_0 {
            return block_476(state, tracer, fn_state);
        } else {
            return block_313(state, tracer, fn_state);
        };
    }
    fn block_313<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_313_0: const #0u : u8
        let s_313_0: bool = false;
        // D s_313_1: write-var gs#383290 <= s_313_0
        fn_state.gs_383290 = s_313_0;
        // N s_313_2: jump b314
        return block_314(state, tracer, fn_state);
    }
    fn block_314<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_314_0: read-var gs#383290:u8
        let s_314_0: bool = fn_state.gs_383290;
        // D s_314_1: not s_314_0
        let s_314_1: bool = !s_314_0;
        // N s_314_2: branch s_314_1 b316 b315
        if s_314_1 {
            return block_316(state, tracer, fn_state);
        } else {
            return block_315(state, tracer, fn_state);
        };
    }
    fn block_315<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_315_0: const #742s : i
        let s_315_0: i128 = 742;
        // C s_315_1: const #14696u : u32
        let s_315_1: u32 = 14696;
        // N s_315_2: write-reg s_315_1 <= s_315_0
        let s_315_2: () = {
            state.write_register::<i128>(s_315_1 as isize, s_315_0);
            tracer.write_register(s_315_1 as isize, s_315_0);
        };
        // C s_315_3: const #0s : i
        let s_315_3: i128 = 0;
        // C s_315_4: const #4s : i
        let s_315_4: i128 = 4;
        // D s_315_5: read-var u#26078:u32
        let s_315_5: u32 = fn_state.u_26078;
        // D s_315_6: cast zx s_315_5 -> bv
        let s_315_6: Bits = Bits::new(s_315_5 as u128, 32u16);
        // D s_315_7: bit-extract s_315_6 s_315_3 s_315_4
        let s_315_7: Bits = (Bits::new(
            ((s_315_6) >> (s_315_3)).value(),
            u16::try_from(s_315_4).unwrap(),
        ));
        // D s_315_8: cast reint s_315_7 -> u8
        let s_315_8: u8 = (s_315_7.value() as u8);
        // C s_315_9: const #5s : i
        let s_315_9: i128 = 5;
        // C s_315_10: const #5s : i
        let s_315_10: i128 = 5;
        // D s_315_11: read-var u#26078:u32
        let s_315_11: u32 = fn_state.u_26078;
        // D s_315_12: cast zx s_315_11 -> bv
        let s_315_12: Bits = Bits::new(s_315_11 as u128, 32u16);
        // D s_315_13: bit-extract s_315_12 s_315_9 s_315_10
        let s_315_13: Bits = (Bits::new(
            ((s_315_12) >> (s_315_9)).value(),
            u16::try_from(s_315_10).unwrap(),
        ));
        // D s_315_14: cast reint s_315_13 -> u8
        let s_315_14: u8 = (s_315_13.value() as u8);
        // C s_315_15: const #15s : i
        let s_315_15: i128 = 15;
        // C s_315_16: const #6s : i
        let s_315_16: i128 = 6;
        // D s_315_17: read-var u#26078:u32
        let s_315_17: u32 = fn_state.u_26078;
        // D s_315_18: cast zx s_315_17 -> bv
        let s_315_18: Bits = Bits::new(s_315_17 as u128, 32u16);
        // D s_315_19: bit-extract s_315_18 s_315_15 s_315_16
        let s_315_19: Bits = (Bits::new(
            ((s_315_18) >> (s_315_15)).value(),
            u16::try_from(s_315_16).unwrap(),
        ));
        // D s_315_20: cast reint s_315_19 -> u8
        let s_315_20: u8 = (s_315_19.value() as u8);
        // C s_315_21: const #31s : i
        let s_315_21: i128 = 31;
        // C s_315_22: const #1s : i
        let s_315_22: i128 = 1;
        // D s_315_23: read-var u#26078:u32
        let s_315_23: u32 = fn_state.u_26078;
        // D s_315_24: cast zx s_315_23 -> bv
        let s_315_24: Bits = Bits::new(s_315_23 as u128, 32u16);
        // D s_315_25: bit-extract s_315_24 s_315_21 s_315_22
        let s_315_25: Bits = (Bits::new(
            ((s_315_24) >> (s_315_21)).value(),
            u16::try_from(s_315_22).unwrap(),
        ));
        // D s_315_26: cast reint s_315_25 -> u8
        let s_315_26: bool = ((s_315_25.value()) != 0);
        // D s_315_27: call decode_rmif_aarch64_instrs_integer_flags_rmif(s_315_8, s_315_14, s_315_20, s_315_26)
        let s_315_27: () = decode_rmif_aarch64_instrs_integer_flags_rmif(
            state,
            tracer,
            s_315_8,
            s_315_14,
            s_315_20,
            s_315_26,
        );
        // N s_315_28: return
        return;
    }
    fn block_316<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_316_0: read-var merge#var.1:struct
        let s_316_0: u32 = fn_state.merge_var._1;
        // D s_316_1: write-var u#26083 <= s_316_0
        fn_state.u_26083 = s_316_0;
        // C s_316_2: const #21s : i
        let s_316_2: i128 = 21;
        // D s_316_3: read-var u#26083:u32
        let s_316_3: u32 = fn_state.u_26083;
        // D s_316_4: cast zx s_316_3 -> bv
        let s_316_4: Bits = Bits::new(s_316_3 as u128, 32u16);
        // C s_316_5: const #1s : i64
        let s_316_5: i64 = 1;
        // C s_316_6: cast zx s_316_5 -> i
        let s_316_6: i128 = (i128::try_from(s_316_5).unwrap());
        // C s_316_7: const #9s : i
        let s_316_7: i128 = 9;
        // C s_316_8: add s_316_7 s_316_6
        let s_316_8: i128 = (s_316_7 + s_316_6);
        // D s_316_9: bit-extract s_316_4 s_316_2 s_316_8
        let s_316_9: Bits = (Bits::new(
            ((s_316_4) >> (s_316_2)).value(),
            u16::try_from(s_316_8).unwrap(),
        ));
        // D s_316_10: cast reint s_316_9 -> u10
        let s_316_10: u16 = (s_316_9.value() as u16);
        // D s_316_11: cast zx s_316_10 -> bv
        let s_316_11: Bits = Bits::new(s_316_10 as u128, 10u16);
        // C s_316_12: const #214u : u10
        let s_316_12: u16 = 214;
        // C s_316_13: cast zx s_316_12 -> bv
        let s_316_13: Bits = Bits::new(s_316_12 as u128, 10u16);
        // D s_316_14: cmp-eq s_316_11 s_316_13
        let s_316_14: bool = ((s_316_11) == (s_316_13));
        // N s_316_15: branch s_316_14 b475 b317
        if s_316_14 {
            return block_475(state, tracer, fn_state);
        } else {
            return block_317(state, tracer, fn_state);
        };
    }
    fn block_317<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_317_0: const #0u : u8
        let s_317_0: bool = false;
        // D s_317_1: write-var gs#383306 <= s_317_0
        fn_state.gs_383306 = s_317_0;
        // N s_317_2: jump b318
        return block_318(state, tracer, fn_state);
    }
    fn block_318<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_318_0: read-var gs#383306:u8
        let s_318_0: bool = fn_state.gs_383306;
        // N s_318_1: branch s_318_0 b474 b319
        if s_318_0 {
            return block_474(state, tracer, fn_state);
        } else {
            return block_319(state, tracer, fn_state);
        };
    }
    fn block_319<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_319_0: const #0u : u8
        let s_319_0: bool = false;
        // D s_319_1: write-var gs#383308 <= s_319_0
        fn_state.gs_383308 = s_319_0;
        // N s_319_2: jump b320
        return block_320(state, tracer, fn_state);
    }
    fn block_320<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_320_0: read-var gs#383308:u8
        let s_320_0: bool = fn_state.gs_383308;
        // D s_320_1: not s_320_0
        let s_320_1: bool = !s_320_0;
        // N s_320_2: branch s_320_1 b322 b321
        if s_320_1 {
            return block_322(state, tracer, fn_state);
        } else {
            return block_321(state, tracer, fn_state);
        };
    }
    fn block_321<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_321_0: const #781s : i
        let s_321_0: i128 = 781;
        // C s_321_1: const #14696u : u32
        let s_321_1: u32 = 14696;
        // N s_321_2: write-reg s_321_1 <= s_321_0
        let s_321_2: () = {
            state.write_register::<i128>(s_321_1 as isize, s_321_0);
            tracer.write_register(s_321_1 as isize, s_321_0);
        };
        // C s_321_3: const #0s : i
        let s_321_3: i128 = 0;
        // C s_321_4: const #5s : i
        let s_321_4: i128 = 5;
        // D s_321_5: read-var u#26083:u32
        let s_321_5: u32 = fn_state.u_26083;
        // D s_321_6: cast zx s_321_5 -> bv
        let s_321_6: Bits = Bits::new(s_321_5 as u128, 32u16);
        // D s_321_7: bit-extract s_321_6 s_321_3 s_321_4
        let s_321_7: Bits = (Bits::new(
            ((s_321_6) >> (s_321_3)).value(),
            u16::try_from(s_321_4).unwrap(),
        ));
        // D s_321_8: cast reint s_321_7 -> u8
        let s_321_8: u8 = (s_321_7.value() as u8);
        // C s_321_9: const #5s : i
        let s_321_9: i128 = 5;
        // C s_321_10: const #5s : i
        let s_321_10: i128 = 5;
        // D s_321_11: read-var u#26083:u32
        let s_321_11: u32 = fn_state.u_26083;
        // D s_321_12: cast zx s_321_11 -> bv
        let s_321_12: Bits = Bits::new(s_321_11 as u128, 32u16);
        // D s_321_13: bit-extract s_321_12 s_321_9 s_321_10
        let s_321_13: Bits = (Bits::new(
            ((s_321_12) >> (s_321_9)).value(),
            u16::try_from(s_321_10).unwrap(),
        ));
        // D s_321_14: cast reint s_321_13 -> u8
        let s_321_14: u8 = (s_321_13.value() as u8);
        // C s_321_15: const #10s : i
        let s_321_15: i128 = 10;
        // C s_321_16: const #1s : i
        let s_321_16: i128 = 1;
        // D s_321_17: read-var u#26083:u32
        let s_321_17: u32 = fn_state.u_26083;
        // D s_321_18: cast zx s_321_17 -> bv
        let s_321_18: Bits = Bits::new(s_321_17 as u128, 32u16);
        // D s_321_19: bit-extract s_321_18 s_321_15 s_321_16
        let s_321_19: Bits = (Bits::new(
            ((s_321_18) >> (s_321_15)).value(),
            u16::try_from(s_321_16).unwrap(),
        ));
        // D s_321_20: cast reint s_321_19 -> u8
        let s_321_20: bool = ((s_321_19.value()) != 0);
        // C s_321_21: const #16s : i
        let s_321_21: i128 = 16;
        // C s_321_22: const #5s : i
        let s_321_22: i128 = 5;
        // D s_321_23: read-var u#26083:u32
        let s_321_23: u32 = fn_state.u_26083;
        // D s_321_24: cast zx s_321_23 -> bv
        let s_321_24: Bits = Bits::new(s_321_23 as u128, 32u16);
        // D s_321_25: bit-extract s_321_24 s_321_21 s_321_22
        let s_321_25: Bits = (Bits::new(
            ((s_321_24) >> (s_321_21)).value(),
            u16::try_from(s_321_22).unwrap(),
        ));
        // D s_321_26: cast reint s_321_25 -> u8
        let s_321_26: u8 = (s_321_25.value() as u8);
        // C s_321_27: const #31s : i
        let s_321_27: i128 = 31;
        // C s_321_28: const #1s : i
        let s_321_28: i128 = 1;
        // D s_321_29: read-var u#26083:u32
        let s_321_29: u32 = fn_state.u_26083;
        // D s_321_30: cast zx s_321_29 -> bv
        let s_321_30: Bits = Bits::new(s_321_29 as u128, 32u16);
        // D s_321_31: bit-extract s_321_30 s_321_27 s_321_28
        let s_321_31: Bits = (Bits::new(
            ((s_321_30) >> (s_321_27)).value(),
            u16::try_from(s_321_28).unwrap(),
        ));
        // D s_321_32: cast reint s_321_31 -> u8
        let s_321_32: bool = ((s_321_31.value()) != 0);
        // D s_321_33: call decode_sdiv_aarch64_instrs_integer_arithmetic_div(s_321_8, s_321_14, s_321_20, s_321_26, s_321_32)
        let s_321_33: () = decode_sdiv_aarch64_instrs_integer_arithmetic_div(
            state,
            tracer,
            s_321_8,
            s_321_14,
            s_321_20,
            s_321_26,
            s_321_32,
        );
        // N s_321_34: return
        return;
    }
    fn block_322<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_322_0: read-var merge#var.1:struct
        let s_322_0: u32 = fn_state.merge_var._1;
        // D s_322_1: write-var u#26089 <= s_322_0
        fn_state.u_26089 = s_322_0;
        // C s_322_2: const #21s : i
        let s_322_2: i128 = 21;
        // D s_322_3: read-var u#26089:u32
        let s_322_3: u32 = fn_state.u_26089;
        // D s_322_4: cast zx s_322_3 -> bv
        let s_322_4: Bits = Bits::new(s_322_3 as u128, 32u16);
        // C s_322_5: const #1s : i64
        let s_322_5: i64 = 1;
        // C s_322_6: cast zx s_322_5 -> i
        let s_322_6: i128 = (i128::try_from(s_322_5).unwrap());
        // C s_322_7: const #9s : i
        let s_322_7: i128 = 9;
        // C s_322_8: add s_322_7 s_322_6
        let s_322_8: i128 = (s_322_7 + s_322_6);
        // D s_322_9: bit-extract s_322_4 s_322_2 s_322_8
        let s_322_9: Bits = (Bits::new(
            ((s_322_4) >> (s_322_2)).value(),
            u16::try_from(s_322_8).unwrap(),
        ));
        // D s_322_10: cast reint s_322_9 -> u10
        let s_322_10: u16 = (s_322_9.value() as u16);
        // D s_322_11: cast zx s_322_10 -> bv
        let s_322_11: Bits = Bits::new(s_322_10 as u128, 10u16);
        // C s_322_12: const #214u : u10
        let s_322_12: u16 = 214;
        // C s_322_13: cast zx s_322_12 -> bv
        let s_322_13: Bits = Bits::new(s_322_12 as u128, 10u16);
        // D s_322_14: cmp-eq s_322_11 s_322_13
        let s_322_14: bool = ((s_322_11) == (s_322_13));
        // N s_322_15: branch s_322_14 b473 b323
        if s_322_14 {
            return block_473(state, tracer, fn_state);
        } else {
            return block_323(state, tracer, fn_state);
        };
    }
    fn block_323<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_323_0: const #0u : u8
        let s_323_0: bool = false;
        // D s_323_1: write-var gs#383326 <= s_323_0
        fn_state.gs_383326 = s_323_0;
        // N s_323_2: jump b324
        return block_324(state, tracer, fn_state);
    }
    fn block_324<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_324_0: read-var gs#383326:u8
        let s_324_0: bool = fn_state.gs_383326;
        // N s_324_1: branch s_324_0 b472 b325
        if s_324_0 {
            return block_472(state, tracer, fn_state);
        } else {
            return block_325(state, tracer, fn_state);
        };
    }
    fn block_325<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_325_0: const #0u : u8
        let s_325_0: bool = false;
        // D s_325_1: write-var gs#383328 <= s_325_0
        fn_state.gs_383328 = s_325_0;
        // N s_325_2: jump b326
        return block_326(state, tracer, fn_state);
    }
    fn block_326<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_326_0: read-var gs#383328:u8
        let s_326_0: bool = fn_state.gs_383328;
        // D s_326_1: not s_326_0
        let s_326_1: bool = !s_326_0;
        // N s_326_2: branch s_326_1 b328 b327
        if s_326_1 {
            return block_328(state, tracer, fn_state);
        } else {
            return block_327(state, tracer, fn_state);
        };
    }
    fn block_327<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_327_0: const #782s : i
        let s_327_0: i128 = 782;
        // C s_327_1: const #14696u : u32
        let s_327_1: u32 = 14696;
        // N s_327_2: write-reg s_327_1 <= s_327_0
        let s_327_2: () = {
            state.write_register::<i128>(s_327_1 as isize, s_327_0);
            tracer.write_register(s_327_1 as isize, s_327_0);
        };
        // C s_327_3: const #0s : i
        let s_327_3: i128 = 0;
        // C s_327_4: const #5s : i
        let s_327_4: i128 = 5;
        // D s_327_5: read-var u#26089:u32
        let s_327_5: u32 = fn_state.u_26089;
        // D s_327_6: cast zx s_327_5 -> bv
        let s_327_6: Bits = Bits::new(s_327_5 as u128, 32u16);
        // D s_327_7: bit-extract s_327_6 s_327_3 s_327_4
        let s_327_7: Bits = (Bits::new(
            ((s_327_6) >> (s_327_3)).value(),
            u16::try_from(s_327_4).unwrap(),
        ));
        // D s_327_8: cast reint s_327_7 -> u8
        let s_327_8: u8 = (s_327_7.value() as u8);
        // C s_327_9: const #5s : i
        let s_327_9: i128 = 5;
        // C s_327_10: const #5s : i
        let s_327_10: i128 = 5;
        // D s_327_11: read-var u#26089:u32
        let s_327_11: u32 = fn_state.u_26089;
        // D s_327_12: cast zx s_327_11 -> bv
        let s_327_12: Bits = Bits::new(s_327_11 as u128, 32u16);
        // D s_327_13: bit-extract s_327_12 s_327_9 s_327_10
        let s_327_13: Bits = (Bits::new(
            ((s_327_12) >> (s_327_9)).value(),
            u16::try_from(s_327_10).unwrap(),
        ));
        // D s_327_14: cast reint s_327_13 -> u8
        let s_327_14: u8 = (s_327_13.value() as u8);
        // C s_327_15: const #10s : i
        let s_327_15: i128 = 10;
        // C s_327_16: const #1s : i
        let s_327_16: i128 = 1;
        // D s_327_17: read-var u#26089:u32
        let s_327_17: u32 = fn_state.u_26089;
        // D s_327_18: cast zx s_327_17 -> bv
        let s_327_18: Bits = Bits::new(s_327_17 as u128, 32u16);
        // D s_327_19: bit-extract s_327_18 s_327_15 s_327_16
        let s_327_19: Bits = (Bits::new(
            ((s_327_18) >> (s_327_15)).value(),
            u16::try_from(s_327_16).unwrap(),
        ));
        // D s_327_20: cast reint s_327_19 -> u8
        let s_327_20: bool = ((s_327_19.value()) != 0);
        // C s_327_21: const #16s : i
        let s_327_21: i128 = 16;
        // C s_327_22: const #5s : i
        let s_327_22: i128 = 5;
        // D s_327_23: read-var u#26089:u32
        let s_327_23: u32 = fn_state.u_26089;
        // D s_327_24: cast zx s_327_23 -> bv
        let s_327_24: Bits = Bits::new(s_327_23 as u128, 32u16);
        // D s_327_25: bit-extract s_327_24 s_327_21 s_327_22
        let s_327_25: Bits = (Bits::new(
            ((s_327_24) >> (s_327_21)).value(),
            u16::try_from(s_327_22).unwrap(),
        ));
        // D s_327_26: cast reint s_327_25 -> u8
        let s_327_26: u8 = (s_327_25.value() as u8);
        // C s_327_27: const #31s : i
        let s_327_27: i128 = 31;
        // C s_327_28: const #1s : i
        let s_327_28: i128 = 1;
        // D s_327_29: read-var u#26089:u32
        let s_327_29: u32 = fn_state.u_26089;
        // D s_327_30: cast zx s_327_29 -> bv
        let s_327_30: Bits = Bits::new(s_327_29 as u128, 32u16);
        // D s_327_31: bit-extract s_327_30 s_327_27 s_327_28
        let s_327_31: Bits = (Bits::new(
            ((s_327_30) >> (s_327_27)).value(),
            u16::try_from(s_327_28).unwrap(),
        ));
        // D s_327_32: cast reint s_327_31 -> u8
        let s_327_32: bool = ((s_327_31.value()) != 0);
        // D s_327_33: call decode_udiv_aarch64_instrs_integer_arithmetic_div(s_327_8, s_327_14, s_327_20, s_327_26, s_327_32)
        let s_327_33: () = decode_udiv_aarch64_instrs_integer_arithmetic_div(
            state,
            tracer,
            s_327_8,
            s_327_14,
            s_327_20,
            s_327_26,
            s_327_32,
        );
        // N s_327_34: return
        return;
    }
    fn block_328<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_328_0: read-var merge#var.1:struct
        let s_328_0: u32 = fn_state.merge_var._1;
        // D s_328_1: write-var u#26096 <= s_328_0
        fn_state.u_26096 = s_328_0;
        // C s_328_2: const #15s : i
        let s_328_2: i128 = 15;
        // D s_328_3: read-var u#26096:u32
        let s_328_3: u32 = fn_state.u_26096;
        // D s_328_4: cast zx s_328_3 -> bv
        let s_328_4: Bits = Bits::new(s_328_3 as u128, 32u16);
        // C s_328_5: const #1s : i64
        let s_328_5: i64 = 1;
        // C s_328_6: cast zx s_328_5 -> i
        let s_328_6: i128 = (i128::try_from(s_328_5).unwrap());
        // C s_328_7: const #16s : i
        let s_328_7: i128 = 16;
        // C s_328_8: add s_328_7 s_328_6
        let s_328_8: i128 = (s_328_7 + s_328_6);
        // D s_328_9: bit-extract s_328_4 s_328_2 s_328_8
        let s_328_9: Bits = (Bits::new(
            ((s_328_4) >> (s_328_2)).value(),
            u16::try_from(s_328_8).unwrap(),
        ));
        // D s_328_10: cast reint s_328_9 -> u17
        let s_328_10: u32 = (s_328_9.value() as u32);
        // D s_328_11: cast zx s_328_10 -> bv
        let s_328_11: Bits = Bits::new(s_328_10 as u128, 17u16);
        // C s_328_12: const #29696u : u17
        let s_328_12: u32 = 29696;
        // C s_328_13: cast zx s_328_12 -> bv
        let s_328_13: Bits = Bits::new(s_328_12 as u128, 17u16);
        // D s_328_14: cmp-eq s_328_11 s_328_13
        let s_328_14: bool = ((s_328_11) == (s_328_13));
        // N s_328_15: branch s_328_14 b468 b329
        if s_328_14 {
            return block_468(state, tracer, fn_state);
        } else {
            return block_329(state, tracer, fn_state);
        };
    }
    fn block_329<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_329_0: const #0u : u8
        let s_329_0: bool = false;
        // D s_329_1: write-var gs#383349 <= s_329_0
        fn_state.gs_383349 = s_329_0;
        // N s_329_2: jump b330
        return block_330(state, tracer, fn_state);
    }
    fn block_330<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_330_0: read-var gs#383349:u8
        let s_330_0: bool = fn_state.gs_383349;
        // N s_330_1: branch s_330_0 b467 b331
        if s_330_0 {
            return block_467(state, tracer, fn_state);
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
        // D s_331_1: write-var gs#383351 <= s_331_0
        fn_state.gs_383351 = s_331_0;
        // N s_331_2: jump b332
        return block_332(state, tracer, fn_state);
    }
    fn block_332<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_332_0: read-var gs#383351:u8
        let s_332_0: bool = fn_state.gs_383351;
        // D s_332_1: not s_332_0
        let s_332_1: bool = !s_332_0;
        // N s_332_2: branch s_332_1 b334 b333
        if s_332_1 {
            return block_334(state, tracer, fn_state);
        } else {
            return block_333(state, tracer, fn_state);
        };
    }
    fn block_333<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_333_0: const #791s : i
        let s_333_0: i128 = 791;
        // C s_333_1: const #14696u : u32
        let s_333_1: u32 = 14696;
        // N s_333_2: write-reg s_333_1 <= s_333_0
        let s_333_2: () = {
            state.write_register::<i128>(s_333_1 as isize, s_333_0);
            tracer.write_register(s_333_1 as isize, s_333_0);
        };
        // C s_333_3: const #5s : i
        let s_333_3: i128 = 5;
        // C s_333_4: const #5s : i
        let s_333_4: i128 = 5;
        // D s_333_5: read-var u#26096:u32
        let s_333_5: u32 = fn_state.u_26096;
        // D s_333_6: cast zx s_333_5 -> bv
        let s_333_6: Bits = Bits::new(s_333_5 as u128, 32u16);
        // D s_333_7: bit-extract s_333_6 s_333_3 s_333_4
        let s_333_7: Bits = (Bits::new(
            ((s_333_6) >> (s_333_3)).value(),
            u16::try_from(s_333_4).unwrap(),
        ));
        // D s_333_8: cast reint s_333_7 -> u8
        let s_333_8: u8 = (s_333_7.value() as u8);
        // C s_333_9: const #14s : i
        let s_333_9: i128 = 14;
        // C s_333_10: const #1s : i
        let s_333_10: i128 = 1;
        // D s_333_11: read-var u#26096:u32
        let s_333_11: u32 = fn_state.u_26096;
        // D s_333_12: cast zx s_333_11 -> bv
        let s_333_12: Bits = Bits::new(s_333_11 as u128, 32u16);
        // D s_333_13: bit-extract s_333_12 s_333_9 s_333_10
        let s_333_13: Bits = (Bits::new(
            ((s_333_12) >> (s_333_9)).value(),
            u16::try_from(s_333_10).unwrap(),
        ));
        // D s_333_14: cast reint s_333_13 -> u8
        let s_333_14: bool = ((s_333_13.value()) != 0);
        // C s_333_15: const #31s : i
        let s_333_15: i128 = 31;
        // C s_333_16: const #1s : i
        let s_333_16: i128 = 1;
        // D s_333_17: read-var u#26096:u32
        let s_333_17: u32 = fn_state.u_26096;
        // D s_333_18: cast zx s_333_17 -> bv
        let s_333_18: Bits = Bits::new(s_333_17 as u128, 32u16);
        // D s_333_19: bit-extract s_333_18 s_333_15 s_333_16
        let s_333_19: Bits = (Bits::new(
            ((s_333_18) >> (s_333_15)).value(),
            u16::try_from(s_333_16).unwrap(),
        ));
        // D s_333_20: cast reint s_333_19 -> u8
        let s_333_20: bool = ((s_333_19.value()) != 0);
        // D s_333_21: call decode_setf_aarch64_instrs_integer_flags_setf(s_333_8, s_333_14, s_333_20)
        let s_333_21: () = decode_setf_aarch64_instrs_integer_flags_setf(
            state,
            tracer,
            s_333_8,
            s_333_14,
            s_333_20,
        );
        // N s_333_22: return
        return;
    }
    fn block_334<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_334_0: read-var merge#var.1:struct
        let s_334_0: u32 = fn_state.merge_var._1;
        // D s_334_1: write-var u#26101 <= s_334_0
        fn_state.u_26101 = s_334_0;
        // C s_334_2: const #21s : i
        let s_334_2: i128 = 21;
        // D s_334_3: read-var u#26101:u32
        let s_334_3: u32 = fn_state.u_26101;
        // D s_334_4: cast zx s_334_3 -> bv
        let s_334_4: Bits = Bits::new(s_334_3 as u128, 32u16);
        // C s_334_5: const #1s : i64
        let s_334_5: i64 = 1;
        // C s_334_6: cast zx s_334_5 -> i
        let s_334_6: i128 = (i128::try_from(s_334_5).unwrap());
        // C s_334_7: const #10s : i
        let s_334_7: i128 = 10;
        // C s_334_8: add s_334_7 s_334_6
        let s_334_8: i128 = (s_334_7 + s_334_6);
        // D s_334_9: bit-extract s_334_4 s_334_2 s_334_8
        let s_334_9: Bits = (Bits::new(
            ((s_334_4) >> (s_334_2)).value(),
            u16::try_from(s_334_8).unwrap(),
        ));
        // D s_334_10: cast reint s_334_9 -> u11
        let s_334_10: u16 = (s_334_9.value() as u16);
        // D s_334_11: cast zx s_334_10 -> bv
        let s_334_11: Bits = Bits::new(s_334_10 as u128, 11u16);
        // C s_334_12: const #1241u : u11
        let s_334_12: u16 = 1241;
        // C s_334_13: cast zx s_334_12 -> bv
        let s_334_13: Bits = Bits::new(s_334_12 as u128, 11u16);
        // D s_334_14: cmp-eq s_334_11 s_334_13
        let s_334_14: bool = ((s_334_11) == (s_334_13));
        // N s_334_15: branch s_334_14 b466 b335
        if s_334_14 {
            return block_466(state, tracer, fn_state);
        } else {
            return block_335(state, tracer, fn_state);
        };
    }
    fn block_335<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_335_0: const #0u : u8
        let s_335_0: bool = false;
        // D s_335_1: write-var gs#383365 <= s_335_0
        fn_state.gs_383365 = s_335_0;
        // N s_335_2: jump b336
        return block_336(state, tracer, fn_state);
    }
    fn block_336<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_336_0: read-var gs#383365:u8
        let s_336_0: bool = fn_state.gs_383365;
        // N s_336_1: branch s_336_0 b465 b337
        if s_336_0 {
            return block_465(state, tracer, fn_state);
        } else {
            return block_337(state, tracer, fn_state);
        };
    }
    fn block_337<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_337_0: const #0u : u8
        let s_337_0: bool = false;
        // D s_337_1: write-var gs#383367 <= s_337_0
        fn_state.gs_383367 = s_337_0;
        // N s_337_2: jump b338
        return block_338(state, tracer, fn_state);
    }
    fn block_338<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_338_0: read-var gs#383367:u8
        let s_338_0: bool = fn_state.gs_383367;
        // D s_338_1: not s_338_0
        let s_338_1: bool = !s_338_0;
        // N s_338_2: branch s_338_1 b340 b339
        if s_338_1 {
            return block_340(state, tracer, fn_state);
        } else {
            return block_339(state, tracer, fn_state);
        };
    }
    fn block_339<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_339_0: const #828s : i
        let s_339_0: i128 = 828;
        // C s_339_1: const #14696u : u32
        let s_339_1: u32 = 14696;
        // N s_339_2: write-reg s_339_1 <= s_339_0
        let s_339_2: () = {
            state.write_register::<i128>(s_339_1 as isize, s_339_0);
            tracer.write_register(s_339_1 as isize, s_339_0);
        };
        // C s_339_3: const #0s : i
        let s_339_3: i128 = 0;
        // C s_339_4: const #5s : i
        let s_339_4: i128 = 5;
        // D s_339_5: read-var u#26101:u32
        let s_339_5: u32 = fn_state.u_26101;
        // D s_339_6: cast zx s_339_5 -> bv
        let s_339_6: Bits = Bits::new(s_339_5 as u128, 32u16);
        // D s_339_7: bit-extract s_339_6 s_339_3 s_339_4
        let s_339_7: Bits = (Bits::new(
            ((s_339_6) >> (s_339_3)).value(),
            u16::try_from(s_339_4).unwrap(),
        ));
        // D s_339_8: cast reint s_339_7 -> u8
        let s_339_8: u8 = (s_339_7.value() as u8);
        // C s_339_9: const #5s : i
        let s_339_9: i128 = 5;
        // C s_339_10: const #5s : i
        let s_339_10: i128 = 5;
        // D s_339_11: read-var u#26101:u32
        let s_339_11: u32 = fn_state.u_26101;
        // D s_339_12: cast zx s_339_11 -> bv
        let s_339_12: Bits = Bits::new(s_339_11 as u128, 32u16);
        // D s_339_13: bit-extract s_339_12 s_339_9 s_339_10
        let s_339_13: Bits = (Bits::new(
            ((s_339_12) >> (s_339_9)).value(),
            u16::try_from(s_339_10).unwrap(),
        ));
        // D s_339_14: cast reint s_339_13 -> u8
        let s_339_14: u8 = (s_339_13.value() as u8);
        // C s_339_15: const #10s : i
        let s_339_15: i128 = 10;
        // C s_339_16: const #5s : i
        let s_339_16: i128 = 5;
        // D s_339_17: read-var u#26101:u32
        let s_339_17: u32 = fn_state.u_26101;
        // D s_339_18: cast zx s_339_17 -> bv
        let s_339_18: Bits = Bits::new(s_339_17 as u128, 32u16);
        // D s_339_19: bit-extract s_339_18 s_339_15 s_339_16
        let s_339_19: Bits = (Bits::new(
            ((s_339_18) >> (s_339_15)).value(),
            u16::try_from(s_339_16).unwrap(),
        ));
        // D s_339_20: cast reint s_339_19 -> u8
        let s_339_20: u8 = (s_339_19.value() as u8);
        // C s_339_21: const #15s : i
        let s_339_21: i128 = 15;
        // C s_339_22: const #1s : i
        let s_339_22: i128 = 1;
        // D s_339_23: read-var u#26101:u32
        let s_339_23: u32 = fn_state.u_26101;
        // D s_339_24: cast zx s_339_23 -> bv
        let s_339_24: Bits = Bits::new(s_339_23 as u128, 32u16);
        // D s_339_25: bit-extract s_339_24 s_339_21 s_339_22
        let s_339_25: Bits = (Bits::new(
            ((s_339_24) >> (s_339_21)).value(),
            u16::try_from(s_339_22).unwrap(),
        ));
        // D s_339_26: cast reint s_339_25 -> u8
        let s_339_26: bool = ((s_339_25.value()) != 0);
        // C s_339_27: const #16s : i
        let s_339_27: i128 = 16;
        // C s_339_28: const #5s : i
        let s_339_28: i128 = 5;
        // D s_339_29: read-var u#26101:u32
        let s_339_29: u32 = fn_state.u_26101;
        // D s_339_30: cast zx s_339_29 -> bv
        let s_339_30: Bits = Bits::new(s_339_29 as u128, 32u16);
        // D s_339_31: bit-extract s_339_30 s_339_27 s_339_28
        let s_339_31: Bits = (Bits::new(
            ((s_339_30) >> (s_339_27)).value(),
            u16::try_from(s_339_28).unwrap(),
        ));
        // D s_339_32: cast reint s_339_31 -> u8
        let s_339_32: u8 = (s_339_31.value() as u8);
        // C s_339_33: const #23s : i
        let s_339_33: i128 = 23;
        // C s_339_34: const #1s : i
        let s_339_34: i128 = 1;
        // D s_339_35: read-var u#26101:u32
        let s_339_35: u32 = fn_state.u_26101;
        // D s_339_36: cast zx s_339_35 -> bv
        let s_339_36: Bits = Bits::new(s_339_35 as u128, 32u16);
        // D s_339_37: bit-extract s_339_36 s_339_33 s_339_34
        let s_339_37: Bits = (Bits::new(
            ((s_339_36) >> (s_339_33)).value(),
            u16::try_from(s_339_34).unwrap(),
        ));
        // D s_339_38: cast reint s_339_37 -> u8
        let s_339_38: bool = ((s_339_37.value()) != 0);
        // D s_339_39: call decode_smaddl_aarch64_instrs_integer_arithmetic_mul_widening_32_64(s_339_8, s_339_14, s_339_20, s_339_26, s_339_32, s_339_38)
        let s_339_39: () = decode_smaddl_aarch64_instrs_integer_arithmetic_mul_widening_32_64(
            state,
            tracer,
            s_339_8,
            s_339_14,
            s_339_20,
            s_339_26,
            s_339_32,
            s_339_38,
        );
        // N s_339_40: return
        return;
    }
    fn block_340<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_340_0: read-var merge#var.1:struct
        let s_340_0: u32 = fn_state.merge_var._1;
        // D s_340_1: write-var u#26108 <= s_340_0
        fn_state.u_26108 = s_340_0;
        // C s_340_2: const #21s : i
        let s_340_2: i128 = 21;
        // D s_340_3: read-var u#26108:u32
        let s_340_3: u32 = fn_state.u_26108;
        // D s_340_4: cast zx s_340_3 -> bv
        let s_340_4: Bits = Bits::new(s_340_3 as u128, 32u16);
        // C s_340_5: const #1s : i64
        let s_340_5: i64 = 1;
        // C s_340_6: cast zx s_340_5 -> i
        let s_340_6: i128 = (i128::try_from(s_340_5).unwrap());
        // C s_340_7: const #10s : i
        let s_340_7: i128 = 10;
        // C s_340_8: add s_340_7 s_340_6
        let s_340_8: i128 = (s_340_7 + s_340_6);
        // D s_340_9: bit-extract s_340_4 s_340_2 s_340_8
        let s_340_9: Bits = (Bits::new(
            ((s_340_4) >> (s_340_2)).value(),
            u16::try_from(s_340_8).unwrap(),
        ));
        // D s_340_10: cast reint s_340_9 -> u11
        let s_340_10: u16 = (s_340_9.value() as u16);
        // D s_340_11: cast zx s_340_10 -> bv
        let s_340_11: Bits = Bits::new(s_340_10 as u128, 11u16);
        // C s_340_12: const #1241u : u11
        let s_340_12: u16 = 1241;
        // C s_340_13: cast zx s_340_12 -> bv
        let s_340_13: Bits = Bits::new(s_340_12 as u128, 11u16);
        // D s_340_14: cmp-eq s_340_11 s_340_13
        let s_340_14: bool = ((s_340_11) == (s_340_13));
        // N s_340_15: branch s_340_14 b464 b341
        if s_340_14 {
            return block_464(state, tracer, fn_state);
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
        // D s_341_1: write-var gs#383387 <= s_341_0
        fn_state.gs_383387 = s_341_0;
        // N s_341_2: jump b342
        return block_342(state, tracer, fn_state);
    }
    fn block_342<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_342_0: read-var gs#383387:u8
        let s_342_0: bool = fn_state.gs_383387;
        // N s_342_1: branch s_342_0 b463 b343
        if s_342_0 {
            return block_463(state, tracer, fn_state);
        } else {
            return block_343(state, tracer, fn_state);
        };
    }
    fn block_343<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_343_0: const #0u : u8
        let s_343_0: bool = false;
        // D s_343_1: write-var gs#383389 <= s_343_0
        fn_state.gs_383389 = s_343_0;
        // N s_343_2: jump b344
        return block_344(state, tracer, fn_state);
    }
    fn block_344<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_344_0: read-var gs#383389:u8
        let s_344_0: bool = fn_state.gs_383389;
        // D s_344_1: not s_344_0
        let s_344_1: bool = !s_344_0;
        // N s_344_2: branch s_344_1 b346 b345
        if s_344_1 {
            return block_346(state, tracer, fn_state);
        } else {
            return block_345(state, tracer, fn_state);
        };
    }
    fn block_345<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_345_0: const #829s : i
        let s_345_0: i128 = 829;
        // C s_345_1: const #14696u : u32
        let s_345_1: u32 = 14696;
        // N s_345_2: write-reg s_345_1 <= s_345_0
        let s_345_2: () = {
            state.write_register::<i128>(s_345_1 as isize, s_345_0);
            tracer.write_register(s_345_1 as isize, s_345_0);
        };
        // C s_345_3: const #0s : i
        let s_345_3: i128 = 0;
        // C s_345_4: const #5s : i
        let s_345_4: i128 = 5;
        // D s_345_5: read-var u#26108:u32
        let s_345_5: u32 = fn_state.u_26108;
        // D s_345_6: cast zx s_345_5 -> bv
        let s_345_6: Bits = Bits::new(s_345_5 as u128, 32u16);
        // D s_345_7: bit-extract s_345_6 s_345_3 s_345_4
        let s_345_7: Bits = (Bits::new(
            ((s_345_6) >> (s_345_3)).value(),
            u16::try_from(s_345_4).unwrap(),
        ));
        // D s_345_8: cast reint s_345_7 -> u8
        let s_345_8: u8 = (s_345_7.value() as u8);
        // C s_345_9: const #5s : i
        let s_345_9: i128 = 5;
        // C s_345_10: const #5s : i
        let s_345_10: i128 = 5;
        // D s_345_11: read-var u#26108:u32
        let s_345_11: u32 = fn_state.u_26108;
        // D s_345_12: cast zx s_345_11 -> bv
        let s_345_12: Bits = Bits::new(s_345_11 as u128, 32u16);
        // D s_345_13: bit-extract s_345_12 s_345_9 s_345_10
        let s_345_13: Bits = (Bits::new(
            ((s_345_12) >> (s_345_9)).value(),
            u16::try_from(s_345_10).unwrap(),
        ));
        // D s_345_14: cast reint s_345_13 -> u8
        let s_345_14: u8 = (s_345_13.value() as u8);
        // C s_345_15: const #10s : i
        let s_345_15: i128 = 10;
        // C s_345_16: const #5s : i
        let s_345_16: i128 = 5;
        // D s_345_17: read-var u#26108:u32
        let s_345_17: u32 = fn_state.u_26108;
        // D s_345_18: cast zx s_345_17 -> bv
        let s_345_18: Bits = Bits::new(s_345_17 as u128, 32u16);
        // D s_345_19: bit-extract s_345_18 s_345_15 s_345_16
        let s_345_19: Bits = (Bits::new(
            ((s_345_18) >> (s_345_15)).value(),
            u16::try_from(s_345_16).unwrap(),
        ));
        // D s_345_20: cast reint s_345_19 -> u8
        let s_345_20: u8 = (s_345_19.value() as u8);
        // C s_345_21: const #15s : i
        let s_345_21: i128 = 15;
        // C s_345_22: const #1s : i
        let s_345_22: i128 = 1;
        // D s_345_23: read-var u#26108:u32
        let s_345_23: u32 = fn_state.u_26108;
        // D s_345_24: cast zx s_345_23 -> bv
        let s_345_24: Bits = Bits::new(s_345_23 as u128, 32u16);
        // D s_345_25: bit-extract s_345_24 s_345_21 s_345_22
        let s_345_25: Bits = (Bits::new(
            ((s_345_24) >> (s_345_21)).value(),
            u16::try_from(s_345_22).unwrap(),
        ));
        // D s_345_26: cast reint s_345_25 -> u8
        let s_345_26: bool = ((s_345_25.value()) != 0);
        // C s_345_27: const #16s : i
        let s_345_27: i128 = 16;
        // C s_345_28: const #5s : i
        let s_345_28: i128 = 5;
        // D s_345_29: read-var u#26108:u32
        let s_345_29: u32 = fn_state.u_26108;
        // D s_345_30: cast zx s_345_29 -> bv
        let s_345_30: Bits = Bits::new(s_345_29 as u128, 32u16);
        // D s_345_31: bit-extract s_345_30 s_345_27 s_345_28
        let s_345_31: Bits = (Bits::new(
            ((s_345_30) >> (s_345_27)).value(),
            u16::try_from(s_345_28).unwrap(),
        ));
        // D s_345_32: cast reint s_345_31 -> u8
        let s_345_32: u8 = (s_345_31.value() as u8);
        // C s_345_33: const #23s : i
        let s_345_33: i128 = 23;
        // C s_345_34: const #1s : i
        let s_345_34: i128 = 1;
        // D s_345_35: read-var u#26108:u32
        let s_345_35: u32 = fn_state.u_26108;
        // D s_345_36: cast zx s_345_35 -> bv
        let s_345_36: Bits = Bits::new(s_345_35 as u128, 32u16);
        // D s_345_37: bit-extract s_345_36 s_345_33 s_345_34
        let s_345_37: Bits = (Bits::new(
            ((s_345_36) >> (s_345_33)).value(),
            u16::try_from(s_345_34).unwrap(),
        ));
        // D s_345_38: cast reint s_345_37 -> u8
        let s_345_38: bool = ((s_345_37.value()) != 0);
        // D s_345_39: call decode_smsubl_aarch64_instrs_integer_arithmetic_mul_widening_32_64(s_345_8, s_345_14, s_345_20, s_345_26, s_345_32, s_345_38)
        let s_345_39: () = decode_smsubl_aarch64_instrs_integer_arithmetic_mul_widening_32_64(
            state,
            tracer,
            s_345_8,
            s_345_14,
            s_345_20,
            s_345_26,
            s_345_32,
            s_345_38,
        );
        // N s_345_40: return
        return;
    }
    fn block_346<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_346_0: read-var merge#var.1:struct
        let s_346_0: u32 = fn_state.merge_var._1;
        // D s_346_1: write-var u#26116 <= s_346_0
        fn_state.u_26116 = s_346_0;
        // C s_346_2: const #21s : i
        let s_346_2: i128 = 21;
        // D s_346_3: read-var u#26116:u32
        let s_346_3: u32 = fn_state.u_26116;
        // D s_346_4: cast zx s_346_3 -> bv
        let s_346_4: Bits = Bits::new(s_346_3 as u128, 32u16);
        // C s_346_5: const #1s : i64
        let s_346_5: i64 = 1;
        // C s_346_6: cast zx s_346_5 -> i
        let s_346_6: i128 = (i128::try_from(s_346_5).unwrap());
        // C s_346_7: const #10s : i
        let s_346_7: i128 = 10;
        // C s_346_8: add s_346_7 s_346_6
        let s_346_8: i128 = (s_346_7 + s_346_6);
        // D s_346_9: bit-extract s_346_4 s_346_2 s_346_8
        let s_346_9: Bits = (Bits::new(
            ((s_346_4) >> (s_346_2)).value(),
            u16::try_from(s_346_8).unwrap(),
        ));
        // D s_346_10: cast reint s_346_9 -> u11
        let s_346_10: u16 = (s_346_9.value() as u16);
        // D s_346_11: cast zx s_346_10 -> bv
        let s_346_11: Bits = Bits::new(s_346_10 as u128, 11u16);
        // C s_346_12: const #1245u : u11
        let s_346_12: u16 = 1245;
        // C s_346_13: cast zx s_346_12 -> bv
        let s_346_13: Bits = Bits::new(s_346_12 as u128, 11u16);
        // D s_346_14: cmp-eq s_346_11 s_346_13
        let s_346_14: bool = ((s_346_11) == (s_346_13));
        // N s_346_15: branch s_346_14 b462 b347
        if s_346_14 {
            return block_462(state, tracer, fn_state);
        } else {
            return block_347(state, tracer, fn_state);
        };
    }
    fn block_347<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_347_0: const #0u : u8
        let s_347_0: bool = false;
        // D s_347_1: write-var gs#383409 <= s_347_0
        fn_state.gs_383409 = s_347_0;
        // N s_347_2: jump b348
        return block_348(state, tracer, fn_state);
    }
    fn block_348<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_348_0: read-var gs#383409:u8
        let s_348_0: bool = fn_state.gs_383409;
        // N s_348_1: branch s_348_0 b461 b349
        if s_348_0 {
            return block_461(state, tracer, fn_state);
        } else {
            return block_349(state, tracer, fn_state);
        };
    }
    fn block_349<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_349_0: const #0u : u8
        let s_349_0: bool = false;
        // D s_349_1: write-var gs#383411 <= s_349_0
        fn_state.gs_383411 = s_349_0;
        // N s_349_2: jump b350
        return block_350(state, tracer, fn_state);
    }
    fn block_350<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_350_0: read-var gs#383411:u8
        let s_350_0: bool = fn_state.gs_383411;
        // D s_350_1: not s_350_0
        let s_350_1: bool = !s_350_0;
        // N s_350_2: branch s_350_1 b352 b351
        if s_350_1 {
            return block_352(state, tracer, fn_state);
        } else {
            return block_351(state, tracer, fn_state);
        };
    }
    fn block_351<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_351_0: const #830s : i
        let s_351_0: i128 = 830;
        // C s_351_1: const #14696u : u32
        let s_351_1: u32 = 14696;
        // N s_351_2: write-reg s_351_1 <= s_351_0
        let s_351_2: () = {
            state.write_register::<i128>(s_351_1 as isize, s_351_0);
            tracer.write_register(s_351_1 as isize, s_351_0);
        };
        // C s_351_3: const #0s : i
        let s_351_3: i128 = 0;
        // C s_351_4: const #5s : i
        let s_351_4: i128 = 5;
        // D s_351_5: read-var u#26116:u32
        let s_351_5: u32 = fn_state.u_26116;
        // D s_351_6: cast zx s_351_5 -> bv
        let s_351_6: Bits = Bits::new(s_351_5 as u128, 32u16);
        // D s_351_7: bit-extract s_351_6 s_351_3 s_351_4
        let s_351_7: Bits = (Bits::new(
            ((s_351_6) >> (s_351_3)).value(),
            u16::try_from(s_351_4).unwrap(),
        ));
        // D s_351_8: cast reint s_351_7 -> u8
        let s_351_8: u8 = (s_351_7.value() as u8);
        // C s_351_9: const #5s : i
        let s_351_9: i128 = 5;
        // C s_351_10: const #5s : i
        let s_351_10: i128 = 5;
        // D s_351_11: read-var u#26116:u32
        let s_351_11: u32 = fn_state.u_26116;
        // D s_351_12: cast zx s_351_11 -> bv
        let s_351_12: Bits = Bits::new(s_351_11 as u128, 32u16);
        // D s_351_13: bit-extract s_351_12 s_351_9 s_351_10
        let s_351_13: Bits = (Bits::new(
            ((s_351_12) >> (s_351_9)).value(),
            u16::try_from(s_351_10).unwrap(),
        ));
        // D s_351_14: cast reint s_351_13 -> u8
        let s_351_14: u8 = (s_351_13.value() as u8);
        // C s_351_15: const #10s : i
        let s_351_15: i128 = 10;
        // C s_351_16: const #5s : i
        let s_351_16: i128 = 5;
        // D s_351_17: read-var u#26116:u32
        let s_351_17: u32 = fn_state.u_26116;
        // D s_351_18: cast zx s_351_17 -> bv
        let s_351_18: Bits = Bits::new(s_351_17 as u128, 32u16);
        // D s_351_19: bit-extract s_351_18 s_351_15 s_351_16
        let s_351_19: Bits = (Bits::new(
            ((s_351_18) >> (s_351_15)).value(),
            u16::try_from(s_351_16).unwrap(),
        ));
        // D s_351_20: cast reint s_351_19 -> u8
        let s_351_20: u8 = (s_351_19.value() as u8);
        // C s_351_21: const #15s : i
        let s_351_21: i128 = 15;
        // C s_351_22: const #1s : i
        let s_351_22: i128 = 1;
        // D s_351_23: read-var u#26116:u32
        let s_351_23: u32 = fn_state.u_26116;
        // D s_351_24: cast zx s_351_23 -> bv
        let s_351_24: Bits = Bits::new(s_351_23 as u128, 32u16);
        // D s_351_25: bit-extract s_351_24 s_351_21 s_351_22
        let s_351_25: Bits = (Bits::new(
            ((s_351_24) >> (s_351_21)).value(),
            u16::try_from(s_351_22).unwrap(),
        ));
        // D s_351_26: cast reint s_351_25 -> u8
        let s_351_26: bool = ((s_351_25.value()) != 0);
        // C s_351_27: const #16s : i
        let s_351_27: i128 = 16;
        // C s_351_28: const #5s : i
        let s_351_28: i128 = 5;
        // D s_351_29: read-var u#26116:u32
        let s_351_29: u32 = fn_state.u_26116;
        // D s_351_30: cast zx s_351_29 -> bv
        let s_351_30: Bits = Bits::new(s_351_29 as u128, 32u16);
        // D s_351_31: bit-extract s_351_30 s_351_27 s_351_28
        let s_351_31: Bits = (Bits::new(
            ((s_351_30) >> (s_351_27)).value(),
            u16::try_from(s_351_28).unwrap(),
        ));
        // D s_351_32: cast reint s_351_31 -> u8
        let s_351_32: u8 = (s_351_31.value() as u8);
        // C s_351_33: const #23s : i
        let s_351_33: i128 = 23;
        // C s_351_34: const #1s : i
        let s_351_34: i128 = 1;
        // D s_351_35: read-var u#26116:u32
        let s_351_35: u32 = fn_state.u_26116;
        // D s_351_36: cast zx s_351_35 -> bv
        let s_351_36: Bits = Bits::new(s_351_35 as u128, 32u16);
        // D s_351_37: bit-extract s_351_36 s_351_33 s_351_34
        let s_351_37: Bits = (Bits::new(
            ((s_351_36) >> (s_351_33)).value(),
            u16::try_from(s_351_34).unwrap(),
        ));
        // D s_351_38: cast reint s_351_37 -> u8
        let s_351_38: bool = ((s_351_37.value()) != 0);
        // D s_351_39: call decode_umaddl_aarch64_instrs_integer_arithmetic_mul_widening_32_64(s_351_8, s_351_14, s_351_20, s_351_26, s_351_32, s_351_38)
        let s_351_39: () = decode_umaddl_aarch64_instrs_integer_arithmetic_mul_widening_32_64(
            state,
            tracer,
            s_351_8,
            s_351_14,
            s_351_20,
            s_351_26,
            s_351_32,
            s_351_38,
        );
        // N s_351_40: return
        return;
    }
    fn block_352<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_352_0: read-var merge#var.1:struct
        let s_352_0: u32 = fn_state.merge_var._1;
        // D s_352_1: write-var u#26124 <= s_352_0
        fn_state.u_26124 = s_352_0;
        // C s_352_2: const #21s : i
        let s_352_2: i128 = 21;
        // D s_352_3: read-var u#26124:u32
        let s_352_3: u32 = fn_state.u_26124;
        // D s_352_4: cast zx s_352_3 -> bv
        let s_352_4: Bits = Bits::new(s_352_3 as u128, 32u16);
        // C s_352_5: const #1s : i64
        let s_352_5: i64 = 1;
        // C s_352_6: cast zx s_352_5 -> i
        let s_352_6: i128 = (i128::try_from(s_352_5).unwrap());
        // C s_352_7: const #10s : i
        let s_352_7: i128 = 10;
        // C s_352_8: add s_352_7 s_352_6
        let s_352_8: i128 = (s_352_7 + s_352_6);
        // D s_352_9: bit-extract s_352_4 s_352_2 s_352_8
        let s_352_9: Bits = (Bits::new(
            ((s_352_4) >> (s_352_2)).value(),
            u16::try_from(s_352_8).unwrap(),
        ));
        // D s_352_10: cast reint s_352_9 -> u11
        let s_352_10: u16 = (s_352_9.value() as u16);
        // D s_352_11: cast zx s_352_10 -> bv
        let s_352_11: Bits = Bits::new(s_352_10 as u128, 11u16);
        // C s_352_12: const #1245u : u11
        let s_352_12: u16 = 1245;
        // C s_352_13: cast zx s_352_12 -> bv
        let s_352_13: Bits = Bits::new(s_352_12 as u128, 11u16);
        // D s_352_14: cmp-eq s_352_11 s_352_13
        let s_352_14: bool = ((s_352_11) == (s_352_13));
        // N s_352_15: branch s_352_14 b460 b353
        if s_352_14 {
            return block_460(state, tracer, fn_state);
        } else {
            return block_353(state, tracer, fn_state);
        };
    }
    fn block_353<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_353_0: const #0u : u8
        let s_353_0: bool = false;
        // D s_353_1: write-var gs#383431 <= s_353_0
        fn_state.gs_383431 = s_353_0;
        // N s_353_2: jump b354
        return block_354(state, tracer, fn_state);
    }
    fn block_354<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_354_0: read-var gs#383431:u8
        let s_354_0: bool = fn_state.gs_383431;
        // N s_354_1: branch s_354_0 b459 b355
        if s_354_0 {
            return block_459(state, tracer, fn_state);
        } else {
            return block_355(state, tracer, fn_state);
        };
    }
    fn block_355<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_355_0: const #0u : u8
        let s_355_0: bool = false;
        // D s_355_1: write-var gs#383433 <= s_355_0
        fn_state.gs_383433 = s_355_0;
        // N s_355_2: jump b356
        return block_356(state, tracer, fn_state);
    }
    fn block_356<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_356_0: read-var gs#383433:u8
        let s_356_0: bool = fn_state.gs_383433;
        // D s_356_1: not s_356_0
        let s_356_1: bool = !s_356_0;
        // N s_356_2: branch s_356_1 b358 b357
        if s_356_1 {
            return block_358(state, tracer, fn_state);
        } else {
            return block_357(state, tracer, fn_state);
        };
    }
    fn block_357<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_357_0: const #831s : i
        let s_357_0: i128 = 831;
        // C s_357_1: const #14696u : u32
        let s_357_1: u32 = 14696;
        // N s_357_2: write-reg s_357_1 <= s_357_0
        let s_357_2: () = {
            state.write_register::<i128>(s_357_1 as isize, s_357_0);
            tracer.write_register(s_357_1 as isize, s_357_0);
        };
        // C s_357_3: const #0s : i
        let s_357_3: i128 = 0;
        // C s_357_4: const #5s : i
        let s_357_4: i128 = 5;
        // D s_357_5: read-var u#26124:u32
        let s_357_5: u32 = fn_state.u_26124;
        // D s_357_6: cast zx s_357_5 -> bv
        let s_357_6: Bits = Bits::new(s_357_5 as u128, 32u16);
        // D s_357_7: bit-extract s_357_6 s_357_3 s_357_4
        let s_357_7: Bits = (Bits::new(
            ((s_357_6) >> (s_357_3)).value(),
            u16::try_from(s_357_4).unwrap(),
        ));
        // D s_357_8: cast reint s_357_7 -> u8
        let s_357_8: u8 = (s_357_7.value() as u8);
        // C s_357_9: const #5s : i
        let s_357_9: i128 = 5;
        // C s_357_10: const #5s : i
        let s_357_10: i128 = 5;
        // D s_357_11: read-var u#26124:u32
        let s_357_11: u32 = fn_state.u_26124;
        // D s_357_12: cast zx s_357_11 -> bv
        let s_357_12: Bits = Bits::new(s_357_11 as u128, 32u16);
        // D s_357_13: bit-extract s_357_12 s_357_9 s_357_10
        let s_357_13: Bits = (Bits::new(
            ((s_357_12) >> (s_357_9)).value(),
            u16::try_from(s_357_10).unwrap(),
        ));
        // D s_357_14: cast reint s_357_13 -> u8
        let s_357_14: u8 = (s_357_13.value() as u8);
        // C s_357_15: const #10s : i
        let s_357_15: i128 = 10;
        // C s_357_16: const #5s : i
        let s_357_16: i128 = 5;
        // D s_357_17: read-var u#26124:u32
        let s_357_17: u32 = fn_state.u_26124;
        // D s_357_18: cast zx s_357_17 -> bv
        let s_357_18: Bits = Bits::new(s_357_17 as u128, 32u16);
        // D s_357_19: bit-extract s_357_18 s_357_15 s_357_16
        let s_357_19: Bits = (Bits::new(
            ((s_357_18) >> (s_357_15)).value(),
            u16::try_from(s_357_16).unwrap(),
        ));
        // D s_357_20: cast reint s_357_19 -> u8
        let s_357_20: u8 = (s_357_19.value() as u8);
        // C s_357_21: const #15s : i
        let s_357_21: i128 = 15;
        // C s_357_22: const #1s : i
        let s_357_22: i128 = 1;
        // D s_357_23: read-var u#26124:u32
        let s_357_23: u32 = fn_state.u_26124;
        // D s_357_24: cast zx s_357_23 -> bv
        let s_357_24: Bits = Bits::new(s_357_23 as u128, 32u16);
        // D s_357_25: bit-extract s_357_24 s_357_21 s_357_22
        let s_357_25: Bits = (Bits::new(
            ((s_357_24) >> (s_357_21)).value(),
            u16::try_from(s_357_22).unwrap(),
        ));
        // D s_357_26: cast reint s_357_25 -> u8
        let s_357_26: bool = ((s_357_25.value()) != 0);
        // C s_357_27: const #16s : i
        let s_357_27: i128 = 16;
        // C s_357_28: const #5s : i
        let s_357_28: i128 = 5;
        // D s_357_29: read-var u#26124:u32
        let s_357_29: u32 = fn_state.u_26124;
        // D s_357_30: cast zx s_357_29 -> bv
        let s_357_30: Bits = Bits::new(s_357_29 as u128, 32u16);
        // D s_357_31: bit-extract s_357_30 s_357_27 s_357_28
        let s_357_31: Bits = (Bits::new(
            ((s_357_30) >> (s_357_27)).value(),
            u16::try_from(s_357_28).unwrap(),
        ));
        // D s_357_32: cast reint s_357_31 -> u8
        let s_357_32: u8 = (s_357_31.value() as u8);
        // C s_357_33: const #23s : i
        let s_357_33: i128 = 23;
        // C s_357_34: const #1s : i
        let s_357_34: i128 = 1;
        // D s_357_35: read-var u#26124:u32
        let s_357_35: u32 = fn_state.u_26124;
        // D s_357_36: cast zx s_357_35 -> bv
        let s_357_36: Bits = Bits::new(s_357_35 as u128, 32u16);
        // D s_357_37: bit-extract s_357_36 s_357_33 s_357_34
        let s_357_37: Bits = (Bits::new(
            ((s_357_36) >> (s_357_33)).value(),
            u16::try_from(s_357_34).unwrap(),
        ));
        // D s_357_38: cast reint s_357_37 -> u8
        let s_357_38: bool = ((s_357_37.value()) != 0);
        // D s_357_39: call decode_umsubl_aarch64_instrs_integer_arithmetic_mul_widening_32_64(s_357_8, s_357_14, s_357_20, s_357_26, s_357_32, s_357_38)
        let s_357_39: () = decode_umsubl_aarch64_instrs_integer_arithmetic_mul_widening_32_64(
            state,
            tracer,
            s_357_8,
            s_357_14,
            s_357_20,
            s_357_26,
            s_357_32,
            s_357_38,
        );
        // N s_357_40: return
        return;
    }
    fn block_358<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_358_0: read-var merge#var.1:struct
        let s_358_0: u32 = fn_state.merge_var._1;
        // D s_358_1: write-var u#26132 <= s_358_0
        fn_state.u_26132 = s_358_0;
        // C s_358_2: const #21s : i
        let s_358_2: i128 = 21;
        // D s_358_3: read-var u#26132:u32
        let s_358_3: u32 = fn_state.u_26132;
        // D s_358_4: cast zx s_358_3 -> bv
        let s_358_4: Bits = Bits::new(s_358_3 as u128, 32u16);
        // C s_358_5: const #1s : i64
        let s_358_5: i64 = 1;
        // C s_358_6: cast zx s_358_5 -> i
        let s_358_6: i128 = (i128::try_from(s_358_5).unwrap());
        // C s_358_7: const #9s : i
        let s_358_7: i128 = 9;
        // C s_358_8: add s_358_7 s_358_6
        let s_358_8: i128 = (s_358_7 + s_358_6);
        // D s_358_9: bit-extract s_358_4 s_358_2 s_358_8
        let s_358_9: Bits = (Bits::new(
            ((s_358_4) >> (s_358_2)).value(),
            u16::try_from(s_358_8).unwrap(),
        ));
        // D s_358_10: cast reint s_358_9 -> u10
        let s_358_10: u16 = (s_358_9.value() as u16);
        // D s_358_11: cast zx s_358_10 -> bv
        let s_358_11: Bits = Bits::new(s_358_10 as u128, 10u16);
        // C s_358_12: const #214u : u10
        let s_358_12: u16 = 214;
        // C s_358_13: cast zx s_358_12 -> bv
        let s_358_13: Bits = Bits::new(s_358_12 as u128, 10u16);
        // D s_358_14: cmp-eq s_358_11 s_358_13
        let s_358_14: bool = ((s_358_11) == (s_358_13));
        // N s_358_15: branch s_358_14 b458 b359
        if s_358_14 {
            return block_458(state, tracer, fn_state);
        } else {
            return block_359(state, tracer, fn_state);
        };
    }
    fn block_359<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_359_0: const #0u : u8
        let s_359_0: bool = false;
        // D s_359_1: write-var gs#383453 <= s_359_0
        fn_state.gs_383453 = s_359_0;
        // N s_359_2: jump b360
        return block_360(state, tracer, fn_state);
    }
    fn block_360<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_360_0: read-var gs#383453:u8
        let s_360_0: bool = fn_state.gs_383453;
        // N s_360_1: branch s_360_0 b457 b361
        if s_360_0 {
            return block_457(state, tracer, fn_state);
        } else {
            return block_361(state, tracer, fn_state);
        };
    }
    fn block_361<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_361_0: const #0u : u8
        let s_361_0: bool = false;
        // D s_361_1: write-var gs#383455 <= s_361_0
        fn_state.gs_383455 = s_361_0;
        // N s_361_2: jump b362
        return block_362(state, tracer, fn_state);
    }
    fn block_362<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_362_0: read-var gs#383455:u8
        let s_362_0: bool = fn_state.gs_383455;
        // D s_362_1: not s_362_0
        let s_362_1: bool = !s_362_0;
        // N s_362_2: branch s_362_1 b364 b363
        if s_362_1 {
            return block_364(state, tracer, fn_state);
        } else {
            return block_363(state, tracer, fn_state);
        };
    }
    fn block_363<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_363_0: const #833s : i
        let s_363_0: i128 = 833;
        // C s_363_1: const #14696u : u32
        let s_363_1: u32 = 14696;
        // N s_363_2: write-reg s_363_1 <= s_363_0
        let s_363_2: () = {
            state.write_register::<i128>(s_363_1 as isize, s_363_0);
            tracer.write_register(s_363_1 as isize, s_363_0);
        };
        // C s_363_3: const #0s : i
        let s_363_3: i128 = 0;
        // C s_363_4: const #5s : i
        let s_363_4: i128 = 5;
        // D s_363_5: read-var u#26132:u32
        let s_363_5: u32 = fn_state.u_26132;
        // D s_363_6: cast zx s_363_5 -> bv
        let s_363_6: Bits = Bits::new(s_363_5 as u128, 32u16);
        // D s_363_7: bit-extract s_363_6 s_363_3 s_363_4
        let s_363_7: Bits = (Bits::new(
            ((s_363_6) >> (s_363_3)).value(),
            u16::try_from(s_363_4).unwrap(),
        ));
        // D s_363_8: cast reint s_363_7 -> u8
        let s_363_8: u8 = (s_363_7.value() as u8);
        // C s_363_9: const #5s : i
        let s_363_9: i128 = 5;
        // C s_363_10: const #5s : i
        let s_363_10: i128 = 5;
        // D s_363_11: read-var u#26132:u32
        let s_363_11: u32 = fn_state.u_26132;
        // D s_363_12: cast zx s_363_11 -> bv
        let s_363_12: Bits = Bits::new(s_363_11 as u128, 32u16);
        // D s_363_13: bit-extract s_363_12 s_363_9 s_363_10
        let s_363_13: Bits = (Bits::new(
            ((s_363_12) >> (s_363_9)).value(),
            u16::try_from(s_363_10).unwrap(),
        ));
        // D s_363_14: cast reint s_363_13 -> u8
        let s_363_14: u8 = (s_363_13.value() as u8);
        // C s_363_15: const #16s : i
        let s_363_15: i128 = 16;
        // C s_363_16: const #5s : i
        let s_363_16: i128 = 5;
        // D s_363_17: read-var u#26132:u32
        let s_363_17: u32 = fn_state.u_26132;
        // D s_363_18: cast zx s_363_17 -> bv
        let s_363_18: Bits = Bits::new(s_363_17 as u128, 32u16);
        // D s_363_19: bit-extract s_363_18 s_363_15 s_363_16
        let s_363_19: Bits = (Bits::new(
            ((s_363_18) >> (s_363_15)).value(),
            u16::try_from(s_363_16).unwrap(),
        ));
        // D s_363_20: cast reint s_363_19 -> u8
        let s_363_20: u8 = (s_363_19.value() as u8);
        // C s_363_21: const #31s : i
        let s_363_21: i128 = 31;
        // C s_363_22: const #1s : i
        let s_363_22: i128 = 1;
        // D s_363_23: read-var u#26132:u32
        let s_363_23: u32 = fn_state.u_26132;
        // D s_363_24: cast zx s_363_23 -> bv
        let s_363_24: Bits = Bits::new(s_363_23 as u128, 32u16);
        // D s_363_25: bit-extract s_363_24 s_363_21 s_363_22
        let s_363_25: Bits = (Bits::new(
            ((s_363_24) >> (s_363_21)).value(),
            u16::try_from(s_363_22).unwrap(),
        ));
        // D s_363_26: cast reint s_363_25 -> u8
        let s_363_26: bool = ((s_363_25.value()) != 0);
        // D s_363_27: call decode_smax_reg_aarch64_instrs_integer_arithmetic_max_min_smax_reg(s_363_8, s_363_14, s_363_20, s_363_26)
        let s_363_27: () = decode_smax_reg_aarch64_instrs_integer_arithmetic_max_min_smax_reg(
            state,
            tracer,
            s_363_8,
            s_363_14,
            s_363_20,
            s_363_26,
        );
        // N s_363_28: return
        return;
    }
    fn block_364<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_364_0: read-var merge#var.1:struct
        let s_364_0: u32 = fn_state.merge_var._1;
        // D s_364_1: write-var u#26138 <= s_364_0
        fn_state.u_26138 = s_364_0;
        // C s_364_2: const #21s : i
        let s_364_2: i128 = 21;
        // D s_364_3: read-var u#26138:u32
        let s_364_3: u32 = fn_state.u_26138;
        // D s_364_4: cast zx s_364_3 -> bv
        let s_364_4: Bits = Bits::new(s_364_3 as u128, 32u16);
        // C s_364_5: const #1s : i64
        let s_364_5: i64 = 1;
        // C s_364_6: cast zx s_364_5 -> i
        let s_364_6: i128 = (i128::try_from(s_364_5).unwrap());
        // C s_364_7: const #9s : i
        let s_364_7: i128 = 9;
        // C s_364_8: add s_364_7 s_364_6
        let s_364_8: i128 = (s_364_7 + s_364_6);
        // D s_364_9: bit-extract s_364_4 s_364_2 s_364_8
        let s_364_9: Bits = (Bits::new(
            ((s_364_4) >> (s_364_2)).value(),
            u16::try_from(s_364_8).unwrap(),
        ));
        // D s_364_10: cast reint s_364_9 -> u10
        let s_364_10: u16 = (s_364_9.value() as u16);
        // D s_364_11: cast zx s_364_10 -> bv
        let s_364_11: Bits = Bits::new(s_364_10 as u128, 10u16);
        // C s_364_12: const #214u : u10
        let s_364_12: u16 = 214;
        // C s_364_13: cast zx s_364_12 -> bv
        let s_364_13: Bits = Bits::new(s_364_12 as u128, 10u16);
        // D s_364_14: cmp-eq s_364_11 s_364_13
        let s_364_14: bool = ((s_364_11) == (s_364_13));
        // N s_364_15: branch s_364_14 b456 b365
        if s_364_14 {
            return block_456(state, tracer, fn_state);
        } else {
            return block_365(state, tracer, fn_state);
        };
    }
    fn block_365<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_365_0: const #0u : u8
        let s_365_0: bool = false;
        // D s_365_1: write-var gs#383471 <= s_365_0
        fn_state.gs_383471 = s_365_0;
        // N s_365_2: jump b366
        return block_366(state, tracer, fn_state);
    }
    fn block_366<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_366_0: read-var gs#383471:u8
        let s_366_0: bool = fn_state.gs_383471;
        // N s_366_1: branch s_366_0 b455 b367
        if s_366_0 {
            return block_455(state, tracer, fn_state);
        } else {
            return block_367(state, tracer, fn_state);
        };
    }
    fn block_367<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_367_0: const #0u : u8
        let s_367_0: bool = false;
        // D s_367_1: write-var gs#383473 <= s_367_0
        fn_state.gs_383473 = s_367_0;
        // N s_367_2: jump b368
        return block_368(state, tracer, fn_state);
    }
    fn block_368<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_368_0: read-var gs#383473:u8
        let s_368_0: bool = fn_state.gs_383473;
        // D s_368_1: not s_368_0
        let s_368_1: bool = !s_368_0;
        // N s_368_2: branch s_368_1 b370 b369
        if s_368_1 {
            return block_370(state, tracer, fn_state);
        } else {
            return block_369(state, tracer, fn_state);
        };
    }
    fn block_369<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_369_0: const #848s : i
        let s_369_0: i128 = 848;
        // C s_369_1: const #14696u : u32
        let s_369_1: u32 = 14696;
        // N s_369_2: write-reg s_369_1 <= s_369_0
        let s_369_2: () = {
            state.write_register::<i128>(s_369_1 as isize, s_369_0);
            tracer.write_register(s_369_1 as isize, s_369_0);
        };
        // C s_369_3: const #0s : i
        let s_369_3: i128 = 0;
        // C s_369_4: const #5s : i
        let s_369_4: i128 = 5;
        // D s_369_5: read-var u#26138:u32
        let s_369_5: u32 = fn_state.u_26138;
        // D s_369_6: cast zx s_369_5 -> bv
        let s_369_6: Bits = Bits::new(s_369_5 as u128, 32u16);
        // D s_369_7: bit-extract s_369_6 s_369_3 s_369_4
        let s_369_7: Bits = (Bits::new(
            ((s_369_6) >> (s_369_3)).value(),
            u16::try_from(s_369_4).unwrap(),
        ));
        // D s_369_8: cast reint s_369_7 -> u8
        let s_369_8: u8 = (s_369_7.value() as u8);
        // C s_369_9: const #5s : i
        let s_369_9: i128 = 5;
        // C s_369_10: const #5s : i
        let s_369_10: i128 = 5;
        // D s_369_11: read-var u#26138:u32
        let s_369_11: u32 = fn_state.u_26138;
        // D s_369_12: cast zx s_369_11 -> bv
        let s_369_12: Bits = Bits::new(s_369_11 as u128, 32u16);
        // D s_369_13: bit-extract s_369_12 s_369_9 s_369_10
        let s_369_13: Bits = (Bits::new(
            ((s_369_12) >> (s_369_9)).value(),
            u16::try_from(s_369_10).unwrap(),
        ));
        // D s_369_14: cast reint s_369_13 -> u8
        let s_369_14: u8 = (s_369_13.value() as u8);
        // C s_369_15: const #16s : i
        let s_369_15: i128 = 16;
        // C s_369_16: const #5s : i
        let s_369_16: i128 = 5;
        // D s_369_17: read-var u#26138:u32
        let s_369_17: u32 = fn_state.u_26138;
        // D s_369_18: cast zx s_369_17 -> bv
        let s_369_18: Bits = Bits::new(s_369_17 as u128, 32u16);
        // D s_369_19: bit-extract s_369_18 s_369_15 s_369_16
        let s_369_19: Bits = (Bits::new(
            ((s_369_18) >> (s_369_15)).value(),
            u16::try_from(s_369_16).unwrap(),
        ));
        // D s_369_20: cast reint s_369_19 -> u8
        let s_369_20: u8 = (s_369_19.value() as u8);
        // C s_369_21: const #31s : i
        let s_369_21: i128 = 31;
        // C s_369_22: const #1s : i
        let s_369_22: i128 = 1;
        // D s_369_23: read-var u#26138:u32
        let s_369_23: u32 = fn_state.u_26138;
        // D s_369_24: cast zx s_369_23 -> bv
        let s_369_24: Bits = Bits::new(s_369_23 as u128, 32u16);
        // D s_369_25: bit-extract s_369_24 s_369_21 s_369_22
        let s_369_25: Bits = (Bits::new(
            ((s_369_24) >> (s_369_21)).value(),
            u16::try_from(s_369_22).unwrap(),
        ));
        // D s_369_26: cast reint s_369_25 -> u8
        let s_369_26: bool = ((s_369_25.value()) != 0);
        // D s_369_27: call decode_smin_reg_aarch64_instrs_integer_arithmetic_max_min_smin_reg(s_369_8, s_369_14, s_369_20, s_369_26)
        let s_369_27: () = decode_smin_reg_aarch64_instrs_integer_arithmetic_max_min_smin_reg(
            state,
            tracer,
            s_369_8,
            s_369_14,
            s_369_20,
            s_369_26,
        );
        // N s_369_28: return
        return;
    }
    fn block_370<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_370_0: read-var merge#var.1:struct
        let s_370_0: u32 = fn_state.merge_var._1;
        // D s_370_1: write-var u#26144 <= s_370_0
        fn_state.u_26144 = s_370_0;
        // C s_370_2: const #21s : i
        let s_370_2: i128 = 21;
        // D s_370_3: read-var u#26144:u32
        let s_370_3: u32 = fn_state.u_26144;
        // D s_370_4: cast zx s_370_3 -> bv
        let s_370_4: Bits = Bits::new(s_370_3 as u128, 32u16);
        // C s_370_5: const #1s : i64
        let s_370_5: i64 = 1;
        // C s_370_6: cast zx s_370_5 -> i
        let s_370_6: i128 = (i128::try_from(s_370_5).unwrap());
        // C s_370_7: const #10s : i
        let s_370_7: i128 = 10;
        // C s_370_8: add s_370_7 s_370_6
        let s_370_8: i128 = (s_370_7 + s_370_6);
        // D s_370_9: bit-extract s_370_4 s_370_2 s_370_8
        let s_370_9: Bits = (Bits::new(
            ((s_370_4) >> (s_370_2)).value(),
            u16::try_from(s_370_8).unwrap(),
        ));
        // D s_370_10: cast reint s_370_9 -> u11
        let s_370_10: u16 = (s_370_9.value() as u16);
        // D s_370_11: cast zx s_370_10 -> bv
        let s_370_11: Bits = Bits::new(s_370_10 as u128, 11u16);
        // C s_370_12: const #1242u : u11
        let s_370_12: u16 = 1242;
        // C s_370_13: cast zx s_370_12 -> bv
        let s_370_13: Bits = Bits::new(s_370_12 as u128, 11u16);
        // D s_370_14: cmp-eq s_370_11 s_370_13
        let s_370_14: bool = ((s_370_11) == (s_370_13));
        // N s_370_15: branch s_370_14 b454 b371
        if s_370_14 {
            return block_454(state, tracer, fn_state);
        } else {
            return block_371(state, tracer, fn_state);
        };
    }
    fn block_371<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_371_0: const #0u : u8
        let s_371_0: bool = false;
        // D s_371_1: write-var gs#383489 <= s_371_0
        fn_state.gs_383489 = s_371_0;
        // N s_371_2: jump b372
        return block_372(state, tracer, fn_state);
    }
    fn block_372<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_372_0: read-var gs#383489:u8
        let s_372_0: bool = fn_state.gs_383489;
        // N s_372_1: branch s_372_0 b453 b373
        if s_372_0 {
            return block_453(state, tracer, fn_state);
        } else {
            return block_373(state, tracer, fn_state);
        };
    }
    fn block_373<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_373_0: const #0u : u8
        let s_373_0: bool = false;
        // D s_373_1: write-var gs#383491 <= s_373_0
        fn_state.gs_383491 = s_373_0;
        // N s_373_2: jump b374
        return block_374(state, tracer, fn_state);
    }
    fn block_374<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_374_0: read-var gs#383491:u8
        let s_374_0: bool = fn_state.gs_383491;
        // D s_374_1: not s_374_0
        let s_374_1: bool = !s_374_0;
        // N s_374_2: branch s_374_1 b390 b375
        if s_374_1 {
            return block_390(state, tracer, fn_state);
        } else {
            return block_375(state, tracer, fn_state);
        };
    }
    fn block_375<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_375_0: const #861s : i
        let s_375_0: i128 = 861;
        // C s_375_1: const #14696u : u32
        let s_375_1: u32 = 14696;
        // N s_375_2: write-reg s_375_1 <= s_375_0
        let s_375_2: () = {
            state.write_register::<i128>(s_375_1 as isize, s_375_0);
            tracer.write_register(s_375_1 as isize, s_375_0);
        };
        // C s_375_3: const #0s : i
        let s_375_3: i128 = 0;
        // C s_375_4: const #5s : i
        let s_375_4: i128 = 5;
        // D s_375_5: read-var u#26144:u32
        let s_375_5: u32 = fn_state.u_26144;
        // D s_375_6: cast zx s_375_5 -> bv
        let s_375_6: Bits = Bits::new(s_375_5 as u128, 32u16);
        // D s_375_7: bit-extract s_375_6 s_375_3 s_375_4
        let s_375_7: Bits = (Bits::new(
            ((s_375_6) >> (s_375_3)).value(),
            u16::try_from(s_375_4).unwrap(),
        ));
        // D s_375_8: cast reint s_375_7 -> u8
        let s_375_8: u8 = (s_375_7.value() as u8);
        // D s_375_9: write-var u#26145 <= s_375_8
        fn_state.u_26145 = s_375_8;
        // C s_375_10: const #5s : i
        let s_375_10: i128 = 5;
        // C s_375_11: const #5s : i
        let s_375_11: i128 = 5;
        // D s_375_12: read-var u#26144:u32
        let s_375_12: u32 = fn_state.u_26144;
        // D s_375_13: cast zx s_375_12 -> bv
        let s_375_13: Bits = Bits::new(s_375_12 as u128, 32u16);
        // D s_375_14: bit-extract s_375_13 s_375_10 s_375_11
        let s_375_14: Bits = (Bits::new(
            ((s_375_13) >> (s_375_10)).value(),
            u16::try_from(s_375_11).unwrap(),
        ));
        // D s_375_15: cast reint s_375_14 -> u8
        let s_375_15: u8 = (s_375_14.value() as u8);
        // D s_375_16: write-var u#26146 <= s_375_15
        fn_state.u_26146 = s_375_15;
        // C s_375_17: const #10s : i
        let s_375_17: i128 = 10;
        // C s_375_18: const #5s : i
        let s_375_18: i128 = 5;
        // D s_375_19: read-var u#26144:u32
        let s_375_19: u32 = fn_state.u_26144;
        // D s_375_20: cast zx s_375_19 -> bv
        let s_375_20: Bits = Bits::new(s_375_19 as u128, 32u16);
        // D s_375_21: bit-extract s_375_20 s_375_17 s_375_18
        let s_375_21: Bits = (Bits::new(
            ((s_375_20) >> (s_375_17)).value(),
            u16::try_from(s_375_18).unwrap(),
        ));
        // D s_375_22: cast reint s_375_21 -> u8
        let s_375_22: u8 = (s_375_21.value() as u8);
        // D s_375_23: write-var u#26147 <= s_375_22
        fn_state.u_26147 = s_375_22;
        // C s_375_24: const #16s : i
        let s_375_24: i128 = 16;
        // C s_375_25: const #5s : i
        let s_375_25: i128 = 5;
        // D s_375_26: read-var u#26144:u32
        let s_375_26: u32 = fn_state.u_26144;
        // D s_375_27: cast zx s_375_26 -> bv
        let s_375_27: Bits = Bits::new(s_375_26 as u128, 32u16);
        // D s_375_28: bit-extract s_375_27 s_375_24 s_375_25
        let s_375_28: Bits = (Bits::new(
            ((s_375_27) >> (s_375_24)).value(),
            u16::try_from(s_375_25).unwrap(),
        ));
        // D s_375_29: cast reint s_375_28 -> u8
        let s_375_29: u8 = (s_375_28.value() as u8);
        // D s_375_30: write-var u#26148 <= s_375_29
        fn_state.u_26148 = s_375_29;
        // C s_375_31: const #23s : i
        let s_375_31: i128 = 23;
        // C s_375_32: const #1s : i
        let s_375_32: i128 = 1;
        // D s_375_33: read-var u#26144:u32
        let s_375_33: u32 = fn_state.u_26144;
        // D s_375_34: cast zx s_375_33 -> bv
        let s_375_34: Bits = Bits::new(s_375_33 as u128, 32u16);
        // D s_375_35: bit-extract s_375_34 s_375_31 s_375_32
        let s_375_35: Bits = (Bits::new(
            ((s_375_34) >> (s_375_31)).value(),
            u16::try_from(s_375_32).unwrap(),
        ));
        // D s_375_36: cast reint s_375_35 -> u8
        let s_375_36: bool = ((s_375_35.value()) != 0);
        // D s_375_37: write-var u#26149 <= s_375_36
        fn_state.u_26149 = s_375_36;
        // C s_375_38: const #10s : i
        let s_375_38: i128 = 10;
        // D s_375_39: read-var u#26144:u32
        let s_375_39: u32 = fn_state.u_26144;
        // D s_375_40: cast zx s_375_39 -> bv
        let s_375_40: Bits = Bits::new(s_375_39 as u128, 32u16);
        // C s_375_41: const #1u : u64
        let s_375_41: u64 = 1;
        // D s_375_42: bit-extract s_375_40 s_375_38 s_375_41
        let s_375_42: Bits = (Bits::new(
            ((s_375_40) >> (s_375_38)).value(),
            u16::try_from(s_375_41).unwrap(),
        ));
        // D s_375_43: cast reint s_375_42 -> u8
        let s_375_43: bool = ((s_375_42.value()) != 0);
        // C s_375_44: const #0s : i
        let s_375_44: i128 = 0;
        // C s_375_45: const #0u : u64
        let s_375_45: u64 = 0;
        // D s_375_46: cast zx s_375_43 -> u64
        let s_375_46: u64 = (s_375_43 as u64);
        // C s_375_47: const #1u : u64
        let s_375_47: u64 = 1;
        // D s_375_48: and s_375_46 s_375_47
        let s_375_48: u64 = ((s_375_46) & (s_375_47));
        // D s_375_49: cmp-eq s_375_48 s_375_47
        let s_375_49: bool = ((s_375_48) == (s_375_47));
        // D s_375_50: lsl s_375_46 s_375_44
        let s_375_50: u64 = s_375_46 << s_375_44;
        // D s_375_51: or s_375_45 s_375_50
        let s_375_51: u64 = ((s_375_45) | (s_375_50));
        // D s_375_52: cmpl s_375_50
        let s_375_52: u64 = !s_375_50;
        // D s_375_53: and s_375_45 s_375_52
        let s_375_53: u64 = ((s_375_45) & (s_375_52));
        // D s_375_54: select s_375_49 s_375_51 s_375_53
        let s_375_54: u64 = if s_375_49 { s_375_51 } else { s_375_53 };
        // D s_375_55: cast trunc s_375_54 -> u8
        let s_375_55: bool = ((s_375_54) != 0);
        // D s_375_56: cast zx s_375_55 -> bv
        let s_375_56: Bits = Bits::new(s_375_55 as u128, 1u16);
        // C s_375_57: const #1u : u8
        let s_375_57: bool = true;
        // C s_375_58: cast zx s_375_57 -> bv
        let s_375_58: Bits = Bits::new(s_375_57 as u128, 1u16);
        // D s_375_59: cmp-ne s_375_56 s_375_58
        let s_375_59: bool = ((s_375_56) != (s_375_58));
        // N s_375_60: branch s_375_59 b389 b376
        if s_375_59 {
            return block_389(state, tracer, fn_state);
        } else {
            return block_376(state, tracer, fn_state);
        };
    }
    fn block_376<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_376_0: const #11s : i
        let s_376_0: i128 = 11;
        // D s_376_1: read-var u#26144:u32
        let s_376_1: u32 = fn_state.u_26144;
        // D s_376_2: cast zx s_376_1 -> bv
        let s_376_2: Bits = Bits::new(s_376_1 as u128, 32u16);
        // C s_376_3: const #1u : u64
        let s_376_3: u64 = 1;
        // D s_376_4: bit-extract s_376_2 s_376_0 s_376_3
        let s_376_4: Bits = (Bits::new(
            ((s_376_2) >> (s_376_0)).value(),
            u16::try_from(s_376_3).unwrap(),
        ));
        // D s_376_5: cast reint s_376_4 -> u8
        let s_376_5: bool = ((s_376_4.value()) != 0);
        // C s_376_6: const #0s : i
        let s_376_6: i128 = 0;
        // C s_376_7: const #0u : u64
        let s_376_7: u64 = 0;
        // D s_376_8: cast zx s_376_5 -> u64
        let s_376_8: u64 = (s_376_5 as u64);
        // C s_376_9: const #1u : u64
        let s_376_9: u64 = 1;
        // D s_376_10: and s_376_8 s_376_9
        let s_376_10: u64 = ((s_376_8) & (s_376_9));
        // D s_376_11: cmp-eq s_376_10 s_376_9
        let s_376_11: bool = ((s_376_10) == (s_376_9));
        // D s_376_12: lsl s_376_8 s_376_6
        let s_376_12: u64 = s_376_8 << s_376_6;
        // D s_376_13: or s_376_7 s_376_12
        let s_376_13: u64 = ((s_376_7) | (s_376_12));
        // D s_376_14: cmpl s_376_12
        let s_376_14: u64 = !s_376_12;
        // D s_376_15: and s_376_7 s_376_14
        let s_376_15: u64 = ((s_376_7) & (s_376_14));
        // D s_376_16: select s_376_11 s_376_13 s_376_15
        let s_376_16: u64 = if s_376_11 { s_376_13 } else { s_376_15 };
        // D s_376_17: cast trunc s_376_16 -> u8
        let s_376_17: bool = ((s_376_16) != 0);
        // D s_376_18: cast zx s_376_17 -> bv
        let s_376_18: Bits = Bits::new(s_376_17 as u128, 1u16);
        // C s_376_19: const #1u : u8
        let s_376_19: bool = true;
        // C s_376_20: cast zx s_376_19 -> bv
        let s_376_20: Bits = Bits::new(s_376_19 as u128, 1u16);
        // D s_376_21: cmp-ne s_376_18 s_376_20
        let s_376_21: bool = ((s_376_18) != (s_376_20));
        // D s_376_22: write-var gs#383508 <= s_376_21
        fn_state.gs_383508 = s_376_21;
        // N s_376_23: jump b377
        return block_377(state, tracer, fn_state);
    }
    fn block_377<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_377_0: read-var gs#383508:u8
        let s_377_0: bool = fn_state.gs_383508;
        // N s_377_1: branch s_377_0 b388 b378
        if s_377_0 {
            return block_388(state, tracer, fn_state);
        } else {
            return block_378(state, tracer, fn_state);
        };
    }
    fn block_378<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_378_0: const #12s : i
        let s_378_0: i128 = 12;
        // D s_378_1: read-var u#26144:u32
        let s_378_1: u32 = fn_state.u_26144;
        // D s_378_2: cast zx s_378_1 -> bv
        let s_378_2: Bits = Bits::new(s_378_1 as u128, 32u16);
        // C s_378_3: const #1u : u64
        let s_378_3: u64 = 1;
        // D s_378_4: bit-extract s_378_2 s_378_0 s_378_3
        let s_378_4: Bits = (Bits::new(
            ((s_378_2) >> (s_378_0)).value(),
            u16::try_from(s_378_3).unwrap(),
        ));
        // D s_378_5: cast reint s_378_4 -> u8
        let s_378_5: bool = ((s_378_4.value()) != 0);
        // C s_378_6: const #0s : i
        let s_378_6: i128 = 0;
        // C s_378_7: const #0u : u64
        let s_378_7: u64 = 0;
        // D s_378_8: cast zx s_378_5 -> u64
        let s_378_8: u64 = (s_378_5 as u64);
        // C s_378_9: const #1u : u64
        let s_378_9: u64 = 1;
        // D s_378_10: and s_378_8 s_378_9
        let s_378_10: u64 = ((s_378_8) & (s_378_9));
        // D s_378_11: cmp-eq s_378_10 s_378_9
        let s_378_11: bool = ((s_378_10) == (s_378_9));
        // D s_378_12: lsl s_378_8 s_378_6
        let s_378_12: u64 = s_378_8 << s_378_6;
        // D s_378_13: or s_378_7 s_378_12
        let s_378_13: u64 = ((s_378_7) | (s_378_12));
        // D s_378_14: cmpl s_378_12
        let s_378_14: u64 = !s_378_12;
        // D s_378_15: and s_378_7 s_378_14
        let s_378_15: u64 = ((s_378_7) & (s_378_14));
        // D s_378_16: select s_378_11 s_378_13 s_378_15
        let s_378_16: u64 = if s_378_11 { s_378_13 } else { s_378_15 };
        // D s_378_17: cast trunc s_378_16 -> u8
        let s_378_17: bool = ((s_378_16) != 0);
        // D s_378_18: cast zx s_378_17 -> bv
        let s_378_18: Bits = Bits::new(s_378_17 as u128, 1u16);
        // C s_378_19: const #1u : u8
        let s_378_19: bool = true;
        // C s_378_20: cast zx s_378_19 -> bv
        let s_378_20: Bits = Bits::new(s_378_19 as u128, 1u16);
        // D s_378_21: cmp-ne s_378_18 s_378_20
        let s_378_21: bool = ((s_378_18) != (s_378_20));
        // D s_378_22: write-var gs#383511 <= s_378_21
        fn_state.gs_383511 = s_378_21;
        // N s_378_23: jump b379
        return block_379(state, tracer, fn_state);
    }
    fn block_379<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_379_0: read-var gs#383511:u8
        let s_379_0: bool = fn_state.gs_383511;
        // N s_379_1: branch s_379_0 b387 b380
        if s_379_0 {
            return block_387(state, tracer, fn_state);
        } else {
            return block_380(state, tracer, fn_state);
        };
    }
    fn block_380<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_380_0: const #13s : i
        let s_380_0: i128 = 13;
        // D s_380_1: read-var u#26144:u32
        let s_380_1: u32 = fn_state.u_26144;
        // D s_380_2: cast zx s_380_1 -> bv
        let s_380_2: Bits = Bits::new(s_380_1 as u128, 32u16);
        // C s_380_3: const #1u : u64
        let s_380_3: u64 = 1;
        // D s_380_4: bit-extract s_380_2 s_380_0 s_380_3
        let s_380_4: Bits = (Bits::new(
            ((s_380_2) >> (s_380_0)).value(),
            u16::try_from(s_380_3).unwrap(),
        ));
        // D s_380_5: cast reint s_380_4 -> u8
        let s_380_5: bool = ((s_380_4.value()) != 0);
        // C s_380_6: const #0s : i
        let s_380_6: i128 = 0;
        // C s_380_7: const #0u : u64
        let s_380_7: u64 = 0;
        // D s_380_8: cast zx s_380_5 -> u64
        let s_380_8: u64 = (s_380_5 as u64);
        // C s_380_9: const #1u : u64
        let s_380_9: u64 = 1;
        // D s_380_10: and s_380_8 s_380_9
        let s_380_10: u64 = ((s_380_8) & (s_380_9));
        // D s_380_11: cmp-eq s_380_10 s_380_9
        let s_380_11: bool = ((s_380_10) == (s_380_9));
        // D s_380_12: lsl s_380_8 s_380_6
        let s_380_12: u64 = s_380_8 << s_380_6;
        // D s_380_13: or s_380_7 s_380_12
        let s_380_13: u64 = ((s_380_7) | (s_380_12));
        // D s_380_14: cmpl s_380_12
        let s_380_14: u64 = !s_380_12;
        // D s_380_15: and s_380_7 s_380_14
        let s_380_15: u64 = ((s_380_7) & (s_380_14));
        // D s_380_16: select s_380_11 s_380_13 s_380_15
        let s_380_16: u64 = if s_380_11 { s_380_13 } else { s_380_15 };
        // D s_380_17: cast trunc s_380_16 -> u8
        let s_380_17: bool = ((s_380_16) != 0);
        // D s_380_18: cast zx s_380_17 -> bv
        let s_380_18: Bits = Bits::new(s_380_17 as u128, 1u16);
        // C s_380_19: const #1u : u8
        let s_380_19: bool = true;
        // C s_380_20: cast zx s_380_19 -> bv
        let s_380_20: Bits = Bits::new(s_380_19 as u128, 1u16);
        // D s_380_21: cmp-ne s_380_18 s_380_20
        let s_380_21: bool = ((s_380_18) != (s_380_20));
        // D s_380_22: write-var gs#383514 <= s_380_21
        fn_state.gs_383514 = s_380_21;
        // N s_380_23: jump b381
        return block_381(state, tracer, fn_state);
    }
    fn block_381<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_381_0: read-var gs#383514:u8
        let s_381_0: bool = fn_state.gs_383514;
        // N s_381_1: branch s_381_0 b386 b382
        if s_381_0 {
            return block_386(state, tracer, fn_state);
        } else {
            return block_382(state, tracer, fn_state);
        };
    }
    fn block_382<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_382_0: const #14s : i
        let s_382_0: i128 = 14;
        // D s_382_1: read-var u#26144:u32
        let s_382_1: u32 = fn_state.u_26144;
        // D s_382_2: cast zx s_382_1 -> bv
        let s_382_2: Bits = Bits::new(s_382_1 as u128, 32u16);
        // C s_382_3: const #1u : u64
        let s_382_3: u64 = 1;
        // D s_382_4: bit-extract s_382_2 s_382_0 s_382_3
        let s_382_4: Bits = (Bits::new(
            ((s_382_2) >> (s_382_0)).value(),
            u16::try_from(s_382_3).unwrap(),
        ));
        // D s_382_5: cast reint s_382_4 -> u8
        let s_382_5: bool = ((s_382_4.value()) != 0);
        // C s_382_6: const #0s : i
        let s_382_6: i128 = 0;
        // C s_382_7: const #0u : u64
        let s_382_7: u64 = 0;
        // D s_382_8: cast zx s_382_5 -> u64
        let s_382_8: u64 = (s_382_5 as u64);
        // C s_382_9: const #1u : u64
        let s_382_9: u64 = 1;
        // D s_382_10: and s_382_8 s_382_9
        let s_382_10: u64 = ((s_382_8) & (s_382_9));
        // D s_382_11: cmp-eq s_382_10 s_382_9
        let s_382_11: bool = ((s_382_10) == (s_382_9));
        // D s_382_12: lsl s_382_8 s_382_6
        let s_382_12: u64 = s_382_8 << s_382_6;
        // D s_382_13: or s_382_7 s_382_12
        let s_382_13: u64 = ((s_382_7) | (s_382_12));
        // D s_382_14: cmpl s_382_12
        let s_382_14: u64 = !s_382_12;
        // D s_382_15: and s_382_7 s_382_14
        let s_382_15: u64 = ((s_382_7) & (s_382_14));
        // D s_382_16: select s_382_11 s_382_13 s_382_15
        let s_382_16: u64 = if s_382_11 { s_382_13 } else { s_382_15 };
        // D s_382_17: cast trunc s_382_16 -> u8
        let s_382_17: bool = ((s_382_16) != 0);
        // D s_382_18: cast zx s_382_17 -> bv
        let s_382_18: Bits = Bits::new(s_382_17 as u128, 1u16);
        // C s_382_19: const #1u : u8
        let s_382_19: bool = true;
        // C s_382_20: cast zx s_382_19 -> bv
        let s_382_20: Bits = Bits::new(s_382_19 as u128, 1u16);
        // D s_382_21: cmp-ne s_382_18 s_382_20
        let s_382_21: bool = ((s_382_18) != (s_382_20));
        // D s_382_22: write-var gs#383517 <= s_382_21
        fn_state.gs_383517 = s_382_21;
        // N s_382_23: jump b383
        return block_383(state, tracer, fn_state);
    }
    fn block_383<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_383_0: read-var gs#383517:u8
        let s_383_0: bool = fn_state.gs_383517;
        // N s_383_1: branch s_383_0 b385 b384
        if s_383_0 {
            return block_385(state, tracer, fn_state);
        } else {
            return block_384(state, tracer, fn_state);
        };
    }
    fn block_384<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_384_0: read-var u#26145:u8
        let s_384_0: u8 = fn_state.u_26145;
        // D s_384_1: read-var u#26146:u8
        let s_384_1: u8 = fn_state.u_26146;
        // D s_384_2: read-var u#26147:u8
        let s_384_2: u8 = fn_state.u_26147;
        // D s_384_3: read-var u#26148:u8
        let s_384_3: u8 = fn_state.u_26148;
        // D s_384_4: read-var u#26149:u8
        let s_384_4: bool = fn_state.u_26149;
        // D s_384_5: call decode_smulh_aarch64_instrs_integer_arithmetic_mul_widening_64_128hi(s_384_0, s_384_1, s_384_2, s_384_3, s_384_4)
        let s_384_5: () = decode_smulh_aarch64_instrs_integer_arithmetic_mul_widening_64_128hi(
            state,
            tracer,
            s_384_0,
            s_384_1,
            s_384_2,
            s_384_3,
            s_384_4,
        );
        // N s_384_6: return
        return;
    }
    fn block_385<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_385_0: panic
        panic!("{:?}", ());
        // N s_385_1: return
        return;
    }
    fn block_386<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_386_0: const #1u : u8
        let s_386_0: bool = true;
        // D s_386_1: write-var gs#383517 <= s_386_0
        fn_state.gs_383517 = s_386_0;
        // N s_386_2: jump b383
        return block_383(state, tracer, fn_state);
    }
    fn block_387<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_387_0: const #1u : u8
        let s_387_0: bool = true;
        // D s_387_1: write-var gs#383514 <= s_387_0
        fn_state.gs_383514 = s_387_0;
        // N s_387_2: jump b381
        return block_381(state, tracer, fn_state);
    }
    fn block_388<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_388_0: const #1u : u8
        let s_388_0: bool = true;
        // D s_388_1: write-var gs#383511 <= s_388_0
        fn_state.gs_383511 = s_388_0;
        // N s_388_2: jump b379
        return block_379(state, tracer, fn_state);
    }
    fn block_389<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_389_0: const #1u : u8
        let s_389_0: bool = true;
        // D s_389_1: write-var gs#383508 <= s_389_0
        fn_state.gs_383508 = s_389_0;
        // N s_389_2: jump b377
        return block_377(state, tracer, fn_state);
    }
    fn block_390<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_390_0: read-var merge#var.1:struct
        let s_390_0: u32 = fn_state.merge_var._1;
        // D s_390_1: write-var u#26151 <= s_390_0
        fn_state.u_26151 = s_390_0;
        // C s_390_2: const #21s : i
        let s_390_2: i128 = 21;
        // D s_390_3: read-var u#26151:u32
        let s_390_3: u32 = fn_state.u_26151;
        // D s_390_4: cast zx s_390_3 -> bv
        let s_390_4: Bits = Bits::new(s_390_3 as u128, 32u16);
        // C s_390_5: const #1s : i64
        let s_390_5: i64 = 1;
        // C s_390_6: cast zx s_390_5 -> i
        let s_390_6: i128 = (i128::try_from(s_390_5).unwrap());
        // C s_390_7: const #10s : i
        let s_390_7: i128 = 10;
        // C s_390_8: add s_390_7 s_390_6
        let s_390_8: i128 = (s_390_7 + s_390_6);
        // D s_390_9: bit-extract s_390_4 s_390_2 s_390_8
        let s_390_9: Bits = (Bits::new(
            ((s_390_4) >> (s_390_2)).value(),
            u16::try_from(s_390_8).unwrap(),
        ));
        // D s_390_10: cast reint s_390_9 -> u11
        let s_390_10: u16 = (s_390_9.value() as u16);
        // D s_390_11: cast zx s_390_10 -> bv
        let s_390_11: Bits = Bits::new(s_390_10 as u128, 11u16);
        // C s_390_12: const #1246u : u11
        let s_390_12: u16 = 1246;
        // C s_390_13: cast zx s_390_12 -> bv
        let s_390_13: Bits = Bits::new(s_390_12 as u128, 11u16);
        // D s_390_14: cmp-eq s_390_11 s_390_13
        let s_390_14: bool = ((s_390_11) == (s_390_13));
        // N s_390_15: branch s_390_14 b452 b391
        if s_390_14 {
            return block_452(state, tracer, fn_state);
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
        // D s_391_1: write-var gs#383523 <= s_391_0
        fn_state.gs_383523 = s_391_0;
        // N s_391_2: jump b392
        return block_392(state, tracer, fn_state);
    }
    fn block_392<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_392_0: read-var gs#383523:u8
        let s_392_0: bool = fn_state.gs_383523;
        // N s_392_1: branch s_392_0 b451 b393
        if s_392_0 {
            return block_451(state, tracer, fn_state);
        } else {
            return block_393(state, tracer, fn_state);
        };
    }
    fn block_393<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_393_0: const #0u : u8
        let s_393_0: bool = false;
        // D s_393_1: write-var gs#383525 <= s_393_0
        fn_state.gs_383525 = s_393_0;
        // N s_393_2: jump b394
        return block_394(state, tracer, fn_state);
    }
    fn block_394<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_394_0: read-var gs#383525:u8
        let s_394_0: bool = fn_state.gs_383525;
        // D s_394_1: not s_394_0
        let s_394_1: bool = !s_394_0;
        // N s_394_2: branch s_394_1 b410 b395
        if s_394_1 {
            return block_410(state, tracer, fn_state);
        } else {
            return block_395(state, tracer, fn_state);
        };
    }
    fn block_395<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_395_0: const #862s : i
        let s_395_0: i128 = 862;
        // C s_395_1: const #14696u : u32
        let s_395_1: u32 = 14696;
        // N s_395_2: write-reg s_395_1 <= s_395_0
        let s_395_2: () = {
            state.write_register::<i128>(s_395_1 as isize, s_395_0);
            tracer.write_register(s_395_1 as isize, s_395_0);
        };
        // C s_395_3: const #0s : i
        let s_395_3: i128 = 0;
        // C s_395_4: const #5s : i
        let s_395_4: i128 = 5;
        // D s_395_5: read-var u#26151:u32
        let s_395_5: u32 = fn_state.u_26151;
        // D s_395_6: cast zx s_395_5 -> bv
        let s_395_6: Bits = Bits::new(s_395_5 as u128, 32u16);
        // D s_395_7: bit-extract s_395_6 s_395_3 s_395_4
        let s_395_7: Bits = (Bits::new(
            ((s_395_6) >> (s_395_3)).value(),
            u16::try_from(s_395_4).unwrap(),
        ));
        // D s_395_8: cast reint s_395_7 -> u8
        let s_395_8: u8 = (s_395_7.value() as u8);
        // D s_395_9: write-var u#26152 <= s_395_8
        fn_state.u_26152 = s_395_8;
        // C s_395_10: const #5s : i
        let s_395_10: i128 = 5;
        // C s_395_11: const #5s : i
        let s_395_11: i128 = 5;
        // D s_395_12: read-var u#26151:u32
        let s_395_12: u32 = fn_state.u_26151;
        // D s_395_13: cast zx s_395_12 -> bv
        let s_395_13: Bits = Bits::new(s_395_12 as u128, 32u16);
        // D s_395_14: bit-extract s_395_13 s_395_10 s_395_11
        let s_395_14: Bits = (Bits::new(
            ((s_395_13) >> (s_395_10)).value(),
            u16::try_from(s_395_11).unwrap(),
        ));
        // D s_395_15: cast reint s_395_14 -> u8
        let s_395_15: u8 = (s_395_14.value() as u8);
        // D s_395_16: write-var u#26153 <= s_395_15
        fn_state.u_26153 = s_395_15;
        // C s_395_17: const #10s : i
        let s_395_17: i128 = 10;
        // C s_395_18: const #5s : i
        let s_395_18: i128 = 5;
        // D s_395_19: read-var u#26151:u32
        let s_395_19: u32 = fn_state.u_26151;
        // D s_395_20: cast zx s_395_19 -> bv
        let s_395_20: Bits = Bits::new(s_395_19 as u128, 32u16);
        // D s_395_21: bit-extract s_395_20 s_395_17 s_395_18
        let s_395_21: Bits = (Bits::new(
            ((s_395_20) >> (s_395_17)).value(),
            u16::try_from(s_395_18).unwrap(),
        ));
        // D s_395_22: cast reint s_395_21 -> u8
        let s_395_22: u8 = (s_395_21.value() as u8);
        // D s_395_23: write-var u#26154 <= s_395_22
        fn_state.u_26154 = s_395_22;
        // C s_395_24: const #16s : i
        let s_395_24: i128 = 16;
        // C s_395_25: const #5s : i
        let s_395_25: i128 = 5;
        // D s_395_26: read-var u#26151:u32
        let s_395_26: u32 = fn_state.u_26151;
        // D s_395_27: cast zx s_395_26 -> bv
        let s_395_27: Bits = Bits::new(s_395_26 as u128, 32u16);
        // D s_395_28: bit-extract s_395_27 s_395_24 s_395_25
        let s_395_28: Bits = (Bits::new(
            ((s_395_27) >> (s_395_24)).value(),
            u16::try_from(s_395_25).unwrap(),
        ));
        // D s_395_29: cast reint s_395_28 -> u8
        let s_395_29: u8 = (s_395_28.value() as u8);
        // D s_395_30: write-var u#26155 <= s_395_29
        fn_state.u_26155 = s_395_29;
        // C s_395_31: const #23s : i
        let s_395_31: i128 = 23;
        // C s_395_32: const #1s : i
        let s_395_32: i128 = 1;
        // D s_395_33: read-var u#26151:u32
        let s_395_33: u32 = fn_state.u_26151;
        // D s_395_34: cast zx s_395_33 -> bv
        let s_395_34: Bits = Bits::new(s_395_33 as u128, 32u16);
        // D s_395_35: bit-extract s_395_34 s_395_31 s_395_32
        let s_395_35: Bits = (Bits::new(
            ((s_395_34) >> (s_395_31)).value(),
            u16::try_from(s_395_32).unwrap(),
        ));
        // D s_395_36: cast reint s_395_35 -> u8
        let s_395_36: bool = ((s_395_35.value()) != 0);
        // D s_395_37: write-var u#26156 <= s_395_36
        fn_state.u_26156 = s_395_36;
        // C s_395_38: const #10s : i
        let s_395_38: i128 = 10;
        // D s_395_39: read-var u#26151:u32
        let s_395_39: u32 = fn_state.u_26151;
        // D s_395_40: cast zx s_395_39 -> bv
        let s_395_40: Bits = Bits::new(s_395_39 as u128, 32u16);
        // C s_395_41: const #1u : u64
        let s_395_41: u64 = 1;
        // D s_395_42: bit-extract s_395_40 s_395_38 s_395_41
        let s_395_42: Bits = (Bits::new(
            ((s_395_40) >> (s_395_38)).value(),
            u16::try_from(s_395_41).unwrap(),
        ));
        // D s_395_43: cast reint s_395_42 -> u8
        let s_395_43: bool = ((s_395_42.value()) != 0);
        // C s_395_44: const #0s : i
        let s_395_44: i128 = 0;
        // C s_395_45: const #0u : u64
        let s_395_45: u64 = 0;
        // D s_395_46: cast zx s_395_43 -> u64
        let s_395_46: u64 = (s_395_43 as u64);
        // C s_395_47: const #1u : u64
        let s_395_47: u64 = 1;
        // D s_395_48: and s_395_46 s_395_47
        let s_395_48: u64 = ((s_395_46) & (s_395_47));
        // D s_395_49: cmp-eq s_395_48 s_395_47
        let s_395_49: bool = ((s_395_48) == (s_395_47));
        // D s_395_50: lsl s_395_46 s_395_44
        let s_395_50: u64 = s_395_46 << s_395_44;
        // D s_395_51: or s_395_45 s_395_50
        let s_395_51: u64 = ((s_395_45) | (s_395_50));
        // D s_395_52: cmpl s_395_50
        let s_395_52: u64 = !s_395_50;
        // D s_395_53: and s_395_45 s_395_52
        let s_395_53: u64 = ((s_395_45) & (s_395_52));
        // D s_395_54: select s_395_49 s_395_51 s_395_53
        let s_395_54: u64 = if s_395_49 { s_395_51 } else { s_395_53 };
        // D s_395_55: cast trunc s_395_54 -> u8
        let s_395_55: bool = ((s_395_54) != 0);
        // D s_395_56: cast zx s_395_55 -> bv
        let s_395_56: Bits = Bits::new(s_395_55 as u128, 1u16);
        // C s_395_57: const #1u : u8
        let s_395_57: bool = true;
        // C s_395_58: cast zx s_395_57 -> bv
        let s_395_58: Bits = Bits::new(s_395_57 as u128, 1u16);
        // D s_395_59: cmp-ne s_395_56 s_395_58
        let s_395_59: bool = ((s_395_56) != (s_395_58));
        // N s_395_60: branch s_395_59 b409 b396
        if s_395_59 {
            return block_409(state, tracer, fn_state);
        } else {
            return block_396(state, tracer, fn_state);
        };
    }
    fn block_396<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_396_0: const #11s : i
        let s_396_0: i128 = 11;
        // D s_396_1: read-var u#26151:u32
        let s_396_1: u32 = fn_state.u_26151;
        // D s_396_2: cast zx s_396_1 -> bv
        let s_396_2: Bits = Bits::new(s_396_1 as u128, 32u16);
        // C s_396_3: const #1u : u64
        let s_396_3: u64 = 1;
        // D s_396_4: bit-extract s_396_2 s_396_0 s_396_3
        let s_396_4: Bits = (Bits::new(
            ((s_396_2) >> (s_396_0)).value(),
            u16::try_from(s_396_3).unwrap(),
        ));
        // D s_396_5: cast reint s_396_4 -> u8
        let s_396_5: bool = ((s_396_4.value()) != 0);
        // C s_396_6: const #0s : i
        let s_396_6: i128 = 0;
        // C s_396_7: const #0u : u64
        let s_396_7: u64 = 0;
        // D s_396_8: cast zx s_396_5 -> u64
        let s_396_8: u64 = (s_396_5 as u64);
        // C s_396_9: const #1u : u64
        let s_396_9: u64 = 1;
        // D s_396_10: and s_396_8 s_396_9
        let s_396_10: u64 = ((s_396_8) & (s_396_9));
        // D s_396_11: cmp-eq s_396_10 s_396_9
        let s_396_11: bool = ((s_396_10) == (s_396_9));
        // D s_396_12: lsl s_396_8 s_396_6
        let s_396_12: u64 = s_396_8 << s_396_6;
        // D s_396_13: or s_396_7 s_396_12
        let s_396_13: u64 = ((s_396_7) | (s_396_12));
        // D s_396_14: cmpl s_396_12
        let s_396_14: u64 = !s_396_12;
        // D s_396_15: and s_396_7 s_396_14
        let s_396_15: u64 = ((s_396_7) & (s_396_14));
        // D s_396_16: select s_396_11 s_396_13 s_396_15
        let s_396_16: u64 = if s_396_11 { s_396_13 } else { s_396_15 };
        // D s_396_17: cast trunc s_396_16 -> u8
        let s_396_17: bool = ((s_396_16) != 0);
        // D s_396_18: cast zx s_396_17 -> bv
        let s_396_18: Bits = Bits::new(s_396_17 as u128, 1u16);
        // C s_396_19: const #1u : u8
        let s_396_19: bool = true;
        // C s_396_20: cast zx s_396_19 -> bv
        let s_396_20: Bits = Bits::new(s_396_19 as u128, 1u16);
        // D s_396_21: cmp-ne s_396_18 s_396_20
        let s_396_21: bool = ((s_396_18) != (s_396_20));
        // D s_396_22: write-var gs#383542 <= s_396_21
        fn_state.gs_383542 = s_396_21;
        // N s_396_23: jump b397
        return block_397(state, tracer, fn_state);
    }
    fn block_397<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_397_0: read-var gs#383542:u8
        let s_397_0: bool = fn_state.gs_383542;
        // N s_397_1: branch s_397_0 b408 b398
        if s_397_0 {
            return block_408(state, tracer, fn_state);
        } else {
            return block_398(state, tracer, fn_state);
        };
    }
    fn block_398<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_398_0: const #12s : i
        let s_398_0: i128 = 12;
        // D s_398_1: read-var u#26151:u32
        let s_398_1: u32 = fn_state.u_26151;
        // D s_398_2: cast zx s_398_1 -> bv
        let s_398_2: Bits = Bits::new(s_398_1 as u128, 32u16);
        // C s_398_3: const #1u : u64
        let s_398_3: u64 = 1;
        // D s_398_4: bit-extract s_398_2 s_398_0 s_398_3
        let s_398_4: Bits = (Bits::new(
            ((s_398_2) >> (s_398_0)).value(),
            u16::try_from(s_398_3).unwrap(),
        ));
        // D s_398_5: cast reint s_398_4 -> u8
        let s_398_5: bool = ((s_398_4.value()) != 0);
        // C s_398_6: const #0s : i
        let s_398_6: i128 = 0;
        // C s_398_7: const #0u : u64
        let s_398_7: u64 = 0;
        // D s_398_8: cast zx s_398_5 -> u64
        let s_398_8: u64 = (s_398_5 as u64);
        // C s_398_9: const #1u : u64
        let s_398_9: u64 = 1;
        // D s_398_10: and s_398_8 s_398_9
        let s_398_10: u64 = ((s_398_8) & (s_398_9));
        // D s_398_11: cmp-eq s_398_10 s_398_9
        let s_398_11: bool = ((s_398_10) == (s_398_9));
        // D s_398_12: lsl s_398_8 s_398_6
        let s_398_12: u64 = s_398_8 << s_398_6;
        // D s_398_13: or s_398_7 s_398_12
        let s_398_13: u64 = ((s_398_7) | (s_398_12));
        // D s_398_14: cmpl s_398_12
        let s_398_14: u64 = !s_398_12;
        // D s_398_15: and s_398_7 s_398_14
        let s_398_15: u64 = ((s_398_7) & (s_398_14));
        // D s_398_16: select s_398_11 s_398_13 s_398_15
        let s_398_16: u64 = if s_398_11 { s_398_13 } else { s_398_15 };
        // D s_398_17: cast trunc s_398_16 -> u8
        let s_398_17: bool = ((s_398_16) != 0);
        // D s_398_18: cast zx s_398_17 -> bv
        let s_398_18: Bits = Bits::new(s_398_17 as u128, 1u16);
        // C s_398_19: const #1u : u8
        let s_398_19: bool = true;
        // C s_398_20: cast zx s_398_19 -> bv
        let s_398_20: Bits = Bits::new(s_398_19 as u128, 1u16);
        // D s_398_21: cmp-ne s_398_18 s_398_20
        let s_398_21: bool = ((s_398_18) != (s_398_20));
        // D s_398_22: write-var gs#383545 <= s_398_21
        fn_state.gs_383545 = s_398_21;
        // N s_398_23: jump b399
        return block_399(state, tracer, fn_state);
    }
    fn block_399<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_399_0: read-var gs#383545:u8
        let s_399_0: bool = fn_state.gs_383545;
        // N s_399_1: branch s_399_0 b407 b400
        if s_399_0 {
            return block_407(state, tracer, fn_state);
        } else {
            return block_400(state, tracer, fn_state);
        };
    }
    fn block_400<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_400_0: const #13s : i
        let s_400_0: i128 = 13;
        // D s_400_1: read-var u#26151:u32
        let s_400_1: u32 = fn_state.u_26151;
        // D s_400_2: cast zx s_400_1 -> bv
        let s_400_2: Bits = Bits::new(s_400_1 as u128, 32u16);
        // C s_400_3: const #1u : u64
        let s_400_3: u64 = 1;
        // D s_400_4: bit-extract s_400_2 s_400_0 s_400_3
        let s_400_4: Bits = (Bits::new(
            ((s_400_2) >> (s_400_0)).value(),
            u16::try_from(s_400_3).unwrap(),
        ));
        // D s_400_5: cast reint s_400_4 -> u8
        let s_400_5: bool = ((s_400_4.value()) != 0);
        // C s_400_6: const #0s : i
        let s_400_6: i128 = 0;
        // C s_400_7: const #0u : u64
        let s_400_7: u64 = 0;
        // D s_400_8: cast zx s_400_5 -> u64
        let s_400_8: u64 = (s_400_5 as u64);
        // C s_400_9: const #1u : u64
        let s_400_9: u64 = 1;
        // D s_400_10: and s_400_8 s_400_9
        let s_400_10: u64 = ((s_400_8) & (s_400_9));
        // D s_400_11: cmp-eq s_400_10 s_400_9
        let s_400_11: bool = ((s_400_10) == (s_400_9));
        // D s_400_12: lsl s_400_8 s_400_6
        let s_400_12: u64 = s_400_8 << s_400_6;
        // D s_400_13: or s_400_7 s_400_12
        let s_400_13: u64 = ((s_400_7) | (s_400_12));
        // D s_400_14: cmpl s_400_12
        let s_400_14: u64 = !s_400_12;
        // D s_400_15: and s_400_7 s_400_14
        let s_400_15: u64 = ((s_400_7) & (s_400_14));
        // D s_400_16: select s_400_11 s_400_13 s_400_15
        let s_400_16: u64 = if s_400_11 { s_400_13 } else { s_400_15 };
        // D s_400_17: cast trunc s_400_16 -> u8
        let s_400_17: bool = ((s_400_16) != 0);
        // D s_400_18: cast zx s_400_17 -> bv
        let s_400_18: Bits = Bits::new(s_400_17 as u128, 1u16);
        // C s_400_19: const #1u : u8
        let s_400_19: bool = true;
        // C s_400_20: cast zx s_400_19 -> bv
        let s_400_20: Bits = Bits::new(s_400_19 as u128, 1u16);
        // D s_400_21: cmp-ne s_400_18 s_400_20
        let s_400_21: bool = ((s_400_18) != (s_400_20));
        // D s_400_22: write-var gs#383548 <= s_400_21
        fn_state.gs_383548 = s_400_21;
        // N s_400_23: jump b401
        return block_401(state, tracer, fn_state);
    }
    fn block_401<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_401_0: read-var gs#383548:u8
        let s_401_0: bool = fn_state.gs_383548;
        // N s_401_1: branch s_401_0 b406 b402
        if s_401_0 {
            return block_406(state, tracer, fn_state);
        } else {
            return block_402(state, tracer, fn_state);
        };
    }
    fn block_402<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_402_0: const #14s : i
        let s_402_0: i128 = 14;
        // D s_402_1: read-var u#26151:u32
        let s_402_1: u32 = fn_state.u_26151;
        // D s_402_2: cast zx s_402_1 -> bv
        let s_402_2: Bits = Bits::new(s_402_1 as u128, 32u16);
        // C s_402_3: const #1u : u64
        let s_402_3: u64 = 1;
        // D s_402_4: bit-extract s_402_2 s_402_0 s_402_3
        let s_402_4: Bits = (Bits::new(
            ((s_402_2) >> (s_402_0)).value(),
            u16::try_from(s_402_3).unwrap(),
        ));
        // D s_402_5: cast reint s_402_4 -> u8
        let s_402_5: bool = ((s_402_4.value()) != 0);
        // C s_402_6: const #0s : i
        let s_402_6: i128 = 0;
        // C s_402_7: const #0u : u64
        let s_402_7: u64 = 0;
        // D s_402_8: cast zx s_402_5 -> u64
        let s_402_8: u64 = (s_402_5 as u64);
        // C s_402_9: const #1u : u64
        let s_402_9: u64 = 1;
        // D s_402_10: and s_402_8 s_402_9
        let s_402_10: u64 = ((s_402_8) & (s_402_9));
        // D s_402_11: cmp-eq s_402_10 s_402_9
        let s_402_11: bool = ((s_402_10) == (s_402_9));
        // D s_402_12: lsl s_402_8 s_402_6
        let s_402_12: u64 = s_402_8 << s_402_6;
        // D s_402_13: or s_402_7 s_402_12
        let s_402_13: u64 = ((s_402_7) | (s_402_12));
        // D s_402_14: cmpl s_402_12
        let s_402_14: u64 = !s_402_12;
        // D s_402_15: and s_402_7 s_402_14
        let s_402_15: u64 = ((s_402_7) & (s_402_14));
        // D s_402_16: select s_402_11 s_402_13 s_402_15
        let s_402_16: u64 = if s_402_11 { s_402_13 } else { s_402_15 };
        // D s_402_17: cast trunc s_402_16 -> u8
        let s_402_17: bool = ((s_402_16) != 0);
        // D s_402_18: cast zx s_402_17 -> bv
        let s_402_18: Bits = Bits::new(s_402_17 as u128, 1u16);
        // C s_402_19: const #1u : u8
        let s_402_19: bool = true;
        // C s_402_20: cast zx s_402_19 -> bv
        let s_402_20: Bits = Bits::new(s_402_19 as u128, 1u16);
        // D s_402_21: cmp-ne s_402_18 s_402_20
        let s_402_21: bool = ((s_402_18) != (s_402_20));
        // D s_402_22: write-var gs#383551 <= s_402_21
        fn_state.gs_383551 = s_402_21;
        // N s_402_23: jump b403
        return block_403(state, tracer, fn_state);
    }
    fn block_403<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_403_0: read-var gs#383551:u8
        let s_403_0: bool = fn_state.gs_383551;
        // N s_403_1: branch s_403_0 b405 b404
        if s_403_0 {
            return block_405(state, tracer, fn_state);
        } else {
            return block_404(state, tracer, fn_state);
        };
    }
    fn block_404<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_404_0: read-var u#26152:u8
        let s_404_0: u8 = fn_state.u_26152;
        // D s_404_1: read-var u#26153:u8
        let s_404_1: u8 = fn_state.u_26153;
        // D s_404_2: read-var u#26154:u8
        let s_404_2: u8 = fn_state.u_26154;
        // D s_404_3: read-var u#26155:u8
        let s_404_3: u8 = fn_state.u_26155;
        // D s_404_4: read-var u#26156:u8
        let s_404_4: bool = fn_state.u_26156;
        // D s_404_5: call decode_umulh_aarch64_instrs_integer_arithmetic_mul_widening_64_128hi(s_404_0, s_404_1, s_404_2, s_404_3, s_404_4)
        let s_404_5: () = decode_umulh_aarch64_instrs_integer_arithmetic_mul_widening_64_128hi(
            state,
            tracer,
            s_404_0,
            s_404_1,
            s_404_2,
            s_404_3,
            s_404_4,
        );
        // N s_404_6: return
        return;
    }
    fn block_405<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_405_0: panic
        panic!("{:?}", ());
        // N s_405_1: return
        return;
    }
    fn block_406<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_406_0: const #1u : u8
        let s_406_0: bool = true;
        // D s_406_1: write-var gs#383551 <= s_406_0
        fn_state.gs_383551 = s_406_0;
        // N s_406_2: jump b403
        return block_403(state, tracer, fn_state);
    }
    fn block_407<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_407_0: const #1u : u8
        let s_407_0: bool = true;
        // D s_407_1: write-var gs#383548 <= s_407_0
        fn_state.gs_383548 = s_407_0;
        // N s_407_2: jump b401
        return block_401(state, tracer, fn_state);
    }
    fn block_408<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_408_0: const #1u : u8
        let s_408_0: bool = true;
        // D s_408_1: write-var gs#383545 <= s_408_0
        fn_state.gs_383545 = s_408_0;
        // N s_408_2: jump b399
        return block_399(state, tracer, fn_state);
    }
    fn block_409<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_409_0: const #1u : u8
        let s_409_0: bool = true;
        // D s_409_1: write-var gs#383542 <= s_409_0
        fn_state.gs_383542 = s_409_0;
        // N s_409_2: jump b397
        return block_397(state, tracer, fn_state);
    }
    fn block_410<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_410_0: read-var merge#var.1:struct
        let s_410_0: u32 = fn_state.merge_var._1;
        // D s_410_1: write-var u#26158 <= s_410_0
        fn_state.u_26158 = s_410_0;
        // C s_410_2: const #21s : i
        let s_410_2: i128 = 21;
        // D s_410_3: read-var u#26158:u32
        let s_410_3: u32 = fn_state.u_26158;
        // D s_410_4: cast zx s_410_3 -> bv
        let s_410_4: Bits = Bits::new(s_410_3 as u128, 32u16);
        // C s_410_5: const #1s : i64
        let s_410_5: i64 = 1;
        // C s_410_6: cast zx s_410_5 -> i
        let s_410_6: i128 = (i128::try_from(s_410_5).unwrap());
        // C s_410_7: const #10s : i
        let s_410_7: i128 = 10;
        // C s_410_8: add s_410_7 s_410_6
        let s_410_8: i128 = (s_410_7 + s_410_6);
        // D s_410_9: bit-extract s_410_4 s_410_2 s_410_8
        let s_410_9: Bits = (Bits::new(
            ((s_410_4) >> (s_410_2)).value(),
            u16::try_from(s_410_8).unwrap(),
        ));
        // D s_410_10: cast reint s_410_9 -> u11
        let s_410_10: u16 = (s_410_9.value() as u16);
        // D s_410_11: cast zx s_410_10 -> bv
        let s_410_11: Bits = Bits::new(s_410_10 as u128, 11u16);
        // C s_410_12: const #1238u : u11
        let s_410_12: u16 = 1238;
        // C s_410_13: cast zx s_410_12 -> bv
        let s_410_13: Bits = Bits::new(s_410_12 as u128, 11u16);
        // D s_410_14: cmp-eq s_410_11 s_410_13
        let s_410_14: bool = ((s_410_11) == (s_410_13));
        // N s_410_15: branch s_410_14 b450 b411
        if s_410_14 {
            return block_450(state, tracer, fn_state);
        } else {
            return block_411(state, tracer, fn_state);
        };
    }
    fn block_411<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_411_0: const #0u : u8
        let s_411_0: bool = false;
        // D s_411_1: write-var gs#383557 <= s_411_0
        fn_state.gs_383557 = s_411_0;
        // N s_411_2: jump b412
        return block_412(state, tracer, fn_state);
    }
    fn block_412<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_412_0: read-var gs#383557:u8
        let s_412_0: bool = fn_state.gs_383557;
        // N s_412_1: branch s_412_0 b449 b413
        if s_412_0 {
            return block_449(state, tracer, fn_state);
        } else {
            return block_413(state, tracer, fn_state);
        };
    }
    fn block_413<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_413_0: const #0u : u8
        let s_413_0: bool = false;
        // D s_413_1: write-var gs#383559 <= s_413_0
        fn_state.gs_383559 = s_413_0;
        // N s_413_2: jump b414
        return block_414(state, tracer, fn_state);
    }
    fn block_414<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_414_0: read-var gs#383559:u8
        let s_414_0: bool = fn_state.gs_383559;
        // D s_414_1: not s_414_0
        let s_414_1: bool = !s_414_0;
        // N s_414_2: branch s_414_1 b416 b415
        if s_414_1 {
            return block_416(state, tracer, fn_state);
        } else {
            return block_415(state, tracer, fn_state);
        };
    }
    fn block_415<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_415_0: const #991s : i
        let s_415_0: i128 = 991;
        // C s_415_1: const #14696u : u32
        let s_415_1: u32 = 14696;
        // N s_415_2: write-reg s_415_1 <= s_415_0
        let s_415_2: () = {
            state.write_register::<i128>(s_415_1 as isize, s_415_0);
            tracer.write_register(s_415_1 as isize, s_415_0);
        };
        // C s_415_3: const #0s : i
        let s_415_3: i128 = 0;
        // C s_415_4: const #5s : i
        let s_415_4: i128 = 5;
        // D s_415_5: read-var u#26158:u32
        let s_415_5: u32 = fn_state.u_26158;
        // D s_415_6: cast zx s_415_5 -> bv
        let s_415_6: Bits = Bits::new(s_415_5 as u128, 32u16);
        // D s_415_7: bit-extract s_415_6 s_415_3 s_415_4
        let s_415_7: Bits = (Bits::new(
            ((s_415_6) >> (s_415_3)).value(),
            u16::try_from(s_415_4).unwrap(),
        ));
        // D s_415_8: cast reint s_415_7 -> u8
        let s_415_8: u8 = (s_415_7.value() as u8);
        // C s_415_9: const #5s : i
        let s_415_9: i128 = 5;
        // C s_415_10: const #5s : i
        let s_415_10: i128 = 5;
        // D s_415_11: read-var u#26158:u32
        let s_415_11: u32 = fn_state.u_26158;
        // D s_415_12: cast zx s_415_11 -> bv
        let s_415_12: Bits = Bits::new(s_415_11 as u128, 32u16);
        // D s_415_13: bit-extract s_415_12 s_415_9 s_415_10
        let s_415_13: Bits = (Bits::new(
            ((s_415_12) >> (s_415_9)).value(),
            u16::try_from(s_415_10).unwrap(),
        ));
        // D s_415_14: cast reint s_415_13 -> u8
        let s_415_14: u8 = (s_415_13.value() as u8);
        // C s_415_15: const #16s : i
        let s_415_15: i128 = 16;
        // C s_415_16: const #5s : i
        let s_415_16: i128 = 5;
        // D s_415_17: read-var u#26158:u32
        let s_415_17: u32 = fn_state.u_26158;
        // D s_415_18: cast zx s_415_17 -> bv
        let s_415_18: Bits = Bits::new(s_415_17 as u128, 32u16);
        // D s_415_19: bit-extract s_415_18 s_415_15 s_415_16
        let s_415_19: Bits = (Bits::new(
            ((s_415_18) >> (s_415_15)).value(),
            u16::try_from(s_415_16).unwrap(),
        ));
        // D s_415_20: cast reint s_415_19 -> u8
        let s_415_20: u8 = (s_415_19.value() as u8);
        // D s_415_21: call decode_subp_aarch64_instrs_integer_arithmetic_pointer_mcsubtracttaggedaddress(s_415_8, s_415_14, s_415_20)
        let s_415_21: () = decode_subp_aarch64_instrs_integer_arithmetic_pointer_mcsubtracttaggedaddress(
            state,
            tracer,
            s_415_8,
            s_415_14,
            s_415_20,
        );
        // N s_415_22: return
        return;
    }
    fn block_416<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_416_0: read-var merge#var.1:struct
        let s_416_0: u32 = fn_state.merge_var._1;
        // D s_416_1: write-var u#26163 <= s_416_0
        fn_state.u_26163 = s_416_0;
        // C s_416_2: const #21s : i
        let s_416_2: i128 = 21;
        // D s_416_3: read-var u#26163:u32
        let s_416_3: u32 = fn_state.u_26163;
        // D s_416_4: cast zx s_416_3 -> bv
        let s_416_4: Bits = Bits::new(s_416_3 as u128, 32u16);
        // C s_416_5: const #1s : i64
        let s_416_5: i64 = 1;
        // C s_416_6: cast zx s_416_5 -> i
        let s_416_6: i128 = (i128::try_from(s_416_5).unwrap());
        // C s_416_7: const #10s : i
        let s_416_7: i128 = 10;
        // C s_416_8: add s_416_7 s_416_6
        let s_416_8: i128 = (s_416_7 + s_416_6);
        // D s_416_9: bit-extract s_416_4 s_416_2 s_416_8
        let s_416_9: Bits = (Bits::new(
            ((s_416_4) >> (s_416_2)).value(),
            u16::try_from(s_416_8).unwrap(),
        ));
        // D s_416_10: cast reint s_416_9 -> u11
        let s_416_10: u16 = (s_416_9.value() as u16);
        // D s_416_11: cast zx s_416_10 -> bv
        let s_416_11: Bits = Bits::new(s_416_10 as u128, 11u16);
        // C s_416_12: const #1494u : u11
        let s_416_12: u16 = 1494;
        // C s_416_13: cast zx s_416_12 -> bv
        let s_416_13: Bits = Bits::new(s_416_12 as u128, 11u16);
        // D s_416_14: cmp-eq s_416_11 s_416_13
        let s_416_14: bool = ((s_416_11) == (s_416_13));
        // N s_416_15: branch s_416_14 b448 b417
        if s_416_14 {
            return block_448(state, tracer, fn_state);
        } else {
            return block_417(state, tracer, fn_state);
        };
    }
    fn block_417<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_417_0: const #0u : u8
        let s_417_0: bool = false;
        // D s_417_1: write-var gs#383573 <= s_417_0
        fn_state.gs_383573 = s_417_0;
        // N s_417_2: jump b418
        return block_418(state, tracer, fn_state);
    }
    fn block_418<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_418_0: read-var gs#383573:u8
        let s_418_0: bool = fn_state.gs_383573;
        // N s_418_1: branch s_418_0 b447 b419
        if s_418_0 {
            return block_447(state, tracer, fn_state);
        } else {
            return block_419(state, tracer, fn_state);
        };
    }
    fn block_419<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_419_0: const #0u : u8
        let s_419_0: bool = false;
        // D s_419_1: write-var gs#383575 <= s_419_0
        fn_state.gs_383575 = s_419_0;
        // N s_419_2: jump b420
        return block_420(state, tracer, fn_state);
    }
    fn block_420<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_420_0: read-var gs#383575:u8
        let s_420_0: bool = fn_state.gs_383575;
        // D s_420_1: not s_420_0
        let s_420_1: bool = !s_420_0;
        // N s_420_2: branch s_420_1 b422 b421
        if s_420_1 {
            return block_422(state, tracer, fn_state);
        } else {
            return block_421(state, tracer, fn_state);
        };
    }
    fn block_421<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_421_0: const #992s : i
        let s_421_0: i128 = 992;
        // C s_421_1: const #14696u : u32
        let s_421_1: u32 = 14696;
        // N s_421_2: write-reg s_421_1 <= s_421_0
        let s_421_2: () = {
            state.write_register::<i128>(s_421_1 as isize, s_421_0);
            tracer.write_register(s_421_1 as isize, s_421_0);
        };
        // C s_421_3: const #0s : i
        let s_421_3: i128 = 0;
        // C s_421_4: const #5s : i
        let s_421_4: i128 = 5;
        // D s_421_5: read-var u#26163:u32
        let s_421_5: u32 = fn_state.u_26163;
        // D s_421_6: cast zx s_421_5 -> bv
        let s_421_6: Bits = Bits::new(s_421_5 as u128, 32u16);
        // D s_421_7: bit-extract s_421_6 s_421_3 s_421_4
        let s_421_7: Bits = (Bits::new(
            ((s_421_6) >> (s_421_3)).value(),
            u16::try_from(s_421_4).unwrap(),
        ));
        // D s_421_8: cast reint s_421_7 -> u8
        let s_421_8: u8 = (s_421_7.value() as u8);
        // C s_421_9: const #5s : i
        let s_421_9: i128 = 5;
        // C s_421_10: const #5s : i
        let s_421_10: i128 = 5;
        // D s_421_11: read-var u#26163:u32
        let s_421_11: u32 = fn_state.u_26163;
        // D s_421_12: cast zx s_421_11 -> bv
        let s_421_12: Bits = Bits::new(s_421_11 as u128, 32u16);
        // D s_421_13: bit-extract s_421_12 s_421_9 s_421_10
        let s_421_13: Bits = (Bits::new(
            ((s_421_12) >> (s_421_9)).value(),
            u16::try_from(s_421_10).unwrap(),
        ));
        // D s_421_14: cast reint s_421_13 -> u8
        let s_421_14: u8 = (s_421_13.value() as u8);
        // C s_421_15: const #16s : i
        let s_421_15: i128 = 16;
        // C s_421_16: const #5s : i
        let s_421_16: i128 = 5;
        // D s_421_17: read-var u#26163:u32
        let s_421_17: u32 = fn_state.u_26163;
        // D s_421_18: cast zx s_421_17 -> bv
        let s_421_18: Bits = Bits::new(s_421_17 as u128, 32u16);
        // D s_421_19: bit-extract s_421_18 s_421_15 s_421_16
        let s_421_19: Bits = (Bits::new(
            ((s_421_18) >> (s_421_15)).value(),
            u16::try_from(s_421_16).unwrap(),
        ));
        // D s_421_20: cast reint s_421_19 -> u8
        let s_421_20: u8 = (s_421_19.value() as u8);
        // D s_421_21: call decode_subps_aarch64_instrs_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags(s_421_8, s_421_14, s_421_20)
        let s_421_21: () = decode_subps_aarch64_instrs_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags(
            state,
            tracer,
            s_421_8,
            s_421_14,
            s_421_20,
        );
        // N s_421_22: return
        return;
    }
    fn block_422<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_422_0: read-var merge#var.1:struct
        let s_422_0: u32 = fn_state.merge_var._1;
        // D s_422_1: write-var u#26168 <= s_422_0
        fn_state.u_26168 = s_422_0;
        // C s_422_2: const #21s : i
        let s_422_2: i128 = 21;
        // D s_422_3: read-var u#26168:u32
        let s_422_3: u32 = fn_state.u_26168;
        // D s_422_4: cast zx s_422_3 -> bv
        let s_422_4: Bits = Bits::new(s_422_3 as u128, 32u16);
        // C s_422_5: const #1s : i64
        let s_422_5: i64 = 1;
        // C s_422_6: cast zx s_422_5 -> i
        let s_422_6: i128 = (i128::try_from(s_422_5).unwrap());
        // C s_422_7: const #9s : i
        let s_422_7: i128 = 9;
        // C s_422_8: add s_422_7 s_422_6
        let s_422_8: i128 = (s_422_7 + s_422_6);
        // D s_422_9: bit-extract s_422_4 s_422_2 s_422_8
        let s_422_9: Bits = (Bits::new(
            ((s_422_4) >> (s_422_2)).value(),
            u16::try_from(s_422_8).unwrap(),
        ));
        // D s_422_10: cast reint s_422_9 -> u10
        let s_422_10: u16 = (s_422_9.value() as u16);
        // D s_422_11: cast zx s_422_10 -> bv
        let s_422_11: Bits = Bits::new(s_422_10 as u128, 10u16);
        // C s_422_12: const #214u : u10
        let s_422_12: u16 = 214;
        // C s_422_13: cast zx s_422_12 -> bv
        let s_422_13: Bits = Bits::new(s_422_12 as u128, 10u16);
        // D s_422_14: cmp-eq s_422_11 s_422_13
        let s_422_14: bool = ((s_422_11) == (s_422_13));
        // N s_422_15: branch s_422_14 b446 b423
        if s_422_14 {
            return block_446(state, tracer, fn_state);
        } else {
            return block_423(state, tracer, fn_state);
        };
    }
    fn block_423<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_423_0: const #0u : u8
        let s_423_0: bool = false;
        // D s_423_1: write-var gs#383589 <= s_423_0
        fn_state.gs_383589 = s_423_0;
        // N s_423_2: jump b424
        return block_424(state, tracer, fn_state);
    }
    fn block_424<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_424_0: read-var gs#383589:u8
        let s_424_0: bool = fn_state.gs_383589;
        // N s_424_1: branch s_424_0 b445 b425
        if s_424_0 {
            return block_445(state, tracer, fn_state);
        } else {
            return block_425(state, tracer, fn_state);
        };
    }
    fn block_425<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_425_0: const #0u : u8
        let s_425_0: bool = false;
        // D s_425_1: write-var gs#383591 <= s_425_0
        fn_state.gs_383591 = s_425_0;
        // N s_425_2: jump b426
        return block_426(state, tracer, fn_state);
    }
    fn block_426<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_426_0: read-var gs#383591:u8
        let s_426_0: bool = fn_state.gs_383591;
        // D s_426_1: not s_426_0
        let s_426_1: bool = !s_426_0;
        // N s_426_2: branch s_426_1 b428 b427
        if s_426_1 {
            return block_428(state, tracer, fn_state);
        } else {
            return block_427(state, tracer, fn_state);
        };
    }
    fn block_427<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_427_0: const #1019s : i
        let s_427_0: i128 = 1019;
        // C s_427_1: const #14696u : u32
        let s_427_1: u32 = 14696;
        // N s_427_2: write-reg s_427_1 <= s_427_0
        let s_427_2: () = {
            state.write_register::<i128>(s_427_1 as isize, s_427_0);
            tracer.write_register(s_427_1 as isize, s_427_0);
        };
        // C s_427_3: const #0s : i
        let s_427_3: i128 = 0;
        // C s_427_4: const #5s : i
        let s_427_4: i128 = 5;
        // D s_427_5: read-var u#26168:u32
        let s_427_5: u32 = fn_state.u_26168;
        // D s_427_6: cast zx s_427_5 -> bv
        let s_427_6: Bits = Bits::new(s_427_5 as u128, 32u16);
        // D s_427_7: bit-extract s_427_6 s_427_3 s_427_4
        let s_427_7: Bits = (Bits::new(
            ((s_427_6) >> (s_427_3)).value(),
            u16::try_from(s_427_4).unwrap(),
        ));
        // D s_427_8: cast reint s_427_7 -> u8
        let s_427_8: u8 = (s_427_7.value() as u8);
        // C s_427_9: const #5s : i
        let s_427_9: i128 = 5;
        // C s_427_10: const #5s : i
        let s_427_10: i128 = 5;
        // D s_427_11: read-var u#26168:u32
        let s_427_11: u32 = fn_state.u_26168;
        // D s_427_12: cast zx s_427_11 -> bv
        let s_427_12: Bits = Bits::new(s_427_11 as u128, 32u16);
        // D s_427_13: bit-extract s_427_12 s_427_9 s_427_10
        let s_427_13: Bits = (Bits::new(
            ((s_427_12) >> (s_427_9)).value(),
            u16::try_from(s_427_10).unwrap(),
        ));
        // D s_427_14: cast reint s_427_13 -> u8
        let s_427_14: u8 = (s_427_13.value() as u8);
        // C s_427_15: const #16s : i
        let s_427_15: i128 = 16;
        // C s_427_16: const #5s : i
        let s_427_16: i128 = 5;
        // D s_427_17: read-var u#26168:u32
        let s_427_17: u32 = fn_state.u_26168;
        // D s_427_18: cast zx s_427_17 -> bv
        let s_427_18: Bits = Bits::new(s_427_17 as u128, 32u16);
        // D s_427_19: bit-extract s_427_18 s_427_15 s_427_16
        let s_427_19: Bits = (Bits::new(
            ((s_427_18) >> (s_427_15)).value(),
            u16::try_from(s_427_16).unwrap(),
        ));
        // D s_427_20: cast reint s_427_19 -> u8
        let s_427_20: u8 = (s_427_19.value() as u8);
        // C s_427_21: const #31s : i
        let s_427_21: i128 = 31;
        // C s_427_22: const #1s : i
        let s_427_22: i128 = 1;
        // D s_427_23: read-var u#26168:u32
        let s_427_23: u32 = fn_state.u_26168;
        // D s_427_24: cast zx s_427_23 -> bv
        let s_427_24: Bits = Bits::new(s_427_23 as u128, 32u16);
        // D s_427_25: bit-extract s_427_24 s_427_21 s_427_22
        let s_427_25: Bits = (Bits::new(
            ((s_427_24) >> (s_427_21)).value(),
            u16::try_from(s_427_22).unwrap(),
        ));
        // D s_427_26: cast reint s_427_25 -> u8
        let s_427_26: bool = ((s_427_25.value()) != 0);
        // D s_427_27: call decode_umax_reg_aarch64_instrs_integer_arithmetic_max_min_umax_reg(s_427_8, s_427_14, s_427_20, s_427_26)
        let s_427_27: () = decode_umax_reg_aarch64_instrs_integer_arithmetic_max_min_umax_reg(
            state,
            tracer,
            s_427_8,
            s_427_14,
            s_427_20,
            s_427_26,
        );
        // N s_427_28: return
        return;
    }
    fn block_428<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_428_0: read-var merge#var.1:struct
        let s_428_0: u32 = fn_state.merge_var._1;
        // D s_428_1: write-var u#26174 <= s_428_0
        fn_state.u_26174 = s_428_0;
        // C s_428_2: const #21s : i
        let s_428_2: i128 = 21;
        // D s_428_3: read-var u#26174:u32
        let s_428_3: u32 = fn_state.u_26174;
        // D s_428_4: cast zx s_428_3 -> bv
        let s_428_4: Bits = Bits::new(s_428_3 as u128, 32u16);
        // C s_428_5: const #1s : i64
        let s_428_5: i64 = 1;
        // C s_428_6: cast zx s_428_5 -> i
        let s_428_6: i128 = (i128::try_from(s_428_5).unwrap());
        // C s_428_7: const #9s : i
        let s_428_7: i128 = 9;
        // C s_428_8: add s_428_7 s_428_6
        let s_428_8: i128 = (s_428_7 + s_428_6);
        // D s_428_9: bit-extract s_428_4 s_428_2 s_428_8
        let s_428_9: Bits = (Bits::new(
            ((s_428_4) >> (s_428_2)).value(),
            u16::try_from(s_428_8).unwrap(),
        ));
        // D s_428_10: cast reint s_428_9 -> u10
        let s_428_10: u16 = (s_428_9.value() as u16);
        // D s_428_11: cast zx s_428_10 -> bv
        let s_428_11: Bits = Bits::new(s_428_10 as u128, 10u16);
        // C s_428_12: const #214u : u10
        let s_428_12: u16 = 214;
        // C s_428_13: cast zx s_428_12 -> bv
        let s_428_13: Bits = Bits::new(s_428_12 as u128, 10u16);
        // D s_428_14: cmp-eq s_428_11 s_428_13
        let s_428_14: bool = ((s_428_11) == (s_428_13));
        // N s_428_15: branch s_428_14 b444 b429
        if s_428_14 {
            return block_444(state, tracer, fn_state);
        } else {
            return block_429(state, tracer, fn_state);
        };
    }
    fn block_429<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_429_0: const #0u : u8
        let s_429_0: bool = false;
        // D s_429_1: write-var gs#383607 <= s_429_0
        fn_state.gs_383607 = s_429_0;
        // N s_429_2: jump b430
        return block_430(state, tracer, fn_state);
    }
    fn block_430<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_430_0: read-var gs#383607:u8
        let s_430_0: bool = fn_state.gs_383607;
        // N s_430_1: branch s_430_0 b443 b431
        if s_430_0 {
            return block_443(state, tracer, fn_state);
        } else {
            return block_431(state, tracer, fn_state);
        };
    }
    fn block_431<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_431_0: const #0u : u8
        let s_431_0: bool = false;
        // D s_431_1: write-var gs#383609 <= s_431_0
        fn_state.gs_383609 = s_431_0;
        // N s_431_2: jump b432
        return block_432(state, tracer, fn_state);
    }
    fn block_432<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_432_0: read-var gs#383609:u8
        let s_432_0: bool = fn_state.gs_383609;
        // D s_432_1: not s_432_0
        let s_432_1: bool = !s_432_0;
        // N s_432_2: branch s_432_1 b434 b433
        if s_432_1 {
            return block_434(state, tracer, fn_state);
        } else {
            return block_433(state, tracer, fn_state);
        };
    }
    fn block_433<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_433_0: const #1020s : i
        let s_433_0: i128 = 1020;
        // C s_433_1: const #14696u : u32
        let s_433_1: u32 = 14696;
        // N s_433_2: write-reg s_433_1 <= s_433_0
        let s_433_2: () = {
            state.write_register::<i128>(s_433_1 as isize, s_433_0);
            tracer.write_register(s_433_1 as isize, s_433_0);
        };
        // C s_433_3: const #0s : i
        let s_433_3: i128 = 0;
        // C s_433_4: const #5s : i
        let s_433_4: i128 = 5;
        // D s_433_5: read-var u#26174:u32
        let s_433_5: u32 = fn_state.u_26174;
        // D s_433_6: cast zx s_433_5 -> bv
        let s_433_6: Bits = Bits::new(s_433_5 as u128, 32u16);
        // D s_433_7: bit-extract s_433_6 s_433_3 s_433_4
        let s_433_7: Bits = (Bits::new(
            ((s_433_6) >> (s_433_3)).value(),
            u16::try_from(s_433_4).unwrap(),
        ));
        // D s_433_8: cast reint s_433_7 -> u8
        let s_433_8: u8 = (s_433_7.value() as u8);
        // C s_433_9: const #5s : i
        let s_433_9: i128 = 5;
        // C s_433_10: const #5s : i
        let s_433_10: i128 = 5;
        // D s_433_11: read-var u#26174:u32
        let s_433_11: u32 = fn_state.u_26174;
        // D s_433_12: cast zx s_433_11 -> bv
        let s_433_12: Bits = Bits::new(s_433_11 as u128, 32u16);
        // D s_433_13: bit-extract s_433_12 s_433_9 s_433_10
        let s_433_13: Bits = (Bits::new(
            ((s_433_12) >> (s_433_9)).value(),
            u16::try_from(s_433_10).unwrap(),
        ));
        // D s_433_14: cast reint s_433_13 -> u8
        let s_433_14: u8 = (s_433_13.value() as u8);
        // C s_433_15: const #16s : i
        let s_433_15: i128 = 16;
        // C s_433_16: const #5s : i
        let s_433_16: i128 = 5;
        // D s_433_17: read-var u#26174:u32
        let s_433_17: u32 = fn_state.u_26174;
        // D s_433_18: cast zx s_433_17 -> bv
        let s_433_18: Bits = Bits::new(s_433_17 as u128, 32u16);
        // D s_433_19: bit-extract s_433_18 s_433_15 s_433_16
        let s_433_19: Bits = (Bits::new(
            ((s_433_18) >> (s_433_15)).value(),
            u16::try_from(s_433_16).unwrap(),
        ));
        // D s_433_20: cast reint s_433_19 -> u8
        let s_433_20: u8 = (s_433_19.value() as u8);
        // C s_433_21: const #31s : i
        let s_433_21: i128 = 31;
        // C s_433_22: const #1s : i
        let s_433_22: i128 = 1;
        // D s_433_23: read-var u#26174:u32
        let s_433_23: u32 = fn_state.u_26174;
        // D s_433_24: cast zx s_433_23 -> bv
        let s_433_24: Bits = Bits::new(s_433_23 as u128, 32u16);
        // D s_433_25: bit-extract s_433_24 s_433_21 s_433_22
        let s_433_25: Bits = (Bits::new(
            ((s_433_24) >> (s_433_21)).value(),
            u16::try_from(s_433_22).unwrap(),
        ));
        // D s_433_26: cast reint s_433_25 -> u8
        let s_433_26: bool = ((s_433_25.value()) != 0);
        // D s_433_27: call decode_umin_reg_aarch64_instrs_integer_arithmetic_max_min_umin_reg(s_433_8, s_433_14, s_433_20, s_433_26)
        let s_433_27: () = decode_umin_reg_aarch64_instrs_integer_arithmetic_max_min_umin_reg(
            state,
            tracer,
            s_433_8,
            s_433_14,
            s_433_20,
            s_433_26,
        );
        // N s_433_28: return
        return;
    }
    fn block_434<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_434_0: read-var merge#var.1:struct
        let s_434_0: u32 = fn_state.merge_var._1;
        // D s_434_1: write-var u#26180 <= s_434_0
        fn_state.u_26180 = s_434_0;
        // C s_434_2: const #11s : i
        let s_434_2: i128 = 11;
        // D s_434_3: read-var u#26180:u32
        let s_434_3: u32 = fn_state.u_26180;
        // D s_434_4: cast zx s_434_3 -> bv
        let s_434_4: Bits = Bits::new(s_434_3 as u128, 32u16);
        // C s_434_5: const #1s : i64
        let s_434_5: i64 = 1;
        // C s_434_6: cast zx s_434_5 -> i
        let s_434_6: i128 = (i128::try_from(s_434_5).unwrap());
        // C s_434_7: const #20s : i
        let s_434_7: i128 = 20;
        // C s_434_8: add s_434_7 s_434_6
        let s_434_8: i128 = (s_434_7 + s_434_6);
        // D s_434_9: bit-extract s_434_4 s_434_2 s_434_8
        let s_434_9: Bits = (Bits::new(
            ((s_434_4) >> (s_434_2)).value(),
            u16::try_from(s_434_8).unwrap(),
        ));
        // D s_434_10: cast reint s_434_9 -> u21
        let s_434_10: u32 = (s_434_9.value() as u32);
        // D s_434_11: cast zx s_434_10 -> bv
        let s_434_11: Bits = Bits::new(s_434_10 as u128, 21u16);
        // C s_434_12: const #1792040u : u21
        let s_434_12: u32 = 1792040;
        // C s_434_13: cast zx s_434_12 -> bv
        let s_434_13: Bits = Bits::new(s_434_12 as u128, 21u16);
        // D s_434_14: cmp-eq s_434_11 s_434_13
        let s_434_14: bool = ((s_434_11) == (s_434_13));
        // N s_434_15: branch s_434_14 b442 b435
        if s_434_14 {
            return block_442(state, tracer, fn_state);
        } else {
            return block_435(state, tracer, fn_state);
        };
    }
    fn block_435<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_435_0: const #0u : u8
        let s_435_0: bool = false;
        // D s_435_1: write-var gs#383625 <= s_435_0
        fn_state.gs_383625 = s_435_0;
        // N s_435_2: jump b436
        return block_436(state, tracer, fn_state);
    }
    fn block_436<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_436_0: read-var gs#383625:u8
        let s_436_0: bool = fn_state.gs_383625;
        // N s_436_1: branch s_436_0 b441 b437
        if s_436_0 {
            return block_441(state, tracer, fn_state);
        } else {
            return block_437(state, tracer, fn_state);
        };
    }
    fn block_437<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_437_0: const #0u : u8
        let s_437_0: bool = false;
        // D s_437_1: write-var gs#383627 <= s_437_0
        fn_state.gs_383627 = s_437_0;
        // N s_437_2: jump b438
        return block_438(state, tracer, fn_state);
    }
    fn block_438<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_438_0: read-var gs#383627:u8
        let s_438_0: bool = fn_state.gs_383627;
        // D s_438_1: not s_438_0
        let s_438_1: bool = !s_438_0;
        // N s_438_2: branch s_438_1 b440 b439
        if s_438_1 {
            return block_440(state, tracer, fn_state);
        } else {
            return block_439(state, tracer, fn_state);
        };
    }
    fn block_439<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_439_0: const #1032s : i
        let s_439_0: i128 = 1032;
        // C s_439_1: const #14696u : u32
        let s_439_1: u32 = 14696;
        // N s_439_2: write-reg s_439_1 <= s_439_0
        let s_439_2: () = {
            state.write_register::<i128>(s_439_1 as isize, s_439_0);
            tracer.write_register(s_439_1 as isize, s_439_0);
        };
        // C s_439_3: const #0s : i
        let s_439_3: i128 = 0;
        // C s_439_4: const #5s : i
        let s_439_4: i128 = 5;
        // D s_439_5: read-var u#26180:u32
        let s_439_5: u32 = fn_state.u_26180;
        // D s_439_6: cast zx s_439_5 -> bv
        let s_439_6: Bits = Bits::new(s_439_5 as u128, 32u16);
        // D s_439_7: bit-extract s_439_6 s_439_3 s_439_4
        let s_439_7: Bits = (Bits::new(
            ((s_439_6) >> (s_439_3)).value(),
            u16::try_from(s_439_4).unwrap(),
        ));
        // D s_439_8: cast reint s_439_7 -> u8
        let s_439_8: u8 = (s_439_7.value() as u8);
        // C s_439_9: const #5s : i
        let s_439_9: i128 = 5;
        // C s_439_10: const #5s : i
        let s_439_10: i128 = 5;
        // D s_439_11: read-var u#26180:u32
        let s_439_11: u32 = fn_state.u_26180;
        // D s_439_12: cast zx s_439_11 -> bv
        let s_439_12: Bits = Bits::new(s_439_11 as u128, 32u16);
        // D s_439_13: bit-extract s_439_12 s_439_9 s_439_10
        let s_439_13: Bits = (Bits::new(
            ((s_439_12) >> (s_439_9)).value(),
            u16::try_from(s_439_10).unwrap(),
        ));
        // D s_439_14: cast reint s_439_13 -> u8
        let s_439_14: u8 = (s_439_13.value() as u8);
        // C s_439_15: const #10s : i
        let s_439_15: i128 = 10;
        // C s_439_16: const #1s : i
        let s_439_16: i128 = 1;
        // D s_439_17: read-var u#26180:u32
        let s_439_17: u32 = fn_state.u_26180;
        // D s_439_18: cast zx s_439_17 -> bv
        let s_439_18: Bits = Bits::new(s_439_17 as u128, 32u16);
        // D s_439_19: bit-extract s_439_18 s_439_15 s_439_16
        let s_439_19: Bits = (Bits::new(
            ((s_439_18) >> (s_439_15)).value(),
            u16::try_from(s_439_16).unwrap(),
        ));
        // D s_439_20: cast reint s_439_19 -> u8
        let s_439_20: bool = ((s_439_19.value()) != 0);
        // D s_439_21: call decode_xpac_aarch64_instrs_integer_pac_strip_dp_1src(s_439_8, s_439_14, s_439_20)
        let s_439_21: () = decode_xpac_aarch64_instrs_integer_pac_strip_dp_1src(
            state,
            tracer,
            s_439_8,
            s_439_14,
            s_439_20,
        );
        // N s_439_22: return
        return;
    }
    fn block_440<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_440_0: panic
        panic!("{:?}", ());
        // N s_440_1: return
        return;
    }
    fn block_441<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_441_0: const #1032s : i
        let s_441_0: i128 = 1032;
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
        // D s_441_4: write-var gs#383627 <= s_441_3
        fn_state.gs_383627 = s_441_3;
        // N s_441_5: jump b438
        return block_438(state, tracer, fn_state);
    }
    fn block_442<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_442_0: const #5s : i
        let s_442_0: i128 = 5;
        // D s_442_1: read-var u#26180:u32
        let s_442_1: u32 = fn_state.u_26180;
        // D s_442_2: cast zx s_442_1 -> bv
        let s_442_2: Bits = Bits::new(s_442_1 as u128, 32u16);
        // C s_442_3: const #1s : i64
        let s_442_3: i64 = 1;
        // C s_442_4: cast zx s_442_3 -> i
        let s_442_4: i128 = (i128::try_from(s_442_3).unwrap());
        // C s_442_5: const #4s : i
        let s_442_5: i128 = 4;
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
        let s_442_9: Bits = Bits::new(s_442_8 as u128, 5u16);
        // C s_442_10: const #31u : u8
        let s_442_10: u8 = 31;
        // C s_442_11: cast zx s_442_10 -> bv
        let s_442_11: Bits = Bits::new(s_442_10 as u128, 5u16);
        // D s_442_12: cmp-eq s_442_9 s_442_11
        let s_442_12: bool = ((s_442_9) == (s_442_11));
        // D s_442_13: write-var gs#383625 <= s_442_12
        fn_state.gs_383625 = s_442_12;
        // N s_442_14: jump b436
        return block_436(state, tracer, fn_state);
    }
    fn block_443<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_443_0: const #1020s : i
        let s_443_0: i128 = 1020;
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
        // D s_443_4: write-var gs#383609 <= s_443_3
        fn_state.gs_383609 = s_443_3;
        // N s_443_5: jump b432
        return block_432(state, tracer, fn_state);
    }
    fn block_444<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_444_0: const #10s : i
        let s_444_0: i128 = 10;
        // D s_444_1: read-var u#26174:u32
        let s_444_1: u32 = fn_state.u_26174;
        // D s_444_2: cast zx s_444_1 -> bv
        let s_444_2: Bits = Bits::new(s_444_1 as u128, 32u16);
        // C s_444_3: const #1s : i64
        let s_444_3: i64 = 1;
        // C s_444_4: cast zx s_444_3 -> i
        let s_444_4: i128 = (i128::try_from(s_444_3).unwrap());
        // C s_444_5: const #5s : i
        let s_444_5: i128 = 5;
        // C s_444_6: add s_444_5 s_444_4
        let s_444_6: i128 = (s_444_5 + s_444_4);
        // D s_444_7: bit-extract s_444_2 s_444_0 s_444_6
        let s_444_7: Bits = (Bits::new(
            ((s_444_2) >> (s_444_0)).value(),
            u16::try_from(s_444_6).unwrap(),
        ));
        // D s_444_8: cast reint s_444_7 -> u8
        let s_444_8: u8 = (s_444_7.value() as u8);
        // D s_444_9: cast zx s_444_8 -> bv
        let s_444_9: Bits = Bits::new(s_444_8 as u128, 6u16);
        // C s_444_10: const #27u : u8
        let s_444_10: u8 = 27;
        // C s_444_11: cast zx s_444_10 -> bv
        let s_444_11: Bits = Bits::new(s_444_10 as u128, 6u16);
        // D s_444_12: cmp-eq s_444_9 s_444_11
        let s_444_12: bool = ((s_444_9) == (s_444_11));
        // D s_444_13: write-var gs#383607 <= s_444_12
        fn_state.gs_383607 = s_444_12;
        // N s_444_14: jump b430
        return block_430(state, tracer, fn_state);
    }
    fn block_445<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_445_0: const #1019s : i
        let s_445_0: i128 = 1019;
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
        // D s_445_4: write-var gs#383591 <= s_445_3
        fn_state.gs_383591 = s_445_3;
        // N s_445_5: jump b426
        return block_426(state, tracer, fn_state);
    }
    fn block_446<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_446_0: const #10s : i
        let s_446_0: i128 = 10;
        // D s_446_1: read-var u#26168:u32
        let s_446_1: u32 = fn_state.u_26168;
        // D s_446_2: cast zx s_446_1 -> bv
        let s_446_2: Bits = Bits::new(s_446_1 as u128, 32u16);
        // C s_446_3: const #1s : i64
        let s_446_3: i64 = 1;
        // C s_446_4: cast zx s_446_3 -> i
        let s_446_4: i128 = (i128::try_from(s_446_3).unwrap());
        // C s_446_5: const #5s : i
        let s_446_5: i128 = 5;
        // C s_446_6: add s_446_5 s_446_4
        let s_446_6: i128 = (s_446_5 + s_446_4);
        // D s_446_7: bit-extract s_446_2 s_446_0 s_446_6
        let s_446_7: Bits = (Bits::new(
            ((s_446_2) >> (s_446_0)).value(),
            u16::try_from(s_446_6).unwrap(),
        ));
        // D s_446_8: cast reint s_446_7 -> u8
        let s_446_8: u8 = (s_446_7.value() as u8);
        // D s_446_9: cast zx s_446_8 -> bv
        let s_446_9: Bits = Bits::new(s_446_8 as u128, 6u16);
        // C s_446_10: const #25u : u8
        let s_446_10: u8 = 25;
        // C s_446_11: cast zx s_446_10 -> bv
        let s_446_11: Bits = Bits::new(s_446_10 as u128, 6u16);
        // D s_446_12: cmp-eq s_446_9 s_446_11
        let s_446_12: bool = ((s_446_9) == (s_446_11));
        // D s_446_13: write-var gs#383589 <= s_446_12
        fn_state.gs_383589 = s_446_12;
        // N s_446_14: jump b424
        return block_424(state, tracer, fn_state);
    }
    fn block_447<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_447_0: const #992s : i
        let s_447_0: i128 = 992;
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
        // D s_447_4: write-var gs#383575 <= s_447_3
        fn_state.gs_383575 = s_447_3;
        // N s_447_5: jump b420
        return block_420(state, tracer, fn_state);
    }
    fn block_448<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_448_0: const #10s : i
        let s_448_0: i128 = 10;
        // D s_448_1: read-var u#26163:u32
        let s_448_1: u32 = fn_state.u_26163;
        // D s_448_2: cast zx s_448_1 -> bv
        let s_448_2: Bits = Bits::new(s_448_1 as u128, 32u16);
        // C s_448_3: const #1s : i64
        let s_448_3: i64 = 1;
        // C s_448_4: cast zx s_448_3 -> i
        let s_448_4: i128 = (i128::try_from(s_448_3).unwrap());
        // C s_448_5: const #5s : i
        let s_448_5: i128 = 5;
        // C s_448_6: add s_448_5 s_448_4
        let s_448_6: i128 = (s_448_5 + s_448_4);
        // D s_448_7: bit-extract s_448_2 s_448_0 s_448_6
        let s_448_7: Bits = (Bits::new(
            ((s_448_2) >> (s_448_0)).value(),
            u16::try_from(s_448_6).unwrap(),
        ));
        // D s_448_8: cast reint s_448_7 -> u8
        let s_448_8: u8 = (s_448_7.value() as u8);
        // D s_448_9: cast zx s_448_8 -> bv
        let s_448_9: Bits = Bits::new(s_448_8 as u128, 6u16);
        // C s_448_10: const #0u : u8
        let s_448_10: u8 = 0;
        // C s_448_11: cast zx s_448_10 -> bv
        let s_448_11: Bits = Bits::new(s_448_10 as u128, 6u16);
        // D s_448_12: cmp-eq s_448_9 s_448_11
        let s_448_12: bool = ((s_448_9) == (s_448_11));
        // D s_448_13: write-var gs#383573 <= s_448_12
        fn_state.gs_383573 = s_448_12;
        // N s_448_14: jump b418
        return block_418(state, tracer, fn_state);
    }
    fn block_449<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_449_0: const #991s : i
        let s_449_0: i128 = 991;
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
        // D s_449_4: write-var gs#383559 <= s_449_3
        fn_state.gs_383559 = s_449_3;
        // N s_449_5: jump b414
        return block_414(state, tracer, fn_state);
    }
    fn block_450<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_450_0: const #10s : i
        let s_450_0: i128 = 10;
        // D s_450_1: read-var u#26158:u32
        let s_450_1: u32 = fn_state.u_26158;
        // D s_450_2: cast zx s_450_1 -> bv
        let s_450_2: Bits = Bits::new(s_450_1 as u128, 32u16);
        // C s_450_3: const #1s : i64
        let s_450_3: i64 = 1;
        // C s_450_4: cast zx s_450_3 -> i
        let s_450_4: i128 = (i128::try_from(s_450_3).unwrap());
        // C s_450_5: const #5s : i
        let s_450_5: i128 = 5;
        // C s_450_6: add s_450_5 s_450_4
        let s_450_6: i128 = (s_450_5 + s_450_4);
        // D s_450_7: bit-extract s_450_2 s_450_0 s_450_6
        let s_450_7: Bits = (Bits::new(
            ((s_450_2) >> (s_450_0)).value(),
            u16::try_from(s_450_6).unwrap(),
        ));
        // D s_450_8: cast reint s_450_7 -> u8
        let s_450_8: u8 = (s_450_7.value() as u8);
        // D s_450_9: cast zx s_450_8 -> bv
        let s_450_9: Bits = Bits::new(s_450_8 as u128, 6u16);
        // C s_450_10: const #0u : u8
        let s_450_10: u8 = 0;
        // C s_450_11: cast zx s_450_10 -> bv
        let s_450_11: Bits = Bits::new(s_450_10 as u128, 6u16);
        // D s_450_12: cmp-eq s_450_9 s_450_11
        let s_450_12: bool = ((s_450_9) == (s_450_11));
        // D s_450_13: write-var gs#383557 <= s_450_12
        fn_state.gs_383557 = s_450_12;
        // N s_450_14: jump b412
        return block_412(state, tracer, fn_state);
    }
    fn block_451<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_451_0: const #862s : i
        let s_451_0: i128 = 862;
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
        // D s_451_4: write-var gs#383525 <= s_451_3
        fn_state.gs_383525 = s_451_3;
        // N s_451_5: jump b394
        return block_394(state, tracer, fn_state);
    }
    fn block_452<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_452_0: const #10s : i
        let s_452_0: i128 = 10;
        // D s_452_1: read-var u#26151:u32
        let s_452_1: u32 = fn_state.u_26151;
        // D s_452_2: cast zx s_452_1 -> bv
        let s_452_2: Bits = Bits::new(s_452_1 as u128, 32u16);
        // C s_452_3: const #1s : i64
        let s_452_3: i64 = 1;
        // C s_452_4: cast zx s_452_3 -> i
        let s_452_4: i128 = (i128::try_from(s_452_3).unwrap());
        // C s_452_5: const #5s : i
        let s_452_5: i128 = 5;
        // C s_452_6: add s_452_5 s_452_4
        let s_452_6: i128 = (s_452_5 + s_452_4);
        // D s_452_7: bit-extract s_452_2 s_452_0 s_452_6
        let s_452_7: Bits = (Bits::new(
            ((s_452_2) >> (s_452_0)).value(),
            u16::try_from(s_452_6).unwrap(),
        ));
        // D s_452_8: cast reint s_452_7 -> u8
        let s_452_8: u8 = (s_452_7.value() as u8);
        // D s_452_9: cast zx s_452_8 -> bv
        let s_452_9: Bits = Bits::new(s_452_8 as u128, 6u16);
        // C s_452_10: const #31u : u8
        let s_452_10: u8 = 31;
        // C s_452_11: cast zx s_452_10 -> bv
        let s_452_11: Bits = Bits::new(s_452_10 as u128, 6u16);
        // D s_452_12: cmp-eq s_452_9 s_452_11
        let s_452_12: bool = ((s_452_9) == (s_452_11));
        // D s_452_13: write-var gs#383523 <= s_452_12
        fn_state.gs_383523 = s_452_12;
        // N s_452_14: jump b392
        return block_392(state, tracer, fn_state);
    }
    fn block_453<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_453_0: const #861s : i
        let s_453_0: i128 = 861;
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
        // D s_453_4: write-var gs#383491 <= s_453_3
        fn_state.gs_383491 = s_453_3;
        // N s_453_5: jump b374
        return block_374(state, tracer, fn_state);
    }
    fn block_454<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_454_0: const #10s : i
        let s_454_0: i128 = 10;
        // D s_454_1: read-var u#26144:u32
        let s_454_1: u32 = fn_state.u_26144;
        // D s_454_2: cast zx s_454_1 -> bv
        let s_454_2: Bits = Bits::new(s_454_1 as u128, 32u16);
        // C s_454_3: const #1s : i64
        let s_454_3: i64 = 1;
        // C s_454_4: cast zx s_454_3 -> i
        let s_454_4: i128 = (i128::try_from(s_454_3).unwrap());
        // C s_454_5: const #5s : i
        let s_454_5: i128 = 5;
        // C s_454_6: add s_454_5 s_454_4
        let s_454_6: i128 = (s_454_5 + s_454_4);
        // D s_454_7: bit-extract s_454_2 s_454_0 s_454_6
        let s_454_7: Bits = (Bits::new(
            ((s_454_2) >> (s_454_0)).value(),
            u16::try_from(s_454_6).unwrap(),
        ));
        // D s_454_8: cast reint s_454_7 -> u8
        let s_454_8: u8 = (s_454_7.value() as u8);
        // D s_454_9: cast zx s_454_8 -> bv
        let s_454_9: Bits = Bits::new(s_454_8 as u128, 6u16);
        // C s_454_10: const #31u : u8
        let s_454_10: u8 = 31;
        // C s_454_11: cast zx s_454_10 -> bv
        let s_454_11: Bits = Bits::new(s_454_10 as u128, 6u16);
        // D s_454_12: cmp-eq s_454_9 s_454_11
        let s_454_12: bool = ((s_454_9) == (s_454_11));
        // D s_454_13: write-var gs#383489 <= s_454_12
        fn_state.gs_383489 = s_454_12;
        // N s_454_14: jump b372
        return block_372(state, tracer, fn_state);
    }
    fn block_455<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_455_0: const #848s : i
        let s_455_0: i128 = 848;
        // C s_455_1: const #14696u : u32
        let s_455_1: u32 = 14696;
        // D s_455_2: read-reg s_455_1:i
        let s_455_2: i128 = {
            let value = state.read_register::<i128>(s_455_1 as isize);
            tracer.read_register(s_455_1 as isize, value);
            value
        };
        // D s_455_3: cmp-lt s_455_2 s_455_0
        let s_455_3: bool = ((s_455_2) < (s_455_0));
        // D s_455_4: write-var gs#383473 <= s_455_3
        fn_state.gs_383473 = s_455_3;
        // N s_455_5: jump b368
        return block_368(state, tracer, fn_state);
    }
    fn block_456<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_456_0: const #10s : i
        let s_456_0: i128 = 10;
        // D s_456_1: read-var u#26138:u32
        let s_456_1: u32 = fn_state.u_26138;
        // D s_456_2: cast zx s_456_1 -> bv
        let s_456_2: Bits = Bits::new(s_456_1 as u128, 32u16);
        // C s_456_3: const #1s : i64
        let s_456_3: i64 = 1;
        // C s_456_4: cast zx s_456_3 -> i
        let s_456_4: i128 = (i128::try_from(s_456_3).unwrap());
        // C s_456_5: const #5s : i
        let s_456_5: i128 = 5;
        // C s_456_6: add s_456_5 s_456_4
        let s_456_6: i128 = (s_456_5 + s_456_4);
        // D s_456_7: bit-extract s_456_2 s_456_0 s_456_6
        let s_456_7: Bits = (Bits::new(
            ((s_456_2) >> (s_456_0)).value(),
            u16::try_from(s_456_6).unwrap(),
        ));
        // D s_456_8: cast reint s_456_7 -> u8
        let s_456_8: u8 = (s_456_7.value() as u8);
        // D s_456_9: cast zx s_456_8 -> bv
        let s_456_9: Bits = Bits::new(s_456_8 as u128, 6u16);
        // C s_456_10: const #26u : u8
        let s_456_10: u8 = 26;
        // C s_456_11: cast zx s_456_10 -> bv
        let s_456_11: Bits = Bits::new(s_456_10 as u128, 6u16);
        // D s_456_12: cmp-eq s_456_9 s_456_11
        let s_456_12: bool = ((s_456_9) == (s_456_11));
        // D s_456_13: write-var gs#383471 <= s_456_12
        fn_state.gs_383471 = s_456_12;
        // N s_456_14: jump b366
        return block_366(state, tracer, fn_state);
    }
    fn block_457<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_457_0: const #833s : i
        let s_457_0: i128 = 833;
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
        // D s_457_4: write-var gs#383455 <= s_457_3
        fn_state.gs_383455 = s_457_3;
        // N s_457_5: jump b362
        return block_362(state, tracer, fn_state);
    }
    fn block_458<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_458_0: const #10s : i
        let s_458_0: i128 = 10;
        // D s_458_1: read-var u#26132:u32
        let s_458_1: u32 = fn_state.u_26132;
        // D s_458_2: cast zx s_458_1 -> bv
        let s_458_2: Bits = Bits::new(s_458_1 as u128, 32u16);
        // C s_458_3: const #1s : i64
        let s_458_3: i64 = 1;
        // C s_458_4: cast zx s_458_3 -> i
        let s_458_4: i128 = (i128::try_from(s_458_3).unwrap());
        // C s_458_5: const #5s : i
        let s_458_5: i128 = 5;
        // C s_458_6: add s_458_5 s_458_4
        let s_458_6: i128 = (s_458_5 + s_458_4);
        // D s_458_7: bit-extract s_458_2 s_458_0 s_458_6
        let s_458_7: Bits = (Bits::new(
            ((s_458_2) >> (s_458_0)).value(),
            u16::try_from(s_458_6).unwrap(),
        ));
        // D s_458_8: cast reint s_458_7 -> u8
        let s_458_8: u8 = (s_458_7.value() as u8);
        // D s_458_9: cast zx s_458_8 -> bv
        let s_458_9: Bits = Bits::new(s_458_8 as u128, 6u16);
        // C s_458_10: const #24u : u8
        let s_458_10: u8 = 24;
        // C s_458_11: cast zx s_458_10 -> bv
        let s_458_11: Bits = Bits::new(s_458_10 as u128, 6u16);
        // D s_458_12: cmp-eq s_458_9 s_458_11
        let s_458_12: bool = ((s_458_9) == (s_458_11));
        // D s_458_13: write-var gs#383453 <= s_458_12
        fn_state.gs_383453 = s_458_12;
        // N s_458_14: jump b360
        return block_360(state, tracer, fn_state);
    }
    fn block_459<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_459_0: const #831s : i
        let s_459_0: i128 = 831;
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
        // D s_459_4: write-var gs#383433 <= s_459_3
        fn_state.gs_383433 = s_459_3;
        // N s_459_5: jump b356
        return block_356(state, tracer, fn_state);
    }
    fn block_460<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_460_0: const #15s : i
        let s_460_0: i128 = 15;
        // D s_460_1: read-var u#26124:u32
        let s_460_1: u32 = fn_state.u_26124;
        // D s_460_2: cast zx s_460_1 -> bv
        let s_460_2: Bits = Bits::new(s_460_1 as u128, 32u16);
        // C s_460_3: const #1s : i64
        let s_460_3: i64 = 1;
        // C s_460_4: cast zx s_460_3 -> i
        let s_460_4: i128 = (i128::try_from(s_460_3).unwrap());
        // C s_460_5: const #0s : i
        let s_460_5: i128 = 0;
        // C s_460_6: add s_460_5 s_460_4
        let s_460_6: i128 = (s_460_5 + s_460_4);
        // D s_460_7: bit-extract s_460_2 s_460_0 s_460_6
        let s_460_7: Bits = (Bits::new(
            ((s_460_2) >> (s_460_0)).value(),
            u16::try_from(s_460_6).unwrap(),
        ));
        // D s_460_8: cast reint s_460_7 -> u8
        let s_460_8: bool = ((s_460_7.value()) != 0);
        // D s_460_9: cast zx s_460_8 -> bv
        let s_460_9: Bits = Bits::new(s_460_8 as u128, 1u16);
        // C s_460_10: const #1u : u8
        let s_460_10: bool = true;
        // C s_460_11: cast zx s_460_10 -> bv
        let s_460_11: Bits = Bits::new(s_460_10 as u128, 1u16);
        // D s_460_12: cmp-eq s_460_9 s_460_11
        let s_460_12: bool = ((s_460_9) == (s_460_11));
        // D s_460_13: write-var gs#383431 <= s_460_12
        fn_state.gs_383431 = s_460_12;
        // N s_460_14: jump b354
        return block_354(state, tracer, fn_state);
    }
    fn block_461<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_461_0: const #830s : i
        let s_461_0: i128 = 830;
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
        // D s_461_4: write-var gs#383411 <= s_461_3
        fn_state.gs_383411 = s_461_3;
        // N s_461_5: jump b350
        return block_350(state, tracer, fn_state);
    }
    fn block_462<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_462_0: const #15s : i
        let s_462_0: i128 = 15;
        // D s_462_1: read-var u#26116:u32
        let s_462_1: u32 = fn_state.u_26116;
        // D s_462_2: cast zx s_462_1 -> bv
        let s_462_2: Bits = Bits::new(s_462_1 as u128, 32u16);
        // C s_462_3: const #1s : i64
        let s_462_3: i64 = 1;
        // C s_462_4: cast zx s_462_3 -> i
        let s_462_4: i128 = (i128::try_from(s_462_3).unwrap());
        // C s_462_5: const #0s : i
        let s_462_5: i128 = 0;
        // C s_462_6: add s_462_5 s_462_4
        let s_462_6: i128 = (s_462_5 + s_462_4);
        // D s_462_7: bit-extract s_462_2 s_462_0 s_462_6
        let s_462_7: Bits = (Bits::new(
            ((s_462_2) >> (s_462_0)).value(),
            u16::try_from(s_462_6).unwrap(),
        ));
        // D s_462_8: cast reint s_462_7 -> u8
        let s_462_8: bool = ((s_462_7.value()) != 0);
        // D s_462_9: cast zx s_462_8 -> bv
        let s_462_9: Bits = Bits::new(s_462_8 as u128, 1u16);
        // C s_462_10: const #0u : u8
        let s_462_10: bool = false;
        // C s_462_11: cast zx s_462_10 -> bv
        let s_462_11: Bits = Bits::new(s_462_10 as u128, 1u16);
        // D s_462_12: cmp-eq s_462_9 s_462_11
        let s_462_12: bool = ((s_462_9) == (s_462_11));
        // D s_462_13: write-var gs#383409 <= s_462_12
        fn_state.gs_383409 = s_462_12;
        // N s_462_14: jump b348
        return block_348(state, tracer, fn_state);
    }
    fn block_463<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_463_0: const #829s : i
        let s_463_0: i128 = 829;
        // C s_463_1: const #14696u : u32
        let s_463_1: u32 = 14696;
        // D s_463_2: read-reg s_463_1:i
        let s_463_2: i128 = {
            let value = state.read_register::<i128>(s_463_1 as isize);
            tracer.read_register(s_463_1 as isize, value);
            value
        };
        // D s_463_3: cmp-lt s_463_2 s_463_0
        let s_463_3: bool = ((s_463_2) < (s_463_0));
        // D s_463_4: write-var gs#383389 <= s_463_3
        fn_state.gs_383389 = s_463_3;
        // N s_463_5: jump b344
        return block_344(state, tracer, fn_state);
    }
    fn block_464<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_464_0: const #15s : i
        let s_464_0: i128 = 15;
        // D s_464_1: read-var u#26108:u32
        let s_464_1: u32 = fn_state.u_26108;
        // D s_464_2: cast zx s_464_1 -> bv
        let s_464_2: Bits = Bits::new(s_464_1 as u128, 32u16);
        // C s_464_3: const #1s : i64
        let s_464_3: i64 = 1;
        // C s_464_4: cast zx s_464_3 -> i
        let s_464_4: i128 = (i128::try_from(s_464_3).unwrap());
        // C s_464_5: const #0s : i
        let s_464_5: i128 = 0;
        // C s_464_6: add s_464_5 s_464_4
        let s_464_6: i128 = (s_464_5 + s_464_4);
        // D s_464_7: bit-extract s_464_2 s_464_0 s_464_6
        let s_464_7: Bits = (Bits::new(
            ((s_464_2) >> (s_464_0)).value(),
            u16::try_from(s_464_6).unwrap(),
        ));
        // D s_464_8: cast reint s_464_7 -> u8
        let s_464_8: bool = ((s_464_7.value()) != 0);
        // D s_464_9: cast zx s_464_8 -> bv
        let s_464_9: Bits = Bits::new(s_464_8 as u128, 1u16);
        // C s_464_10: const #1u : u8
        let s_464_10: bool = true;
        // C s_464_11: cast zx s_464_10 -> bv
        let s_464_11: Bits = Bits::new(s_464_10 as u128, 1u16);
        // D s_464_12: cmp-eq s_464_9 s_464_11
        let s_464_12: bool = ((s_464_9) == (s_464_11));
        // D s_464_13: write-var gs#383387 <= s_464_12
        fn_state.gs_383387 = s_464_12;
        // N s_464_14: jump b342
        return block_342(state, tracer, fn_state);
    }
    fn block_465<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_465_0: const #828s : i
        let s_465_0: i128 = 828;
        // C s_465_1: const #14696u : u32
        let s_465_1: u32 = 14696;
        // D s_465_2: read-reg s_465_1:i
        let s_465_2: i128 = {
            let value = state.read_register::<i128>(s_465_1 as isize);
            tracer.read_register(s_465_1 as isize, value);
            value
        };
        // D s_465_3: cmp-lt s_465_2 s_465_0
        let s_465_3: bool = ((s_465_2) < (s_465_0));
        // D s_465_4: write-var gs#383367 <= s_465_3
        fn_state.gs_383367 = s_465_3;
        // N s_465_5: jump b338
        return block_338(state, tracer, fn_state);
    }
    fn block_466<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_466_0: const #15s : i
        let s_466_0: i128 = 15;
        // D s_466_1: read-var u#26101:u32
        let s_466_1: u32 = fn_state.u_26101;
        // D s_466_2: cast zx s_466_1 -> bv
        let s_466_2: Bits = Bits::new(s_466_1 as u128, 32u16);
        // C s_466_3: const #1s : i64
        let s_466_3: i64 = 1;
        // C s_466_4: cast zx s_466_3 -> i
        let s_466_4: i128 = (i128::try_from(s_466_3).unwrap());
        // C s_466_5: const #0s : i
        let s_466_5: i128 = 0;
        // C s_466_6: add s_466_5 s_466_4
        let s_466_6: i128 = (s_466_5 + s_466_4);
        // D s_466_7: bit-extract s_466_2 s_466_0 s_466_6
        let s_466_7: Bits = (Bits::new(
            ((s_466_2) >> (s_466_0)).value(),
            u16::try_from(s_466_6).unwrap(),
        ));
        // D s_466_8: cast reint s_466_7 -> u8
        let s_466_8: bool = ((s_466_7.value()) != 0);
        // D s_466_9: cast zx s_466_8 -> bv
        let s_466_9: Bits = Bits::new(s_466_8 as u128, 1u16);
        // C s_466_10: const #0u : u8
        let s_466_10: bool = false;
        // C s_466_11: cast zx s_466_10 -> bv
        let s_466_11: Bits = Bits::new(s_466_10 as u128, 1u16);
        // D s_466_12: cmp-eq s_466_9 s_466_11
        let s_466_12: bool = ((s_466_9) == (s_466_11));
        // D s_466_13: write-var gs#383365 <= s_466_12
        fn_state.gs_383365 = s_466_12;
        // N s_466_14: jump b336
        return block_336(state, tracer, fn_state);
    }
    fn block_467<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_467_0: const #791s : i
        let s_467_0: i128 = 791;
        // C s_467_1: const #14696u : u32
        let s_467_1: u32 = 14696;
        // D s_467_2: read-reg s_467_1:i
        let s_467_2: i128 = {
            let value = state.read_register::<i128>(s_467_1 as isize);
            tracer.read_register(s_467_1 as isize, value);
            value
        };
        // D s_467_3: cmp-lt s_467_2 s_467_0
        let s_467_3: bool = ((s_467_2) < (s_467_0));
        // D s_467_4: write-var gs#383351 <= s_467_3
        fn_state.gs_383351 = s_467_3;
        // N s_467_5: jump b332
        return block_332(state, tracer, fn_state);
    }
    fn block_468<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_468_0: const #10s : i
        let s_468_0: i128 = 10;
        // D s_468_1: read-var u#26096:u32
        let s_468_1: u32 = fn_state.u_26096;
        // D s_468_2: cast zx s_468_1 -> bv
        let s_468_2: Bits = Bits::new(s_468_1 as u128, 32u16);
        // C s_468_3: const #1s : i64
        let s_468_3: i64 = 1;
        // C s_468_4: cast zx s_468_3 -> i
        let s_468_4: i128 = (i128::try_from(s_468_3).unwrap());
        // C s_468_5: const #3s : i
        let s_468_5: i128 = 3;
        // C s_468_6: add s_468_5 s_468_4
        let s_468_6: i128 = (s_468_5 + s_468_4);
        // D s_468_7: bit-extract s_468_2 s_468_0 s_468_6
        let s_468_7: Bits = (Bits::new(
            ((s_468_2) >> (s_468_0)).value(),
            u16::try_from(s_468_6).unwrap(),
        ));
        // D s_468_8: cast reint s_468_7 -> u8
        let s_468_8: u8 = (s_468_7.value() as u8);
        // D s_468_9: cast zx s_468_8 -> bv
        let s_468_9: Bits = Bits::new(s_468_8 as u128, 4u16);
        // C s_468_10: const #2u : u8
        let s_468_10: u8 = 2;
        // C s_468_11: cast zx s_468_10 -> bv
        let s_468_11: Bits = Bits::new(s_468_10 as u128, 4u16);
        // D s_468_12: cmp-eq s_468_9 s_468_11
        let s_468_12: bool = ((s_468_9) == (s_468_11));
        // N s_468_13: branch s_468_12 b471 b469
        if s_468_12 {
            return block_471(state, tracer, fn_state);
        } else {
            return block_469(state, tracer, fn_state);
        };
    }
    fn block_469<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_469_0: const #0u : u8
        let s_469_0: bool = false;
        // D s_469_1: write-var gs#383348 <= s_469_0
        fn_state.gs_383348 = s_469_0;
        // N s_469_2: jump b470
        return block_470(state, tracer, fn_state);
    }
    fn block_470<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_470_0: read-var gs#383348:u8
        let s_470_0: bool = fn_state.gs_383348;
        // D s_470_1: write-var gs#383349 <= s_470_0
        fn_state.gs_383349 = s_470_0;
        // N s_470_2: jump b330
        return block_330(state, tracer, fn_state);
    }
    fn block_471<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_471_0: const #0s : i
        let s_471_0: i128 = 0;
        // D s_471_1: read-var u#26096:u32
        let s_471_1: u32 = fn_state.u_26096;
        // D s_471_2: cast zx s_471_1 -> bv
        let s_471_2: Bits = Bits::new(s_471_1 as u128, 32u16);
        // C s_471_3: const #1s : i64
        let s_471_3: i64 = 1;
        // C s_471_4: cast zx s_471_3 -> i
        let s_471_4: i128 = (i128::try_from(s_471_3).unwrap());
        // C s_471_5: const #4s : i
        let s_471_5: i128 = 4;
        // C s_471_6: add s_471_5 s_471_4
        let s_471_6: i128 = (s_471_5 + s_471_4);
        // D s_471_7: bit-extract s_471_2 s_471_0 s_471_6
        let s_471_7: Bits = (Bits::new(
            ((s_471_2) >> (s_471_0)).value(),
            u16::try_from(s_471_6).unwrap(),
        ));
        // D s_471_8: cast reint s_471_7 -> u8
        let s_471_8: u8 = (s_471_7.value() as u8);
        // D s_471_9: cast zx s_471_8 -> bv
        let s_471_9: Bits = Bits::new(s_471_8 as u128, 5u16);
        // C s_471_10: const #13u : u8
        let s_471_10: u8 = 13;
        // C s_471_11: cast zx s_471_10 -> bv
        let s_471_11: Bits = Bits::new(s_471_10 as u128, 5u16);
        // D s_471_12: cmp-eq s_471_9 s_471_11
        let s_471_12: bool = ((s_471_9) == (s_471_11));
        // D s_471_13: write-var gs#383348 <= s_471_12
        fn_state.gs_383348 = s_471_12;
        // N s_471_14: jump b470
        return block_470(state, tracer, fn_state);
    }
    fn block_472<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_472_0: const #782s : i
        let s_472_0: i128 = 782;
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
        // D s_472_4: write-var gs#383328 <= s_472_3
        fn_state.gs_383328 = s_472_3;
        // N s_472_5: jump b326
        return block_326(state, tracer, fn_state);
    }
    fn block_473<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_473_0: const #10s : i
        let s_473_0: i128 = 10;
        // D s_473_1: read-var u#26089:u32
        let s_473_1: u32 = fn_state.u_26089;
        // D s_473_2: cast zx s_473_1 -> bv
        let s_473_2: Bits = Bits::new(s_473_1 as u128, 32u16);
        // C s_473_3: const #1s : i64
        let s_473_3: i64 = 1;
        // C s_473_4: cast zx s_473_3 -> i
        let s_473_4: i128 = (i128::try_from(s_473_3).unwrap());
        // C s_473_5: const #5s : i
        let s_473_5: i128 = 5;
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
        let s_473_9: Bits = Bits::new(s_473_8 as u128, 6u16);
        // C s_473_10: const #2u : u8
        let s_473_10: u8 = 2;
        // C s_473_11: cast zx s_473_10 -> bv
        let s_473_11: Bits = Bits::new(s_473_10 as u128, 6u16);
        // D s_473_12: cmp-eq s_473_9 s_473_11
        let s_473_12: bool = ((s_473_9) == (s_473_11));
        // D s_473_13: write-var gs#383326 <= s_473_12
        fn_state.gs_383326 = s_473_12;
        // N s_473_14: jump b324
        return block_324(state, tracer, fn_state);
    }
    fn block_474<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_474_0: const #781s : i
        let s_474_0: i128 = 781;
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
        // D s_474_4: write-var gs#383308 <= s_474_3
        fn_state.gs_383308 = s_474_3;
        // N s_474_5: jump b320
        return block_320(state, tracer, fn_state);
    }
    fn block_475<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_475_0: const #10s : i
        let s_475_0: i128 = 10;
        // D s_475_1: read-var u#26083:u32
        let s_475_1: u32 = fn_state.u_26083;
        // D s_475_2: cast zx s_475_1 -> bv
        let s_475_2: Bits = Bits::new(s_475_1 as u128, 32u16);
        // C s_475_3: const #1s : i64
        let s_475_3: i64 = 1;
        // C s_475_4: cast zx s_475_3 -> i
        let s_475_4: i128 = (i128::try_from(s_475_3).unwrap());
        // C s_475_5: const #5s : i
        let s_475_5: i128 = 5;
        // C s_475_6: add s_475_5 s_475_4
        let s_475_6: i128 = (s_475_5 + s_475_4);
        // D s_475_7: bit-extract s_475_2 s_475_0 s_475_6
        let s_475_7: Bits = (Bits::new(
            ((s_475_2) >> (s_475_0)).value(),
            u16::try_from(s_475_6).unwrap(),
        ));
        // D s_475_8: cast reint s_475_7 -> u8
        let s_475_8: u8 = (s_475_7.value() as u8);
        // D s_475_9: cast zx s_475_8 -> bv
        let s_475_9: Bits = Bits::new(s_475_8 as u128, 6u16);
        // C s_475_10: const #3u : u8
        let s_475_10: u8 = 3;
        // C s_475_11: cast zx s_475_10 -> bv
        let s_475_11: Bits = Bits::new(s_475_10 as u128, 6u16);
        // D s_475_12: cmp-eq s_475_9 s_475_11
        let s_475_12: bool = ((s_475_9) == (s_475_11));
        // D s_475_13: write-var gs#383306 <= s_475_12
        fn_state.gs_383306 = s_475_12;
        // N s_475_14: jump b318
        return block_318(state, tracer, fn_state);
    }
    fn block_476<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_476_0: const #742s : i
        let s_476_0: i128 = 742;
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
        // D s_476_4: write-var gs#383290 <= s_476_3
        fn_state.gs_383290 = s_476_3;
        // N s_476_5: jump b314
        return block_314(state, tracer, fn_state);
    }
    fn block_477<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_477_0: const #10s : i
        let s_477_0: i128 = 10;
        // D s_477_1: read-var u#26078:u32
        let s_477_1: u32 = fn_state.u_26078;
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
        // C s_477_10: const #1u : u8
        let s_477_10: u8 = 1;
        // C s_477_11: cast zx s_477_10 -> bv
        let s_477_11: Bits = Bits::new(s_477_10 as u128, 5u16);
        // D s_477_12: cmp-eq s_477_9 s_477_11
        let s_477_12: bool = ((s_477_9) == (s_477_11));
        // N s_477_13: branch s_477_12 b480 b478
        if s_477_12 {
            return block_480(state, tracer, fn_state);
        } else {
            return block_478(state, tracer, fn_state);
        };
    }
    fn block_478<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_478_0: const #0u : u8
        let s_478_0: bool = false;
        // D s_478_1: write-var gs#383287 <= s_478_0
        fn_state.gs_383287 = s_478_0;
        // N s_478_2: jump b479
        return block_479(state, tracer, fn_state);
    }
    fn block_479<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_479_0: read-var gs#383287:u8
        let s_479_0: bool = fn_state.gs_383287;
        // D s_479_1: write-var gs#383288 <= s_479_0
        fn_state.gs_383288 = s_479_0;
        // N s_479_2: jump b312
        return block_312(state, tracer, fn_state);
    }
    fn block_480<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_480_0: const #4s : i
        let s_480_0: i128 = 4;
        // D s_480_1: read-var u#26078:u32
        let s_480_1: u32 = fn_state.u_26078;
        // D s_480_2: cast zx s_480_1 -> bv
        let s_480_2: Bits = Bits::new(s_480_1 as u128, 32u16);
        // C s_480_3: const #1s : i64
        let s_480_3: i64 = 1;
        // C s_480_4: cast zx s_480_3 -> i
        let s_480_4: i128 = (i128::try_from(s_480_3).unwrap());
        // C s_480_5: const #0s : i
        let s_480_5: i128 = 0;
        // C s_480_6: add s_480_5 s_480_4
        let s_480_6: i128 = (s_480_5 + s_480_4);
        // D s_480_7: bit-extract s_480_2 s_480_0 s_480_6
        let s_480_7: Bits = (Bits::new(
            ((s_480_2) >> (s_480_0)).value(),
            u16::try_from(s_480_6).unwrap(),
        ));
        // D s_480_8: cast reint s_480_7 -> u8
        let s_480_8: bool = ((s_480_7.value()) != 0);
        // D s_480_9: cast zx s_480_8 -> bv
        let s_480_9: Bits = Bits::new(s_480_8 as u128, 1u16);
        // C s_480_10: const #0u : u8
        let s_480_10: bool = false;
        // C s_480_11: cast zx s_480_10 -> bv
        let s_480_11: Bits = Bits::new(s_480_10 as u128, 1u16);
        // D s_480_12: cmp-eq s_480_9 s_480_11
        let s_480_12: bool = ((s_480_9) == (s_480_11));
        // D s_480_13: write-var gs#383287 <= s_480_12
        fn_state.gs_383287 = s_480_12;
        // N s_480_14: jump b479
        return block_479(state, tracer, fn_state);
    }
    fn block_481<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_481_0: const #738s : i
        let s_481_0: i128 = 738;
        // C s_481_1: const #14696u : u32
        let s_481_1: u32 = 14696;
        // D s_481_2: read-reg s_481_1:i
        let s_481_2: i128 = {
            let value = state.read_register::<i128>(s_481_1 as isize);
            tracer.read_register(s_481_1 as isize, value);
            value
        };
        // D s_481_3: cmp-lt s_481_2 s_481_0
        let s_481_3: bool = ((s_481_2) < (s_481_0));
        // D s_481_4: write-var gs#383269 <= s_481_3
        fn_state.gs_383269 = s_481_3;
        // N s_481_5: jump b308
        return block_308(state, tracer, fn_state);
    }
    fn block_482<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_482_0: const #737s : i
        let s_482_0: i128 = 737;
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
        // D s_482_4: write-var gs#383254 <= s_482_3
        fn_state.gs_383254 = s_482_3;
        // N s_482_5: jump b304
        return block_304(state, tracer, fn_state);
    }
    fn block_483<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_483_0: const #736s : i
        let s_483_0: i128 = 736;
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
        // D s_483_4: write-var gs#383239 <= s_483_3
        fn_state.gs_383239 = s_483_3;
        // N s_483_5: jump b300
        return block_300(state, tracer, fn_state);
    }
    fn block_484<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_484_0: const #719s : i
        let s_484_0: i128 = 719;
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
        // D s_484_4: write-var gs#383226 <= s_484_3
        fn_state.gs_383226 = s_484_3;
        // N s_484_5: jump b296
        return block_296(state, tracer, fn_state);
    }
    fn block_485<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_485_0: const #713s : i
        let s_485_0: i128 = 713;
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
        // D s_485_4: write-var gs#383213 <= s_485_3
        fn_state.gs_383213 = s_485_3;
        // N s_485_5: jump b292
        return block_292(state, tracer, fn_state);
    }
    fn block_486<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_486_0: const #10s : i
        let s_486_0: i128 = 10;
        // D s_486_1: read-var u#26050:u32
        let s_486_1: u32 = fn_state.u_26050;
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
        // C s_486_10: const #1u : u8
        let s_486_10: u8 = 1;
        // C s_486_11: cast zx s_486_10 -> bv
        let s_486_11: Bits = Bits::new(s_486_10 as u128, 3u16);
        // D s_486_12: cmp-eq s_486_9 s_486_11
        let s_486_12: bool = ((s_486_9) == (s_486_11));
        // D s_486_13: write-var gs#383211 <= s_486_12
        fn_state.gs_383211 = s_486_12;
        // N s_486_14: jump b290
        return block_290(state, tracer, fn_state);
    }
    fn block_487<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_487_0: const #711s : i
        let s_487_0: i128 = 711;
        // C s_487_1: const #14696u : u32
        let s_487_1: u32 = 14696;
        // D s_487_2: read-reg s_487_1:i
        let s_487_2: i128 = {
            let value = state.read_register::<i128>(s_487_1 as isize);
            tracer.read_register(s_487_1 as isize, value);
            value
        };
        // D s_487_3: cmp-lt s_487_2 s_487_0
        let s_487_3: bool = ((s_487_2) < (s_487_0));
        // D s_487_4: write-var gs#383197 <= s_487_3
        fn_state.gs_383197 = s_487_3;
        // N s_487_5: jump b286
        return block_286(state, tracer, fn_state);
    }
    fn block_488<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_488_0: const #10s : i
        let s_488_0: i128 = 10;
        // D s_488_1: read-var u#26045:u32
        let s_488_1: u32 = fn_state.u_26045;
        // D s_488_2: cast zx s_488_1 -> bv
        let s_488_2: Bits = Bits::new(s_488_1 as u128, 32u16);
        // C s_488_3: const #1s : i64
        let s_488_3: i64 = 1;
        // C s_488_4: cast zx s_488_3 -> i
        let s_488_4: i128 = (i128::try_from(s_488_3).unwrap());
        // C s_488_5: const #2s : i
        let s_488_5: i128 = 2;
        // C s_488_6: add s_488_5 s_488_4
        let s_488_6: i128 = (s_488_5 + s_488_4);
        // D s_488_7: bit-extract s_488_2 s_488_0 s_488_6
        let s_488_7: Bits = (Bits::new(
            ((s_488_2) >> (s_488_0)).value(),
            u16::try_from(s_488_6).unwrap(),
        ));
        // D s_488_8: cast reint s_488_7 -> u8
        let s_488_8: u8 = (s_488_7.value() as u8);
        // D s_488_9: cast zx s_488_8 -> bv
        let s_488_9: Bits = Bits::new(s_488_8 as u128, 3u16);
        // C s_488_10: const #0u : u8
        let s_488_10: u8 = 0;
        // C s_488_11: cast zx s_488_10 -> bv
        let s_488_11: Bits = Bits::new(s_488_10 as u128, 3u16);
        // D s_488_12: cmp-eq s_488_9 s_488_11
        let s_488_12: bool = ((s_488_9) == (s_488_11));
        // D s_488_13: write-var gs#383195 <= s_488_12
        fn_state.gs_383195 = s_488_12;
        // N s_488_14: jump b284
        return block_284(state, tracer, fn_state);
    }
    fn block_489<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_489_0: const #710s : i
        let s_489_0: i128 = 710;
        // C s_489_1: const #14696u : u32
        let s_489_1: u32 = 14696;
        // D s_489_2: read-reg s_489_1:i
        let s_489_2: i128 = {
            let value = state.read_register::<i128>(s_489_1 as isize);
            tracer.read_register(s_489_1 as isize, value);
            value
        };
        // D s_489_3: cmp-lt s_489_2 s_489_0
        let s_489_3: bool = ((s_489_2) < (s_489_0));
        // D s_489_4: write-var gs#383181 <= s_489_3
        fn_state.gs_383181 = s_489_3;
        // N s_489_5: jump b280
        return block_280(state, tracer, fn_state);
    }
    fn block_490<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_490_0: const #10s : i
        let s_490_0: i128 = 10;
        // D s_490_1: read-var u#26040:u32
        let s_490_1: u32 = fn_state.u_26040;
        // D s_490_2: cast zx s_490_1 -> bv
        let s_490_2: Bits = Bits::new(s_490_1 as u128, 32u16);
        // C s_490_3: const #1s : i64
        let s_490_3: i64 = 1;
        // C s_490_4: cast zx s_490_3 -> i
        let s_490_4: i128 = (i128::try_from(s_490_3).unwrap());
        // C s_490_5: const #5s : i
        let s_490_5: i128 = 5;
        // C s_490_6: add s_490_5 s_490_4
        let s_490_6: i128 = (s_490_5 + s_490_4);
        // D s_490_7: bit-extract s_490_2 s_490_0 s_490_6
        let s_490_7: Bits = (Bits::new(
            ((s_490_2) >> (s_490_0)).value(),
            u16::try_from(s_490_6).unwrap(),
        ));
        // D s_490_8: cast reint s_490_7 -> u8
        let s_490_8: u8 = (s_490_7.value() as u8);
        // D s_490_9: cast zx s_490_8 -> bv
        let s_490_9: Bits = Bits::new(s_490_8 as u128, 6u16);
        // C s_490_10: const #12u : u8
        let s_490_10: u8 = 12;
        // C s_490_11: cast zx s_490_10 -> bv
        let s_490_11: Bits = Bits::new(s_490_10 as u128, 6u16);
        // D s_490_12: cmp-eq s_490_9 s_490_11
        let s_490_12: bool = ((s_490_9) == (s_490_11));
        // D s_490_13: write-var gs#383179 <= s_490_12
        fn_state.gs_383179 = s_490_12;
        // N s_490_14: jump b278
        return block_278(state, tracer, fn_state);
    }
    fn block_491<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_491_0: const #709s : i
        let s_491_0: i128 = 709;
        // C s_491_1: const #14696u : u32
        let s_491_1: u32 = 14696;
        // D s_491_2: read-reg s_491_1:i
        let s_491_2: i128 = {
            let value = state.read_register::<i128>(s_491_1 as isize);
            tracer.read_register(s_491_1 as isize, value);
            value
        };
        // D s_491_3: cmp-lt s_491_2 s_491_0
        let s_491_3: bool = ((s_491_2) < (s_491_0));
        // D s_491_4: write-var gs#383165 <= s_491_3
        fn_state.gs_383165 = s_491_3;
        // N s_491_5: jump b274
        return block_274(state, tracer, fn_state);
    }
    fn block_492<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_492_0: const #10s : i
        let s_492_0: i128 = 10;
        // D s_492_1: read-var u#26035:u32
        let s_492_1: u32 = fn_state.u_26035;
        // D s_492_2: cast zx s_492_1 -> bv
        let s_492_2: Bits = Bits::new(s_492_1 as u128, 32u16);
        // C s_492_3: const #1s : i64
        let s_492_3: i64 = 1;
        // C s_492_4: cast zx s_492_3 -> i
        let s_492_4: i128 = (i128::try_from(s_492_3).unwrap());
        // C s_492_5: const #2s : i
        let s_492_5: i128 = 2;
        // C s_492_6: add s_492_5 s_492_4
        let s_492_6: i128 = (s_492_5 + s_492_4);
        // D s_492_7: bit-extract s_492_2 s_492_0 s_492_6
        let s_492_7: Bits = (Bits::new(
            ((s_492_2) >> (s_492_0)).value(),
            u16::try_from(s_492_6).unwrap(),
        ));
        // D s_492_8: cast reint s_492_7 -> u8
        let s_492_8: u8 = (s_492_7.value() as u8);
        // D s_492_9: cast zx s_492_8 -> bv
        let s_492_9: Bits = Bits::new(s_492_8 as u128, 3u16);
        // C s_492_10: const #3u : u8
        let s_492_10: u8 = 3;
        // C s_492_11: cast zx s_492_10 -> bv
        let s_492_11: Bits = Bits::new(s_492_10 as u128, 3u16);
        // D s_492_12: cmp-eq s_492_9 s_492_11
        let s_492_12: bool = ((s_492_9) == (s_492_11));
        // D s_492_13: write-var gs#383163 <= s_492_12
        fn_state.gs_383163 = s_492_12;
        // N s_492_14: jump b272
        return block_272(state, tracer, fn_state);
    }
    fn block_493<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_493_0: const #708s : i
        let s_493_0: i128 = 708;
        // C s_493_1: const #14696u : u32
        let s_493_1: u32 = 14696;
        // D s_493_2: read-reg s_493_1:i
        let s_493_2: i128 = {
            let value = state.read_register::<i128>(s_493_1 as isize);
            tracer.read_register(s_493_1 as isize, value);
            value
        };
        // D s_493_3: cmp-lt s_493_2 s_493_0
        let s_493_3: bool = ((s_493_2) < (s_493_0));
        // D s_493_4: write-var gs#383149 <= s_493_3
        fn_state.gs_383149 = s_493_3;
        // N s_493_5: jump b268
        return block_268(state, tracer, fn_state);
    }
    fn block_494<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_494_0: const #10s : i
        let s_494_0: i128 = 10;
        // D s_494_1: read-var u#26030:u32
        let s_494_1: u32 = fn_state.u_26030;
        // D s_494_2: cast zx s_494_1 -> bv
        let s_494_2: Bits = Bits::new(s_494_1 as u128, 32u16);
        // C s_494_3: const #1s : i64
        let s_494_3: i64 = 1;
        // C s_494_4: cast zx s_494_3 -> i
        let s_494_4: i128 = (i128::try_from(s_494_3).unwrap());
        // C s_494_5: const #2s : i
        let s_494_5: i128 = 2;
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
        let s_494_9: Bits = Bits::new(s_494_8 as u128, 3u16);
        // C s_494_10: const #2u : u8
        let s_494_10: u8 = 2;
        // C s_494_11: cast zx s_494_10 -> bv
        let s_494_11: Bits = Bits::new(s_494_10 as u128, 3u16);
        // D s_494_12: cmp-eq s_494_9 s_494_11
        let s_494_12: bool = ((s_494_9) == (s_494_11));
        // D s_494_13: write-var gs#383147 <= s_494_12
        fn_state.gs_383147 = s_494_12;
        // N s_494_14: jump b266
        return block_266(state, tracer, fn_state);
    }
    fn block_495<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_495_0: const #691s : i
        let s_495_0: i128 = 691;
        // C s_495_1: const #14696u : u32
        let s_495_1: u32 = 14696;
        // D s_495_2: read-reg s_495_1:i
        let s_495_2: i128 = {
            let value = state.read_register::<i128>(s_495_1 as isize);
            tracer.read_register(s_495_1 as isize, value);
            value
        };
        // D s_495_3: cmp-lt s_495_2 s_495_0
        let s_495_3: bool = ((s_495_2) < (s_495_0));
        // D s_495_4: write-var gs#383127 <= s_495_3
        fn_state.gs_383127 = s_495_3;
        // N s_495_5: jump b262
        return block_262(state, tracer, fn_state);
    }
    fn block_496<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_496_0: const #15s : i
        let s_496_0: i128 = 15;
        // D s_496_1: read-var u#26022:u32
        let s_496_1: u32 = fn_state.u_26022;
        // D s_496_2: cast zx s_496_1 -> bv
        let s_496_2: Bits = Bits::new(s_496_1 as u128, 32u16);
        // C s_496_3: const #1s : i64
        let s_496_3: i64 = 1;
        // C s_496_4: cast zx s_496_3 -> i
        let s_496_4: i128 = (i128::try_from(s_496_3).unwrap());
        // C s_496_5: const #0s : i
        let s_496_5: i128 = 0;
        // C s_496_6: add s_496_5 s_496_4
        let s_496_6: i128 = (s_496_5 + s_496_4);
        // D s_496_7: bit-extract s_496_2 s_496_0 s_496_6
        let s_496_7: Bits = (Bits::new(
            ((s_496_2) >> (s_496_0)).value(),
            u16::try_from(s_496_6).unwrap(),
        ));
        // D s_496_8: cast reint s_496_7 -> u8
        let s_496_8: bool = ((s_496_7.value()) != 0);
        // D s_496_9: cast zx s_496_8 -> bv
        let s_496_9: Bits = Bits::new(s_496_8 as u128, 1u16);
        // C s_496_10: const #1u : u8
        let s_496_10: bool = true;
        // C s_496_11: cast zx s_496_10 -> bv
        let s_496_11: Bits = Bits::new(s_496_10 as u128, 1u16);
        // D s_496_12: cmp-eq s_496_9 s_496_11
        let s_496_12: bool = ((s_496_9) == (s_496_11));
        // D s_496_13: write-var gs#383125 <= s_496_12
        fn_state.gs_383125 = s_496_12;
        // N s_496_14: jump b260
        return block_260(state, tracer, fn_state);
    }
    fn block_497<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_497_0: const #690s : i
        let s_497_0: i128 = 690;
        // C s_497_1: const #14696u : u32
        let s_497_1: u32 = 14696;
        // D s_497_2: read-reg s_497_1:i
        let s_497_2: i128 = {
            let value = state.read_register::<i128>(s_497_1 as isize);
            tracer.read_register(s_497_1 as isize, value);
            value
        };
        // D s_497_3: cmp-lt s_497_2 s_497_0
        let s_497_3: bool = ((s_497_2) < (s_497_0));
        // D s_497_4: write-var gs#383105 <= s_497_3
        fn_state.gs_383105 = s_497_3;
        // N s_497_5: jump b256
        return block_256(state, tracer, fn_state);
    }
    fn block_498<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_498_0: const #15s : i
        let s_498_0: i128 = 15;
        // D s_498_1: read-var u#26016:u32
        let s_498_1: u32 = fn_state.u_26016;
        // D s_498_2: cast zx s_498_1 -> bv
        let s_498_2: Bits = Bits::new(s_498_1 as u128, 32u16);
        // C s_498_3: const #1s : i64
        let s_498_3: i64 = 1;
        // C s_498_4: cast zx s_498_3 -> i
        let s_498_4: i128 = (i128::try_from(s_498_3).unwrap());
        // C s_498_5: const #0s : i
        let s_498_5: i128 = 0;
        // C s_498_6: add s_498_5 s_498_4
        let s_498_6: i128 = (s_498_5 + s_498_4);
        // D s_498_7: bit-extract s_498_2 s_498_0 s_498_6
        let s_498_7: Bits = (Bits::new(
            ((s_498_2) >> (s_498_0)).value(),
            u16::try_from(s_498_6).unwrap(),
        ));
        // D s_498_8: cast reint s_498_7 -> u8
        let s_498_8: bool = ((s_498_7.value()) != 0);
        // D s_498_9: cast zx s_498_8 -> bv
        let s_498_9: Bits = Bits::new(s_498_8 as u128, 1u16);
        // C s_498_10: const #0u : u8
        let s_498_10: bool = false;
        // C s_498_11: cast zx s_498_10 -> bv
        let s_498_11: Bits = Bits::new(s_498_10 as u128, 1u16);
        // D s_498_12: cmp-eq s_498_9 s_498_11
        let s_498_12: bool = ((s_498_9) == (s_498_11));
        // D s_498_13: write-var gs#383103 <= s_498_12
        fn_state.gs_383103 = s_498_12;
        // N s_498_14: jump b254
        return block_254(state, tracer, fn_state);
    }
    fn block_499<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_499_0: const #482s : i
        let s_499_0: i128 = 482;
        // C s_499_1: const #14696u : u32
        let s_499_1: u32 = 14696;
        // D s_499_2: read-reg s_499_1:i
        let s_499_2: i128 = {
            let value = state.read_register::<i128>(s_499_1 as isize);
            tracer.read_register(s_499_1 as isize, value);
            value
        };
        // D s_499_3: cmp-lt s_499_2 s_499_0
        let s_499_3: bool = ((s_499_2) < (s_499_0));
        // D s_499_4: write-var gs#383089 <= s_499_3
        fn_state.gs_383089 = s_499_3;
        // N s_499_5: jump b250
        return block_250(state, tracer, fn_state);
    }
    fn block_500<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_500_0: const #10s : i
        let s_500_0: i128 = 10;
        // D s_500_1: read-var u#26011:u32
        let s_500_1: u32 = fn_state.u_26011;
        // D s_500_2: cast zx s_500_1 -> bv
        let s_500_2: Bits = Bits::new(s_500_1 as u128, 32u16);
        // C s_500_3: const #1s : i64
        let s_500_3: i64 = 1;
        // C s_500_4: cast zx s_500_3 -> i
        let s_500_4: i128 = (i128::try_from(s_500_3).unwrap());
        // C s_500_5: const #5s : i
        let s_500_5: i128 = 5;
        // C s_500_6: add s_500_5 s_500_4
        let s_500_6: i128 = (s_500_5 + s_500_4);
        // D s_500_7: bit-extract s_500_2 s_500_0 s_500_6
        let s_500_7: Bits = (Bits::new(
            ((s_500_2) >> (s_500_0)).value(),
            u16::try_from(s_500_6).unwrap(),
        ));
        // D s_500_8: cast reint s_500_7 -> u8
        let s_500_8: u8 = (s_500_7.value() as u8);
        // D s_500_9: cast zx s_500_8 -> bv
        let s_500_9: Bits = Bits::new(s_500_8 as u128, 6u16);
        // C s_500_10: const #4u : u8
        let s_500_10: u8 = 4;
        // C s_500_11: cast zx s_500_10 -> bv
        let s_500_11: Bits = Bits::new(s_500_10 as u128, 6u16);
        // D s_500_12: cmp-eq s_500_9 s_500_11
        let s_500_12: bool = ((s_500_9) == (s_500_11));
        // D s_500_13: write-var gs#383087 <= s_500_12
        fn_state.gs_383087 = s_500_12;
        // N s_500_14: jump b248
        return block_248(state, tracer, fn_state);
    }
    fn block_501<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_501_0: const #477s : i
        let s_501_0: i128 = 477;
        // C s_501_1: const #14696u : u32
        let s_501_1: u32 = 14696;
        // D s_501_2: read-reg s_501_1:i
        let s_501_2: i128 = {
            let value = state.read_register::<i128>(s_501_1 as isize);
            tracer.read_register(s_501_1 as isize, value);
            value
        };
        // D s_501_3: cmp-lt s_501_2 s_501_0
        let s_501_3: bool = ((s_501_2) < (s_501_0));
        // D s_501_4: write-var gs#383073 <= s_501_3
        fn_state.gs_383073 = s_501_3;
        // N s_501_5: jump b244
        return block_244(state, tracer, fn_state);
    }
    fn block_502<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_502_0: const #10s : i
        let s_502_0: i128 = 10;
        // D s_502_1: read-var u#26009:u32
        let s_502_1: u32 = fn_state.u_26009;
        // D s_502_2: cast zx s_502_1 -> bv
        let s_502_2: Bits = Bits::new(s_502_1 as u128, 32u16);
        // C s_502_3: const #1s : i64
        let s_502_3: i64 = 1;
        // C s_502_4: cast zx s_502_3 -> i
        let s_502_4: i128 = (i128::try_from(s_502_3).unwrap());
        // C s_502_5: const #5s : i
        let s_502_5: i128 = 5;
        // C s_502_6: add s_502_5 s_502_4
        let s_502_6: i128 = (s_502_5 + s_502_4);
        // D s_502_7: bit-extract s_502_2 s_502_0 s_502_6
        let s_502_7: Bits = (Bits::new(
            ((s_502_2) >> (s_502_0)).value(),
            u16::try_from(s_502_6).unwrap(),
        ));
        // D s_502_8: cast reint s_502_7 -> u8
        let s_502_8: u8 = (s_502_7.value() as u8);
        // D s_502_9: cast zx s_502_8 -> bv
        let s_502_9: Bits = Bits::new(s_502_8 as u128, 6u16);
        // C s_502_10: const #5u : u8
        let s_502_10: u8 = 5;
        // C s_502_11: cast zx s_502_10 -> bv
        let s_502_11: Bits = Bits::new(s_502_10 as u128, 6u16);
        // D s_502_12: cmp-eq s_502_9 s_502_11
        let s_502_12: bool = ((s_502_9) == (s_502_11));
        // D s_502_13: write-var gs#383071 <= s_502_12
        fn_state.gs_383071 = s_502_12;
        // N s_502_14: jump b242
        return block_242(state, tracer, fn_state);
    }
    fn block_503<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_503_0: const #191s : i
        let s_503_0: i128 = 191;
        // C s_503_1: const #14696u : u32
        let s_503_1: u32 = 14696;
        // D s_503_2: read-reg s_503_1:i
        let s_503_2: i128 = {
            let value = state.read_register::<i128>(s_503_1 as isize);
            tracer.read_register(s_503_1 as isize, value);
            value
        };
        // D s_503_3: cmp-lt s_503_2 s_503_0
        let s_503_3: bool = ((s_503_2) < (s_503_0));
        // D s_503_4: write-var gs#383057 <= s_503_3
        fn_state.gs_383057 = s_503_3;
        // N s_503_5: jump b238
        return block_238(state, tracer, fn_state);
    }
    fn block_504<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_504_0: const #190s : i
        let s_504_0: i128 = 190;
        // C s_504_1: const #14696u : u32
        let s_504_1: u32 = 14696;
        // D s_504_2: read-reg s_504_1:i
        let s_504_2: i128 = {
            let value = state.read_register::<i128>(s_504_1 as isize);
            tracer.read_register(s_504_1 as isize, value);
            value
        };
        // D s_504_3: cmp-lt s_504_2 s_504_0
        let s_504_3: bool = ((s_504_2) < (s_504_0));
        // D s_504_4: write-var gs#383036 <= s_504_3
        fn_state.gs_383036 = s_504_3;
        // N s_504_5: jump b234
        return block_234(state, tracer, fn_state);
    }
    fn block_505<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_505_0: const #10s : i
        let s_505_0: i128 = 10;
        // D s_505_1: read-var u#25995:u32
        let s_505_1: u32 = fn_state.u_25995;
        // D s_505_2: cast zx s_505_1 -> bv
        let s_505_2: Bits = Bits::new(s_505_1 as u128, 32u16);
        // C s_505_3: const #1s : i64
        let s_505_3: i64 = 1;
        // C s_505_4: cast zx s_505_3 -> i
        let s_505_4: i128 = (i128::try_from(s_505_3).unwrap());
        // C s_505_5: const #1s : i
        let s_505_5: i128 = 1;
        // C s_505_6: add s_505_5 s_505_4
        let s_505_6: i128 = (s_505_5 + s_505_4);
        // D s_505_7: bit-extract s_505_2 s_505_0 s_505_6
        let s_505_7: Bits = (Bits::new(
            ((s_505_2) >> (s_505_0)).value(),
            u16::try_from(s_505_6).unwrap(),
        ));
        // D s_505_8: cast reint s_505_7 -> u8
        let s_505_8: u8 = (s_505_7.value() as u8);
        // D s_505_9: cast zx s_505_8 -> bv
        let s_505_9: Bits = Bits::new(s_505_8 as u128, 2u16);
        // C s_505_10: const #1u : u8
        let s_505_10: u8 = 1;
        // C s_505_11: cast zx s_505_10 -> bv
        let s_505_11: Bits = Bits::new(s_505_10 as u128, 2u16);
        // D s_505_12: cmp-eq s_505_9 s_505_11
        let s_505_12: bool = ((s_505_9) == (s_505_11));
        // D s_505_13: write-var gs#383034 <= s_505_12
        fn_state.gs_383034 = s_505_12;
        // N s_505_14: jump b232
        return block_232(state, tracer, fn_state);
    }
    fn block_506<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_506_0: const #189s : i
        let s_506_0: i128 = 189;
        // C s_506_1: const #14696u : u32
        let s_506_1: u32 = 14696;
        // D s_506_2: read-reg s_506_1:i
        let s_506_2: i128 = {
            let value = state.read_register::<i128>(s_506_1 as isize);
            tracer.read_register(s_506_1 as isize, value);
            value
        };
        // D s_506_3: cmp-lt s_506_2 s_506_0
        let s_506_3: bool = ((s_506_2) < (s_506_0));
        // D s_506_4: write-var gs#383012 <= s_506_3
        fn_state.gs_383012 = s_506_3;
        // N s_506_5: jump b228
        return block_228(state, tracer, fn_state);
    }
    fn block_507<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_507_0: const #10s : i
        let s_507_0: i128 = 10;
        // D s_507_1: read-var u#25986:u32
        let s_507_1: u32 = fn_state.u_25986;
        // D s_507_2: cast zx s_507_1 -> bv
        let s_507_2: Bits = Bits::new(s_507_1 as u128, 32u16);
        // C s_507_3: const #1s : i64
        let s_507_3: i64 = 1;
        // C s_507_4: cast zx s_507_3 -> i
        let s_507_4: i128 = (i128::try_from(s_507_3).unwrap());
        // C s_507_5: const #1s : i
        let s_507_5: i128 = 1;
        // C s_507_6: add s_507_5 s_507_4
        let s_507_6: i128 = (s_507_5 + s_507_4);
        // D s_507_7: bit-extract s_507_2 s_507_0 s_507_6
        let s_507_7: Bits = (Bits::new(
            ((s_507_2) >> (s_507_0)).value(),
            u16::try_from(s_507_6).unwrap(),
        ));
        // D s_507_8: cast reint s_507_7 -> u8
        let s_507_8: u8 = (s_507_7.value() as u8);
        // D s_507_9: cast zx s_507_8 -> bv
        let s_507_9: Bits = Bits::new(s_507_8 as u128, 2u16);
        // C s_507_10: const #0u : u8
        let s_507_10: u8 = 0;
        // C s_507_11: cast zx s_507_10 -> bv
        let s_507_11: Bits = Bits::new(s_507_10 as u128, 2u16);
        // D s_507_12: cmp-eq s_507_9 s_507_11
        let s_507_12: bool = ((s_507_9) == (s_507_11));
        // D s_507_13: write-var gs#383010 <= s_507_12
        fn_state.gs_383010 = s_507_12;
        // N s_507_14: jump b226
        return block_226(state, tracer, fn_state);
    }
    fn block_508<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_508_0: const #188s : i
        let s_508_0: i128 = 188;
        // C s_508_1: const #14696u : u32
        let s_508_1: u32 = 14696;
        // D s_508_2: read-reg s_508_1:i
        let s_508_2: i128 = {
            let value = state.read_register::<i128>(s_508_1 as isize);
            tracer.read_register(s_508_1 as isize, value);
            value
        };
        // D s_508_3: cmp-lt s_508_2 s_508_0
        let s_508_3: bool = ((s_508_2) < (s_508_0));
        // D s_508_4: write-var gs#382988 <= s_508_3
        fn_state.gs_382988 = s_508_3;
        // N s_508_5: jump b222
        return block_222(state, tracer, fn_state);
    }
    fn block_509<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_509_0: const #10s : i
        let s_509_0: i128 = 10;
        // D s_509_1: read-var u#25977:u32
        let s_509_1: u32 = fn_state.u_25977;
        // D s_509_2: cast zx s_509_1 -> bv
        let s_509_2: Bits = Bits::new(s_509_1 as u128, 32u16);
        // C s_509_3: const #1s : i64
        let s_509_3: i64 = 1;
        // C s_509_4: cast zx s_509_3 -> i
        let s_509_4: i128 = (i128::try_from(s_509_3).unwrap());
        // C s_509_5: const #1s : i
        let s_509_5: i128 = 1;
        // C s_509_6: add s_509_5 s_509_4
        let s_509_6: i128 = (s_509_5 + s_509_4);
        // D s_509_7: bit-extract s_509_2 s_509_0 s_509_6
        let s_509_7: Bits = (Bits::new(
            ((s_509_2) >> (s_509_0)).value(),
            u16::try_from(s_509_6).unwrap(),
        ));
        // D s_509_8: cast reint s_509_7 -> u8
        let s_509_8: u8 = (s_509_7.value() as u8);
        // D s_509_9: cast zx s_509_8 -> bv
        let s_509_9: Bits = Bits::new(s_509_8 as u128, 2u16);
        // C s_509_10: const #1u : u8
        let s_509_10: u8 = 1;
        // C s_509_11: cast zx s_509_10 -> bv
        let s_509_11: Bits = Bits::new(s_509_10 as u128, 2u16);
        // D s_509_12: cmp-eq s_509_9 s_509_11
        let s_509_12: bool = ((s_509_9) == (s_509_11));
        // D s_509_13: write-var gs#382986 <= s_509_12
        fn_state.gs_382986 = s_509_12;
        // N s_509_14: jump b220
        return block_220(state, tracer, fn_state);
    }
    fn block_510<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_510_0: const #187s : i
        let s_510_0: i128 = 187;
        // C s_510_1: const #14696u : u32
        let s_510_1: u32 = 14696;
        // D s_510_2: read-reg s_510_1:i
        let s_510_2: i128 = {
            let value = state.read_register::<i128>(s_510_1 as isize);
            tracer.read_register(s_510_1 as isize, value);
            value
        };
        // D s_510_3: cmp-lt s_510_2 s_510_0
        let s_510_3: bool = ((s_510_2) < (s_510_0));
        // D s_510_4: write-var gs#382964 <= s_510_3
        fn_state.gs_382964 = s_510_3;
        // N s_510_5: jump b216
        return block_216(state, tracer, fn_state);
    }
    fn block_511<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_511_0: const #10s : i
        let s_511_0: i128 = 10;
        // D s_511_1: read-var u#25969:u32
        let s_511_1: u32 = fn_state.u_25969;
        // D s_511_2: cast zx s_511_1 -> bv
        let s_511_2: Bits = Bits::new(s_511_1 as u128, 32u16);
        // C s_511_3: const #1s : i64
        let s_511_3: i64 = 1;
        // C s_511_4: cast zx s_511_3 -> i
        let s_511_4: i128 = (i128::try_from(s_511_3).unwrap());
        // C s_511_5: const #1s : i
        let s_511_5: i128 = 1;
        // C s_511_6: add s_511_5 s_511_4
        let s_511_6: i128 = (s_511_5 + s_511_4);
        // D s_511_7: bit-extract s_511_2 s_511_0 s_511_6
        let s_511_7: Bits = (Bits::new(
            ((s_511_2) >> (s_511_0)).value(),
            u16::try_from(s_511_6).unwrap(),
        ));
        // D s_511_8: cast reint s_511_7 -> u8
        let s_511_8: u8 = (s_511_7.value() as u8);
        // D s_511_9: cast zx s_511_8 -> bv
        let s_511_9: Bits = Bits::new(s_511_8 as u128, 2u16);
        // C s_511_10: const #0u : u8
        let s_511_10: u8 = 0;
        // C s_511_11: cast zx s_511_10 -> bv
        let s_511_11: Bits = Bits::new(s_511_10 as u128, 2u16);
        // D s_511_12: cmp-eq s_511_9 s_511_11
        let s_511_12: bool = ((s_511_9) == (s_511_11));
        // D s_511_13: write-var gs#382962 <= s_511_12
        fn_state.gs_382962 = s_511_12;
        // N s_511_14: jump b214
        return block_214(state, tracer, fn_state);
    }
    fn block_512<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_512_0: const #186s : i
        let s_512_0: i128 = 186;
        // C s_512_1: const #14696u : u32
        let s_512_1: u32 = 14696;
        // D s_512_2: read-reg s_512_1:i
        let s_512_2: i128 = {
            let value = state.read_register::<i128>(s_512_1 as isize);
            tracer.read_register(s_512_1 as isize, value);
            value
        };
        // D s_512_3: cmp-lt s_512_2 s_512_0
        let s_512_3: bool = ((s_512_2) < (s_512_0));
        // D s_512_4: write-var gs#382942 <= s_512_3
        fn_state.gs_382942 = s_512_3;
        // N s_512_5: jump b210
        return block_210(state, tracer, fn_state);
    }
    fn block_513<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_513_0: const #12s : i
        let s_513_0: i128 = 12;
        // D s_513_1: read-var u#25961:u32
        let s_513_1: u32 = fn_state.u_25961;
        // D s_513_2: cast zx s_513_1 -> bv
        let s_513_2: Bits = Bits::new(s_513_1 as u128, 32u16);
        // C s_513_3: const #1s : i64
        let s_513_3: i64 = 1;
        // C s_513_4: cast zx s_513_3 -> i
        let s_513_4: i128 = (i128::try_from(s_513_3).unwrap());
        // C s_513_5: const #3s : i
        let s_513_5: i128 = 3;
        // C s_513_6: add s_513_5 s_513_4
        let s_513_6: i128 = (s_513_5 + s_513_4);
        // D s_513_7: bit-extract s_513_2 s_513_0 s_513_6
        let s_513_7: Bits = (Bits::new(
            ((s_513_2) >> (s_513_0)).value(),
            u16::try_from(s_513_6).unwrap(),
        ));
        // D s_513_8: cast reint s_513_7 -> u8
        let s_513_8: u8 = (s_513_7.value() as u8);
        // D s_513_9: cast zx s_513_8 -> bv
        let s_513_9: Bits = Bits::new(s_513_8 as u128, 4u16);
        // C s_513_10: const #5u : u8
        let s_513_10: u8 = 5;
        // C s_513_11: cast zx s_513_10 -> bv
        let s_513_11: Bits = Bits::new(s_513_10 as u128, 4u16);
        // D s_513_12: cmp-eq s_513_9 s_513_11
        let s_513_12: bool = ((s_513_9) == (s_513_11));
        // D s_513_13: write-var gs#382940 <= s_513_12
        fn_state.gs_382940 = s_513_12;
        // N s_513_14: jump b208
        return block_208(state, tracer, fn_state);
    }
    fn block_514<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_514_0: const #185s : i
        let s_514_0: i128 = 185;
        // C s_514_1: const #14696u : u32
        let s_514_1: u32 = 14696;
        // D s_514_2: read-reg s_514_1:i
        let s_514_2: i128 = {
            let value = state.read_register::<i128>(s_514_1 as isize);
            tracer.read_register(s_514_1 as isize, value);
            value
        };
        // D s_514_3: cmp-lt s_514_2 s_514_0
        let s_514_3: bool = ((s_514_2) < (s_514_0));
        // D s_514_4: write-var gs#382920 <= s_514_3
        fn_state.gs_382920 = s_514_3;
        // N s_514_5: jump b204
        return block_204(state, tracer, fn_state);
    }
    fn block_515<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_515_0: const #12s : i
        let s_515_0: i128 = 12;
        // D s_515_1: read-var u#25955:u32
        let s_515_1: u32 = fn_state.u_25955;
        // D s_515_2: cast zx s_515_1 -> bv
        let s_515_2: Bits = Bits::new(s_515_1 as u128, 32u16);
        // C s_515_3: const #1s : i64
        let s_515_3: i64 = 1;
        // C s_515_4: cast zx s_515_3 -> i
        let s_515_4: i128 = (i128::try_from(s_515_3).unwrap());
        // C s_515_5: const #3s : i
        let s_515_5: i128 = 3;
        // C s_515_6: add s_515_5 s_515_4
        let s_515_6: i128 = (s_515_5 + s_515_4);
        // D s_515_7: bit-extract s_515_2 s_515_0 s_515_6
        let s_515_7: Bits = (Bits::new(
            ((s_515_2) >> (s_515_0)).value(),
            u16::try_from(s_515_6).unwrap(),
        ));
        // D s_515_8: cast reint s_515_7 -> u8
        let s_515_8: u8 = (s_515_7.value() as u8);
        // D s_515_9: cast zx s_515_8 -> bv
        let s_515_9: Bits = Bits::new(s_515_8 as u128, 4u16);
        // C s_515_10: const #4u : u8
        let s_515_10: u8 = 4;
        // C s_515_11: cast zx s_515_10 -> bv
        let s_515_11: Bits = Bits::new(s_515_10 as u128, 4u16);
        // D s_515_12: cmp-eq s_515_9 s_515_11
        let s_515_12: bool = ((s_515_9) == (s_515_11));
        // D s_515_13: write-var gs#382918 <= s_515_12
        fn_state.gs_382918 = s_515_12;
        // N s_515_14: jump b202
        return block_202(state, tracer, fn_state);
    }
    fn block_516<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_516_0: const #151s : i
        let s_516_0: i128 = 151;
        // C s_516_1: const #14696u : u32
        let s_516_1: u32 = 14696;
        // D s_516_2: read-reg s_516_1:i
        let s_516_2: i128 = {
            let value = state.read_register::<i128>(s_516_1 as isize);
            tracer.read_register(s_516_1 as isize, value);
            value
        };
        // D s_516_3: cmp-lt s_516_2 s_516_0
        let s_516_3: bool = ((s_516_2) < (s_516_0));
        // D s_516_4: write-var gs#382904 <= s_516_3
        fn_state.gs_382904 = s_516_3;
        // N s_516_5: jump b198
        return block_198(state, tracer, fn_state);
    }
    fn block_517<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_517_0: const #128s : i
        let s_517_0: i128 = 128;
        // C s_517_1: const #14696u : u32
        let s_517_1: u32 = 14696;
        // D s_517_2: read-reg s_517_1:i
        let s_517_2: i128 = {
            let value = state.read_register::<i128>(s_517_1 as isize);
            tracer.read_register(s_517_1 as isize, value);
            value
        };
        // D s_517_3: cmp-lt s_517_2 s_517_0
        let s_517_3: bool = ((s_517_2) < (s_517_0));
        // D s_517_4: write-var gs#382889 <= s_517_3
        fn_state.gs_382889 = s_517_3;
        // N s_517_5: jump b194
        return block_194(state, tracer, fn_state);
    }
    fn block_518<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_518_0: const #127s : i
        let s_518_0: i128 = 127;
        // C s_518_1: const #14696u : u32
        let s_518_1: u32 = 14696;
        // D s_518_2: read-reg s_518_1:i
        let s_518_2: i128 = {
            let value = state.read_register::<i128>(s_518_1 as isize);
            tracer.read_register(s_518_1 as isize, value);
            value
        };
        // D s_518_3: cmp-lt s_518_2 s_518_0
        let s_518_3: bool = ((s_518_2) < (s_518_0));
        // D s_518_4: write-var gs#382874 <= s_518_3
        fn_state.gs_382874 = s_518_3;
        // N s_518_5: jump b190
        return block_190(state, tracer, fn_state);
    }
    fn block_519<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_519_0: const #122s : i
        let s_519_0: i128 = 122;
        // C s_519_1: const #14696u : u32
        let s_519_1: u32 = 14696;
        // D s_519_2: read-reg s_519_1:i
        let s_519_2: i128 = {
            let value = state.read_register::<i128>(s_519_1 as isize);
            tracer.read_register(s_519_1 as isize, value);
            value
        };
        // D s_519_3: cmp-lt s_519_2 s_519_0
        let s_519_3: bool = ((s_519_2) < (s_519_0));
        // D s_519_4: write-var gs#382855 <= s_519_3
        fn_state.gs_382855 = s_519_3;
        // N s_519_5: jump b186
        return block_186(state, tracer, fn_state);
    }
    fn block_520<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_520_0: const #10s : i
        let s_520_0: i128 = 10;
        // D s_520_1: read-var u#25930:u32
        let s_520_1: u32 = fn_state.u_25930;
        // D s_520_2: cast zx s_520_1 -> bv
        let s_520_2: Bits = Bits::new(s_520_1 as u128, 32u16);
        // C s_520_3: const #1s : i64
        let s_520_3: i64 = 1;
        // C s_520_4: cast zx s_520_3 -> i
        let s_520_4: i128 = (i128::try_from(s_520_3).unwrap());
        // C s_520_5: const #1s : i
        let s_520_5: i128 = 1;
        // C s_520_6: add s_520_5 s_520_4
        let s_520_6: i128 = (s_520_5 + s_520_4);
        // D s_520_7: bit-extract s_520_2 s_520_0 s_520_6
        let s_520_7: Bits = (Bits::new(
            ((s_520_2) >> (s_520_0)).value(),
            u16::try_from(s_520_6).unwrap(),
        ));
        // D s_520_8: cast reint s_520_7 -> u8
        let s_520_8: u8 = (s_520_7.value() as u8);
        // D s_520_9: cast zx s_520_8 -> bv
        let s_520_9: Bits = Bits::new(s_520_8 as u128, 2u16);
        // C s_520_10: const #2u : u8
        let s_520_10: u8 = 2;
        // C s_520_11: cast zx s_520_10 -> bv
        let s_520_11: Bits = Bits::new(s_520_10 as u128, 2u16);
        // D s_520_12: cmp-eq s_520_9 s_520_11
        let s_520_12: bool = ((s_520_9) == (s_520_11));
        // N s_520_13: branch s_520_12 b523 b521
        if s_520_12 {
            return block_523(state, tracer, fn_state);
        } else {
            return block_521(state, tracer, fn_state);
        };
    }
    fn block_521<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_521_0: const #0u : u8
        let s_521_0: bool = false;
        // D s_521_1: write-var gs#382852 <= s_521_0
        fn_state.gs_382852 = s_521_0;
        // N s_521_2: jump b522
        return block_522(state, tracer, fn_state);
    }
    fn block_522<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_522_0: read-var gs#382852:u8
        let s_522_0: bool = fn_state.gs_382852;
        // D s_522_1: write-var gs#382853 <= s_522_0
        fn_state.gs_382853 = s_522_0;
        // N s_522_2: jump b184
        return block_184(state, tracer, fn_state);
    }
    fn block_523<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_523_0: const #4s : i
        let s_523_0: i128 = 4;
        // D s_523_1: read-var u#25930:u32
        let s_523_1: u32 = fn_state.u_25930;
        // D s_523_2: cast zx s_523_1 -> bv
        let s_523_2: Bits = Bits::new(s_523_1 as u128, 32u16);
        // C s_523_3: const #1s : i64
        let s_523_3: i64 = 1;
        // C s_523_4: cast zx s_523_3 -> i
        let s_523_4: i128 = (i128::try_from(s_523_3).unwrap());
        // C s_523_5: const #0s : i
        let s_523_5: i128 = 0;
        // C s_523_6: add s_523_5 s_523_4
        let s_523_6: i128 = (s_523_5 + s_523_4);
        // D s_523_7: bit-extract s_523_2 s_523_0 s_523_6
        let s_523_7: Bits = (Bits::new(
            ((s_523_2) >> (s_523_0)).value(),
            u16::try_from(s_523_6).unwrap(),
        ));
        // D s_523_8: cast reint s_523_7 -> u8
        let s_523_8: bool = ((s_523_7.value()) != 0);
        // D s_523_9: cast zx s_523_8 -> bv
        let s_523_9: Bits = Bits::new(s_523_8 as u128, 1u16);
        // C s_523_10: const #0u : u8
        let s_523_10: bool = false;
        // C s_523_11: cast zx s_523_10 -> bv
        let s_523_11: Bits = Bits::new(s_523_10 as u128, 1u16);
        // D s_523_12: cmp-eq s_523_9 s_523_11
        let s_523_12: bool = ((s_523_9) == (s_523_11));
        // D s_523_13: write-var gs#382852 <= s_523_12
        fn_state.gs_382852 = s_523_12;
        // N s_523_14: jump b522
        return block_522(state, tracer, fn_state);
    }
    fn block_524<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_524_0: const #121s : i
        let s_524_0: i128 = 121;
        // C s_524_1: const #14696u : u32
        let s_524_1: u32 = 14696;
        // D s_524_2: read-reg s_524_1:i
        let s_524_2: i128 = {
            let value = state.read_register::<i128>(s_524_1 as isize);
            tracer.read_register(s_524_1 as isize, value);
            value
        };
        // D s_524_3: cmp-lt s_524_2 s_524_0
        let s_524_3: bool = ((s_524_2) < (s_524_0));
        // D s_524_4: write-var gs#382830 <= s_524_3
        fn_state.gs_382830 = s_524_3;
        // N s_524_5: jump b180
        return block_180(state, tracer, fn_state);
    }
    fn block_525<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_525_0: const #10s : i
        let s_525_0: i128 = 10;
        // D s_525_1: read-var u#25923:u32
        let s_525_1: u32 = fn_state.u_25923;
        // D s_525_2: cast zx s_525_1 -> bv
        let s_525_2: Bits = Bits::new(s_525_1 as u128, 32u16);
        // C s_525_3: const #1s : i64
        let s_525_3: i64 = 1;
        // C s_525_4: cast zx s_525_3 -> i
        let s_525_4: i128 = (i128::try_from(s_525_3).unwrap());
        // C s_525_5: const #1s : i
        let s_525_5: i128 = 1;
        // C s_525_6: add s_525_5 s_525_4
        let s_525_6: i128 = (s_525_5 + s_525_4);
        // D s_525_7: bit-extract s_525_2 s_525_0 s_525_6
        let s_525_7: Bits = (Bits::new(
            ((s_525_2) >> (s_525_0)).value(),
            u16::try_from(s_525_6).unwrap(),
        ));
        // D s_525_8: cast reint s_525_7 -> u8
        let s_525_8: u8 = (s_525_7.value() as u8);
        // D s_525_9: cast zx s_525_8 -> bv
        let s_525_9: Bits = Bits::new(s_525_8 as u128, 2u16);
        // C s_525_10: const #2u : u8
        let s_525_10: u8 = 2;
        // C s_525_11: cast zx s_525_10 -> bv
        let s_525_11: Bits = Bits::new(s_525_10 as u128, 2u16);
        // D s_525_12: cmp-eq s_525_9 s_525_11
        let s_525_12: bool = ((s_525_9) == (s_525_11));
        // N s_525_13: branch s_525_12 b528 b526
        if s_525_12 {
            return block_528(state, tracer, fn_state);
        } else {
            return block_526(state, tracer, fn_state);
        };
    }
    fn block_526<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_526_0: const #0u : u8
        let s_526_0: bool = false;
        // D s_526_1: write-var gs#382827 <= s_526_0
        fn_state.gs_382827 = s_526_0;
        // N s_526_2: jump b527
        return block_527(state, tracer, fn_state);
    }
    fn block_527<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_527_0: read-var gs#382827:u8
        let s_527_0: bool = fn_state.gs_382827;
        // D s_527_1: write-var gs#382828 <= s_527_0
        fn_state.gs_382828 = s_527_0;
        // N s_527_2: jump b178
        return block_178(state, tracer, fn_state);
    }
    fn block_528<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_528_0: const #4s : i
        let s_528_0: i128 = 4;
        // D s_528_1: read-var u#25923:u32
        let s_528_1: u32 = fn_state.u_25923;
        // D s_528_2: cast zx s_528_1 -> bv
        let s_528_2: Bits = Bits::new(s_528_1 as u128, 32u16);
        // C s_528_3: const #1s : i64
        let s_528_3: i64 = 1;
        // C s_528_4: cast zx s_528_3 -> i
        let s_528_4: i128 = (i128::try_from(s_528_3).unwrap());
        // C s_528_5: const #0s : i
        let s_528_5: i128 = 0;
        // C s_528_6: add s_528_5 s_528_4
        let s_528_6: i128 = (s_528_5 + s_528_4);
        // D s_528_7: bit-extract s_528_2 s_528_0 s_528_6
        let s_528_7: Bits = (Bits::new(
            ((s_528_2) >> (s_528_0)).value(),
            u16::try_from(s_528_6).unwrap(),
        ));
        // D s_528_8: cast reint s_528_7 -> u8
        let s_528_8: bool = ((s_528_7.value()) != 0);
        // D s_528_9: cast zx s_528_8 -> bv
        let s_528_9: Bits = Bits::new(s_528_8 as u128, 1u16);
        // C s_528_10: const #0u : u8
        let s_528_10: bool = false;
        // C s_528_11: cast zx s_528_10 -> bv
        let s_528_11: Bits = Bits::new(s_528_10 as u128, 1u16);
        // D s_528_12: cmp-eq s_528_9 s_528_11
        let s_528_12: bool = ((s_528_9) == (s_528_11));
        // D s_528_13: write-var gs#382827 <= s_528_12
        fn_state.gs_382827 = s_528_12;
        // N s_528_14: jump b527
        return block_527(state, tracer, fn_state);
    }
    fn block_529<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_529_0: const #120s : i
        let s_529_0: i128 = 120;
        // C s_529_1: const #14696u : u32
        let s_529_1: u32 = 14696;
        // D s_529_2: read-reg s_529_1:i
        let s_529_2: i128 = {
            let value = state.read_register::<i128>(s_529_1 as isize);
            tracer.read_register(s_529_1 as isize, value);
            value
        };
        // D s_529_3: cmp-lt s_529_2 s_529_0
        let s_529_3: bool = ((s_529_2) < (s_529_0));
        // D s_529_4: write-var gs#382805 <= s_529_3
        fn_state.gs_382805 = s_529_3;
        // N s_529_5: jump b174
        return block_174(state, tracer, fn_state);
    }
    fn block_530<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_530_0: const #10s : i
        let s_530_0: i128 = 10;
        // D s_530_1: read-var u#25915:u32
        let s_530_1: u32 = fn_state.u_25915;
        // D s_530_2: cast zx s_530_1 -> bv
        let s_530_2: Bits = Bits::new(s_530_1 as u128, 32u16);
        // C s_530_3: const #1s : i64
        let s_530_3: i64 = 1;
        // C s_530_4: cast zx s_530_3 -> i
        let s_530_4: i128 = (i128::try_from(s_530_3).unwrap());
        // C s_530_5: const #1s : i
        let s_530_5: i128 = 1;
        // C s_530_6: add s_530_5 s_530_4
        let s_530_6: i128 = (s_530_5 + s_530_4);
        // D s_530_7: bit-extract s_530_2 s_530_0 s_530_6
        let s_530_7: Bits = (Bits::new(
            ((s_530_2) >> (s_530_0)).value(),
            u16::try_from(s_530_6).unwrap(),
        ));
        // D s_530_8: cast reint s_530_7 -> u8
        let s_530_8: u8 = (s_530_7.value() as u8);
        // D s_530_9: cast zx s_530_8 -> bv
        let s_530_9: Bits = Bits::new(s_530_8 as u128, 2u16);
        // C s_530_10: const #0u : u8
        let s_530_10: u8 = 0;
        // C s_530_11: cast zx s_530_10 -> bv
        let s_530_11: Bits = Bits::new(s_530_10 as u128, 2u16);
        // D s_530_12: cmp-eq s_530_9 s_530_11
        let s_530_12: bool = ((s_530_9) == (s_530_11));
        // N s_530_13: branch s_530_12 b533 b531
        if s_530_12 {
            return block_533(state, tracer, fn_state);
        } else {
            return block_531(state, tracer, fn_state);
        };
    }
    fn block_531<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_531_0: const #0u : u8
        let s_531_0: bool = false;
        // D s_531_1: write-var gs#382802 <= s_531_0
        fn_state.gs_382802 = s_531_0;
        // N s_531_2: jump b532
        return block_532(state, tracer, fn_state);
    }
    fn block_532<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_532_0: read-var gs#382802:u8
        let s_532_0: bool = fn_state.gs_382802;
        // D s_532_1: write-var gs#382803 <= s_532_0
        fn_state.gs_382803 = s_532_0;
        // N s_532_2: jump b172
        return block_172(state, tracer, fn_state);
    }
    fn block_533<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_533_0: const #4s : i
        let s_533_0: i128 = 4;
        // D s_533_1: read-var u#25915:u32
        let s_533_1: u32 = fn_state.u_25915;
        // D s_533_2: cast zx s_533_1 -> bv
        let s_533_2: Bits = Bits::new(s_533_1 as u128, 32u16);
        // C s_533_3: const #1s : i64
        let s_533_3: i64 = 1;
        // C s_533_4: cast zx s_533_3 -> i
        let s_533_4: i128 = (i128::try_from(s_533_3).unwrap());
        // C s_533_5: const #0s : i
        let s_533_5: i128 = 0;
        // C s_533_6: add s_533_5 s_533_4
        let s_533_6: i128 = (s_533_5 + s_533_4);
        // D s_533_7: bit-extract s_533_2 s_533_0 s_533_6
        let s_533_7: Bits = (Bits::new(
            ((s_533_2) >> (s_533_0)).value(),
            u16::try_from(s_533_6).unwrap(),
        ));
        // D s_533_8: cast reint s_533_7 -> u8
        let s_533_8: bool = ((s_533_7.value()) != 0);
        // D s_533_9: cast zx s_533_8 -> bv
        let s_533_9: Bits = Bits::new(s_533_8 as u128, 1u16);
        // C s_533_10: const #0u : u8
        let s_533_10: bool = false;
        // C s_533_11: cast zx s_533_10 -> bv
        let s_533_11: Bits = Bits::new(s_533_10 as u128, 1u16);
        // D s_533_12: cmp-eq s_533_9 s_533_11
        let s_533_12: bool = ((s_533_9) == (s_533_11));
        // D s_533_13: write-var gs#382802 <= s_533_12
        fn_state.gs_382802 = s_533_12;
        // N s_533_14: jump b532
        return block_532(state, tracer, fn_state);
    }
    fn block_534<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_534_0: const #119s : i
        let s_534_0: i128 = 119;
        // C s_534_1: const #14696u : u32
        let s_534_1: u32 = 14696;
        // D s_534_2: read-reg s_534_1:i
        let s_534_2: i128 = {
            let value = state.read_register::<i128>(s_534_1 as isize);
            tracer.read_register(s_534_1 as isize, value);
            value
        };
        // D s_534_3: cmp-lt s_534_2 s_534_0
        let s_534_3: bool = ((s_534_2) < (s_534_0));
        // D s_534_4: write-var gs#382780 <= s_534_3
        fn_state.gs_382780 = s_534_3;
        // N s_534_5: jump b168
        return block_168(state, tracer, fn_state);
    }
    fn block_535<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_535_0: const #10s : i
        let s_535_0: i128 = 10;
        // D s_535_1: read-var u#25909:u32
        let s_535_1: u32 = fn_state.u_25909;
        // D s_535_2: cast zx s_535_1 -> bv
        let s_535_2: Bits = Bits::new(s_535_1 as u128, 32u16);
        // C s_535_3: const #1s : i64
        let s_535_3: i64 = 1;
        // C s_535_4: cast zx s_535_3 -> i
        let s_535_4: i128 = (i128::try_from(s_535_3).unwrap());
        // C s_535_5: const #1s : i
        let s_535_5: i128 = 1;
        // C s_535_6: add s_535_5 s_535_4
        let s_535_6: i128 = (s_535_5 + s_535_4);
        // D s_535_7: bit-extract s_535_2 s_535_0 s_535_6
        let s_535_7: Bits = (Bits::new(
            ((s_535_2) >> (s_535_0)).value(),
            u16::try_from(s_535_6).unwrap(),
        ));
        // D s_535_8: cast reint s_535_7 -> u8
        let s_535_8: u8 = (s_535_7.value() as u8);
        // D s_535_9: cast zx s_535_8 -> bv
        let s_535_9: Bits = Bits::new(s_535_8 as u128, 2u16);
        // C s_535_10: const #0u : u8
        let s_535_10: u8 = 0;
        // C s_535_11: cast zx s_535_10 -> bv
        let s_535_11: Bits = Bits::new(s_535_10 as u128, 2u16);
        // D s_535_12: cmp-eq s_535_9 s_535_11
        let s_535_12: bool = ((s_535_9) == (s_535_11));
        // N s_535_13: branch s_535_12 b538 b536
        if s_535_12 {
            return block_538(state, tracer, fn_state);
        } else {
            return block_536(state, tracer, fn_state);
        };
    }
    fn block_536<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_536_0: const #0u : u8
        let s_536_0: bool = false;
        // D s_536_1: write-var gs#382777 <= s_536_0
        fn_state.gs_382777 = s_536_0;
        // N s_536_2: jump b537
        return block_537(state, tracer, fn_state);
    }
    fn block_537<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_537_0: read-var gs#382777:u8
        let s_537_0: bool = fn_state.gs_382777;
        // D s_537_1: write-var gs#382778 <= s_537_0
        fn_state.gs_382778 = s_537_0;
        // N s_537_2: jump b166
        return block_166(state, tracer, fn_state);
    }
    fn block_538<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_538_0: const #4s : i
        let s_538_0: i128 = 4;
        // D s_538_1: read-var u#25909:u32
        let s_538_1: u32 = fn_state.u_25909;
        // D s_538_2: cast zx s_538_1 -> bv
        let s_538_2: Bits = Bits::new(s_538_1 as u128, 32u16);
        // C s_538_3: const #1s : i64
        let s_538_3: i64 = 1;
        // C s_538_4: cast zx s_538_3 -> i
        let s_538_4: i128 = (i128::try_from(s_538_3).unwrap());
        // C s_538_5: const #0s : i
        let s_538_5: i128 = 0;
        // C s_538_6: add s_538_5 s_538_4
        let s_538_6: i128 = (s_538_5 + s_538_4);
        // D s_538_7: bit-extract s_538_2 s_538_0 s_538_6
        let s_538_7: Bits = (Bits::new(
            ((s_538_2) >> (s_538_0)).value(),
            u16::try_from(s_538_6).unwrap(),
        ));
        // D s_538_8: cast reint s_538_7 -> u8
        let s_538_8: bool = ((s_538_7.value()) != 0);
        // D s_538_9: cast zx s_538_8 -> bv
        let s_538_9: Bits = Bits::new(s_538_8 as u128, 1u16);
        // C s_538_10: const #0u : u8
        let s_538_10: bool = false;
        // C s_538_11: cast zx s_538_10 -> bv
        let s_538_11: Bits = Bits::new(s_538_10 as u128, 1u16);
        // D s_538_12: cmp-eq s_538_9 s_538_11
        let s_538_12: bool = ((s_538_9) == (s_538_11));
        // D s_538_13: write-var gs#382777 <= s_538_12
        fn_state.gs_382777 = s_538_12;
        // N s_538_14: jump b537
        return block_537(state, tracer, fn_state);
    }
    fn block_539<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_539_0: const #64s : i
        let s_539_0: i128 = 64;
        // C s_539_1: const #14696u : u32
        let s_539_1: u32 = 14696;
        // D s_539_2: read-reg s_539_1:i
        let s_539_2: i128 = {
            let value = state.read_register::<i128>(s_539_1 as isize);
            tracer.read_register(s_539_1 as isize, value);
            value
        };
        // D s_539_3: cmp-lt s_539_2 s_539_0
        let s_539_3: bool = ((s_539_2) < (s_539_0));
        // D s_539_4: write-var gs#382761 <= s_539_3
        fn_state.gs_382761 = s_539_3;
        // N s_539_5: jump b162
        return block_162(state, tracer, fn_state);
    }
    fn block_540<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_540_0: const #10s : i
        let s_540_0: i128 = 10;
        // D s_540_1: read-var u#25904:u32
        let s_540_1: u32 = fn_state.u_25904;
        // D s_540_2: cast zx s_540_1 -> bv
        let s_540_2: Bits = Bits::new(s_540_1 as u128, 32u16);
        // C s_540_3: const #1s : i64
        let s_540_3: i64 = 1;
        // C s_540_4: cast zx s_540_3 -> i
        let s_540_4: i128 = (i128::try_from(s_540_3).unwrap());
        // C s_540_5: const #2s : i
        let s_540_5: i128 = 2;
        // C s_540_6: add s_540_5 s_540_4
        let s_540_6: i128 = (s_540_5 + s_540_4);
        // D s_540_7: bit-extract s_540_2 s_540_0 s_540_6
        let s_540_7: Bits = (Bits::new(
            ((s_540_2) >> (s_540_0)).value(),
            u16::try_from(s_540_6).unwrap(),
        ));
        // D s_540_8: cast reint s_540_7 -> u8
        let s_540_8: u8 = (s_540_7.value() as u8);
        // D s_540_9: cast zx s_540_8 -> bv
        let s_540_9: Bits = Bits::new(s_540_8 as u128, 3u16);
        // C s_540_10: const #5u : u8
        let s_540_10: u8 = 5;
        // C s_540_11: cast zx s_540_10 -> bv
        let s_540_11: Bits = Bits::new(s_540_10 as u128, 3u16);
        // D s_540_12: cmp-eq s_540_9 s_540_11
        let s_540_12: bool = ((s_540_9) == (s_540_11));
        // D s_540_13: write-var gs#382759 <= s_540_12
        fn_state.gs_382759 = s_540_12;
        // N s_540_14: jump b160
        return block_160(state, tracer, fn_state);
    }
    fn block_541<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_541_0: const #62s : i
        let s_541_0: i128 = 62;
        // C s_541_1: const #14696u : u32
        let s_541_1: u32 = 14696;
        // D s_541_2: read-reg s_541_1:i
        let s_541_2: i128 = {
            let value = state.read_register::<i128>(s_541_1 as isize);
            tracer.read_register(s_541_1 as isize, value);
            value
        };
        // D s_541_3: cmp-lt s_541_2 s_541_0
        let s_541_3: bool = ((s_541_2) < (s_541_0));
        // D s_541_4: write-var gs#382745 <= s_541_3
        fn_state.gs_382745 = s_541_3;
        // N s_541_5: jump b156
        return block_156(state, tracer, fn_state);
    }
    fn block_542<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_542_0: const #10s : i
        let s_542_0: i128 = 10;
        // D s_542_1: read-var u#25899:u32
        let s_542_1: u32 = fn_state.u_25899;
        // D s_542_2: cast zx s_542_1 -> bv
        let s_542_2: Bits = Bits::new(s_542_1 as u128, 32u16);
        // C s_542_3: const #1s : i64
        let s_542_3: i64 = 1;
        // C s_542_4: cast zx s_542_3 -> i
        let s_542_4: i128 = (i128::try_from(s_542_3).unwrap());
        // C s_542_5: const #2s : i
        let s_542_5: i128 = 2;
        // C s_542_6: add s_542_5 s_542_4
        let s_542_6: i128 = (s_542_5 + s_542_4);
        // D s_542_7: bit-extract s_542_2 s_542_0 s_542_6
        let s_542_7: Bits = (Bits::new(
            ((s_542_2) >> (s_542_0)).value(),
            u16::try_from(s_542_6).unwrap(),
        ));
        // D s_542_8: cast reint s_542_7 -> u8
        let s_542_8: u8 = (s_542_7.value() as u8);
        // D s_542_9: cast zx s_542_8 -> bv
        let s_542_9: Bits = Bits::new(s_542_8 as u128, 3u16);
        // C s_542_10: const #4u : u8
        let s_542_10: u8 = 4;
        // C s_542_11: cast zx s_542_10 -> bv
        let s_542_11: Bits = Bits::new(s_542_10 as u128, 3u16);
        // D s_542_12: cmp-eq s_542_9 s_542_11
        let s_542_12: bool = ((s_542_9) == (s_542_11));
        // D s_542_13: write-var gs#382743 <= s_542_12
        fn_state.gs_382743 = s_542_12;
        // N s_542_14: jump b154
        return block_154(state, tracer, fn_state);
    }
    fn block_543<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_543_0: const #61s : i
        let s_543_0: i128 = 61;
        // C s_543_1: const #14696u : u32
        let s_543_1: u32 = 14696;
        // D s_543_2: read-reg s_543_1:i
        let s_543_2: i128 = {
            let value = state.read_register::<i128>(s_543_1 as isize);
            tracer.read_register(s_543_1 as isize, value);
            value
        };
        // D s_543_3: cmp-lt s_543_2 s_543_0
        let s_543_3: bool = ((s_543_2) < (s_543_0));
        // D s_543_4: write-var gs#382729 <= s_543_3
        fn_state.gs_382729 = s_543_3;
        // N s_543_5: jump b150
        return block_150(state, tracer, fn_state);
    }
    fn block_544<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_544_0: const #10s : i
        let s_544_0: i128 = 10;
        // D s_544_1: read-var u#25894:u32
        let s_544_1: u32 = fn_state.u_25894;
        // D s_544_2: cast zx s_544_1 -> bv
        let s_544_2: Bits = Bits::new(s_544_1 as u128, 32u16);
        // C s_544_3: const #1s : i64
        let s_544_3: i64 = 1;
        // C s_544_4: cast zx s_544_3 -> i
        let s_544_4: i128 = (i128::try_from(s_544_3).unwrap());
        // C s_544_5: const #2s : i
        let s_544_5: i128 = 2;
        // C s_544_6: add s_544_5 s_544_4
        let s_544_6: i128 = (s_544_5 + s_544_4);
        // D s_544_7: bit-extract s_544_2 s_544_0 s_544_6
        let s_544_7: Bits = (Bits::new(
            ((s_544_2) >> (s_544_0)).value(),
            u16::try_from(s_544_6).unwrap(),
        ));
        // D s_544_8: cast reint s_544_7 -> u8
        let s_544_8: u8 = (s_544_7.value() as u8);
        // D s_544_9: cast zx s_544_8 -> bv
        let s_544_9: Bits = Bits::new(s_544_8 as u128, 3u16);
        // C s_544_10: const #7u : u8
        let s_544_10: u8 = 7;
        // C s_544_11: cast zx s_544_10 -> bv
        let s_544_11: Bits = Bits::new(s_544_10 as u128, 3u16);
        // D s_544_12: cmp-eq s_544_9 s_544_11
        let s_544_12: bool = ((s_544_9) == (s_544_11));
        // D s_544_13: write-var gs#382727 <= s_544_12
        fn_state.gs_382727 = s_544_12;
        // N s_544_14: jump b148
        return block_148(state, tracer, fn_state);
    }
    fn block_545<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_545_0: const #60s : i
        let s_545_0: i128 = 60;
        // C s_545_1: const #14696u : u32
        let s_545_1: u32 = 14696;
        // D s_545_2: read-reg s_545_1:i
        let s_545_2: i128 = {
            let value = state.read_register::<i128>(s_545_1 as isize);
            tracer.read_register(s_545_1 as isize, value);
            value
        };
        // D s_545_3: cmp-lt s_545_2 s_545_0
        let s_545_3: bool = ((s_545_2) < (s_545_0));
        // D s_545_4: write-var gs#382713 <= s_545_3
        fn_state.gs_382713 = s_545_3;
        // N s_545_5: jump b144
        return block_144(state, tracer, fn_state);
    }
    fn block_546<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_546_0: const #10s : i
        let s_546_0: i128 = 10;
        // D s_546_1: read-var u#25890:u32
        let s_546_1: u32 = fn_state.u_25890;
        // D s_546_2: cast zx s_546_1 -> bv
        let s_546_2: Bits = Bits::new(s_546_1 as u128, 32u16);
        // C s_546_3: const #1s : i64
        let s_546_3: i64 = 1;
        // C s_546_4: cast zx s_546_3 -> i
        let s_546_4: i128 = (i128::try_from(s_546_3).unwrap());
        // C s_546_5: const #2s : i
        let s_546_5: i128 = 2;
        // C s_546_6: add s_546_5 s_546_4
        let s_546_6: i128 = (s_546_5 + s_546_4);
        // D s_546_7: bit-extract s_546_2 s_546_0 s_546_6
        let s_546_7: Bits = (Bits::new(
            ((s_546_2) >> (s_546_0)).value(),
            u16::try_from(s_546_6).unwrap(),
        ));
        // D s_546_8: cast reint s_546_7 -> u8
        let s_546_8: u8 = (s_546_7.value() as u8);
        // D s_546_9: cast zx s_546_8 -> bv
        let s_546_9: Bits = Bits::new(s_546_8 as u128, 3u16);
        // C s_546_10: const #6u : u8
        let s_546_10: u8 = 6;
        // C s_546_11: cast zx s_546_10 -> bv
        let s_546_11: Bits = Bits::new(s_546_10 as u128, 3u16);
        // D s_546_12: cmp-eq s_546_9 s_546_11
        let s_546_12: bool = ((s_546_9) == (s_546_11));
        // D s_546_13: write-var gs#382711 <= s_546_12
        fn_state.gs_382711 = s_546_12;
        // N s_546_14: jump b142
        return block_142(state, tracer, fn_state);
    }
    fn block_547<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_547_0: const #59s : i
        let s_547_0: i128 = 59;
        // C s_547_1: const #14696u : u32
        let s_547_1: u32 = 14696;
        // D s_547_2: read-reg s_547_1:i
        let s_547_2: i128 = {
            let value = state.read_register::<i128>(s_547_1 as isize);
            tracer.read_register(s_547_1 as isize, value);
            value
        };
        // D s_547_3: cmp-lt s_547_2 s_547_0
        let s_547_3: bool = ((s_547_2) < (s_547_0));
        // D s_547_4: write-var gs#382693 <= s_547_3
        fn_state.gs_382693 = s_547_3;
        // N s_547_5: jump b138
        return block_138(state, tracer, fn_state);
    }
    fn block_548<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_548_0: const #10s : i
        let s_548_0: i128 = 10;
        // D s_548_1: read-var u#25883:u32
        let s_548_1: u32 = fn_state.u_25883;
        // D s_548_2: cast zx s_548_1 -> bv
        let s_548_2: Bits = Bits::new(s_548_1 as u128, 32u16);
        // C s_548_3: const #1s : i64
        let s_548_3: i64 = 1;
        // C s_548_4: cast zx s_548_3 -> i
        let s_548_4: i128 = (i128::try_from(s_548_3).unwrap());
        // C s_548_5: const #5s : i
        let s_548_5: i128 = 5;
        // C s_548_6: add s_548_5 s_548_4
        let s_548_6: i128 = (s_548_5 + s_548_4);
        // D s_548_7: bit-extract s_548_2 s_548_0 s_548_6
        let s_548_7: Bits = (Bits::new(
            ((s_548_2) >> (s_548_0)).value(),
            u16::try_from(s_548_6).unwrap(),
        ));
        // D s_548_8: cast reint s_548_7 -> u8
        let s_548_8: u8 = (s_548_7.value() as u8);
        // D s_548_9: cast zx s_548_8 -> bv
        let s_548_9: Bits = Bits::new(s_548_8 as u128, 6u16);
        // C s_548_10: const #11u : u8
        let s_548_10: u8 = 11;
        // C s_548_11: cast zx s_548_10 -> bv
        let s_548_11: Bits = Bits::new(s_548_10 as u128, 6u16);
        // D s_548_12: cmp-eq s_548_9 s_548_11
        let s_548_12: bool = ((s_548_9) == (s_548_11));
        // D s_548_13: write-var gs#382691 <= s_548_12
        fn_state.gs_382691 = s_548_12;
        // N s_548_14: jump b136
        return block_136(state, tracer, fn_state);
    }
    fn block_549<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_549_0: const #58s : i
        let s_549_0: i128 = 58;
        // C s_549_1: const #14696u : u32
        let s_549_1: u32 = 14696;
        // D s_549_2: read-reg s_549_1:i
        let s_549_2: i128 = {
            let value = state.read_register::<i128>(s_549_1 as isize);
            tracer.read_register(s_549_1 as isize, value);
            value
        };
        // D s_549_3: cmp-lt s_549_2 s_549_0
        let s_549_3: bool = ((s_549_2) < (s_549_0));
        // D s_549_4: write-var gs#382673 <= s_549_3
        fn_state.gs_382673 = s_549_3;
        // N s_549_5: jump b132
        return block_132(state, tracer, fn_state);
    }
    fn block_550<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_550_0: const #10s : i
        let s_550_0: i128 = 10;
        // D s_550_1: read-var u#25876:u32
        let s_550_1: u32 = fn_state.u_25876;
        // D s_550_2: cast zx s_550_1 -> bv
        let s_550_2: Bits = Bits::new(s_550_1 as u128, 32u16);
        // C s_550_3: const #1s : i64
        let s_550_3: i64 = 1;
        // C s_550_4: cast zx s_550_3 -> i
        let s_550_4: i128 = (i128::try_from(s_550_3).unwrap());
        // C s_550_5: const #5s : i
        let s_550_5: i128 = 5;
        // C s_550_6: add s_550_5 s_550_4
        let s_550_6: i128 = (s_550_5 + s_550_4);
        // D s_550_7: bit-extract s_550_2 s_550_0 s_550_6
        let s_550_7: Bits = (Bits::new(
            ((s_550_2) >> (s_550_0)).value(),
            u16::try_from(s_550_6).unwrap(),
        ));
        // D s_550_8: cast reint s_550_7 -> u8
        let s_550_8: u8 = (s_550_7.value() as u8);
        // D s_550_9: cast zx s_550_8 -> bv
        let s_550_9: Bits = Bits::new(s_550_8 as u128, 6u16);
        // C s_550_10: const #9u : u8
        let s_550_10: u8 = 9;
        // C s_550_11: cast zx s_550_10 -> bv
        let s_550_11: Bits = Bits::new(s_550_10 as u128, 6u16);
        // D s_550_12: cmp-eq s_550_9 s_550_11
        let s_550_12: bool = ((s_550_9) == (s_550_11));
        // D s_550_13: write-var gs#382671 <= s_550_12
        fn_state.gs_382671 = s_550_12;
        // N s_550_14: jump b130
        return block_130(state, tracer, fn_state);
    }
    fn block_551<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_551_0: const #57s : i
        let s_551_0: i128 = 57;
        // C s_551_1: const #14696u : u32
        let s_551_1: u32 = 14696;
        // D s_551_2: read-reg s_551_1:i
        let s_551_2: i128 = {
            let value = state.read_register::<i128>(s_551_1 as isize);
            tracer.read_register(s_551_1 as isize, value);
            value
        };
        // D s_551_3: cmp-lt s_551_2 s_551_0
        let s_551_3: bool = ((s_551_2) < (s_551_0));
        // D s_551_4: write-var gs#382653 <= s_551_3
        fn_state.gs_382653 = s_551_3;
        // N s_551_5: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_552<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_552_0: const #10s : i
        let s_552_0: i128 = 10;
        // D s_552_1: read-var u#25869:u32
        let s_552_1: u32 = fn_state.u_25869;
        // D s_552_2: cast zx s_552_1 -> bv
        let s_552_2: Bits = Bits::new(s_552_1 as u128, 32u16);
        // C s_552_3: const #1s : i64
        let s_552_3: i64 = 1;
        // C s_552_4: cast zx s_552_3 -> i
        let s_552_4: i128 = (i128::try_from(s_552_3).unwrap());
        // C s_552_5: const #5s : i
        let s_552_5: i128 = 5;
        // C s_552_6: add s_552_5 s_552_4
        let s_552_6: i128 = (s_552_5 + s_552_4);
        // D s_552_7: bit-extract s_552_2 s_552_0 s_552_6
        let s_552_7: Bits = (Bits::new(
            ((s_552_2) >> (s_552_0)).value(),
            u16::try_from(s_552_6).unwrap(),
        ));
        // D s_552_8: cast reint s_552_7 -> u8
        let s_552_8: u8 = (s_552_7.value() as u8);
        // D s_552_9: cast zx s_552_8 -> bv
        let s_552_9: Bits = Bits::new(s_552_8 as u128, 6u16);
        // C s_552_10: const #8u : u8
        let s_552_10: u8 = 8;
        // C s_552_11: cast zx s_552_10 -> bv
        let s_552_11: Bits = Bits::new(s_552_10 as u128, 6u16);
        // D s_552_12: cmp-eq s_552_9 s_552_11
        let s_552_12: bool = ((s_552_9) == (s_552_11));
        // D s_552_13: write-var gs#382651 <= s_552_12
        fn_state.gs_382651 = s_552_12;
        // N s_552_14: jump b124
        return block_124(state, tracer, fn_state);
    }
    fn block_553<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_553_0: const #56s : i
        let s_553_0: i128 = 56;
        // C s_553_1: const #14696u : u32
        let s_553_1: u32 = 14696;
        // D s_553_2: read-reg s_553_1:i
        let s_553_2: i128 = {
            let value = state.read_register::<i128>(s_553_1 as isize);
            tracer.read_register(s_553_1 as isize, value);
            value
        };
        // D s_553_3: cmp-lt s_553_2 s_553_0
        let s_553_3: bool = ((s_553_2) < (s_553_0));
        // D s_553_4: write-var gs#382633 <= s_553_3
        fn_state.gs_382633 = s_553_3;
        // N s_553_5: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_554<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_554_0: const #10s : i
        let s_554_0: i128 = 10;
        // D s_554_1: read-var u#25863:u32
        let s_554_1: u32 = fn_state.u_25863;
        // D s_554_2: cast zx s_554_1 -> bv
        let s_554_2: Bits = Bits::new(s_554_1 as u128, 32u16);
        // C s_554_3: const #1s : i64
        let s_554_3: i64 = 1;
        // C s_554_4: cast zx s_554_3 -> i
        let s_554_4: i128 = (i128::try_from(s_554_3).unwrap());
        // C s_554_5: const #5s : i
        let s_554_5: i128 = 5;
        // C s_554_6: add s_554_5 s_554_4
        let s_554_6: i128 = (s_554_5 + s_554_4);
        // D s_554_7: bit-extract s_554_2 s_554_0 s_554_6
        let s_554_7: Bits = (Bits::new(
            ((s_554_2) >> (s_554_0)).value(),
            u16::try_from(s_554_6).unwrap(),
        ));
        // D s_554_8: cast reint s_554_7 -> u8
        let s_554_8: u8 = (s_554_7.value() as u8);
        // D s_554_9: cast zx s_554_8 -> bv
        let s_554_9: Bits = Bits::new(s_554_8 as u128, 6u16);
        // C s_554_10: const #10u : u8
        let s_554_10: u8 = 10;
        // C s_554_11: cast zx s_554_10 -> bv
        let s_554_11: Bits = Bits::new(s_554_10 as u128, 6u16);
        // D s_554_12: cmp-eq s_554_9 s_554_11
        let s_554_12: bool = ((s_554_9) == (s_554_11));
        // D s_554_13: write-var gs#382631 <= s_554_12
        fn_state.gs_382631 = s_554_12;
        // N s_554_14: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_555<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_555_0: const #55s : i
        let s_555_0: i128 = 55;
        // C s_555_1: const #14696u : u32
        let s_555_1: u32 = 14696;
        // D s_555_2: read-reg s_555_1:i
        let s_555_2: i128 = {
            let value = state.read_register::<i128>(s_555_1 as isize);
            tracer.read_register(s_555_1 as isize, value);
            value
        };
        // D s_555_3: cmp-lt s_555_2 s_555_0
        let s_555_3: bool = ((s_555_2) < (s_555_0));
        // D s_555_4: write-var gs#382607 <= s_555_3
        fn_state.gs_382607 = s_555_3;
        // N s_555_5: jump b114
        return block_114(state, tracer, fn_state);
    }
    fn block_556<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_556_0: const #21s : i
        let s_556_0: i128 = 21;
        // D s_556_1: read-var u#25853:u32
        let s_556_1: u32 = fn_state.u_25853;
        // D s_556_2: cast zx s_556_1 -> bv
        let s_556_2: Bits = Bits::new(s_556_1 as u128, 32u16);
        // C s_556_3: const #1s : i64
        let s_556_3: i64 = 1;
        // C s_556_4: cast zx s_556_3 -> i
        let s_556_4: i128 = (i128::try_from(s_556_3).unwrap());
        // C s_556_5: const #0s : i
        let s_556_5: i128 = 0;
        // C s_556_6: add s_556_5 s_556_4
        let s_556_6: i128 = (s_556_5 + s_556_4);
        // D s_556_7: bit-extract s_556_2 s_556_0 s_556_6
        let s_556_7: Bits = (Bits::new(
            ((s_556_2) >> (s_556_0)).value(),
            u16::try_from(s_556_6).unwrap(),
        ));
        // D s_556_8: cast reint s_556_7 -> u8
        let s_556_8: bool = ((s_556_7.value()) != 0);
        // D s_556_9: cast zx s_556_8 -> bv
        let s_556_9: Bits = Bits::new(s_556_8 as u128, 1u16);
        // C s_556_10: const #0u : u8
        let s_556_10: bool = false;
        // C s_556_11: cast zx s_556_10 -> bv
        let s_556_11: Bits = Bits::new(s_556_10 as u128, 1u16);
        // D s_556_12: cmp-eq s_556_9 s_556_11
        let s_556_12: bool = ((s_556_9) == (s_556_11));
        // D s_556_13: write-var gs#382605 <= s_556_12
        fn_state.gs_382605 = s_556_12;
        // N s_556_14: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_557<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_557_0: const #54s : i
        let s_557_0: i128 = 54;
        // C s_557_1: const #14696u : u32
        let s_557_1: u32 = 14696;
        // D s_557_2: read-reg s_557_1:i
        let s_557_2: i128 = {
            let value = state.read_register::<i128>(s_557_1 as isize);
            tracer.read_register(s_557_1 as isize, value);
            value
        };
        // D s_557_3: cmp-lt s_557_2 s_557_0
        let s_557_3: bool = ((s_557_2) < (s_557_0));
        // D s_557_4: write-var gs#382581 <= s_557_3
        fn_state.gs_382581 = s_557_3;
        // N s_557_5: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_558<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_558_0: const #21s : i
        let s_558_0: i128 = 21;
        // D s_558_1: read-var u#25843:u32
        let s_558_1: u32 = fn_state.u_25843;
        // D s_558_2: cast zx s_558_1 -> bv
        let s_558_2: Bits = Bits::new(s_558_1 as u128, 32u16);
        // C s_558_3: const #1s : i64
        let s_558_3: i64 = 1;
        // C s_558_4: cast zx s_558_3 -> i
        let s_558_4: i128 = (i128::try_from(s_558_3).unwrap());
        // C s_558_5: const #0s : i
        let s_558_5: i128 = 0;
        // C s_558_6: add s_558_5 s_558_4
        let s_558_6: i128 = (s_558_5 + s_558_4);
        // D s_558_7: bit-extract s_558_2 s_558_0 s_558_6
        let s_558_7: Bits = (Bits::new(
            ((s_558_2) >> (s_558_0)).value(),
            u16::try_from(s_558_6).unwrap(),
        ));
        // D s_558_8: cast reint s_558_7 -> u8
        let s_558_8: bool = ((s_558_7.value()) != 0);
        // D s_558_9: cast zx s_558_8 -> bv
        let s_558_9: Bits = Bits::new(s_558_8 as u128, 1u16);
        // C s_558_10: const #1u : u8
        let s_558_10: bool = true;
        // C s_558_11: cast zx s_558_10 -> bv
        let s_558_11: Bits = Bits::new(s_558_10 as u128, 1u16);
        // D s_558_12: cmp-eq s_558_9 s_558_11
        let s_558_12: bool = ((s_558_9) == (s_558_11));
        // D s_558_13: write-var gs#382579 <= s_558_12
        fn_state.gs_382579 = s_558_12;
        // N s_558_14: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_559<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_559_0: const #53s : i
        let s_559_0: i128 = 53;
        // C s_559_1: const #14696u : u32
        let s_559_1: u32 = 14696;
        // D s_559_2: read-reg s_559_1:i
        let s_559_2: i128 = {
            let value = state.read_register::<i128>(s_559_1 as isize);
            tracer.read_register(s_559_1 as isize, value);
            value
        };
        // D s_559_3: cmp-lt s_559_2 s_559_0
        let s_559_3: bool = ((s_559_2) < (s_559_0));
        // D s_559_4: write-var gs#382555 <= s_559_3
        fn_state.gs_382555 = s_559_3;
        // N s_559_5: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_560<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_560_0: const #21s : i
        let s_560_0: i128 = 21;
        // D s_560_1: read-var u#25833:u32
        let s_560_1: u32 = fn_state.u_25833;
        // D s_560_2: cast zx s_560_1 -> bv
        let s_560_2: Bits = Bits::new(s_560_1 as u128, 32u16);
        // C s_560_3: const #1s : i64
        let s_560_3: i64 = 1;
        // C s_560_4: cast zx s_560_3 -> i
        let s_560_4: i128 = (i128::try_from(s_560_3).unwrap());
        // C s_560_5: const #0s : i
        let s_560_5: i128 = 0;
        // C s_560_6: add s_560_5 s_560_4
        let s_560_6: i128 = (s_560_5 + s_560_4);
        // D s_560_7: bit-extract s_560_2 s_560_0 s_560_6
        let s_560_7: Bits = (Bits::new(
            ((s_560_2) >> (s_560_0)).value(),
            u16::try_from(s_560_6).unwrap(),
        ));
        // D s_560_8: cast reint s_560_7 -> u8
        let s_560_8: bool = ((s_560_7.value()) != 0);
        // D s_560_9: cast zx s_560_8 -> bv
        let s_560_9: Bits = Bits::new(s_560_8 as u128, 1u16);
        // C s_560_10: const #0u : u8
        let s_560_10: bool = false;
        // C s_560_11: cast zx s_560_10 -> bv
        let s_560_11: Bits = Bits::new(s_560_10 as u128, 1u16);
        // D s_560_12: cmp-eq s_560_9 s_560_11
        let s_560_12: bool = ((s_560_9) == (s_560_11));
        // D s_560_13: write-var gs#382553 <= s_560_12
        fn_state.gs_382553 = s_560_12;
        // N s_560_14: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_561<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_561_0: const #52s : i
        let s_561_0: i128 = 52;
        // C s_561_1: const #14696u : u32
        let s_561_1: u32 = 14696;
        // D s_561_2: read-reg s_561_1:i
        let s_561_2: i128 = {
            let value = state.read_register::<i128>(s_561_1 as isize);
            tracer.read_register(s_561_1 as isize, value);
            value
        };
        // D s_561_3: cmp-lt s_561_2 s_561_0
        let s_561_3: bool = ((s_561_2) < (s_561_0));
        // D s_561_4: write-var gs#382529 <= s_561_3
        fn_state.gs_382529 = s_561_3;
        // N s_561_5: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_562<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_562_0: const #21s : i
        let s_562_0: i128 = 21;
        // D s_562_1: read-var u#25823:u32
        let s_562_1: u32 = fn_state.u_25823;
        // D s_562_2: cast zx s_562_1 -> bv
        let s_562_2: Bits = Bits::new(s_562_1 as u128, 32u16);
        // C s_562_3: const #1s : i64
        let s_562_3: i64 = 1;
        // C s_562_4: cast zx s_562_3 -> i
        let s_562_4: i128 = (i128::try_from(s_562_3).unwrap());
        // C s_562_5: const #0s : i
        let s_562_5: i128 = 0;
        // C s_562_6: add s_562_5 s_562_4
        let s_562_6: i128 = (s_562_5 + s_562_4);
        // D s_562_7: bit-extract s_562_2 s_562_0 s_562_6
        let s_562_7: Bits = (Bits::new(
            ((s_562_2) >> (s_562_0)).value(),
            u16::try_from(s_562_6).unwrap(),
        ));
        // D s_562_8: cast reint s_562_7 -> u8
        let s_562_8: bool = ((s_562_7.value()) != 0);
        // D s_562_9: cast zx s_562_8 -> bv
        let s_562_9: Bits = Bits::new(s_562_8 as u128, 1u16);
        // C s_562_10: const #1u : u8
        let s_562_10: bool = true;
        // C s_562_11: cast zx s_562_10 -> bv
        let s_562_11: Bits = Bits::new(s_562_10 as u128, 1u16);
        // D s_562_12: cmp-eq s_562_9 s_562_11
        let s_562_12: bool = ((s_562_9) == (s_562_11));
        // D s_562_13: write-var gs#382527 <= s_562_12
        fn_state.gs_382527 = s_562_12;
        // N s_562_14: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_563<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_563_0: const #51s : i
        let s_563_0: i128 = 51;
        // C s_563_1: const #14696u : u32
        let s_563_1: u32 = 14696;
        // D s_563_2: read-reg s_563_1:i
        let s_563_2: i128 = {
            let value = state.read_register::<i128>(s_563_1 as isize);
            tracer.read_register(s_563_1 as isize, value);
            value
        };
        // D s_563_3: cmp-lt s_563_2 s_563_0
        let s_563_3: bool = ((s_563_2) < (s_563_0));
        // D s_563_4: write-var gs#382503 <= s_563_3
        fn_state.gs_382503 = s_563_3;
        // N s_563_5: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_564<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_564_0: const #21s : i
        let s_564_0: i128 = 21;
        // D s_564_1: read-var u#25813:u32
        let s_564_1: u32 = fn_state.u_25813;
        // D s_564_2: cast zx s_564_1 -> bv
        let s_564_2: Bits = Bits::new(s_564_1 as u128, 32u16);
        // C s_564_3: const #1s : i64
        let s_564_3: i64 = 1;
        // C s_564_4: cast zx s_564_3 -> i
        let s_564_4: i128 = (i128::try_from(s_564_3).unwrap());
        // C s_564_5: const #0s : i
        let s_564_5: i128 = 0;
        // C s_564_6: add s_564_5 s_564_4
        let s_564_6: i128 = (s_564_5 + s_564_4);
        // D s_564_7: bit-extract s_564_2 s_564_0 s_564_6
        let s_564_7: Bits = (Bits::new(
            ((s_564_2) >> (s_564_0)).value(),
            u16::try_from(s_564_6).unwrap(),
        ));
        // D s_564_8: cast reint s_564_7 -> u8
        let s_564_8: bool = ((s_564_7.value()) != 0);
        // D s_564_9: cast zx s_564_8 -> bv
        let s_564_9: Bits = Bits::new(s_564_8 as u128, 1u16);
        // C s_564_10: const #1u : u8
        let s_564_10: bool = true;
        // C s_564_11: cast zx s_564_10 -> bv
        let s_564_11: Bits = Bits::new(s_564_10 as u128, 1u16);
        // D s_564_12: cmp-eq s_564_9 s_564_11
        let s_564_12: bool = ((s_564_9) == (s_564_11));
        // D s_564_13: write-var gs#382501 <= s_564_12
        fn_state.gs_382501 = s_564_12;
        // N s_564_14: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_565<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_565_0: const #50s : i
        let s_565_0: i128 = 50;
        // C s_565_1: const #14696u : u32
        let s_565_1: u32 = 14696;
        // D s_565_2: read-reg s_565_1:i
        let s_565_2: i128 = {
            let value = state.read_register::<i128>(s_565_1 as isize);
            tracer.read_register(s_565_1 as isize, value);
            value
        };
        // D s_565_3: cmp-lt s_565_2 s_565_0
        let s_565_3: bool = ((s_565_2) < (s_565_0));
        // D s_565_4: write-var gs#382477 <= s_565_3
        fn_state.gs_382477 = s_565_3;
        // N s_565_5: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_566<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_566_0: const #21s : i
        let s_566_0: i128 = 21;
        // D s_566_1: read-var u#25803:u32
        let s_566_1: u32 = fn_state.u_25803;
        // D s_566_2: cast zx s_566_1 -> bv
        let s_566_2: Bits = Bits::new(s_566_1 as u128, 32u16);
        // C s_566_3: const #1s : i64
        let s_566_3: i64 = 1;
        // C s_566_4: cast zx s_566_3 -> i
        let s_566_4: i128 = (i128::try_from(s_566_3).unwrap());
        // C s_566_5: const #0s : i
        let s_566_5: i128 = 0;
        // C s_566_6: add s_566_5 s_566_4
        let s_566_6: i128 = (s_566_5 + s_566_4);
        // D s_566_7: bit-extract s_566_2 s_566_0 s_566_6
        let s_566_7: Bits = (Bits::new(
            ((s_566_2) >> (s_566_0)).value(),
            u16::try_from(s_566_6).unwrap(),
        ));
        // D s_566_8: cast reint s_566_7 -> u8
        let s_566_8: bool = ((s_566_7.value()) != 0);
        // D s_566_9: cast zx s_566_8 -> bv
        let s_566_9: Bits = Bits::new(s_566_8 as u128, 1u16);
        // C s_566_10: const #1u : u8
        let s_566_10: bool = true;
        // C s_566_11: cast zx s_566_10 -> bv
        let s_566_11: Bits = Bits::new(s_566_10 as u128, 1u16);
        // D s_566_12: cmp-eq s_566_9 s_566_11
        let s_566_12: bool = ((s_566_9) == (s_566_11));
        // D s_566_13: write-var gs#382475 <= s_566_12
        fn_state.gs_382475 = s_566_12;
        // N s_566_14: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_567<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_567_0: const #49s : i
        let s_567_0: i128 = 49;
        // C s_567_1: const #14696u : u32
        let s_567_1: u32 = 14696;
        // D s_567_2: read-reg s_567_1:i
        let s_567_2: i128 = {
            let value = state.read_register::<i128>(s_567_1 as isize);
            tracer.read_register(s_567_1 as isize, value);
            value
        };
        // D s_567_3: cmp-lt s_567_2 s_567_0
        let s_567_3: bool = ((s_567_2) < (s_567_0));
        // D s_567_4: write-var gs#382451 <= s_567_3
        fn_state.gs_382451 = s_567_3;
        // N s_567_5: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_568<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_568_0: const #21s : i
        let s_568_0: i128 = 21;
        // D s_568_1: read-var u#25793:u32
        let s_568_1: u32 = fn_state.u_25793;
        // D s_568_2: cast zx s_568_1 -> bv
        let s_568_2: Bits = Bits::new(s_568_1 as u128, 32u16);
        // C s_568_3: const #1s : i64
        let s_568_3: i64 = 1;
        // C s_568_4: cast zx s_568_3 -> i
        let s_568_4: i128 = (i128::try_from(s_568_3).unwrap());
        // C s_568_5: const #0s : i
        let s_568_5: i128 = 0;
        // C s_568_6: add s_568_5 s_568_4
        let s_568_6: i128 = (s_568_5 + s_568_4);
        // D s_568_7: bit-extract s_568_2 s_568_0 s_568_6
        let s_568_7: Bits = (Bits::new(
            ((s_568_2) >> (s_568_0)).value(),
            u16::try_from(s_568_6).unwrap(),
        ));
        // D s_568_8: cast reint s_568_7 -> u8
        let s_568_8: bool = ((s_568_7.value()) != 0);
        // D s_568_9: cast zx s_568_8 -> bv
        let s_568_9: Bits = Bits::new(s_568_8 as u128, 1u16);
        // C s_568_10: const #0u : u8
        let s_568_10: bool = false;
        // C s_568_11: cast zx s_568_10 -> bv
        let s_568_11: Bits = Bits::new(s_568_10 as u128, 1u16);
        // D s_568_12: cmp-eq s_568_9 s_568_11
        let s_568_12: bool = ((s_568_9) == (s_568_11));
        // D s_568_13: write-var gs#382449 <= s_568_12
        fn_state.gs_382449 = s_568_12;
        // N s_568_14: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_569<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_569_0: const #48s : i
        let s_569_0: i128 = 48;
        // C s_569_1: const #14696u : u32
        let s_569_1: u32 = 14696;
        // D s_569_2: read-reg s_569_1:i
        let s_569_2: i128 = {
            let value = state.read_register::<i128>(s_569_1 as isize);
            tracer.read_register(s_569_1 as isize, value);
            value
        };
        // D s_569_3: cmp-lt s_569_2 s_569_0
        let s_569_3: bool = ((s_569_2) < (s_569_0));
        // D s_569_4: write-var gs#382425 <= s_569_3
        fn_state.gs_382425 = s_569_3;
        // N s_569_5: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_570<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_570_0: const #21s : i
        let s_570_0: i128 = 21;
        // D s_570_1: read-var u#25785:u32
        let s_570_1: u32 = fn_state.u_25785;
        // D s_570_2: cast zx s_570_1 -> bv
        let s_570_2: Bits = Bits::new(s_570_1 as u128, 32u16);
        // C s_570_3: const #1s : i64
        let s_570_3: i64 = 1;
        // C s_570_4: cast zx s_570_3 -> i
        let s_570_4: i128 = (i128::try_from(s_570_3).unwrap());
        // C s_570_5: const #0s : i
        let s_570_5: i128 = 0;
        // C s_570_6: add s_570_5 s_570_4
        let s_570_6: i128 = (s_570_5 + s_570_4);
        // D s_570_7: bit-extract s_570_2 s_570_0 s_570_6
        let s_570_7: Bits = (Bits::new(
            ((s_570_2) >> (s_570_0)).value(),
            u16::try_from(s_570_6).unwrap(),
        ));
        // D s_570_8: cast reint s_570_7 -> u8
        let s_570_8: bool = ((s_570_7.value()) != 0);
        // D s_570_9: cast zx s_570_8 -> bv
        let s_570_9: Bits = Bits::new(s_570_8 as u128, 1u16);
        // C s_570_10: const #0u : u8
        let s_570_10: bool = false;
        // C s_570_11: cast zx s_570_10 -> bv
        let s_570_11: Bits = Bits::new(s_570_10 as u128, 1u16);
        // D s_570_12: cmp-eq s_570_9 s_570_11
        let s_570_12: bool = ((s_570_9) == (s_570_11));
        // D s_570_13: write-var gs#382423 <= s_570_12
        fn_state.gs_382423 = s_570_12;
        // N s_570_14: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_571<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_571_0: const #25s : i
        let s_571_0: i128 = 25;
        // C s_571_1: const #14696u : u32
        let s_571_1: u32 = 14696;
        // D s_571_2: read-reg s_571_1:i
        let s_571_2: i128 = {
            let value = state.read_register::<i128>(s_571_1 as isize);
            tracer.read_register(s_571_1 as isize, value);
            value
        };
        // D s_571_3: cmp-lt s_571_2 s_571_0
        let s_571_3: bool = ((s_571_2) < (s_571_0));
        // D s_571_4: write-var gs#382399 <= s_571_3
        fn_state.gs_382399 = s_571_3;
        // N s_571_5: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_572<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_572_0: const #24s : i
        let s_572_0: i128 = 24;
        // C s_572_1: const #14696u : u32
        let s_572_1: u32 = 14696;
        // D s_572_2: read-reg s_572_1:i
        let s_572_2: i128 = {
            let value = state.read_register::<i128>(s_572_1 as isize);
            tracer.read_register(s_572_1 as isize, value);
            value
        };
        // D s_572_3: cmp-lt s_572_2 s_572_0
        let s_572_3: bool = ((s_572_2) < (s_572_0));
        // D s_572_4: write-var gs#382376 <= s_572_3
        fn_state.gs_382376 = s_572_3;
        // N s_572_5: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_573<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_573_0: const #23s : i
        let s_573_0: i128 = 23;
        // C s_573_1: const #14696u : u32
        let s_573_1: u32 = 14696;
        // D s_573_2: read-reg s_573_1:i
        let s_573_2: i128 = {
            let value = state.read_register::<i128>(s_573_1 as isize);
            tracer.read_register(s_573_1 as isize, value);
            value
        };
        // D s_573_3: cmp-lt s_573_2 s_573_0
        let s_573_3: bool = ((s_573_2) < (s_573_0));
        // D s_573_4: write-var gs#382353 <= s_573_3
        fn_state.gs_382353 = s_573_3;
        // N s_573_5: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_574<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_574_0: const #22s : i
        let s_574_0: i128 = 22;
        // C s_574_1: const #14696u : u32
        let s_574_1: u32 = 14696;
        // D s_574_2: read-reg s_574_1:i
        let s_574_2: i128 = {
            let value = state.read_register::<i128>(s_574_1 as isize);
            tracer.read_register(s_574_1 as isize, value);
            value
        };
        // D s_574_3: cmp-lt s_574_2 s_574_0
        let s_574_3: bool = ((s_574_2) < (s_574_0));
        // D s_574_4: write-var gs#382330 <= s_574_3
        fn_state.gs_382330 = s_574_3;
        // N s_574_5: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_575<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_575_0: const #17s : i
        let s_575_0: i128 = 17;
        // C s_575_1: const #14696u : u32
        let s_575_1: u32 = 14696;
        // D s_575_2: read-reg s_575_1:i
        let s_575_2: i128 = {
            let value = state.read_register::<i128>(s_575_1 as isize);
            tracer.read_register(s_575_1 as isize, value);
            value
        };
        // D s_575_3: cmp-lt s_575_2 s_575_0
        let s_575_3: bool = ((s_575_2) < (s_575_0));
        // D s_575_4: write-var gs#382307 <= s_575_3
        fn_state.gs_382307 = s_575_3;
        // N s_575_5: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_576<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_576_0: const #21s : i
        let s_576_0: i128 = 21;
        // D s_576_1: read-var u#25737:u32
        let s_576_1: u32 = fn_state.u_25737;
        // D s_576_2: cast zx s_576_1 -> bv
        let s_576_2: Bits = Bits::new(s_576_1 as u128, 32u16);
        // C s_576_3: const #1s : i64
        let s_576_3: i64 = 1;
        // C s_576_4: cast zx s_576_3 -> i
        let s_576_4: i128 = (i128::try_from(s_576_3).unwrap());
        // C s_576_5: const #0s : i
        let s_576_5: i128 = 0;
        // C s_576_6: add s_576_5 s_576_4
        let s_576_6: i128 = (s_576_5 + s_576_4);
        // D s_576_7: bit-extract s_576_2 s_576_0 s_576_6
        let s_576_7: Bits = (Bits::new(
            ((s_576_2) >> (s_576_0)).value(),
            u16::try_from(s_576_6).unwrap(),
        ));
        // D s_576_8: cast reint s_576_7 -> u8
        let s_576_8: bool = ((s_576_7.value()) != 0);
        // D s_576_9: cast zx s_576_8 -> bv
        let s_576_9: Bits = Bits::new(s_576_8 as u128, 1u16);
        // C s_576_10: const #0u : u8
        let s_576_10: bool = false;
        // C s_576_11: cast zx s_576_10 -> bv
        let s_576_11: Bits = Bits::new(s_576_10 as u128, 1u16);
        // D s_576_12: cmp-eq s_576_9 s_576_11
        let s_576_12: bool = ((s_576_9) == (s_576_11));
        // D s_576_13: write-var gs#382305 <= s_576_12
        fn_state.gs_382305 = s_576_12;
        // N s_576_14: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_577<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_577_0: const #16s : i
        let s_577_0: i128 = 16;
        // C s_577_1: const #14696u : u32
        let s_577_1: u32 = 14696;
        // D s_577_2: read-reg s_577_1:i
        let s_577_2: i128 = {
            let value = state.read_register::<i128>(s_577_1 as isize);
            tracer.read_register(s_577_1 as isize, value);
            value
        };
        // D s_577_3: cmp-lt s_577_2 s_577_0
        let s_577_3: bool = ((s_577_2) < (s_577_0));
        // D s_577_4: write-var gs#382281 <= s_577_3
        fn_state.gs_382281 = s_577_3;
        // N s_577_5: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_578<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_578_0: const #21s : i
        let s_578_0: i128 = 21;
        // D s_578_1: read-var u#25727:u32
        let s_578_1: u32 = fn_state.u_25727;
        // D s_578_2: cast zx s_578_1 -> bv
        let s_578_2: Bits = Bits::new(s_578_1 as u128, 32u16);
        // C s_578_3: const #1s : i64
        let s_578_3: i64 = 1;
        // C s_578_4: cast zx s_578_3 -> i
        let s_578_4: i128 = (i128::try_from(s_578_3).unwrap());
        // C s_578_5: const #0s : i
        let s_578_5: i128 = 0;
        // C s_578_6: add s_578_5 s_578_4
        let s_578_6: i128 = (s_578_5 + s_578_4);
        // D s_578_7: bit-extract s_578_2 s_578_0 s_578_6
        let s_578_7: Bits = (Bits::new(
            ((s_578_2) >> (s_578_0)).value(),
            u16::try_from(s_578_6).unwrap(),
        ));
        // D s_578_8: cast reint s_578_7 -> u8
        let s_578_8: bool = ((s_578_7.value()) != 0);
        // D s_578_9: cast zx s_578_8 -> bv
        let s_578_9: Bits = Bits::new(s_578_8 as u128, 1u16);
        // C s_578_10: const #0u : u8
        let s_578_10: bool = false;
        // C s_578_11: cast zx s_578_10 -> bv
        let s_578_11: Bits = Bits::new(s_578_10 as u128, 1u16);
        // D s_578_12: cmp-eq s_578_9 s_578_11
        let s_578_12: bool = ((s_578_9) == (s_578_11));
        // D s_578_13: write-var gs#382279 <= s_578_12
        fn_state.gs_382279 = s_578_12;
        // N s_578_14: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_579<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_579_0: const #15s : i
        let s_579_0: i128 = 15;
        // C s_579_1: const #14696u : u32
        let s_579_1: u32 = 14696;
        // D s_579_2: read-reg s_579_1:i
        let s_579_2: i128 = {
            let value = state.read_register::<i128>(s_579_1 as isize);
            tracer.read_register(s_579_1 as isize, value);
            value
        };
        // D s_579_3: cmp-lt s_579_2 s_579_0
        let s_579_3: bool = ((s_579_2) < (s_579_0));
        // D s_579_4: write-var gs#382255 <= s_579_3
        fn_state.gs_382255 = s_579_3;
        // N s_579_5: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_580<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_580_0: const #21s : i
        let s_580_0: i128 = 21;
        // D s_580_1: read-var u#25717:u32
        let s_580_1: u32 = fn_state.u_25717;
        // D s_580_2: cast zx s_580_1 -> bv
        let s_580_2: Bits = Bits::new(s_580_1 as u128, 32u16);
        // C s_580_3: const #1s : i64
        let s_580_3: i64 = 1;
        // C s_580_4: cast zx s_580_3 -> i
        let s_580_4: i128 = (i128::try_from(s_580_3).unwrap());
        // C s_580_5: const #0s : i
        let s_580_5: i128 = 0;
        // C s_580_6: add s_580_5 s_580_4
        let s_580_6: i128 = (s_580_5 + s_580_4);
        // D s_580_7: bit-extract s_580_2 s_580_0 s_580_6
        let s_580_7: Bits = (Bits::new(
            ((s_580_2) >> (s_580_0)).value(),
            u16::try_from(s_580_6).unwrap(),
        ));
        // D s_580_8: cast reint s_580_7 -> u8
        let s_580_8: bool = ((s_580_7.value()) != 0);
        // D s_580_9: cast zx s_580_8 -> bv
        let s_580_9: Bits = Bits::new(s_580_8 as u128, 1u16);
        // C s_580_10: const #0u : u8
        let s_580_10: bool = false;
        // C s_580_11: cast zx s_580_10 -> bv
        let s_580_11: Bits = Bits::new(s_580_10 as u128, 1u16);
        // D s_580_12: cmp-eq s_580_9 s_580_11
        let s_580_12: bool = ((s_580_9) == (s_580_11));
        // D s_580_13: write-var gs#382253 <= s_580_12
        fn_state.gs_382253 = s_580_12;
        // N s_580_14: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_581<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_581_0: const #14s : i
        let s_581_0: i128 = 14;
        // C s_581_1: const #14696u : u32
        let s_581_1: u32 = 14696;
        // D s_581_2: read-reg s_581_1:i
        let s_581_2: i128 = {
            let value = state.read_register::<i128>(s_581_1 as isize);
            tracer.read_register(s_581_1 as isize, value);
            value
        };
        // D s_581_3: cmp-lt s_581_2 s_581_0
        let s_581_3: bool = ((s_581_2) < (s_581_0));
        // D s_581_4: write-var gs#382229 <= s_581_3
        fn_state.gs_382229 = s_581_3;
        // N s_581_5: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_582<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_582_0: const #21s : i
        let s_582_0: i128 = 21;
        // D s_582_1: read-var u#25709:u32
        let s_582_1: u32 = fn_state.u_25709;
        // D s_582_2: cast zx s_582_1 -> bv
        let s_582_2: Bits = Bits::new(s_582_1 as u128, 32u16);
        // C s_582_3: const #1s : i64
        let s_582_3: i64 = 1;
        // C s_582_4: cast zx s_582_3 -> i
        let s_582_4: i128 = (i128::try_from(s_582_3).unwrap());
        // C s_582_5: const #0s : i
        let s_582_5: i128 = 0;
        // C s_582_6: add s_582_5 s_582_4
        let s_582_6: i128 = (s_582_5 + s_582_4);
        // D s_582_7: bit-extract s_582_2 s_582_0 s_582_6
        let s_582_7: Bits = (Bits::new(
            ((s_582_2) >> (s_582_0)).value(),
            u16::try_from(s_582_6).unwrap(),
        ));
        // D s_582_8: cast reint s_582_7 -> u8
        let s_582_8: bool = ((s_582_7.value()) != 0);
        // D s_582_9: cast zx s_582_8 -> bv
        let s_582_9: Bits = Bits::new(s_582_8 as u128, 1u16);
        // C s_582_10: const #0u : u8
        let s_582_10: bool = false;
        // C s_582_11: cast zx s_582_10 -> bv
        let s_582_11: Bits = Bits::new(s_582_10 as u128, 1u16);
        // D s_582_12: cmp-eq s_582_9 s_582_11
        let s_582_12: bool = ((s_582_9) == (s_582_11));
        // D s_582_13: write-var gs#382227 <= s_582_12
        fn_state.gs_382227 = s_582_12;
        // N s_582_14: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_583<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_583_0: const #9s : i
        let s_583_0: i128 = 9;
        // C s_583_1: const #14696u : u32
        let s_583_1: u32 = 14696;
        // D s_583_2: read-reg s_583_1:i
        let s_583_2: i128 = {
            let value = state.read_register::<i128>(s_583_1 as isize);
            tracer.read_register(s_583_1 as isize, value);
            value
        };
        // D s_583_3: cmp-lt s_583_2 s_583_0
        let s_583_3: bool = ((s_583_2) < (s_583_0));
        // D s_583_4: write-var gs#382207 <= s_583_3
        fn_state.gs_382207 = s_583_3;
        // N s_583_5: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_584<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_584_0: const #10s : i
        let s_584_0: i128 = 10;
        // D s_584_1: read-var u#25701:u32
        let s_584_1: u32 = fn_state.u_25701;
        // D s_584_2: cast zx s_584_1 -> bv
        let s_584_2: Bits = Bits::new(s_584_1 as u128, 32u16);
        // C s_584_3: const #1s : i64
        let s_584_3: i64 = 1;
        // C s_584_4: cast zx s_584_3 -> i
        let s_584_4: i128 = (i128::try_from(s_584_3).unwrap());
        // C s_584_5: const #5s : i
        let s_584_5: i128 = 5;
        // C s_584_6: add s_584_5 s_584_4
        let s_584_6: i128 = (s_584_5 + s_584_4);
        // D s_584_7: bit-extract s_584_2 s_584_0 s_584_6
        let s_584_7: Bits = (Bits::new(
            ((s_584_2) >> (s_584_0)).value(),
            u16::try_from(s_584_6).unwrap(),
        ));
        // D s_584_8: cast reint s_584_7 -> u8
        let s_584_8: u8 = (s_584_7.value() as u8);
        // D s_584_9: cast zx s_584_8 -> bv
        let s_584_9: Bits = Bits::new(s_584_8 as u128, 6u16);
        // C s_584_10: const #0u : u8
        let s_584_10: u8 = 0;
        // C s_584_11: cast zx s_584_10 -> bv
        let s_584_11: Bits = Bits::new(s_584_10 as u128, 6u16);
        // D s_584_12: cmp-eq s_584_9 s_584_11
        let s_584_12: bool = ((s_584_9) == (s_584_11));
        // D s_584_13: write-var gs#382205 <= s_584_12
        fn_state.gs_382205 = s_584_12;
        // N s_584_14: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_585<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_585_0: const #8s : i
        let s_585_0: i128 = 8;
        // C s_585_1: const #14696u : u32
        let s_585_1: u32 = 14696;
        // D s_585_2: read-reg s_585_1:i
        let s_585_2: i128 = {
            let value = state.read_register::<i128>(s_585_1 as isize);
            tracer.read_register(s_585_1 as isize, value);
            value
        };
        // D s_585_3: cmp-lt s_585_2 s_585_0
        let s_585_3: bool = ((s_585_2) < (s_585_0));
        // D s_585_4: write-var gs#382185 <= s_585_3
        fn_state.gs_382185 = s_585_3;
        // N s_585_5: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_586<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_586_0: const #10s : i
        let s_586_0: i128 = 10;
        // D s_586_1: read-var u#25693:u32
        let s_586_1: u32 = fn_state.u_25693;
        // D s_586_2: cast zx s_586_1 -> bv
        let s_586_2: Bits = Bits::new(s_586_1 as u128, 32u16);
        // C s_586_3: const #1s : i64
        let s_586_3: i64 = 1;
        // C s_586_4: cast zx s_586_3 -> i
        let s_586_4: i128 = (i128::try_from(s_586_3).unwrap());
        // C s_586_5: const #5s : i
        let s_586_5: i128 = 5;
        // C s_586_6: add s_586_5 s_586_4
        let s_586_6: i128 = (s_586_5 + s_586_4);
        // D s_586_7: bit-extract s_586_2 s_586_0 s_586_6
        let s_586_7: Bits = (Bits::new(
            ((s_586_2) >> (s_586_0)).value(),
            u16::try_from(s_586_6).unwrap(),
        ));
        // D s_586_8: cast reint s_586_7 -> u8
        let s_586_8: u8 = (s_586_7.value() as u8);
        // D s_586_9: cast zx s_586_8 -> bv
        let s_586_9: Bits = Bits::new(s_586_8 as u128, 6u16);
        // C s_586_10: const #0u : u8
        let s_586_10: u8 = 0;
        // C s_586_11: cast zx s_586_10 -> bv
        let s_586_11: Bits = Bits::new(s_586_10 as u128, 6u16);
        // D s_586_12: cmp-eq s_586_9 s_586_11
        let s_586_12: bool = ((s_586_9) == (s_586_11));
        // D s_586_13: write-var gs#382183 <= s_586_12
        fn_state.gs_382183 = s_586_12;
        // N s_586_14: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_587<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_587_0: const #7s : i
        let s_587_0: i128 = 7;
        // C s_587_1: const #14696u : u32
        let s_587_1: u32 = 14696;
        // D s_587_2: read-reg s_587_1:i
        let s_587_2: i128 = {
            let value = state.read_register::<i128>(s_587_1 as isize);
            tracer.read_register(s_587_1 as isize, value);
            value
        };
        // D s_587_3: cmp-lt s_587_2 s_587_0
        let s_587_3: bool = ((s_587_2) < (s_587_0));
        // D s_587_4: write-var gs#382163 <= s_587_3
        fn_state.gs_382163 = s_587_3;
        // N s_587_5: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_588<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_588_0: const #10s : i
        let s_588_0: i128 = 10;
        // D s_588_1: read-var u#25685:u32
        let s_588_1: u32 = fn_state.u_25685;
        // D s_588_2: cast zx s_588_1 -> bv
        let s_588_2: Bits = Bits::new(s_588_1 as u128, 32u16);
        // C s_588_3: const #1s : i64
        let s_588_3: i64 = 1;
        // C s_588_4: cast zx s_588_3 -> i
        let s_588_4: i128 = (i128::try_from(s_588_3).unwrap());
        // C s_588_5: const #5s : i
        let s_588_5: i128 = 5;
        // C s_588_6: add s_588_5 s_588_4
        let s_588_6: i128 = (s_588_5 + s_588_4);
        // D s_588_7: bit-extract s_588_2 s_588_0 s_588_6
        let s_588_7: Bits = (Bits::new(
            ((s_588_2) >> (s_588_0)).value(),
            u16::try_from(s_588_6).unwrap(),
        ));
        // D s_588_8: cast reint s_588_7 -> u8
        let s_588_8: u8 = (s_588_7.value() as u8);
        // D s_588_9: cast zx s_588_8 -> bv
        let s_588_9: Bits = Bits::new(s_588_8 as u128, 6u16);
        // C s_588_10: const #0u : u8
        let s_588_10: u8 = 0;
        // C s_588_11: cast zx s_588_10 -> bv
        let s_588_11: Bits = Bits::new(s_588_10 as u128, 6u16);
        // D s_588_12: cmp-eq s_588_9 s_588_11
        let s_588_12: bool = ((s_588_9) == (s_588_11));
        // D s_588_13: write-var gs#382161 <= s_588_12
        fn_state.gs_382161 = s_588_12;
        // N s_588_14: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_589<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_589_0: const #6s : i
        let s_589_0: i128 = 6;
        // C s_589_1: const #14696u : u32
        let s_589_1: u32 = 14696;
        // D s_589_2: read-reg s_589_1:i
        let s_589_2: i128 = {
            let value = state.read_register::<i128>(s_589_1 as isize);
            tracer.read_register(s_589_1 as isize, value);
            value
        };
        // D s_589_3: cmp-lt s_589_2 s_589_0
        let s_589_3: bool = ((s_589_2) < (s_589_0));
        // D s_589_4: write-var gs#382141 <= s_589_3
        fn_state.gs_382141 = s_589_3;
        // N s_589_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_590<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_590_0: const #10s : i
        let s_590_0: i128 = 10;
        // D s_590_1: read-var u#25680:u32
        let s_590_1: u32 = fn_state.u_25680;
        // D s_590_2: cast zx s_590_1 -> bv
        let s_590_2: Bits = Bits::new(s_590_1 as u128, 32u16);
        // C s_590_3: const #1s : i64
        let s_590_3: i64 = 1;
        // C s_590_4: cast zx s_590_3 -> i
        let s_590_4: i128 = (i128::try_from(s_590_3).unwrap());
        // C s_590_5: const #5s : i
        let s_590_5: i128 = 5;
        // C s_590_6: add s_590_5 s_590_4
        let s_590_6: i128 = (s_590_5 + s_590_4);
        // D s_590_7: bit-extract s_590_2 s_590_0 s_590_6
        let s_590_7: Bits = (Bits::new(
            ((s_590_2) >> (s_590_0)).value(),
            u16::try_from(s_590_6).unwrap(),
        ));
        // D s_590_8: cast reint s_590_7 -> u8
        let s_590_8: u8 = (s_590_7.value() as u8);
        // D s_590_9: cast zx s_590_8 -> bv
        let s_590_9: Bits = Bits::new(s_590_8 as u128, 6u16);
        // C s_590_10: const #0u : u8
        let s_590_10: u8 = 0;
        // C s_590_11: cast zx s_590_10 -> bv
        let s_590_11: Bits = Bits::new(s_590_10 as u128, 6u16);
        // D s_590_12: cmp-eq s_590_9 s_590_11
        let s_590_12: bool = ((s_590_9) == (s_590_11));
        // D s_590_13: write-var gs#382139 <= s_590_12
        fn_state.gs_382139 = s_590_12;
        // N s_590_14: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_591<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_591_0: const #1s : i
        let s_591_0: i128 = 1;
        // C s_591_1: const #14696u : u32
        let s_591_1: u32 = 14696;
        // D s_591_2: read-reg s_591_1:i
        let s_591_2: i128 = {
            let value = state.read_register::<i128>(s_591_1 as isize);
            tracer.read_register(s_591_1 as isize, value);
            value
        };
        // D s_591_3: cmp-lt s_591_2 s_591_0
        let s_591_3: bool = ((s_591_2) < (s_591_0));
        // D s_591_4: write-var gs#382125 <= s_591_3
        fn_state.gs_382125 = s_591_3;
        // N s_591_5: jump b2
        return block_2(state, tracer, fn_state);
    }
}
