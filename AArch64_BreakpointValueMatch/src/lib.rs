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
use IsInHost::*;
use u_get_VTTBR_EL2_Type_VMID::*;
use Have16bitVMID::*;
use ConstrainUnpredictableBool::*;
use HaveFeatABLE::*;
use u_get_VTCR_EL2_Type_VS::*;
use DebugAddrTop::*;
use IsBreakpointEnabled::*;
use is_zero_subrange::*;
use u_get_DBGBCR_EL1_Type_BAS::*;
use NumBreakpointsImplemented::*;
use VTTBR_EL2_read::*;
use u_get_DBGBCR_EL1_Type_MASK::*;
use neq_int::*;
use u__id::*;
use HaveAArch32::*;
use is_ones_subrange::*;
use ConstrainUnpredictableInteger::*;
use subrange_subrange_eq::*;
use UsingAArch32::*;
use u_get_DBGBCR_EL1_Type_BT::*;
use EL2Enabled::*;
use AArch64_ReservedBreakpointType::*;
use u_get_DBGBCR_EL1_Type_BT2::*;
use common::*;
pub fn AArch64_BreakpointValueMatch<T: Tracer>(
    state: &mut State,
    tracer: &T,
    n_in: i128,
    vaddress: u64,
    linked_to: bool,
    isbreakpnt: bool,
) -> ProductType8b847afc727d5818 {
    #[derive(Default)]
    struct FunctionState {
        gs_16215: bool,
        gs_16308: bool,
        gs_16230: bool,
        ga_12096: ProductType5c790c8ef59cc8b2,
        vmid: u16,
        gs_16254: bool,
        c: u32,
        gs_16306: bool,
        mask: i128,
        byte_select_match: bool,
        gs_16299: bool,
        gs_16171: bool,
        ga_12088: ProductType5c790c8ef59cc8b2,
        ga_12054: ProductType5c790c8ef59cc8b2,
        gs_16174: bool,
        gs_16224: bool,
        match_cid: bool,
        match_cid2: bool,
        gs_16140: bool,
        match_cid1: bool,
        gs_16289: bool,
        gs_16247: bool,
        gs_16163: bool,
        maskshadow_271: i128,
        gs_16307: bool,
        gs_16279: bool,
        byte: i64,
        gs_16243: bool,
        gs_16281: bool,
        ga_12050: ProductType5c790c8ef59cc8b2,
        ga_12068: ProductType5c790c8ef59cc8b2,
        gs_16297: bool,
        ga_12123: ProductType5c790c8ef59cc8b2,
        gs_16290: bool,
        ga_12134: ProductType5c790c8ef59cc8b2,
        mismatch: bool,
        gs_16295: bool,
        gs_16242: bool,
        gs_16245: bool,
        gs_16127: bool,
        match_vmid: bool,
        dbgtype: u8,
        nshadow_270: i128,
        bvr_match: bool,
        gs_16177: bool,
        ga_12107: ProductType5c790c8ef59cc8b2,
        gs_16305: bool,
        gs_16216: bool,
        gs_16282: bool,
        gs_16310: bool,
        gs_16178: bool,
        gs_16194: bool,
        return_value: ProductType8b847afc727d5818,
        gs_16175: bool,
        ga_12001: ProductType396b95aa74979079,
        ga_12071: ProductType5c790c8ef59cc8b2,
        gs_16292: bool,
        gs_16176: bool,
        ga_12119: ProductType5c790c8ef59cc8b2,
        top: i128,
        gs_16229: bool,
        ga_12083: ProductType5c790c8ef59cc8b2,
        gs_16158: bool,
        gs_16116: bool,
        gs_16135: bool,
        gs_16309: bool,
        gs_16272: bool,
        gs_16150: bool,
        b__8: u8,
        gs_16259: bool,
        bxvr_match: bool,
        gs_16145: bool,
        n: i128,
        bt2: bool,
        gs_16217: bool,
        gs_16218: bool,
        gs_16244: bool,
        bvr_match_valid: bool,
        gs_16132: bool,
        gs_16201: bool,
        ga_12011: ProductTypeea264718a40d3f4a,
        gs_16202: bool,
        b__1: u8,
        match_addr: bool,
        gs_16179: bool,
        topshadow_272: i128,
        gs_16248: bool,
        linking_enabled: bool,
        bvr_vmid: u16,
        gs_16180: bool,
        gs_16296: bool,
        gs_16108: bool,
        ga_12027: ProductType396b95aa74979079,
        gs_16253: bool,
        n_in: i128,
        vaddress: u64,
        linked_to: bool,
        isbreakpnt: bool,
    }
    let fn_state = FunctionState {
        n_in,
        vaddress,
        linked_to,
        isbreakpnt,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_0_0: read-var n_in:i
        let s_0_0: i128 = fn_state.n_in;
        // D s_0_1: write-var n <= s_0_0
        fn_state.n = s_0_0;
        // C s_0_2: const #() : ()
        let s_0_2: () = ();
        // S s_0_3: call NumBreakpointsImplemented(s_0_2)
        let s_0_3: i128 = NumBreakpointsImplemented(state, tracer, s_0_2);
        // D s_0_4: read-var n:i
        let s_0_4: i128 = fn_state.n;
        // D s_0_5: cmp-ge s_0_4 s_0_3
        let s_0_5: bool = ((s_0_4) >= (s_0_3));
        // N s_0_6: branch s_0_5 b234 b1
        if s_0_5 {
            return block_234(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_2_0: read-var n:i
        let s_2_0: i128 = fn_state.n;
        // D s_2_1: write-var nshadow#270 <= s_2_0
        fn_state.nshadow_270 = s_2_0;
        // D s_2_2: read-var nshadow#270:i
        let s_2_2: i128 = fn_state.nshadow_270;
        // D s_2_3: call __id(s_2_2)
        let s_2_3: i128 = u__id(state, tracer, s_2_2);
        // C s_2_4: const #0s : i
        let s_2_4: i128 = 0;
        // D s_2_5: cmp-le s_2_4 s_2_3
        let s_2_5: bool = ((s_2_4) <= (s_2_3));
        // N s_2_6: branch s_2_5 b233 b3
        if s_2_5 {
            return block_233(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#16116 <= s_3_0
        fn_state.gs_16116 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_4_0: read-var gs#16116:u8
        let s_4_0: bool = fn_state.gs_16116;
        // N s_4_1: assert s_4_0
        let s_4_1: () = assert!(s_4_0);
        // D s_4_2: read-var nshadow#270:i
        let s_4_2: i128 = fn_state.nshadow_270;
        // D s_4_3: cast reint s_4_2 -> i64
        let s_4_3: i64 = (s_4_2 as i64);
        // D s_4_4: call IsBreakpointEnabled(s_4_3)
        let s_4_4: bool = IsBreakpointEnabled(state, tracer, s_4_3);
        // D s_4_5: not s_4_4
        let s_4_5: bool = !s_4_4;
        // N s_4_6: branch s_4_5 b232 b5
        if s_4_5 {
            return block_232(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_5_0: const #12184u : u32
        let s_5_0: u32 = 12184;
        // D s_5_1: read-reg s_5_0:[struct; 64]
        let s_5_1: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 64usize]>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // D s_5_2: read-var nshadow#270:i
        let s_5_2: i128 = fn_state.nshadow_270;
        // D s_5_3: read-element s_5_1[s_5_2]
        let s_5_3: ProductType5c790c8ef59cc8b2 = s_5_1[(s_5_2) as usize];
        // D s_5_4: call _get_DBGBCR_EL1_Type_BT(s_5_3)
        let s_5_4: u8 = u_get_DBGBCR_EL1_Type_BT(state, tracer, s_5_3);
        // D s_5_5: write-var dbgtype <= s_5_4
        fn_state.dbgtype = s_5_4;
        // C s_5_6: const #() : ()
        let s_5_6: () = ();
        // S s_5_7: call HaveFeatABLE(s_5_6)
        let s_5_7: bool = HaveFeatABLE(state, tracer, s_5_6);
        // N s_5_8: branch s_5_7 b231 b6
        if s_5_7 {
            return block_231(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var bt2 <= s_6_0
        fn_state.bt2 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_7_0: read-var nshadow#270:i
        let s_7_0: i128 = fn_state.nshadow_270;
        // D s_7_1: read-var bt2:u8
        let s_7_1: bool = fn_state.bt2;
        // D s_7_2: read-var dbgtype:u8
        let s_7_2: u8 = fn_state.dbgtype;
        // D s_7_3: call AArch64_ReservedBreakpointType(s_7_0, s_7_1, s_7_2)
        let s_7_3: ProductTypeea264718a40d3f4a = AArch64_ReservedBreakpointType(
            state,
            tracer,
            s_7_0,
            s_7_1,
            s_7_2,
        );
        // D s_7_4: write-var ga#12011 <= s_7_3
        fn_state.ga_12011 = s_7_3;
        // D s_7_5: read-var ga#12011.0:struct
        let s_7_5: u32 = fn_state.ga_12011._0;
        // D s_7_6: read-var ga#12011.1:struct
        let s_7_6: bool = fn_state.ga_12011._1;
        // D s_7_7: read-var ga#12011.2:struct
        let s_7_7: u8 = fn_state.ga_12011._2;
        // D s_7_8: write-var c <= s_7_5
        fn_state.c = s_7_5;
        // D s_7_9: write-var bt2 <= s_7_6
        fn_state.bt2 = s_7_6;
        // D s_7_10: write-var dbgtype <= s_7_7
        fn_state.dbgtype = s_7_7;
        // D s_7_11: read-var c:u32
        let s_7_11: u32 = fn_state.c;
        // C s_7_12: const #7u : u32
        let s_7_12: u32 = 7;
        // D s_7_13: cmp-eq s_7_11 s_7_12
        let s_7_13: bool = ((s_7_11) == (s_7_12));
        // N s_7_14: branch s_7_13 b230 b8
        if s_7_13 {
            return block_230(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_8_0: read-var dbgtype:u8
        let s_8_0: u8 = fn_state.dbgtype;
        // D s_8_1: write-var b__8 <= s_8_0
        fn_state.b__8 = s_8_0;
        // C s_8_2: const #3s : i
        let s_8_2: i128 = 3;
        // D s_8_3: read-var b__8:u8
        let s_8_3: u8 = fn_state.b__8;
        // D s_8_4: cast zx s_8_3 -> bv
        let s_8_4: Bits = Bits::new(s_8_3 as u128, 4u16);
        // C s_8_5: const #1s : i64
        let s_8_5: i64 = 1;
        // C s_8_6: cast zx s_8_5 -> i
        let s_8_6: i128 = (i128::try_from(s_8_5).unwrap());
        // C s_8_7: const #0s : i
        let s_8_7: i128 = 0;
        // C s_8_8: add s_8_7 s_8_6
        let s_8_8: i128 = (s_8_7 + s_8_6);
        // D s_8_9: bit-extract s_8_4 s_8_2 s_8_8
        let s_8_9: Bits = (Bits::new(
            ((s_8_4) >> (s_8_2)).value(),
            u16::try_from(s_8_8).unwrap(),
        ));
        // D s_8_10: cast reint s_8_9 -> u8
        let s_8_10: bool = ((s_8_9.value()) != 0);
        // D s_8_11: cast zx s_8_10 -> bv
        let s_8_11: Bits = Bits::new(s_8_10 as u128, 1u16);
        // C s_8_12: const #0u : u8
        let s_8_12: bool = false;
        // C s_8_13: cast zx s_8_12 -> bv
        let s_8_13: Bits = Bits::new(s_8_12 as u128, 1u16);
        // D s_8_14: cmp-eq s_8_11 s_8_13
        let s_8_14: bool = ((s_8_11) == (s_8_13));
        // N s_8_15: branch s_8_14 b229 b9
        if s_8_14 {
            return block_229(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#16132 <= s_9_0
        fn_state.gs_16132 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_10_0: read-var gs#16132:u8
        let s_10_0: bool = fn_state.gs_16132;
        // D s_10_1: not s_10_0
        let s_10_1: bool = !s_10_0;
        // N s_10_2: branch s_10_1 b228 b11
        if s_10_1 {
            return block_228(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_11_0: const #1u : u8
        let s_11_0: bool = true;
        // D s_11_1: write-var gs#16127 <= s_11_0
        fn_state.gs_16127 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_12_0: read-var gs#16127:u8
        let s_12_0: bool = fn_state.gs_16127;
        // D s_12_1: write-var match_addr <= s_12_0
        fn_state.match_addr = s_12_0;
        // D s_12_2: read-var dbgtype:u8
        let s_12_2: u8 = fn_state.dbgtype;
        // C s_12_3: const #1s : i
        let s_12_3: i128 = 1;
        // D s_12_4: cast zx s_12_2 -> bv
        let s_12_4: Bits = Bits::new(s_12_2 as u128, 4u16);
        // C s_12_5: const #1s : i64
        let s_12_5: i64 = 1;
        // C s_12_6: cast zx s_12_5 -> i
        let s_12_6: i128 = (i128::try_from(s_12_5).unwrap());
        // C s_12_7: const #2s : i
        let s_12_7: i128 = 2;
        // C s_12_8: add s_12_7 s_12_6
        let s_12_8: i128 = (s_12_7 + s_12_6);
        // D s_12_9: bit-extract s_12_4 s_12_3 s_12_8
        let s_12_9: Bits = (Bits::new(
            ((s_12_4) >> (s_12_3)).value(),
            u16::try_from(s_12_8).unwrap(),
        ));
        // D s_12_10: cast reint s_12_9 -> u8
        let s_12_10: u8 = (s_12_9.value() as u8);
        // D s_12_11: cast zx s_12_10 -> bv
        let s_12_11: Bits = Bits::new(s_12_10 as u128, 3u16);
        // C s_12_12: const #2u : u8
        let s_12_12: u8 = 2;
        // C s_12_13: cast zx s_12_12 -> bv
        let s_12_13: Bits = Bits::new(s_12_12 as u128, 3u16);
        // D s_12_14: cmp-eq s_12_11 s_12_13
        let s_12_14: bool = ((s_12_11) == (s_12_13));
        // D s_12_15: not s_12_14
        let s_12_15: bool = !s_12_14;
        // N s_12_16: branch s_12_15 b227 b13
        if s_12_15 {
            return block_227(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_13_0: const #1u : u8
        let s_13_0: bool = true;
        // D s_13_1: write-var gs#16135 <= s_13_0
        fn_state.gs_16135 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_14_0: read-var gs#16135:u8
        let s_14_0: bool = fn_state.gs_16135;
        // D s_14_1: write-var mismatch <= s_14_0
        fn_state.mismatch = s_14_0;
        // D s_14_2: read-var dbgtype:u8
        let s_14_2: u8 = fn_state.dbgtype;
        // C s_14_3: const #2s : i
        let s_14_3: i128 = 2;
        // D s_14_4: cast zx s_14_2 -> bv
        let s_14_4: Bits = Bits::new(s_14_2 as u128, 4u16);
        // C s_14_5: const #1s : i64
        let s_14_5: i64 = 1;
        // C s_14_6: cast zx s_14_5 -> i
        let s_14_6: i128 = (i128::try_from(s_14_5).unwrap());
        // C s_14_7: const #1s : i
        let s_14_7: i128 = 1;
        // C s_14_8: add s_14_7 s_14_6
        let s_14_8: i128 = (s_14_7 + s_14_6);
        // D s_14_9: bit-extract s_14_4 s_14_3 s_14_8
        let s_14_9: Bits = (Bits::new(
            ((s_14_4) >> (s_14_3)).value(),
            u16::try_from(s_14_8).unwrap(),
        ));
        // D s_14_10: cast reint s_14_9 -> u8
        let s_14_10: u8 = (s_14_9.value() as u8);
        // D s_14_11: cast zx s_14_10 -> bv
        let s_14_11: Bits = Bits::new(s_14_10 as u128, 2u16);
        // C s_14_12: const #2u : u8
        let s_14_12: u8 = 2;
        // C s_14_13: cast zx s_14_12 -> bv
        let s_14_13: Bits = Bits::new(s_14_12 as u128, 2u16);
        // D s_14_14: cmp-eq s_14_11 s_14_13
        let s_14_14: bool = ((s_14_11) == (s_14_13));
        // D s_14_15: not s_14_14
        let s_14_15: bool = !s_14_14;
        // N s_14_16: branch s_14_15 b226 b15
        if s_14_15 {
            return block_226(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // D s_15_1: write-var gs#16140 <= s_15_0
        fn_state.gs_16140 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_16_0: read-var gs#16140:u8
        let s_16_0: bool = fn_state.gs_16140;
        // D s_16_1: write-var match_vmid <= s_16_0
        fn_state.match_vmid = s_16_0;
        // D s_16_2: read-var dbgtype:u8
        let s_16_2: u8 = fn_state.dbgtype;
        // C s_16_3: const #1s : i
        let s_16_3: i128 = 1;
        // D s_16_4: cast zx s_16_2 -> bv
        let s_16_4: Bits = Bits::new(s_16_2 as u128, 4u16);
        // C s_16_5: const #1s : i64
        let s_16_5: i64 = 1;
        // C s_16_6: cast zx s_16_5 -> i
        let s_16_6: i128 = (i128::try_from(s_16_5).unwrap());
        // C s_16_7: const #2s : i
        let s_16_7: i128 = 2;
        // C s_16_8: add s_16_7 s_16_6
        let s_16_8: i128 = (s_16_7 + s_16_6);
        // D s_16_9: bit-extract s_16_4 s_16_3 s_16_8
        let s_16_9: Bits = (Bits::new(
            ((s_16_4) >> (s_16_3)).value(),
            u16::try_from(s_16_8).unwrap(),
        ));
        // D s_16_10: cast reint s_16_9 -> u8
        let s_16_10: u8 = (s_16_9.value() as u8);
        // D s_16_11: cast zx s_16_10 -> bv
        let s_16_11: Bits = Bits::new(s_16_10 as u128, 3u16);
        // C s_16_12: const #1u : u8
        let s_16_12: u8 = 1;
        // C s_16_13: cast zx s_16_12 -> bv
        let s_16_13: Bits = Bits::new(s_16_12 as u128, 3u16);
        // D s_16_14: cmp-eq s_16_11 s_16_13
        let s_16_14: bool = ((s_16_11) == (s_16_13));
        // D s_16_15: not s_16_14
        let s_16_15: bool = !s_16_14;
        // N s_16_16: branch s_16_15 b225 b17
        if s_16_15 {
            return block_225(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_17_0: const #1u : u8
        let s_17_0: bool = true;
        // D s_17_1: write-var gs#16145 <= s_17_0
        fn_state.gs_16145 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_18_0: read-var gs#16145:u8
        let s_18_0: bool = fn_state.gs_16145;
        // D s_18_1: write-var match_cid <= s_18_0
        fn_state.match_cid = s_18_0;
        // D s_18_2: read-var dbgtype:u8
        let s_18_2: u8 = fn_state.dbgtype;
        // C s_18_3: const #1s : i
        let s_18_3: i128 = 1;
        // D s_18_4: cast zx s_18_2 -> bv
        let s_18_4: Bits = Bits::new(s_18_2 as u128, 4u16);
        // C s_18_5: const #1s : i64
        let s_18_5: i64 = 1;
        // C s_18_6: cast zx s_18_5 -> i
        let s_18_6: i128 = (i128::try_from(s_18_5).unwrap());
        // C s_18_7: const #2s : i
        let s_18_7: i128 = 2;
        // C s_18_8: add s_18_7 s_18_6
        let s_18_8: i128 = (s_18_7 + s_18_6);
        // D s_18_9: bit-extract s_18_4 s_18_3 s_18_8
        let s_18_9: Bits = (Bits::new(
            ((s_18_4) >> (s_18_3)).value(),
            u16::try_from(s_18_8).unwrap(),
        ));
        // D s_18_10: cast reint s_18_9 -> u8
        let s_18_10: u8 = (s_18_9.value() as u8);
        // D s_18_11: cast zx s_18_10 -> bv
        let s_18_11: Bits = Bits::new(s_18_10 as u128, 3u16);
        // C s_18_12: const #5u : u8
        let s_18_12: u8 = 5;
        // C s_18_13: cast zx s_18_12 -> bv
        let s_18_13: Bits = Bits::new(s_18_12 as u128, 3u16);
        // D s_18_14: cmp-eq s_18_11 s_18_13
        let s_18_14: bool = ((s_18_11) == (s_18_13));
        // D s_18_15: not s_18_14
        let s_18_15: bool = !s_18_14;
        // N s_18_16: branch s_18_15 b222 b19
        if s_18_15 {
            return block_222(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_19_0: const #1u : u8
        let s_19_0: bool = true;
        // D s_19_1: write-var gs#16150 <= s_19_0
        fn_state.gs_16150 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_20_0: read-var gs#16150:u8
        let s_20_0: bool = fn_state.gs_16150;
        // D s_20_1: write-var match_cid1 <= s_20_0
        fn_state.match_cid1 = s_20_0;
        // D s_20_2: read-var dbgtype:u8
        let s_20_2: u8 = fn_state.dbgtype;
        // C s_20_3: const #2s : i
        let s_20_3: i128 = 2;
        // D s_20_4: cast zx s_20_2 -> bv
        let s_20_4: Bits = Bits::new(s_20_2 as u128, 4u16);
        // C s_20_5: const #1s : i64
        let s_20_5: i64 = 1;
        // C s_20_6: cast zx s_20_5 -> i
        let s_20_6: i128 = (i128::try_from(s_20_5).unwrap());
        // C s_20_7: const #1s : i
        let s_20_7: i128 = 1;
        // C s_20_8: add s_20_7 s_20_6
        let s_20_8: i128 = (s_20_7 + s_20_6);
        // D s_20_9: bit-extract s_20_4 s_20_3 s_20_8
        let s_20_9: Bits = (Bits::new(
            ((s_20_4) >> (s_20_3)).value(),
            u16::try_from(s_20_8).unwrap(),
        ));
        // D s_20_10: cast reint s_20_9 -> u8
        let s_20_10: u8 = (s_20_9.value() as u8);
        // D s_20_11: cast zx s_20_10 -> bv
        let s_20_11: Bits = Bits::new(s_20_10 as u128, 2u16);
        // C s_20_12: const #3u : u8
        let s_20_12: u8 = 3;
        // C s_20_13: cast zx s_20_12 -> bv
        let s_20_13: Bits = Bits::new(s_20_12 as u128, 2u16);
        // D s_20_14: cmp-eq s_20_11 s_20_13
        let s_20_14: bool = ((s_20_11) == (s_20_13));
        // D s_20_15: not s_20_14
        let s_20_15: bool = !s_20_14;
        // N s_20_16: branch s_20_15 b221 b21
        if s_20_15 {
            return block_221(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var gs#16158 <= s_21_0
        fn_state.gs_16158 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_22_0: read-var gs#16158:u8
        let s_22_0: bool = fn_state.gs_16158;
        // D s_22_1: write-var match_cid2 <= s_22_0
        fn_state.match_cid2 = s_22_0;
        // D s_22_2: read-var dbgtype:u8
        let s_22_2: u8 = fn_state.dbgtype;
        // C s_22_3: const #0s : i
        let s_22_3: i128 = 0;
        // D s_22_4: cast zx s_22_2 -> bv
        let s_22_4: Bits = Bits::new(s_22_2 as u128, 4u16);
        // C s_22_5: const #1s : i64
        let s_22_5: i64 = 1;
        // C s_22_6: cast zx s_22_5 -> i
        let s_22_6: i128 = (i128::try_from(s_22_5).unwrap());
        // C s_22_7: const #1s : i
        let s_22_7: i128 = 1;
        // C s_22_8: add s_22_7 s_22_6
        let s_22_8: i128 = (s_22_7 + s_22_6);
        // D s_22_9: bit-extract s_22_4 s_22_3 s_22_8
        let s_22_9: Bits = (Bits::new(
            ((s_22_4) >> (s_22_3)).value(),
            u16::try_from(s_22_8).unwrap(),
        ));
        // D s_22_10: cast reint s_22_9 -> u8
        let s_22_10: u8 = (s_22_9.value() as u8);
        // D s_22_11: cast zx s_22_10 -> bv
        let s_22_11: Bits = Bits::new(s_22_10 as u128, 2u16);
        // C s_22_12: const #3u : u8
        let s_22_12: u8 = 3;
        // C s_22_13: cast zx s_22_12 -> bv
        let s_22_13: Bits = Bits::new(s_22_12 as u128, 2u16);
        // D s_22_14: cmp-eq s_22_11 s_22_13
        let s_22_14: bool = ((s_22_11) == (s_22_13));
        // D s_22_15: not s_22_14
        let s_22_15: bool = !s_22_14;
        // N s_22_16: branch s_22_15 b215 b23
        if s_22_15 {
            return block_215(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_23_0: const #1u : u8
        let s_23_0: bool = true;
        // D s_23_1: write-var gs#16163 <= s_23_0
        fn_state.gs_16163 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_24_0: read-var gs#16163:u8
        let s_24_0: bool = fn_state.gs_16163;
        // N s_24_1: branch s_24_0 b214 b25
        if s_24_0 {
            return block_214(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_25_0: read-var bt2:u8
        let s_25_0: bool = fn_state.bt2;
        // D s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 1u16);
        // C s_25_2: const #1u : u8
        let s_25_2: bool = true;
        // C s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 1u16);
        // D s_25_4: cmp-eq s_25_1 s_25_3
        let s_25_4: bool = ((s_25_1) == (s_25_3));
        // D s_25_5: write-var gs#16174 <= s_25_4
        fn_state.gs_16174 = s_25_4;
        // N s_25_6: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_26_0: read-var gs#16174:u8
        let s_26_0: bool = fn_state.gs_16174;
        // D s_26_1: write-var linking_enabled <= s_26_0
        fn_state.linking_enabled = s_26_0;
        // D s_26_2: read-var linked_to:u8
        let s_26_2: bool = fn_state.linked_to;
        // N s_26_3: branch s_26_2 b213 b27
        if s_26_2 {
            return block_213(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_27_0: const #0u : u8
        let s_27_0: bool = false;
        // D s_27_1: write-var gs#16175 <= s_27_0
        fn_state.gs_16175 = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_28_0: read-var gs#16175:u8
        let s_28_0: bool = fn_state.gs_16175;
        // N s_28_1: branch s_28_0 b212 b29
        if s_28_0 {
            return block_212(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_29_0: read-var linked_to:u8
        let s_29_0: bool = fn_state.linked_to;
        // D s_29_1: not s_29_0
        let s_29_1: bool = !s_29_0;
        // N s_29_2: branch s_29_1 b211 b30
        if s_29_1 {
            return block_211(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_30_0: const #0u : u8
        let s_30_0: bool = false;
        // D s_30_1: write-var gs#16176 <= s_30_0
        fn_state.gs_16176 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_31_0: read-var gs#16176:u8
        let s_31_0: bool = fn_state.gs_16176;
        // N s_31_1: branch s_31_0 b210 b32
        if s_31_0 {
            return block_210(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_32_0: const #0u : u8
        let s_32_0: bool = false;
        // D s_32_1: write-var gs#16177 <= s_32_0
        fn_state.gs_16177 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_33_0: read-var gs#16177:u8
        let s_33_0: bool = fn_state.gs_16177;
        // N s_33_1: branch s_33_0 b209 b34
        if s_33_0 {
            return block_209(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_34_0: read-var linked_to:u8
        let s_34_0: bool = fn_state.linked_to;
        // N s_34_1: branch s_34_0 b208 b35
        if s_34_0 {
            return block_208(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_35_0: const #0u : u8
        let s_35_0: bool = false;
        // D s_35_1: write-var gs#16178 <= s_35_0
        fn_state.gs_16178 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_36_0: read-var gs#16178:u8
        let s_36_0: bool = fn_state.gs_16178;
        // N s_36_1: branch s_36_0 b207 b37
        if s_36_0 {
            return block_207(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_37_0: const #0u : u8
        let s_37_0: bool = false;
        // D s_37_1: write-var gs#16179 <= s_37_0
        fn_state.gs_16179 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_38_0: read-var gs#16179:u8
        let s_38_0: bool = fn_state.gs_16179;
        // N s_38_1: branch s_38_0 b204 b39
        if s_38_0 {
            return block_204(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // N s_39_0: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_40_0: read-var mismatch:u8
        let s_40_0: bool = fn_state.mismatch;
        // N s_40_1: branch s_40_0 b203 b41
        if s_40_0 {
            return block_203(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_41_0: const #0u : u8
        let s_41_0: bool = false;
        // D s_41_1: write-var gs#16180 <= s_41_0
        fn_state.gs_16180 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_42_0: read-var gs#16180:u8
        let s_42_0: bool = fn_state.gs_16180;
        // N s_42_1: branch s_42_0 b202 b43
        if s_42_0 {
            return block_202(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_43_0: const #0u : u8
        let s_43_0: bool = false;
        // D s_43_1: write-var bvr_match <= s_43_0
        fn_state.bvr_match = s_43_0;
        // C s_43_2: const #0u : u8
        let s_43_2: bool = false;
        // D s_43_3: write-var bxvr_match <= s_43_2
        fn_state.bxvr_match = s_43_2;
        // C s_43_4: const #() : ()
        let s_43_4: () = ();
        // S s_43_5: call HaveFeatABLE(s_43_4)
        let s_43_5: bool = HaveFeatABLE(state, tracer, s_43_4);
        // N s_43_6: branch s_43_5 b163 b44
        if s_43_5 {
            return block_163(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_44_0: const #0s : i
        let s_44_0: i128 = 0;
        // D s_44_1: write-var mask <= s_44_0
        fn_state.mask = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_45_0: read-var mask:i
        let s_45_0: i128 = fn_state.mask;
        // D s_45_1: write-var maskshadow#271 <= s_45_0
        fn_state.maskshadow_271 = s_45_0;
        // D s_45_2: read-var match_addr:u8
        let s_45_2: bool = fn_state.match_addr;
        // N s_45_3: branch s_45_2 b117 b46
        if s_45_2 {
            return block_117(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_46_0: read-var match_cid:u8
        let s_46_0: bool = fn_state.match_cid;
        // N s_46_1: branch s_46_0 b108 b47
        if s_46_0 {
            return block_108(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_47_0: read-var match_cid1:u8
        let s_47_0: bool = fn_state.match_cid1;
        // N s_47_1: branch s_47_0 b98 b48
        if s_47_0 {
            return block_98(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // N s_48_0: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_49_0: read-var match_vmid:u8
        let s_49_0: bool = fn_state.match_vmid;
        // N s_49_1: branch s_49_0 b79 b50
        if s_49_0 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_50_0: read-var match_cid2:u8
        let s_50_0: bool = fn_state.match_cid2;
        // N s_50_1: branch s_50_0 b72 b51
        if s_50_0 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // N s_51_0: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_52_0: read-var match_addr:u8
        let s_52_0: bool = fn_state.match_addr;
        // N s_52_1: branch s_52_0 b71 b53
        if s_52_0 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_53_0: read-var match_cid:u8
        let s_53_0: bool = fn_state.match_cid;
        // D s_53_1: write-var gs#16305 <= s_53_0
        fn_state.gs_16305 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_54_0: read-var gs#16305:u8
        let s_54_0: bool = fn_state.gs_16305;
        // N s_54_1: branch s_54_0 b70 b55
        if s_54_0 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_55_0: read-var match_cid1:u8
        let s_55_0: bool = fn_state.match_cid1;
        // D s_55_1: write-var gs#16306 <= s_55_0
        fn_state.gs_16306 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_56_0: read-var gs#16306:u8
        let s_56_0: bool = fn_state.gs_16306;
        // D s_56_1: write-var bvr_match_valid <= s_56_0
        fn_state.bvr_match_valid = s_56_0;
        // D s_56_2: read-var match_vmid:u8
        let s_56_2: bool = fn_state.match_vmid;
        // N s_56_3: branch s_56_2 b69 b57
        if s_56_2 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_57_0: read-var match_cid2:u8
        let s_57_0: bool = fn_state.match_cid2;
        // D s_57_1: write-var gs#16307 <= s_57_0
        fn_state.gs_16307 = s_57_0;
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_58_0: read-var gs#16307:u8
        let s_58_0: bool = fn_state.gs_16307;
        // D s_58_1: not s_58_0
        let s_58_1: bool = !s_58_0;
        // N s_58_2: branch s_58_1 b68 b59
        if s_58_1 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_59_0: read-var bxvr_match:u8
        let s_59_0: bool = fn_state.bxvr_match;
        // D s_59_1: write-var gs#16308 <= s_59_0
        fn_state.gs_16308 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_60_0: read-var gs#16308:u8
        let s_60_0: bool = fn_state.gs_16308;
        // N s_60_1: branch s_60_0 b64 b61
        if s_60_0 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_61_0: const #0u : u8
        let s_61_0: bool = false;
        // D s_61_1: write-var gs#16310 <= s_61_0
        fn_state.gs_16310 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_62_0: read-var gs#16310:u8
        let s_62_0: bool = fn_state.gs_16310;
        // D s_62_1: read-var mismatch:u8
        let s_62_1: bool = fn_state.mismatch;
        // D s_62_2: create-product struct = ["s_62_0", "s_62_1"]
        let s_62_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_62_0,
            _1: s_62_1,
        };
        // D s_62_3: write-var return_value <= s_62_2
        fn_state.return_value = s_62_2;
        // N s_62_4: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_63_0: read-var return_value:struct
        let s_63_0: ProductType8b847afc727d5818 = fn_state.return_value;
        // N s_63_1: return s_63_0
        return s_63_0;
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_64_0: read-var bvr_match_valid:u8
        let s_64_0: bool = fn_state.bvr_match_valid;
        // D s_64_1: not s_64_0
        let s_64_1: bool = !s_64_0;
        // N s_64_2: branch s_64_1 b67 b65
        if s_64_1 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_65_0: read-var bvr_match:u8
        let s_65_0: bool = fn_state.bvr_match;
        // D s_65_1: write-var gs#16309 <= s_65_0
        fn_state.gs_16309 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_66_0: read-var gs#16309:u8
        let s_66_0: bool = fn_state.gs_16309;
        // D s_66_1: write-var gs#16310 <= s_66_0
        fn_state.gs_16310 = s_66_0;
        // N s_66_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_67_0: const #1u : u8
        let s_67_0: bool = true;
        // D s_67_1: write-var gs#16309 <= s_67_0
        fn_state.gs_16309 = s_67_0;
        // N s_67_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_68_0: const #1u : u8
        let s_68_0: bool = true;
        // D s_68_1: write-var gs#16308 <= s_68_0
        fn_state.gs_16308 = s_68_0;
        // N s_68_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_69_0: const #1u : u8
        let s_69_0: bool = true;
        // D s_69_1: write-var gs#16307 <= s_69_0
        fn_state.gs_16307 = s_69_0;
        // N s_69_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_70_0: const #1u : u8
        let s_70_0: bool = true;
        // D s_70_1: write-var gs#16306 <= s_70_0
        fn_state.gs_16306 = s_70_0;
        // N s_70_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_71_0: const #1u : u8
        let s_71_0: bool = true;
        // D s_71_1: write-var gs#16305 <= s_71_0
        fn_state.gs_16305 = s_71_0;
        // N s_71_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_72_0: const #16975u : u32
        let s_72_0: u32 = 16975;
        // D s_72_1: read-reg s_72_0:u8
        let s_72_1: u8 = {
            let value = state.read_register::<u8>(s_72_0 as isize);
            tracer.read_register(s_72_0 as isize, value);
            value
        };
        // D s_72_2: cast zx s_72_1 -> bv
        let s_72_2: Bits = Bits::new(s_72_1 as u128, 2u16);
        // C s_72_3: const #424u : u32
        let s_72_3: u32 = 424;
        // D s_72_4: read-reg s_72_3:u8
        let s_72_4: u8 = {
            let value = state.read_register::<u8>(s_72_3 as isize);
            tracer.read_register(s_72_3 as isize, value);
            value
        };
        // D s_72_5: cast zx s_72_4 -> bv
        let s_72_5: Bits = Bits::new(s_72_4 as u128, 2u16);
        // D s_72_6: cmp-ne s_72_2 s_72_5
        let s_72_6: bool = ((s_72_2) != (s_72_5));
        // N s_72_7: branch s_72_6 b78 b73
        if s_72_6 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_73_0: const #0u : u8
        let s_73_0: bool = false;
        // D s_73_1: write-var gs#16224 <= s_73_0
        fn_state.gs_16224 = s_73_0;
        // N s_73_2: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_74_0: read-var gs#16224:u8
        let s_74_0: bool = fn_state.gs_16224;
        // N s_74_1: branch s_74_0 b77 b75
        if s_74_0 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_75(state, tracer, fn_state);
        };
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_75_0: const #0u : u8
        let s_75_0: bool = false;
        // D s_75_1: write-var gs#16229 <= s_75_0
        fn_state.gs_16229 = s_75_0;
        // N s_75_2: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_76_0: read-var gs#16229:u8
        let s_76_0: bool = fn_state.gs_16229;
        // D s_76_1: write-var bxvr_match <= s_76_0
        fn_state.bxvr_match = s_76_0;
        // N s_76_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_77_0: const #103424u : u32
        let s_77_0: u32 = 103424;
        // D s_77_1: read-reg s_77_0:[struct; 64]
        let s_77_1: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_77_0 as isize);
            tracer.read_register(s_77_0 as isize, value);
            value
        };
        // D s_77_2: read-var nshadow#270:i
        let s_77_2: i128 = fn_state.nshadow_270;
        // D s_77_3: read-element s_77_1[s_77_2]
        let s_77_3: ProductType5c790c8ef59cc8b2 = s_77_1[(s_77_2) as usize];
        // D s_77_4: write-var ga#12134 <= s_77_3
        fn_state.ga_12134 = s_77_3;
        // D s_77_5: read-var ga#12134.0:struct
        let s_77_5: u64 = fn_state.ga_12134._0;
        // C s_77_6: const #32s : i
        let s_77_6: i128 = 32;
        // D s_77_7: cast zx s_77_5 -> bv
        let s_77_7: Bits = Bits::new(s_77_5 as u128, 64u16);
        // C s_77_8: const #1s : i64
        let s_77_8: i64 = 1;
        // C s_77_9: cast zx s_77_8 -> i
        let s_77_9: i128 = (i128::try_from(s_77_8).unwrap());
        // C s_77_10: const #31s : i
        let s_77_10: i128 = 31;
        // C s_77_11: add s_77_10 s_77_9
        let s_77_11: i128 = (s_77_10 + s_77_9);
        // D s_77_12: bit-extract s_77_7 s_77_6 s_77_11
        let s_77_12: Bits = (Bits::new(
            ((s_77_7) >> (s_77_6)).value(),
            u16::try_from(s_77_11).unwrap(),
        ));
        // D s_77_13: cast reint s_77_12 -> u32
        let s_77_13: u32 = (s_77_12.value() as u32);
        // C s_77_14: const #91008u : u32
        let s_77_14: u32 = 91008;
        // D s_77_15: read-reg s_77_14:u64
        let s_77_15: u64 = {
            let value = state.read_register::<u64>(s_77_14 as isize);
            tracer.read_register(s_77_14 as isize, value);
            value
        };
        // C s_77_16: const #0s : i
        let s_77_16: i128 = 0;
        // D s_77_17: cast zx s_77_15 -> bv
        let s_77_17: Bits = Bits::new(s_77_15 as u128, 64u16);
        // C s_77_18: const #1s : i64
        let s_77_18: i64 = 1;
        // C s_77_19: cast zx s_77_18 -> i
        let s_77_19: i128 = (i128::try_from(s_77_18).unwrap());
        // C s_77_20: const #31s : i
        let s_77_20: i128 = 31;
        // C s_77_21: add s_77_20 s_77_19
        let s_77_21: i128 = (s_77_20 + s_77_19);
        // D s_77_22: bit-extract s_77_17 s_77_16 s_77_21
        let s_77_22: Bits = (Bits::new(
            ((s_77_17) >> (s_77_16)).value(),
            u16::try_from(s_77_21).unwrap(),
        ));
        // D s_77_23: cast reint s_77_22 -> u32
        let s_77_23: u32 = (s_77_22.value() as u32);
        // D s_77_24: cast zx s_77_13 -> bv
        let s_77_24: Bits = Bits::new(s_77_13 as u128, 32u16);
        // D s_77_25: cast zx s_77_23 -> bv
        let s_77_25: Bits = Bits::new(s_77_23 as u128, 32u16);
        // D s_77_26: cmp-eq s_77_24 s_77_25
        let s_77_26: bool = ((s_77_24) == (s_77_25));
        // D s_77_27: write-var gs#16229 <= s_77_26
        fn_state.gs_16229 = s_77_26;
        // N s_77_28: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_78_0: const #() : ()
        let s_78_0: () = ();
        // S s_78_1: call EL2Enabled(s_78_0)
        let s_78_1: bool = EL2Enabled(state, tracer, s_78_0);
        // D s_78_2: write-var gs#16224 <= s_78_1
        fn_state.gs_16224 = s_78_1;
        // N s_78_3: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_79_0: const #() : ()
        let s_79_0: () = ();
        // S s_79_1: call Have16bitVMID(s_79_0)
        let s_79_1: bool = Have16bitVMID(state, tracer, s_79_0);
        // S s_79_2: not s_79_1
        let s_79_2: bool = !s_79_1;
        // N s_79_3: branch s_79_2 b97 b80
        if s_79_2 {
            return block_97(state, tracer, fn_state);
        } else {
            return block_80(state, tracer, fn_state);
        };
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_80_0: const #15328u : u32
        let s_80_0: u32 = 15328;
        // D s_80_1: read-reg s_80_0:struct
        let s_80_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_80_0 as isize);
            tracer.read_register(s_80_0 as isize, value);
            value
        };
        // D s_80_2: call _get_VTCR_EL2_Type_VS(s_80_1)
        let s_80_2: bool = u_get_VTCR_EL2_Type_VS(state, tracer, s_80_1);
        // D s_80_3: cast zx s_80_2 -> bv
        let s_80_3: Bits = Bits::new(s_80_2 as u128, 1u16);
        // C s_80_4: const #0u : u8
        let s_80_4: bool = false;
        // C s_80_5: cast zx s_80_4 -> bv
        let s_80_5: Bits = Bits::new(s_80_4 as u128, 1u16);
        // D s_80_6: cmp-eq s_80_3 s_80_5
        let s_80_6: bool = ((s_80_3) == (s_80_5));
        // D s_80_7: write-var gs#16230 <= s_80_6
        fn_state.gs_16230 = s_80_6;
        // N s_80_8: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_81_0: read-var gs#16230:u8
        let s_81_0: bool = fn_state.gs_16230;
        // N s_81_1: branch s_81_0 b96 b82
        if s_81_0 {
            return block_96(state, tracer, fn_state);
        } else {
            return block_82(state, tracer, fn_state);
        };
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_82_0: const #() : ()
        let s_82_0: () = ();
        // S s_82_1: call VTTBR_EL2_read(s_82_0)
        let s_82_1: ProductType782ac6922b48c20d = VTTBR_EL2_read(state, tracer, s_82_0);
        // S s_82_2: call _get_VTTBR_EL2_Type_VMID(s_82_1)
        let s_82_2: u16 = u_get_VTTBR_EL2_Type_VMID(state, tracer, s_82_1);
        // D s_82_3: write-var vmid <= s_82_2
        fn_state.vmid = s_82_2;
        // C s_82_4: const #103424u : u32
        let s_82_4: u32 = 103424;
        // D s_82_5: read-reg s_82_4:[struct; 64]
        let s_82_5: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_82_4 as isize);
            tracer.read_register(s_82_4 as isize, value);
            value
        };
        // D s_82_6: read-var nshadow#270:i
        let s_82_6: i128 = fn_state.nshadow_270;
        // D s_82_7: read-element s_82_5[s_82_6]
        let s_82_7: ProductType5c790c8ef59cc8b2 = s_82_5[(s_82_6) as usize];
        // D s_82_8: write-var ga#12123 <= s_82_7
        fn_state.ga_12123 = s_82_7;
        // D s_82_9: read-var ga#12123.0:struct
        let s_82_9: u64 = fn_state.ga_12123._0;
        // C s_82_10: const #32s : i
        let s_82_10: i128 = 32;
        // D s_82_11: cast zx s_82_9 -> bv
        let s_82_11: Bits = Bits::new(s_82_9 as u128, 64u16);
        // C s_82_12: const #1s : i64
        let s_82_12: i64 = 1;
        // C s_82_13: cast zx s_82_12 -> i
        let s_82_13: i128 = (i128::try_from(s_82_12).unwrap());
        // C s_82_14: const #15s : i
        let s_82_14: i128 = 15;
        // C s_82_15: add s_82_14 s_82_13
        let s_82_15: i128 = (s_82_14 + s_82_13);
        // D s_82_16: bit-extract s_82_11 s_82_10 s_82_15
        let s_82_16: Bits = (Bits::new(
            ((s_82_11) >> (s_82_10)).value(),
            u16::try_from(s_82_15).unwrap(),
        ));
        // D s_82_17: cast reint s_82_16 -> u16
        let s_82_17: u16 = (s_82_16.value() as u16);
        // D s_82_18: write-var bvr_vmid <= s_82_17
        fn_state.bvr_vmid = s_82_17;
        // N s_82_19: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_83_0: const #16975u : u32
        let s_83_0: u32 = 16975;
        // D s_83_1: read-reg s_83_0:u8
        let s_83_1: u8 = {
            let value = state.read_register::<u8>(s_83_0 as isize);
            tracer.read_register(s_83_0 as isize, value);
            value
        };
        // D s_83_2: cast zx s_83_1 -> bv
        let s_83_2: Bits = Bits::new(s_83_1 as u128, 2u16);
        // C s_83_3: const #448u : u32
        let s_83_3: u32 = 448;
        // D s_83_4: read-reg s_83_3:u8
        let s_83_4: u8 = {
            let value = state.read_register::<u8>(s_83_3 as isize);
            tracer.read_register(s_83_3 as isize, value);
            value
        };
        // D s_83_5: cast zx s_83_4 -> bv
        let s_83_5: Bits = Bits::new(s_83_4 as u128, 2u16);
        // D s_83_6: cmp-eq s_83_2 s_83_5
        let s_83_6: bool = ((s_83_2) == (s_83_5));
        // N s_83_7: branch s_83_6 b95 b84
        if s_83_6 {
            return block_95(state, tracer, fn_state);
        } else {
            return block_84(state, tracer, fn_state);
        };
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_84_0: const #16975u : u32
        let s_84_0: u32 = 16975;
        // D s_84_1: read-reg s_84_0:u8
        let s_84_1: u8 = {
            let value = state.read_register::<u8>(s_84_0 as isize);
            tracer.read_register(s_84_0 as isize, value);
            value
        };
        // D s_84_2: cast zx s_84_1 -> bv
        let s_84_2: Bits = Bits::new(s_84_1 as u128, 2u16);
        // C s_84_3: const #440u : u32
        let s_84_3: u32 = 440;
        // D s_84_4: read-reg s_84_3:u8
        let s_84_4: u8 = {
            let value = state.read_register::<u8>(s_84_3 as isize);
            tracer.read_register(s_84_3 as isize, value);
            value
        };
        // D s_84_5: cast zx s_84_4 -> bv
        let s_84_5: Bits = Bits::new(s_84_4 as u128, 2u16);
        // D s_84_6: cmp-eq s_84_2 s_84_5
        let s_84_6: bool = ((s_84_2) == (s_84_5));
        // D s_84_7: write-var gs#16242 <= s_84_6
        fn_state.gs_16242 = s_84_6;
        // N s_84_8: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_85_0: read-var gs#16242:u8
        let s_85_0: bool = fn_state.gs_16242;
        // N s_85_1: branch s_85_0 b94 b86
        if s_85_0 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_86(state, tracer, fn_state);
        };
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_86_0: const #0u : u8
        let s_86_0: bool = false;
        // D s_86_1: write-var gs#16243 <= s_86_0
        fn_state.gs_16243 = s_86_0;
        // N s_86_2: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_87_0: read-var gs#16243:u8
        let s_87_0: bool = fn_state.gs_16243;
        // N s_87_1: branch s_87_0 b93 b88
        if s_87_0 {
            return block_93(state, tracer, fn_state);
        } else {
            return block_88(state, tracer, fn_state);
        };
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_88_0: const #0u : u8
        let s_88_0: bool = false;
        // D s_88_1: write-var gs#16244 <= s_88_0
        fn_state.gs_16244 = s_88_0;
        // N s_88_2: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_89_0: read-var gs#16244:u8
        let s_89_0: bool = fn_state.gs_16244;
        // N s_89_1: branch s_89_0 b92 b90
        if s_89_0 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_90(state, tracer, fn_state);
        };
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_90_0: const #0u : u8
        let s_90_0: bool = false;
        // D s_90_1: write-var gs#16245 <= s_90_0
        fn_state.gs_16245 = s_90_0;
        // N s_90_2: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_91_0: read-var gs#16245:u8
        let s_91_0: bool = fn_state.gs_16245;
        // D s_91_1: write-var bxvr_match <= s_91_0
        fn_state.bxvr_match = s_91_0;
        // N s_91_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_92_0: read-var vmid:u16
        let s_92_0: u16 = fn_state.vmid;
        // D s_92_1: cast zx s_92_0 -> bv
        let s_92_1: Bits = Bits::new(s_92_0 as u128, 16u16);
        // D s_92_2: read-var bvr_vmid:u16
        let s_92_2: u16 = fn_state.bvr_vmid;
        // D s_92_3: cast zx s_92_2 -> bv
        let s_92_3: Bits = Bits::new(s_92_2 as u128, 16u16);
        // D s_92_4: cmp-eq s_92_1 s_92_3
        let s_92_4: bool = ((s_92_1) == (s_92_3));
        // D s_92_5: write-var gs#16245 <= s_92_4
        fn_state.gs_16245 = s_92_4;
        // N s_92_6: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_93_0: const #() : ()
        let s_93_0: () = ();
        // S s_93_1: call IsInHost(s_93_0)
        let s_93_1: bool = IsInHost(state, tracer, s_93_0);
        // S s_93_2: not s_93_1
        let s_93_2: bool = !s_93_1;
        // D s_93_3: write-var gs#16244 <= s_93_2
        fn_state.gs_16244 = s_93_2;
        // N s_93_4: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_94_0: const #() : ()
        let s_94_0: () = ();
        // S s_94_1: call EL2Enabled(s_94_0)
        let s_94_1: bool = EL2Enabled(state, tracer, s_94_0);
        // D s_94_2: write-var gs#16243 <= s_94_1
        fn_state.gs_16243 = s_94_1;
        // N s_94_3: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_95_0: const #1u : u8
        let s_95_0: bool = true;
        // D s_95_1: write-var gs#16242 <= s_95_0
        fn_state.gs_16242 = s_95_0;
        // N s_95_2: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_96_0: const #() : ()
        let s_96_0: () = ();
        // S s_96_1: call VTTBR_EL2_read(s_96_0)
        let s_96_1: ProductType782ac6922b48c20d = VTTBR_EL2_read(state, tracer, s_96_0);
        // S s_96_2: call _get_VTTBR_EL2_Type_VMID(s_96_1)
        let s_96_2: u16 = u_get_VTTBR_EL2_Type_VMID(state, tracer, s_96_1);
        // C s_96_3: const #0s : i
        let s_96_3: i128 = 0;
        // S s_96_4: cast zx s_96_2 -> bv
        let s_96_4: Bits = Bits::new(s_96_2 as u128, 16u16);
        // C s_96_5: const #1s : i64
        let s_96_5: i64 = 1;
        // C s_96_6: cast zx s_96_5 -> i
        let s_96_6: i128 = (i128::try_from(s_96_5).unwrap());
        // C s_96_7: const #7s : i
        let s_96_7: i128 = 7;
        // C s_96_8: add s_96_7 s_96_6
        let s_96_8: i128 = (s_96_7 + s_96_6);
        // D s_96_9: bit-extract s_96_4 s_96_3 s_96_8
        let s_96_9: Bits = (Bits::new(
            ((s_96_4) >> (s_96_3)).value(),
            u16::try_from(s_96_8).unwrap(),
        ));
        // D s_96_10: cast reint s_96_9 -> u8
        let s_96_10: u8 = (s_96_9.value() as u8);
        // C s_96_11: const #16s : i
        let s_96_11: i128 = 16;
        // D s_96_12: cast zx s_96_10 -> bv
        let s_96_12: Bits = Bits::new(s_96_10 as u128, 8u16);
        // D s_96_13: bits-cast zx s_96_12 -> bv length s_96_11
        let s_96_13: Bits = s_96_12.zero_extend(s_96_11);
        // D s_96_14: cast reint s_96_13 -> u16
        let s_96_14: u16 = (s_96_13.value() as u16);
        // D s_96_15: write-var vmid <= s_96_14
        fn_state.vmid = s_96_14;
        // C s_96_16: const #103424u : u32
        let s_96_16: u32 = 103424;
        // D s_96_17: read-reg s_96_16:[struct; 64]
        let s_96_17: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_96_16 as isize);
            tracer.read_register(s_96_16 as isize, value);
            value
        };
        // D s_96_18: read-var nshadow#270:i
        let s_96_18: i128 = fn_state.nshadow_270;
        // D s_96_19: read-element s_96_17[s_96_18]
        let s_96_19: ProductType5c790c8ef59cc8b2 = s_96_17[(s_96_18) as usize];
        // D s_96_20: write-var ga#12119 <= s_96_19
        fn_state.ga_12119 = s_96_19;
        // D s_96_21: read-var ga#12119.0:struct
        let s_96_21: u64 = fn_state.ga_12119._0;
        // C s_96_22: const #32s : i
        let s_96_22: i128 = 32;
        // D s_96_23: cast zx s_96_21 -> bv
        let s_96_23: Bits = Bits::new(s_96_21 as u128, 64u16);
        // C s_96_24: const #1s : i64
        let s_96_24: i64 = 1;
        // C s_96_25: cast zx s_96_24 -> i
        let s_96_25: i128 = (i128::try_from(s_96_24).unwrap());
        // C s_96_26: const #7s : i
        let s_96_26: i128 = 7;
        // C s_96_27: add s_96_26 s_96_25
        let s_96_27: i128 = (s_96_26 + s_96_25);
        // D s_96_28: bit-extract s_96_23 s_96_22 s_96_27
        let s_96_28: Bits = (Bits::new(
            ((s_96_23) >> (s_96_22)).value(),
            u16::try_from(s_96_27).unwrap(),
        ));
        // D s_96_29: cast reint s_96_28 -> u8
        let s_96_29: u8 = (s_96_28.value() as u8);
        // C s_96_30: const #16s : i
        let s_96_30: i128 = 16;
        // D s_96_31: cast zx s_96_29 -> bv
        let s_96_31: Bits = Bits::new(s_96_29 as u128, 8u16);
        // D s_96_32: bits-cast zx s_96_31 -> bv length s_96_30
        let s_96_32: Bits = s_96_31.zero_extend(s_96_30);
        // D s_96_33: cast reint s_96_32 -> u16
        let s_96_33: u16 = (s_96_32.value() as u16);
        // D s_96_34: write-var bvr_vmid <= s_96_33
        fn_state.bvr_vmid = s_96_33;
        // N s_96_35: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_97_0: const #1u : u8
        let s_97_0: bool = true;
        // D s_97_1: write-var gs#16230 <= s_97_0
        fn_state.gs_16230 = s_97_0;
        // N s_97_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_98_0: const #16975u : u32
        let s_98_0: u32 = 16975;
        // D s_98_1: read-reg s_98_0:u8
        let s_98_1: u8 = {
            let value = state.read_register::<u8>(s_98_0 as isize);
            tracer.read_register(s_98_0 as isize, value);
            value
        };
        // D s_98_2: cast zx s_98_1 -> bv
        let s_98_2: Bits = Bits::new(s_98_1 as u128, 2u16);
        // C s_98_3: const #448u : u32
        let s_98_3: u32 = 448;
        // D s_98_4: read-reg s_98_3:u8
        let s_98_4: u8 = {
            let value = state.read_register::<u8>(s_98_3 as isize);
            tracer.read_register(s_98_3 as isize, value);
            value
        };
        // D s_98_5: cast zx s_98_4 -> bv
        let s_98_5: Bits = Bits::new(s_98_4 as u128, 2u16);
        // D s_98_6: cmp-eq s_98_2 s_98_5
        let s_98_6: bool = ((s_98_2) == (s_98_5));
        // N s_98_7: branch s_98_6 b107 b99
        if s_98_6 {
            return block_107(state, tracer, fn_state);
        } else {
            return block_99(state, tracer, fn_state);
        };
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_99_0: const #16975u : u32
        let s_99_0: u32 = 16975;
        // D s_99_1: read-reg s_99_0:u8
        let s_99_1: u8 = {
            let value = state.read_register::<u8>(s_99_0 as isize);
            tracer.read_register(s_99_0 as isize, value);
            value
        };
        // D s_99_2: cast zx s_99_1 -> bv
        let s_99_2: Bits = Bits::new(s_99_1 as u128, 2u16);
        // C s_99_3: const #440u : u32
        let s_99_3: u32 = 440;
        // D s_99_4: read-reg s_99_3:u8
        let s_99_4: u8 = {
            let value = state.read_register::<u8>(s_99_3 as isize);
            tracer.read_register(s_99_3 as isize, value);
            value
        };
        // D s_99_5: cast zx s_99_4 -> bv
        let s_99_5: Bits = Bits::new(s_99_4 as u128, 2u16);
        // D s_99_6: cmp-eq s_99_2 s_99_5
        let s_99_6: bool = ((s_99_2) == (s_99_5));
        // D s_99_7: write-var gs#16247 <= s_99_6
        fn_state.gs_16247 = s_99_6;
        // N s_99_8: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_100_0: read-var gs#16247:u8
        let s_100_0: bool = fn_state.gs_16247;
        // N s_100_1: branch s_100_0 b106 b101
        if s_100_0 {
            return block_106(state, tracer, fn_state);
        } else {
            return block_101(state, tracer, fn_state);
        };
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_101_0: const #0u : u8
        let s_101_0: bool = false;
        // D s_101_1: write-var gs#16248 <= s_101_0
        fn_state.gs_16248 = s_101_0;
        // N s_101_2: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_102_0: read-var gs#16248:u8
        let s_102_0: bool = fn_state.gs_16248;
        // N s_102_1: branch s_102_0 b105 b103
        if s_102_0 {
            return block_105(state, tracer, fn_state);
        } else {
            return block_103(state, tracer, fn_state);
        };
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_103_0: const #0u : u8
        let s_103_0: bool = false;
        // D s_103_1: write-var gs#16253 <= s_103_0
        fn_state.gs_16253 = s_103_0;
        // N s_103_2: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_104_0: read-var gs#16253:u8
        let s_104_0: bool = fn_state.gs_16253;
        // D s_104_1: write-var bvr_match <= s_104_0
        fn_state.bvr_match = s_104_0;
        // N s_104_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_105_0: const #10048u : u32
        let s_105_0: u32 = 10048;
        // D s_105_1: read-reg s_105_0:u64
        let s_105_1: u64 = {
            let value = state.read_register::<u64>(s_105_0 as isize);
            tracer.read_register(s_105_0 as isize, value);
            value
        };
        // C s_105_2: const #0s : i
        let s_105_2: i128 = 0;
        // D s_105_3: cast zx s_105_1 -> bv
        let s_105_3: Bits = Bits::new(s_105_1 as u128, 64u16);
        // C s_105_4: const #1s : i64
        let s_105_4: i64 = 1;
        // C s_105_5: cast zx s_105_4 -> i
        let s_105_5: i128 = (i128::try_from(s_105_4).unwrap());
        // C s_105_6: const #31s : i
        let s_105_6: i128 = 31;
        // C s_105_7: add s_105_6 s_105_5
        let s_105_7: i128 = (s_105_6 + s_105_5);
        // D s_105_8: bit-extract s_105_3 s_105_2 s_105_7
        let s_105_8: Bits = (Bits::new(
            ((s_105_3) >> (s_105_2)).value(),
            u16::try_from(s_105_7).unwrap(),
        ));
        // D s_105_9: cast reint s_105_8 -> u32
        let s_105_9: u32 = (s_105_8.value() as u32);
        // C s_105_10: const #103424u : u32
        let s_105_10: u32 = 103424;
        // D s_105_11: read-reg s_105_10:[struct; 64]
        let s_105_11: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_105_10 as isize);
            tracer.read_register(s_105_10 as isize, value);
            value
        };
        // D s_105_12: read-var nshadow#270:i
        let s_105_12: i128 = fn_state.nshadow_270;
        // D s_105_13: read-element s_105_11[s_105_12]
        let s_105_13: ProductType5c790c8ef59cc8b2 = s_105_11[(s_105_12) as usize];
        // D s_105_14: write-var ga#12107 <= s_105_13
        fn_state.ga_12107 = s_105_13;
        // D s_105_15: read-var ga#12107.0:struct
        let s_105_15: u64 = fn_state.ga_12107._0;
        // C s_105_16: const #0s : i
        let s_105_16: i128 = 0;
        // D s_105_17: cast zx s_105_15 -> bv
        let s_105_17: Bits = Bits::new(s_105_15 as u128, 64u16);
        // C s_105_18: const #1s : i64
        let s_105_18: i64 = 1;
        // C s_105_19: cast zx s_105_18 -> i
        let s_105_19: i128 = (i128::try_from(s_105_18).unwrap());
        // C s_105_20: const #31s : i
        let s_105_20: i128 = 31;
        // C s_105_21: add s_105_20 s_105_19
        let s_105_21: i128 = (s_105_20 + s_105_19);
        // D s_105_22: bit-extract s_105_17 s_105_16 s_105_21
        let s_105_22: Bits = (Bits::new(
            ((s_105_17) >> (s_105_16)).value(),
            u16::try_from(s_105_21).unwrap(),
        ));
        // D s_105_23: cast reint s_105_22 -> u32
        let s_105_23: u32 = (s_105_22.value() as u32);
        // D s_105_24: cast zx s_105_9 -> bv
        let s_105_24: Bits = Bits::new(s_105_9 as u128, 32u16);
        // D s_105_25: cast zx s_105_23 -> bv
        let s_105_25: Bits = Bits::new(s_105_23 as u128, 32u16);
        // D s_105_26: cmp-eq s_105_24 s_105_25
        let s_105_26: bool = ((s_105_24) == (s_105_25));
        // D s_105_27: write-var gs#16253 <= s_105_26
        fn_state.gs_16253 = s_105_26;
        // N s_105_28: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_106_0: const #() : ()
        let s_106_0: () = ();
        // S s_106_1: call IsInHost(s_106_0)
        let s_106_1: bool = IsInHost(state, tracer, s_106_0);
        // S s_106_2: not s_106_1
        let s_106_2: bool = !s_106_1;
        // D s_106_3: write-var gs#16248 <= s_106_2
        fn_state.gs_16248 = s_106_2;
        // N s_106_4: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_107_0: const #1u : u8
        let s_107_0: bool = true;
        // D s_107_1: write-var gs#16247 <= s_107_0
        fn_state.gs_16247 = s_107_0;
        // N s_107_2: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_108_0: const #() : ()
        let s_108_0: () = ();
        // S s_108_1: call IsInHost(s_108_0)
        let s_108_1: bool = IsInHost(state, tracer, s_108_0);
        // N s_108_2: branch s_108_1 b116 b109
        if s_108_1 {
            return block_116(state, tracer, fn_state);
        } else {
            return block_109(state, tracer, fn_state);
        };
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_109_0: const #16975u : u32
        let s_109_0: u32 = 16975;
        // D s_109_1: read-reg s_109_0:u8
        let s_109_1: u8 = {
            let value = state.read_register::<u8>(s_109_0 as isize);
            tracer.read_register(s_109_0 as isize, value);
            value
        };
        // D s_109_2: cast zx s_109_1 -> bv
        let s_109_2: Bits = Bits::new(s_109_1 as u128, 2u16);
        // C s_109_3: const #448u : u32
        let s_109_3: u32 = 448;
        // D s_109_4: read-reg s_109_3:u8
        let s_109_4: u8 = {
            let value = state.read_register::<u8>(s_109_3 as isize);
            tracer.read_register(s_109_3 as isize, value);
            value
        };
        // D s_109_5: cast zx s_109_4 -> bv
        let s_109_5: Bits = Bits::new(s_109_4 as u128, 2u16);
        // D s_109_6: cmp-eq s_109_2 s_109_5
        let s_109_6: bool = ((s_109_2) == (s_109_5));
        // N s_109_7: branch s_109_6 b115 b110
        if s_109_6 {
            return block_115(state, tracer, fn_state);
        } else {
            return block_110(state, tracer, fn_state);
        };
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_110_0: const #16975u : u32
        let s_110_0: u32 = 16975;
        // D s_110_1: read-reg s_110_0:u8
        let s_110_1: u8 = {
            let value = state.read_register::<u8>(s_110_0 as isize);
            tracer.read_register(s_110_0 as isize, value);
            value
        };
        // D s_110_2: cast zx s_110_1 -> bv
        let s_110_2: Bits = Bits::new(s_110_1 as u128, 2u16);
        // C s_110_3: const #440u : u32
        let s_110_3: u32 = 440;
        // D s_110_4: read-reg s_110_3:u8
        let s_110_4: u8 = {
            let value = state.read_register::<u8>(s_110_3 as isize);
            tracer.read_register(s_110_3 as isize, value);
            value
        };
        // D s_110_5: cast zx s_110_4 -> bv
        let s_110_5: Bits = Bits::new(s_110_4 as u128, 2u16);
        // D s_110_6: cmp-eq s_110_2 s_110_5
        let s_110_6: bool = ((s_110_2) == (s_110_5));
        // D s_110_7: write-var gs#16254 <= s_110_6
        fn_state.gs_16254 = s_110_6;
        // N s_110_8: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_111_0: read-var gs#16254:u8
        let s_111_0: bool = fn_state.gs_16254;
        // N s_111_1: branch s_111_0 b114 b112
        if s_111_0 {
            return block_114(state, tracer, fn_state);
        } else {
            return block_112(state, tracer, fn_state);
        };
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_112_0: const #0u : u8
        let s_112_0: bool = false;
        // D s_112_1: write-var gs#16259 <= s_112_0
        fn_state.gs_16259 = s_112_0;
        // N s_112_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_113_0: read-var gs#16259:u8
        let s_113_0: bool = fn_state.gs_16259;
        // D s_113_1: write-var bvr_match <= s_113_0
        fn_state.bvr_match = s_113_0;
        // N s_113_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_114_0: const #10048u : u32
        let s_114_0: u32 = 10048;
        // D s_114_1: read-reg s_114_0:u64
        let s_114_1: u64 = {
            let value = state.read_register::<u64>(s_114_0 as isize);
            tracer.read_register(s_114_0 as isize, value);
            value
        };
        // C s_114_2: const #0s : i
        let s_114_2: i128 = 0;
        // D s_114_3: cast zx s_114_1 -> bv
        let s_114_3: Bits = Bits::new(s_114_1 as u128, 64u16);
        // C s_114_4: const #1s : i64
        let s_114_4: i64 = 1;
        // C s_114_5: cast zx s_114_4 -> i
        let s_114_5: i128 = (i128::try_from(s_114_4).unwrap());
        // C s_114_6: const #31s : i
        let s_114_6: i128 = 31;
        // C s_114_7: add s_114_6 s_114_5
        let s_114_7: i128 = (s_114_6 + s_114_5);
        // D s_114_8: bit-extract s_114_3 s_114_2 s_114_7
        let s_114_8: Bits = (Bits::new(
            ((s_114_3) >> (s_114_2)).value(),
            u16::try_from(s_114_7).unwrap(),
        ));
        // D s_114_9: cast reint s_114_8 -> u32
        let s_114_9: u32 = (s_114_8.value() as u32);
        // C s_114_10: const #103424u : u32
        let s_114_10: u32 = 103424;
        // D s_114_11: read-reg s_114_10:[struct; 64]
        let s_114_11: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_114_10 as isize);
            tracer.read_register(s_114_10 as isize, value);
            value
        };
        // D s_114_12: read-var nshadow#270:i
        let s_114_12: i128 = fn_state.nshadow_270;
        // D s_114_13: read-element s_114_11[s_114_12]
        let s_114_13: ProductType5c790c8ef59cc8b2 = s_114_11[(s_114_12) as usize];
        // D s_114_14: write-var ga#12096 <= s_114_13
        fn_state.ga_12096 = s_114_13;
        // D s_114_15: read-var ga#12096.0:struct
        let s_114_15: u64 = fn_state.ga_12096._0;
        // C s_114_16: const #0s : i
        let s_114_16: i128 = 0;
        // D s_114_17: cast zx s_114_15 -> bv
        let s_114_17: Bits = Bits::new(s_114_15 as u128, 64u16);
        // C s_114_18: const #1s : i64
        let s_114_18: i64 = 1;
        // C s_114_19: cast zx s_114_18 -> i
        let s_114_19: i128 = (i128::try_from(s_114_18).unwrap());
        // C s_114_20: const #31s : i
        let s_114_20: i128 = 31;
        // C s_114_21: add s_114_20 s_114_19
        let s_114_21: i128 = (s_114_20 + s_114_19);
        // D s_114_22: bit-extract s_114_17 s_114_16 s_114_21
        let s_114_22: Bits = (Bits::new(
            ((s_114_17) >> (s_114_16)).value(),
            u16::try_from(s_114_21).unwrap(),
        ));
        // D s_114_23: cast reint s_114_22 -> u32
        let s_114_23: u32 = (s_114_22.value() as u32);
        // D s_114_24: cast zx s_114_9 -> bv
        let s_114_24: Bits = Bits::new(s_114_9 as u128, 32u16);
        // D s_114_25: cast zx s_114_23 -> bv
        let s_114_25: Bits = Bits::new(s_114_23 as u128, 32u16);
        // D s_114_26: cmp-eq s_114_24 s_114_25
        let s_114_26: bool = ((s_114_24) == (s_114_25));
        // D s_114_27: write-var gs#16259 <= s_114_26
        fn_state.gs_16259 = s_114_26;
        // N s_114_28: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_115_0: const #1u : u8
        let s_115_0: bool = true;
        // D s_115_1: write-var gs#16254 <= s_115_0
        fn_state.gs_16254 = s_115_0;
        // N s_115_2: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_116_0: const #91008u : u32
        let s_116_0: u32 = 91008;
        // D s_116_1: read-reg s_116_0:u64
        let s_116_1: u64 = {
            let value = state.read_register::<u64>(s_116_0 as isize);
            tracer.read_register(s_116_0 as isize, value);
            value
        };
        // C s_116_2: const #0s : i
        let s_116_2: i128 = 0;
        // D s_116_3: cast zx s_116_1 -> bv
        let s_116_3: Bits = Bits::new(s_116_1 as u128, 64u16);
        // C s_116_4: const #1s : i64
        let s_116_4: i64 = 1;
        // C s_116_5: cast zx s_116_4 -> i
        let s_116_5: i128 = (i128::try_from(s_116_4).unwrap());
        // C s_116_6: const #31s : i
        let s_116_6: i128 = 31;
        // C s_116_7: add s_116_6 s_116_5
        let s_116_7: i128 = (s_116_6 + s_116_5);
        // D s_116_8: bit-extract s_116_3 s_116_2 s_116_7
        let s_116_8: Bits = (Bits::new(
            ((s_116_3) >> (s_116_2)).value(),
            u16::try_from(s_116_7).unwrap(),
        ));
        // D s_116_9: cast reint s_116_8 -> u32
        let s_116_9: u32 = (s_116_8.value() as u32);
        // C s_116_10: const #103424u : u32
        let s_116_10: u32 = 103424;
        // D s_116_11: read-reg s_116_10:[struct; 64]
        let s_116_11: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_116_10 as isize);
            tracer.read_register(s_116_10 as isize, value);
            value
        };
        // D s_116_12: read-var nshadow#270:i
        let s_116_12: i128 = fn_state.nshadow_270;
        // D s_116_13: read-element s_116_11[s_116_12]
        let s_116_13: ProductType5c790c8ef59cc8b2 = s_116_11[(s_116_12) as usize];
        // D s_116_14: write-var ga#12088 <= s_116_13
        fn_state.ga_12088 = s_116_13;
        // D s_116_15: read-var ga#12088.0:struct
        let s_116_15: u64 = fn_state.ga_12088._0;
        // C s_116_16: const #0s : i
        let s_116_16: i128 = 0;
        // D s_116_17: cast zx s_116_15 -> bv
        let s_116_17: Bits = Bits::new(s_116_15 as u128, 64u16);
        // C s_116_18: const #1s : i64
        let s_116_18: i64 = 1;
        // C s_116_19: cast zx s_116_18 -> i
        let s_116_19: i128 = (i128::try_from(s_116_18).unwrap());
        // C s_116_20: const #31s : i
        let s_116_20: i128 = 31;
        // C s_116_21: add s_116_20 s_116_19
        let s_116_21: i128 = (s_116_20 + s_116_19);
        // D s_116_22: bit-extract s_116_17 s_116_16 s_116_21
        let s_116_22: Bits = (Bits::new(
            ((s_116_17) >> (s_116_16)).value(),
            u16::try_from(s_116_21).unwrap(),
        ));
        // D s_116_23: cast reint s_116_22 -> u32
        let s_116_23: u32 = (s_116_22.value() as u32);
        // D s_116_24: cast zx s_116_9 -> bv
        let s_116_24: Bits = Bits::new(s_116_9 as u128, 32u16);
        // D s_116_25: cast zx s_116_23 -> bv
        let s_116_25: Bits = Bits::new(s_116_23 as u128, 32u16);
        // D s_116_26: cmp-eq s_116_24 s_116_25
        let s_116_26: bool = ((s_116_24) == (s_116_25));
        // D s_116_27: write-var bvr_match <= s_116_26
        fn_state.bvr_match = s_116_26;
        // N s_116_28: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_117_0: const #0s : i
        let s_117_0: i128 = 0;
        // D s_117_1: read-var vaddress:u64
        let s_117_1: u64 = fn_state.vaddress;
        // D s_117_2: cast zx s_117_1 -> bv
        let s_117_2: Bits = Bits::new(s_117_1 as u128, 64u16);
        // C s_117_3: const #1s : i64
        let s_117_3: i64 = 1;
        // C s_117_4: cast zx s_117_3 -> i
        let s_117_4: i128 = (i128::try_from(s_117_3).unwrap());
        // C s_117_5: const #1s : i
        let s_117_5: i128 = 1;
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
        let s_117_9: Bits = Bits::new(s_117_8 as u128, 2u16);
        // D s_117_10: cast zx s_117_9 -> i
        let s_117_10: i128 = (s_117_9.value() as i128);
        // D s_117_11: cast reint s_117_10 -> i64
        let s_117_11: i64 = (s_117_10 as i64);
        // D s_117_12: write-var byte <= s_117_11
        fn_state.byte = s_117_11;
        // C s_117_13: const #() : ()
        let s_117_13: () = ();
        // S s_117_14: call HaveAArch32(s_117_13)
        let s_117_14: bool = HaveAArch32(state, tracer, s_117_13);
        // N s_117_15: branch s_117_14 b159 b118
        if s_117_14 {
            return block_159(state, tracer, fn_state);
        } else {
            return block_118(state, tracer, fn_state);
        };
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_118_0: const #0s : i
        let s_118_0: i128 = 0;
        // D s_118_1: read-var byte:i64
        let s_118_1: i64 = fn_state.byte;
        // D s_118_2: cast zx s_118_1 -> i
        let s_118_2: i128 = (i128::try_from(s_118_1).unwrap());
        // D s_118_3: cmp-eq s_118_2 s_118_0
        let s_118_3: bool = ((s_118_2) == (s_118_0));
        // N s_118_4: assert s_118_3
        let s_118_4: () = assert!(s_118_3);
        // C s_118_5: const #1u : u8
        let s_118_5: bool = true;
        // D s_118_6: write-var byte_select_match <= s_118_5
        fn_state.byte_select_match = s_118_5;
        // N s_118_7: jump b119
        return block_119(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_119_0: const #() : ()
        let s_119_0: () = ();
        // S s_119_1: call DebugAddrTop(s_119_0)
        let s_119_1: i128 = DebugAddrTop(state, tracer, s_119_0);
        // D s_119_2: write-var top <= s_119_1
        fn_state.top = s_119_1;
        // C s_119_3: const #55s : i
        let s_119_3: i128 = 55;
        // D s_119_4: read-var top:i
        let s_119_4: i128 = fn_state.top;
        // D s_119_5: cmp-lt s_119_4 s_119_3
        let s_119_5: bool = ((s_119_4) < (s_119_3));
        // N s_119_6: branch s_119_5 b158 b120
        if s_119_5 {
            return block_158(state, tracer, fn_state);
        } else {
            return block_120(state, tracer, fn_state);
        };
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_120_0: const #0u : u8
        let s_120_0: bool = false;
        // D s_120_1: write-var gs#16279 <= s_120_0
        fn_state.gs_16279 = s_120_0;
        // N s_120_2: jump b121
        return block_121(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_121_0: read-var gs#16279:u8
        let s_121_0: bool = fn_state.gs_16279;
        // N s_121_1: branch s_121_0 b157 b122
        if s_121_0 {
            return block_157(state, tracer, fn_state);
        } else {
            return block_122(state, tracer, fn_state);
        };
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_122_0: const #0u : u8
        let s_122_0: bool = false;
        // D s_122_1: write-var gs#16281 <= s_122_0
        fn_state.gs_16281 = s_122_0;
        // N s_122_2: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_123_0: read-var gs#16281:u8
        let s_123_0: bool = fn_state.gs_16281;
        // N s_123_1: branch s_123_0 b156 b124
        if s_123_0 {
            return block_156(state, tracer, fn_state);
        } else {
            return block_124(state, tracer, fn_state);
        };
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_124_0: const #0u : u8
        let s_124_0: bool = false;
        // D s_124_1: write-var gs#16282 <= s_124_0
        fn_state.gs_16282 = s_124_0;
        // N s_124_2: jump b125
        return block_125(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_125_0: read-var gs#16282:u8
        let s_125_0: bool = fn_state.gs_16282;
        // N s_125_1: branch s_125_0 b155 b126
        if s_125_0 {
            return block_155(state, tracer, fn_state);
        } else {
            return block_126(state, tracer, fn_state);
        };
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // N s_126_0: jump b127
        return block_127(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_127_0: read-var top:i
        let s_127_0: i128 = fn_state.top;
        // D s_127_1: write-var topshadow#272 <= s_127_0
        fn_state.topshadow_272 = s_127_0;
        // C s_127_2: const #2s : i64
        let s_127_2: i64 = 2;
        // C s_127_3: cast zx s_127_2 -> i
        let s_127_3: i128 = (i128::try_from(s_127_2).unwrap());
        // D s_127_4: read-var maskshadow#271:i
        let s_127_4: i128 = fn_state.maskshadow_271;
        // D s_127_5: cmp-gt s_127_4 s_127_3
        let s_127_5: bool = ((s_127_4) > (s_127_3));
        // N s_127_6: branch s_127_5 b139 b128
        if s_127_5 {
            return block_139(state, tracer, fn_state);
        } else {
            return block_128(state, tracer, fn_state);
        };
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_128_0: const #2s : i64
        let s_128_0: i64 = 2;
        // C s_128_1: cast zx s_128_0 -> i
        let s_128_1: i128 = (i128::try_from(s_128_0).unwrap());
        // S s_128_2: call __id(s_128_1)
        let s_128_2: i128 = u__id(state, tracer, s_128_1);
        // S s_128_3: cast reint s_128_2 -> i64
        let s_128_3: i64 = (s_128_2 as i64);
        // C s_128_4: const #0s : i
        let s_128_4: i128 = 0;
        // S s_128_5: cast zx s_128_3 -> i
        let s_128_5: i128 = (i128::try_from(s_128_3).unwrap());
        // S s_128_6: cmp-le s_128_4 s_128_5
        let s_128_6: bool = ((s_128_4) <= (s_128_5));
        // N s_128_7: branch s_128_6 b135 b129
        if s_128_6 {
            return block_135(state, tracer, fn_state);
        } else {
            return block_129(state, tracer, fn_state);
        };
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_129_0: const #0u : u8
        let s_129_0: bool = false;
        // D s_129_1: write-var gs#16290 <= s_129_0
        fn_state.gs_16290 = s_129_0;
        // N s_129_2: jump b130
        return block_130(state, tracer, fn_state);
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_130_0: read-var gs#16290:u8
        let s_130_0: bool = fn_state.gs_16290;
        // N s_130_1: assert s_130_0
        let s_130_1: () = assert!(s_130_0);
        // C s_130_2: const #103424u : u32
        let s_130_2: u32 = 103424;
        // D s_130_3: read-reg s_130_2:[struct; 64]
        let s_130_3: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_130_2 as isize);
            tracer.read_register(s_130_2 as isize, value);
            value
        };
        // D s_130_4: read-var nshadow#270:i
        let s_130_4: i128 = fn_state.nshadow_270;
        // D s_130_5: read-element s_130_3[s_130_4]
        let s_130_5: ProductType5c790c8ef59cc8b2 = s_130_3[(s_130_4) as usize];
        // D s_130_6: write-var ga#12083 <= s_130_5
        fn_state.ga_12083 = s_130_5;
        // D s_130_7: read-var ga#12083.0:struct
        let s_130_7: u64 = fn_state.ga_12083._0;
        // D s_130_8: read-var vaddress:u64
        let s_130_8: u64 = fn_state.vaddress;
        // D s_130_9: cast zx s_130_8 -> bv
        let s_130_9: Bits = Bits::new(s_130_8 as u128, 64u16);
        // C s_130_10: const #2s : i64
        let s_130_10: i64 = 2;
        // C s_130_11: cast zx s_130_10 -> i
        let s_130_11: i128 = (i128::try_from(s_130_10).unwrap());
        // D s_130_12: cast zx s_130_7 -> bv
        let s_130_12: Bits = Bits::new(s_130_7 as u128, 64u16);
        // C s_130_13: const #2s : i64
        let s_130_13: i64 = 2;
        // C s_130_14: cast zx s_130_13 -> i
        let s_130_14: i128 = (i128::try_from(s_130_13).unwrap());
        // D s_130_15: read-var topshadow#272:i
        let s_130_15: i128 = fn_state.topshadow_272;
        // D s_130_16: read-var topshadow#272:i
        let s_130_16: i128 = fn_state.topshadow_272;
        // D s_130_17: call subrange_subrange_eq(s_130_9, s_130_15, s_130_11, s_130_12, s_130_16, s_130_14)
        let s_130_17: bool = subrange_subrange_eq(
            state,
            tracer,
            s_130_9,
            s_130_15,
            s_130_11,
            s_130_12,
            s_130_16,
            s_130_14,
        );
        // N s_130_18: branch s_130_17 b134 b131
        if s_130_17 {
            return block_134(state, tracer, fn_state);
        } else {
            return block_131(state, tracer, fn_state);
        };
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_131_0: const #0u : u8
        let s_131_0: bool = false;
        // D s_131_1: write-var gs#16292 <= s_131_0
        fn_state.gs_16292 = s_131_0;
        // N s_131_2: jump b132
        return block_132(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_132_0: read-var gs#16292:u8
        let s_132_0: bool = fn_state.gs_16292;
        // D s_132_1: write-var bvr_match <= s_132_0
        fn_state.bvr_match = s_132_0;
        // N s_132_2: jump b133
        return block_133(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // N s_133_0: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_134_0: read-var byte_select_match:u8
        let s_134_0: bool = fn_state.byte_select_match;
        // D s_134_1: write-var gs#16292 <= s_134_0
        fn_state.gs_16292 = s_134_0;
        // N s_134_2: jump b132
        return block_132(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_135_0: const #2s : i64
        let s_135_0: i64 = 2;
        // C s_135_1: cast zx s_135_0 -> i
        let s_135_1: i128 = (i128::try_from(s_135_0).unwrap());
        // S s_135_2: call __id(s_135_1)
        let s_135_2: i128 = u__id(state, tracer, s_135_1);
        // S s_135_3: cast reint s_135_2 -> i64
        let s_135_3: i64 = (s_135_2 as i64);
        // D s_135_4: read-var topshadow#272:i
        let s_135_4: i128 = fn_state.topshadow_272;
        // D s_135_5: call __id(s_135_4)
        let s_135_5: i128 = u__id(state, tracer, s_135_4);
        // S s_135_6: cast zx s_135_3 -> i
        let s_135_6: i128 = (i128::try_from(s_135_3).unwrap());
        // D s_135_7: cmp-le s_135_6 s_135_5
        let s_135_7: bool = ((s_135_6) <= (s_135_5));
        // N s_135_8: branch s_135_7 b138 b136
        if s_135_7 {
            return block_138(state, tracer, fn_state);
        } else {
            return block_136(state, tracer, fn_state);
        };
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_136_0: const #0u : u8
        let s_136_0: bool = false;
        // D s_136_1: write-var gs#16289 <= s_136_0
        fn_state.gs_16289 = s_136_0;
        // N s_136_2: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_137_0: read-var gs#16289:u8
        let s_137_0: bool = fn_state.gs_16289;
        // D s_137_1: write-var gs#16290 <= s_137_0
        fn_state.gs_16290 = s_137_0;
        // N s_137_2: jump b130
        return block_130(state, tracer, fn_state);
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_138_0: read-var topshadow#272:i
        let s_138_0: i128 = fn_state.topshadow_272;
        // D s_138_1: call __id(s_138_0)
        let s_138_1: i128 = u__id(state, tracer, s_138_0);
        // C s_138_2: const #64s : i
        let s_138_2: i128 = 64;
        // D s_138_3: cmp-lt s_138_1 s_138_2
        let s_138_3: bool = ((s_138_1) < (s_138_2));
        // D s_138_4: write-var gs#16289 <= s_138_3
        fn_state.gs_16289 = s_138_3;
        // N s_138_5: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_139_0: read-var maskshadow#271:i
        let s_139_0: i128 = fn_state.maskshadow_271;
        // D s_139_1: call __id(s_139_0)
        let s_139_1: i128 = u__id(state, tracer, s_139_0);
        // C s_139_2: const #0s : i
        let s_139_2: i128 = 0;
        // D s_139_3: cmp-le s_139_2 s_139_1
        let s_139_3: bool = ((s_139_2) <= (s_139_1));
        // N s_139_4: branch s_139_3 b151 b140
        if s_139_3 {
            return block_151(state, tracer, fn_state);
        } else {
            return block_140(state, tracer, fn_state);
        };
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_140_0: const #0u : u8
        let s_140_0: bool = false;
        // D s_140_1: write-var gs#16296 <= s_140_0
        fn_state.gs_16296 = s_140_0;
        // N s_140_2: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_141_0: read-var gs#16296:u8
        let s_141_0: bool = fn_state.gs_16296;
        // N s_141_1: assert s_141_0
        let s_141_1: () = assert!(s_141_0);
        // C s_141_2: const #103424u : u32
        let s_141_2: u32 = 103424;
        // D s_141_3: read-reg s_141_2:[struct; 64]
        let s_141_3: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_141_2 as isize);
            tracer.read_register(s_141_2 as isize, value);
            value
        };
        // D s_141_4: read-var nshadow#270:i
        let s_141_4: i128 = fn_state.nshadow_270;
        // D s_141_5: read-element s_141_3[s_141_4]
        let s_141_5: ProductType5c790c8ef59cc8b2 = s_141_3[(s_141_4) as usize];
        // D s_141_6: write-var ga#12068 <= s_141_5
        fn_state.ga_12068 = s_141_5;
        // D s_141_7: read-var ga#12068.0:struct
        let s_141_7: u64 = fn_state.ga_12068._0;
        // D s_141_8: read-var vaddress:u64
        let s_141_8: u64 = fn_state.vaddress;
        // D s_141_9: cast zx s_141_8 -> bv
        let s_141_9: Bits = Bits::new(s_141_8 as u128, 64u16);
        // D s_141_10: cast zx s_141_7 -> bv
        let s_141_10: Bits = Bits::new(s_141_7 as u128, 64u16);
        // D s_141_11: read-var topshadow#272:i
        let s_141_11: i128 = fn_state.topshadow_272;
        // D s_141_12: read-var maskshadow#271:i
        let s_141_12: i128 = fn_state.maskshadow_271;
        // D s_141_13: read-var topshadow#272:i
        let s_141_13: i128 = fn_state.topshadow_272;
        // D s_141_14: read-var maskshadow#271:i
        let s_141_14: i128 = fn_state.maskshadow_271;
        // D s_141_15: call subrange_subrange_eq(s_141_9, s_141_11, s_141_12, s_141_10, s_141_13, s_141_14)
        let s_141_15: bool = subrange_subrange_eq(
            state,
            tracer,
            s_141_9,
            s_141_11,
            s_141_12,
            s_141_10,
            s_141_13,
            s_141_14,
        );
        // N s_141_16: branch s_141_15 b150 b142
        if s_141_15 {
            return block_150(state, tracer, fn_state);
        } else {
            return block_142(state, tracer, fn_state);
        };
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_142_0: const #0u : u8
        let s_142_0: bool = false;
        // D s_142_1: write-var gs#16297 <= s_142_0
        fn_state.gs_16297 = s_142_0;
        // N s_142_2: jump b143
        return block_143(state, tracer, fn_state);
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_143_0: read-var gs#16297:u8
        let s_143_0: bool = fn_state.gs_16297;
        // D s_143_1: write-var bvr_match <= s_143_0
        fn_state.bvr_match = s_143_0;
        // D s_143_2: read-var bvr_match:u8
        let s_143_2: bool = fn_state.bvr_match;
        // N s_143_3: branch s_143_2 b149 b144
        if s_143_2 {
            return block_149(state, tracer, fn_state);
        } else {
            return block_144(state, tracer, fn_state);
        };
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_144_0: const #0u : u8
        let s_144_0: bool = false;
        // D s_144_1: write-var gs#16299 <= s_144_0
        fn_state.gs_16299 = s_144_0;
        // N s_144_2: jump b145
        return block_145(state, tracer, fn_state);
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_145_0: read-var gs#16299:u8
        let s_145_0: bool = fn_state.gs_16299;
        // N s_145_1: branch s_145_0 b148 b146
        if s_145_0 {
            return block_148(state, tracer, fn_state);
        } else {
            return block_146(state, tracer, fn_state);
        };
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // N s_146_0: jump b147
        return block_147(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // N s_147_0: jump b133
        return block_133(state, tracer, fn_state);
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_148_0: const #41u : u32
        let s_148_0: u32 = 41;
        // S s_148_1: call ConstrainUnpredictableBool(s_148_0)
        let s_148_1: bool = ConstrainUnpredictableBool(state, tracer, s_148_0);
        // D s_148_2: write-var bvr_match <= s_148_1
        fn_state.bvr_match = s_148_1;
        // N s_148_3: jump b147
        return block_147(state, tracer, fn_state);
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_149_0: const #103424u : u32
        let s_149_0: u32 = 103424;
        // D s_149_1: read-reg s_149_0:[struct; 64]
        let s_149_1: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_149_0 as isize);
            tracer.read_register(s_149_0 as isize, value);
            value
        };
        // D s_149_2: read-var nshadow#270:i
        let s_149_2: i128 = fn_state.nshadow_270;
        // D s_149_3: read-element s_149_1[s_149_2]
        let s_149_3: ProductType5c790c8ef59cc8b2 = s_149_1[(s_149_2) as usize];
        // D s_149_4: write-var ga#12071 <= s_149_3
        fn_state.ga_12071 = s_149_3;
        // D s_149_5: read-var ga#12071.0:struct
        let s_149_5: u64 = fn_state.ga_12071._0;
        // C s_149_6: const #1s : i
        let s_149_6: i128 = 1;
        // D s_149_7: read-var maskshadow#271:i
        let s_149_7: i128 = fn_state.maskshadow_271;
        // D s_149_8: sub s_149_7 s_149_6
        let s_149_8: i128 = ((s_149_7) - (s_149_6));
        // D s_149_9: cast reint s_149_8 -> i64
        let s_149_9: i64 = (s_149_8 as i64);
        // D s_149_10: cast zx s_149_5 -> bv
        let s_149_10: Bits = Bits::new(s_149_5 as u128, 64u16);
        // D s_149_11: cast zx s_149_9 -> i
        let s_149_11: i128 = (i128::try_from(s_149_9).unwrap());
        // C s_149_12: const #2s : i64
        let s_149_12: i64 = 2;
        // C s_149_13: cast zx s_149_12 -> i
        let s_149_13: i128 = (i128::try_from(s_149_12).unwrap());
        // D s_149_14: call is_zero_subrange(s_149_10, s_149_11, s_149_13)
        let s_149_14: bool = is_zero_subrange(
            state,
            tracer,
            s_149_10,
            s_149_11,
            s_149_13,
        );
        // D s_149_15: not s_149_14
        let s_149_15: bool = !s_149_14;
        // D s_149_16: write-var gs#16299 <= s_149_15
        fn_state.gs_16299 = s_149_15;
        // N s_149_17: jump b145
        return block_145(state, tracer, fn_state);
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_150_0: read-var byte_select_match:u8
        let s_150_0: bool = fn_state.byte_select_match;
        // D s_150_1: write-var gs#16297 <= s_150_0
        fn_state.gs_16297 = s_150_0;
        // N s_150_2: jump b143
        return block_143(state, tracer, fn_state);
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_151_0: read-var maskshadow#271:i
        let s_151_0: i128 = fn_state.maskshadow_271;
        // D s_151_1: call __id(s_151_0)
        let s_151_1: i128 = u__id(state, tracer, s_151_0);
        // D s_151_2: read-var topshadow#272:i
        let s_151_2: i128 = fn_state.topshadow_272;
        // D s_151_3: call __id(s_151_2)
        let s_151_3: i128 = u__id(state, tracer, s_151_2);
        // D s_151_4: cmp-le s_151_1 s_151_3
        let s_151_4: bool = ((s_151_1) <= (s_151_3));
        // N s_151_5: branch s_151_4 b154 b152
        if s_151_4 {
            return block_154(state, tracer, fn_state);
        } else {
            return block_152(state, tracer, fn_state);
        };
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_152_0: const #0u : u8
        let s_152_0: bool = false;
        // D s_152_1: write-var gs#16295 <= s_152_0
        fn_state.gs_16295 = s_152_0;
        // N s_152_2: jump b153
        return block_153(state, tracer, fn_state);
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_153_0: read-var gs#16295:u8
        let s_153_0: bool = fn_state.gs_16295;
        // D s_153_1: write-var gs#16296 <= s_153_0
        fn_state.gs_16296 = s_153_0;
        // N s_153_2: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_154_0: read-var topshadow#272:i
        let s_154_0: i128 = fn_state.topshadow_272;
        // D s_154_1: call __id(s_154_0)
        let s_154_1: i128 = u__id(state, tracer, s_154_0);
        // C s_154_2: const #64s : i
        let s_154_2: i128 = 64;
        // D s_154_3: cmp-lt s_154_1 s_154_2
        let s_154_3: bool = ((s_154_1) < (s_154_2));
        // D s_154_4: write-var gs#16295 <= s_154_3
        fn_state.gs_16295 = s_154_3;
        // N s_154_5: jump b153
        return block_153(state, tracer, fn_state);
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_155_0: const #63s : i
        let s_155_0: i128 = 63;
        // D s_155_1: write-var top <= s_155_0
        fn_state.top = s_155_0;
        // N s_155_2: jump b127
        return block_127(state, tracer, fn_state);
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_156_0: const #71u : u32
        let s_156_0: u32 = 71;
        // S s_156_1: call ConstrainUnpredictableBool(s_156_0)
        let s_156_1: bool = ConstrainUnpredictableBool(state, tracer, s_156_0);
        // D s_156_2: write-var gs#16282 <= s_156_1
        fn_state.gs_16282 = s_156_1;
        // N s_156_3: jump b125
        return block_125(state, tracer, fn_state);
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_157_0: const #103424u : u32
        let s_157_0: u32 = 103424;
        // D s_157_1: read-reg s_157_0:[struct; 64]
        let s_157_1: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_157_0 as isize);
            tracer.read_register(s_157_0 as isize, value);
            value
        };
        // D s_157_2: read-var nshadow#270:i
        let s_157_2: i128 = fn_state.nshadow_270;
        // D s_157_3: read-element s_157_1[s_157_2]
        let s_157_3: ProductType5c790c8ef59cc8b2 = s_157_1[(s_157_2) as usize];
        // D s_157_4: write-var ga#12054 <= s_157_3
        fn_state.ga_12054 = s_157_3;
        // D s_157_5: read-var ga#12054.0:struct
        let s_157_5: u64 = fn_state.ga_12054._0;
        // C s_157_6: const #63s : i
        let s_157_6: i128 = 63;
        // D s_157_7: cast zx s_157_5 -> bv
        let s_157_7: Bits = Bits::new(s_157_5 as u128, 64u16);
        // D s_157_8: read-var top:i
        let s_157_8: i128 = fn_state.top;
        // D s_157_9: call is_zero_subrange(s_157_7, s_157_6, s_157_8)
        let s_157_9: bool = is_zero_subrange(state, tracer, s_157_7, s_157_6, s_157_8);
        // D s_157_10: not s_157_9
        let s_157_10: bool = !s_157_9;
        // D s_157_11: write-var gs#16281 <= s_157_10
        fn_state.gs_16281 = s_157_10;
        // N s_157_12: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_158_0: const #103424u : u32
        let s_158_0: u32 = 103424;
        // D s_158_1: read-reg s_158_0:[struct; 64]
        let s_158_1: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_158_0 as isize);
            tracer.read_register(s_158_0 as isize, value);
            value
        };
        // D s_158_2: read-var nshadow#270:i
        let s_158_2: i128 = fn_state.nshadow_270;
        // D s_158_3: read-element s_158_1[s_158_2]
        let s_158_3: ProductType5c790c8ef59cc8b2 = s_158_1[(s_158_2) as usize];
        // D s_158_4: write-var ga#12050 <= s_158_3
        fn_state.ga_12050 = s_158_3;
        // D s_158_5: read-var ga#12050.0:struct
        let s_158_5: u64 = fn_state.ga_12050._0;
        // C s_158_6: const #63s : i
        let s_158_6: i128 = 63;
        // D s_158_7: cast zx s_158_5 -> bv
        let s_158_7: Bits = Bits::new(s_158_5 as u128, 64u16);
        // D s_158_8: read-var top:i
        let s_158_8: i128 = fn_state.top;
        // D s_158_9: call is_ones_subrange(s_158_7, s_158_6, s_158_8)
        let s_158_9: bool = is_ones_subrange(state, tracer, s_158_7, s_158_6, s_158_8);
        // D s_158_10: not s_158_9
        let s_158_10: bool = !s_158_9;
        // D s_158_11: write-var gs#16279 <= s_158_10
        fn_state.gs_16279 = s_158_10;
        // N s_158_12: jump b121
        return block_121(state, tracer, fn_state);
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_159_0: const #0s : i
        let s_159_0: i128 = 0;
        // D s_159_1: read-var byte:i64
        let s_159_1: i64 = fn_state.byte;
        // D s_159_2: cast zx s_159_1 -> i
        let s_159_2: i128 = (i128::try_from(s_159_1).unwrap());
        // D s_159_3: cmp-eq s_159_2 s_159_0
        let s_159_3: bool = ((s_159_2) == (s_159_0));
        // N s_159_4: branch s_159_3 b162 b160
        if s_159_3 {
            return block_162(state, tracer, fn_state);
        } else {
            return block_160(state, tracer, fn_state);
        };
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_160_0: const #2s : i
        let s_160_0: i128 = 2;
        // D s_160_1: read-var byte:i64
        let s_160_1: i64 = fn_state.byte;
        // D s_160_2: cast zx s_160_1 -> i
        let s_160_2: i128 = (i128::try_from(s_160_1).unwrap());
        // D s_160_3: cmp-eq s_160_2 s_160_0
        let s_160_3: bool = ((s_160_2) == (s_160_0));
        // D s_160_4: write-var gs#16272 <= s_160_3
        fn_state.gs_16272 = s_160_3;
        // N s_160_5: jump b161
        return block_161(state, tracer, fn_state);
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_161_0: read-var gs#16272:u8
        let s_161_0: bool = fn_state.gs_16272;
        // N s_161_1: assert s_161_0
        let s_161_1: () = assert!(s_161_0);
        // C s_161_2: const #12184u : u32
        let s_161_2: u32 = 12184;
        // D s_161_3: read-reg s_161_2:[struct; 64]
        let s_161_3: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_161_2 as isize);
            tracer.read_register(s_161_2 as isize, value);
            value
        };
        // D s_161_4: read-var nshadow#270:i
        let s_161_4: i128 = fn_state.nshadow_270;
        // D s_161_5: read-element s_161_3[s_161_4]
        let s_161_5: ProductType5c790c8ef59cc8b2 = s_161_3[(s_161_4) as usize];
        // D s_161_6: call _get_DBGBCR_EL1_Type_BAS(s_161_5)
        let s_161_6: u8 = u_get_DBGBCR_EL1_Type_BAS(state, tracer, s_161_5);
        // D s_161_7: cast zx s_161_6 -> bv
        let s_161_7: Bits = Bits::new(s_161_6 as u128, 4u16);
        // D s_161_8: read-var byte:i64
        let s_161_8: i64 = fn_state.byte;
        // D s_161_9: cast zx s_161_8 -> i
        let s_161_9: i128 = (i128::try_from(s_161_8).unwrap());
        // C s_161_10: const #1u : u64
        let s_161_10: u64 = 1;
        // D s_161_11: bit-extract s_161_7 s_161_9 s_161_10
        let s_161_11: Bits = (Bits::new(
            ((s_161_7) >> (s_161_9)).value(),
            u16::try_from(s_161_10).unwrap(),
        ));
        // D s_161_12: cast reint s_161_11 -> u8
        let s_161_12: bool = ((s_161_11.value()) != 0);
        // C s_161_13: const #0s : i
        let s_161_13: i128 = 0;
        // C s_161_14: const #0u : u64
        let s_161_14: u64 = 0;
        // D s_161_15: cast zx s_161_12 -> u64
        let s_161_15: u64 = (s_161_12 as u64);
        // C s_161_16: const #1u : u64
        let s_161_16: u64 = 1;
        // D s_161_17: and s_161_15 s_161_16
        let s_161_17: u64 = ((s_161_15) & (s_161_16));
        // D s_161_18: cmp-eq s_161_17 s_161_16
        let s_161_18: bool = ((s_161_17) == (s_161_16));
        // D s_161_19: lsl s_161_15 s_161_13
        let s_161_19: u64 = s_161_15 << s_161_13;
        // D s_161_20: or s_161_14 s_161_19
        let s_161_20: u64 = ((s_161_14) | (s_161_19));
        // D s_161_21: cmpl s_161_19
        let s_161_21: u64 = !s_161_19;
        // D s_161_22: and s_161_14 s_161_21
        let s_161_22: u64 = ((s_161_14) & (s_161_21));
        // D s_161_23: select s_161_18 s_161_20 s_161_22
        let s_161_23: u64 = if s_161_18 { s_161_20 } else { s_161_22 };
        // D s_161_24: cast trunc s_161_23 -> u8
        let s_161_24: bool = ((s_161_23) != 0);
        // D s_161_25: cast zx s_161_24 -> bv
        let s_161_25: Bits = Bits::new(s_161_24 as u128, 1u16);
        // C s_161_26: const #1u : u8
        let s_161_26: bool = true;
        // C s_161_27: cast zx s_161_26 -> bv
        let s_161_27: Bits = Bits::new(s_161_26 as u128, 1u16);
        // D s_161_28: cmp-eq s_161_25 s_161_27
        let s_161_28: bool = ((s_161_25) == (s_161_27));
        // D s_161_29: write-var byte_select_match <= s_161_28
        fn_state.byte_select_match = s_161_28;
        // N s_161_30: jump b119
        return block_119(state, tracer, fn_state);
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_162_0: const #1u : u8
        let s_162_0: bool = true;
        // D s_162_1: write-var gs#16272 <= s_162_0
        fn_state.gs_16272 = s_162_0;
        // N s_162_2: jump b161
        return block_161(state, tracer, fn_state);
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_163_0: const #12184u : u32
        let s_163_0: u32 = 12184;
        // D s_163_1: read-reg s_163_0:[struct; 64]
        let s_163_1: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_163_0 as isize);
            tracer.read_register(s_163_0 as isize, value);
            value
        };
        // D s_163_2: read-var nshadow#270:i
        let s_163_2: i128 = fn_state.nshadow_270;
        // D s_163_3: read-element s_163_1[s_163_2]
        let s_163_3: ProductType5c790c8ef59cc8b2 = s_163_1[(s_163_2) as usize];
        // D s_163_4: call _get_DBGBCR_EL1_Type_MASK(s_163_3)
        let s_163_4: u8 = u_get_DBGBCR_EL1_Type_MASK(state, tracer, s_163_3);
        // D s_163_5: cast zx s_163_4 -> bv
        let s_163_5: Bits = Bits::new(s_163_4 as u128, 5u16);
        // D s_163_6: cast zx s_163_5 -> i
        let s_163_6: i128 = (s_163_5.value() as i128);
        // D s_163_7: write-var mask <= s_163_6
        fn_state.mask = s_163_6;
        // C s_163_8: const #1s : i
        let s_163_8: i128 = 1;
        // D s_163_9: read-var mask:i
        let s_163_9: i128 = fn_state.mask;
        // D s_163_10: cmp-eq s_163_9 s_163_8
        let s_163_10: bool = ((s_163_9) == (s_163_8));
        // N s_163_11: branch s_163_10 b201 b164
        if s_163_10 {
            return block_201(state, tracer, fn_state);
        } else {
            return block_164(state, tracer, fn_state);
        };
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_164_0: const #2s : i
        let s_164_0: i128 = 2;
        // D s_164_1: read-var mask:i
        let s_164_1: i128 = fn_state.mask;
        // D s_164_2: cmp-eq s_164_1 s_164_0
        let s_164_2: bool = ((s_164_1) == (s_164_0));
        // D s_164_3: write-var gs#16194 <= s_164_2
        fn_state.gs_16194 = s_164_2;
        // N s_164_4: jump b165
        return block_165(state, tracer, fn_state);
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_165_0: read-var gs#16194:u8
        let s_165_0: bool = fn_state.gs_16194;
        // N s_165_1: branch s_165_0 b189 b166
        if s_165_0 {
            return block_189(state, tracer, fn_state);
        } else {
            return block_166(state, tracer, fn_state);
        };
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // N s_166_0: jump b167
        return block_167(state, tracer, fn_state);
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_167_0: const #0s : i
        let s_167_0: i128 = 0;
        // D s_167_1: read-var mask:i
        let s_167_1: i128 = fn_state.mask;
        // D s_167_2: call neq_int(s_167_1, s_167_0)
        let s_167_2: bool = neq_int(state, tracer, s_167_1, s_167_0);
        // N s_167_3: branch s_167_2 b171 b168
        if s_167_2 {
            return block_171(state, tracer, fn_state);
        } else {
            return block_168(state, tracer, fn_state);
        };
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_168_0: read-var mismatch:u8
        let s_168_0: bool = fn_state.mismatch;
        // N s_168_1: branch s_168_0 b170 b169
        if s_168_0 {
            return block_170(state, tracer, fn_state);
        } else {
            return block_169(state, tracer, fn_state);
        };
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // N s_169_0: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_170_0: const #0u : u8
        let s_170_0: bool = false;
        // C s_170_1: const #0u : u8
        let s_170_1: bool = false;
        // D s_170_2: create-product struct = ["s_170_0", "s_170_1"]
        let s_170_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_170_0,
            _1: s_170_1,
        };
        // D s_170_3: write-var return_value <= s_170_2
        fn_state.return_value = s_170_2;
        // N s_170_4: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_171_0: read-var match_cid:u8
        let s_171_0: bool = fn_state.match_cid;
        // N s_171_1: branch s_171_0 b188 b172
        if s_171_0 {
            return block_188(state, tracer, fn_state);
        } else {
            return block_172(state, tracer, fn_state);
        };
    }
    fn block_172<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_172_0: read-var match_cid1:u8
        let s_172_0: bool = fn_state.match_cid1;
        // D s_172_1: write-var gs#16215 <= s_172_0
        fn_state.gs_16215 = s_172_0;
        // N s_172_2: jump b173
        return block_173(state, tracer, fn_state);
    }
    fn block_173<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_173_0: read-var gs#16215:u8
        let s_173_0: bool = fn_state.gs_16215;
        // N s_173_1: branch s_173_0 b187 b174
        if s_173_0 {
            return block_187(state, tracer, fn_state);
        } else {
            return block_174(state, tracer, fn_state);
        };
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_174_0: read-var match_cid2:u8
        let s_174_0: bool = fn_state.match_cid2;
        // D s_174_1: write-var gs#16216 <= s_174_0
        fn_state.gs_16216 = s_174_0;
        // N s_174_2: jump b175
        return block_175(state, tracer, fn_state);
    }
    fn block_175<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_175_0: read-var gs#16216:u8
        let s_175_0: bool = fn_state.gs_16216;
        // N s_175_1: branch s_175_0 b186 b176
        if s_175_0 {
            return block_186(state, tracer, fn_state);
        } else {
            return block_176(state, tracer, fn_state);
        };
    }
    fn block_176<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_176_0: const #12184u : u32
        let s_176_0: u32 = 12184;
        // D s_176_1: read-reg s_176_0:[struct; 64]
        let s_176_1: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_176_0 as isize);
            tracer.read_register(s_176_0 as isize, value);
            value
        };
        // D s_176_2: read-var nshadow#270:i
        let s_176_2: i128 = fn_state.nshadow_270;
        // D s_176_3: read-element s_176_1[s_176_2]
        let s_176_3: ProductType5c790c8ef59cc8b2 = s_176_1[(s_176_2) as usize];
        // D s_176_4: call _get_DBGBCR_EL1_Type_BAS(s_176_3)
        let s_176_4: u8 = u_get_DBGBCR_EL1_Type_BAS(state, tracer, s_176_3);
        // D s_176_5: cast zx s_176_4 -> bv
        let s_176_5: Bits = Bits::new(s_176_4 as u128, 4u16);
        // C s_176_6: const #15u : u8
        let s_176_6: u8 = 15;
        // C s_176_7: cast zx s_176_6 -> bv
        let s_176_7: Bits = Bits::new(s_176_6 as u128, 4u16);
        // D s_176_8: cmp-ne s_176_5 s_176_7
        let s_176_8: bool = ((s_176_5) != (s_176_7));
        // N s_176_9: branch s_176_8 b185 b177
        if s_176_8 {
            return block_185(state, tracer, fn_state);
        } else {
            return block_177(state, tracer, fn_state);
        };
    }
    fn block_177<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_177_0: const #0u : u8
        let s_177_0: bool = false;
        // D s_177_1: write-var gs#16217 <= s_177_0
        fn_state.gs_16217 = s_177_0;
        // N s_177_2: jump b178
        return block_178(state, tracer, fn_state);
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_178_0: read-var gs#16217:u8
        let s_178_0: bool = fn_state.gs_16217;
        // D s_178_1: write-var gs#16218 <= s_178_0
        fn_state.gs_16218 = s_178_0;
        // N s_178_2: jump b179
        return block_179(state, tracer, fn_state);
    }
    fn block_179<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_179_0: read-var gs#16218:u8
        let s_179_0: bool = fn_state.gs_16218;
        // N s_179_1: branch s_179_0 b182 b180
        if s_179_0 {
            return block_182(state, tracer, fn_state);
        } else {
            return block_180(state, tracer, fn_state);
        };
    }
    fn block_180<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // N s_180_0: jump b181
        return block_181(state, tracer, fn_state);
    }
    fn block_181<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // N s_181_0: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_182<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_182_0: const #40u : u32
        let s_182_0: u32 = 40;
        // S s_182_1: call ConstrainUnpredictableBool(s_182_0)
        let s_182_1: bool = ConstrainUnpredictableBool(state, tracer, s_182_0);
        // S s_182_2: not s_182_1
        let s_182_2: bool = !s_182_1;
        // N s_182_3: branch s_182_2 b184 b183
        if s_182_2 {
            return block_184(state, tracer, fn_state);
        } else {
            return block_183(state, tracer, fn_state);
        };
    }
    fn block_183<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // N s_183_0: jump b181
        return block_181(state, tracer, fn_state);
    }
    fn block_184<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_184_0: const #0u : u8
        let s_184_0: bool = false;
        // C s_184_1: const #0u : u8
        let s_184_1: bool = false;
        // D s_184_2: create-product struct = ["s_184_0", "s_184_1"]
        let s_184_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_184_0,
            _1: s_184_1,
        };
        // D s_184_3: write-var return_value <= s_184_2
        fn_state.return_value = s_184_2;
        // N s_184_4: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_185<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_185_0: const #() : ()
        let s_185_0: () = ();
        // S s_185_1: call HaveAArch32(s_185_0)
        let s_185_1: bool = HaveAArch32(state, tracer, s_185_0);
        // D s_185_2: write-var gs#16217 <= s_185_1
        fn_state.gs_16217 = s_185_1;
        // N s_185_3: jump b178
        return block_178(state, tracer, fn_state);
    }
    fn block_186<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_186_0: const #1u : u8
        let s_186_0: bool = true;
        // D s_186_1: write-var gs#16218 <= s_186_0
        fn_state.gs_16218 = s_186_0;
        // N s_186_2: jump b179
        return block_179(state, tracer, fn_state);
    }
    fn block_187<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_187_0: const #1u : u8
        let s_187_0: bool = true;
        // D s_187_1: write-var gs#16216 <= s_187_0
        fn_state.gs_16216 = s_187_0;
        // N s_187_2: jump b175
        return block_175(state, tracer, fn_state);
    }
    fn block_188<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_188_0: const #1u : u8
        let s_188_0: bool = true;
        // D s_188_1: write-var gs#16215 <= s_188_0
        fn_state.gs_16215 = s_188_0;
        // N s_188_2: jump b173
        return block_173(state, tracer, fn_state);
    }
    fn block_189<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_189_0: const #3s : i
        let s_189_0: i128 = 3;
        // C s_189_1: const #31s : i
        let s_189_1: i128 = 31;
        // C s_189_2: const #39u : u32
        let s_189_2: u32 = 39;
        // S s_189_3: call ConstrainUnpredictableInteger(s_189_0, s_189_1, s_189_2)
        let s_189_3: ProductType396b95aa74979079 = ConstrainUnpredictableInteger(
            state,
            tracer,
            s_189_0,
            s_189_1,
            s_189_2,
        );
        // D s_189_4: write-var ga#12027 <= s_189_3
        fn_state.ga_12027 = s_189_3;
        // D s_189_5: read-var ga#12027.0:struct
        let s_189_5: u32 = fn_state.ga_12027._0;
        // D s_189_6: read-var ga#12027.1:struct
        let s_189_6: i128 = fn_state.ga_12027._1;
        // D s_189_7: write-var c <= s_189_5
        fn_state.c = s_189_5;
        // D s_189_8: write-var mask <= s_189_6
        fn_state.mask = s_189_6;
        // D s_189_9: read-var c:u32
        let s_189_9: u32 = fn_state.c;
        // C s_189_10: const #7u : u32
        let s_189_10: u32 = 7;
        // D s_189_11: cmp-eq s_189_9 s_189_10
        let s_189_11: bool = ((s_189_9) == (s_189_10));
        // N s_189_12: branch s_189_11 b200 b190
        if s_189_11 {
            return block_200(state, tracer, fn_state);
        } else {
            return block_190(state, tracer, fn_state);
        };
    }
    fn block_190<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_190_0: read-var c:u32
        let s_190_0: u32 = fn_state.c;
        // C s_190_1: const #0u : u32
        let s_190_1: u32 = 0;
        // D s_190_2: cmp-eq s_190_0 s_190_1
        let s_190_2: bool = ((s_190_0) == (s_190_1));
        // N s_190_3: branch s_190_2 b199 b191
        if s_190_2 {
            return block_199(state, tracer, fn_state);
        } else {
            return block_191(state, tracer, fn_state);
        };
    }
    fn block_191<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_191_0: read-var c:u32
        let s_191_0: u32 = fn_state.c;
        // C s_191_1: const #1u : u32
        let s_191_1: u32 = 1;
        // D s_191_2: cmp-eq s_191_0 s_191_1
        let s_191_2: bool = ((s_191_0) == (s_191_1));
        // D s_191_3: write-var gs#16201 <= s_191_2
        fn_state.gs_16201 = s_191_2;
        // N s_191_4: jump b192
        return block_192(state, tracer, fn_state);
    }
    fn block_192<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_192_0: read-var gs#16201:u8
        let s_192_0: bool = fn_state.gs_16201;
        // D s_192_1: write-var gs#16202 <= s_192_0
        fn_state.gs_16202 = s_192_0;
        // N s_192_2: jump b193
        return block_193(state, tracer, fn_state);
    }
    fn block_193<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_193_0: read-var gs#16202:u8
        let s_193_0: bool = fn_state.gs_16202;
        // N s_193_1: assert s_193_0
        let s_193_1: () = assert!(s_193_0);
        // C s_193_2: const #7u : u32
        let s_193_2: u32 = 7;
        // D s_193_3: read-var c:u32
        let s_193_3: u32 = fn_state.c;
        // D s_193_4: cmp-eq s_193_2 s_193_3
        let s_193_4: bool = ((s_193_2) == (s_193_3));
        // D s_193_5: not s_193_4
        let s_193_5: bool = !s_193_4;
        // N s_193_6: branch s_193_5 b195 b194
        if s_193_5 {
            return block_195(state, tracer, fn_state);
        } else {
            return block_194(state, tracer, fn_state);
        };
    }
    fn block_194<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_194_0: const #0u : u8
        let s_194_0: bool = false;
        // C s_194_1: const #0u : u8
        let s_194_1: bool = false;
        // D s_194_2: create-product struct = ["s_194_0", "s_194_1"]
        let s_194_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_194_0,
            _1: s_194_1,
        };
        // D s_194_3: write-var return_value <= s_194_2
        fn_state.return_value = s_194_2;
        // N s_194_4: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_195<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_195_0: const #0u : u32
        let s_195_0: u32 = 0;
        // D s_195_1: read-var c:u32
        let s_195_1: u32 = fn_state.c;
        // D s_195_2: cmp-eq s_195_0 s_195_1
        let s_195_2: bool = ((s_195_0) == (s_195_1));
        // D s_195_3: not s_195_2
        let s_195_3: bool = !s_195_2;
        // N s_195_4: branch s_195_3 b198 b196
        if s_195_3 {
            return block_198(state, tracer, fn_state);
        } else {
            return block_196(state, tracer, fn_state);
        };
    }
    fn block_196<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_196_0: const #0s : i
        let s_196_0: i128 = 0;
        // D s_196_1: write-var mask <= s_196_0
        fn_state.mask = s_196_0;
        // N s_196_2: jump b197
        return block_197(state, tracer, fn_state);
    }
    fn block_197<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // N s_197_0: jump b167
        return block_167(state, tracer, fn_state);
    }
    fn block_198<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // N s_198_0: jump b197
        return block_197(state, tracer, fn_state);
    }
    fn block_199<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_199_0: const #1u : u8
        let s_199_0: bool = true;
        // D s_199_1: write-var gs#16201 <= s_199_0
        fn_state.gs_16201 = s_199_0;
        // N s_199_2: jump b192
        return block_192(state, tracer, fn_state);
    }
    fn block_200<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_200_0: const #1u : u8
        let s_200_0: bool = true;
        // D s_200_1: write-var gs#16202 <= s_200_0
        fn_state.gs_16202 = s_200_0;
        // N s_200_2: jump b193
        return block_193(state, tracer, fn_state);
    }
    fn block_201<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_201_0: const #1u : u8
        let s_201_0: bool = true;
        // D s_201_1: write-var gs#16194 <= s_201_0
        fn_state.gs_16194 = s_201_0;
        // N s_201_2: jump b165
        return block_165(state, tracer, fn_state);
    }
    fn block_202<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_202_0: const #0u : u8
        let s_202_0: bool = false;
        // C s_202_1: const #0u : u8
        let s_202_1: bool = false;
        // D s_202_2: create-product struct = ["s_202_0", "s_202_1"]
        let s_202_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_202_0,
            _1: s_202_1,
        };
        // D s_202_3: write-var return_value <= s_202_2
        fn_state.return_value = s_202_2;
        // N s_202_4: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_203<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_203_0: const #() : ()
        let s_203_0: () = ();
        // S s_203_1: call UsingAArch32(s_203_0)
        let s_203_1: bool = UsingAArch32(state, tracer, s_203_0);
        // D s_203_2: write-var gs#16180 <= s_203_1
        fn_state.gs_16180 = s_203_1;
        // N s_203_3: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_204<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_204_0: const #42u : u32
        let s_204_0: u32 = 42;
        // S s_204_1: call ConstrainUnpredictableBool(s_204_0)
        let s_204_1: bool = ConstrainUnpredictableBool(state, tracer, s_204_0);
        // S s_204_2: not s_204_1
        let s_204_2: bool = !s_204_1;
        // N s_204_3: branch s_204_2 b206 b205
        if s_204_2 {
            return block_206(state, tracer, fn_state);
        } else {
            return block_205(state, tracer, fn_state);
        };
    }
    fn block_205<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // N s_205_0: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_206<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_206_0: const #0u : u8
        let s_206_0: bool = false;
        // C s_206_1: const #0u : u8
        let s_206_1: bool = false;
        // D s_206_2: create-product struct = ["s_206_0", "s_206_1"]
        let s_206_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_206_0,
            _1: s_206_1,
        };
        // D s_206_3: write-var return_value <= s_206_2
        fn_state.return_value = s_206_2;
        // N s_206_4: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_207<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_207_0: read-var isbreakpnt:u8
        let s_207_0: bool = fn_state.isbreakpnt;
        // D s_207_1: write-var gs#16179 <= s_207_0
        fn_state.gs_16179 = s_207_0;
        // N s_207_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_208<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_208_0: read-var match_addr:u8
        let s_208_0: bool = fn_state.match_addr;
        // D s_208_1: write-var gs#16178 <= s_208_0
        fn_state.gs_16178 = s_208_0;
        // N s_208_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_209<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_209_0: const #0u : u8
        let s_209_0: bool = false;
        // C s_209_1: const #0u : u8
        let s_209_1: bool = false;
        // D s_209_2: create-product struct = ["s_209_0", "s_209_1"]
        let s_209_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_209_0,
            _1: s_209_1,
        };
        // D s_209_3: write-var return_value <= s_209_2
        fn_state.return_value = s_209_2;
        // N s_209_4: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_210<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_210_0: read-var match_addr:u8
        let s_210_0: bool = fn_state.match_addr;
        // D s_210_1: not s_210_0
        let s_210_1: bool = !s_210_0;
        // D s_210_2: write-var gs#16177 <= s_210_1
        fn_state.gs_16177 = s_210_1;
        // N s_210_3: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_211<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_211_0: read-var linking_enabled:u8
        let s_211_0: bool = fn_state.linking_enabled;
        // D s_211_1: write-var gs#16176 <= s_211_0
        fn_state.gs_16176 = s_211_0;
        // N s_211_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_212<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_212_0: const #0u : u8
        let s_212_0: bool = false;
        // C s_212_1: const #0u : u8
        let s_212_1: bool = false;
        // D s_212_2: create-product struct = ["s_212_0", "s_212_1"]
        let s_212_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_212_0,
            _1: s_212_1,
        };
        // D s_212_3: write-var return_value <= s_212_2
        fn_state.return_value = s_212_2;
        // N s_212_4: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_213<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_213_0: read-var linking_enabled:u8
        let s_213_0: bool = fn_state.linking_enabled;
        // D s_213_1: not s_213_0
        let s_213_1: bool = !s_213_0;
        // D s_213_2: write-var gs#16175 <= s_213_1
        fn_state.gs_16175 = s_213_1;
        // N s_213_3: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_214<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_214_0: const #1u : u8
        let s_214_0: bool = true;
        // D s_214_1: write-var gs#16174 <= s_214_0
        fn_state.gs_16174 = s_214_0;
        // N s_214_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_215<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_215_0: read-var dbgtype:u8
        let s_215_0: u8 = fn_state.dbgtype;
        // D s_215_1: write-var b__1 <= s_215_0
        fn_state.b__1 = s_215_0;
        // C s_215_2: const #3s : i
        let s_215_2: i128 = 3;
        // D s_215_3: read-var b__1:u8
        let s_215_3: u8 = fn_state.b__1;
        // D s_215_4: cast zx s_215_3 -> bv
        let s_215_4: Bits = Bits::new(s_215_3 as u128, 4u16);
        // C s_215_5: const #1s : i64
        let s_215_5: i64 = 1;
        // C s_215_6: cast zx s_215_5 -> i
        let s_215_6: i128 = (i128::try_from(s_215_5).unwrap());
        // C s_215_7: const #0s : i
        let s_215_7: i128 = 0;
        // C s_215_8: add s_215_7 s_215_6
        let s_215_8: i128 = (s_215_7 + s_215_6);
        // D s_215_9: bit-extract s_215_4 s_215_2 s_215_8
        let s_215_9: Bits = (Bits::new(
            ((s_215_4) >> (s_215_2)).value(),
            u16::try_from(s_215_8).unwrap(),
        ));
        // D s_215_10: cast reint s_215_9 -> u8
        let s_215_10: bool = ((s_215_9.value()) != 0);
        // D s_215_11: cast zx s_215_10 -> bv
        let s_215_11: Bits = Bits::new(s_215_10 as u128, 1u16);
        // C s_215_12: const #1u : u8
        let s_215_12: bool = true;
        // C s_215_13: cast zx s_215_12 -> bv
        let s_215_13: Bits = Bits::new(s_215_12 as u128, 1u16);
        // D s_215_14: cmp-eq s_215_11 s_215_13
        let s_215_14: bool = ((s_215_11) == (s_215_13));
        // N s_215_15: branch s_215_14 b220 b216
        if s_215_14 {
            return block_220(state, tracer, fn_state);
        } else {
            return block_216(state, tracer, fn_state);
        };
    }
    fn block_216<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_216_0: const #0u : u8
        let s_216_0: bool = false;
        // D s_216_1: write-var gs#16171 <= s_216_0
        fn_state.gs_16171 = s_216_0;
        // N s_216_2: jump b217
        return block_217(state, tracer, fn_state);
    }
    fn block_217<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_217_0: read-var gs#16171:u8
        let s_217_0: bool = fn_state.gs_16171;
        // D s_217_1: not s_217_0
        let s_217_1: bool = !s_217_0;
        // N s_217_2: branch s_217_1 b219 b218
        if s_217_1 {
            return block_219(state, tracer, fn_state);
        } else {
            return block_218(state, tracer, fn_state);
        };
    }
    fn block_218<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_218_0: const #1u : u8
        let s_218_0: bool = true;
        // D s_218_1: write-var gs#16163 <= s_218_0
        fn_state.gs_16163 = s_218_0;
        // N s_218_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_219<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_219_0: const #0u : u8
        let s_219_0: bool = false;
        // D s_219_1: write-var gs#16163 <= s_219_0
        fn_state.gs_16163 = s_219_0;
        // N s_219_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_220<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_220_0: const #0s : i
        let s_220_0: i128 = 0;
        // D s_220_1: read-var b__1:u8
        let s_220_1: u8 = fn_state.b__1;
        // D s_220_2: cast zx s_220_1 -> bv
        let s_220_2: Bits = Bits::new(s_220_1 as u128, 4u16);
        // C s_220_3: const #1s : i64
        let s_220_3: i64 = 1;
        // C s_220_4: cast zx s_220_3 -> i
        let s_220_4: i128 = (i128::try_from(s_220_3).unwrap());
        // C s_220_5: const #0s : i
        let s_220_5: i128 = 0;
        // C s_220_6: add s_220_5 s_220_4
        let s_220_6: i128 = (s_220_5 + s_220_4);
        // D s_220_7: bit-extract s_220_2 s_220_0 s_220_6
        let s_220_7: Bits = (Bits::new(
            ((s_220_2) >> (s_220_0)).value(),
            u16::try_from(s_220_6).unwrap(),
        ));
        // D s_220_8: cast reint s_220_7 -> u8
        let s_220_8: bool = ((s_220_7.value()) != 0);
        // D s_220_9: cast zx s_220_8 -> bv
        let s_220_9: Bits = Bits::new(s_220_8 as u128, 1u16);
        // C s_220_10: const #1u : u8
        let s_220_10: bool = true;
        // C s_220_11: cast zx s_220_10 -> bv
        let s_220_11: Bits = Bits::new(s_220_10 as u128, 1u16);
        // D s_220_12: cmp-eq s_220_9 s_220_11
        let s_220_12: bool = ((s_220_9) == (s_220_11));
        // D s_220_13: write-var gs#16171 <= s_220_12
        fn_state.gs_16171 = s_220_12;
        // N s_220_14: jump b217
        return block_217(state, tracer, fn_state);
    }
    fn block_221<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_221_0: const #0u : u8
        let s_221_0: bool = false;
        // D s_221_1: write-var gs#16158 <= s_221_0
        fn_state.gs_16158 = s_221_0;
        // N s_221_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_222<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_222_0: read-var dbgtype:u8
        let s_222_0: u8 = fn_state.dbgtype;
        // C s_222_1: const #1s : i
        let s_222_1: i128 = 1;
        // D s_222_2: cast zx s_222_0 -> bv
        let s_222_2: Bits = Bits::new(s_222_0 as u128, 4u16);
        // C s_222_3: const #1s : i64
        let s_222_3: i64 = 1;
        // C s_222_4: cast zx s_222_3 -> i
        let s_222_4: i128 = (i128::try_from(s_222_3).unwrap());
        // C s_222_5: const #1s : i
        let s_222_5: i128 = 1;
        // C s_222_6: add s_222_5 s_222_4
        let s_222_6: i128 = (s_222_5 + s_222_4);
        // D s_222_7: bit-extract s_222_2 s_222_1 s_222_6
        let s_222_7: Bits = (Bits::new(
            ((s_222_2) >> (s_222_1)).value(),
            u16::try_from(s_222_6).unwrap(),
        ));
        // D s_222_8: cast reint s_222_7 -> u8
        let s_222_8: u8 = (s_222_7.value() as u8);
        // D s_222_9: cast zx s_222_8 -> bv
        let s_222_9: Bits = Bits::new(s_222_8 as u128, 2u16);
        // C s_222_10: const #3u : u8
        let s_222_10: u8 = 3;
        // C s_222_11: cast zx s_222_10 -> bv
        let s_222_11: Bits = Bits::new(s_222_10 as u128, 2u16);
        // D s_222_12: cmp-eq s_222_9 s_222_11
        let s_222_12: bool = ((s_222_9) == (s_222_11));
        // D s_222_13: not s_222_12
        let s_222_13: bool = !s_222_12;
        // N s_222_14: branch s_222_13 b224 b223
        if s_222_13 {
            return block_224(state, tracer, fn_state);
        } else {
            return block_223(state, tracer, fn_state);
        };
    }
    fn block_223<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_223_0: const #1u : u8
        let s_223_0: bool = true;
        // D s_223_1: write-var gs#16150 <= s_223_0
        fn_state.gs_16150 = s_223_0;
        // N s_223_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_224<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_224_0: const #0u : u8
        let s_224_0: bool = false;
        // D s_224_1: write-var gs#16150 <= s_224_0
        fn_state.gs_16150 = s_224_0;
        // N s_224_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_225<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_225_0: const #0u : u8
        let s_225_0: bool = false;
        // D s_225_1: write-var gs#16145 <= s_225_0
        fn_state.gs_16145 = s_225_0;
        // N s_225_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_226<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_226_0: const #0u : u8
        let s_226_0: bool = false;
        // D s_226_1: write-var gs#16140 <= s_226_0
        fn_state.gs_16140 = s_226_0;
        // N s_226_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_227<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_227_0: const #0u : u8
        let s_227_0: bool = false;
        // D s_227_1: write-var gs#16135 <= s_227_0
        fn_state.gs_16135 = s_227_0;
        // N s_227_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_228<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_228_0: const #0u : u8
        let s_228_0: bool = false;
        // D s_228_1: write-var gs#16127 <= s_228_0
        fn_state.gs_16127 = s_228_0;
        // N s_228_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_229<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_229_0: const #1s : i
        let s_229_0: i128 = 1;
        // D s_229_1: read-var b__8:u8
        let s_229_1: u8 = fn_state.b__8;
        // D s_229_2: cast zx s_229_1 -> bv
        let s_229_2: Bits = Bits::new(s_229_1 as u128, 4u16);
        // C s_229_3: const #1s : i64
        let s_229_3: i64 = 1;
        // C s_229_4: cast zx s_229_3 -> i
        let s_229_4: i128 = (i128::try_from(s_229_3).unwrap());
        // C s_229_5: const #0s : i
        let s_229_5: i128 = 0;
        // C s_229_6: add s_229_5 s_229_4
        let s_229_6: i128 = (s_229_5 + s_229_4);
        // D s_229_7: bit-extract s_229_2 s_229_0 s_229_6
        let s_229_7: Bits = (Bits::new(
            ((s_229_2) >> (s_229_0)).value(),
            u16::try_from(s_229_6).unwrap(),
        ));
        // D s_229_8: cast reint s_229_7 -> u8
        let s_229_8: bool = ((s_229_7.value()) != 0);
        // D s_229_9: cast zx s_229_8 -> bv
        let s_229_9: Bits = Bits::new(s_229_8 as u128, 1u16);
        // C s_229_10: const #0u : u8
        let s_229_10: bool = false;
        // C s_229_11: cast zx s_229_10 -> bv
        let s_229_11: Bits = Bits::new(s_229_10 as u128, 1u16);
        // D s_229_12: cmp-eq s_229_9 s_229_11
        let s_229_12: bool = ((s_229_9) == (s_229_11));
        // D s_229_13: write-var gs#16132 <= s_229_12
        fn_state.gs_16132 = s_229_12;
        // N s_229_14: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_230<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_230_0: const #0u : u8
        let s_230_0: bool = false;
        // C s_230_1: const #0u : u8
        let s_230_1: bool = false;
        // D s_230_2: create-product struct = ["s_230_0", "s_230_1"]
        let s_230_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_230_0,
            _1: s_230_1,
        };
        // D s_230_3: write-var return_value <= s_230_2
        fn_state.return_value = s_230_2;
        // N s_230_4: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_231<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_231_0: const #12184u : u32
        let s_231_0: u32 = 12184;
        // D s_231_1: read-reg s_231_0:[struct; 64]
        let s_231_1: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_231_0 as isize);
            tracer.read_register(s_231_0 as isize, value);
            value
        };
        // D s_231_2: read-var nshadow#270:i
        let s_231_2: i128 = fn_state.nshadow_270;
        // D s_231_3: read-element s_231_1[s_231_2]
        let s_231_3: ProductType5c790c8ef59cc8b2 = s_231_1[(s_231_2) as usize];
        // D s_231_4: call _get_DBGBCR_EL1_Type_BT2(s_231_3)
        let s_231_4: bool = u_get_DBGBCR_EL1_Type_BT2(state, tracer, s_231_3);
        // D s_231_5: write-var bt2 <= s_231_4
        fn_state.bt2 = s_231_4;
        // N s_231_6: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_232<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_232_0: const #0u : u8
        let s_232_0: bool = false;
        // C s_232_1: const #0u : u8
        let s_232_1: bool = false;
        // D s_232_2: create-product struct = ["s_232_0", "s_232_1"]
        let s_232_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_232_0,
            _1: s_232_1,
        };
        // D s_232_3: write-var return_value <= s_232_2
        fn_state.return_value = s_232_2;
        // N s_232_4: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_233<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_233_0: read-var nshadow#270:i
        let s_233_0: i128 = fn_state.nshadow_270;
        // D s_233_1: call __id(s_233_0)
        let s_233_1: i128 = u__id(state, tracer, s_233_0);
        // C s_233_2: const #64s : i
        let s_233_2: i128 = 64;
        // D s_233_3: cmp-lt s_233_1 s_233_2
        let s_233_3: bool = ((s_233_1) < (s_233_2));
        // D s_233_4: write-var gs#16116 <= s_233_3
        fn_state.gs_16116 = s_233_3;
        // N s_233_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_234<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_234_0: const #() : ()
        let s_234_0: () = ();
        // S s_234_1: call NumBreakpointsImplemented(s_234_0)
        let s_234_1: i128 = NumBreakpointsImplemented(state, tracer, s_234_0);
        // C s_234_2: const #1s : i
        let s_234_2: i128 = 1;
        // S s_234_3: sub s_234_1 s_234_2
        let s_234_3: i128 = ((s_234_1) - (s_234_2));
        // C s_234_4: const #0s : i
        let s_234_4: i128 = 0;
        // C s_234_5: const #32u : u32
        let s_234_5: u32 = 32;
        // S s_234_6: call ConstrainUnpredictableInteger(s_234_4, s_234_3, s_234_5)
        let s_234_6: ProductType396b95aa74979079 = ConstrainUnpredictableInteger(
            state,
            tracer,
            s_234_4,
            s_234_3,
            s_234_5,
        );
        // D s_234_7: write-var ga#12001 <= s_234_6
        fn_state.ga_12001 = s_234_6;
        // D s_234_8: read-var ga#12001.0:struct
        let s_234_8: u32 = fn_state.ga_12001._0;
        // D s_234_9: read-var ga#12001.1:struct
        let s_234_9: i128 = fn_state.ga_12001._1;
        // D s_234_10: write-var c <= s_234_8
        fn_state.c = s_234_8;
        // D s_234_11: write-var n <= s_234_9
        fn_state.n = s_234_9;
        // D s_234_12: read-var c:u32
        let s_234_12: u32 = fn_state.c;
        // C s_234_13: const #7u : u32
        let s_234_13: u32 = 7;
        // D s_234_14: cmp-eq s_234_12 s_234_13
        let s_234_14: bool = ((s_234_12) == (s_234_13));
        // N s_234_15: branch s_234_14 b239 b235
        if s_234_14 {
            return block_239(state, tracer, fn_state);
        } else {
            return block_235(state, tracer, fn_state);
        };
    }
    fn block_235<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_235_0: read-var c:u32
        let s_235_0: u32 = fn_state.c;
        // C s_235_1: const #1u : u32
        let s_235_1: u32 = 1;
        // D s_235_2: cmp-eq s_235_0 s_235_1
        let s_235_2: bool = ((s_235_0) == (s_235_1));
        // D s_235_3: write-var gs#16108 <= s_235_2
        fn_state.gs_16108 = s_235_2;
        // N s_235_4: jump b236
        return block_236(state, tracer, fn_state);
    }
    fn block_236<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_236_0: read-var gs#16108:u8
        let s_236_0: bool = fn_state.gs_16108;
        // N s_236_1: assert s_236_0
        let s_236_1: () = assert!(s_236_0);
        // D s_236_2: read-var c:u32
        let s_236_2: u32 = fn_state.c;
        // C s_236_3: const #7u : u32
        let s_236_3: u32 = 7;
        // D s_236_4: cmp-eq s_236_2 s_236_3
        let s_236_4: bool = ((s_236_2) == (s_236_3));
        // N s_236_5: branch s_236_4 b238 b237
        if s_236_4 {
            return block_238(state, tracer, fn_state);
        } else {
            return block_237(state, tracer, fn_state);
        };
    }
    fn block_237<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // N s_237_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_238<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_238_0: const #0u : u8
        let s_238_0: bool = false;
        // C s_238_1: const #0u : u8
        let s_238_1: bool = false;
        // D s_238_2: create-product struct = ["s_238_0", "s_238_1"]
        let s_238_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_238_0,
            _1: s_238_1,
        };
        // D s_238_3: write-var return_value <= s_238_2
        fn_state.return_value = s_238_2;
        // N s_238_4: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_239<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_239_0: const #1u : u8
        let s_239_0: bool = true;
        // D s_239_1: write-var gs#16108 <= s_239_0
        fn_state.gs_16108 = s_239_0;
        // N s_239_2: jump b236
        return block_236(state, tracer, fn_state);
    }
}
