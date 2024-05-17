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
use DBGBCR_read::*;
use u_get_VTTBR_EL2_Type_VMID::*;
use u_get_DBGBCR_Type_E::*;
use Have16bitVMID::*;
use ConstrainUnpredictableBool::*;
use u_get_VTCR_EL2_Type_VS::*;
use DBGBXVR_read::*;
use NumBreakpointsImplemented::*;
use VTTBR_EL2_read::*;
use u_get_DBGBCR_Type_BAS::*;
use u__id::*;
use u_get_VTTBR_Type_VMID::*;
use ELUsingAArch32::*;
use DBGBVR_read::*;
use ConstrainUnpredictableInteger::*;
use u_get_DBGBCR_Type_BT::*;
use EL2Enabled::*;
use CONTEXTIDR_read::*;
use AArch32_ReservedBreakpointType::*;
use common::*;
pub fn AArch32_BreakpointValueMatch<T: Tracer>(
    state: &mut State,
    tracer: &T,
    n_in: i128,
    vaddress: u32,
    linked_to: bool,
) -> ProductType8b847afc727d5818 {
    #[derive(Default)]
    struct FunctionState {
        ga_23130: bool,
        gs_29804: bool,
        gs_29882: bool,
        vmid: u16,
        match_vmid: bool,
        ga_23068: ProductType54c7e1b9151093d0,
        gs_29796: bool,
        dbgtype: u8,
        bvr_match: bool,
        gs_29832: bool,
        c: u32,
        nshadow_544: i128,
        gs_29862: bool,
        gs_29819: bool,
        byte_select_match: bool,
        gs_29849: bool,
        gs_29904: bool,
        val_match: bool,
        return_value: ProductType8b847afc727d5818,
        ga_23098: ProductType700c18a878c5601b,
        ga_23112: ProductType700c18a878c5601b,
        ga_23090: ProductType700c18a878c5601b,
        gs_29842: bool,
        gs_29906: bool,
        match_cid2: bool,
        gs_29886: bool,
        ga_23091: ProductType700c18a878c5601b,
        gs_29857: bool,
        match_cid1: bool,
        gs_29892: bool,
        gs_29881: bool,
        bxvr_match: bool,
        gs_29827: bool,
        b__5: u8,
        ga_23057: ProductType396b95aa74979079,
        gs_29848: bool,
        n: i128,
        gs_29901: bool,
        gs_29909: bool,
        bvr_match_valid: bool,
        gs_29905: bool,
        gs_29910: bool,
        ga_23080: ProductType700c18a878c5601b,
        match_addr: bool,
        gs_29908: bool,
        gs_29907: bool,
        byte: i64,
        ga_23108: ProductType700c18a878c5601b,
        gs_29837: bool,
        gs_29847: bool,
        gs_29864: bool,
        gs_29814: bool,
        gs_29856: bool,
        gs_29822: bool,
        ga_23123: ProductType700c18a878c5601b,
        bvr_vmid: u16,
        gs_29880: bool,
        linking_enabled: bool,
        mismatch: bool,
        n_in: i128,
        vaddress: u32,
        linked_to: bool,
    }
    let fn_state = FunctionState {
        n_in,
        vaddress,
        linked_to,
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
        // N s_0_6: branch s_0_5 b114 b1
        if s_0_5 {
            return block_114(state, tracer, fn_state);
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
        // D s_2_1: write-var nshadow#544 <= s_2_0
        fn_state.nshadow_544 = s_2_0;
        // D s_2_2: read-var nshadow#544:i
        let s_2_2: i128 = fn_state.nshadow_544;
        // D s_2_3: call __id(s_2_2)
        let s_2_3: i128 = u__id(state, tracer, s_2_2);
        // C s_2_4: const #0s : i
        let s_2_4: i128 = 0;
        // D s_2_5: cmp-le s_2_4 s_2_3
        let s_2_5: bool = ((s_2_4) <= (s_2_3));
        // N s_2_6: branch s_2_5 b113 b3
        if s_2_5 {
            return block_113(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#29804 <= s_3_0
        fn_state.gs_29804 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_4_0: read-var gs#29804:u8
        let s_4_0: bool = fn_state.gs_29804;
        // N s_4_1: assert s_4_0
        let s_4_1: () = assert!(s_4_0);
        // D s_4_2: read-var nshadow#544:i
        let s_4_2: i128 = fn_state.nshadow_544;
        // D s_4_3: cast reint s_4_2 -> i64
        let s_4_3: i64 = (s_4_2 as i64);
        // D s_4_4: call DBGBCR_read(s_4_3)
        let s_4_4: ProductType700c18a878c5601b = DBGBCR_read(state, tracer, s_4_3);
        // D s_4_5: call _get_DBGBCR_Type_E(s_4_4)
        let s_4_5: bool = u_get_DBGBCR_Type_E(state, tracer, s_4_4);
        // D s_4_6: cast zx s_4_5 -> bv
        let s_4_6: Bits = Bits::new(s_4_5 as u128, 1u16);
        // C s_4_7: const #0u : u8
        let s_4_7: bool = false;
        // C s_4_8: cast zx s_4_7 -> bv
        let s_4_8: Bits = Bits::new(s_4_7 as u128, 1u16);
        // D s_4_9: cmp-eq s_4_6 s_4_8
        let s_4_9: bool = ((s_4_6) == (s_4_8));
        // N s_4_10: branch s_4_9 b112 b5
        if s_4_9 {
            return block_112(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_5_0: read-var nshadow#544:i
        let s_5_0: i128 = fn_state.nshadow_544;
        // D s_5_1: cast reint s_5_0 -> i64
        let s_5_1: i64 = (s_5_0 as i64);
        // D s_5_2: call DBGBCR_read(s_5_1)
        let s_5_2: ProductType700c18a878c5601b = DBGBCR_read(state, tracer, s_5_1);
        // D s_5_3: call _get_DBGBCR_Type_BT(s_5_2)
        let s_5_3: u8 = u_get_DBGBCR_Type_BT(state, tracer, s_5_2);
        // D s_5_4: write-var dbgtype <= s_5_3
        fn_state.dbgtype = s_5_3;
        // D s_5_5: read-var nshadow#544:i
        let s_5_5: i128 = fn_state.nshadow_544;
        // D s_5_6: read-var dbgtype:u8
        let s_5_6: u8 = fn_state.dbgtype;
        // D s_5_7: call AArch32_ReservedBreakpointType(s_5_5, s_5_6)
        let s_5_7: ProductType54c7e1b9151093d0 = AArch32_ReservedBreakpointType(
            state,
            tracer,
            s_5_5,
            s_5_6,
        );
        // D s_5_8: write-var ga#23068 <= s_5_7
        fn_state.ga_23068 = s_5_7;
        // D s_5_9: read-var ga#23068.0:struct
        let s_5_9: u32 = fn_state.ga_23068._0;
        // D s_5_10: read-var ga#23068.1:struct
        let s_5_10: u8 = fn_state.ga_23068._1;
        // D s_5_11: write-var c <= s_5_9
        fn_state.c = s_5_9;
        // D s_5_12: write-var dbgtype <= s_5_10
        fn_state.dbgtype = s_5_10;
        // D s_5_13: read-var c:u32
        let s_5_13: u32 = fn_state.c;
        // C s_5_14: const #7u : u32
        let s_5_14: u32 = 7;
        // D s_5_15: cmp-eq s_5_13 s_5_14
        let s_5_15: bool = ((s_5_13) == (s_5_14));
        // N s_5_16: branch s_5_15 b111 b6
        if s_5_15 {
            return block_111(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_6_0: read-var dbgtype:u8
        let s_6_0: u8 = fn_state.dbgtype;
        // D s_6_1: write-var b__5 <= s_6_0
        fn_state.b__5 = s_6_0;
        // C s_6_2: const #3s : i
        let s_6_2: i128 = 3;
        // D s_6_3: read-var b__5:u8
        let s_6_3: u8 = fn_state.b__5;
        // D s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 4u16);
        // C s_6_5: const #1s : i64
        let s_6_5: i64 = 1;
        // C s_6_6: cast zx s_6_5 -> i
        let s_6_6: i128 = (i128::try_from(s_6_5).unwrap());
        // C s_6_7: const #0s : i
        let s_6_7: i128 = 0;
        // C s_6_8: add s_6_7 s_6_6
        let s_6_8: i128 = (s_6_7 + s_6_6);
        // D s_6_9: bit-extract s_6_4 s_6_2 s_6_8
        let s_6_9: Bits = (Bits::new(
            ((s_6_4) >> (s_6_2)).value(),
            u16::try_from(s_6_8).unwrap(),
        ));
        // D s_6_10: cast reint s_6_9 -> u8
        let s_6_10: bool = ((s_6_9.value()) != 0);
        // D s_6_11: cast zx s_6_10 -> bv
        let s_6_11: Bits = Bits::new(s_6_10 as u128, 1u16);
        // C s_6_12: const #0u : u8
        let s_6_12: bool = false;
        // C s_6_13: cast zx s_6_12 -> bv
        let s_6_13: Bits = Bits::new(s_6_12 as u128, 1u16);
        // D s_6_14: cmp-eq s_6_11 s_6_13
        let s_6_14: bool = ((s_6_11) == (s_6_13));
        // N s_6_15: branch s_6_14 b110 b7
        if s_6_14 {
            return block_110(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#29819 <= s_7_0
        fn_state.gs_29819 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_8_0: read-var gs#29819:u8
        let s_8_0: bool = fn_state.gs_29819;
        // D s_8_1: not s_8_0
        let s_8_1: bool = !s_8_0;
        // N s_8_2: branch s_8_1 b109 b9
        if s_8_1 {
            return block_109(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_9_0: const #1u : u8
        let s_9_0: bool = true;
        // D s_9_1: write-var gs#29814 <= s_9_0
        fn_state.gs_29814 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_10_0: read-var gs#29814:u8
        let s_10_0: bool = fn_state.gs_29814;
        // D s_10_1: write-var match_addr <= s_10_0
        fn_state.match_addr = s_10_0;
        // D s_10_2: read-var dbgtype:u8
        let s_10_2: u8 = fn_state.dbgtype;
        // C s_10_3: const #1s : i
        let s_10_3: i128 = 1;
        // D s_10_4: cast zx s_10_2 -> bv
        let s_10_4: Bits = Bits::new(s_10_2 as u128, 4u16);
        // C s_10_5: const #1s : i64
        let s_10_5: i64 = 1;
        // C s_10_6: cast zx s_10_5 -> i
        let s_10_6: i128 = (i128::try_from(s_10_5).unwrap());
        // C s_10_7: const #2s : i
        let s_10_7: i128 = 2;
        // C s_10_8: add s_10_7 s_10_6
        let s_10_8: i128 = (s_10_7 + s_10_6);
        // D s_10_9: bit-extract s_10_4 s_10_3 s_10_8
        let s_10_9: Bits = (Bits::new(
            ((s_10_4) >> (s_10_3)).value(),
            u16::try_from(s_10_8).unwrap(),
        ));
        // D s_10_10: cast reint s_10_9 -> u8
        let s_10_10: u8 = (s_10_9.value() as u8);
        // D s_10_11: cast zx s_10_10 -> bv
        let s_10_11: Bits = Bits::new(s_10_10 as u128, 3u16);
        // C s_10_12: const #2u : u8
        let s_10_12: u8 = 2;
        // C s_10_13: cast zx s_10_12 -> bv
        let s_10_13: Bits = Bits::new(s_10_12 as u128, 3u16);
        // D s_10_14: cmp-eq s_10_11 s_10_13
        let s_10_14: bool = ((s_10_11) == (s_10_13));
        // D s_10_15: not s_10_14
        let s_10_15: bool = !s_10_14;
        // N s_10_16: branch s_10_15 b108 b11
        if s_10_15 {
            return block_108(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#29822 <= s_11_0
        fn_state.gs_29822 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_12_0: read-var gs#29822:u8
        let s_12_0: bool = fn_state.gs_29822;
        // D s_12_1: write-var mismatch <= s_12_0
        fn_state.mismatch = s_12_0;
        // D s_12_2: read-var dbgtype:u8
        let s_12_2: u8 = fn_state.dbgtype;
        // C s_12_3: const #2s : i
        let s_12_3: i128 = 2;
        // D s_12_4: cast zx s_12_2 -> bv
        let s_12_4: Bits = Bits::new(s_12_2 as u128, 4u16);
        // C s_12_5: const #1s : i64
        let s_12_5: i64 = 1;
        // C s_12_6: cast zx s_12_5 -> i
        let s_12_6: i128 = (i128::try_from(s_12_5).unwrap());
        // C s_12_7: const #1s : i
        let s_12_7: i128 = 1;
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
        let s_12_11: Bits = Bits::new(s_12_10 as u128, 2u16);
        // C s_12_12: const #2u : u8
        let s_12_12: u8 = 2;
        // C s_12_13: cast zx s_12_12 -> bv
        let s_12_13: Bits = Bits::new(s_12_12 as u128, 2u16);
        // D s_12_14: cmp-eq s_12_11 s_12_13
        let s_12_14: bool = ((s_12_11) == (s_12_13));
        // D s_12_15: not s_12_14
        let s_12_15: bool = !s_12_14;
        // N s_12_16: branch s_12_15 b107 b13
        if s_12_15 {
            return block_107(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#29827 <= s_13_0
        fn_state.gs_29827 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_14_0: read-var gs#29827:u8
        let s_14_0: bool = fn_state.gs_29827;
        // D s_14_1: write-var match_vmid <= s_14_0
        fn_state.match_vmid = s_14_0;
        // D s_14_2: read-var dbgtype:u8
        let s_14_2: u8 = fn_state.dbgtype;
        // C s_14_3: const #1s : i
        let s_14_3: i128 = 1;
        // D s_14_4: cast zx s_14_2 -> bv
        let s_14_4: Bits = Bits::new(s_14_2 as u128, 4u16);
        // C s_14_5: const #1s : i64
        let s_14_5: i64 = 1;
        // C s_14_6: cast zx s_14_5 -> i
        let s_14_6: i128 = (i128::try_from(s_14_5).unwrap());
        // C s_14_7: const #0s : i
        let s_14_7: i128 = 0;
        // C s_14_8: add s_14_7 s_14_6
        let s_14_8: i128 = (s_14_7 + s_14_6);
        // D s_14_9: bit-extract s_14_4 s_14_3 s_14_8
        let s_14_9: Bits = (Bits::new(
            ((s_14_4) >> (s_14_3)).value(),
            u16::try_from(s_14_8).unwrap(),
        ));
        // D s_14_10: cast reint s_14_9 -> u8
        let s_14_10: bool = ((s_14_9.value()) != 0);
        // D s_14_11: cast zx s_14_10 -> bv
        let s_14_11: Bits = Bits::new(s_14_10 as u128, 1u16);
        // C s_14_12: const #1u : u8
        let s_14_12: bool = true;
        // C s_14_13: cast zx s_14_12 -> bv
        let s_14_13: Bits = Bits::new(s_14_12 as u128, 1u16);
        // D s_14_14: cmp-eq s_14_11 s_14_13
        let s_14_14: bool = ((s_14_11) == (s_14_13));
        // D s_14_15: not s_14_14
        let s_14_15: bool = !s_14_14;
        // N s_14_16: branch s_14_15 b106 b15
        if s_14_15 {
            return block_106(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#29832 <= s_15_0
        fn_state.gs_29832 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_16_0: read-var gs#29832:u8
        let s_16_0: bool = fn_state.gs_29832;
        // D s_16_1: write-var match_cid1 <= s_16_0
        fn_state.match_cid1 = s_16_0;
        // D s_16_2: read-var dbgtype:u8
        let s_16_2: u8 = fn_state.dbgtype;
        // C s_16_3: const #2s : i
        let s_16_3: i128 = 2;
        // D s_16_4: cast zx s_16_2 -> bv
        let s_16_4: Bits = Bits::new(s_16_2 as u128, 4u16);
        // C s_16_5: const #1s : i64
        let s_16_5: i64 = 1;
        // C s_16_6: cast zx s_16_5 -> i
        let s_16_6: i128 = (i128::try_from(s_16_5).unwrap());
        // C s_16_7: const #1s : i
        let s_16_7: i128 = 1;
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
        let s_16_11: Bits = Bits::new(s_16_10 as u128, 2u16);
        // C s_16_12: const #3u : u8
        let s_16_12: u8 = 3;
        // C s_16_13: cast zx s_16_12 -> bv
        let s_16_13: Bits = Bits::new(s_16_12 as u128, 2u16);
        // D s_16_14: cmp-eq s_16_11 s_16_13
        let s_16_14: bool = ((s_16_11) == (s_16_13));
        // D s_16_15: not s_16_14
        let s_16_15: bool = !s_16_14;
        // N s_16_16: branch s_16_15 b105 b17
        if s_16_15 {
            return block_105(state, tracer, fn_state);
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
        // D s_17_1: write-var gs#29837 <= s_17_0
        fn_state.gs_29837 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_18_0: read-var gs#29837:u8
        let s_18_0: bool = fn_state.gs_29837;
        // D s_18_1: write-var match_cid2 <= s_18_0
        fn_state.match_cid2 = s_18_0;
        // D s_18_2: read-var dbgtype:u8
        let s_18_2: u8 = fn_state.dbgtype;
        // C s_18_3: const #0s : i
        let s_18_3: i128 = 0;
        // D s_18_4: cast zx s_18_2 -> bv
        let s_18_4: Bits = Bits::new(s_18_2 as u128, 4u16);
        // C s_18_5: const #1s : i64
        let s_18_5: i64 = 1;
        // C s_18_6: cast zx s_18_5 -> i
        let s_18_6: i128 = (i128::try_from(s_18_5).unwrap());
        // C s_18_7: const #0s : i
        let s_18_7: i128 = 0;
        // C s_18_8: add s_18_7 s_18_6
        let s_18_8: i128 = (s_18_7 + s_18_6);
        // D s_18_9: bit-extract s_18_4 s_18_3 s_18_8
        let s_18_9: Bits = (Bits::new(
            ((s_18_4) >> (s_18_3)).value(),
            u16::try_from(s_18_8).unwrap(),
        ));
        // D s_18_10: cast reint s_18_9 -> u8
        let s_18_10: bool = ((s_18_9.value()) != 0);
        // D s_18_11: cast zx s_18_10 -> bv
        let s_18_11: Bits = Bits::new(s_18_10 as u128, 1u16);
        // C s_18_12: const #1u : u8
        let s_18_12: bool = true;
        // C s_18_13: cast zx s_18_12 -> bv
        let s_18_13: Bits = Bits::new(s_18_12 as u128, 1u16);
        // D s_18_14: cmp-eq s_18_11 s_18_13
        let s_18_14: bool = ((s_18_11) == (s_18_13));
        // D s_18_15: not s_18_14
        let s_18_15: bool = !s_18_14;
        // N s_18_16: branch s_18_15 b104 b19
        if s_18_15 {
            return block_104(state, tracer, fn_state);
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
        // D s_19_1: write-var gs#29842 <= s_19_0
        fn_state.gs_29842 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_20_0: read-var gs#29842:u8
        let s_20_0: bool = fn_state.gs_29842;
        // D s_20_1: write-var linking_enabled <= s_20_0
        fn_state.linking_enabled = s_20_0;
        // D s_20_2: read-var linked_to:u8
        let s_20_2: bool = fn_state.linked_to;
        // N s_20_3: branch s_20_2 b103 b21
        if s_20_2 {
            return block_103(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var gs#29847 <= s_21_0
        fn_state.gs_29847 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_22_0: read-var gs#29847:u8
        let s_22_0: bool = fn_state.gs_29847;
        // N s_22_1: branch s_22_0 b100 b23
        if s_22_0 {
            return block_100(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // N s_23_0: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_24_0: read-var linked_to:u8
        let s_24_0: bool = fn_state.linked_to;
        // D s_24_1: not s_24_0
        let s_24_1: bool = !s_24_0;
        // N s_24_2: branch s_24_1 b99 b25
        if s_24_1 {
            return block_99(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // D s_25_1: write-var gs#29848 <= s_25_0
        fn_state.gs_29848 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_26_0: read-var gs#29848:u8
        let s_26_0: bool = fn_state.gs_29848;
        // N s_26_1: branch s_26_0 b98 b27
        if s_26_0 {
            return block_98(state, tracer, fn_state);
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
        // D s_27_1: write-var gs#29849 <= s_27_0
        fn_state.gs_29849 = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_28_0: read-var gs#29849:u8
        let s_28_0: bool = fn_state.gs_29849;
        // N s_28_1: branch s_28_0 b97 b29
        if s_28_0 {
            return block_97(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_29_0: const #0u : u8
        let s_29_0: bool = false;
        // D s_29_1: write-var bvr_match <= s_29_0
        fn_state.bvr_match = s_29_0;
        // C s_29_2: const #0u : u8
        let s_29_2: bool = false;
        // D s_29_3: write-var bxvr_match <= s_29_2
        fn_state.bxvr_match = s_29_2;
        // D s_29_4: read-var match_addr:u8
        let s_29_4: bool = fn_state.match_addr;
        // N s_29_5: branch s_29_4 b90 b30
        if s_29_4 {
            return block_90(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_30_0: read-var match_cid1:u8
        let s_30_0: bool = fn_state.match_cid1;
        // N s_30_1: branch s_30_0 b86 b31
        if s_30_0 {
            return block_86(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // N s_31_0: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_32_0: read-var match_vmid:u8
        let s_32_0: bool = fn_state.match_vmid;
        // N s_32_1: branch s_32_0 b68 b33
        if s_32_0 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_33_0: read-var match_cid2:u8
        let s_33_0: bool = fn_state.match_cid2;
        // N s_33_1: branch s_33_0 b58 b34
        if s_33_0 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // N s_34_0: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_35_0: read-var match_addr:u8
        let s_35_0: bool = fn_state.match_addr;
        // N s_35_1: branch s_35_0 b57 b36
        if s_35_0 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_36_0: read-var match_cid1:u8
        let s_36_0: bool = fn_state.match_cid1;
        // D s_36_1: write-var gs#29904 <= s_36_0
        fn_state.gs_29904 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_37_0: read-var gs#29904:u8
        let s_37_0: bool = fn_state.gs_29904;
        // D s_37_1: write-var bvr_match_valid <= s_37_0
        fn_state.bvr_match_valid = s_37_0;
        // D s_37_2: read-var match_vmid:u8
        let s_37_2: bool = fn_state.match_vmid;
        // N s_37_3: branch s_37_2 b56 b38
        if s_37_2 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_38_0: read-var match_cid2:u8
        let s_38_0: bool = fn_state.match_cid2;
        // D s_38_1: write-var gs#29905 <= s_38_0
        fn_state.gs_29905 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_39_0: read-var gs#29905:u8
        let s_39_0: bool = fn_state.gs_29905;
        // D s_39_1: not s_39_0
        let s_39_1: bool = !s_39_0;
        // N s_39_2: branch s_39_1 b55 b40
        if s_39_1 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_40_0: read-var bxvr_match:u8
        let s_40_0: bool = fn_state.bxvr_match;
        // D s_40_1: write-var gs#29906 <= s_40_0
        fn_state.gs_29906 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_41_0: read-var gs#29906:u8
        let s_41_0: bool = fn_state.gs_29906;
        // N s_41_1: branch s_41_0 b51 b42
        if s_41_0 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_42_0: const #0u : u8
        let s_42_0: bool = false;
        // D s_42_1: write-var gs#29908 <= s_42_0
        fn_state.gs_29908 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_43_0: read-var gs#29908:u8
        let s_43_0: bool = fn_state.gs_29908;
        // D s_43_1: write-var val_match <= s_43_0
        fn_state.val_match = s_43_0;
        // D s_43_2: read-var val_match:u8
        let s_43_2: bool = fn_state.val_match;
        // N s_43_3: branch s_43_2 b50 b44
        if s_43_2 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_44_0: const #0u : u8
        let s_44_0: bool = false;
        // D s_44_1: write-var gs#29909 <= s_44_0
        fn_state.gs_29909 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_45_0: read-var gs#29909:u8
        let s_45_0: bool = fn_state.gs_29909;
        // D s_45_1: write-var ga#23130 <= s_45_0
        fn_state.ga_23130 = s_45_0;
        // D s_45_2: read-var val_match:u8
        let s_45_2: bool = fn_state.val_match;
        // D s_45_3: not s_45_2
        let s_45_3: bool = !s_45_2;
        // N s_45_4: branch s_45_3 b49 b46
        if s_45_3 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_46_0: const #0u : u8
        let s_46_0: bool = false;
        // D s_46_1: write-var gs#29910 <= s_46_0
        fn_state.gs_29910 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_47_0: read-var gs#29910:u8
        let s_47_0: bool = fn_state.gs_29910;
        // D s_47_1: read-var ga#23130:u8
        let s_47_1: bool = fn_state.ga_23130;
        // D s_47_2: create-product struct = ["s_47_1", "s_47_0"]
        let s_47_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_47_1,
            _1: s_47_0,
        };
        // D s_47_3: write-var return_value <= s_47_2
        fn_state.return_value = s_47_2;
        // N s_47_4: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_48_0: read-var return_value:struct
        let s_48_0: ProductType8b847afc727d5818 = fn_state.return_value;
        // N s_48_1: return s_48_0
        return s_48_0;
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_49_0: read-var mismatch:u8
        let s_49_0: bool = fn_state.mismatch;
        // D s_49_1: write-var gs#29910 <= s_49_0
        fn_state.gs_29910 = s_49_0;
        // N s_49_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_50_0: read-var mismatch:u8
        let s_50_0: bool = fn_state.mismatch;
        // D s_50_1: not s_50_0
        let s_50_1: bool = !s_50_0;
        // D s_50_2: write-var gs#29909 <= s_50_1
        fn_state.gs_29909 = s_50_1;
        // N s_50_3: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_51_0: read-var bvr_match_valid:u8
        let s_51_0: bool = fn_state.bvr_match_valid;
        // D s_51_1: not s_51_0
        let s_51_1: bool = !s_51_0;
        // N s_51_2: branch s_51_1 b54 b52
        if s_51_1 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_52_0: read-var bvr_match:u8
        let s_52_0: bool = fn_state.bvr_match;
        // D s_52_1: write-var gs#29907 <= s_52_0
        fn_state.gs_29907 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_53_0: read-var gs#29907:u8
        let s_53_0: bool = fn_state.gs_29907;
        // D s_53_1: write-var gs#29908 <= s_53_0
        fn_state.gs_29908 = s_53_0;
        // N s_53_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_54_0: const #1u : u8
        let s_54_0: bool = true;
        // D s_54_1: write-var gs#29907 <= s_54_0
        fn_state.gs_29907 = s_54_0;
        // N s_54_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_55_0: const #1u : u8
        let s_55_0: bool = true;
        // D s_55_1: write-var gs#29906 <= s_55_0
        fn_state.gs_29906 = s_55_0;
        // N s_55_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_56_0: const #1u : u8
        let s_56_0: bool = true;
        // D s_56_1: write-var gs#29905 <= s_56_0
        fn_state.gs_29905 = s_56_0;
        // N s_56_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_57_0: const #1u : u8
        let s_57_0: bool = true;
        // D s_57_1: write-var gs#29904 <= s_57_0
        fn_state.gs_29904 = s_57_0;
        // N s_57_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_58_0: const #16975u : u32
        let s_58_0: u32 = 16975;
        // D s_58_1: read-reg s_58_0:u8
        let s_58_1: u8 = {
            let value = state.read_register::<u8>(s_58_0 as isize);
            tracer.read_register(s_58_0 as isize, value);
            value
        };
        // D s_58_2: cast zx s_58_1 -> bv
        let s_58_2: Bits = Bits::new(s_58_1 as u128, 2u16);
        // C s_58_3: const #424u : u32
        let s_58_3: u32 = 424;
        // D s_58_4: read-reg s_58_3:u8
        let s_58_4: u8 = {
            let value = state.read_register::<u8>(s_58_3 as isize);
            tracer.read_register(s_58_3 as isize, value);
            value
        };
        // D s_58_5: cast zx s_58_4 -> bv
        let s_58_5: Bits = Bits::new(s_58_4 as u128, 2u16);
        // D s_58_6: cmp-ne s_58_2 s_58_5
        let s_58_6: bool = ((s_58_2) != (s_58_5));
        // N s_58_7: branch s_58_6 b67 b59
        if s_58_6 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_59_0: const #0u : u8
        let s_59_0: bool = false;
        // D s_59_1: write-var gs#29856 <= s_59_0
        fn_state.gs_29856 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_60_0: read-var gs#29856:u8
        let s_60_0: bool = fn_state.gs_29856;
        // N s_60_1: branch s_60_0 b66 b61
        if s_60_0 {
            return block_66(state, tracer, fn_state);
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
        // D s_61_1: write-var gs#29857 <= s_61_0
        fn_state.gs_29857 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_62_0: read-var gs#29857:u8
        let s_62_0: bool = fn_state.gs_29857;
        // N s_62_1: branch s_62_0 b65 b63
        if s_62_0 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_63_0: const #0u : u8
        let s_63_0: bool = false;
        // D s_63_1: write-var gs#29862 <= s_63_0
        fn_state.gs_29862 = s_63_0;
        // N s_63_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_64_0: read-var gs#29862:u8
        let s_64_0: bool = fn_state.gs_29862;
        // D s_64_1: write-var bxvr_match <= s_64_0
        fn_state.bxvr_match = s_64_0;
        // N s_64_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_65_0: read-var nshadow#544:i
        let s_65_0: i128 = fn_state.nshadow_544;
        // D s_65_1: cast reint s_65_0 -> i64
        let s_65_1: i64 = (s_65_0 as i64);
        // D s_65_2: call DBGBXVR_read(s_65_1)
        let s_65_2: ProductType700c18a878c5601b = DBGBXVR_read(state, tracer, s_65_1);
        // D s_65_3: write-var ga#23123 <= s_65_2
        fn_state.ga_23123 = s_65_2;
        // D s_65_4: read-var ga#23123.0:struct
        let s_65_4: u32 = fn_state.ga_23123._0;
        // C s_65_5: const #0s : i
        let s_65_5: i128 = 0;
        // D s_65_6: cast zx s_65_4 -> bv
        let s_65_6: Bits = Bits::new(s_65_4 as u128, 32u16);
        // C s_65_7: const #1s : i64
        let s_65_7: i64 = 1;
        // C s_65_8: cast zx s_65_7 -> i
        let s_65_8: i128 = (i128::try_from(s_65_7).unwrap());
        // C s_65_9: const #31s : i
        let s_65_9: i128 = 31;
        // C s_65_10: add s_65_9 s_65_8
        let s_65_10: i128 = (s_65_9 + s_65_8);
        // D s_65_11: bit-extract s_65_6 s_65_5 s_65_10
        let s_65_11: Bits = (Bits::new(
            ((s_65_6) >> (s_65_5)).value(),
            u16::try_from(s_65_10).unwrap(),
        ));
        // D s_65_12: cast reint s_65_11 -> u32
        let s_65_12: u32 = (s_65_11.value() as u32);
        // C s_65_13: const #91008u : u32
        let s_65_13: u32 = 91008;
        // D s_65_14: read-reg s_65_13:u64
        let s_65_14: u64 = {
            let value = state.read_register::<u64>(s_65_13 as isize);
            tracer.read_register(s_65_13 as isize, value);
            value
        };
        // C s_65_15: const #0s : i
        let s_65_15: i128 = 0;
        // D s_65_16: cast zx s_65_14 -> bv
        let s_65_16: Bits = Bits::new(s_65_14 as u128, 64u16);
        // C s_65_17: const #1s : i64
        let s_65_17: i64 = 1;
        // C s_65_18: cast zx s_65_17 -> i
        let s_65_18: i128 = (i128::try_from(s_65_17).unwrap());
        // C s_65_19: const #31s : i
        let s_65_19: i128 = 31;
        // C s_65_20: add s_65_19 s_65_18
        let s_65_20: i128 = (s_65_19 + s_65_18);
        // D s_65_21: bit-extract s_65_16 s_65_15 s_65_20
        let s_65_21: Bits = (Bits::new(
            ((s_65_16) >> (s_65_15)).value(),
            u16::try_from(s_65_20).unwrap(),
        ));
        // D s_65_22: cast reint s_65_21 -> u32
        let s_65_22: u32 = (s_65_21.value() as u32);
        // D s_65_23: cast zx s_65_12 -> bv
        let s_65_23: Bits = Bits::new(s_65_12 as u128, 32u16);
        // D s_65_24: cast zx s_65_22 -> bv
        let s_65_24: Bits = Bits::new(s_65_22 as u128, 32u16);
        // D s_65_25: cmp-eq s_65_23 s_65_24
        let s_65_25: bool = ((s_65_23) == (s_65_24));
        // D s_65_26: write-var gs#29862 <= s_65_25
        fn_state.gs_29862 = s_65_25;
        // N s_65_27: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_66_0: const #432u : u32
        let s_66_0: u32 = 432;
        // D s_66_1: read-reg s_66_0:u8
        let s_66_1: u8 = {
            let value = state.read_register::<u8>(s_66_0 as isize);
            tracer.read_register(s_66_0 as isize, value);
            value
        };
        // D s_66_2: call ELUsingAArch32(s_66_1)
        let s_66_2: bool = ELUsingAArch32(state, tracer, s_66_1);
        // D s_66_3: not s_66_2
        let s_66_3: bool = !s_66_2;
        // D s_66_4: write-var gs#29857 <= s_66_3
        fn_state.gs_29857 = s_66_3;
        // N s_66_5: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_67_0: const #() : ()
        let s_67_0: () = ();
        // S s_67_1: call EL2Enabled(s_67_0)
        let s_67_1: bool = EL2Enabled(state, tracer, s_67_0);
        // D s_67_2: write-var gs#29856 <= s_67_1
        fn_state.gs_29856 = s_67_1;
        // N s_67_3: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_68_0: const #432u : u32
        let s_68_0: u32 = 432;
        // D s_68_1: read-reg s_68_0:u8
        let s_68_1: u8 = {
            let value = state.read_register::<u8>(s_68_0 as isize);
            tracer.read_register(s_68_0 as isize, value);
            value
        };
        // D s_68_2: call ELUsingAArch32(s_68_1)
        let s_68_2: bool = ELUsingAArch32(state, tracer, s_68_1);
        // N s_68_3: branch s_68_2 b85 b69
        if s_68_2 {
            return block_85(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_69_0: const #() : ()
        let s_69_0: () = ();
        // S s_69_1: call Have16bitVMID(s_69_0)
        let s_69_1: bool = Have16bitVMID(state, tracer, s_69_0);
        // S s_69_2: not s_69_1
        let s_69_2: bool = !s_69_1;
        // N s_69_3: branch s_69_2 b84 b70
        if s_69_2 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_70(state, tracer, fn_state);
        };
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_70_0: const #15328u : u32
        let s_70_0: u32 = 15328;
        // D s_70_1: read-reg s_70_0:struct
        let s_70_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_70_0 as isize);
            tracer.read_register(s_70_0 as isize, value);
            value
        };
        // D s_70_2: call _get_VTCR_EL2_Type_VS(s_70_1)
        let s_70_2: bool = u_get_VTCR_EL2_Type_VS(state, tracer, s_70_1);
        // D s_70_3: cast zx s_70_2 -> bv
        let s_70_3: Bits = Bits::new(s_70_2 as u128, 1u16);
        // C s_70_4: const #0u : u8
        let s_70_4: bool = false;
        // C s_70_5: cast zx s_70_4 -> bv
        let s_70_5: Bits = Bits::new(s_70_4 as u128, 1u16);
        // D s_70_6: cmp-eq s_70_3 s_70_5
        let s_70_6: bool = ((s_70_3) == (s_70_5));
        // D s_70_7: write-var gs#29864 <= s_70_6
        fn_state.gs_29864 = s_70_6;
        // N s_70_8: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_71_0: read-var gs#29864:u8
        let s_71_0: bool = fn_state.gs_29864;
        // N s_71_1: branch s_71_0 b83 b72
        if s_71_0 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_72(state, tracer, fn_state);
        };
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_72_0: const #() : ()
        let s_72_0: () = ();
        // S s_72_1: call VTTBR_EL2_read(s_72_0)
        let s_72_1: ProductType782ac6922b48c20d = VTTBR_EL2_read(state, tracer, s_72_0);
        // S s_72_2: call _get_VTTBR_EL2_Type_VMID(s_72_1)
        let s_72_2: u16 = u_get_VTTBR_EL2_Type_VMID(state, tracer, s_72_1);
        // D s_72_3: write-var vmid <= s_72_2
        fn_state.vmid = s_72_2;
        // D s_72_4: read-var nshadow#544:i
        let s_72_4: i128 = fn_state.nshadow_544;
        // D s_72_5: cast reint s_72_4 -> i64
        let s_72_5: i64 = (s_72_4 as i64);
        // D s_72_6: call DBGBXVR_read(s_72_5)
        let s_72_6: ProductType700c18a878c5601b = DBGBXVR_read(state, tracer, s_72_5);
        // D s_72_7: write-var ga#23112 <= s_72_6
        fn_state.ga_23112 = s_72_6;
        // D s_72_8: read-var ga#23112.0:struct
        let s_72_8: u32 = fn_state.ga_23112._0;
        // C s_72_9: const #0s : i
        let s_72_9: i128 = 0;
        // D s_72_10: cast zx s_72_8 -> bv
        let s_72_10: Bits = Bits::new(s_72_8 as u128, 32u16);
        // C s_72_11: const #1s : i64
        let s_72_11: i64 = 1;
        // C s_72_12: cast zx s_72_11 -> i
        let s_72_12: i128 = (i128::try_from(s_72_11).unwrap());
        // C s_72_13: const #15s : i
        let s_72_13: i128 = 15;
        // C s_72_14: add s_72_13 s_72_12
        let s_72_14: i128 = (s_72_13 + s_72_12);
        // D s_72_15: bit-extract s_72_10 s_72_9 s_72_14
        let s_72_15: Bits = (Bits::new(
            ((s_72_10) >> (s_72_9)).value(),
            u16::try_from(s_72_14).unwrap(),
        ));
        // D s_72_16: cast reint s_72_15 -> u16
        let s_72_16: u16 = (s_72_15.value() as u16);
        // D s_72_17: write-var bvr_vmid <= s_72_16
        fn_state.bvr_vmid = s_72_16;
        // N s_72_18: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_73_0: const #16975u : u32
        let s_73_0: u32 = 16975;
        // D s_73_1: read-reg s_73_0:u8
        let s_73_1: u8 = {
            let value = state.read_register::<u8>(s_73_0 as isize);
            tracer.read_register(s_73_0 as isize, value);
            value
        };
        // D s_73_2: cast zx s_73_1 -> bv
        let s_73_2: Bits = Bits::new(s_73_1 as u128, 2u16);
        // C s_73_3: const #448u : u32
        let s_73_3: u32 = 448;
        // D s_73_4: read-reg s_73_3:u8
        let s_73_4: u8 = {
            let value = state.read_register::<u8>(s_73_3 as isize);
            tracer.read_register(s_73_3 as isize, value);
            value
        };
        // D s_73_5: cast zx s_73_4 -> bv
        let s_73_5: Bits = Bits::new(s_73_4 as u128, 2u16);
        // D s_73_6: cmp-eq s_73_2 s_73_5
        let s_73_6: bool = ((s_73_2) == (s_73_5));
        // N s_73_7: branch s_73_6 b82 b74
        if s_73_6 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_74(state, tracer, fn_state);
        };
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
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
        // C s_74_3: const #440u : u32
        let s_74_3: u32 = 440;
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
        // D s_74_7: write-var gs#29880 <= s_74_6
        fn_state.gs_29880 = s_74_6;
        // N s_74_8: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_75_0: read-var gs#29880:u8
        let s_75_0: bool = fn_state.gs_29880;
        // N s_75_1: branch s_75_0 b81 b76
        if s_75_0 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_76(state, tracer, fn_state);
        };
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_76_0: const #0u : u8
        let s_76_0: bool = false;
        // D s_76_1: write-var gs#29881 <= s_76_0
        fn_state.gs_29881 = s_76_0;
        // N s_76_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_77_0: read-var gs#29881:u8
        let s_77_0: bool = fn_state.gs_29881;
        // N s_77_1: branch s_77_0 b80 b78
        if s_77_0 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_78(state, tracer, fn_state);
        };
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_78_0: const #0u : u8
        let s_78_0: bool = false;
        // D s_78_1: write-var gs#29882 <= s_78_0
        fn_state.gs_29882 = s_78_0;
        // N s_78_2: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_79_0: read-var gs#29882:u8
        let s_79_0: bool = fn_state.gs_29882;
        // D s_79_1: write-var bxvr_match <= s_79_0
        fn_state.bxvr_match = s_79_0;
        // N s_79_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_80_0: read-var vmid:u16
        let s_80_0: u16 = fn_state.vmid;
        // D s_80_1: cast zx s_80_0 -> bv
        let s_80_1: Bits = Bits::new(s_80_0 as u128, 16u16);
        // D s_80_2: read-var bvr_vmid:u16
        let s_80_2: u16 = fn_state.bvr_vmid;
        // D s_80_3: cast zx s_80_2 -> bv
        let s_80_3: Bits = Bits::new(s_80_2 as u128, 16u16);
        // D s_80_4: cmp-eq s_80_1 s_80_3
        let s_80_4: bool = ((s_80_1) == (s_80_3));
        // D s_80_5: write-var gs#29882 <= s_80_4
        fn_state.gs_29882 = s_80_4;
        // N s_80_6: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_81_0: const #() : ()
        let s_81_0: () = ();
        // S s_81_1: call EL2Enabled(s_81_0)
        let s_81_1: bool = EL2Enabled(state, tracer, s_81_0);
        // D s_81_2: write-var gs#29881 <= s_81_1
        fn_state.gs_29881 = s_81_1;
        // N s_81_3: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_82_0: const #1u : u8
        let s_82_0: bool = true;
        // D s_82_1: write-var gs#29880 <= s_82_0
        fn_state.gs_29880 = s_82_0;
        // N s_82_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_83_0: const #() : ()
        let s_83_0: () = ();
        // S s_83_1: call VTTBR_EL2_read(s_83_0)
        let s_83_1: ProductType782ac6922b48c20d = VTTBR_EL2_read(state, tracer, s_83_0);
        // S s_83_2: call _get_VTTBR_EL2_Type_VMID(s_83_1)
        let s_83_2: u16 = u_get_VTTBR_EL2_Type_VMID(state, tracer, s_83_1);
        // C s_83_3: const #0s : i
        let s_83_3: i128 = 0;
        // S s_83_4: cast zx s_83_2 -> bv
        let s_83_4: Bits = Bits::new(s_83_2 as u128, 16u16);
        // C s_83_5: const #1s : i64
        let s_83_5: i64 = 1;
        // C s_83_6: cast zx s_83_5 -> i
        let s_83_6: i128 = (i128::try_from(s_83_5).unwrap());
        // C s_83_7: const #7s : i
        let s_83_7: i128 = 7;
        // C s_83_8: add s_83_7 s_83_6
        let s_83_8: i128 = (s_83_7 + s_83_6);
        // D s_83_9: bit-extract s_83_4 s_83_3 s_83_8
        let s_83_9: Bits = (Bits::new(
            ((s_83_4) >> (s_83_3)).value(),
            u16::try_from(s_83_8).unwrap(),
        ));
        // D s_83_10: cast reint s_83_9 -> u8
        let s_83_10: u8 = (s_83_9.value() as u8);
        // C s_83_11: const #16s : i
        let s_83_11: i128 = 16;
        // D s_83_12: cast zx s_83_10 -> bv
        let s_83_12: Bits = Bits::new(s_83_10 as u128, 8u16);
        // D s_83_13: bits-cast zx s_83_12 -> bv length s_83_11
        let s_83_13: Bits = s_83_12.zero_extend(s_83_11);
        // D s_83_14: cast reint s_83_13 -> u16
        let s_83_14: u16 = (s_83_13.value() as u16);
        // D s_83_15: write-var vmid <= s_83_14
        fn_state.vmid = s_83_14;
        // D s_83_16: read-var nshadow#544:i
        let s_83_16: i128 = fn_state.nshadow_544;
        // D s_83_17: cast reint s_83_16 -> i64
        let s_83_17: i64 = (s_83_16 as i64);
        // D s_83_18: call DBGBXVR_read(s_83_17)
        let s_83_18: ProductType700c18a878c5601b = DBGBXVR_read(state, tracer, s_83_17);
        // D s_83_19: write-var ga#23108 <= s_83_18
        fn_state.ga_23108 = s_83_18;
        // D s_83_20: read-var ga#23108.0:struct
        let s_83_20: u32 = fn_state.ga_23108._0;
        // C s_83_21: const #0s : i
        let s_83_21: i128 = 0;
        // D s_83_22: cast zx s_83_20 -> bv
        let s_83_22: Bits = Bits::new(s_83_20 as u128, 32u16);
        // C s_83_23: const #1s : i64
        let s_83_23: i64 = 1;
        // C s_83_24: cast zx s_83_23 -> i
        let s_83_24: i128 = (i128::try_from(s_83_23).unwrap());
        // C s_83_25: const #7s : i
        let s_83_25: i128 = 7;
        // C s_83_26: add s_83_25 s_83_24
        let s_83_26: i128 = (s_83_25 + s_83_24);
        // D s_83_27: bit-extract s_83_22 s_83_21 s_83_26
        let s_83_27: Bits = (Bits::new(
            ((s_83_22) >> (s_83_21)).value(),
            u16::try_from(s_83_26).unwrap(),
        ));
        // D s_83_28: cast reint s_83_27 -> u8
        let s_83_28: u8 = (s_83_27.value() as u8);
        // C s_83_29: const #16s : i
        let s_83_29: i128 = 16;
        // D s_83_30: cast zx s_83_28 -> bv
        let s_83_30: Bits = Bits::new(s_83_28 as u128, 8u16);
        // D s_83_31: bits-cast zx s_83_30 -> bv length s_83_29
        let s_83_31: Bits = s_83_30.zero_extend(s_83_29);
        // D s_83_32: cast reint s_83_31 -> u16
        let s_83_32: u16 = (s_83_31.value() as u16);
        // D s_83_33: write-var bvr_vmid <= s_83_32
        fn_state.bvr_vmid = s_83_32;
        // N s_83_34: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_84_0: const #1u : u8
        let s_84_0: bool = true;
        // D s_84_1: write-var gs#29864 <= s_84_0
        fn_state.gs_29864 = s_84_0;
        // N s_84_2: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_85_0: const #22408u : u32
        let s_85_0: u32 = 22408;
        // D s_85_1: read-reg s_85_0:struct
        let s_85_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_85_0 as isize);
            tracer.read_register(s_85_0 as isize, value);
            value
        };
        // D s_85_2: call _get_VTTBR_Type_VMID(s_85_1)
        let s_85_2: u8 = u_get_VTTBR_Type_VMID(state, tracer, s_85_1);
        // C s_85_3: const #16s : i
        let s_85_3: i128 = 16;
        // D s_85_4: cast zx s_85_2 -> bv
        let s_85_4: Bits = Bits::new(s_85_2 as u128, 8u16);
        // D s_85_5: bits-cast zx s_85_4 -> bv length s_85_3
        let s_85_5: Bits = s_85_4.zero_extend(s_85_3);
        // D s_85_6: cast reint s_85_5 -> u16
        let s_85_6: u16 = (s_85_5.value() as u16);
        // D s_85_7: write-var vmid <= s_85_6
        fn_state.vmid = s_85_6;
        // D s_85_8: read-var nshadow#544:i
        let s_85_8: i128 = fn_state.nshadow_544;
        // D s_85_9: cast reint s_85_8 -> i64
        let s_85_9: i64 = (s_85_8 as i64);
        // D s_85_10: call DBGBXVR_read(s_85_9)
        let s_85_10: ProductType700c18a878c5601b = DBGBXVR_read(state, tracer, s_85_9);
        // D s_85_11: write-var ga#23098 <= s_85_10
        fn_state.ga_23098 = s_85_10;
        // D s_85_12: read-var ga#23098.0:struct
        let s_85_12: u32 = fn_state.ga_23098._0;
        // C s_85_13: const #0s : i
        let s_85_13: i128 = 0;
        // D s_85_14: cast zx s_85_12 -> bv
        let s_85_14: Bits = Bits::new(s_85_12 as u128, 32u16);
        // C s_85_15: const #1s : i64
        let s_85_15: i64 = 1;
        // C s_85_16: cast zx s_85_15 -> i
        let s_85_16: i128 = (i128::try_from(s_85_15).unwrap());
        // C s_85_17: const #7s : i
        let s_85_17: i128 = 7;
        // C s_85_18: add s_85_17 s_85_16
        let s_85_18: i128 = (s_85_17 + s_85_16);
        // D s_85_19: bit-extract s_85_14 s_85_13 s_85_18
        let s_85_19: Bits = (Bits::new(
            ((s_85_14) >> (s_85_13)).value(),
            u16::try_from(s_85_18).unwrap(),
        ));
        // D s_85_20: cast reint s_85_19 -> u8
        let s_85_20: u8 = (s_85_19.value() as u8);
        // C s_85_21: const #16s : i
        let s_85_21: i128 = 16;
        // D s_85_22: cast zx s_85_20 -> bv
        let s_85_22: Bits = Bits::new(s_85_20 as u128, 8u16);
        // D s_85_23: bits-cast zx s_85_22 -> bv length s_85_21
        let s_85_23: Bits = s_85_22.zero_extend(s_85_21);
        // D s_85_24: cast reint s_85_23 -> u16
        let s_85_24: u16 = (s_85_23.value() as u16);
        // D s_85_25: write-var bvr_vmid <= s_85_24
        fn_state.bvr_vmid = s_85_24;
        // N s_85_26: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_86_0: const #16975u : u32
        let s_86_0: u32 = 16975;
        // D s_86_1: read-reg s_86_0:u8
        let s_86_1: u8 = {
            let value = state.read_register::<u8>(s_86_0 as isize);
            tracer.read_register(s_86_0 as isize, value);
            value
        };
        // D s_86_2: cast zx s_86_1 -> bv
        let s_86_2: Bits = Bits::new(s_86_1 as u128, 2u16);
        // C s_86_3: const #432u : u32
        let s_86_3: u32 = 432;
        // D s_86_4: read-reg s_86_3:u8
        let s_86_4: u8 = {
            let value = state.read_register::<u8>(s_86_3 as isize);
            tracer.read_register(s_86_3 as isize, value);
            value
        };
        // D s_86_5: cast zx s_86_4 -> bv
        let s_86_5: Bits = Bits::new(s_86_4 as u128, 2u16);
        // D s_86_6: cmp-ne s_86_2 s_86_5
        let s_86_6: bool = ((s_86_2) != (s_86_5));
        // N s_86_7: branch s_86_6 b89 b87
        if s_86_6 {
            return block_89(state, tracer, fn_state);
        } else {
            return block_87(state, tracer, fn_state);
        };
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_87_0: const #0u : u8
        let s_87_0: bool = false;
        // D s_87_1: write-var gs#29886 <= s_87_0
        fn_state.gs_29886 = s_87_0;
        // N s_87_2: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_88_0: read-var gs#29886:u8
        let s_88_0: bool = fn_state.gs_29886;
        // D s_88_1: write-var bvr_match <= s_88_0
        fn_state.bvr_match = s_88_0;
        // N s_88_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_89_0: const #() : ()
        let s_89_0: () = ();
        // S s_89_1: call CONTEXTIDR_read(s_89_0)
        let s_89_1: ProductType700c18a878c5601b = CONTEXTIDR_read(state, tracer, s_89_0);
        // D s_89_2: write-var ga#23090 <= s_89_1
        fn_state.ga_23090 = s_89_1;
        // D s_89_3: read-var ga#23090.0:struct
        let s_89_3: u32 = fn_state.ga_23090._0;
        // D s_89_4: read-var nshadow#544:i
        let s_89_4: i128 = fn_state.nshadow_544;
        // D s_89_5: cast reint s_89_4 -> i64
        let s_89_5: i64 = (s_89_4 as i64);
        // D s_89_6: call DBGBVR_read(s_89_5)
        let s_89_6: ProductType700c18a878c5601b = DBGBVR_read(state, tracer, s_89_5);
        // D s_89_7: write-var ga#23091 <= s_89_6
        fn_state.ga_23091 = s_89_6;
        // D s_89_8: read-var ga#23091.0:struct
        let s_89_8: u32 = fn_state.ga_23091._0;
        // C s_89_9: const #0s : i
        let s_89_9: i128 = 0;
        // D s_89_10: cast zx s_89_8 -> bv
        let s_89_10: Bits = Bits::new(s_89_8 as u128, 32u16);
        // C s_89_11: const #1s : i64
        let s_89_11: i64 = 1;
        // C s_89_12: cast zx s_89_11 -> i
        let s_89_12: i128 = (i128::try_from(s_89_11).unwrap());
        // C s_89_13: const #31s : i
        let s_89_13: i128 = 31;
        // C s_89_14: add s_89_13 s_89_12
        let s_89_14: i128 = (s_89_13 + s_89_12);
        // D s_89_15: bit-extract s_89_10 s_89_9 s_89_14
        let s_89_15: Bits = (Bits::new(
            ((s_89_10) >> (s_89_9)).value(),
            u16::try_from(s_89_14).unwrap(),
        ));
        // D s_89_16: cast reint s_89_15 -> u32
        let s_89_16: u32 = (s_89_15.value() as u32);
        // D s_89_17: cast zx s_89_3 -> bv
        let s_89_17: Bits = Bits::new(s_89_3 as u128, 32u16);
        // D s_89_18: cast zx s_89_16 -> bv
        let s_89_18: Bits = Bits::new(s_89_16 as u128, 32u16);
        // D s_89_19: cmp-eq s_89_17 s_89_18
        let s_89_19: bool = ((s_89_17) == (s_89_18));
        // D s_89_20: write-var gs#29886 <= s_89_19
        fn_state.gs_29886 = s_89_19;
        // N s_89_21: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_90_0: const #0s : i
        let s_90_0: i128 = 0;
        // D s_90_1: read-var vaddress:u32
        let s_90_1: u32 = fn_state.vaddress;
        // D s_90_2: cast zx s_90_1 -> bv
        let s_90_2: Bits = Bits::new(s_90_1 as u128, 32u16);
        // C s_90_3: const #1s : i64
        let s_90_3: i64 = 1;
        // C s_90_4: cast zx s_90_3 -> i
        let s_90_4: i128 = (i128::try_from(s_90_3).unwrap());
        // C s_90_5: const #1s : i
        let s_90_5: i128 = 1;
        // C s_90_6: add s_90_5 s_90_4
        let s_90_6: i128 = (s_90_5 + s_90_4);
        // D s_90_7: bit-extract s_90_2 s_90_0 s_90_6
        let s_90_7: Bits = (Bits::new(
            ((s_90_2) >> (s_90_0)).value(),
            u16::try_from(s_90_6).unwrap(),
        ));
        // D s_90_8: cast reint s_90_7 -> u8
        let s_90_8: u8 = (s_90_7.value() as u8);
        // D s_90_9: cast zx s_90_8 -> bv
        let s_90_9: Bits = Bits::new(s_90_8 as u128, 2u16);
        // D s_90_10: cast zx s_90_9 -> i
        let s_90_10: i128 = (s_90_9.value() as i128);
        // D s_90_11: cast reint s_90_10 -> i64
        let s_90_11: i64 = (s_90_10 as i64);
        // D s_90_12: write-var byte <= s_90_11
        fn_state.byte = s_90_11;
        // C s_90_13: const #0s : i
        let s_90_13: i128 = 0;
        // D s_90_14: read-var byte:i64
        let s_90_14: i64 = fn_state.byte;
        // D s_90_15: cast zx s_90_14 -> i
        let s_90_15: i128 = (i128::try_from(s_90_14).unwrap());
        // D s_90_16: cmp-eq s_90_15 s_90_13
        let s_90_16: bool = ((s_90_15) == (s_90_13));
        // N s_90_17: branch s_90_16 b96 b91
        if s_90_16 {
            return block_96(state, tracer, fn_state);
        } else {
            return block_91(state, tracer, fn_state);
        };
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_91_0: const #2s : i
        let s_91_0: i128 = 2;
        // D s_91_1: read-var byte:i64
        let s_91_1: i64 = fn_state.byte;
        // D s_91_2: cast zx s_91_1 -> i
        let s_91_2: i128 = (i128::try_from(s_91_1).unwrap());
        // D s_91_3: cmp-eq s_91_2 s_91_0
        let s_91_3: bool = ((s_91_2) == (s_91_0));
        // D s_91_4: write-var gs#29892 <= s_91_3
        fn_state.gs_29892 = s_91_3;
        // N s_91_5: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_92_0: read-var gs#29892:u8
        let s_92_0: bool = fn_state.gs_29892;
        // N s_92_1: assert s_92_0
        let s_92_1: () = assert!(s_92_0);
        // D s_92_2: read-var nshadow#544:i
        let s_92_2: i128 = fn_state.nshadow_544;
        // D s_92_3: cast reint s_92_2 -> i64
        let s_92_3: i64 = (s_92_2 as i64);
        // D s_92_4: call DBGBCR_read(s_92_3)
        let s_92_4: ProductType700c18a878c5601b = DBGBCR_read(state, tracer, s_92_3);
        // D s_92_5: call _get_DBGBCR_Type_BAS(s_92_4)
        let s_92_5: u8 = u_get_DBGBCR_Type_BAS(state, tracer, s_92_4);
        // D s_92_6: cast zx s_92_5 -> bv
        let s_92_6: Bits = Bits::new(s_92_5 as u128, 4u16);
        // D s_92_7: read-var byte:i64
        let s_92_7: i64 = fn_state.byte;
        // D s_92_8: cast zx s_92_7 -> i
        let s_92_8: i128 = (i128::try_from(s_92_7).unwrap());
        // C s_92_9: const #1u : u64
        let s_92_9: u64 = 1;
        // D s_92_10: bit-extract s_92_6 s_92_8 s_92_9
        let s_92_10: Bits = (Bits::new(
            ((s_92_6) >> (s_92_8)).value(),
            u16::try_from(s_92_9).unwrap(),
        ));
        // D s_92_11: cast reint s_92_10 -> u8
        let s_92_11: bool = ((s_92_10.value()) != 0);
        // C s_92_12: const #0s : i
        let s_92_12: i128 = 0;
        // C s_92_13: const #0u : u64
        let s_92_13: u64 = 0;
        // D s_92_14: cast zx s_92_11 -> u64
        let s_92_14: u64 = (s_92_11 as u64);
        // C s_92_15: const #1u : u64
        let s_92_15: u64 = 1;
        // D s_92_16: and s_92_14 s_92_15
        let s_92_16: u64 = ((s_92_14) & (s_92_15));
        // D s_92_17: cmp-eq s_92_16 s_92_15
        let s_92_17: bool = ((s_92_16) == (s_92_15));
        // D s_92_18: lsl s_92_14 s_92_12
        let s_92_18: u64 = s_92_14 << s_92_12;
        // D s_92_19: or s_92_13 s_92_18
        let s_92_19: u64 = ((s_92_13) | (s_92_18));
        // D s_92_20: cmpl s_92_18
        let s_92_20: u64 = !s_92_18;
        // D s_92_21: and s_92_13 s_92_20
        let s_92_21: u64 = ((s_92_13) & (s_92_20));
        // D s_92_22: select s_92_17 s_92_19 s_92_21
        let s_92_22: u64 = if s_92_17 { s_92_19 } else { s_92_21 };
        // D s_92_23: cast trunc s_92_22 -> u8
        let s_92_23: bool = ((s_92_22) != 0);
        // D s_92_24: cast zx s_92_23 -> bv
        let s_92_24: Bits = Bits::new(s_92_23 as u128, 1u16);
        // C s_92_25: const #1u : u8
        let s_92_25: bool = true;
        // C s_92_26: cast zx s_92_25 -> bv
        let s_92_26: Bits = Bits::new(s_92_25 as u128, 1u16);
        // D s_92_27: cmp-eq s_92_24 s_92_26
        let s_92_27: bool = ((s_92_24) == (s_92_26));
        // D s_92_28: write-var byte_select_match <= s_92_27
        fn_state.byte_select_match = s_92_27;
        // C s_92_29: const #2s : i
        let s_92_29: i128 = 2;
        // C s_92_30: const #30s : i
        let s_92_30: i128 = 30;
        // D s_92_31: read-var vaddress:u32
        let s_92_31: u32 = fn_state.vaddress;
        // D s_92_32: cast zx s_92_31 -> bv
        let s_92_32: Bits = Bits::new(s_92_31 as u128, 32u16);
        // D s_92_33: bit-extract s_92_32 s_92_29 s_92_30
        let s_92_33: Bits = (Bits::new(
            ((s_92_32) >> (s_92_29)).value(),
            u16::try_from(s_92_30).unwrap(),
        ));
        // D s_92_34: cast reint s_92_33 -> u30
        let s_92_34: u32 = (s_92_33.value() as u32);
        // D s_92_35: read-var nshadow#544:i
        let s_92_35: i128 = fn_state.nshadow_544;
        // D s_92_36: cast reint s_92_35 -> i64
        let s_92_36: i64 = (s_92_35 as i64);
        // D s_92_37: call DBGBVR_read(s_92_36)
        let s_92_37: ProductType700c18a878c5601b = DBGBVR_read(state, tracer, s_92_36);
        // D s_92_38: write-var ga#23080 <= s_92_37
        fn_state.ga_23080 = s_92_37;
        // D s_92_39: read-var ga#23080.0:struct
        let s_92_39: u32 = fn_state.ga_23080._0;
        // C s_92_40: const #2s : i
        let s_92_40: i128 = 2;
        // C s_92_41: const #30s : i
        let s_92_41: i128 = 30;
        // D s_92_42: cast zx s_92_39 -> bv
        let s_92_42: Bits = Bits::new(s_92_39 as u128, 32u16);
        // D s_92_43: bit-extract s_92_42 s_92_40 s_92_41
        let s_92_43: Bits = (Bits::new(
            ((s_92_42) >> (s_92_40)).value(),
            u16::try_from(s_92_41).unwrap(),
        ));
        // D s_92_44: cast reint s_92_43 -> u30
        let s_92_44: u32 = (s_92_43.value() as u32);
        // D s_92_45: cast zx s_92_34 -> bv
        let s_92_45: Bits = Bits::new(s_92_34 as u128, 30u16);
        // D s_92_46: cast zx s_92_44 -> bv
        let s_92_46: Bits = Bits::new(s_92_44 as u128, 30u16);
        // D s_92_47: cmp-eq s_92_45 s_92_46
        let s_92_47: bool = ((s_92_45) == (s_92_46));
        // N s_92_48: branch s_92_47 b95 b93
        if s_92_47 {
            return block_95(state, tracer, fn_state);
        } else {
            return block_93(state, tracer, fn_state);
        };
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_93_0: const #0u : u8
        let s_93_0: bool = false;
        // D s_93_1: write-var gs#29901 <= s_93_0
        fn_state.gs_29901 = s_93_0;
        // N s_93_2: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_94_0: read-var gs#29901:u8
        let s_94_0: bool = fn_state.gs_29901;
        // D s_94_1: write-var bvr_match <= s_94_0
        fn_state.bvr_match = s_94_0;
        // N s_94_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_95_0: read-var byte_select_match:u8
        let s_95_0: bool = fn_state.byte_select_match;
        // D s_95_1: write-var gs#29901 <= s_95_0
        fn_state.gs_29901 = s_95_0;
        // N s_95_2: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_96_0: const #1u : u8
        let s_96_0: bool = true;
        // D s_96_1: write-var gs#29892 <= s_96_0
        fn_state.gs_29892 = s_96_0;
        // N s_96_2: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_97_0: const #0u : u8
        let s_97_0: bool = false;
        // C s_97_1: const #0u : u8
        let s_97_1: bool = false;
        // D s_97_2: create-product struct = ["s_97_0", "s_97_1"]
        let s_97_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_97_0,
            _1: s_97_1,
        };
        // D s_97_3: write-var return_value <= s_97_2
        fn_state.return_value = s_97_2;
        // N s_97_4: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_98_0: read-var match_addr:u8
        let s_98_0: bool = fn_state.match_addr;
        // D s_98_1: not s_98_0
        let s_98_1: bool = !s_98_0;
        // D s_98_2: write-var gs#29849 <= s_98_1
        fn_state.gs_29849 = s_98_1;
        // N s_98_3: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_99_0: read-var linking_enabled:u8
        let s_99_0: bool = fn_state.linking_enabled;
        // D s_99_1: write-var gs#29848 <= s_99_0
        fn_state.gs_29848 = s_99_0;
        // N s_99_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_100_0: const #38u : u32
        let s_100_0: u32 = 38;
        // S s_100_1: call ConstrainUnpredictableBool(s_100_0)
        let s_100_1: bool = ConstrainUnpredictableBool(state, tracer, s_100_0);
        // S s_100_2: not s_100_1
        let s_100_2: bool = !s_100_1;
        // N s_100_3: branch s_100_2 b102 b101
        if s_100_2 {
            return block_102(state, tracer, fn_state);
        } else {
            return block_101(state, tracer, fn_state);
        };
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // N s_101_0: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_102_0: const #0u : u8
        let s_102_0: bool = false;
        // C s_102_1: const #0u : u8
        let s_102_1: bool = false;
        // D s_102_2: create-product struct = ["s_102_0", "s_102_1"]
        let s_102_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_102_0,
            _1: s_102_1,
        };
        // D s_102_3: write-var return_value <= s_102_2
        fn_state.return_value = s_102_2;
        // N s_102_4: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_103_0: read-var linking_enabled:u8
        let s_103_0: bool = fn_state.linking_enabled;
        // D s_103_1: not s_103_0
        let s_103_1: bool = !s_103_0;
        // D s_103_2: write-var gs#29847 <= s_103_1
        fn_state.gs_29847 = s_103_1;
        // N s_103_3: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_104_0: const #0u : u8
        let s_104_0: bool = false;
        // D s_104_1: write-var gs#29842 <= s_104_0
        fn_state.gs_29842 = s_104_0;
        // N s_104_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_105_0: const #0u : u8
        let s_105_0: bool = false;
        // D s_105_1: write-var gs#29837 <= s_105_0
        fn_state.gs_29837 = s_105_0;
        // N s_105_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_106_0: const #0u : u8
        let s_106_0: bool = false;
        // D s_106_1: write-var gs#29832 <= s_106_0
        fn_state.gs_29832 = s_106_0;
        // N s_106_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_107_0: const #0u : u8
        let s_107_0: bool = false;
        // D s_107_1: write-var gs#29827 <= s_107_0
        fn_state.gs_29827 = s_107_0;
        // N s_107_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_108_0: const #0u : u8
        let s_108_0: bool = false;
        // D s_108_1: write-var gs#29822 <= s_108_0
        fn_state.gs_29822 = s_108_0;
        // N s_108_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_109_0: const #0u : u8
        let s_109_0: bool = false;
        // D s_109_1: write-var gs#29814 <= s_109_0
        fn_state.gs_29814 = s_109_0;
        // N s_109_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_110_0: const #1s : i
        let s_110_0: i128 = 1;
        // D s_110_1: read-var b__5:u8
        let s_110_1: u8 = fn_state.b__5;
        // D s_110_2: cast zx s_110_1 -> bv
        let s_110_2: Bits = Bits::new(s_110_1 as u128, 4u16);
        // C s_110_3: const #1s : i64
        let s_110_3: i64 = 1;
        // C s_110_4: cast zx s_110_3 -> i
        let s_110_4: i128 = (i128::try_from(s_110_3).unwrap());
        // C s_110_5: const #0s : i
        let s_110_5: i128 = 0;
        // C s_110_6: add s_110_5 s_110_4
        let s_110_6: i128 = (s_110_5 + s_110_4);
        // D s_110_7: bit-extract s_110_2 s_110_0 s_110_6
        let s_110_7: Bits = (Bits::new(
            ((s_110_2) >> (s_110_0)).value(),
            u16::try_from(s_110_6).unwrap(),
        ));
        // D s_110_8: cast reint s_110_7 -> u8
        let s_110_8: bool = ((s_110_7.value()) != 0);
        // D s_110_9: cast zx s_110_8 -> bv
        let s_110_9: Bits = Bits::new(s_110_8 as u128, 1u16);
        // C s_110_10: const #0u : u8
        let s_110_10: bool = false;
        // C s_110_11: cast zx s_110_10 -> bv
        let s_110_11: Bits = Bits::new(s_110_10 as u128, 1u16);
        // D s_110_12: cmp-eq s_110_9 s_110_11
        let s_110_12: bool = ((s_110_9) == (s_110_11));
        // D s_110_13: write-var gs#29819 <= s_110_12
        fn_state.gs_29819 = s_110_12;
        // N s_110_14: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_111_0: const #0u : u8
        let s_111_0: bool = false;
        // C s_111_1: const #0u : u8
        let s_111_1: bool = false;
        // D s_111_2: create-product struct = ["s_111_0", "s_111_1"]
        let s_111_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_111_0,
            _1: s_111_1,
        };
        // D s_111_3: write-var return_value <= s_111_2
        fn_state.return_value = s_111_2;
        // N s_111_4: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_112_0: const #0u : u8
        let s_112_0: bool = false;
        // C s_112_1: const #0u : u8
        let s_112_1: bool = false;
        // D s_112_2: create-product struct = ["s_112_0", "s_112_1"]
        let s_112_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_112_0,
            _1: s_112_1,
        };
        // D s_112_3: write-var return_value <= s_112_2
        fn_state.return_value = s_112_2;
        // N s_112_4: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_113_0: read-var nshadow#544:i
        let s_113_0: i128 = fn_state.nshadow_544;
        // D s_113_1: call __id(s_113_0)
        let s_113_1: i128 = u__id(state, tracer, s_113_0);
        // C s_113_2: const #16s : i
        let s_113_2: i128 = 16;
        // D s_113_3: cmp-lt s_113_1 s_113_2
        let s_113_3: bool = ((s_113_1) < (s_113_2));
        // D s_113_4: write-var gs#29804 <= s_113_3
        fn_state.gs_29804 = s_113_3;
        // N s_113_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_114_0: const #() : ()
        let s_114_0: () = ();
        // S s_114_1: call NumBreakpointsImplemented(s_114_0)
        let s_114_1: i128 = NumBreakpointsImplemented(state, tracer, s_114_0);
        // C s_114_2: const #1s : i
        let s_114_2: i128 = 1;
        // S s_114_3: sub s_114_1 s_114_2
        let s_114_3: i128 = ((s_114_1) - (s_114_2));
        // C s_114_4: const #0s : i
        let s_114_4: i128 = 0;
        // C s_114_5: const #32u : u32
        let s_114_5: u32 = 32;
        // S s_114_6: call ConstrainUnpredictableInteger(s_114_4, s_114_3, s_114_5)
        let s_114_6: ProductType396b95aa74979079 = ConstrainUnpredictableInteger(
            state,
            tracer,
            s_114_4,
            s_114_3,
            s_114_5,
        );
        // D s_114_7: write-var ga#23057 <= s_114_6
        fn_state.ga_23057 = s_114_6;
        // D s_114_8: read-var ga#23057.0:struct
        let s_114_8: u32 = fn_state.ga_23057._0;
        // D s_114_9: read-var ga#23057.1:struct
        let s_114_9: i128 = fn_state.ga_23057._1;
        // D s_114_10: write-var c <= s_114_8
        fn_state.c = s_114_8;
        // D s_114_11: write-var n <= s_114_9
        fn_state.n = s_114_9;
        // D s_114_12: read-var c:u32
        let s_114_12: u32 = fn_state.c;
        // C s_114_13: const #7u : u32
        let s_114_13: u32 = 7;
        // D s_114_14: cmp-eq s_114_12 s_114_13
        let s_114_14: bool = ((s_114_12) == (s_114_13));
        // N s_114_15: branch s_114_14 b119 b115
        if s_114_14 {
            return block_119(state, tracer, fn_state);
        } else {
            return block_115(state, tracer, fn_state);
        };
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_115_0: read-var c:u32
        let s_115_0: u32 = fn_state.c;
        // C s_115_1: const #1u : u32
        let s_115_1: u32 = 1;
        // D s_115_2: cmp-eq s_115_0 s_115_1
        let s_115_2: bool = ((s_115_0) == (s_115_1));
        // D s_115_3: write-var gs#29796 <= s_115_2
        fn_state.gs_29796 = s_115_2;
        // N s_115_4: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_116_0: read-var gs#29796:u8
        let s_116_0: bool = fn_state.gs_29796;
        // N s_116_1: assert s_116_0
        let s_116_1: () = assert!(s_116_0);
        // D s_116_2: read-var c:u32
        let s_116_2: u32 = fn_state.c;
        // C s_116_3: const #7u : u32
        let s_116_3: u32 = 7;
        // D s_116_4: cmp-eq s_116_2 s_116_3
        let s_116_4: bool = ((s_116_2) == (s_116_3));
        // N s_116_5: branch s_116_4 b118 b117
        if s_116_4 {
            return block_118(state, tracer, fn_state);
        } else {
            return block_117(state, tracer, fn_state);
        };
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // N s_117_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_118_0: const #0u : u8
        let s_118_0: bool = false;
        // C s_118_1: const #0u : u8
        let s_118_1: bool = false;
        // D s_118_2: create-product struct = ["s_118_0", "s_118_1"]
        let s_118_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_118_0,
            _1: s_118_1,
        };
        // D s_118_3: write-var return_value <= s_118_2
        fn_state.return_value = s_118_2;
        // N s_118_4: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_119_0: const #1u : u8
        let s_119_0: bool = true;
        // D s_119_1: write-var gs#29796 <= s_119_0
        fn_state.gs_29796 = s_119_0;
        // N s_119_2: jump b116
        return block_116(state, tracer, fn_state);
    }
}
