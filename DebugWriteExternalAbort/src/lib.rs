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
use IsZero::*;
use PendSErrorInterrupt::*;
use HaveRME::*;
use EncodeLDFSC::*;
use Bit::*;
use u__IMPDEF_boolean::*;
use Zeros::*;
use NoFault::*;
use HaveRASExt::*;
use Unreachable::*;
use common::*;
pub fn DebugWriteExternalAbort<T: Tracer>(
    state: &mut State,
    tracer: &T,
    memstatus: ProductTypef8c3639b88223255,
    addrdesc: ProductTypece7c66ccb2cab13e,
    start_vaddr: u64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_20279: ProductType1d757adad216cdef,
        ga_20274: ProductType1d757adad216cdef,
        ga_20295: ProductType396b95aa74979079,
        gs_25979: bool,
        ga_20267: ProductType9878976b5bcce9c9,
        gs_25980: bool,
        ga_20275: ProductType9878976b5bcce9c9,
        fault: ProductType1d757adad216cdef,
        gs_25958: bool,
        gs_25949: bool,
        ec: u8,
        ga_20266: ProductType1d757adad216cdef,
        statuscode: u32,
        gs_25952: bool,
        ga_20315: ProductType1d757adad216cdef,
        ga_20308: ProductType1d757adad216cdef,
        gs_25951: bool,
        gs_25948: bool,
        ga_20317: ProductType1d757adad216cdef,
        ttw_abort: bool,
        ga_20291: ProductType396b95aa74979079,
        ga_20290: ProductType1d757adad216cdef,
        extflag: bool,
        ga_20314: ProductType1d757adad216cdef,
        ga_20303: ProductType1d757adad216cdef,
        async_external_abort: bool,
        gs_25947: bool,
        handle_as_SError: bool,
        ga_20309: ProductType9878976b5bcce9c9,
        ga_20294: ProductType1d757adad216cdef,
        ga_20280: ProductType9878976b5bcce9c9,
        syndrome: u64,
        gs_25959: bool,
        ga_20313: ProductType1d757adad216cdef,
        ga_20299: ProductType1d757adad216cdef,
        memstatus: ProductTypef8c3639b88223255,
        addrdesc: ProductTypece7c66ccb2cab13e,
        start_vaddr: u64,
    }
    let fn_state = FunctionState {
        memstatus,
        addrdesc,
        start_vaddr,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #0u : u8
        let s_0_0: bool = false;
        // D s_0_1: write-var handle_as_SError <= s_0_0
        fn_state.handle_as_SError = s_0_0;
        // C s_0_2: const #0u : u8
        let s_0_2: bool = false;
        // D s_0_3: write-var async_external_abort <= s_0_2
        fn_state.async_external_abort = s_0_2;
        // D s_0_4: read-var addrdesc.0:struct
        let s_0_4: ProductType1d757adad216cdef = fn_state.addrdesc._0;
        // D s_0_5: write-var ga#20266 <= s_0_4
        fn_state.ga_20266 = s_0_4;
        // D s_0_6: read-var ga#20266.0:struct
        let s_0_6: ProductType9878976b5bcce9c9 = fn_state.ga_20266._0;
        // D s_0_7: write-var ga#20267 <= s_0_6
        fn_state.ga_20267 = s_0_6;
        // D s_0_8: read-var ga#20267.1:struct
        let s_0_8: u32 = fn_state.ga_20267._1;
        // C s_0_9: const #10u : u32
        let s_0_9: u32 = 10;
        // D s_0_10: cmp-eq s_0_9 s_0_8
        let s_0_10: bool = ((s_0_9) == (s_0_8));
        // D s_0_11: not s_0_10
        let s_0_11: bool = !s_0_10;
        // N s_0_12: branch s_0_11 b60 b1
        if s_0_11 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #"SPE SyncExternal as SError" : str
        let s_1_0: &'static str = "SPE SyncExternal as SError";
        // S s_1_1: call __IMPDEF_boolean(s_1_0)
        let s_1_1: bool = u__IMPDEF_boolean(state, tracer, s_1_0);
        // D s_1_2: write-var handle_as_SError <= s_1_1
        fn_state.handle_as_SError = s_1_1;
        // C s_1_3: const #"SPE async External abort" : str
        let s_1_3: &'static str = "SPE async External abort";
        // S s_1_4: call __IMPDEF_boolean(s_1_3)
        let s_1_4: bool = u__IMPDEF_boolean(state, tracer, s_1_3);
        // D s_1_5: write-var async_external_abort <= s_1_4
        fn_state.async_external_abort = s_1_4;
        // C s_1_6: const #13704u : u32
        let s_1_6: u32 = 13704;
        // D s_1_7: read-reg s_1_6:u64
        let s_1_7: u64 = {
            let value = state.read_register::<u64>(s_1_6 as isize);
            tracer.read_register(s_1_6 as isize, value);
            value
        };
        // C s_1_8: const #0s : i
        let s_1_8: i128 = 0;
        // D s_1_9: cast zx s_1_7 -> bv
        let s_1_9: Bits = Bits::new(s_1_7 as u128, 64u16);
        // C s_1_10: const #1s : i64
        let s_1_10: i64 = 1;
        // C s_1_11: cast zx s_1_10 -> i
        let s_1_11: i128 = (i128::try_from(s_1_10).unwrap());
        // C s_1_12: const #63s : i
        let s_1_12: i128 = 63;
        // C s_1_13: add s_1_12 s_1_11
        let s_1_13: i128 = (s_1_12 + s_1_11);
        // D s_1_14: bit-extract s_1_9 s_1_8 s_1_13
        let s_1_14: Bits = (Bits::new(
            ((s_1_9) >> (s_1_8)).value(),
            u16::try_from(s_1_13).unwrap(),
        ));
        // D s_1_15: cast reint s_1_14 -> u64
        let s_1_15: u64 = (s_1_14.value() as u64);
        // D s_1_16: write-var syndrome <= s_1_15
        fn_state.syndrome = s_1_15;
        // N s_1_17: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var addrdesc.0:struct
        let s_2_0: ProductType1d757adad216cdef = fn_state.addrdesc._0;
        // D s_2_1: write-var ga#20315 <= s_2_0
        fn_state.ga_20315 = s_2_0;
        // D s_2_2: read-var ga#20315.16:struct
        let s_2_2: u32 = fn_state.ga_20315._16;
        // C s_2_3: const #9u : u32
        let s_2_3: u32 = 9;
        // D s_2_4: cmp-eq s_2_2 s_2_3
        let s_2_4: bool = ((s_2_2) == (s_2_3));
        // N s_2_5: branch s_2_4 b59 b3
        if s_2_4 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var addrdesc.0:struct
        let s_3_0: ProductType1d757adad216cdef = fn_state.addrdesc._0;
        // D s_3_1: write-var ga#20317 <= s_3_0
        fn_state.ga_20317 = s_3_0;
        // D s_3_2: read-var ga#20317.16:struct
        let s_3_2: u32 = fn_state.ga_20317._16;
        // C s_3_3: const #11u : u32
        let s_3_3: u32 = 11;
        // D s_3_4: cmp-eq s_3_2 s_3_3
        let s_3_4: bool = ((s_3_2) == (s_3_3));
        // D s_3_5: write-var gs#25947 <= s_3_4
        fn_state.gs_25947 = s_3_4;
        // N s_3_6: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#25947:u8
        let s_4_0: bool = fn_state.gs_25947;
        // D s_4_1: write-var ttw_abort <= s_4_0
        fn_state.ttw_abort = s_4_0;
        // D s_4_2: read-var ttw_abort:u8
        let s_4_2: bool = fn_state.ttw_abort;
        // N s_4_3: branch s_4_2 b58 b5
        if s_4_2 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var memstatus.2:struct
        let s_5_0: u32 = fn_state.memstatus._2;
        // D s_5_1: write-var statuscode <= s_5_0
        fn_state.statuscode = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var ttw_abort:u8
        let s_6_0: bool = fn_state.ttw_abort;
        // N s_6_1: branch s_6_0 b57 b7
        if s_6_0 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var memstatus.0:struct
        let s_7_0: bool = fn_state.memstatus._0;
        // D s_7_1: write-var extflag <= s_7_0
        fn_state.extflag = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var statuscode:u32
        let s_8_0: u32 = fn_state.statuscode;
        // C s_8_1: const #15u : u32
        let s_8_1: u32 = 15;
        // D s_8_2: cmp-eq s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) == (s_8_1));
        // N s_8_3: branch s_8_2 b56 b9
        if s_8_2 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var statuscode:u32
        let s_9_0: u32 = fn_state.statuscode;
        // C s_9_1: const #14u : u32
        let s_9_1: u32 = 14;
        // D s_9_2: cmp-eq s_9_0 s_9_1
        let s_9_2: bool = ((s_9_0) == (s_9_1));
        // D s_9_3: write-var gs#25948 <= s_9_2
        fn_state.gs_25948 = s_9_2;
        // N s_9_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#25948:u8
        let s_10_0: bool = fn_state.gs_25948;
        // N s_10_1: branch s_10_0 b55 b11
        if s_10_0 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var handle_as_SError:u8
        let s_11_0: bool = fn_state.handle_as_SError;
        // D s_11_1: write-var gs#25949 <= s_11_0
        fn_state.gs_25949 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#25949:u8
        let s_12_0: bool = fn_state.gs_25949;
        // N s_12_1: branch s_12_0 b42 b13
        if s_12_0 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #1u : u8
        let s_13_0: bool = true;
        // S s_13_1: call Bit(s_13_0)
        let s_13_1: bool = Bit(state, tracer, s_13_0);
        // C s_13_2: const #18s : i
        let s_13_2: i128 = 18;
        // D s_13_3: read-var syndrome:u64
        let s_13_3: u64 = fn_state.syndrome;
        // D s_13_4: cast zx s_13_3 -> bv
        let s_13_4: Bits = Bits::new(s_13_3 as u128, 64u16);
        // C s_13_5: const #1u : u64
        let s_13_5: u64 = 1;
        // D s_13_6: bit-insert s_13_4 s_13_4 s_13_2 s_13_5
        let s_13_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_13_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_13_4.length(),
            );
            (s_13_4 & mask) | (s_13_4 << s_13_2)
        };
        // D s_13_7: cast reint s_13_6 -> u64
        let s_13_7: u64 = (s_13_6.value() as u64);
        // D s_13_8: write-var syndrome <= s_13_7
        fn_state.syndrome = s_13_7;
        // D s_13_9: read-var addrdesc.0:struct
        let s_13_9: ProductType1d757adad216cdef = fn_state.addrdesc._0;
        // D s_13_10: write-var ga#20279 <= s_13_9
        fn_state.ga_20279 = s_13_9;
        // D s_13_11: read-var ga#20279.0:struct
        let s_13_11: ProductType9878976b5bcce9c9 = fn_state.ga_20279._0;
        // D s_13_12: write-var ga#20280 <= s_13_11
        fn_state.ga_20280 = s_13_11;
        // D s_13_13: read-var ga#20280.1:struct
        let s_13_13: u32 = fn_state.ga_20280._1;
        // C s_13_14: const #10u : u32
        let s_13_14: u32 = 10;
        // D s_13_15: cmp-eq s_13_13 s_13_14
        let s_13_15: bool = ((s_13_13) == (s_13_14));
        // N s_13_16: branch s_13_15 b38 b14
        if s_13_15 {
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
        // D s_14_1: write-var gs#25952 <= s_14_0
        fn_state.gs_25952 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#25952:u8
        let s_15_0: bool = fn_state.gs_25952;
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
        // N s_16_0: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #17s : i
        let s_17_0: i128 = 17;
        // D s_17_1: read-var syndrome:u64
        let s_17_1: u64 = fn_state.syndrome;
        // D s_17_2: cast zx s_17_1 -> bv
        let s_17_2: Bits = Bits::new(s_17_1 as u128, 64u16);
        // C s_17_3: const #1u : u64
        let s_17_3: u64 = 1;
        // D s_17_4: bit-extract s_17_2 s_17_0 s_17_3
        let s_17_4: Bits = (Bits::new(
            ((s_17_2) >> (s_17_0)).value(),
            u16::try_from(s_17_3).unwrap(),
        ));
        // D s_17_5: cast reint s_17_4 -> u8
        let s_17_5: bool = ((s_17_4.value()) != 0);
        // C s_17_6: const #0s : i
        let s_17_6: i128 = 0;
        // C s_17_7: const #0u : u64
        let s_17_7: u64 = 0;
        // D s_17_8: cast zx s_17_5 -> u64
        let s_17_8: u64 = (s_17_5 as u64);
        // C s_17_9: const #1u : u64
        let s_17_9: u64 = 1;
        // D s_17_10: and s_17_8 s_17_9
        let s_17_10: u64 = ((s_17_8) & (s_17_9));
        // D s_17_11: cmp-eq s_17_10 s_17_9
        let s_17_11: bool = ((s_17_10) == (s_17_9));
        // D s_17_12: lsl s_17_8 s_17_6
        let s_17_12: u64 = s_17_8 << s_17_6;
        // D s_17_13: or s_17_7 s_17_12
        let s_17_13: u64 = ((s_17_7) | (s_17_12));
        // D s_17_14: cmpl s_17_12
        let s_17_14: u64 = !s_17_12;
        // D s_17_15: and s_17_7 s_17_14
        let s_17_15: u64 = ((s_17_7) & (s_17_14));
        // D s_17_16: select s_17_11 s_17_13 s_17_15
        let s_17_16: u64 = if s_17_11 { s_17_13 } else { s_17_15 };
        // D s_17_17: cast trunc s_17_16 -> u8
        let s_17_17: bool = ((s_17_16) != 0);
        // D s_17_18: cast zx s_17_17 -> bv
        let s_17_18: Bits = Bits::new(s_17_17 as u128, 1u16);
        // D s_17_19: call IsZero(s_17_18)
        let s_17_19: bool = IsZero(state, tracer, s_17_18);
        // N s_17_20: branch s_17_19 b22 b18
        if s_17_19 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var addrdesc.0:struct
        let s_19_0: ProductType1d757adad216cdef = fn_state.addrdesc._0;
        // D s_19_1: write-var ga#20308 <= s_19_0
        fn_state.ga_20308 = s_19_0;
        // D s_19_2: read-var ga#20308.0:struct
        let s_19_2: ProductType9878976b5bcce9c9 = fn_state.ga_20308._0;
        // D s_19_3: write-var ga#20309 <= s_19_2
        fn_state.ga_20309 = s_19_2;
        // D s_19_4: read-var ga#20309.1:struct
        let s_19_4: u32 = fn_state.ga_20309._1;
        // C s_19_5: const #10u : u32
        let s_19_5: u32 = 10;
        // D s_19_6: cmp-eq s_19_5 s_19_4
        let s_19_6: bool = ((s_19_5) == (s_19_4));
        // D s_19_7: not s_19_6
        let s_19_7: bool = !s_19_6;
        // N s_19_8: branch s_19_7 b21 b20
        if s_19_7 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #13704u : u32
        let s_20_0: u32 = 13704;
        // D s_20_1: read-reg s_20_0:struct
        let s_20_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_20_0 as isize);
            tracer.read_register(s_20_0 as isize, value);
            value
        };
        // C s_20_2: const #13704u : u32
        let s_20_2: u32 = 13704;
        // N s_20_3: write-reg s_20_2 <= s_20_1
        let s_20_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_20_2 as isize, s_20_1);
            tracer.write_register(s_20_2 as isize, s_20_1);
        };
        // N s_20_4: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #() : ()
        let s_21_0: () = ();
        // S s_21_1: call Unreachable(s_21_0)
        let s_21_1: () = Unreachable(state, tracer, s_21_0);
        // N s_21_2: return
        return;
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // S s_22_1: call Bit(s_22_0)
        let s_22_1: bool = Bit(state, tracer, s_22_0);
        // C s_22_2: const #17s : i
        let s_22_2: i128 = 17;
        // D s_22_3: read-var syndrome:u64
        let s_22_3: u64 = fn_state.syndrome;
        // D s_22_4: cast zx s_22_3 -> bv
        let s_22_4: Bits = Bits::new(s_22_3 as u128, 64u16);
        // C s_22_5: const #1u : u64
        let s_22_5: u64 = 1;
        // D s_22_6: bit-insert s_22_4 s_22_4 s_22_2 s_22_5
        let s_22_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_22_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_22_4.length(),
            );
            (s_22_4 & mask) | (s_22_4 << s_22_2)
        };
        // D s_22_7: cast reint s_22_6 -> u64
        let s_22_7: u64 = (s_22_6.value() as u64);
        // D s_22_8: write-var syndrome <= s_22_7
        fn_state.syndrome = s_22_7;
        // C s_22_9: const #() : ()
        let s_22_9: () = ();
        // S s_22_10: call HaveRME(s_22_9)
        let s_22_10: bool = HaveRME(state, tracer, s_22_9);
        // N s_22_11: branch s_22_10 b36 b23
        if s_22_10 {
            return block_36(state, tracer, fn_state);
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
        // D s_23_1: write-var gs#25958 <= s_23_0
        fn_state.gs_25958 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#25958:u8
        let s_24_0: bool = fn_state.gs_25958;
        // N s_24_1: branch s_24_0 b35 b25
        if s_24_0 {
            return block_35(state, tracer, fn_state);
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
        // D s_25_1: write-var gs#25959 <= s_25_0
        fn_state.gs_25959 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#25959:u8
        let s_26_0: bool = fn_state.gs_25959;
        // N s_26_1: branch s_26_0 b34 b27
        if s_26_0 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var addrdesc.0:struct
        let s_27_0: ProductType1d757adad216cdef = fn_state.addrdesc._0;
        // D s_27_1: write-var ga#20299 <= s_27_0
        fn_state.ga_20299 = s_27_0;
        // D s_27_2: read-var ga#20299.15:struct
        let s_27_2: bool = fn_state.ga_20299._15;
        // N s_27_3: branch s_27_2 b33 b28
        if s_27_2 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #36u : u8
        let s_28_0: u8 = 36;
        // D s_28_1: write-var ec <= s_28_0
        fn_state.ec = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
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
        // C s_30_0: const #26s : i
        let s_30_0: i128 = 26;
        // D s_30_1: read-var syndrome:u64
        let s_30_1: u64 = fn_state.syndrome;
        // D s_30_2: cast zx s_30_1 -> bv
        let s_30_2: Bits = Bits::new(s_30_1 as u128, 64u16);
        // D s_30_3: read-var ec:u8
        let s_30_3: u8 = fn_state.ec;
        // D s_30_4: cast zx s_30_3 -> bv
        let s_30_4: Bits = Bits::new(s_30_3 as u128, 6u16);
        // C s_30_5: const #5s : i
        let s_30_5: i128 = 5;
        // C s_30_6: const #1u : u64
        let s_30_6: u64 = 1;
        // C s_30_7: cast zx s_30_6 -> bv
        let s_30_7: Bits = Bits::new(s_30_6 as u128, 64u16);
        // C s_30_8: lsl s_30_7 s_30_5
        let s_30_8: Bits = s_30_7 << s_30_5;
        // C s_30_9: sub s_30_8 s_30_7
        let s_30_9: Bits = ((s_30_8) - (s_30_7));
        // D s_30_10: and s_30_4 s_30_9
        let s_30_10: Bits = ((s_30_4) & (s_30_9));
        // D s_30_11: lsl s_30_10 s_30_0
        let s_30_11: Bits = s_30_10 << s_30_0;
        // C s_30_12: lsl s_30_9 s_30_0
        let s_30_12: Bits = s_30_9 << s_30_0;
        // C s_30_13: cmpl s_30_12
        let s_30_13: Bits = !s_30_12;
        // D s_30_14: and s_30_2 s_30_13
        let s_30_14: Bits = ((s_30_2) & (s_30_13));
        // D s_30_15: or s_30_14 s_30_11
        let s_30_15: Bits = ((s_30_14) | (s_30_11));
        // D s_30_16: cast reint s_30_15 -> u64
        let s_30_16: u64 = (s_30_15.value() as u64);
        // D s_30_17: write-var syndrome <= s_30_16
        fn_state.syndrome = s_30_16;
        // D s_30_18: read-var async_external_abort:u8
        let s_30_18: bool = fn_state.async_external_abort;
        // N s_30_19: branch s_30_18 b32 b31
        if s_30_18 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #10s : i
        let s_31_0: i128 = 10;
        // S s_31_1: call Zeros(s_31_0)
        let s_31_1: Bits = Zeros(state, tracer, s_31_0);
        // S s_31_2: cast reint s_31_1 -> u10
        let s_31_2: u16 = (s_31_1.value() as u16);
        // D s_31_3: read-var addrdesc.0:struct
        let s_31_3: ProductType1d757adad216cdef = fn_state.addrdesc._0;
        // D s_31_4: write-var ga#20303 <= s_31_3
        fn_state.ga_20303 = s_31_3;
        // D s_31_5: read-var ga#20303.9:struct
        let s_31_5: i128 = fn_state.ga_20303._9;
        // D s_31_6: read-var statuscode:u32
        let s_31_6: u32 = fn_state.statuscode;
        // D s_31_7: call EncodeLDFSC(s_31_6, s_31_5)
        let s_31_7: u8 = EncodeLDFSC(state, tracer, s_31_6, s_31_5);
        // S s_31_8: cast zx s_31_2 -> bv
        let s_31_8: Bits = Bits::new(s_31_2 as u128, 10u16);
        // D s_31_9: cast zx s_31_7 -> bv
        let s_31_9: Bits = Bits::new(s_31_7 as u128, 6u16);
        // S s_31_10: cast reint s_31_8 -> u128
        let s_31_10: u128 = (s_31_8.value() as u128);
        // D s_31_11: size-of s_31_8
        let s_31_11: u16 = s_31_8.length();
        // D s_31_12: cast reint s_31_9 -> u128
        let s_31_12: u128 = (s_31_9.value() as u128);
        // D s_31_13: size-of s_31_9
        let s_31_13: u16 = s_31_9.length();
        // D s_31_14: lsl s_31_10 s_31_13
        let s_31_14: u128 = s_31_10 << s_31_13;
        // D s_31_15: or s_31_14 s_31_12
        let s_31_15: u128 = ((s_31_14) | (s_31_12));
        // D s_31_16: add s_31_11 s_31_13
        let s_31_16: u16 = (s_31_11 + s_31_13);
        // D s_31_17: create-bits s_31_15 s_31_16
        let s_31_17: Bits = Bits::new(s_31_15, s_31_16);
        // D s_31_18: cast reint s_31_17 -> u16
        let s_31_18: u16 = (s_31_17.value() as u16);
        // C s_31_19: const #0s : i
        let s_31_19: i128 = 0;
        // D s_31_20: read-var syndrome:u64
        let s_31_20: u64 = fn_state.syndrome;
        // D s_31_21: cast zx s_31_20 -> bv
        let s_31_21: Bits = Bits::new(s_31_20 as u128, 64u16);
        // D s_31_22: cast zx s_31_18 -> bv
        let s_31_22: Bits = Bits::new(s_31_18 as u128, 16u16);
        // C s_31_23: const #15s : i
        let s_31_23: i128 = 15;
        // C s_31_24: const #1u : u64
        let s_31_24: u64 = 1;
        // C s_31_25: cast zx s_31_24 -> bv
        let s_31_25: Bits = Bits::new(s_31_24 as u128, 64u16);
        // C s_31_26: lsl s_31_25 s_31_23
        let s_31_26: Bits = s_31_25 << s_31_23;
        // C s_31_27: sub s_31_26 s_31_25
        let s_31_27: Bits = ((s_31_26) - (s_31_25));
        // D s_31_28: and s_31_22 s_31_27
        let s_31_28: Bits = ((s_31_22) & (s_31_27));
        // D s_31_29: lsl s_31_28 s_31_19
        let s_31_29: Bits = s_31_28 << s_31_19;
        // C s_31_30: lsl s_31_27 s_31_19
        let s_31_30: Bits = s_31_27 << s_31_19;
        // C s_31_31: cmpl s_31_30
        let s_31_31: Bits = !s_31_30;
        // D s_31_32: and s_31_21 s_31_31
        let s_31_32: Bits = ((s_31_21) & (s_31_31));
        // D s_31_33: or s_31_32 s_31_29
        let s_31_33: Bits = ((s_31_32) | (s_31_29));
        // D s_31_34: cast reint s_31_33 -> u64
        let s_31_34: u64 = (s_31_33.value() as u64);
        // D s_31_35: write-var syndrome <= s_31_34
        fn_state.syndrome = s_31_34;
        // N s_31_36: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #10s : i
        let s_32_0: i128 = 10;
        // S s_32_1: call Zeros(s_32_0)
        let s_32_1: Bits = Zeros(state, tracer, s_32_0);
        // S s_32_2: cast reint s_32_1 -> u10
        let s_32_2: u16 = (s_32_1.value() as u16);
        // S s_32_3: cast zx s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 10u16);
        // C s_32_4: const #17u : u8
        let s_32_4: u8 = 17;
        // C s_32_5: cast zx s_32_4 -> bv
        let s_32_5: Bits = Bits::new(s_32_4 as u128, 6u16);
        // S s_32_6: cast reint s_32_3 -> u128
        let s_32_6: u128 = (s_32_3.value() as u128);
        // D s_32_7: size-of s_32_3
        let s_32_7: u16 = s_32_3.length();
        // C s_32_8: cast reint s_32_5 -> u128
        let s_32_8: u128 = (s_32_5.value() as u128);
        // D s_32_9: size-of s_32_5
        let s_32_9: u16 = s_32_5.length();
        // D s_32_10: lsl s_32_6 s_32_9
        let s_32_10: u128 = s_32_6 << s_32_9;
        // D s_32_11: or s_32_10 s_32_8
        let s_32_11: u128 = ((s_32_10) | (s_32_8));
        // D s_32_12: add s_32_7 s_32_9
        let s_32_12: u16 = (s_32_7 + s_32_9);
        // D s_32_13: create-bits s_32_11 s_32_12
        let s_32_13: Bits = Bits::new(s_32_11, s_32_12);
        // D s_32_14: cast reint s_32_13 -> u16
        let s_32_14: u16 = (s_32_13.value() as u16);
        // C s_32_15: const #0s : i
        let s_32_15: i128 = 0;
        // D s_32_16: read-var syndrome:u64
        let s_32_16: u64 = fn_state.syndrome;
        // D s_32_17: cast zx s_32_16 -> bv
        let s_32_17: Bits = Bits::new(s_32_16 as u128, 64u16);
        // D s_32_18: cast zx s_32_14 -> bv
        let s_32_18: Bits = Bits::new(s_32_14 as u128, 16u16);
        // C s_32_19: const #15s : i
        let s_32_19: i128 = 15;
        // C s_32_20: const #1u : u64
        let s_32_20: u64 = 1;
        // C s_32_21: cast zx s_32_20 -> bv
        let s_32_21: Bits = Bits::new(s_32_20 as u128, 64u16);
        // C s_32_22: lsl s_32_21 s_32_19
        let s_32_22: Bits = s_32_21 << s_32_19;
        // C s_32_23: sub s_32_22 s_32_21
        let s_32_23: Bits = ((s_32_22) - (s_32_21));
        // D s_32_24: and s_32_18 s_32_23
        let s_32_24: Bits = ((s_32_18) & (s_32_23));
        // D s_32_25: lsl s_32_24 s_32_15
        let s_32_25: Bits = s_32_24 << s_32_15;
        // C s_32_26: lsl s_32_23 s_32_15
        let s_32_26: Bits = s_32_23 << s_32_15;
        // C s_32_27: cmpl s_32_26
        let s_32_27: Bits = !s_32_26;
        // D s_32_28: and s_32_17 s_32_27
        let s_32_28: Bits = ((s_32_17) & (s_32_27));
        // D s_32_29: or s_32_28 s_32_25
        let s_32_29: Bits = ((s_32_28) | (s_32_25));
        // D s_32_30: cast reint s_32_29 -> u64
        let s_32_30: u64 = (s_32_29.value() as u64);
        // D s_32_31: write-var syndrome <= s_32_30
        fn_state.syndrome = s_32_30;
        // N s_32_32: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #37u : u8
        let s_33_0: u8 = 37;
        // D s_33_1: write-var ec <= s_33_0
        fn_state.ec = s_33_0;
        // N s_33_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #30u : u8
        let s_34_0: u8 = 30;
        // D s_34_1: write-var ec <= s_34_0
        fn_state.ec = s_34_0;
        // N s_34_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var addrdesc.0:struct
        let s_35_0: ProductType1d757adad216cdef = fn_state.addrdesc._0;
        // D s_35_1: write-var ga#20294 <= s_35_0
        fn_state.ga_20294 = s_35_0;
        // D s_35_2: read-var ga#20294.6:struct
        let s_35_2: ProductType396b95aa74979079 = fn_state.ga_20294._6;
        // D s_35_3: write-var ga#20295 <= s_35_2
        fn_state.ga_20295 = s_35_2;
        // D s_35_4: read-var ga#20295.0:struct
        let s_35_4: u32 = fn_state.ga_20295._0;
        // C s_35_5: const #4u : u32
        let s_35_5: u32 = 4;
        // D s_35_6: cmp-eq s_35_4 s_35_5
        let s_35_6: bool = ((s_35_4) == (s_35_5));
        // D s_35_7: write-var gs#25959 <= s_35_6
        fn_state.gs_25959 = s_35_6;
        // N s_35_8: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var addrdesc.0:struct
        let s_36_0: ProductType1d757adad216cdef = fn_state.addrdesc._0;
        // D s_36_1: write-var ga#20290 <= s_36_0
        fn_state.ga_20290 = s_36_0;
        // D s_36_2: read-var ga#20290.6:struct
        let s_36_2: ProductType396b95aa74979079 = fn_state.ga_20290._6;
        // D s_36_3: write-var ga#20291 <= s_36_2
        fn_state.ga_20291 = s_36_2;
        // D s_36_4: read-var ga#20291.0:struct
        let s_36_4: u32 = fn_state.ga_20291._0;
        // C s_36_5: const #0u : u32
        let s_36_5: u32 = 0;
        // D s_36_6: cmp-eq s_36_4 s_36_5
        let s_36_6: bool = ((s_36_4) == (s_36_5));
        // D s_36_7: write-var gs#25958 <= s_36_6
        fn_state.gs_25958 = s_36_6;
        // N s_36_8: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #1u : u8
        let s_37_0: bool = true;
        // S s_37_1: call Bit(s_37_0)
        let s_37_1: bool = Bit(state, tracer, s_37_0);
        // C s_37_2: const #19s : i
        let s_37_2: i128 = 19;
        // D s_37_3: read-var syndrome:u64
        let s_37_3: u64 = fn_state.syndrome;
        // D s_37_4: cast zx s_37_3 -> bv
        let s_37_4: Bits = Bits::new(s_37_3 as u128, 64u16);
        // C s_37_5: const #1u : u64
        let s_37_5: u64 = 1;
        // D s_37_6: bit-insert s_37_4 s_37_4 s_37_2 s_37_5
        let s_37_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_37_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_37_4.length(),
            );
            (s_37_4 & mask) | (s_37_4 << s_37_2)
        };
        // D s_37_7: cast reint s_37_6 -> u64
        let s_37_7: u64 = (s_37_6.value() as u64);
        // D s_37_8: write-var syndrome <= s_37_7
        fn_state.syndrome = s_37_7;
        // N s_37_9: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var async_external_abort:u8
        let s_38_0: bool = fn_state.async_external_abort;
        // N s_38_1: branch s_38_0 b41 b39
        if s_38_0 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var addrdesc.7:struct
        let s_39_0: u64 = fn_state.addrdesc._7;
        // D s_39_1: read-var start_vaddr:u64
        let s_39_1: u64 = fn_state.start_vaddr;
        // D s_39_2: cast zx s_39_1 -> bv
        let s_39_2: Bits = Bits::new(s_39_1 as u128, 64u16);
        // D s_39_3: cast zx s_39_0 -> bv
        let s_39_3: Bits = Bits::new(s_39_0 as u128, 64u16);
        // D s_39_4: cmp-ne s_39_2 s_39_3
        let s_39_4: bool = ((s_39_2) != (s_39_3));
        // D s_39_5: write-var gs#25951 <= s_39_4
        fn_state.gs_25951 = s_39_4;
        // N s_39_6: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#25951:u8
        let s_40_0: bool = fn_state.gs_25951;
        // D s_40_1: write-var gs#25952 <= s_40_0
        fn_state.gs_25952 = s_40_0;
        // N s_40_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #1u : u8
        let s_41_0: bool = true;
        // D s_41_1: write-var gs#25951 <= s_41_0
        fn_state.gs_25951 = s_41_0;
        // N s_41_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #() : ()
        let s_42_0: () = ();
        // S s_42_1: call NoFault(s_42_0)
        let s_42_1: ProductType1d757adad216cdef = NoFault(state, tracer, s_42_0);
        // D s_42_2: write-var fault <= s_42_1
        fn_state.fault = s_42_1;
        // D s_42_3: read-var statuscode:u32
        let s_42_3: u32 = fn_state.statuscode;
        // C s_42_4: const #10u : u32
        let s_42_4: u32 = 10;
        // D s_42_5: cmp-eq s_42_3 s_42_4
        let s_42_5: bool = ((s_42_3) == (s_42_4));
        // N s_42_6: branch s_42_5 b54 b43
        if s_42_5 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var statuscode:u32
        let s_43_0: u32 = fn_state.statuscode;
        // C s_43_1: const #14u : u32
        let s_43_1: u32 = 14;
        // D s_43_2: cmp-eq s_43_0 s_43_1
        let s_43_2: bool = ((s_43_0) == (s_43_1));
        // N s_43_3: branch s_43_2 b53 b44
        if s_43_2 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var statuscode:u32
        let s_44_0: u32 = fn_state.statuscode;
        // C s_44_1: const #11u : u32
        let s_44_1: u32 = 11;
        // D s_44_2: cmp-eq s_44_0 s_44_1
        let s_44_2: bool = ((s_44_0) == (s_44_1));
        // D s_44_3: write-var gs#25979 <= s_44_2
        fn_state.gs_25979 = s_44_2;
        // N s_44_4: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#25979:u8
        let s_45_0: bool = fn_state.gs_25979;
        // D s_45_1: write-var gs#25980 <= s_45_0
        fn_state.gs_25980 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#25980:u8
        let s_46_0: bool = fn_state.gs_25980;
        // N s_46_1: branch s_46_0 b52 b47
        if s_46_0 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #15u : u32
        let s_47_0: u32 = 15;
        // D s_47_1: write-var fault.16 <= s_47_0
        fn_state.fault._16 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #() : ()
        let s_48_0: () = ();
        // S s_48_1: call HaveRASExt(s_48_0)
        let s_48_1: bool = HaveRASExt(state, tracer, s_48_0);
        // N s_48_2: branch s_48_1 b51 b49
        if s_48_1 {
            return block_51(state, tracer, fn_state);
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
        // D s_50_0: read-var extflag:u8
        let s_50_0: bool = fn_state.extflag;
        // D s_50_1: write-var fault.5 <= s_50_0
        fn_state.fault._5 = s_50_0;
        // D s_50_2: read-var addrdesc.0:struct
        let s_50_2: ProductType1d757adad216cdef = fn_state.addrdesc._0;
        // D s_50_3: write-var ga#20274 <= s_50_2
        fn_state.ga_20274 = s_50_2;
        // D s_50_4: read-var ga#20274.0:struct
        let s_50_4: ProductType9878976b5bcce9c9 = fn_state.ga_20274._0;
        // D s_50_5: write-var ga#20275 <= s_50_4
        fn_state.ga_20275 = s_50_4;
        // D s_50_6: read-var ga#20275.1:struct
        let s_50_6: u32 = fn_state.ga_20275._1;
        // D s_50_7: write-var fault.0.1 <= s_50_6
        fn_state.fault._0._1 = s_50_6;
        // D s_50_8: read-var fault:struct
        let s_50_8: ProductType1d757adad216cdef = fn_state.fault;
        // D s_50_9: call PendSErrorInterrupt(s_50_8)
        let s_50_9: () = PendSErrorInterrupt(state, tracer, s_50_8);
        // N s_50_10: return
        return;
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var memstatus.1:struct
        let s_51_0: u32 = fn_state.memstatus._1;
        // D s_51_1: write-var fault.10 <= s_51_0
        fn_state.fault._10 = s_51_0;
        // N s_51_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #14u : u32
        let s_52_0: u32 = 14;
        // D s_52_1: write-var fault.16 <= s_52_0
        fn_state.fault._16 = s_52_0;
        // N s_52_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #1u : u8
        let s_53_0: bool = true;
        // D s_53_1: write-var gs#25979 <= s_53_0
        fn_state.gs_25979 = s_53_0;
        // N s_53_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #1u : u8
        let s_54_0: bool = true;
        // D s_54_1: write-var gs#25980 <= s_54_0
        fn_state.gs_25980 = s_54_0;
        // N s_54_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #1u : u8
        let s_55_0: bool = true;
        // D s_55_1: write-var gs#25949 <= s_55_0
        fn_state.gs_25949 = s_55_0;
        // N s_55_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #1u : u8
        let s_56_0: bool = true;
        // D s_56_1: write-var gs#25948 <= s_56_0
        fn_state.gs_25948 = s_56_0;
        // N s_56_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var addrdesc.0:struct
        let s_57_0: ProductType1d757adad216cdef = fn_state.addrdesc._0;
        // D s_57_1: write-var ga#20313 <= s_57_0
        fn_state.ga_20313 = s_57_0;
        // D s_57_2: read-var ga#20313.5:struct
        let s_57_2: bool = fn_state.ga_20313._5;
        // D s_57_3: write-var extflag <= s_57_2
        fn_state.extflag = s_57_2;
        // N s_57_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var addrdesc.0:struct
        let s_58_0: ProductType1d757adad216cdef = fn_state.addrdesc._0;
        // D s_58_1: write-var ga#20314 <= s_58_0
        fn_state.ga_20314 = s_58_0;
        // D s_58_2: read-var ga#20314.16:struct
        let s_58_2: u32 = fn_state.ga_20314._16;
        // D s_58_3: write-var statuscode <= s_58_2
        fn_state.statuscode = s_58_2;
        // N s_58_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #1u : u8
        let s_59_0: bool = true;
        // D s_59_1: write-var gs#25947 <= s_59_0
        fn_state.gs_25947 = s_59_0;
        // N s_59_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #() : ()
        let s_60_0: () = ();
        // S s_60_1: call Unreachable(s_60_0)
        let s_60_1: () = Unreachable(state, tracer, s_60_0);
        // N s_60_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
