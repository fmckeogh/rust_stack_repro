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
use u_get_SCTLR_Type_AFE::*;
use SCTLR_read__2::*;
use HaveAArch32EL::*;
use u_get_SCR_EL3_Type_SIF::*;
use u_get_SCTLR_Type_UWXN::*;
use ELUsingAArch32::*;
use SCTLR_NS_read::*;
use u_get_SCR_Type_SIF::*;
use u__IMPDEF_bits::*;
use ConstrainUnpredictable::*;
use HavePANExt::*;
use u_get_SCTLR_Type_WXN::*;
use common::*;
pub fn AArch32_S1SDHasPermissionsFault<T: Tracer>(
    state: &mut State,
    tracer: &T,
    regime: u32,
    perms_in: ProductTypebf05c51f33174538,
    memtype: u32,
    paspace: u32,
    accdesc: ProductType9878976b5bcce9c9,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        r: bool,
        ga_22084: u8,
        ga_22077: ProductTypede60d0d1f6e7c94c,
        ga_22097: ProductTyped8f896a024a4e2cb,
        ga_22082: ProductTypede60d0d1f6e7c94c,
        ga_22085: ProductTypede60d0d1f6e7c94c,
        gs_28538: bool,
        return_value: bool,
        w: bool,
        ga_22086: ProductTypede60d0d1f6e7c94c,
        ga_22079: ProductTypede60d0d1f6e7c94c,
        gs_28544: bool,
        ga_22088: ProductTypede60d0d1f6e7c94c,
        x: bool,
        sctlr: ProductType700c18a878c5601b,
        ux: bool,
        ga_22078: ProductTypede60d0d1f6e7c94c,
        pw: bool,
        ga_22076: ProductTypede60d0d1f6e7c94c,
        perms: ProductTypebf05c51f33174538,
        px: bool,
        ga_22087: ProductTypede60d0d1f6e7c94c,
        pr: bool,
        ga_22075: u8,
        gs_28531: bool,
        gs_28543: bool,
        ur: bool,
        ga_22080: ProductTypede60d0d1f6e7c94c,
        ga_22102: bool,
        uw: bool,
        ga_22081: ProductTypede60d0d1f6e7c94c,
        xshadow_528: bool,
        regime: u32,
        perms_in: ProductTypebf05c51f33174538,
        memtype: u32,
        paspace: u32,
        accdesc: ProductType9878976b5bcce9c9,
    }
    let fn_state = FunctionState {
        regime,
        perms_in,
        memtype,
        paspace,
        accdesc,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var perms_in:struct
        let s_0_0: ProductTypebf05c51f33174538 = fn_state.perms_in;
        // D s_0_1: write-var perms <= s_0_0
        fn_state.perms = s_0_0;
        // D s_0_2: read-var regime:u32
        let s_0_2: u32 = fn_state.regime;
        // C s_0_3: const #1u : u32
        let s_0_3: u32 = 1;
        // D s_0_4: cmp-eq s_0_2 s_0_3
        let s_0_4: bool = ((s_0_2) == (s_0_3));
        // N s_0_5: branch s_0_4 b68 b1
        if s_0_4 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_1_0: const #424u : u32
        let s_1_0: u32 = 424;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: call HaveAArch32EL(s_1_1)
        let s_1_2: bool = HaveAArch32EL(state, tracer, s_1_1);
        // N s_1_3: branch s_1_2 b67 b2
        if s_1_2 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call SCTLR_read__2(s_2_0)
        let s_2_1: ProductType700c18a878c5601b = SCTLR_read__2(state, tracer, s_2_0);
        // D s_2_2: write-var sctlr <= s_2_1
        fn_state.sctlr = s_2_1;
        // N s_2_3: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_3_0: read-var sctlr:struct
        let s_3_0: ProductType700c18a878c5601b = fn_state.sctlr;
        // D s_3_1: call _get_SCTLR_Type_AFE(s_3_0)
        let s_3_1: bool = u_get_SCTLR_Type_AFE(state, tracer, s_3_0);
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 1u16);
        // C s_3_3: const #0u : u8
        let s_3_3: bool = false;
        // C s_3_4: cast zx s_3_3 -> bv
        let s_3_4: Bits = Bits::new(s_3_3 as u128, 1u16);
        // D s_3_5: cmp-eq s_3_2 s_3_4
        let s_3_5: bool = ((s_3_2) == (s_3_4));
        // N s_3_6: branch s_3_5 b48 b4
        if s_3_5 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_4_0: read-var perms.0:struct
        let s_4_0: u8 = fn_state.perms._0;
        // C s_4_1: const #1s : i
        let s_4_1: i128 = 1;
        // D s_4_2: cast zx s_4_0 -> bv
        let s_4_2: Bits = Bits::new(s_4_0 as u128, 3u16);
        // C s_4_3: const #1s : i64
        let s_4_3: i64 = 1;
        // C s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // C s_4_5: const #1s : i
        let s_4_5: i128 = 1;
        // C s_4_6: add s_4_5 s_4_4
        let s_4_6: i128 = (s_4_5 + s_4_4);
        // D s_4_7: bit-extract s_4_2 s_4_1 s_4_6
        let s_4_7: Bits = (Bits::new(
            ((s_4_2) >> (s_4_1)).value(),
            u16::try_from(s_4_6).unwrap(),
        ));
        // D s_4_8: cast reint s_4_7 -> u8
        let s_4_8: u8 = (s_4_7.value() as u8);
        // D s_4_9: write-var ga#22084 <= s_4_8
        fn_state.ga_22084 = s_4_8;
        // D s_4_10: read-var ga#22084:u8
        let s_4_10: u8 = fn_state.ga_22084;
        // D s_4_11: cast zx s_4_10 -> bv
        let s_4_11: Bits = Bits::new(s_4_10 as u128, 2u16);
        // C s_4_12: const #0u : u8
        let s_4_12: u8 = 0;
        // C s_4_13: cast zx s_4_12 -> bv
        let s_4_13: Bits = Bits::new(s_4_12 as u128, 2u16);
        // D s_4_14: cmp-eq s_4_11 s_4_13
        let s_4_14: bool = ((s_4_11) == (s_4_13));
        // D s_4_15: not s_4_14
        let s_4_15: bool = !s_4_14;
        // N s_4_16: branch s_4_15 b41 b5
        if s_4_15 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_5_0: const #1u : u8
        let s_5_0: bool = true;
        // C s_5_1: const #1u : u8
        let s_5_1: bool = true;
        // C s_5_2: const #0u : u8
        let s_5_2: bool = false;
        // C s_5_3: const #0u : u8
        let s_5_3: bool = false;
        // D s_5_4: create-product struct = ["s_5_0", "s_5_1", "s_5_2", "s_5_3"]
        let s_5_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_5_0,
            _1: s_5_1,
            _2: s_5_2,
            _3: s_5_3,
        };
        // D s_5_5: write-var ga#22085 <= s_5_4
        fn_state.ga_22085 = s_5_4;
        // D s_5_6: read-var ga#22085.0:struct
        let s_5_6: bool = fn_state.ga_22085._0;
        // D s_5_7: read-var ga#22085.1:struct
        let s_5_7: bool = fn_state.ga_22085._1;
        // D s_5_8: read-var ga#22085.2:struct
        let s_5_8: bool = fn_state.ga_22085._2;
        // D s_5_9: read-var ga#22085.3:struct
        let s_5_9: bool = fn_state.ga_22085._3;
        // D s_5_10: write-var pr <= s_5_6
        fn_state.pr = s_5_6;
        // D s_5_11: write-var pw <= s_5_7
        fn_state.pw = s_5_7;
        // D s_5_12: write-var ur <= s_5_8
        fn_state.ur = s_5_8;
        // D s_5_13: write-var uw <= s_5_9
        fn_state.uw = s_5_9;
        // N s_5_14: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_6_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_7_0: read-var perms.17:struct
        let s_7_0: bool = fn_state.perms._17;
        // D s_7_1: read-var sctlr:struct
        let s_7_1: ProductType700c18a878c5601b = fn_state.sctlr;
        // D s_7_2: call _get_SCTLR_Type_WXN(s_7_1)
        let s_7_2: bool = u_get_SCTLR_Type_WXN(state, tracer, s_7_1);
        // D s_7_3: read-var uw:u8
        let s_7_3: bool = fn_state.uw;
        // D s_7_4: cast zx s_7_3 -> bv
        let s_7_4: Bits = Bits::new(s_7_3 as u128, 1u16);
        // D s_7_5: cast zx s_7_2 -> bv
        let s_7_5: Bits = Bits::new(s_7_2 as u128, 1u16);
        // D s_7_6: and s_7_4 s_7_5
        let s_7_6: Bits = ((s_7_4) & (s_7_5));
        // D s_7_7: cast reint s_7_6 -> u8
        let s_7_7: bool = ((s_7_6.value()) != 0);
        // D s_7_8: cast zx s_7_0 -> bv
        let s_7_8: Bits = Bits::new(s_7_0 as u128, 1u16);
        // D s_7_9: cast zx s_7_7 -> bv
        let s_7_9: Bits = Bits::new(s_7_7 as u128, 1u16);
        // D s_7_10: or s_7_8 s_7_9
        let s_7_10: Bits = ((s_7_8) | (s_7_9));
        // D s_7_11: cast reint s_7_10 -> u8
        let s_7_11: bool = ((s_7_10.value()) != 0);
        // D s_7_12: cast zx s_7_11 -> bv
        let s_7_12: Bits = Bits::new(s_7_11 as u128, 1u16);
        // D s_7_13: not s_7_12
        let s_7_13: Bits = !s_7_12;
        // D s_7_14: cast reint s_7_13 -> u8
        let s_7_14: bool = ((s_7_13.value()) != 0);
        // D s_7_15: read-var ur:u8
        let s_7_15: bool = fn_state.ur;
        // D s_7_16: cast zx s_7_15 -> bv
        let s_7_16: Bits = Bits::new(s_7_15 as u128, 1u16);
        // D s_7_17: cast zx s_7_14 -> bv
        let s_7_17: Bits = Bits::new(s_7_14 as u128, 1u16);
        // D s_7_18: and s_7_16 s_7_17
        let s_7_18: Bits = ((s_7_16) & (s_7_17));
        // D s_7_19: cast reint s_7_18 -> u8
        let s_7_19: bool = ((s_7_18.value()) != 0);
        // D s_7_20: write-var ux <= s_7_19
        fn_state.ux = s_7_19;
        // D s_7_21: read-var perms.17:struct
        let s_7_21: bool = fn_state.perms._17;
        // D s_7_22: read-var perms.5:struct
        let s_7_22: bool = fn_state.perms._5;
        // D s_7_23: cast zx s_7_21 -> bv
        let s_7_23: Bits = Bits::new(s_7_21 as u128, 1u16);
        // D s_7_24: cast zx s_7_22 -> bv
        let s_7_24: Bits = Bits::new(s_7_22 as u128, 1u16);
        // D s_7_25: or s_7_23 s_7_24
        let s_7_25: Bits = ((s_7_23) | (s_7_24));
        // D s_7_26: cast reint s_7_25 -> u8
        let s_7_26: bool = ((s_7_25.value()) != 0);
        // D s_7_27: read-var sctlr:struct
        let s_7_27: ProductType700c18a878c5601b = fn_state.sctlr;
        // D s_7_28: call _get_SCTLR_Type_WXN(s_7_27)
        let s_7_28: bool = u_get_SCTLR_Type_WXN(state, tracer, s_7_27);
        // D s_7_29: read-var pw:u8
        let s_7_29: bool = fn_state.pw;
        // D s_7_30: cast zx s_7_29 -> bv
        let s_7_30: Bits = Bits::new(s_7_29 as u128, 1u16);
        // D s_7_31: cast zx s_7_28 -> bv
        let s_7_31: Bits = Bits::new(s_7_28 as u128, 1u16);
        // D s_7_32: and s_7_30 s_7_31
        let s_7_32: Bits = ((s_7_30) & (s_7_31));
        // D s_7_33: cast reint s_7_32 -> u8
        let s_7_33: bool = ((s_7_32.value()) != 0);
        // D s_7_34: cast zx s_7_26 -> bv
        let s_7_34: Bits = Bits::new(s_7_26 as u128, 1u16);
        // D s_7_35: cast zx s_7_33 -> bv
        let s_7_35: Bits = Bits::new(s_7_33 as u128, 1u16);
        // D s_7_36: or s_7_34 s_7_35
        let s_7_36: Bits = ((s_7_34) | (s_7_35));
        // D s_7_37: cast reint s_7_36 -> u8
        let s_7_37: bool = ((s_7_36.value()) != 0);
        // D s_7_38: read-var sctlr:struct
        let s_7_38: ProductType700c18a878c5601b = fn_state.sctlr;
        // D s_7_39: call _get_SCTLR_Type_UWXN(s_7_38)
        let s_7_39: bool = u_get_SCTLR_Type_UWXN(state, tracer, s_7_38);
        // D s_7_40: read-var uw:u8
        let s_7_40: bool = fn_state.uw;
        // D s_7_41: cast zx s_7_40 -> bv
        let s_7_41: Bits = Bits::new(s_7_40 as u128, 1u16);
        // D s_7_42: cast zx s_7_39 -> bv
        let s_7_42: Bits = Bits::new(s_7_39 as u128, 1u16);
        // D s_7_43: and s_7_41 s_7_42
        let s_7_43: Bits = ((s_7_41) & (s_7_42));
        // D s_7_44: cast reint s_7_43 -> u8
        let s_7_44: bool = ((s_7_43.value()) != 0);
        // D s_7_45: cast zx s_7_37 -> bv
        let s_7_45: Bits = Bits::new(s_7_37 as u128, 1u16);
        // D s_7_46: cast zx s_7_44 -> bv
        let s_7_46: Bits = Bits::new(s_7_44 as u128, 1u16);
        // D s_7_47: or s_7_45 s_7_46
        let s_7_47: Bits = ((s_7_45) | (s_7_46));
        // D s_7_48: cast reint s_7_47 -> u8
        let s_7_48: bool = ((s_7_47.value()) != 0);
        // D s_7_49: cast zx s_7_48 -> bv
        let s_7_49: Bits = Bits::new(s_7_48 as u128, 1u16);
        // D s_7_50: not s_7_49
        let s_7_50: Bits = !s_7_49;
        // D s_7_51: cast reint s_7_50 -> u8
        let s_7_51: bool = ((s_7_50.value()) != 0);
        // D s_7_52: read-var pr:u8
        let s_7_52: bool = fn_state.pr;
        // D s_7_53: cast zx s_7_52 -> bv
        let s_7_53: Bits = Bits::new(s_7_52 as u128, 1u16);
        // D s_7_54: cast zx s_7_51 -> bv
        let s_7_54: Bits = Bits::new(s_7_51 as u128, 1u16);
        // D s_7_55: and s_7_53 s_7_54
        let s_7_55: Bits = ((s_7_53) & (s_7_54));
        // D s_7_56: cast reint s_7_55 -> u8
        let s_7_56: bool = ((s_7_55.value()) != 0);
        // D s_7_57: write-var px <= s_7_56
        fn_state.px = s_7_56;
        // C s_7_58: const #() : ()
        let s_7_58: () = ();
        // S s_7_59: call HavePANExt(s_7_58)
        let s_7_59: bool = HavePANExt(state, tracer, s_7_58);
        // N s_7_60: branch s_7_59 b40 b8
        if s_7_59 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#28531 <= s_8_0
        fn_state.gs_28531 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_9_0: read-var gs#28531:u8
        let s_9_0: bool = fn_state.gs_28531;
        // N s_9_1: branch s_9_0 b39 b10
        if s_9_0 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_10_0: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_11_0: read-var accdesc.8:struct
        let s_11_0: u8 = fn_state.accdesc._8;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 2u16);
        // C s_11_2: const #448u : u32
        let s_11_2: u32 = 448;
        // D s_11_3: read-reg s_11_2:u8
        let s_11_3: u8 = {
            let value = state.read_register::<u8>(s_11_2 as isize);
            tracer.read_register(s_11_2 as isize, value);
            value
        };
        // D s_11_4: cast zx s_11_3 -> bv
        let s_11_4: Bits = Bits::new(s_11_3 as u128, 2u16);
        // D s_11_5: cmp-eq s_11_1 s_11_4
        let s_11_5: bool = ((s_11_1) == (s_11_4));
        // N s_11_6: branch s_11_5 b38 b12
        if s_11_5 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_12_0: read-var pr:u8
        let s_12_0: bool = fn_state.pr;
        // D s_12_1: read-var pw:u8
        let s_12_1: bool = fn_state.pw;
        // D s_12_2: read-var px:u8
        let s_12_2: bool = fn_state.px;
        // D s_12_3: create-product struct = ["s_12_0", "s_12_1", "s_12_2"]
        let s_12_3: ProductTyped8f896a024a4e2cb = ProductTyped8f896a024a4e2cb {
            _0: s_12_0,
            _1: s_12_1,
            _2: s_12_2,
        };
        // D s_12_4: write-var ga#22097 <= s_12_3
        fn_state.ga_22097 = s_12_3;
        // N s_12_5: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_13_0: read-var ga#22097.0:struct
        let s_13_0: bool = fn_state.ga_22097._0;
        // D s_13_1: read-var ga#22097.1:struct
        let s_13_1: bool = fn_state.ga_22097._1;
        // D s_13_2: read-var ga#22097.2:struct
        let s_13_2: bool = fn_state.ga_22097._2;
        // D s_13_3: write-var r <= s_13_0
        fn_state.r = s_13_0;
        // D s_13_4: write-var w <= s_13_1
        fn_state.w = s_13_1;
        // D s_13_5: write-var x <= s_13_2
        fn_state.x = s_13_2;
        // D s_13_6: read-var accdesc.25:struct
        let s_13_6: u32 = fn_state.accdesc._25;
        // C s_13_7: const #3u : u32
        let s_13_7: u32 = 3;
        // D s_13_8: cmp-eq s_13_6 s_13_7
        let s_13_8: bool = ((s_13_6) == (s_13_7));
        // N s_13_9: branch s_13_8 b37 b14
        if s_13_8 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var gs#28538 <= s_14_0
        fn_state.gs_28538 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_15_0: read-var gs#28538:u8
        let s_15_0: bool = fn_state.gs_28538;
        // N s_15_1: branch s_15_0 b33 b16
        if s_15_0 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_16_0: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_17_0: read-var x:u8
        let s_17_0: bool = fn_state.x;
        // D s_17_1: write-var xshadow#528 <= s_17_0
        fn_state.xshadow_528 = s_17_0;
        // D s_17_2: read-var accdesc.1:struct
        let s_17_2: u32 = fn_state.accdesc._1;
        // C s_17_3: const #0u : u32
        let s_17_3: u32 = 0;
        // D s_17_4: cmp-eq s_17_2 s_17_3
        let s_17_4: bool = ((s_17_2) == (s_17_3));
        // N s_17_5: branch s_17_4 b27 b18
        if s_17_4 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_18_0: read-var accdesc.1:struct
        let s_18_0: u32 = fn_state.accdesc._1;
        // C s_18_1: const #5u : u32
        let s_18_1: u32 = 5;
        // D s_18_2: cmp-eq s_18_0 s_18_1
        let s_18_2: bool = ((s_18_0) == (s_18_1));
        // N s_18_3: branch s_18_2 b26 b19
        if s_18_2 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_19_0: read-var accdesc.1:struct
        let s_19_0: u32 = fn_state.accdesc._1;
        // C s_19_1: const #6u : u32
        let s_19_1: u32 = 6;
        // D s_19_2: cmp-eq s_19_0 s_19_1
        let s_19_2: bool = ((s_19_0) == (s_19_1));
        // D s_19_3: write-var gs#28543 <= s_19_2
        fn_state.gs_28543 = s_19_2;
        // N s_19_4: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_20_0: read-var gs#28543:u8
        let s_20_0: bool = fn_state.gs_28543;
        // N s_20_1: branch s_20_0 b25 b21
        if s_20_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_21_0: read-var accdesc.32:struct
        let s_21_0: bool = fn_state.accdesc._32;
        // N s_21_1: branch s_21_0 b24 b22
        if s_21_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_22_0: read-var r:u8
        let s_22_0: bool = fn_state.r;
        // D s_22_1: cast zx s_22_0 -> bv
        let s_22_1: Bits = Bits::new(s_22_0 as u128, 1u16);
        // C s_22_2: const #0u : u8
        let s_22_2: bool = false;
        // C s_22_3: cast zx s_22_2 -> bv
        let s_22_3: Bits = Bits::new(s_22_2 as u128, 1u16);
        // D s_22_4: cmp-eq s_22_1 s_22_3
        let s_22_4: bool = ((s_22_1) == (s_22_3));
        // D s_22_5: write-var return_value <= s_22_4
        fn_state.return_value = s_22_4;
        // N s_22_6: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_23_0: read-var return_value:u8
        let s_23_0: bool = fn_state.return_value;
        // N s_23_1: return s_23_0
        return s_23_0;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_24_0: read-var w:u8
        let s_24_0: bool = fn_state.w;
        // D s_24_1: cast zx s_24_0 -> bv
        let s_24_1: Bits = Bits::new(s_24_0 as u128, 1u16);
        // C s_24_2: const #0u : u8
        let s_24_2: bool = false;
        // C s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 1u16);
        // D s_24_4: cmp-eq s_24_1 s_24_3
        let s_24_4: bool = ((s_24_1) == (s_24_3));
        // D s_24_5: write-var return_value <= s_24_4
        fn_state.return_value = s_24_4;
        // N s_24_6: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // D s_25_1: write-var return_value <= s_25_0
        fn_state.return_value = s_25_0;
        // N s_25_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_26_0: const #1u : u8
        let s_26_0: bool = true;
        // D s_26_1: write-var gs#28543 <= s_26_0
        fn_state.gs_28543 = s_26_0;
        // N s_26_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_27_0: read-var memtype:u32
        let s_27_0: u32 = fn_state.memtype;
        // C s_27_1: const #1u : u32
        let s_27_1: u32 = 1;
        // D s_27_2: cmp-eq s_27_0 s_27_1
        let s_27_2: bool = ((s_27_0) == (s_27_1));
        // N s_27_3: branch s_27_2 b32 b28
        if s_27_2 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_28_0: const #0u : u8
        let s_28_0: bool = false;
        // D s_28_1: write-var gs#28544 <= s_28_0
        fn_state.gs_28544 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_29_0: read-var gs#28544:u8
        let s_29_0: bool = fn_state.gs_28544;
        // N s_29_1: branch s_29_0 b31 b30
        if s_29_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_30_0: read-var xshadow#528:u8
        let s_30_0: bool = fn_state.xshadow_528;
        // D s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 1u16);
        // C s_30_2: const #0u : u8
        let s_30_2: bool = false;
        // C s_30_3: cast zx s_30_2 -> bv
        let s_30_3: Bits = Bits::new(s_30_2 as u128, 1u16);
        // D s_30_4: cmp-eq s_30_1 s_30_3
        let s_30_4: bool = ((s_30_1) == (s_30_3));
        // D s_30_5: write-var return_value <= s_30_4
        fn_state.return_value = s_30_4;
        // N s_30_6: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_31_0: const #1u : u8
        let s_31_0: bool = true;
        // D s_31_1: write-var return_value <= s_31_0
        fn_state.return_value = s_31_0;
        // N s_31_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_32_0: const #7u : u32
        let s_32_0: u32 = 7;
        // S s_32_1: call ConstrainUnpredictable(s_32_0)
        let s_32_1: u32 = ConstrainUnpredictable(state, tracer, s_32_0);
        // C s_32_2: const #12u : u32
        let s_32_2: u32 = 12;
        // S s_32_3: cmp-eq s_32_1 s_32_2
        let s_32_3: bool = ((s_32_1) == (s_32_2));
        // D s_32_4: write-var gs#28544 <= s_32_3
        fn_state.gs_28544 = s_32_3;
        // N s_32_5: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
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
        // N s_33_3: branch s_33_2 b36 b34
        if s_33_2 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_34_0: const #90704u : u32
        let s_34_0: u32 = 90704;
        // D s_34_1: read-reg s_34_0:struct
        let s_34_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_34_0 as isize);
            tracer.read_register(s_34_0 as isize, value);
            value
        };
        // D s_34_2: call _get_SCR_EL3_Type_SIF(s_34_1)
        let s_34_2: bool = u_get_SCR_EL3_Type_SIF(state, tracer, s_34_1);
        // D s_34_3: write-var ga#22102 <= s_34_2
        fn_state.ga_22102 = s_34_2;
        // N s_34_4: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_35_0: read-var ga#22102:u8
        let s_35_0: bool = fn_state.ga_22102;
        // D s_35_1: cast zx s_35_0 -> bv
        let s_35_1: Bits = Bits::new(s_35_0 as u128, 1u16);
        // D s_35_2: not s_35_1
        let s_35_2: Bits = !s_35_1;
        // D s_35_3: cast reint s_35_2 -> u8
        let s_35_3: bool = ((s_35_2.value()) != 0);
        // D s_35_4: read-var x:u8
        let s_35_4: bool = fn_state.x;
        // D s_35_5: cast zx s_35_4 -> bv
        let s_35_5: Bits = Bits::new(s_35_4 as u128, 1u16);
        // D s_35_6: cast zx s_35_3 -> bv
        let s_35_6: Bits = Bits::new(s_35_3 as u128, 1u16);
        // D s_35_7: and s_35_5 s_35_6
        let s_35_7: Bits = ((s_35_5) & (s_35_6));
        // D s_35_8: cast reint s_35_7 -> u8
        let s_35_8: bool = ((s_35_7.value()) != 0);
        // D s_35_9: write-var x <= s_35_8
        fn_state.x = s_35_8;
        // N s_35_10: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_36_0: const #20920u : u32
        let s_36_0: u32 = 20920;
        // D s_36_1: read-reg s_36_0:struct
        let s_36_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_36_0 as isize);
            tracer.read_register(s_36_0 as isize, value);
            value
        };
        // D s_36_2: call _get_SCR_Type_SIF(s_36_1)
        let s_36_2: bool = u_get_SCR_Type_SIF(state, tracer, s_36_1);
        // D s_36_3: write-var ga#22102 <= s_36_2
        fn_state.ga_22102 = s_36_2;
        // N s_36_4: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_37_0: read-var paspace:u32
        let s_37_0: u32 = fn_state.paspace;
        // C s_37_1: const #0u : u32
        let s_37_1: u32 = 0;
        // D s_37_2: cmp-eq s_37_0 s_37_1
        let s_37_2: bool = ((s_37_0) == (s_37_1));
        // D s_37_3: write-var gs#28538 <= s_37_2
        fn_state.gs_28538 = s_37_2;
        // N s_37_4: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_38_0: read-var ur:u8
        let s_38_0: bool = fn_state.ur;
        // D s_38_1: read-var uw:u8
        let s_38_1: bool = fn_state.uw;
        // D s_38_2: read-var ux:u8
        let s_38_2: bool = fn_state.ux;
        // D s_38_3: create-product struct = ["s_38_0", "s_38_1", "s_38_2"]
        let s_38_3: ProductTyped8f896a024a4e2cb = ProductTyped8f896a024a4e2cb {
            _0: s_38_0,
            _1: s_38_1,
            _2: s_38_2,
        };
        // D s_38_4: write-var ga#22097 <= s_38_3
        fn_state.ga_22097 = s_38_3;
        // N s_38_5: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_39_0: const #16985u : u32
        let s_39_0: u32 = 16985;
        // D s_39_1: read-reg s_39_0:u8
        let s_39_1: bool = {
            let value = state.read_register::<bool>(s_39_0 as isize);
            tracer.read_register(s_39_0 as isize, value);
            value
        };
        // D s_39_2: read-var ur:u8
        let s_39_2: bool = fn_state.ur;
        // D s_39_3: cast zx s_39_2 -> bv
        let s_39_3: Bits = Bits::new(s_39_2 as u128, 1u16);
        // D s_39_4: read-var uw:u8
        let s_39_4: bool = fn_state.uw;
        // D s_39_5: cast zx s_39_4 -> bv
        let s_39_5: Bits = Bits::new(s_39_4 as u128, 1u16);
        // D s_39_6: or s_39_3 s_39_5
        let s_39_6: Bits = ((s_39_3) | (s_39_5));
        // D s_39_7: cast reint s_39_6 -> u8
        let s_39_7: bool = ((s_39_6.value()) != 0);
        // D s_39_8: cast zx s_39_1 -> bv
        let s_39_8: Bits = Bits::new(s_39_1 as u128, 1u16);
        // D s_39_9: cast zx s_39_7 -> bv
        let s_39_9: Bits = Bits::new(s_39_7 as u128, 1u16);
        // D s_39_10: and s_39_8 s_39_9
        let s_39_10: Bits = ((s_39_8) & (s_39_9));
        // D s_39_11: cast reint s_39_10 -> u8
        let s_39_11: bool = ((s_39_10.value()) != 0);
        // D s_39_12: cast zx s_39_11 -> bv
        let s_39_12: Bits = Bits::new(s_39_11 as u128, 1u16);
        // D s_39_13: not s_39_12
        let s_39_13: Bits = !s_39_12;
        // D s_39_14: cast reint s_39_13 -> u8
        let s_39_14: bool = ((s_39_13.value()) != 0);
        // D s_39_15: read-var pr:u8
        let s_39_15: bool = fn_state.pr;
        // D s_39_16: cast zx s_39_15 -> bv
        let s_39_16: Bits = Bits::new(s_39_15 as u128, 1u16);
        // D s_39_17: cast zx s_39_14 -> bv
        let s_39_17: Bits = Bits::new(s_39_14 as u128, 1u16);
        // D s_39_18: and s_39_16 s_39_17
        let s_39_18: Bits = ((s_39_16) & (s_39_17));
        // D s_39_19: cast reint s_39_18 -> u8
        let s_39_19: bool = ((s_39_18.value()) != 0);
        // D s_39_20: write-var pr <= s_39_19
        fn_state.pr = s_39_19;
        // D s_39_21: cast zx s_39_11 -> bv
        let s_39_21: Bits = Bits::new(s_39_11 as u128, 1u16);
        // D s_39_22: not s_39_21
        let s_39_22: Bits = !s_39_21;
        // D s_39_23: cast reint s_39_22 -> u8
        let s_39_23: bool = ((s_39_22.value()) != 0);
        // D s_39_24: read-var pw:u8
        let s_39_24: bool = fn_state.pw;
        // D s_39_25: cast zx s_39_24 -> bv
        let s_39_25: Bits = Bits::new(s_39_24 as u128, 1u16);
        // D s_39_26: cast zx s_39_23 -> bv
        let s_39_26: Bits = Bits::new(s_39_23 as u128, 1u16);
        // D s_39_27: and s_39_25 s_39_26
        let s_39_27: Bits = ((s_39_25) & (s_39_26));
        // D s_39_28: cast reint s_39_27 -> u8
        let s_39_28: bool = ((s_39_27.value()) != 0);
        // D s_39_29: write-var pw <= s_39_28
        fn_state.pw = s_39_28;
        // N s_39_30: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_40_0: read-var accdesc.20:struct
        let s_40_0: bool = fn_state.accdesc._20;
        // D s_40_1: write-var gs#28531 <= s_40_0
        fn_state.gs_28531 = s_40_0;
        // N s_40_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_41_0: read-var ga#22084:u8
        let s_41_0: u8 = fn_state.ga_22084;
        // D s_41_1: cast zx s_41_0 -> bv
        let s_41_1: Bits = Bits::new(s_41_0 as u128, 2u16);
        // C s_41_2: const #1u : u8
        let s_41_2: u8 = 1;
        // C s_41_3: cast zx s_41_2 -> bv
        let s_41_3: Bits = Bits::new(s_41_2 as u128, 2u16);
        // D s_41_4: cmp-eq s_41_1 s_41_3
        let s_41_4: bool = ((s_41_1) == (s_41_3));
        // D s_41_5: not s_41_4
        let s_41_5: bool = !s_41_4;
        // N s_41_6: branch s_41_5 b43 b42
        if s_41_5 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_42_0: const #1u : u8
        let s_42_0: bool = true;
        // C s_42_1: const #1u : u8
        let s_42_1: bool = true;
        // C s_42_2: const #1u : u8
        let s_42_2: bool = true;
        // C s_42_3: const #1u : u8
        let s_42_3: bool = true;
        // D s_42_4: create-product struct = ["s_42_0", "s_42_1", "s_42_2", "s_42_3"]
        let s_42_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_42_0,
            _1: s_42_1,
            _2: s_42_2,
            _3: s_42_3,
        };
        // D s_42_5: write-var ga#22086 <= s_42_4
        fn_state.ga_22086 = s_42_4;
        // D s_42_6: read-var ga#22086.0:struct
        let s_42_6: bool = fn_state.ga_22086._0;
        // D s_42_7: read-var ga#22086.1:struct
        let s_42_7: bool = fn_state.ga_22086._1;
        // D s_42_8: read-var ga#22086.2:struct
        let s_42_8: bool = fn_state.ga_22086._2;
        // D s_42_9: read-var ga#22086.3:struct
        let s_42_9: bool = fn_state.ga_22086._3;
        // D s_42_10: write-var pr <= s_42_6
        fn_state.pr = s_42_6;
        // D s_42_11: write-var pw <= s_42_7
        fn_state.pw = s_42_7;
        // D s_42_12: write-var ur <= s_42_8
        fn_state.ur = s_42_8;
        // D s_42_13: write-var uw <= s_42_9
        fn_state.uw = s_42_9;
        // N s_42_14: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_43_0: read-var ga#22084:u8
        let s_43_0: u8 = fn_state.ga_22084;
        // D s_43_1: cast zx s_43_0 -> bv
        let s_43_1: Bits = Bits::new(s_43_0 as u128, 2u16);
        // C s_43_2: const #2u : u8
        let s_43_2: u8 = 2;
        // C s_43_3: cast zx s_43_2 -> bv
        let s_43_3: Bits = Bits::new(s_43_2 as u128, 2u16);
        // D s_43_4: cmp-eq s_43_1 s_43_3
        let s_43_4: bool = ((s_43_1) == (s_43_3));
        // D s_43_5: not s_43_4
        let s_43_5: bool = !s_43_4;
        // N s_43_6: branch s_43_5 b45 b44
        if s_43_5 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_44_0: const #1u : u8
        let s_44_0: bool = true;
        // C s_44_1: const #0u : u8
        let s_44_1: bool = false;
        // C s_44_2: const #0u : u8
        let s_44_2: bool = false;
        // C s_44_3: const #0u : u8
        let s_44_3: bool = false;
        // D s_44_4: create-product struct = ["s_44_0", "s_44_1", "s_44_2", "s_44_3"]
        let s_44_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_44_0,
            _1: s_44_1,
            _2: s_44_2,
            _3: s_44_3,
        };
        // D s_44_5: write-var ga#22087 <= s_44_4
        fn_state.ga_22087 = s_44_4;
        // D s_44_6: read-var ga#22087.0:struct
        let s_44_6: bool = fn_state.ga_22087._0;
        // D s_44_7: read-var ga#22087.1:struct
        let s_44_7: bool = fn_state.ga_22087._1;
        // D s_44_8: read-var ga#22087.2:struct
        let s_44_8: bool = fn_state.ga_22087._2;
        // D s_44_9: read-var ga#22087.3:struct
        let s_44_9: bool = fn_state.ga_22087._3;
        // D s_44_10: write-var pr <= s_44_6
        fn_state.pr = s_44_6;
        // D s_44_11: write-var pw <= s_44_7
        fn_state.pw = s_44_7;
        // D s_44_12: write-var ur <= s_44_8
        fn_state.ur = s_44_8;
        // D s_44_13: write-var uw <= s_44_9
        fn_state.uw = s_44_9;
        // N s_44_14: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_45_0: read-var ga#22084:u8
        let s_45_0: u8 = fn_state.ga_22084;
        // D s_45_1: cast zx s_45_0 -> bv
        let s_45_1: Bits = Bits::new(s_45_0 as u128, 2u16);
        // C s_45_2: const #3u : u8
        let s_45_2: u8 = 3;
        // C s_45_3: cast zx s_45_2 -> bv
        let s_45_3: Bits = Bits::new(s_45_2 as u128, 2u16);
        // D s_45_4: cmp-eq s_45_1 s_45_3
        let s_45_4: bool = ((s_45_1) == (s_45_3));
        // D s_45_5: not s_45_4
        let s_45_5: bool = !s_45_4;
        // N s_45_6: branch s_45_5 b47 b46
        if s_45_5 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_46_0: const #1u : u8
        let s_46_0: bool = true;
        // C s_46_1: const #0u : u8
        let s_46_1: bool = false;
        // C s_46_2: const #1u : u8
        let s_46_2: bool = true;
        // C s_46_3: const #0u : u8
        let s_46_3: bool = false;
        // D s_46_4: create-product struct = ["s_46_0", "s_46_1", "s_46_2", "s_46_3"]
        let s_46_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_46_0,
            _1: s_46_1,
            _2: s_46_2,
            _3: s_46_3,
        };
        // D s_46_5: write-var ga#22088 <= s_46_4
        fn_state.ga_22088 = s_46_4;
        // D s_46_6: read-var ga#22088.0:struct
        let s_46_6: bool = fn_state.ga_22088._0;
        // D s_46_7: read-var ga#22088.1:struct
        let s_46_7: bool = fn_state.ga_22088._1;
        // D s_46_8: read-var ga#22088.2:struct
        let s_46_8: bool = fn_state.ga_22088._2;
        // D s_46_9: read-var ga#22088.3:struct
        let s_46_9: bool = fn_state.ga_22088._3;
        // D s_46_10: write-var pr <= s_46_6
        fn_state.pr = s_46_6;
        // D s_46_11: write-var pw <= s_46_7
        fn_state.pw = s_46_7;
        // D s_46_12: write-var ur <= s_46_8
        fn_state.ur = s_46_8;
        // D s_46_13: write-var uw <= s_46_9
        fn_state.uw = s_46_9;
        // N s_46_14: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_47_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_48_0: read-var perms.0:struct
        let s_48_0: u8 = fn_state.perms._0;
        // D s_48_1: cast zx s_48_0 -> bv
        let s_48_1: Bits = Bits::new(s_48_0 as u128, 3u16);
        // C s_48_2: const #4u : u8
        let s_48_2: u8 = 4;
        // C s_48_3: cast zx s_48_2 -> bv
        let s_48_3: Bits = Bits::new(s_48_2 as u128, 3u16);
        // D s_48_4: cmp-eq s_48_1 s_48_3
        let s_48_4: bool = ((s_48_1) == (s_48_3));
        // N s_48_5: branch s_48_4 b66 b49
        if s_48_4 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_49_0: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_50_0: read-var perms.0:struct
        let s_50_0: u8 = fn_state.perms._0;
        // D s_50_1: write-var ga#22075 <= s_50_0
        fn_state.ga_22075 = s_50_0;
        // D s_50_2: read-var ga#22075:u8
        let s_50_2: u8 = fn_state.ga_22075;
        // D s_50_3: cast zx s_50_2 -> bv
        let s_50_3: Bits = Bits::new(s_50_2 as u128, 3u16);
        // C s_50_4: const #0u : u8
        let s_50_4: u8 = 0;
        // C s_50_5: cast zx s_50_4 -> bv
        let s_50_5: Bits = Bits::new(s_50_4 as u128, 3u16);
        // D s_50_6: cmp-eq s_50_3 s_50_5
        let s_50_6: bool = ((s_50_3) == (s_50_5));
        // D s_50_7: not s_50_6
        let s_50_7: bool = !s_50_6;
        // N s_50_8: branch s_50_7 b53 b51
        if s_50_7 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_51_0: const #0u : u8
        let s_51_0: bool = false;
        // C s_51_1: const #0u : u8
        let s_51_1: bool = false;
        // C s_51_2: const #0u : u8
        let s_51_2: bool = false;
        // C s_51_3: const #0u : u8
        let s_51_3: bool = false;
        // D s_51_4: create-product struct = ["s_51_0", "s_51_1", "s_51_2", "s_51_3"]
        let s_51_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_51_0,
            _1: s_51_1,
            _2: s_51_2,
            _3: s_51_3,
        };
        // D s_51_5: write-var ga#22076 <= s_51_4
        fn_state.ga_22076 = s_51_4;
        // D s_51_6: read-var ga#22076.0:struct
        let s_51_6: bool = fn_state.ga_22076._0;
        // D s_51_7: read-var ga#22076.1:struct
        let s_51_7: bool = fn_state.ga_22076._1;
        // D s_51_8: read-var ga#22076.2:struct
        let s_51_8: bool = fn_state.ga_22076._2;
        // D s_51_9: read-var ga#22076.3:struct
        let s_51_9: bool = fn_state.ga_22076._3;
        // D s_51_10: write-var pr <= s_51_6
        fn_state.pr = s_51_6;
        // D s_51_11: write-var pw <= s_51_7
        fn_state.pw = s_51_7;
        // D s_51_12: write-var ur <= s_51_8
        fn_state.ur = s_51_8;
        // D s_51_13: write-var uw <= s_51_9
        fn_state.uw = s_51_9;
        // N s_51_14: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_52_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_53_0: read-var ga#22075:u8
        let s_53_0: u8 = fn_state.ga_22075;
        // D s_53_1: cast zx s_53_0 -> bv
        let s_53_1: Bits = Bits::new(s_53_0 as u128, 3u16);
        // C s_53_2: const #1u : u8
        let s_53_2: u8 = 1;
        // C s_53_3: cast zx s_53_2 -> bv
        let s_53_3: Bits = Bits::new(s_53_2 as u128, 3u16);
        // D s_53_4: cmp-eq s_53_1 s_53_3
        let s_53_4: bool = ((s_53_1) == (s_53_3));
        // D s_53_5: not s_53_4
        let s_53_5: bool = !s_53_4;
        // N s_53_6: branch s_53_5 b55 b54
        if s_53_5 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_54_0: const #1u : u8
        let s_54_0: bool = true;
        // C s_54_1: const #1u : u8
        let s_54_1: bool = true;
        // C s_54_2: const #0u : u8
        let s_54_2: bool = false;
        // C s_54_3: const #0u : u8
        let s_54_3: bool = false;
        // D s_54_4: create-product struct = ["s_54_0", "s_54_1", "s_54_2", "s_54_3"]
        let s_54_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_54_0,
            _1: s_54_1,
            _2: s_54_2,
            _3: s_54_3,
        };
        // D s_54_5: write-var ga#22077 <= s_54_4
        fn_state.ga_22077 = s_54_4;
        // D s_54_6: read-var ga#22077.0:struct
        let s_54_6: bool = fn_state.ga_22077._0;
        // D s_54_7: read-var ga#22077.1:struct
        let s_54_7: bool = fn_state.ga_22077._1;
        // D s_54_8: read-var ga#22077.2:struct
        let s_54_8: bool = fn_state.ga_22077._2;
        // D s_54_9: read-var ga#22077.3:struct
        let s_54_9: bool = fn_state.ga_22077._3;
        // D s_54_10: write-var pr <= s_54_6
        fn_state.pr = s_54_6;
        // D s_54_11: write-var pw <= s_54_7
        fn_state.pw = s_54_7;
        // D s_54_12: write-var ur <= s_54_8
        fn_state.ur = s_54_8;
        // D s_54_13: write-var uw <= s_54_9
        fn_state.uw = s_54_9;
        // N s_54_14: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_55_0: read-var ga#22075:u8
        let s_55_0: u8 = fn_state.ga_22075;
        // D s_55_1: cast zx s_55_0 -> bv
        let s_55_1: Bits = Bits::new(s_55_0 as u128, 3u16);
        // C s_55_2: const #2u : u8
        let s_55_2: u8 = 2;
        // C s_55_3: cast zx s_55_2 -> bv
        let s_55_3: Bits = Bits::new(s_55_2 as u128, 3u16);
        // D s_55_4: cmp-eq s_55_1 s_55_3
        let s_55_4: bool = ((s_55_1) == (s_55_3));
        // D s_55_5: not s_55_4
        let s_55_5: bool = !s_55_4;
        // N s_55_6: branch s_55_5 b57 b56
        if s_55_5 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_56_0: const #1u : u8
        let s_56_0: bool = true;
        // C s_56_1: const #1u : u8
        let s_56_1: bool = true;
        // C s_56_2: const #1u : u8
        let s_56_2: bool = true;
        // C s_56_3: const #0u : u8
        let s_56_3: bool = false;
        // D s_56_4: create-product struct = ["s_56_0", "s_56_1", "s_56_2", "s_56_3"]
        let s_56_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_56_0,
            _1: s_56_1,
            _2: s_56_2,
            _3: s_56_3,
        };
        // D s_56_5: write-var ga#22078 <= s_56_4
        fn_state.ga_22078 = s_56_4;
        // D s_56_6: read-var ga#22078.0:struct
        let s_56_6: bool = fn_state.ga_22078._0;
        // D s_56_7: read-var ga#22078.1:struct
        let s_56_7: bool = fn_state.ga_22078._1;
        // D s_56_8: read-var ga#22078.2:struct
        let s_56_8: bool = fn_state.ga_22078._2;
        // D s_56_9: read-var ga#22078.3:struct
        let s_56_9: bool = fn_state.ga_22078._3;
        // D s_56_10: write-var pr <= s_56_6
        fn_state.pr = s_56_6;
        // D s_56_11: write-var pw <= s_56_7
        fn_state.pw = s_56_7;
        // D s_56_12: write-var ur <= s_56_8
        fn_state.ur = s_56_8;
        // D s_56_13: write-var uw <= s_56_9
        fn_state.uw = s_56_9;
        // N s_56_14: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_57_0: read-var ga#22075:u8
        let s_57_0: u8 = fn_state.ga_22075;
        // D s_57_1: cast zx s_57_0 -> bv
        let s_57_1: Bits = Bits::new(s_57_0 as u128, 3u16);
        // C s_57_2: const #3u : u8
        let s_57_2: u8 = 3;
        // C s_57_3: cast zx s_57_2 -> bv
        let s_57_3: Bits = Bits::new(s_57_2 as u128, 3u16);
        // D s_57_4: cmp-eq s_57_1 s_57_3
        let s_57_4: bool = ((s_57_1) == (s_57_3));
        // D s_57_5: not s_57_4
        let s_57_5: bool = !s_57_4;
        // N s_57_6: branch s_57_5 b59 b58
        if s_57_5 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_58_0: const #1u : u8
        let s_58_0: bool = true;
        // C s_58_1: const #1u : u8
        let s_58_1: bool = true;
        // C s_58_2: const #1u : u8
        let s_58_2: bool = true;
        // C s_58_3: const #1u : u8
        let s_58_3: bool = true;
        // D s_58_4: create-product struct = ["s_58_0", "s_58_1", "s_58_2", "s_58_3"]
        let s_58_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_58_0,
            _1: s_58_1,
            _2: s_58_2,
            _3: s_58_3,
        };
        // D s_58_5: write-var ga#22079 <= s_58_4
        fn_state.ga_22079 = s_58_4;
        // D s_58_6: read-var ga#22079.0:struct
        let s_58_6: bool = fn_state.ga_22079._0;
        // D s_58_7: read-var ga#22079.1:struct
        let s_58_7: bool = fn_state.ga_22079._1;
        // D s_58_8: read-var ga#22079.2:struct
        let s_58_8: bool = fn_state.ga_22079._2;
        // D s_58_9: read-var ga#22079.3:struct
        let s_58_9: bool = fn_state.ga_22079._3;
        // D s_58_10: write-var pr <= s_58_6
        fn_state.pr = s_58_6;
        // D s_58_11: write-var pw <= s_58_7
        fn_state.pw = s_58_7;
        // D s_58_12: write-var ur <= s_58_8
        fn_state.ur = s_58_8;
        // D s_58_13: write-var uw <= s_58_9
        fn_state.uw = s_58_9;
        // N s_58_14: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_59_0: read-var ga#22075:u8
        let s_59_0: u8 = fn_state.ga_22075;
        // D s_59_1: cast zx s_59_0 -> bv
        let s_59_1: Bits = Bits::new(s_59_0 as u128, 3u16);
        // C s_59_2: const #5u : u8
        let s_59_2: u8 = 5;
        // C s_59_3: cast zx s_59_2 -> bv
        let s_59_3: Bits = Bits::new(s_59_2 as u128, 3u16);
        // D s_59_4: cmp-eq s_59_1 s_59_3
        let s_59_4: bool = ((s_59_1) == (s_59_3));
        // D s_59_5: not s_59_4
        let s_59_5: bool = !s_59_4;
        // N s_59_6: branch s_59_5 b61 b60
        if s_59_5 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_60(state, tracer, fn_state);
        };
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_60_0: const #1u : u8
        let s_60_0: bool = true;
        // C s_60_1: const #0u : u8
        let s_60_1: bool = false;
        // C s_60_2: const #0u : u8
        let s_60_2: bool = false;
        // C s_60_3: const #0u : u8
        let s_60_3: bool = false;
        // D s_60_4: create-product struct = ["s_60_0", "s_60_1", "s_60_2", "s_60_3"]
        let s_60_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_60_0,
            _1: s_60_1,
            _2: s_60_2,
            _3: s_60_3,
        };
        // D s_60_5: write-var ga#22080 <= s_60_4
        fn_state.ga_22080 = s_60_4;
        // D s_60_6: read-var ga#22080.0:struct
        let s_60_6: bool = fn_state.ga_22080._0;
        // D s_60_7: read-var ga#22080.1:struct
        let s_60_7: bool = fn_state.ga_22080._1;
        // D s_60_8: read-var ga#22080.2:struct
        let s_60_8: bool = fn_state.ga_22080._2;
        // D s_60_9: read-var ga#22080.3:struct
        let s_60_9: bool = fn_state.ga_22080._3;
        // D s_60_10: write-var pr <= s_60_6
        fn_state.pr = s_60_6;
        // D s_60_11: write-var pw <= s_60_7
        fn_state.pw = s_60_7;
        // D s_60_12: write-var ur <= s_60_8
        fn_state.ur = s_60_8;
        // D s_60_13: write-var uw <= s_60_9
        fn_state.uw = s_60_9;
        // N s_60_14: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_61_0: read-var ga#22075:u8
        let s_61_0: u8 = fn_state.ga_22075;
        // D s_61_1: cast zx s_61_0 -> bv
        let s_61_1: Bits = Bits::new(s_61_0 as u128, 3u16);
        // C s_61_2: const #6u : u8
        let s_61_2: u8 = 6;
        // C s_61_3: cast zx s_61_2 -> bv
        let s_61_3: Bits = Bits::new(s_61_2 as u128, 3u16);
        // D s_61_4: cmp-eq s_61_1 s_61_3
        let s_61_4: bool = ((s_61_1) == (s_61_3));
        // D s_61_5: not s_61_4
        let s_61_5: bool = !s_61_4;
        // N s_61_6: branch s_61_5 b63 b62
        if s_61_5 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_62_0: const #1u : u8
        let s_62_0: bool = true;
        // C s_62_1: const #0u : u8
        let s_62_1: bool = false;
        // C s_62_2: const #1u : u8
        let s_62_2: bool = true;
        // C s_62_3: const #0u : u8
        let s_62_3: bool = false;
        // D s_62_4: create-product struct = ["s_62_0", "s_62_1", "s_62_2", "s_62_3"]
        let s_62_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_62_0,
            _1: s_62_1,
            _2: s_62_2,
            _3: s_62_3,
        };
        // D s_62_5: write-var ga#22081 <= s_62_4
        fn_state.ga_22081 = s_62_4;
        // D s_62_6: read-var ga#22081.0:struct
        let s_62_6: bool = fn_state.ga_22081._0;
        // D s_62_7: read-var ga#22081.1:struct
        let s_62_7: bool = fn_state.ga_22081._1;
        // D s_62_8: read-var ga#22081.2:struct
        let s_62_8: bool = fn_state.ga_22081._2;
        // D s_62_9: read-var ga#22081.3:struct
        let s_62_9: bool = fn_state.ga_22081._3;
        // D s_62_10: write-var pr <= s_62_6
        fn_state.pr = s_62_6;
        // D s_62_11: write-var pw <= s_62_7
        fn_state.pw = s_62_7;
        // D s_62_12: write-var ur <= s_62_8
        fn_state.ur = s_62_8;
        // D s_62_13: write-var uw <= s_62_9
        fn_state.uw = s_62_9;
        // N s_62_14: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_63_0: read-var ga#22075:u8
        let s_63_0: u8 = fn_state.ga_22075;
        // D s_63_1: cast zx s_63_0 -> bv
        let s_63_1: Bits = Bits::new(s_63_0 as u128, 3u16);
        // C s_63_2: const #7u : u8
        let s_63_2: u8 = 7;
        // C s_63_3: cast zx s_63_2 -> bv
        let s_63_3: Bits = Bits::new(s_63_2 as u128, 3u16);
        // D s_63_4: cmp-eq s_63_1 s_63_3
        let s_63_4: bool = ((s_63_1) == (s_63_3));
        // D s_63_5: not s_63_4
        let s_63_5: bool = !s_63_4;
        // N s_63_6: branch s_63_5 b65 b64
        if s_63_5 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_64(state, tracer, fn_state);
        };
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_64_0: const #1u : u8
        let s_64_0: bool = true;
        // C s_64_1: const #0u : u8
        let s_64_1: bool = false;
        // C s_64_2: const #1u : u8
        let s_64_2: bool = true;
        // C s_64_3: const #0u : u8
        let s_64_3: bool = false;
        // D s_64_4: create-product struct = ["s_64_0", "s_64_1", "s_64_2", "s_64_3"]
        let s_64_4: ProductTypede60d0d1f6e7c94c = ProductTypede60d0d1f6e7c94c {
            _0: s_64_0,
            _1: s_64_1,
            _2: s_64_2,
            _3: s_64_3,
        };
        // D s_64_5: write-var ga#22082 <= s_64_4
        fn_state.ga_22082 = s_64_4;
        // D s_64_6: read-var ga#22082.0:struct
        let s_64_6: bool = fn_state.ga_22082._0;
        // D s_64_7: read-var ga#22082.1:struct
        let s_64_7: bool = fn_state.ga_22082._1;
        // D s_64_8: read-var ga#22082.2:struct
        let s_64_8: bool = fn_state.ga_22082._2;
        // D s_64_9: read-var ga#22082.3:struct
        let s_64_9: bool = fn_state.ga_22082._3;
        // D s_64_10: write-var pr <= s_64_6
        fn_state.pr = s_64_6;
        // D s_64_11: write-var pw <= s_64_7
        fn_state.pw = s_64_7;
        // D s_64_12: write-var ur <= s_64_8
        fn_state.ur = s_64_8;
        // D s_64_13: write-var uw <= s_64_9
        fn_state.uw = s_64_9;
        // N s_64_14: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_65_0: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_66_0: const #3s : i64
        let s_66_0: i64 = 3;
        // C s_66_1: cast zx s_66_0 -> i
        let s_66_1: i128 = (i128::try_from(s_66_0).unwrap());
        // C s_66_2: const #"Reserved short descriptor AP encoding" : str
        let s_66_2: &'static str = "Reserved short descriptor AP encoding";
        // S s_66_3: call __IMPDEF_bits(s_66_1, s_66_2)
        let s_66_3: Bits = u__IMPDEF_bits(state, tracer, s_66_1, s_66_2);
        // S s_66_4: cast reint s_66_3 -> u8
        let s_66_4: u8 = (s_66_3.value() as u8);
        // D s_66_5: write-var perms.0 <= s_66_4
        fn_state.perms._0 = s_66_4;
        // N s_66_6: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_67_0: const #() : ()
        let s_67_0: () = ();
        // S s_67_1: call SCTLR_NS_read(s_67_0)
        let s_67_1: ProductType700c18a878c5601b = SCTLR_NS_read(state, tracer, s_67_0);
        // D s_67_2: write-var sctlr <= s_67_1
        fn_state.sctlr = s_67_1;
        // N s_67_3: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_68_0: const #16456u : u32
        let s_68_0: u32 = 16456;
        // D s_68_1: read-reg s_68_0:struct
        let s_68_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_68_0 as isize);
            tracer.read_register(s_68_0 as isize, value);
            value
        };
        // D s_68_2: write-var sctlr <= s_68_1
        fn_state.sctlr = s_68_1;
        // N s_68_3: jump b3
        return block_3(state, tracer, fn_state);
    }
}
